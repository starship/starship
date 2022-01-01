# 進階設定

正因為 Starship 是一個多才多藝的 shell，有時候你必須要做比修改 `starship.toml` 更多事情來讓它完成特定工作。 這個頁面說明了一些用於 Starship 的進階設定技巧。

::: warning

這個章節內的設定可能會隨著未來 Starship 的版本發行而變動。

:::

## Bash 中的自定義預提示 (pre-prompt) 與預執行 (pre-execution) 指令

Bash 不像其他大多的 shell 具有正式的預執行/預指令框架。 因為這個原因，很難在 `bash` 中提供能完全自定義的 hook。 然而，Starship 有提供給你有限的能力來插入你自己的函式到渲染提示字元的程序中：

- 為了在畫出提示字元之前執行一個自定義的函式，請定義一個函式，並將它的名稱放入 `starship_precmd_user_func` 之中。 例如，為了要在提示字元前畫出一個火箭，你就要

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). 然而，你**必須**在初始化 Starship *之前* 對 DEBUG 訊號設下trap！ Starship 可以保留 DEBUG trap 的數值，但是如果該 trap 在 starship 啟動後被被覆寫，某些功能會損壞。

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # 在 Starship 啟用*前*對 DEBUG 訊號設下 trap
eval $(starship init bash)
```

## Custom pre-prompt and pre-execution Commands in PowerShell

PowerShell does not have a formal preexec/precmd framework like most other shells. Because of this, it is difficult to provide fully customizable hooks in `powershell`. 然而，Starship 有提供給你有限的能力來插入你自己的函式到渲染提示字元的程序中：

Create a function named `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## 改變視窗標題

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish 甚至預設就會這樣做。 Starship 沒有幫你這樣做，但是可以用直覺的方式加入這個功能到 `bash` 或 `zsh` 之中。

首先，定義一個改變視窗標題的函式（在 bash 與 zsh 之中都一樣）：

```bash
function set_win_title(){
    echo -ne "\033]0; 你的標題在此 \007"
}
```

你可以利用變數來自定義這個標題（`$USER`、`$HOSTNAME` 與 `$PWD` 是很受歡迎的選項）。

在 `bash` 中，將這個函式設定為 Starship 的預執行函式：

```bash
starship_precmd_user_func="set_win_title"
```

在 `zsh` 中，將這個函式加入 `precmd_functions` 陣列：

```bash
precmd_functions+=(set_win_title)
```

如果你喜歡這個結果，把這幾行加入你的 shell 設定檔中(`~/.bashrc` or `~/.zsrhc`)來將此設為永久設定。

For example, if you want to display your current directory in your terminal tab title, add the following snippet to your `~/.bashrc` or `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
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

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh.

### 範例

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

### 範例

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled in arrows
continuation_prompt = "▶▶"
```

## 風格字串

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
