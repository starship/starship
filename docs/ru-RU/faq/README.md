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
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

[Реализация для Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash), встроенная в Starship, несколько сложнее, чтобы предоставить дополнительные возможности, такие как [модуль длительности команды](https://starship.rs/config/#Command-Duration) и обеспечить совместимость Starship с заранее установленными конфигурациями Bash.

Для списка всех флагов, принимаемых `starship prompt`, используйте следующую команду:

```sh
starship prompt --help
```

Подсказка будет использовать столько контекста, сколько доступно, но ни один флаг не обязателен.

## Как запускать Starship на Linux-дистрибутивах с более ранними версиями glibc?

Если вы получаете ошибку типа "*version 'GLIBC_2.18' not found (required by starship)*" при использовании заранее собранного бинарного файла (например, на CentOS 6 или 7), вы можете использовать бинарный файл, скомпилированый с `musl` вместо `glibc`:

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
