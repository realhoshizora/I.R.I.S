use serde::Serialize;
use serde_json::{json, Value};
use std::time::Duration;

use crate::text::{lang_name, pick_target};

#[derive(Serialize, Clone, Default)]
pub struct DictEntry {
    pub pos: String,
    pub terms: Vec<String>,
}

#[derive(Serialize, Clone, Default)]
pub struct Translation {
    pub source: String,
    pub src_lang: String,
    pub tgt_lang: String,
    pub text: String,
    /// Romanized source (romaji for Japanese), when Google returns one.
    pub romaji: String,
    /// Kana reading for a Japanese word, IPA for an English word.
    pub reading: String,
    /// Dictionary rows; non-empty only when the source is a single word.
    pub dict: Vec<DictEntry>,
    pub engine: String,
}

pub struct Ai {
    pub key: String,
    pub model: String,
}

const NET_ERR: &str = "Không kết nối được dịch vụ dịch. Kiểm tra mạng rồi thử lại.";

pub async fn translate(http: &reqwest::Client, text: &str, preferred: &str, ai: Option<Ai>) -> Result<Translation, String> {
    let mut t = match google(http, text, preferred).await {
        Ok(t) if t.src_lang == t.tgt_lang => google(http, text, pick_target(&t.src_lang, preferred)).await?,
        Ok(t) => t,
        // Google down or blocked: Claude can still translate on its own.
        Err(e) => match &ai {
            Some(_) => Translation { source: text.into(), src_lang: "auto".into(), tgt_lang: preferred.into(), ..Default::default() },
            None => return Err(e),
        },
    };

    if !t.dict.is_empty() {
        t.reading = reading(http, text.trim(), &t.src_lang).await.unwrap_or_default();
    } else if let Some(ai) = ai {
        t.text = claude(http, &ai, text, &t.tgt_lang).await?;
        t.engine = "Claude".into();
    }
    Ok(t)
}

async fn get_json(http: &reqwest::Client, url: &str, query: &[(&str, &str)]) -> Result<Value, String> {
    let res = http.get(url).query(query).timeout(Duration::from_secs(8)).send().await.map_err(|_| NET_ERR.to_string())?;
    if !res.status().is_success() {
        return Err(format!("Dịch vụ dịch trả lỗi {}. Thử lại sau ít phút.", res.status().as_u16()));
    }
    res.json().await.map_err(|_| NET_ERR.to_string())
}

async fn google(http: &reqwest::Client, text: &str, target: &str) -> Result<Translation, String> {
    let v = get_json(
        http,
        "https://translate.googleapis.com/translate_a/single",
        &[("client", "gtx"), ("sl", "auto"), ("tl", target), ("hl", "vi"), ("dj", "1"), ("dt", "t"), ("dt", "bd"), ("dt", "rm"), ("q", text)],
    )
    .await?;
    Ok(parse_google(&v, text, target))
}

fn parse_google(v: &Value, text: &str, target: &str) -> Translation {
    let str_of = |x: &Value, k: &str| x.get(k).and_then(Value::as_str).unwrap_or("").to_string();
    let sentences = v["sentences"].as_array().cloned().unwrap_or_default();
    Translation {
        source: text.into(),
        src_lang: str_of(v, "src"),
        tgt_lang: target.into(),
        text: sentences.iter().map(|s| str_of(s, "trans")).collect(),
        romaji: sentences.iter().map(|s| str_of(s, "src_translit")).collect::<String>(),
        reading: String::new(),
        dict: v["dict"]
            .as_array()
            .map(|d| {
                d.iter()
                    .map(|e| DictEntry {
                        pos: str_of(e, "pos"),
                        terms: e["terms"].as_array().map(|t| t.iter().filter_map(|x| x.as_str().map(String::from)).take(6).collect()).unwrap_or_default(),
                    })
                    .filter(|e| !e.terms.is_empty())
                    .collect()
            })
            .unwrap_or_default(),
        engine: "Google".into(),
    }
}

