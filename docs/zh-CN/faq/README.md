# Frequently Asked Questions

## 主页示例图中的效果用的是什么配置？

- **终端模拟器**：[iTerm2](https://iterm2.com/)
  - **主题**：Minimal
  - **颜色方案**：[Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **字体**：[Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**：[Fish Shell](https://fishshell.com/)
  - **fish 配置**：[matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **提示符工具**：[Starship](https://starship.rs/)

## 如何实现示例图中自动补全的功能？

Completion support, or autocomplete, is provided by your shell of choice. In the case of the demo, the demo was done with [Fish Shell](https://fishshell.com/), which provides completions by default. If you use Z Shell (zsh), I'd suggest taking a look at [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Do top level `format` and `<module>.disabled` do the same thing?

是的，他们都可以用来禁用提示符中的组件。 如果你只是想禁用组件，推荐使用 `<module>.disabled`，原因如下：

- Disabling modules is more explicit than omitting them from the top level `format`
- 当 Starship 升级后，新组件将能够自动被加入提示符中

## The docs say Starship is cross-shell. Why isn't my preferred shell supported?

Starship 的构建方式决定了它应当能够增加对几乎所有 shell 的支持。 Starship 的二进制文件是无状态、不知道当前 shell 的，所以只要你的 shell 支持自定义提示符和 shell 扩展，就能使用 Starship。

这是一个在 bash 上使用 Starship 的简单例子：

```sh
# Get the status code from the last command executed
STATUS=$?

# Get the number of jobs running.
NUM_JOBS=$(jobs -p | wc -l)

# Set the prompt to the output of `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

内置于 Starship 的 [Bash 适配](https://github.com/starship/starship/blob/master/src/init/starship.bash) 稍微复杂一些，实现了像 [命令用时统计组件](https://starship.rs/config/#command-duration) 这样的功能，还确保 Starship 能与之前设置的 Bash 配置相兼容。

使用以下命令了解 `starship prompt` 所能接受的所有参数：

```sh
starship prompt --help
```

Starship 会处理所提供的全部上下文参数并在提示符中显示，但没有参数是“必需”的。

## How do I run Starship on Linux distributions with older versions of glibc?

If you get an error like "_version 'GLIBC_2.18' not found (required by starship)_" when using the prebuilt binary (for example, on CentOS 6 or 7), you can use a binary compiled with `musl` instead of `glibc`:

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## Why do I see `Executing command "..." timed out.` warnings?

Starship executes different commands to get information to display in the prompt, for example the version of a program or the current git status. To make sure starship doesn't hang while trying to execute these commands we set a time limit, if a command takes longer than this limit starship will stop the execution of the command and output the above warning, this is expected behaviour. This time limit is configurable using the [`command_timeout`key](/config/#prompt) so if you want you can increase the time limit. You can also follow the debugging steps below to see which command is being slow and see if you can optimise it. Finally you can set the `STARSHIP_LOG` env var to `error` to hide these warnings.

## I see symbols I don't understand or expect, what do they mean?

If you see symbols that you don't recognise you can use `starship explain` to explain the currently showing modules.

## Starship is doing something unexpected, how can I debug it?

You can enable the debug logs by using the `STARSHIP_LOG` env var. These logs can be very verbose so it is often useful to use the `module` command if you are trying to debug a particular module, for example, if you are trying to debug the `rust` module you could run the following command to get the trace logs and output from the module.

```sh
env STARSHIP_LOG=trace starship module rust
```

If starship is being slow you can try using the `timings` command to see if there is a particular module or command that to blame.

```sh
env STARSHIP_LOG=trace starship timings
```

This will output the trace log and a breakdown of all modules that either took more than 1ms to execute or produced some output.

Finally if you find a bug you can use the `bug-report` command to create a Github issue.

```sh
starship bug-report
```

## Why don't I see a glyph symbol in my prompt?

最常见的原因是系统配置有问题。 有个别Linux发行版不自带对字体的支持。 请确保：

- Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. If `LC_ALL` is not a UTF-8 value, [you will need to change it](https://www.tecmint.com/set-system-locales-in-linux/).
- 安装了 emoji 字体。 大部分系统都会自带 emoji 字体，但有些系统（例如 Arch Linux）则没有。 字体一般可以用系统的包管理器安装，常见的字体有 [Noto emoji](https://www.google.com/get/noto/help/emoji/) 等。
- You are using a [Nerd Font](https://www.nerdfonts.com/).

To test your system, run the following commands in a terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

第一行应该显示出一个[蛇的 emoji](https://emojipedia.org/snake/)，第二行应该显示出 [powerline 的分支符号（e0a0）。](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## 如何卸载 Starship？

Starship 的卸载过程与安装过程一样简单。

1. 将 shell 的配置文件（比如 `~/.bashrc`）中初始化 Starship 的部分全部删除。
1. 删除 Starship 的二进制文件。

如果 Starship 是用包管理器安装的，请到包管理器的文档中查找卸载的步骤。

如果 Starship 是用安装脚本安装的，可以用以下命令删除二进制文件：

```sh
# Locate and delete the starship binary
sh -c 'rm "$(command -v 'starship')"'
```
