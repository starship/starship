# 高级配置

Starship 功能繁多，有时您必须在编辑 `starship.toml` 之外做更多工作才能实现某些效果。 此页面详细介绍了一些在 starship 中使用的高级配置技巧。

::: warning

本节所述的配置内容可能随 Starship 未来版本的更新而改变。

:::

## PowerShell中的命令行提示(TransientPrompt)

可以用自定义字符串替换预设的命令行提示。 这在不经常需要所有提示信息的情况下很有用。 若要启用该功能，请在 shell 中运行 `Enable-TransitientPrompt`命令 若要永久启用该功能，请将 上述语句放在您的 `$PROFILE` 中。 通过在shell中运行 `Disable-TransientPrompt`命令来禁用这项功能。

默认情况下，输入的左侧是 `>`符号。 要自定义它，请定义一个新函数，名为 `Invoke-Starship-TransitentFunction`。 For example, to display Starship's `character` module here, you would do

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## 在 Cmd 中的 命令行提示(TransitentPrompt )和 语法高亮(TransentRightPrompt)

可以用自定义字符串替换预设的命令行提示。 这在不经常需要所有提示信息的情况下很有用。 要启用该功能，运行命令`clink set prompt.transient <value>`，&lt;prompt.transient>之后跟以下单词中的一个：

- `always`: 总是预设的提示
- `same_dir`: 仅在工作目录相同的情况下替换预设的提示
- `off`: 不要替换提示(即关闭命令行提示)

你只需要这样做一次。 对您的 `starship.lua` 进行以下更改，以自定义左侧和右侧显示的内容：

- 默认情况下，输入的左侧是 `>`符号。 要自定义它，请定义一个新函数，名为 `Invoke-Starship-TransitentFunction`。 This function receives the current prompt as a string that you can utilize. 例如，要 在这里显示Starship的 `character` 模块，您需要如下操作：

```lua
function starship_transitent_propt_func(empt)
  return io.popen("starship module character"
    ..." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))
```

- 默认情况下，输入的右侧为空。 要自定义它，请定义一个新函数，名为 `Invoke-Starship-TransitentFunction`。 This function receives the current prompt as a string that you can utilize. 例如，要在这里显示 最后一个命令开始的时间，您需要如下操作：

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## 在 Fish 中的 命令行提示(TransitentPrompt)和语法高亮(TransentRightPrompt)

可以用自定义字符串替换预设的命令行提示。 这在不经常需要所有提示信息的情况下很有用。 若要启用该功能，请在 shell 中运行 `Enable-TransitientPrompt`命令 若要永久启用该功能，请将 上述语句放在您的 `~/.config/fish/config.fish` 中。 通过在shell中运行 `Disable-TransientPrompt`命令来禁用这项功能。

请注意，对于Fish，命令行提示只在命令行非空 和语法正确的情况下才会显示。

- 默认情况下，输入的左侧是 粗体绿色的❯</code>符号。 要自定义它，请定义一个新函数，名为 `Invoke-Starship-TransitentFunction`。 例如，要 在这里显示Starship的 `character` 模块，您需要如下操作：

```fish
function starship_transent_rmpt_func
  starship module time
end
starship init fish | source
enable_transience
```

- 默认情况下，输入的右侧为空。 要自定义它，请定义一个新函数，名为 `Invoke-Starship-TransitentFunction`。 例如，要在这里显示 最后一个命令开始的时间，您需要如下操作：

```fish
function starship_transent_rmpt_func
  starship module time
end
starship init fish | source
enable_transience
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

## 在 Bash 中自定义提示符显示前和执行前的命令

Bash 不像多数其他的 Shell 有成体系的预执行框架。 因此，很难在 `bash` 中提供完全可自定义的 hook 机制。 然而，Starship 确实能使您有限地在提示符渲染过程中插入自己的函数执行：

- 若要在提示符显示之前运行自定义函数，需要定义此函数，然后将函数名赋值给 `starship_reserved_user_func`。 例如，如果想在提示符前绘制一个火箭，可以这样写：

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

一些 Shell 支持右侧提示, 它与输入区渲染在同一行。 使用 `right_format` 选项来设置 Starship 的右侧提示。 所有支持 `format` 的组件也同时支持 `right_format`。 未显式在 `format` 或 `right_format` 中使用的组件，会保存在变量 `$all` 中。

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

`<color>` 可以声明颜色，会在下面解释。 `fg:<color>` 和 `<color>` 的功能暂时相同，未来可能会更改。 `inverted` 会反转背景和文字的颜色。 字符串中的单词顺序不影响显示结果。

若 `none` 不是 `bg:` 的一部分，则它会覆盖其他的设置：比如 `fg:red none fg:blue` 不会更改任何样式。 `bg:none` 会设置成默认背景色，因此 `fg:red bg:none`、`red`、`fg:red` 的作用相同；类似，`bg:green fg:red bg:none`、`fg:red`、`red` 的作用也相同。 未来可能会将 `none` 与其它单词一起使用视为错误。

颜色可以由以下任一内容定义：

- 任一标准的终端颜色：`black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`。 您也可以使用前缀 `bright-` 定义浅色版本（例如 `bright-white`）。
- 一个 `#` 后跟一个六位十六进制数。 这将指定一个 [十六进制 RGB 颜色代码](https://www.w3schools.com/colors/colors_hexadecimal.asp)。
- 0-255 之间的数字。 这将指定一个 [8 位 ANSI 颜色码](https://i.stack.imgur.com/KTSQa.png)。

如果为文本/背景指定了多个颜色，字符串中最后指定的颜色将具有最高优先级。

并非每种类型的字符串都会被每个终端正确显示。 特别地，以下是已知的几种情况：

- 许多终端默认禁用对 `blink` 的支持
- `hidden` 在 iTerm (https://gitlab.com/gnachman/iterm2/-/issues/4564) 上不被支持。
- `strikethrough` 不被默认 macOS Terminal.app 支持
