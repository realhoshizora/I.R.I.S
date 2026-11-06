# I.R.I.S: tổng kết

App chạy nền ở khay hệ thống Windows, dịch nhanh chữ trên màn hình giữa tiếng Nhật, tiếng Anh và tiếng Việt. Giao diện theo phong cách **phụ đề song ngữ**: dải nền tối, câu gốc nhỏ màu trắng, bản dịch màu vàng, mỗi ngôn ngữ một màu (JA đỏ son, EN xanh, VI vàng).

## Trạng thái

| Hạng mục | Kết quả |
|---|---|
| Biên dịch cho Windows (`cargo check --target x86_64-pc-windows-msvc`) | Đạt |
| Test (ghép chữ OCR, chọn ngôn ngữ đích, đọc kết quả Google) | 6/6 đạt |
| Review giao diện độc lập | Đạt ("ship") sau một vòng sửa |
| Chạy thử trên Windows thật | **Chưa làm** |
| Đẩy code lên GitHub | **Chưa được**: Claude chưa có quyền ghi vào repo |

## Tính năng

- **Chạy nền ở khay hệ thống**, có tùy chọn mở cùng Windows. Bấm vào biểu tượng để mở Cài đặt.
- **`Ctrl+Alt+D`**: dịch chữ đang bôi đen.
- **Nút nổi**: bôi đen bằng chuột sẽ hiện một nút nhỏ cạnh chữ, bấm vào là dịch, tự ẩn sau khoảng 3 giây. Tắt được trong Cài đặt.
- **`Ctrl+Alt+S`**: màn hình tối đi, kéo khung quanh chữ trong ảnh, game, video hoặc PDF scan. App đọc chữ bằng OCR có sẵn của Windows rồi dịch.
- **Tra từ**: chọn đúng một từ thì popup hiện phiên âm (kana/romaji cho tiếng Nhật, IPA cho tiếng Anh) và nghĩa theo từ loại.
- **Không che chữ đang đọc**: popup nằm dưới vùng chọn, hết chỗ thì lên trên. Nếu cả hai phía đều chật, bản dịch cuộn bên trong popup.
- **Popup**: sao chép bản dịch, ghim (không tự đóng khi bấm ra ngoài), đóng bằng `Esc`.
- **Dịch bằng Claude (tùy chọn)**:
  - Mặc định dùng Google Translate miễn phí. Nhập API key của Anthropic trong Cài đặt thì câu và đoạn văn sẽ dịch bằng Claude.
  - Model mặc định `claude-opus-5-5`, mức suy nghĩ thấp để trả lời nhanh. Muốn rẻ hơn, đổi sang `claude-haiku-5-5` trong Cài đặt.
  - Đã bật chế độ dự phòng: nếu bộ lọc an toàn từ chối một đoạn, Anthropic tự chạy lại trên model khác.
  - API key được cất trong Windows Credential Manager, không ghi ra file.

## Phím tắt

| Phím | Việc |
|---|---|
| `Ctrl+Alt+D` | Dịch chữ đang bôi đen |
| `Ctrl+Alt+S` | Khoanh vùng màn hình để dịch |
| `Esc` | Đóng popup, hủy khoanh vùng |

## Việc bạn cần làm

1. Kết nối lại GitHub tại https://claude.ai/connect-github.
2. Cài Claude GitHub App cho repo `realhoshizora/I.R.I.S` tại https://github.com/apps/claude/installations/select_target (repo thuộc tổ chức thì người quản trị cài giúp).
3. Báo để đẩy các commit trên nhánh `claude/festive-heisenberg-culz3g` lên GitHub.
4. GitHub Actions sẽ tự đóng gói bộ cài. Tải file `.exe` ở **Actions → Build Windows installer → Artifacts**, chạy để cài.
5. Muốn đọc chữ tiếng Nhật trong ảnh: Settings → Time & language → Language & region → Add a language → 日本語. Không cần đổi ngôn ngữ hiển thị.

## Tự build (không cần GitHub Actions)

Cần Windows 10/11, Node.js 20+, Rust và Visual Studio Build Tools (C++).

```bash
npm install
```

```bash
npm run build
```

Bộ cài nằm ở `src-tauri/target/release/bundle/nsis/`.

## Chưa kiểm chứng và rủi ro

- **Chưa chạy trên Windows thật**: phím tắt, sao chép chữ bôi đen, nút nổi, OCR và vị trí popup mới chỉ kiểm tra bằng biên dịch và ảnh chụp giao diện mô phỏng.
- **Google và các trang từ điển (Jisho, dictionaryapi.dev) chặn máy chủ dùng để làm app**, nên phần đọc kết quả được viết theo định dạng đã biết và kiểm tra bằng dữ liệu mẫu, chưa gọi thật.
- **Chưa gọi thử Claude API.**
- **Clipboard**: để lấy chữ bôi đen, app giả lập `Ctrl+C` rồi trả lại clipboard cũ. Chữ và ảnh được giữ nguyên; file hoặc định dạng đặc biệt sẽ bị thay bằng đoạn chữ vừa chọn.
- **Bôi đen bằng bàn phím**: popup neo theo con trỏ nhập liệu. Một số app không có con trỏ nhập liệu hệ thống thì popup neo theo chuột và có thể đè lên chữ.
- **Giao diện chỉ có nền tối**, không đổi theo theme sáng của Windows (chủ ý của phong cách phụ đề).

## Đề xuất cho bản sau

- Sổ từ vựng: lưu từ đã tra, ôn bằng flashcard nhắc lại ngắt quãng.
- Dịch liên tục một vùng: ghim vùng phụ đề video hoặc hội thoại game, tự dịch mỗi khi chữ đổi.
- Đọc to câu hoặc từ.
- Lịch sử những gì đã dịch.
- Âm Hán Việt cho kanji (食 = THỰC).
- Đổi phím tắt trong Cài đặt.

## Cấu trúc code

| Đường dẫn | Nội dung |
|---|---|
| `src-tauri/src/main.rs` | Khay hệ thống, phím tắt, các cửa sổ, vị trí popup, cài đặt |
| `src-tauri/src/translate.rs` | Google Translate, từ điển, Claude |
| `src-tauri/src/win.rs` | Riêng Windows: lấy chữ bôi đen, theo dõi chuột, con trỏ nhập liệu, OCR |
| `src-tauri/src/text.rs` | Ghép chữ từ OCR, chọn ngôn ngữ đích |
| `src/` | Giao diện: popup, màn khoanh vùng, nút nổi, cài đặt |
| `PRODUCT.md` | Thông tin sản phẩm |
| `DESIGN.md` | Màu, font, quy tắc của phong cách phụ đề |
| `README.md` | Hướng dẫn cài và build |
