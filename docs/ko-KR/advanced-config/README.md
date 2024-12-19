# 고급 설정

While Starship is a versatile shell, sometimes you need to do more than edit `starship.toml` to get it to do certain things. This page details some of the more advanced configuration techniques used in starship.

::: warning

The configurations in this section are subject to change in future releases of Starship.

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

## TransientPrompt and TransientRightPrompt in Fish

It is possible to replace the previous-printed prompt with a custom string. This is useful in cases where all the prompt information is not always needed. To enable this, run `enable_transience` in the shell session. To make it permanent, put this statement in your `~/.config/fish/config.fish`. Transience can be disabled on-the-fly with `disable_transience`.

Note that in case of Fish, the transient prompt is only printed if the commandline is non-empty, and syntactically correct.

- By default, the left side of input gets replaced with a bold-green `❯`. To customize this, define a new function called `starship_transient_prompt_func`. For example, to display Starship's `character` module here, you would do

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. For example, to display the time at which the last command was started here, you would do

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt and TransientRightPrompt in Bash

The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework at v0.4 or higher allows you to replace the previous-printed prompt with custom strings. This is useful in cases where all the prompt information is not always needed. To enable this, put this in `~/.bashrc` `bleopt prompt_ps1_transient=<value>`:

The \<value\> here is a colon-separated list of `always`, `same-dir` and `trim`. When `prompt_ps1_final` is empty and the option `prompt_ps1_transient` has a non-empty \<value\>, the prompt specified by `PS1` is erased on leaving the current command line. If \<value\> contains a field `trim`, only the last line of multiline `PS1` is preserved and the other lines are erased. Otherwise, the command line will be redrawn as if `PS1=` is specified. When a field `same-dir` is contained in \<value\> and the current working directory is different from the final directory of the previous command line, this option `prompt_ps1_transient` is ignored.

Make the following changes to your `~/.blerc` (or in `~/.config/blesh/init.sh`) to customize what gets displayed on the left and on the right:

- To customize what the left side of input gets replaced with, configure the `prompt_ps1_final` Ble.sh option. For example, to display Starship's `character` module here, you would do

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- To customize what the right side of input gets replaced with, configure the `prompt_rps1_final` Ble.sh option. For example, to display the time at which the last command was started here, you would do

```bash
bleopt prompt_rps1_final='$(starship module time)'
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

## Custom pre-prompt and pre-execution Commands in Bash

Bash does not have a formal preexec/precmd framework like most other shells. Because of this, it is difficult to provide fully customizable hooks in `bash`. However, Starship does give you limited ability to insert your own functions into the prompt-rendering procedure:

- To run a custom function right before the prompt is drawn, define a new function and then assign its name to `starship_precmd_user_func`. For example, to draw a rocket before the prompt, you would do

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). However, you **must** trap the DEBUG signal _before_ initializing Starship! Starship can preserve the value of the DEBUG trap, but if the trap is overwritten after starship starts up, some functionality will break.

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

PowerShell does not have a formal preexec/precmd framework like most other shells. Because of this, it is difficult to provide fully customizable hooks in `powershell`. However, Starship does give you limited ability to insert your own functions into the prompt-rendering procedure:

Create a function named `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Change Window Title

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish even does it by default. Starship does not do this, but it's fairly straightforward to add this functionality to `bash`, `zsh`, `cmd` or `powershell`.

First, define a window title change function (identical in bash and zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

You can use variables to customize this title (`$USER`, `$HOSTNAME`, and `$PWD` are popular choices).

In `bash`, set this function to be the precmd starship function:

```bash
starship_precmd_user_func="set_win_title"
```

In `zsh`, add this to the `precmd_functions` array:

```bash
precmd_functions+=(set_win_title)
```

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zshrc`) to make it permanent.

For example, if you want to display your current directory in your terminal tab title, add the following snippet to your `~/.bashrc` or `~/.zshrc`:

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
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## 오른쪽 프롬프트 활성화

일부 셸은 입력과 같은 줄에 렌더링되는 오른쪽 프롬프트를 지원합니다. Starship에서는 `right_format` 옵션을 사용하여 오른쪽 프롬프트의 내용을 설정할 수 있습니다. `format`에서 사용할 수 있는 모든 모듈은 `right_format`에서도 지원됩니다. `$all` 변수는 `format` 또는 `right_format`에서 명시적으로 사용하지 않는 모듈만 포함합니다.

알림: 오른쪽 프롬프트는 입력 위치에 따라 한 줄로 표시됩니다. To right align modules above the input line in a multi-line prompt, see the [`fill` module](../config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Note: The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework v0.4 or higher should be installed in order to use right prompt in bash.

### 예시

```toml
# ~/.config/starship.toml

# 간결한 왼쪽 프롬프트
format = """$character"""

# 프롬프트의 나머지를 오른쪽으로 옮기기
right_format = """$all"""
```

위 설정은 아래와 같은 프롬프트를 출력합니다:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Continuation Prompt

Some shells support a continuation prompt along with the normal prompt. This prompt is rendered instead of the normal prompt when the user has entered an incomplete statement (such as a single left parenthesis or quote).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `'[∙](bright-black) '`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Note: Continuation prompts are only available in the following shells:

- `bash`
- `zsh`
- `Powershell`

### 예시

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled-in arrows
continuation_prompt = '▶▶ '
```

## 스타일 문자열

스타일 문자열은 공백으로 구분된 단어 목록입니다. 단어는 대소문자를 구분하지 않습니다 (즉, `bold`와 `BoLd`는 동일한 문자열로 간주됩니다). 각 단어는 다음 중 하나가 될 수 있습니다:

- `bold`
- `italic`
- `underline`
- `dimmed`
- `inverted`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

`<color>` 부분은 색상 지정자입니다 (아래에 후술). 현재, `fg:<color>` 와 `<color>`는 동일한 동작을 하지만 차후에 바뀔 수 있습니다. `<color>` can also be set to `prev_fg` or `prev_bg` which evaluates to the previous item's foreground or background color respectively if available or `none` otherwise. `inverted`는 배경 색과 전경 색을 서로 바꿉니다. 문자열의 단어 순서는 중요하지 않습니다.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. 향후 다른 토큰과 함께 `none`을 사용하는 것은 오류가 발생할 수 있습니다.

색상 지정자는 다음 중 하나가 될 수 있습니다:

- 표준 터미널 색상: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. 앞에 `bright-`를 붙여 밝게 만들 수도 있습니다 (예시: `bright-white`).
- `#` 다음의 여섯 자리 16진수 숫자. 이는 [RGB 색상 16진수 코드](https://www.w3schools.com/colors/colors_hexadecimal.asp)입니다.
- 0~255 사이의 숫자. 이는 [8비트 ANSI 색상 코드](https://i.stack.imgur.com/KTSQa.png)입니다.

If multiple colors are specified for foreground/background, the last one in the string will take priority.

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default.
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app.
