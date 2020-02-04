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

- 要在一个命令运行前运行自定义函数，您可以使用 [`DEBUG` trap 机制](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/)。 然而，您**必须**在捕捉 DEBUG 信号*之前*启动 Starship！ Starship can preserve the value of the DEBUG trap, but if the trap is overwritten after starship starts up, some functionality will break.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
eval $(starship init bash)
```

## 更改窗口标题

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish even does it by default. Starship does not do this, but it's fairly straightforward to add this functionality to `bash` or `zsh`.

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

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zsrhc`) to make it permanent.

## 样式设定

样式字符串是用空白分隔的单词列表。 The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `underline`
  - `dimmed`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing , though this may change in the future. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string, so that e.g. `fg:red none fg:blue` will still create a string with no styling. It may become an error to use `none` in conjunction with other tokens in the future.

颜色指定器可以是以下内容之一：

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
