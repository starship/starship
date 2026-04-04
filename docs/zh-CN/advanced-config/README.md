# 高级配置

Starship 功能繁多，有时您必须在编辑 `starship.toml` 之外做更多工作才能实现某些效果。 此页面详细介绍了一些在 starship 中使用的高级配置技巧。

> [!WARNING] 本节所述的配置内容可能随 Starship 未来版本的更新而改变。

## PowerShell 中的 TransientPrompt

可以用自定义字符串替换预设的命令行提示。 这在不经常需要所有提示信息的情况下很有用。 若要启用该功能，请在 shell 中运行 `Enable-TransientPrompt`命令 若要永久启用该功能，请将 上述语句放在您的 `$PROFILE` 中。 通过在shell中运行 `Disable-TransientPrompt`命令来禁用这项功能。

默认情况下，输入的左侧是 `>`符号。 要自定义它，请定义一个新函数，名为 `Invoke-Starship-TransitentFunction`。 例如，要 在这里显示Starship的 `character` 模块，您需要如下操作：

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## 在 Cmd 中的命令行提示（TransientPrompt）和语法高亮（TransientRightPrompt）

可以用自定义字符串替换预设的命令行提示。 这在不经常需要所有提示信息的情况下很有用。 要启用该功能，运行命令`clink set prompt.transient <value>`，&lt;prompt.transient>之后跟以下单词中的一个：

- `always`: 总是预设的提示
- `same_dir`: 仅在工作目录相同的情况下替换预设的提示
- `off`: 不要替换提示(即关闭命令行提示)

你只需要这样做一次。 对您的 `starship.lua` 进行以下更改，以自定义左侧和右侧显示的内容：

- 默认情况下，输入的左侧是 `>`符号。 要自定义它，请定义一个新函数，名为 `Invoke-Starship-TransitentFunction`。 这个函数接受当前的提示符作为字符串参数，你可以在函数中使用它。 例如，要 在这里显示Starship的 `character` 模块，您需要如下操作：

```lua
function starship_transitent_propt_func(empt)
  return io.popen("starship module character"
    ..." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))
```

- 默认情况下，输入的右侧为空。 要自定义它，请定义一个新函数，名为 `Invoke-Starship-TransitentFunction`。 这个函数接受当前的提示符作为字符串参数，你可以在函数中使用它。 例如，要在这里显示 最后一个命令开始的时间，您需要如下操作：

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## 在 Fish 中的命令行提示（TransientPrompt）和语法高亮（TransientRightPrompt）

可以用自定义字符串替换预设的命令行提示。 这在不经常需要所有提示信息的情况下很有用。 若要启用该功能，请在 shell 中运行 `Enable-TransitientPrompt`命令 若要永久启用该功能，请将 上述语句放在您的 `~/.config/fish/config.fish` 中。 通过在shell中运行 `Disable-TransientPrompt`命令来禁用这项功能。

请注意，对于Fish，命令行提示只在命令行非空 且语法正确的情况下才会显示。

- 默认情况下，输入的左侧是 粗体绿色的`❯`符号。 要自定义它，请定义一个新函数，名为 `Invoke-Starship-TransitentFunction`。 例如，要在这里显示 Starship 的 `character` 组件，您需要如下操作：

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

## Bash 中的 TransientPrompt 和 TransientRightPrompt

Ble.sh 框架在V0.4或更高版本允许您用自定义字符串替换之前打印的提示符。 这在并非总是需要所有提示信息的情况下非常有用。 To enable this, put this in `~/.bashrc` `bleopt prompt_ps1_transient=<value>`:

The \<value\> here is a colon-separated list of `always`, `same-dir` and `trim`. When `prompt_ps1_final` is empty and the option `prompt_ps1_transient` has a non-empty \<value\>, the prompt specified by `PS1` is erased on leaving the current command line. If \<value\> contains a field `trim`, only the last line of multiline `PS1` is preserved and the other lines are erased. Otherwise, the command line will be redrawn as if `PS1=` is specified. When a field `same-dir` is contained in \<value\> and the current working directory is different from the final directory of the previous command line, this option `prompt_ps1_transient` is ignored.

