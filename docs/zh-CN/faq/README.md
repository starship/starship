# 常见问题

## 主页 GIF 示例中的效果用的是什么配置？

- **终端模拟器**：[iTerm2](https://iterm2.com/)
  - **主题**：Minimal
  - **颜色方案**：[Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **字体**：[Fira Code](https://github.com/tonsky/FiraCode)
- **Shell**：[Fish Shell](https://fishshell.com/)
  - **fish 配置**：[matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/master/.config/fish/config.fish)
  - **提示符工具**：[Starship](https://starship.rs/)

## `prompt_order` 和 `<module>.disabled` 的效果是一样的吗？

是的，他们都可以用来禁用提示符中的组件。 如果你只是想禁用组件，推荐使用 `<module>.disabled`，原因如下：

- “禁用组件”比在 prompt_order 中忽略某个组件更为清晰明确
- 当 Starship 升级后，新组件将能够自动被加入提示符中

## 你们的文档说“Starship 是跨 shell 的”，但它不支持 X shell。 为什么？

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

内置于 Starship 的 [Bash 适配](https://github.com/starship/starship/blob/master/src/init/starship.bash) 稍微复杂一些，实现了像 [命令用时统计组件](https://starship.rs/config/#Command-Duration) 这样的功能，还确保 Starship 能与之前设置的 Bash 配置相兼容。

使用以下命令了解 `starship prompt` 所能接受的所有参数：

```sh
starship prompt --help
```

Starship 会处理所提供的全部上下文参数并在提示符中显示，但没有参数是“必需”的。

## How do I run Starship on Linux distributions with older versions of glibc?

If you get an error like "*version 'GLIBC_2.18' not found (required by starship)*" when using the prebuilt binary (for example, on CentOS 6 or 7), you can use a binary compiled with `musl` instead of `glibc`:

```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- --platform unknown-linux-musl
```

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

  - Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. If `LC_ALL` is not a UTF-8 value, [you will need to change it](https://www.tecmint.com/set-system-locales-in-linux/).
  - You have an emoji font installed. Most systems come with an emoji font by default, but some (notably Arch Linux) do not. You can usually install one through your system's package manager--[noto emoji](https://www.google.com/get/noto/help/emoji/) is a popular choice.
  - You are using a [powerline-patched font](https://github.com/powerline/fonts).

To test your system, run the following commands in a terminal:

```
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)
