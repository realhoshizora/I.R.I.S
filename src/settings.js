const { invoke } = window.__TAURI__.core;

const $ = (id) => document.getElementById(id);
let hasKey = false;

function status(text, isError = false) {
  $("status").textContent = text;
  $("status").classList.toggle("error", isError);
}

function showKeyState() {
  $("key").placeholder = hasKey ? "Đã lưu key (để trống nếu giữ nguyên)" : "sk-ant-…";
  $("clear-key").hidden = !hasKey;
}

async function load() {
  const v = await invoke("get_settings");
  $("target").value = v.settings.target_lang;
  $("fab").checked = v.settings.show_fab;
  $("ai").checked = v.settings.ai_enabled;
  $("model").value = v.settings.ai_model;
  $("autostart").checked = v.autostart;
  hasKey = v.has_key;
  showKeyState();
}

async function save(apiKey) {
  const settings = {
    target_lang: $("target").value,
    show_fab: $("fab").checked,
    ai_enabled: $("ai").checked,
    ai_model: $("model").value.trim() || "claude-opus-5-5",
  };
  if (settings.ai_enabled && !hasKey && !apiKey) {
    status("Nhập API key để dùng Claude, hoặc bỏ chọn mục dịch bằng AI.", true);
    $("key").focus();
    return;
  }
  $("save").disabled = true;
  try {
    await invoke("save_settings", { settings, apiKey, autostart: $("autostart").checked });
    if (apiKey !== null) hasKey = apiKey !== "";
    $("key").value = "";
    $("model").value = settings.ai_model;
    showKeyState();
    status("Đã lưu.");
  } catch (e) {
    status(String(e), true);
  } finally {
    $("save").disabled = false;
  }
}

$("form").addEventListener("submit", (e) => {
  e.preventDefault();
  save($("key").value.trim() || null);
});
$("clear-key").addEventListener("click", () => save(""));
$("form").addEventListener("input", () => status(""));
load().catch((e) => status(String(e), true));
