# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

(Desktop app for Windows; its UI is rendered by a webview inside a Tauri shell, so the design language is web, not native WinUI.)

## Stack

delegated: Tauri 2 (Rust backend + plain HTML/CSS/JS frontend). Chosen because it stays resident in the tray at a few tens of MB of RAM, ships as a ~10 MB installer, and Rust can call the built-in Windows OCR (Windows.Media.Ocr) and global hotkeys directly.

## Users

Vietnamese speakers on Windows who read Japanese and English content (web pages, documents, games, videos, PDFs, screenshots) and need the meaning in Vietnamese without leaving what they are reading.

## Product Purpose

I.R.I.S lives in the system tray all day. The user highlights text, or draws a box over any part of the screen, and gets the translation in a small popup right next to it. Single words get a dictionary entry (reading, part of speech, meanings) instead of a bare translation. Success: the user never alt-tabs to a browser translator.

## Positioning

Works on anything visible on screen, including text that cannot be selected (images, games, video, scanned PDFs), through on-device Windows OCR; translation comes free by default with an optional AI engine for context-aware results.

## Operating Context

- Triggered dozens of times per session, mid-reading, by keyboard shortcut.
- The popup appears over another app the user is focused on; it must be quick to read and quick to dismiss.
- Japanese text may be vertical or mixed with kana/kanji; English text may be a single word or a paragraph.

## Capabilities and Constraints

- Languages, first release: Japanese, English, Vietnamese (any direction; target defaults to Vietnamese).
- Selected-text translation via global hotkey (copies selection, translates, restores the clipboard).
- Screen-region translation via global hotkey: dim overlay, drag a box, OCR, translate.
- Dictionary lookup for single words/short phrases.
- Engines: Google Translate free endpoint by default; optional AI engine with the user's own API key.
- OCR uses Windows' built-in engine; Japanese OCR needs the Japanese language pack installed in Windows.
- Undecided: vocabulary notebook, continuous region translation, text-to-speech, history (offered, not chosen for v1).

## Brand Commitments

Name: I.R.I.S (from the repository name).

## Evidence on Hand

None. No users, reviews or benchmarks exist; do not invent any.

## Product Principles

1. Never pull the user out of what they are reading.
2. Keyboard first; mouse optional.
3. Free by default; paid AI only when the user brings a key.
4. Words get explained, sentences get translated.