Make the following changes to your `~/.blerc` (or in `~/.config/blesh/init.sh`) to customize what gets displayed on the left and on the right:

- To customize what the left side of input gets replaced with, configure the `prompt_ps1_final` Ble.sh option. For example, to display Starship's `character` module here, you would do

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- To customize what the right side of input gets replaced with, configure the `prompt_rps1_final` Ble.sh option. 例如，要在这里显示 最后一个命令开始的时间，您需要如下操作：

```bash
bleopt prompt_rps1_final='$(starship module time)'
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
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## 启用右侧提示

一些 Shell 支持右侧提示, 它与输入区渲染在同一行。 使用 `right_format` 选项来设置 Starship 的右侧提示。 所有支持 `format` 的组件也同时支持 `right_format`。 未显式在 `format` 或 `right_format` 中使用的组件，会保存在变量 `$all` 中。

注意：右侧提示和输入区显示在同一行。 To right align modules above the input line in a multi-line prompt, see the [`fill` module](../config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Note: The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework v0.4 or higher should be installed in order to use right prompt in bash.

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

使用 `continuation_prompt` 选项来设置 Starship 的多行提示符。 The default prompt is `'[∙](bright-black) '`.

注意：`continuation_prompt` 应设置为没有变量的字符串。

注意，仅以下 Shell 支持多行提示符：

- `bash`
- `zsh`
- `PowerShell`

### 示例

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled-in arrows
continuation_prompt = '▶▶ '
```

## Statusline for Claude Code

Starship supports displaying a custom statusline when running inside Claude Code, Anthropic's CLI tool for interactive coding with Claude. This statusline provides real-time information about your Claude session, including the model being used, context window usage, and session costs.

