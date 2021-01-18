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

- Để chạy một hàm tuỳ biến trước khi một câu lệnh chạy, bạn có thể sử dụng cơ chế bẫy [`DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Tuy nhiên, bạn **phải** đặt bẫy tín hiệu DEBUG *trước* khởi tạo Starship! Starship có thể giữ giá trị của DEBUG trap, nhưng nếu trap được ghi đè sau khi starship khởi động, một vài chức năng sẽ không hoạt động.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Bẫy DEBUG *trước khi* starship chạy
eval $(starship init bash)
```

## Thay đổi tiêu đề của sổ

Một vào prompts sẽ tự động thay đổi tiêu đề cửa sổ cho bạn (ví dụ phản ánh thư mục hiện hành của bạn). Fish thậm chí là nó một cách mặc định. Starship không làm điều này, nhưng nó khá đơn giản để thêm điều này vào chức năng cho `bash` hoặc `zsh`.

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

Nếu bạn thích thành quả, thêm những giòng này vào tập tin cấu hình shell của bạn (`~/.bashrc` or `~/.zshrc`) để cấu hình nó vĩnh viễn.

For example, if you want to display your current directory in your terminal tab title, add the following snippet to your `~/.bashrc` or `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename $PWD) \007"
}
starship_precmd_user_func="set_win_title"
```

## Style Strings

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `underline`
  - `dimmed`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing , though this may change in the future. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none`  sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
