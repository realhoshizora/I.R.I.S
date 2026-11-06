#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod text;
mod translate;
mod win;

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicIsize, AtomicU64, Ordering};
use std::sync::Mutex;
use std::{fs, thread, time::{Duration, Instant}};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, RunEvent, State, WindowEvent};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, ShortcutState};

const KEY_SERVICE: &str = "I.R.I.S";
const KEY_USER: &str = "anthropic-api-key";
const POPUP_WIDTH: f64 = 420.0;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
struct Settings {
    target_lang: String,
    show_fab: bool,
    ai_enabled: bool,
    ai_model: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self { target_lang: "vi".into(), show_fab: true, ai_enabled: false, ai_model: "claude-opus-5-5".into() }
    }
}

/// Screen rectangle in physical pixels that the popup must not cover.
#[derive(Clone, Copy, Default)]
struct Rect {
    x: i32,
    y: i32,
    h: i32,
}

#[derive(Default)]
struct AppState {
    settings: Mutex<Settings>,
    anchor: Mutex<Rect>,
    overlay_origin: Mutex<(i32, i32)>,
    http: reqwest::Client,
    fab_gen: AtomicU64,
    fab_point: Mutex<(i32, i32)>,
    fab_owner: AtomicIsize,
    last_selection: Mutex<Option<(win::Selection, Instant)>>,
}

#[derive(Serialize, Clone)]
struct ShowPayload {
    text: Option<String>,
    error: Option<String>,
}

fn key_entry() -> Result<keyring::Entry, String> {
    keyring::Entry::new(KEY_SERVICE, KEY_USER).map_err(|e| e.to_string())
}

fn settings_path(app: &AppHandle) -> Option<std::path::PathBuf> {
    app.path().app_config_dir().ok().map(|d| d.join("settings.json"))
}

fn load_settings(app: &AppHandle) -> Settings {
    settings_path(app).and_then(|p| fs::read_to_string(p).ok()).and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default()
}

/// Sends text (or an error) to the popup; the popup sizes itself and calls `fit_popup` to appear.
fn show_popup(app: &AppHandle, anchor: Rect, payload: ShowPayload) {
    *app.state::<AppState>().anchor.lock().unwrap() = anchor;
    let _ = app.emit_to("popup", "show", payload);
}

/// The caret line sits around the cursor; keep the popup clear of it.
fn cursor_anchor(x: i32, y: i32) -> Rect {
    Rect { x: x - 24, y: y - 18, h: 44 }
}

/// Covers every line of the last mouse selection when the cursor is still at its end,
/// so a selection dragged upwards does not get the popup over its lower lines.
/// Keyboard selections fall back to the text caret, then to the mouse cursor.
// shortcut: keyboard selections only know the cursor position; upgrade with UI Automation TextPattern bounds if that matters.
fn selection_anchor(app: &AppHandle, x: i32, y: i32) -> Rect {
    if let Some(((a, b), at)) = *app.state::<AppState>().last_selection.lock().unwrap() {
        if at.elapsed() < Duration::from_secs(120) && (x - b.0).abs() < 40 && (y - b.1).abs() < 40 {
            let top = a.1.min(b.1);
            return Rect { x: a.0.min(b.0) - 24, y: top - 18, h: (a.1 - b.1).abs() + 44 };
        }
    }
    match win::caret() {
        Some((cx, cy, ch)) => Rect { x: cx - 24, y: cy - 4, h: ch + 8 },
        None => cursor_anchor(x, y),
    }
}

fn translate_selection(app: AppHandle, anchor: Rect) {
    if let Some(fab) = app.get_webview_window("fab") {
        let _ = fab.hide();
    }
    thread::spawn(move || {
        let payload = match win::copy_selection() {
            Some(text) => ShowPayload { text: Some(text), error: None },
            None => ShowPayload { text: None, error: Some("Không có chữ nào đang được bôi đen. Bôi đen đoạn cần dịch rồi bấm lại Ctrl+Alt+D.".into()) },
        };
        show_popup(&app, anchor, payload);
    });
}

fn start_region(app: &AppHandle) {
    let Some(overlay) = app.get_webview_window("overlay") else { return };
    let (cx, cy) = win::cursor_pos();
    let Ok(Some(mon)) = overlay.monitor_from_point(cx as f64, cy as f64) else { return };
    let pos = *mon.position();
    *app.state::<AppState>().overlay_origin.lock().unwrap() = (pos.x, pos.y);
    let _ = overlay.set_position(pos);
    let _ = overlay.set_size(*mon.size());
    let _ = overlay.emit("reset", ());
    let _ = overlay.show();
    let _ = overlay.set_focus();
}

