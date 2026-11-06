const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const win = window.__TAURI__.window.getCurrentWindow();

const box = document.getElementById("box");
let start = null;

function rect(e) {
  const x = Math.min(start.x, e.clientX), y = Math.min(start.y, e.clientY);
  return { x, y, w: Math.abs(e.clientX - start.x), h: Math.abs(e.clientY - start.y) };
}

function reset() {
  start = null;
  box.hidden = true;
  document.body.classList.remove("dragging");
}

function cancel() {
  reset();
  win.hide();
}

document.addEventListener("pointerdown", (e) => {
  if (e.button !== 0) return cancel();
  start = { x: e.clientX, y: e.clientY };
  document.body.setPointerCapture(e.pointerId);
});

document.addEventListener("pointermove", (e) => {
  if (!start) return;
  const r = rect(e);
  Object.assign(box.style, { left: r.x + "px", top: r.y + "px", width: r.w + "px", height: r.h + "px" });
  box.hidden = false;
  document.body.classList.add("dragging");
});

document.addEventListener("pointerup", (e) => {
  if (!start) return;
  const r = rect(e);
  reset();
  if (r.w < 6 || r.h < 6) return cancel();
  // The backend works in physical pixels relative to this monitor.
  const s = window.devicePixelRatio;
  invoke("ocr_region", { x: Math.round(r.x * s), y: Math.round(r.y * s), w: Math.round(r.w * s), h: Math.round(r.h * s) });
});

document.addEventListener("contextmenu", (e) => {
  e.preventDefault();
  cancel();
});
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape") cancel();
});
listen("reset", reset);
