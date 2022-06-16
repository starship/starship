# 常见问题

## 主页示例图中的效果用的是什么配置？

- **终端模拟器**：[iTerm2](https://iterm2.com/)
  - **主题**：Minimal
  - **颜色方案**：[Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **字体**：[Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**：[Fish Shell](https://fishshell.com/)
  - **fish 配置**：[matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **提示符工具**：[Starship](https://starship.rs/)

## 如何实现示例图中自动补全的功能？

命令自动补全功能是由您选择的 Shell 提供。 演示中的自动补全由 [Fish Shell](https://fishshell.com/) 完成，它默认开启自动补全。 若您使用 Z Shell (zsh)，推荐您查看 [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions)。

## 顶层配置 `format` 与 `<module>.disabled` 的功能是相同的吗？

是的，他们都可以用来禁用提示符中的组件。 如果你只是想禁用组件，推荐使用 `<module>.disabled`，原因如下：

- 在顶层配置 `format` 里忽略组件与禁用组件相比较，后者更加明确
- 当 Starship 升级后，新组件将能够自动被加入提示符中

## 文档中写 Starship 适用于任何 Shell。 为什么我常用的 Shell 不支持？

Starship 的构建方式决定了它应当能够增加对几乎所有 shell 的支持。 Starship 的二进制文件是无状态、不知道当前 shell 的，所以只要你的 shell 支持自定义提示符和 shell 扩展，就能使用 Starship。

这是一个在 bash 上使用 Starship 的简单例子：

```sh
# 获取最后执行命令的返回值
STATUS=$?

# 获取运行后台程序的数量
NUM_JOBS=$(jobs -p | wc -l)

# 设置提示符为 `starship prompt` 的输出
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

内置于 Starship 的 [Bash 适配](https://github.com/starship/starship/blob/master/src/init/starship.bash) 稍微复杂一些，实现了像 [命令用时统计组件](https://starship.rs/config/#command-duration) 这样的功能，还确保 Starship 能与之前设置的 Bash 配置相兼容。

使用以下命令了解 `starship prompt` 所能接受的所有参数：

```sh
starship prompt --help
```

Starship 会处理所提供的全部上下文参数并在提示符中显示，但没有参数是“必需”的。

## 我如何在 Linux 发行版上使用旧版本 glibc 运行 Starship？

若您在使用预构建的程序时，遇到类似“_version 'GLIBC_2.18' not found (required by starship)_”的错误（例如在 CentOS 6 或 7 里运行）您可以使用 `musl` 的编译版本，而非 `glibc` 版本。

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## 为什么我会遇到 `Executing command "..." timed out.` 警告？

Starship 会执行数个不同的命令来获取应该显示的信息，例如某个程序的版本号、现在 Git 的工作树状态。 为保证 Starship 不会在执行某条命令时卡住，Starship 会终止执行时间过长的命令并且输出以上警告。这是正常现象。 若希望增加时长限制，它可以在 [`command_timeout` 设置](/config/#prompt) 处自定义。 您也可以按照下文的调试步骤查看并优化运行慢的命令。 最后，您也可以设置环境变量 `STARSHIP_LOG` 为 `error` 来隐藏这些警告。

## 我不理解某些符号，它们是什么意思？

若您不清楚某些符号，可以使用 `starship explain` 查看正在显示的组件。

## Starship 运行不正常，我该如何调试？

您可以使用环境变量 `STARSHIP_LOG` 来开启调试日志。 这些日志可能过于详细，所以更常使用 `module` 来调试特定的组件。例如要调试 `rust` 组件，您可以使用下列命令来获取它的日志和输出。

```sh
env STARSHIP_LOG=trace starship module rust
```

若 Starship 运行缓慢，您可以使用 `timings` 命令查看运行缓慢的组件或命令。

```sh
env STARSHIP_LOG=trace starship timings
```

它会输出日志，并且记录有输出的组件、运行时长超过 1 毫秒的组件。

最后，如果您发现了错误，可以使用 `bug-report` 命令创建 GitHub Issue。

```sh
starship bug-report
```

## 为什么我在提示符中看不到某个字符？

最常见的原因是系统配置有问题。 有个别Linux发行版不自带对字体的支持。 请确保：

- 您的字符集应设置为 UTF-8，例如 `de_DE.UTF-8` 或 `ja_JP.UTF-8`。 若 `LC_ALL` 不是 UTF-8， [你需要手动设置它](https://www.tecmint.com/set-system-locales-in-linux/)。
- 安装了 emoji 字体。 大部分系统都会自带 emoji 字体，但有些系统（例如 Arch Linux）则没有。 字体一般可以用系统的包管理器安装，常见的字体有 [Noto emoji](https://www.google.com/get/noto/help/emoji/) 等。
- 您正在使用 [Nerd Font](https://www.nerdfonts.com/)。

在终端内运行以下命令来测试：

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

第一行应该显示出一个[蛇的 emoji](https://emojipedia.org/snake/)，第二行应该显示出 [powerline 的分支符号（e0a0）。](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

若任一符号显示错误，则您的系统配置不正确。 有些时候正确配置字体并不简单。 Discord 里的用户或许能提供帮助。 若两个字符显示正确，但您仍然不能在 Starship 中看到那个字符，请 [提交错误报告](https://github.com/starship/starship/issues/new/choose)！

## 如何卸载 Starship？

Starship 的卸载过程与安装过程一样简单。

1. 将 shell 的配置文件（比如 `~/.bashrc`）中初始化 Starship 的部分全部删除。
1. 删除 Starship 的二进制文件。

如果 Starship 是用包管理器安装的，请到包管理器的文档中查找卸载的步骤。

如果 Starship 是用安装脚本安装的，可以用以下命令删除二进制文件：

```sh
# 找到并且删除 Starship 二进制文件
sh -c 'rm "$(command -v 'starship')"'
```
