# Cấu hình nâng cao

Trong khi Starship là một shell linh hoạt, đôi khi bạn vẫn cần làm nhiều hơn là chỉnh sửa `starship.toml` để có thể là được những việc nhất định. Tài liệu này sẽ mô tả chi tiết các tùy chỉnh nâng cao trong starship.

::: cảnh báo

Các tùy chỉnh được mô tả trong phần này có thể sẽ thay đổi trong các phiên bản tương lai của Starship.

:::

## Tùy chỉnh các hàm được thực thi trước prompt và các lệnh Linux mặc định của bash shell

Bash không có một preexec/precmd framework chính thống giống như các shells khác. Do đó rất khó để cung cấp các hook với khả năng tuỳ biến hoàn toàn cho `bash` shell. Tuy nhiên, Starship cho phép bạn viết các hàm riêng của bạn để tùy biến việc render prompt:

- Để thực thi một hàm custom trước khi prompt được render, ta cần định nghĩa một hàm mới và gán `starship_precmd_user_func` cho tên của hàm này. Ví dụ, để vẽ một tên lửa trước prompt

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Để thực thi một hàm custom trước khi một câu lệnh Linux chạy, ta có thể sử dụng cơ chế bẫy tín hiệu [`DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Tuy nhiên, bạn **phải** đặt bẫy tín hiệu DEBUG *trước* khởi tạo Starship! Starship có thể giữ giá trị của DEBUG trap, nhưng nếu trap bị ghi đè sau khi starship khởi động, một vài chức năng sẽ không hoạt động.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Bẫy DEBUG *trước khi* starship chạy
eval $(starship init bash)
```

## Thay đổi tên gọi trên cửa sổ của chương trình terminal

Một vài shell có khả năng tự động thay đổi tên hiển thị (chẳng hạn như tên của thư mục hiện thời) trên cửa số của trình mô phỏng terminal. Fish shell mặc định thực hiện thay đổi này. Tuy không được set mặc định trên Starship, chức năng này có thể được tích hợp dễ dàng trên `bash` shell và `zsh` shell.

Đầu tiên, ta cần định nghĩa một hàm thay đổi tiêu đề cửa sổ (dùng chung cho cả bash và zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

Ta có thể sử dụng biến số để tuỳ chỉnh tên hiển thị này (`$USER`, `$HOSTNAME`, và `$PWD` là những biến số thường được dùng).

Với `bash` shell, set precmd của starship bằng tên của hàm này:

```bash
starship_precmd_user_func="set_win_title"
```

Với `zsh` shell, thêm hàm này vào mảng `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

Nếu ta hài lòng với các tùy biến đã được thiết lập, thêm những dòng sau vào cấu hình shell (`~/.bashrc` hoặc `~/.zshrc`) để thực thi chúng mỗi khi ta khởi tạo một shell mới.

Ví dụ, nếu ta muốn hiển thị đường dẫn thư mục hiện tại trong tiêu đề của một terminal tab, thêm đoạn code sau vào `~/.bashrc` hoặc `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh.

### Ví dụ

```toml
# ~/.config/starship.toml

# A minimal left prompt
format = """$character"""

# move the rest of the prompt to the right
right_format = """$all"""
```

Produces a prompt like the following:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```


## Các chuỗi kiểu

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `nghiêng`
  - `gạch dưới`
  - `làm tối đi`
  - `đảo ngược`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - Một tròn các màu chuẩn của terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Bạn có thể tuỳ chọn những tiền tố này với `bright` để có được phiên bản sáng hơn (ví dụ: `bright-white`).
 - Một `#` được theo sau bởi một số thập lục phân gồm sáu chữ số. Cái này quy định một [mã thập lục phân cho màu RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Một số nằm giữa 0-255. Cái này quy định một [mã màu ANSI 8-bit](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
