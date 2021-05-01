# Cấu hình nâng cao

Trong khi Starship là một shell linh hoạt, đôi khi bạn vẫn cần làm nhiều hơn là chỉnh sửa `starship.toml` để có thể là được những việc nhất định. Trang này chi tiết một vài cấu hình kĩ thuật nâng cao hơn được sử dụng trong starship.

::: cảnh báo

Các cấu hình trong phần này có thể thay đổi trong các bản phát hành Starship trong tương lai.

:::

## Tuỳ biến pre-prompt và pre-execution Commands trong Bash

Bash không có một preexec/precmd framwork chính thống giống như các shells khác. Bởi vì điều này, nó là khó để cung cấp các hook cho việc tuỳ biến được đầy đủ trong `bash`. Tuy nhiên, Starship cung cấp cho bạn khả năng hạn chế để chèn các chức năng của riêng bạn vào trong thủ tục prompt-rendering:

- Để chạy một hàm tuỳ biến trước khi prompt được vẽ ra, định nghĩa một hàm mới và sau đó gán tên của nó tới `starship_precmd_user_func`. Ví dụ, để vẽ một tên lửa trước prompt, bạn sẽ làm

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Tuy nhiên, bạn **phải** đặt bẫy tín hiệu DEBUG *trước* khởi tạo Starship! Starship có thể giữ giá trị của DEBUG trap, nhưng nếu trap được ghi đè sau khi starship khởi động, một vài chức năng sẽ không hoạt động.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Bẫy DEBUG *trước khi* starship chạy
eval $(starship init bash)
```

## Thay đổi tiêu đề của sổ

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish thậm chí là nó một cách mặc định. Starship không làm điều này, nhưng nó khá đơn giản để thêm điều này vào chức năng cho `bash` hoặc `zsh`.

Đầu tiên, định nghĩa một hàm thay đổi tiêu đề cửa sổ (giống hệt trong bash và zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

Bạn có thể tuỳ biến để tuỳ biến tiêu đề này (`$USER`, `$HOSTNAME`, và `$PWD` là những lựa chọn phổ biến).

Trong `bash`, thiết lập hàm này thành hàm precmd của starship:

```bash
starship_precmd_user_func="set_win_title"
```

Trong `zsh`, thêm cái này vào mảng `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zshrc`) to make it permanent.

Ví dụ, nếu bạn muốn hiển thị đường dẫn hiện tại của bạn trong tiêu đề tab terminal, thêm snippet sau vào `~/.bashrc` hoặc `~/.zshrc` của bạn:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

## Các chuỗi kiểu

Chuỗi kiểu là một danh sách các từ, được phân cách bởi khoảng trắng. Các từ là không phân biệt hoa thường (ví dụ. `bold` và `Bold` là hai chuỗi tương đương). Mỗi từ có thể là một trong các từ sau:

  - `bold`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

`<color>` là một nơi quy định màu (được bàn luận ở phía dưới). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. Thứ tự các từ trong chuỗi là không quan trọng.

Từ mã `none` ghi đè tất cả các từ mã khác trong chuỗi nếu nó không là một phần của `bg:` specifier, vậy nên `fg:red none fg:blue` sẽ vẫn tạo một chuỗi mà không có kiểu. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. Nó có thể trở thành một lỗi để sử dụng `none` trong việc kết hợp với các từ mã khác trong tương lai.

Một quy định màu có thể là một trong các thứ sau:

 - Một tròn các màu chuẩn của terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Bạn có thể tuỳ chọn những tiền tố này với `bright` để có được phiên bản sáng hơn (ví dụ: `bright-white`).
 - Một `#` được theo sau bởi một số thập lục phân gồm sáu chữ số. Cái này quy định một [mã thập lục phân cho màu RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Một số nằm giữa 0-255. Cái này quy định một [mã màu ANSI 8-bit](https://i.stack.imgur.com/KTSQa.png).

Nếu nhiều màu được quy định cho màu chữ/màu nền, cái cuối cùng trong chuỗi sẽ được ưu tiên.