fn on_mouse_selection(app: &AppHandle, sel: win::Selection) {
    let state = app.state::<AppState>();
    *state.last_selection.lock().unwrap() = Some((sel, Instant::now()));
    let (x, y) = sel.1;
    if !state.settings.lock().unwrap().show_fab || win::foreground_is_ours() {
        return;
    }
    let Some(fab) = app.get_webview_window("fab") else { return };
    state.fab_owner.store(win::foreground(), Ordering::SeqCst);
    *state.fab_point.lock().unwrap() = (x, y);
    let gen = state.fab_gen.fetch_add(1, Ordering::SeqCst) + 1;
    let _ = fab.set_position(PhysicalPosition::new(x + 14, y + 14));
    if let Ok(hwnd) = fab.hwnd() {
        win::show_no_activate(hwnd.0 as isize);
    }
    let app = app.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(2600));
        if app.state::<AppState>().fab_gen.load(Ordering::SeqCst) == gen {
            if let Some(fab) = app.get_webview_window("fab") {
                let _ = fab.hide();
            }
        }
    });
}

#[tauri::command]
async fn translate(text: String, state: State<'_, AppState>) -> Result<translate::Translation, String> {
    let s = state.settings.lock().unwrap().clone();
    let ai = if s.ai_enabled { key_entry().ok().and_then(|e| e.get_password().ok()).map(|key| translate::Ai { key, model: s.ai_model.clone() }) } else { None };
    translate::translate(&state.http, &text, &s.target_lang, ai).await
}

/// Places the popup below the anchor, or above it when there is no room, and shows it.
/// Returns the height (logical px) it was given, which is less than asked when neither side fits.
#[tauri::command]
fn fit_popup(app: AppHandle, height: f64, state: State<'_, AppState>) -> f64 {
    let Some(popup) = app.get_webview_window("popup") else { return height };
    let a = *state.anchor.lock().unwrap();
    let Ok(Some(mon)) = popup.monitor_from_point(a.x as f64, a.y as f64).or_else(|_| popup.primary_monitor()) else { return height };
    let s = mon.scale_factor();
    let area = mon.work_area();
    let (left, top) = (area.position.x, area.position.y);
    let (right, bottom) = (left + area.size.width as i32, top + area.size.height as i32);
    let (w, want) = ((POPUP_WIDTH * s) as i32, (height * s).ceil() as i32);
    let gap = (4.0 * s) as i32;
    let below = bottom - (a.y + a.h + gap);
    let above = a.y - gap - top;
    // Stay off the selection: take the side where it fits, else the roomier side and let the text scroll.
    let min = (120.0 * s) as i32;
    let (y, h) = if want <= below || (want > above && below >= above) {
        let h = want.min(below.max(min));
        (a.y + a.h + gap, h)
    } else {
        let h = want.min(above.max(min));
        ((a.y - gap - h).max(top), h)
    };
    // shortcut: a selection taller than the whole screen leaves no free side, so the popup clamps onto it.
    let y = y.min(bottom - h).max(top);
    let x = a.x.min(right - w).max(left);
    let _ = popup.set_size(PhysicalSize::new(w as u32, h as u32));
    let _ = popup.set_position(PhysicalPosition::new(x, y));
    if !popup.is_visible().unwrap_or(false) {
        let _ = popup.show();
        let _ = popup.set_focus();
    }
    h as f64 / s
}

#[tauri::command]
fn ocr_region(app: AppHandle, x: i32, y: i32, w: i32, h: i32) {
    if let Some(o) = app.get_webview_window("overlay") {
        let _ = o.hide();
    }
    thread::spawn(move || {
        // Let the dimmed overlay disappear before the capture.
        thread::sleep(Duration::from_millis(150));
        let (ox, oy) = *app.state::<AppState>().overlay_origin.lock().unwrap();
        let anchor = Rect { x: ox + x, y: oy + y, h };
        let result = xcap::Monitor::from_point(ox, oy)
            .and_then(|m| m.capture_image())
            .map_err(|e| format!("Không chụp được màn hình: {e}"))
            .and_then(|img| {
                let (iw, ih) = img.dimensions();
                let (cx, cy) = (x.clamp(0, iw as i32 - 1) as u32, y.clamp(0, ih as i32 - 1) as u32);
                let crop = xcap::image::imageops::crop_imm(&img, cx, cy, (w as u32).min(iw - cx), (h as u32).min(ih - cy)).to_image();
                win::ocr(&crop)
            });
        let payload = match result {
            Ok(text) => ShowPayload { text: Some(text), error: None },
            Err(e) => ShowPayload { text: None, error: Some(e) },
        };
        show_popup(&app, anchor, payload);
    });
}

#[tauri::command]
fn fab_clicked(app: AppHandle, state: State<'_, AppState>) {
    // Give focus back to the app that owns the selection before copying it.
    win::focus(state.fab_owner.load(Ordering::SeqCst));
    let (x, y) = *state.fab_point.lock().unwrap();
    thread::sleep(Duration::from_millis(60));
    let anchor = selection_anchor(&app, x, y);
    translate_selection(app, anchor);
}

