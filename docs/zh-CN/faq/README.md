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
PS1="$(starship prompt --status=$STATUS --jobs=NUM_JOBS)"
```

内置于 Starship 的 [Bash 适配](https://github.com/starship/starship/blob/master/src/init/starship.bash) 稍微复杂一些，实现了像 [命令用时统计组件](https://starship.rs/config/#Command-Duration) 这样的功能，还确保 Starship 能与预安装的 Bash 配置相兼容。

使用以下命令了解 `starship prompt` 所能接受的所有参数：

```sh
starship prompt --help
```

Starship 会处理所提供的全部上下文参数并在提示符中显示，但没有参数是“必需”的。
