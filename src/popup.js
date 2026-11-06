const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const win = window.__TAURI__.window.getCurrentWindow();

const $ = (id) => document.getElementById(id);
const cues = $("cues");
let seq = 0;
let pinned = false;
let lastText = null;
let copyValue = "";
let target = "vi";

function el(tag, attrs = {}, text) {
  const n = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) if (v) n.setAttribute(k, v);
  if (text != null) n.textContent = text;
  return n;
}

function setTracks(src, tgt) {
  for (const [id, code] of [["src-track", src], ["tgt-track", tgt]]) {
    const t = $(id);
    t.dataset.lang = code || "";
    t.textContent = code && code !== "auto" ? code.toUpperCase() : "…";
  }
}

async function fit() {
  await new Promise(requestAnimationFrame);
  await invoke("fit_popup", { height: Math.ceil(document.querySelector(".frame").getBoundingClientRect().height) });
}

function renderLoading(text) {
  setTracks("auto", target);
  $("engine").textContent = "";
  copyValue = "";
  const lang = /[\u3040-\u30ff\u3400-\u9fff]/.test(text) ? "ja" : null;
  cues.replaceChildren(el("p", { class: "src", lang }, text), el("p", { class: "tl pending" }, "Đang dịch…"));
}

function renderError(message, canRetry) {
  copyValue = "";
  const nodes = [el("p", { class: "err", role: "alert" }, message)];
  if (canRetry) {
    const b = el("button", { class: "retry", type: "button" }, "Thử lại");
    b.addEventListener("click", () => run(lastText));
    nodes.push(b);
  }
  cues.replaceChildren(...nodes);
}

function renderResult(t) {
  setTracks(t.src_lang, t.tgt_lang);
  $("engine").textContent = t.engine;
  if (t.dict.length) {
    const head = el("div", { class: "head" });
    head.append(el("span", { class: "w", lang: t.src_lang }, t.source));
    const reading = [t.reading, t.romaji && t.romaji !== t.reading ? t.romaji : ""].filter(Boolean).join(" · ");
    if (reading) head.append(el("span", { class: "r" }, reading));
    const dl = el("dl", { class: "pos" });
    for (const d of t.dict) dl.append(el("dt", {}, d.pos || "Nghĩa"), el("dd", { lang: t.tgt_lang }, d.terms.join(", ")));
    cues.replaceChildren(head, dl);
    copyValue = t.dict.map((d) => `${d.pos}: ${d.terms.join(", ")}`).join("\n");
  } else {
    cues.replaceChildren(el("p", { class: "src", lang: t.src_lang }, t.source), el("p", { class: "tl", lang: t.tgt_lang }, t.text));
    copyValue = t.text;
  }
}

async function run(text) {
  const my = ++seq;
  lastText = text;
  renderLoading(text);
  await fit();
  try {
    const t = await invoke("translate", { text });
    if (my !== seq) return;
    renderResult(t);
  } catch (e) {
    if (my !== seq) return;
    renderError(String(e), true);
  }
  await fit();
}

listen("show", async ({ payload }) => {
  if (payload.error) {
    seq++;
    setTracks("auto", target);
    $("engine").textContent = "";
    renderError(payload.error, false);
    await fit();
  } else {
    run(payload.text);
  }
});

function hide() {
  seq++;
  pinned = false;
  $("pin").setAttribute("aria-pressed", "false");
  win.hide();
}

$("close").addEventListener("click", hide);
$("pin").addEventListener("click", () => {
  pinned = !pinned;
  $("pin").setAttribute("aria-pressed", String(pinned));
});
$("copy").addEventListener("click", async () => {
  if (!copyValue) return;
  await invoke("copy_text", { text: copyValue });
  const use = $("copy").querySelector("use");
  use.setAttribute("href", "#i-check");
  $("copy").setAttribute("aria-label", "Đã sao chép");
  setTimeout(() => {
    use.setAttribute("href", "#i-copy");
    $("copy").setAttribute("aria-label", "Sao chép bản dịch");
  }, 1200);
});
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape") hide();
});
window.addEventListener("blur", () => {
  if (!pinned) hide();
});

async function syncTarget() {
  target = (await invoke("get_settings")).settings.target_lang;
}
listen("settings-changed", syncTarget);
syncTarget();
