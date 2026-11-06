//! Plain text helpers with no dependencies, so they can be tested with `rustc --test src/text.rs`.

/// Kana, kanji, CJK punctuation and full-width forms: text written without spaces.
pub fn is_cjk(c: char) -> bool {
    matches!(c as u32, 0x3000..=0x30FF | 0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF | 0xFF00..=0xFFEF)
}

/// Joins OCR words. Windows OCR splits Japanese into one "word" per character,
/// so a space is only added between two non-CJK neighbours.
pub fn join_words<I: IntoIterator<Item = String>>(words: I) -> String {
    let mut out = String::new();
    for w in words {
        let w = w.trim();
        if w.is_empty() {
            continue;
        }
        let needs_space = match (out.chars().last(), w.chars().next()) {
            (Some(a), Some(b)) => !is_cjk(a) && !is_cjk(b),
            _ => false,
        };
        if needs_space {
            out.push(' ');
        }
        out.push_str(w);
    }
    out
}

/// Target language for a detected source. Text already in the preferred
/// language goes to English, or to Vietnamese when English is preferred.
pub fn pick_target<'a>(src: &str, preferred: &'a str) -> &'a str {
    if src != preferred {
        preferred
    } else if preferred == "en" {
        "vi"
    } else {
        "en"
    }
}

pub fn lang_name(code: &str) -> &'static str {
    match code {
        "ja" => "Japanese",
        "en" => "English",
        _ => "Vietnamese",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn joins_japanese_without_spaces() {
        assert_eq!(join_words(s(&["勇", "者", "よ", "、", "北", "の", "森"])), "勇者よ、北の森");
    }

    #[test]
    fn joins_english_with_spaces() {
        assert_eq!(join_words(s(&["The", "quick", " ", "fox."])), "The quick fox.");
    }

    #[test]
    fn mixed_text_spaces_only_between_latin() {
        assert_eq!(join_words(s(&["HP", "が", "100", "回", "復"])), "HPが100回復");
        assert_eq!(join_words(s(&["Open", "the", "メニュー"])), "Open theメニュー");
    }

    #[test]
    fn target_avoids_same_language() {
        assert_eq!(pick_target("ja", "vi"), "vi");
        assert_eq!(pick_target("vi", "vi"), "en");
        assert_eq!(pick_target("en", "en"), "vi");
    }
}
