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

The [Bash implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) built into Starship is slightly more complex to allow for advanced features like the [Command Duration module](https://starship.rs/config/#Command-Duration) and to ensure that Starship is compatible with pre-installed Bash configurations.

For a list of all flags accepted by `starship prompt`, use the following command:

```sh
starship prompt --help
```

The prompt will use as much context as is provided, but no flags are "required".