#[tauri::command]
fn copy_text(text: String) -> Result<(), String> {
    arboard::Clipboard::new().and_then(|mut c| c.set_text(text)).map_err(|_| "Không sao chép được.".to_string())
}

#[derive(Serialize)]
struct SettingsView {
    settings: Settings,
    has_key: bool,
    autostart: bool,
}

#[tauri::command]
fn get_settings(app: AppHandle, state: State<'_, AppState>) -> SettingsView {
    SettingsView {
        settings: state.settings.lock().unwrap().clone(),
        has_key: key_entry().and_then(|e| e.get_password().map_err(|e| e.to_string())).is_ok(),
        autostart: app.autolaunch().is_enabled().unwrap_or(false),
    }
}

/// `api_key`: None keeps the saved key, Some("") deletes it.
#[tauri::command]
fn save_settings(app: AppHandle, settings: Settings, api_key: Option<String>, autostart: bool, state: State<'_, AppState>) -> Result<(), String> {
    match api_key.as_deref().map(str::trim) {
        Some("") => {
            let _ = key_entry()?.delete_credential();
        }
        Some(k) => key_entry()?.set_password(k).map_err(|e| format!("Không lưu được API key: {e}"))?,
        None => {}
    }
    let auto = app.autolaunch();
    let _ = if autostart { auto.enable() } else { auto.disable() };
    let path = settings_path(&app).ok_or("Không tìm thấy thư mục cài đặt.")?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    fs::write(&path, serde_json::to_string_pretty(&settings).unwrap()).map_err(|e| format!("Không lưu được cài đặt: {e}"))?;
    *state.settings.lock().unwrap() = settings;
    let _ = app.emit("settings-changed", ());
    Ok(())
}

fn open_settings(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    if shortcut.matches(Modifiers::CONTROL | Modifiers::ALT, Code::KeyD) {
                        let (x, y) = win::cursor_pos();
                        translate_selection(app.clone(), selection_anchor(app, x, y));
                    } else if shortcut.matches(Modifiers::CONTROL | Modifiers::ALT, Code::KeyS) {
                        start_region(app);
                    }
                })
                .build(),
        )
        .manage(AppState { http: reqwest::Client::new(), ..Default::default() })
        .invoke_handler(tauri::generate_handler![translate, fit_popup, ocr_region, fab_clicked, copy_text, get_settings, save_settings])
        .setup(|app| {
            let handle = app.handle().clone();
            *app.state::<AppState>().settings.lock().unwrap() = load_settings(&handle);

            let mut failed = Vec::new();
            for hk in ["ctrl+alt+d", "ctrl+alt+s"] {
                if app.global_shortcut().register(hk).is_err() {
                    failed.push(hk);
                }
            }

            let item = |id: &str, label: &str| MenuItem::with_id(app, id, label, true, None::<&str>);
            let menu = Menu::with_items(
                app,
                &[
                    &item("selection", "Dịch chữ đang bôi đen    Ctrl+Alt+D")?,
                    &item("region", "Khoanh vùng để dịch    Ctrl+Alt+S")?,
                    &PredefinedMenuItem::separator(app)?,
                    &item("settings", "Cài đặt")?,
                    &item("quit", "Thoát I.R.I.S")?,
                ],
            )?;
            let tooltip = if failed.is_empty() { "I.R.I.S".to_string() } else { format!("I.R.I.S: phím tắt {} đang bị app khác dùng", failed.join(", ")) };
            TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().cloned().unwrap())
                .tooltip(tooltip)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, e| match e.id.as_ref() {
                    "selection" => {
                        let (x, y) = win::cursor_pos();
                        translate_selection(app.clone(), selection_anchor(app, x, y));
                    }
                    "region" => start_region(app),
                    "settings" => open_settings(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, e| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                        open_settings(tray.app_handle());
                    }
                })
                .build(app)?;

            if let Some(w) = app.get_webview_window("settings") {
                let w2 = w.clone();
                w.on_window_event(move |e| {
                    if let WindowEvent::CloseRequested { api, .. } = e {
                        api.prevent_close();
                        let _ = w2.hide();
                    }
                });
            }

            let (tx, rx) = std::sync::mpsc::channel();
            win::watch_selections(tx);
            thread::spawn(move || {
                for p in rx {
                    on_mouse_selection(&handle, p);
                }
            });
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("I.R.I.S failed to start");

    // Stay in the tray when every window is hidden; only "Thoát" (exit code 0) quits.
    app.run(|_, e| {
        if let RunEvent::ExitRequested { code: None, api, .. } = e {
            api.prevent_exit();
        }
    });
}
