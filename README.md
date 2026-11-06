# I.R.I.S

App nhỏ chạy ở khay hệ thống Windows để dịch nhanh chữ trên màn hình, giữa tiếng Nhật, tiếng Anh và tiếng Việt.

- **Bôi đen rồi bấm `Ctrl+Alt+D`**: bản dịch hiện ngay dưới đoạn chữ, không che chữ đang đọc.
- **Nút nhỏ cạnh vùng bôi đen**: bôi đen bằng chuột sẽ hiện một nút nhỏ, bấm vào là dịch. Tắt được trong Cài đặt.
- **Khoanh vùng màn hình bằng `Ctrl+Alt+S`**: kéo khung quanh chữ trong ảnh, game, video hoặc PDF scan. App đọc chữ bằng OCR có sẵn của Windows rồi dịch.
- **Tra từ**: nếu chọn đúng một từ, popup hiện phiên âm (kana/romaji cho tiếng Nhật, IPA cho tiếng Anh) và nghĩa theo từ loại.
- **Dịch bằng Claude (tùy chọn)**: nhập API key của Anthropic trong Cài đặt để dịch câu và đoạn văn theo ngữ cảnh. Không có key thì app dùng Google Translate miễn phí.

Bấm `Esc` hoặc bấm ra ngoài để đóng popup. Nút ghim giữ popup lại khi bạn bấm sang chỗ khác.

## Cài đặt

Tải bộ cài `I.R.I.S_x.y.z_x64-setup.exe` trong mục **Actions → Build Windows installer → Artifacts** của repo, chạy file, xong app nằm ở khay hệ thống (góc phải thanh taskbar). Bấm vào biểu tượng để mở Cài đặt.

Muốn đọc chữ tiếng Nhật trong ảnh, Windows cần gói ngôn ngữ tiếng Nhật: Settings → Time & language → Language & region → Add a language → 日本語. Không cần đổi ngôn ngữ hiển thị.

## Tự build

Cần Windows 10/11, [Node.js](https://nodejs.org) 20+, [Rust](https://rustup.rs) và Visual Studio Build Tools (C++).

```bash
npm install
```

```bash
npm run dev
```

```bash
npm run build
```

Bộ cài nằm ở `src-tauri/target/release/bundle/nsis/`.

Chạy test:

```bash
cd src-tauri && cargo test
```

## Cấu trúc

| Đường dẫn | Nội dung |
|---|---|
| `src-tauri/src/main.rs` | Khay hệ thống, phím tắt, các cửa sổ, cài đặt |
| `src-tauri/src/translate.rs` | Gọi Google Translate, từ điển (Jisho, dictionaryapi.dev) và Claude |
| `src-tauri/src/win.rs` | Phần riêng của Windows: lấy chữ đang chọn, theo dõi thao tác bôi đen, OCR |
| `src-tauri/src/text.rs` | Ghép chữ từ OCR, chọn ngôn ngữ đích |
| `src/` | Giao diện: popup, màn khoanh vùng, nút nổi, cài đặt |

## Lưu ý

- Để lấy chữ đang bôi đen, app giả lập `Ctrl+C` rồi trả lại nội dung clipboard cũ. Clipboard chứa chữ hoặc ảnh sẽ được giữ nguyên; clipboard chứa file hoặc định dạng khác sẽ bị thay bằng đoạn chữ vừa chọn.
- API key của Claude được cất trong Windows Credential Manager, không ghi ra file.
- Font Be Vietnam Pro được đóng gói kèm theo giấy phép SIL Open Font License (`src/fonts/OFL.txt`).
