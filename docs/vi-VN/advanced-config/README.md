# Cấu hình nâng cao

Trong khi Starship là một shell linh hoạt, đôi khi bạn vẫn cần làm nhiều hơn là chỉnh sửa `starship.toml` để có thể là được những việc nhất định. Tài liệu này sẽ mô tả chi tiết các tùy chỉnh nâng cao trong starship.

::: cảnh báo

Các tùy chỉnh được mô tả trong phần này có thể sẽ thay đổi trong các phiên bản tương lai của Starship.

:::

## TransientPrompt in PowerShell

It is possible to replace the previous-printed prompt with a custom string. This is useful in cases where all the prompt information is not always needed. To enable this, run `Enable-TransientPrompt` in the shell session. To make it permanent, put this statement in your `$PROFILE`. Transience can be disabled on-the-fly with `Disable-TransientPrompt`.

By default, the left side of input gets replaced with `>`. To customize this, define a new function called `Invoke-Starship-TransientFunction`. For example, to display Starship's `character` module here, you would do

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt and TransientRightPrompt in Cmd

Clink allows you to replace the previous-printed prompt with custom strings. This is useful in cases where all the prompt information is not always needed. To enable this, run `clink set prompt.transient <value>` where \<value\> can be one of:

- `always`: always replace the previous prompt
- `same_dir`: replace the previous prompt only if the working directory is same
- `off`: do not replace the prompt (i.e. turn off transience)

You need to do this only once. Make the following changes to your `starship.lua` to customize what gets displayed on the left and on the right:

- By default, the left side of input gets replaced with `>`. To customize this, define a new function called `starship_transient_prompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display Starship's `character` module here, you would do

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display the time at which the last command was started here, you would do

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## Custom pre-prompt and pre-execution Commands in Cmd

Clink provides extremely flexible APIs to run pre-prompt and pre-exec commands in Cmd shell. It is fairly simple to use with Starship. Make the following changes to your `starship.lua` file as per your requirements:

- To run a custom function right before the prompt is drawn, define a new function called `starship_preprompt_user_func`. This function receives the current prompt as a string that you can utilize. For example, to draw a rocket before the prompt, you would do

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- To run a custom function right before a command is executed, define a new function called `starship_precmd_user_func`. This function receives the current commandline as a string that you can utilize. For example, to print the command that's about to be executed, you would do

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Tùy chỉnh các hàm được thực thi trước prompt và các lệnh Linux mặc định của bash shell

Bash không có một preexec/precmd framework chính thống giống như các shells khác. Do đó rất khó để cung cấp các hook với khả năng tuỳ biến hoàn toàn cho `bash` shell. Tuy nhiên, Starship cho phép bạn viết các hàm riêng của bạn để tùy biến việc render prompt:

- Để thực thi một hàm custom trước khi prompt được render, ta cần định nghĩa một hàm mới và gán `starship_precmd_user_func` cho tên của hàm này. Ví dụ, để vẽ một tên lửa trước prompt

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Để thực thi một hàm custom trước khi một câu lệnh Linux chạy, ta có thể sử dụng cơ chế bẫy tín hiệu [`DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Tuy nhiên, bạn **phải** đặt bẫy tín hiệu DEBUG _trước_ khởi tạo Starship! Starship có thể giữ giá trị của DEBUG trap, nhưng nếu trap bị ghi đè sau khi starship khởi động, một vài chức năng sẽ không hoạt động.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Custom pre-prompt and pre-execution Commands in PowerShell

PowerShell does not have a formal preexec/precmd framework like most other shells. Because of this, it is difficult to provide fully customizable hooks in `powershell`. Tuy nhiên, Starship cho phép bạn viết các hàm riêng của bạn để tùy biến việc render prompt:

Create a function named `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Thay đổi tên gọi trên cửa sổ của chương trình terminal

Một vài shell có khả năng tự động thay đổi tên hiển thị (chẳng hạn như tên của thư mục hiện thời) trên cửa số của trình mô phỏng terminal. Fish shell mặc định thực hiện thay đổi này. Starship does not do this, but it's fairly straightforward to add this functionality to `bash`, `zsh`, `cmd` or `powershell`.

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

For Cmd, you can change the window title using the `starship_preprompt_user_func` function.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

You can also set a similar output with PowerShell by creating a function named `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [`fill` module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd.

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

## Continuation Prompt

Some shells support a continuation prompt along with the normal prompt. This prompt is rendered instead of the normal prompt when the user has entered an incomplete statement (such as a single left parenthesis or quote).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `"[∙](bright-black) "`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Note: Continuation prompts are only available in the following shells:

- `bash`
- `zsh`
- `PowerShell`

### Ví dụ

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled in arrows
continuation_prompt = "▶▶"
```

## Các chuỗi kiểu

Chuỗi kiểu là một danh sách các từ, được phân cách bởi khoảng trắng. Các từ là không phân biệt hoa thường (ví dụ. `bold` và `Bold` là hai chuỗi tương đương). Mỗi từ có thể là một trong các từ sau:

- `bold`
- `nghiêng`
- `underline`
- `dimmed`
- `đảo ngược`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

`<color>` là một nơi quy định màu (được bàn luận ở phía dưới). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. Thứ tự các từ trong chuỗi là không quan trọng.

Từ mã `none` ghi đè tất cả các từ mã khác trong chuỗi nếu nó không là một phần của `bg:` specifier, vậy nên `fg:red none fg:blue` sẽ vẫn tạo một chuỗi mà không có kiểu. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. Nó có thể trở thành một lỗi để sử dụng `none` trong việc kết hợp với các từ mã khác trong tương lai.

Một quy định màu có thể là một trong các thứ sau:

- One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
- Một `#` được theo sau bởi một số thập lục phân gồm sáu chữ số. Cái này quy định một [mã thập lục phân cho màu RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Một số nằm giữa 0-255. Cái này quy định một [mã màu ANSI 8-bit](https://i.stack.imgur.com/KTSQa.png).

Nếu nhiều màu được quy định cho màu chữ/màu nền, cái cuối cùng trong chuỗi sẽ được ưu tiên.

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default
- `hidden` is not supported on iTerm (https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app
