//! Windows-only pieces: reading the selection, watching mouse selections, OCR.

use std::cell::Cell;
use std::mem::size_of;
use std::sync::{mpsc::Sender, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use windows::core::HSTRING;
use windows::Globalization::Language;
use windows::Graphics::Imaging::{BitmapPixelFormat, SoftwareBitmap};
use windows::Media::Ocr::OcrEngine;
use windows::Storage::Streams::DataWriter;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use xcap::image::{imageops, RgbaImage};

use crate::text::{is_cjk, join_words};

pub fn cursor_pos() -> (i32, i32) {
    let mut p = POINT::default();
    unsafe {
        let _ = GetCursorPos(&mut p);
    }
    (p.x, p.y)
}

pub fn foreground() -> isize {
    unsafe { GetForegroundWindow().0 as isize }
}

pub fn focus(hwnd: isize) {
    if hwnd != 0 {
        unsafe {
            let _ = SetForegroundWindow(HWND(hwnd as _));
        }
    }
}

pub fn foreground_is_ours() -> bool {
    let mut pid = 0u32;
    unsafe {
        GetWindowThreadProcessId(GetForegroundWindow(), Some(&mut pid));
        pid == GetCurrentProcessId()
    }
}

/// Shows a window without taking focus away from the app the user is reading.
pub fn show_no_activate(hwnd: isize) {
    unsafe {
        let h = HWND(hwnd as _);
        let ex = GetWindowLongPtrW(h, GWL_EXSTYLE);
        SetWindowLongPtrW(h, GWL_EXSTYLE, ex | WS_EX_NOACTIVATE.0 as isize);
        let _ = ShowWindow(h, SW_SHOWNOACTIVATE);
    }
}

fn key_down(vk: VIRTUAL_KEY) -> bool {
    unsafe { (GetAsyncKeyState(vk.0 as i32) as u16) & 0x8000 != 0 }
}

fn send_keys(keys: &[(VIRTUAL_KEY, bool)]) {
    let inputs: Vec<INPUT> = keys
        .iter()
        .map(|&(vk, up)| INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: vk, dwFlags: if up { KEYEVENTF_KEYUP } else { KEYBD_EVENT_FLAGS(0) }, ..Default::default() },
            },
        })
        .collect();
    unsafe {
        SendInput(&inputs, size_of::<INPUT>() as i32);
    }
}

/// Copies the current selection with Ctrl+C and puts the user's clipboard back.
pub fn copy_selection() -> Option<String> {
    // Alt/Shift still held from the hotkey would turn our Ctrl+C into Ctrl+Alt+C.
    let start = Instant::now();
    while [VK_MENU, VK_SHIFT, VK_LWIN, VK_RWIN].into_iter().any(key_down) && start.elapsed() < Duration::from_millis(800) {
        thread::sleep(Duration::from_millis(15));
    }

    let mut cb = arboard::Clipboard::new().ok()?;
    let saved_text = cb.get_text().ok();
    let saved_img = if saved_text.is_none() { cb.get_image().ok().map(|i| i.to_owned_img()) } else { None };
    let _ = cb.clear();

    send_keys(&[(VK_CONTROL, false), (VK_C, false), (VK_C, true), (VK_CONTROL, true)]);
    let mut got = None;
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(500) {
        thread::sleep(Duration::from_millis(25));
        if let Ok(t) = cb.get_text() {
            if !t.trim().is_empty() {
                got = Some(t.trim().to_string());
                break;
            }
        }
    }

    // shortcut: only text or an image is restored; files or rich formats on the clipboard are lost. Upgrade by saving every clipboard format if users complain.
    match (saved_text, saved_img) {
        (Some(t), _) => {
            let _ = cb.set_text(t);
        }
        (_, Some(i)) => {
            let _ = cb.set_image(i);
        }
        _ => {}
    }
    got
}

/// (press point, release point) of a mouse text selection.
pub type Selection = ((i32, i32), (i32, i32));

static SELECT_TX: OnceLock<Sender<Selection>> = OnceLock::new();

thread_local! {
    // (press point, press happened over text, last release time, last release point)
    static MOUSE: Cell<((i32, i32), bool, u32, (i32, i32))> = Cell::new(((0, 0), false, 0, (0, 0)));
}

fn cursor_is_ibeam() -> bool {
    unsafe {
        let mut ci = CURSORINFO { cbSize: size_of::<CURSORINFO>() as u32, ..Default::default() };
        let ibeam = LoadCursorW(None, IDC_IBEAM).unwrap_or_default();
        GetCursorInfo(&mut ci).is_ok() && ci.hCursor == ibeam
    }
}

