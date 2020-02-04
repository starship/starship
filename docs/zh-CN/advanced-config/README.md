# 高级配置

Starship 功能繁多，有时您必须在编辑 `starship.toml` 之外做更多工作才能实现某些鲜果。 此页面详细介绍了一些在 starship 中使用的高级配置技术。

::: warning

本节所述的配置内容可能随 Starship 未来版本的更新而改变。

:::

## 在 Bash 中自定义预提示和预执行命令

Bash 没有像大多数其它 shell 一样的正式预执行/预命令框架。 因此，很难在 `bash` 中提供完全可自定义的 hook 机制。 然而，Starship 确实能使您有限地在提示符渲染过程中插入自己的函数执行：

- 若要在提示符显示之前运行自定义函数，需要定义此函数，然后将函数名赋值给`starship_reserved_user_func`。 例如，在提示符之前绘制一枚火箭，您应该写

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- 要在一个命令运行前运行自定义函数，您可以使用 [`DEBUG` trap 机制](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/)。 然而，您**必须**在捕捉 DEBUG 信号*之前*启动 Starship！ Starship 可以保留 DEBUG trap 的值，但如果该 trap 在 starship 启动后被覆盖，一些功能将会被破坏。

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # 启动 starship *之前* 设置 DEBUG trap
eval $(starship init bash)
```

## 更改窗口标题

一些 shell 提示符会自动更改您的窗口标题（比如为了提示您的工作目录）。 Fish 甚至默认会执行此功能。 Starship 没有实现此功能，但将这个功能添加到 `bash` 或 `zsh` 是相当简单的。

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

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zsrhc`) to make it permanent.

## 样式设定

样式字符串是用空格分隔的单词列表。 其中单词不是大小写敏感的（例如 `bold` 和 `BoLd` 被视为同一字符串）。 每个单词可以是以下之一：

  - `bold`
  - `underline`
  - `dimmed`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

`<color>` 是颜色说明符（下面解释）。 `fg:<color>` 和 `<color>` 当前产生一样的效果，尽管未来可能会改变。 字符串中的单词顺序不影响结果。

`none` 标识符会覆盖字符串中所有其他标识符，比如 `fg:red none fg:blue` 将创建一个没有样式设置的字符串。 未来可能会将 `none` 与其它标识符一起使用视为一种错误。

颜色说明符可以是以下内容之一：

 - 标准终端颜色之一：`black`，`red`，`green`，`blue`，`yellow`，`purple`，`cyan`，`white`。 您可以使用可选前缀 `bright-` 来获取明亮版本的颜色（例如，`bright-white`）。
 - 一个 `#` 后跟一个六位十六进制数。 这将指定一个 [十六进制 RGB 颜色代码](https://www.w3schools.com/colors/colors_hexadecimal.asp)。
 - 0-255 之间的数字。 这将指定一个 [8 位 ANSI 颜色码](https://i.stack.imgur.com/KTSQa.png)。

如果为文本/背景指定了多个颜色，字符串中最后指定的颜色将具有最高优先级。
