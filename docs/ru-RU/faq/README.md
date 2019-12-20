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

Да, они могут быть использованы для отключения модулей в подсказке. Если всё, что вы хотите сделать - это отключить модули, `<module>.disabled` - предпочитаемый способ сделать это по следующим причинам:

- Отключение модулей является более явным, чем удаление их из prompt_order
- Новосозданные модули будут добавлены в подсказку по мере обновления Starship

## В документации написано, что Starship - для многих оболочек, но он не поддерживает оболочку X. Почему?

Starship устроен так, что есть возможность добавить поддержку практически любой оболочки. Бинарный файл Starship не зависит от оболочки и не имеет состояния, так что если ваша оболочка поддерживает расширение подстрок и настройку подсказки, то Starship может быть использован.

Вот небольшой пример работы Starship с bash:

```sh
# Get the status code from the last command executed
STATUS=$?

# Get the number of jobs running.
NUM_JOBS=$(jobs -p | wc -l)

# Set the prompt to the output of `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=NUM_JOBS)"
```

[Реализация для Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash), встроенная в Starship, несколько сложнее, чтобы предоставить дополнительные возможности, такие как [модуль длительности команды](https://starship.rs/config/#Command-Duration) и обеспечить совместимость Starship с заранее установленными конфигурациями Bash.

Для списка всех флагов, принимаемых `starship prompt`, используйте следующую команду:

```sh
starship prompt --help
```

Подсказка будет использовать столько контекста, сколько доступно, но ни один флаг не обязателен.