unsafe extern "system" fn mouse_proc(code: i32, w: WPARAM, l: LPARAM) -> LRESULT {
    if code >= 0 {
        let info = &*(l.0 as *const MSLLHOOKSTRUCT);
        let p = (info.pt.x, info.pt.y);
        match w.0 as u32 {
            WM_LBUTTONDOWN => MOUSE.with(|m| {
                let mut v = m.get();
                v.0 = p;
                v.1 = cursor_is_ibeam();
                m.set(v);
            }),
            WM_LBUTTONUP => MOUSE.with(|m| {
                let mut v = m.get();
                let dragged = (p.0 - v.0 .0).abs() + (p.1 - v.0 .1).abs() > 12;
                let double = info.time.wrapping_sub(v.2) < GetDoubleClickTime() && (p.0 - v.3 .0).abs() < 5 && (p.1 - v.3 .1).abs() < 5;
                // Only drags and double-clicks that started on text (I-beam cursor) count as a selection.
                if v.1 && (dragged || double) {
                    if let Some(tx) = SELECT_TX.get() {
                        let _ = tx.send((v.0, p));
                    }
                }
                v.2 = info.time;
                v.3 = p;
                m.set(v);
            }),
            _ => {}
        }
    }
    CallNextHookEx(None, code, w, l)
}

/// Sends every mouse text selection to `tx`.
pub fn watch_selections(tx: Sender<Selection>) {
    let _ = SELECT_TX.set(tx);
    thread::spawn(|| unsafe {
        if SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), None, 0).is_err() {
            return;
        }
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {}
    });
}

fn engine(tag: &str) -> Option<OcrEngine> {
    let lang = Language::CreateLanguage(&HSTRING::from(tag)).ok()?;
    OcrEngine::TryCreateFromLanguage(&lang).ok()
}

fn recognize(engine: &OcrEngine, bmp: &SoftwareBitmap) -> windows::core::Result<String> {
    let result = engine.RecognizeAsync(bmp)?.get()?;
    let mut words = Vec::new();
    for line in result.Lines()? {
        for word in line.Words()? {
            words.push(word.Text()?.to_string());
        }
    }
    Ok(join_words(words))
}

/// Reads text in a screen capture with the OCR engine built into Windows.
pub fn ocr(img: &RgbaImage) -> Result<String, String> {
    let max = OcrEngine::MaxImageDimension().unwrap_or(2600);
    let (w, h) = img.dimensions();
    if w > max || h > max {
        return Err("Vùng chọn quá lớn. Hãy khoanh nhỏ hơn quanh đoạn chữ cần dịch.".into());
    }
    // Screen text is small for Windows OCR; doubling the size reads it far more reliably.
    let img = if w * 2 <= max && h * 2 <= max { imageops::resize(img, w * 2, h * 2, imageops::FilterType::CatmullRom) } else { img.clone() };
    let (w, h) = img.dimensions();
    let mut bgra = img.into_raw();
    for px in bgra.chunks_exact_mut(4) {
        px.swap(0, 2);
    }
    let bmp = (|| -> windows::core::Result<SoftwareBitmap> {
        let writer = DataWriter::new()?;
        writer.WriteBytes(&bgra)?;
        SoftwareBitmap::CreateCopyFromBuffer(&writer.DetachBuffer()?, BitmapPixelFormat::Bgra8, w as i32, h as i32)
    })()
    .map_err(|e| format!("Không đọc được ảnh chụp màn hình: {e}"))?;

    // Try Japanese first (it also reads Latin letters), then English for text with no kana/kanji.
    let ja = engine("ja").and_then(|e| recognize(&e, &bmp).ok());
    if let Some(t) = &ja {
        if t.chars().any(is_cjk) {
            return Ok(t.clone());
        }
    }
    let other = engine("en-US").or_else(|| OcrEngine::TryCreateFromUserProfileLanguages().ok()).and_then(|e| recognize(&e, &bmp).ok());
    match other.or(ja) {
        Some(t) if !t.trim().is_empty() => Ok(t),
        Some(_) => Err("Không tìm thấy chữ nào trong vùng này. Thử khoanh sát đoạn chữ hơn.".into()),
        None => Err("Windows chưa có bộ nhận dạng chữ. Mở Cài đặt của I.R.I.S để xem cách cài gói ngôn ngữ.".into()),
    }
}