/// Best effort: a missing reading never fails the lookup.
async fn reading(http: &reqwest::Client, word: &str, lang: &str) -> Option<String> {
    let found = match lang {
        "ja" => {
            let v = get_json(http, "https://jisho.org/api/v1/search/words", &[("keyword", word)]).await.ok()?;
            v["data"][0]["japanese"][0]["reading"].as_str().map(String::from)
        }
        "en" => {
            let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word.to_lowercase());
            let v = get_json(http, &url, &[]).await.ok()?;
            let e = &v[0];
            e["phonetic"].as_str().map(String::from).or_else(|| {
                e["phonetics"].as_array()?.iter().find_map(|p| p["text"].as_str().filter(|s| !s.is_empty()).map(String::from))
            })
        }
        _ => None,
    };
    found.filter(|r| r != word)
}

async fn claude(http: &reqwest::Client, ai: &Ai, text: &str, target: &str) -> Result<String, String> {
    let mut body = json!({
        "model": ai.model,
        "max_tokens": 16000,
        "output_config": { "effort": "low" },
        "system": format!(
            "You translate text that a user selected on their screen into {}. Reply with the translation only: no notes, quotes or explanations. Keep the original line breaks. The text may come from OCR and contain small recognition errors; translate what it was meant to say.",
            lang_name(target)
        ),
        "messages": [{ "role": "user", "content": text }],
    });
    // Server-side fallback re-runs a safety-classifier refusal on another model; only these models accept the "default" form.
    let fallback = matches!(ai.model.as_str(), "claude-opus-5-5" | "claude-opus-5" | "claude-fable-5-1" | "claude-sonnet-5-5");
    if fallback {
        body["fallbacks"] = json!("default");
    }
    let mut req = http
        .post("https://api.anthropic.com/v1/messages")
        .timeout(Duration::from_secs(45))
        .header("x-api-key", &ai.key)
        .header("anthropic-version", "2026-10-21")
        .json(&body);
    if fallback {
        req = req.header("anthropic-beta", "server-side-fallback-2026-07-01");
    }
    let res = req.send().await.map_err(|_| "Không kết nối được Claude. Kiểm tra mạng rồi thử lại.".to_string())?;
    let status = res.status().as_u16();
    let v: Value = res.json().await.map_err(|_| "Claude trả về dữ liệu không đọc được.".to_string())?;
    match status {
        200 => {}
        401 | 403 => return Err("API key của Claude không đúng hoặc đã bị thu hồi. Mở Cài đặt để nhập lại.".into()),
        429 => return Err("Claude đang giới hạn số lần gọi. Đợi một lát rồi thử lại.".into()),
        _ => return Err(format!("Claude báo lỗi {status}: {}", v["error"]["message"].as_str().unwrap_or("không rõ"))),
    }
    if v["stop_reason"] == "refusal" {
        return Err("Claude từ chối dịch đoạn này. Tắt dịch bằng AI để dùng Google.".into());
    }
    let out: String = v["content"].as_array().into_iter().flatten().filter(|b| b["type"] == "text").filter_map(|b| b["text"].as_str()).collect();
    if out.trim().is_empty() {
        return Err("Claude không trả về bản dịch. Thử lại.".into());
    }
    Ok(out.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_google_word_response() {
        let v = json!({
            "sentences": [{"trans": "ăn", "orig": "食べる"}, {"src_translit": "taberu"}],
            "dict": [{"pos": "động từ", "terms": ["ăn", "dùng bữa"]}, {"pos": "danh từ", "terms": []}],
            "src": "ja"
        });
        let t = parse_google(&v, "食べる", "vi");
        assert_eq!(t.text, "ăn");
        assert_eq!(t.romaji, "taberu");
        assert_eq!(t.src_lang, "ja");
        assert_eq!(t.dict.len(), 1);
        assert_eq!(t.dict[0].terms, vec!["ăn", "dùng bữa"]);
    }

    #[test]
    fn parses_google_sentence_response() {
        let v = json!({"sentences": [{"trans": "Câu một. "}, {"trans": "Câu hai."}], "src": "en"});
        let t = parse_google(&v, "One. Two.", "vi");
        assert_eq!(t.text, "Câu một. Câu hai.");
        assert!(t.dict.is_empty());
    }
}
