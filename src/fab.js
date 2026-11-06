const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const lang = document.getElementById("lang");

async function syncLang() {
  const { settings } = await invoke("get_settings");
  lang.dataset.lang = settings.target_lang;
  lang.textContent = settings.target_lang.toUpperCase();
}

document.getElementById("fab").addEventListener("click", () => invoke("fab_clicked"));
listen("settings-changed", syncLang);
syncLang();
