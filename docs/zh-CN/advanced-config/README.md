# 高级配置

Starship 功能繁多，有时您必须在编辑 `starship.toml` 之外做更多工作才能实现某些效果。 此页面详细介绍了一些在 starship 中使用的高级配置技巧。

::: warning

本节所述的配置内容可能随 Starship 未来版本的更新而改变。

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

## 在 Cmd 中自定义提示符显示前和执行前的命令

Clink 提供了很灵活的 API，能在 Cmd shell 中运行预提示和执行前命令。 在 Starship 中使用这些 API 很容易。 对你的 `starship.lua` 按需做出如下修改：

- 为了在提示符显示前运行一个自定义函数，你需要定义一个名为 `starship_preprompt_user_func` 的函数。 这个函数接受当前的提示符作为字符串参数，你可以在函数中使用它。 例如，如果想在提示符前绘制一个火箭，可以这样写：

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- 为了在命令执行前运行一个自定义函数，你需要定义一个名为 `starship_precmd_user_func` 的函数。 这个函数接受当前的命令行内容作为字符串参数，同样，你可以在函数中使用它。 例如，要打印即将被执行的命令，可以这样写：

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## 在 Bash 中自定义预提示和预执行命令

Bash 并没有类似大多数其它 shell 的正式预执行/预命令框架。 因此，很难在 `bash` 中提供完全可自定义的 hook 机制。 然而，Starship 确实能使您有限地在提示符渲染过程中插入自己的函数执行：

- 若要在提示符显示之前运行自定义函数，需要定义此函数，然后将函数名赋值给 `starship_reserved_user_func`。 例如，要在提示符之前绘制一枚火箭，您应该写

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- 要在一个命令运行前运行自定义函数，您可以使用 [`DEBUG` trap 机制](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/)。 然而，您**必须**在捕捉 DEBUG 信号_之前_启动 Starship！ Starship 可以保留 DEBUG trap 的值，但如果该 trap 在 starship 启动后被覆盖，一些功能将会被破坏。

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## 在 Powershell 中自定义提示符显示前和执行前的命令

Powershell 不像多数其他的 Shell 有成体系的预执行框架。 因此，很难在 `Powershell` 中提供完全可自定义的 hook 机制。 然而，Starship 确实能使您有限地在提示符渲染过程中插入自己的函数执行：

创建一个名为 `Invoke-Starship-PreCommand` 的函数

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## 更改窗口标题

一些 shell 会自动更改您的窗口标题（比如改成您的工作目录）。 Fish 甚至默认会执行此功能。 Starship 没有实现此功能，但将这个功能添加到 `bash`、`zsh`、`cmd` 或 `powershell` 是相当简单的。

首先，定义窗口标题更改函数（在 bash 和 zsh 中相同）：

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

您可以使用变量来定制标题（常用的有 `$USER`，`$HOSTNAME` 和 `$PWD`）。

在 `bash` 中，设置此函数为 starship 预执行函数：

```bash
starship_precmd_user_func="set_win_title"
```

在 `zsh`中，将此函数添加到 `reservmd_functions` 列表：

```bash
precmd_functions+=(set_win_title)
```

如果您对产生的效果感到满意，请将以上代码添加到您的 shell 配置文件（`~/.bashrc` 或 `~/zsrhc`）中以使其永久化。

例如，如果您想要在终端标签标题中显示当前目录， 将以下代码添加到您的 `~/.ashrc` 或 `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

对于 Cmd，您可以使用 `starship_preprompt_user_func` 函数修改窗口标题。

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

您也可以为 Powershell 创建一个函数 `Invoke-Starship-PreCommand`，来设置类似的输出。

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## 启用右侧提示

一些 Shell 支持右侧提示, 它与输入区渲染在同一行。 Starship 可以设置右侧提示的内容，使用 `right_format` 选项。 所有支持 `format` 的组件也同时支持 `right_format`。 未显式在 `format` 或 `right_format` 中使用的组件，会保存在变量 `$all` 中。

注意：右侧提示和输入区显示在同一行。 如果需要在输入区的上方显示右对齐的组件，请查阅 [`fill` 组件](/config/#fill)。

`right_format` 现支持 elvish、fish、zsh、xonsh、cmd。

### 示例

```toml
# ~/.config/starship.toml

# A minimal left prompt
format = """$character"""

# move the rest of the prompt to the right
right_format = """$all"""
```

会显示成如下方案

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## 多行提示符

一些 Shell 也同时支持多行提示符。 若用户输入了不完整的命令（例如一个左括号或引号），Shell 会渲染多行提示符。

使用 `continuation_prompt` 选项来设置 Starship 的多行提示符。 它的默认值为 `[∙](bright-black)`。

注意：`continuation_prompt` 应设置为没有变量的字符串。

注意，仅以下 Shell 支持多行提示符：

- `bash`
- `zsh`
- `PowerShell`

### 示例

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled in arrows
continuation_prompt = "▶▶"
```

## 样式设定

样式字符串是用空格分隔的单词列表。 其中单词不是大小写敏感的（例如 `bold` 和 `BoLd` 被视为同一字符串）。 每个单词可以是以下之一：

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

`<color>` 是颜色说明符（下面解释）。 `fg:<color>` 和 `<color>` 的功能暂时相同，未来可能会更改。 `inverted` 会反转背景和文字的颜色。 字符串中的单词顺序不影响显示结果。

若 `none` 不是 `bg:` 的一部分，则它会覆盖其他的设置：比如 `fg:red none fg:blue` 不会更改任何样式。 `bg:none` 会设置成默认背景色，因此 `fg:red bg:none`、`red`、`fg:red` 的作用相同；类似，`bg:green fg:red bg:none`、`fg:red`、`red` 的作用也相同。 未来可能会将 `none` 与其它标识符一起使用视为一种错误。

颜色说明符可以是以下内容之一：

- 任一标准的终端颜色：`black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`。 您也可以使用前缀 `bright-` 定义浅色版本（例如 `bright-white`）。
- 一个 `#` 后跟一个六位十六进制数。 这将指定一个 [十六进制 RGB 颜色代码](https://www.w3schools.com/colors/colors_hexadecimal.asp)。
- 0-255 之间的数字。 这将指定一个 [8 位 ANSI 颜色码](https://i.stack.imgur.com/KTSQa.png)。

如果为文本/背景指定了多个颜色，字符串中最后指定的颜色将具有最高优先级。

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default
- `hidden` is not supported on iTerm (https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app