For more information about the Claude Code statusline feature, see the [Claude Code statusline documentation](https://code.claude.com/docs/en/statusline).

### Setup

To use Starship as your Claude Code statusline:

1. Run `/statusline` in Claude Code and ask it to configure Starship, or manually add the following to your `.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "starship statusline claude-code"
  }
}
```

2. Customize the statusline appearance in your `~/.config/starship.toml` (see [Configuration](#configuration) below)

### Overview

When invoked with `starship statusline claude-code`, Starship receives Claude Code session data via stdin and renders a statusline using a dedicated profile named `claude-code`.

The profile includes three specialized modules:

- `claude_model`: Displays the current Claude model being used
- `claude_context`: Shows context window usage with a visual gauge
- `claude_cost`: Displays session cost and statistics

The default profile format is:

```toml
[profiles]
claude-code = "$claude_model$git_branch$claude_context$claude_cost"
```

### 配置

You can customize the Claude Code statusline by modifying the `claude-code` profile and individual module configurations in your `~/.config/starship.toml`:

```toml
# ~/.config/starship.toml

# Customize the claude-code profile
[profiles]
claude-code = "$claude_model$claude_context$claude_cost"

# Configure individual modules
[claude_model]
format = "[$symbol$model]($style) "
symbol = "🤖 "
style = "bold blue"

[claude_context]
format = "[$gauge $percentage]($style) "
gauge_width = 10

[claude_cost]
format = "[$symbol$cost]($style) "
symbol = "💰 "
```

### Claude Model

The `claude_model` module displays the current Claude model being used in the session.

#### 配置项

| 选项              | 默认值                          | 描述                                                                                        |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------------------- |
| `format`        | `'[$symbol$model]($style) '` | 组件格式化模板。                                                                                  |
| `symbol`        | `'🤖 '`                       | The symbol shown before the model name.                                                   |
| `style`         | `'bold blue'`                | 此组件的样式。                                                                                   |
| `model_aliases` | `{}`                         | Map of model IDs or display names to shorter aliases. Checks ID first, then display name. |
| `disabled`      | `false`                      | Disables the `claude_model` module.                                                       |

#### 变量

| 字段        | 示例                  | 描述                                    |
| --------- | ------------------- | ------------------------------------- |
| model     | `Claude 3.5 Sonnet` | The display name of the current model |
| model_id  | `claude-3-5-sonnet` | The model ID                          |
| symbol    |                     | `symbol`对应值                           |
| style\* |                     | `style`对应值                            |

\*: 此变量只能作为样式字符串的一部分使用

#### 示例

```toml
# ~/.config/starship.toml

# Basic customization
[claude_model]
format = "on [$symbol$model]($style) "
symbol = "🧠 "
style = "bold cyan"

# Using model aliases for vendor-specific model names
# You can alias by model ID or display name
[claude_model.model_aliases]
# Alias by vendor model ID (e.g. AWS Bedrock)
"global.anthropic.claude-sonnet-4-5-20250929-v1:0" = "Sonnet 4.5"
# Alias by display name
"Claude Sonnet 4.5 (Vendor Proxy)" = "Sonnet"
```

### Claude Context

The `claude_context` module displays context window usage as a percentage and visual gauge. The style automatically changes based on configurable thresholds.

#### 配置项

| 选项                     | 默认值                               | 描述                                                 |
| ---------------------- | --------------------------------- | -------------------------------------------------- |
| `format`               | `'[$gauge $percentage]($style) '` | 组件格式化模板。                                           |
| `symbol`               | `''`                              | The symbol shown before the gauge.                 |
| `gauge_width`          | `5`                               | The width of the gauge in characters.              |
| `gauge_full_symbol`    | `'█'`                             | The symbol used for filled segments of the gauge.  |
| `gauge_partial_symbol` | `'▒'`                             | The symbol used for partial segments of the gauge. |
| `gauge_empty_symbol`   | `'░'`                             | The symbol used for empty segments of the gauge.   |
| `display`              | [见下文解释](#display)                 | Threshold and style configurations.                |
| `disabled`             | `false`                           | Disables the `claude_context` module.              |

##### Display

The `display` option is an array of objects that define thresholds and styles for different usage levels. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| 选项          | 默认值          | 描述                                                                       |
| ----------- | ------------ | ------------------------------------------------------------------------ |
| `threshold` | `0.0`        | The minimum context windows usage percentage to match this configuration |
| `style`     | `bold green` | The value of `style` if this display configuration is matched            |
| `hidden`    | `false`      | Hide this module if this the configuration is matched.                   |

```toml
[[claude_context.display]]
threshold = 0
hidden = true

[[claude_context.display]]
threshold = 30
style = "bold green"

[[claude_context.display]]
threshold = 60
style = "bold yellow"

[[claude_context.display]]
threshold = 80
style = "bold red"
```

#### 变量

| 字段                           | 示例      | 描述                                                    |
| ---------------------------- | ------- | ----------------------------------------------------- |
| gauge                        | `██▒░░` | Visual representation of context usage                |
| percentage                   | `65%`   | Context usage as a percentage                         |
| input_tokens                 | `45.2k` | Total input tokens in conversation                    |
| output_tokens                | `12.3k` | Total output tokens in conversation                   |
| curr_input_tokens          | `5.1k`  | Input tokens from most recent API call                |
| curr_output_tokens         | `1.2k`  | Output tokens from most recent API call               |
| curr_cache_creation_tokens | `1.5k`  | Cache creation tokens from most recent API call       |
| curr_cache_read_tokens     | `23.4k` | Cache read tokens from most recent API call           |
| total_tokens                 | `200k`  | Total context window size                             |
| symbol                       |         | `symbol`对应值                                           |
| style\*                    |         | Mirrors the style from the matching display threshold |

\*: 此变量只能作为样式字符串的一部分使用

#### 示例

**Minimal gauge-only display**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$gauge]($style) "
gauge_width = 10
```

**Detailed token information**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$percentage ($input_tokens in / $output_tokens out)]($style) "
```

**Custom gauge symbols**

```toml
# ~/.config/starship.toml

[claude_context]
gauge_full_symbol = "▰"
gauge_partial_symbol = ""
gauge_empty_symbol = "▱"
gauge_width = 10
format = "[$gauge]($style) "
```

**Custom thresholds**

```toml
# ~/.config/starship.toml

[[claude_context.display]]
threshold = 0
style = "bold green"

[[claude_context.display]]
threshold = 50
style = "bold yellow"

[[claude_context.display]]
threshold = 75
style = "bold orange"

[[claude_context.display]]
threshold = 90
style = "bold red"
```

### Claude Cost

The `claude_cost` module displays the total cost of the current Claude Code session in USD. Like `claude_context`, it supports threshold-based styling.

#### 配置项

| 选项         | 默认值                                | 描述                                  |
| ---------- | ---------------------------------- | ----------------------------------- |
| `format`   | `'[$symbol(\\$$cost)]($style) '` | 组件格式化模板。                            |
| `symbol`   | `'💰 '`                             | The symbol shown before the cost.   |
| `display`  | [见下文解释](#display-1)                | Threshold and style configurations. |
| `disabled` | `false`                            | Disables the `claude_cost` module.  |

##### Display

The `display` option is an array of objects that define cost thresholds and styles. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| 选项          | 默认值          | 描述                                                            |
| ----------- | ------------ | ------------------------------------------------------------- |
| `threshold` | `0.0`        | The minimum cost in USD to match this configuration           |
| `style`     | `bold green` | The value of `style` if this display configuration is matched |
| `hidden`    | `false`      | Hide this module if this configuration is matched.            |

**Default configuration:**

```toml
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 1.0
style = "bold yellow"

[[claude_cost.display]]
threshold = 5.0
style = "bold red"
```

#### 变量

| 字段            | 示例       | 描述                                                    |
| ------------- | -------- | ----------------------------------------------------- |
| cost          | `1.23`   | Total session cost in USD (formatted to 2 decimals)   |
| duration      | `1m 30s` | Total session duration                                |
| api_duration  | `45s`    | Total API call duration                               |
| lines_added   | `1.2k`   | Total lines of code added                             |
| lines_removed | `500`    | Total lines of code removed                           |
| symbol        |          | `symbol`对应值                                           |
| style\*     |          | Mirrors the style from the matching display threshold |

\*: 此变量只能作为样式字符串的一部分使用

#### 示例

```toml
# ~/.config/starship.toml

# Cost with code change statistics
[claude_cost]
format = "[$symbol$cost (+$lines_added -$lines_removed)]($style) "

# Hide module until cost exceeds $0.10
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 0.10
style = "bold yellow"

[[claude_cost.display]]
threshold = 2.0
style = "bold red"

# Show duration information
[claude_cost]
format = "[$symbol$cost ($duration)]($style) "
```

## 样式字符串

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

`<color>` 可以声明颜色，会在下面解释。 `fg:<color>` 和 `<color>` 的功能暂时相同，未来可能会更改。 `<color>` can also be set to `prev_fg` or `prev_bg` which evaluates to the previous item's foreground or background color respectively if available or `none` otherwise. `inverted` 会反转背景和文字的颜色。 字符串中的单词顺序不影响显示结果。

若 `none` 不是 `bg:` 的一部分，则它会覆盖其他的设置：比如 `fg:red none fg:blue` 不会更改任何样式。 `bg:none` 会设置成默认背景色，因此 `fg:red bg:none`、`red`、`fg:red` 的作用相同；类似，`bg:green fg:red bg:none`、`fg:red`、`red` 的作用也相同。 未来可能会将 `none` 与其它单词一起使用视为错误。

颜色可以由以下任一内容定义：

- 任一标准的终端颜色：`black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`。 您也可以使用前缀 `bright-` 定义浅色版本（例如 `bright-white`）。
- 一个 `#` 后跟一个六位十六进制数。 这将指定一个 [十六进制 RGB 颜色代码](https://www.w3schools.com/colors/colors_hexadecimal.asp)。
- 0-255 之间的数字。 这将指定一个 [8 位 ANSI 颜色码](https://i.stack.imgur.com/KTSQa.png)。

如果为文本/背景指定了多个颜色，字符串中最后指定的颜色将具有最高优先级。

并非每种类型的字符串都会被每个终端正确显示。 特别地，以下是已知的几种情况：

- 许多终端默认禁用对 `blink` 的支持.
- [iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564) 不支持 `hidden`
- macOS 的默认终端不支持 `strikethrough`.
