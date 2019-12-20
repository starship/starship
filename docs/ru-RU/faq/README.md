# FAQ

## Какая конфигурация используется в демо-GIF?

- **Эмулятор терминала**: [iTerm2](https://iterm2.com/)
  - **Тема**: Минимальная
  - **Цветовая схема**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Шрифт**: [Fira Code](https://github.com/tonsky/FiraCode)
- **Оболочка**: [Fish Shell](https://fishshell.com/)
  - **Конфигурация**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/master/.config/fish/config.fish)
  - **Подсказка**: [Starship](https://starship.rs/)

## `prompt_order` и `<module>.disabled` - это одно и то же?

Да, они могут быть использованы для отключения модулей в подсказке. If all you plan to do is disable modules, `<module>.disabled` is the preferred way to do so for these reasons:

- Disabling modules is more explicit than omitting them from the prompt_order
- Newly created modules will be added to the prompt as Starship is updated

## The docs say Starship is cross-shell, but it doesn't support X shell. Why?

The way Starship is built, it should be possible to add support for virtually any shell. The starship binary is stateless and shell agnostic, so as long as your shell supports prompt customization and shell expansion, Starship can be used.

Here's a small example getting Starship working with bash:

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
