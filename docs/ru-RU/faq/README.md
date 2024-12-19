# Часто задаваемые вопросы

## Какая конфигурация используется в демо-GIF?

- **Эмулятор терминала**: [iTerm2](https://iterm2.com/)
  - **Тема**: Minimal
  - **Цветовая схема**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Шрифт**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Командная оболочка**: [Fish Shell](https://fishshell.com/)
  - **Конфигурация**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Подсказка**: [Starship](https://starship.rs/)

## Как мне получить автодополнение команд, как показано на демонстрационной GIF?

Автодополнение команд обеспечивается выбранной вами оболочкой. В данном случае, демо было выполнено с [Fish Shell](https://fishshell.com/), которая обеспечивает дополнения по умолчанию. Если вы используете Z Shell (zsh), я бы посоветовал взглянуть на [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Do top level `format` and `<module>.disabled` do the same thing?

Да, они могут быть использованы для отключения модулей в подсказке. Если всё, что вы хотите сделать - это отключить модули, `<module>.disabled` - предпочитаемый способ сделать это по следующим причинам:

- Disabling modules is more explicit than omitting them from the top level `format`
- Новосозданные модули будут добавлены в подсказку по мере обновления Starship

## Доки говорят, что Starship поддерживается на всех оболочках *(cross-shell)*. Почему моя любимая оболочка не поддерживается?

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

[Реализация для Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash), встроенная в Starship, несколько сложнее, чтобы предоставить дополнительные возможности, такие как [модуль длительности команды](https://starship.rs/config/#command-duration) и обеспечить совместимость Starship с заранее установленными конфигурациями Bash.

Для списка всех флагов, принимаемых `starship prompt`, используйте следующую команду:

```sh
starship prompt --help
```

Подсказка будет использовать столько контекста, сколько доступно, но ни один флаг не обязателен.

## Как запускать Starship на Linux-дистрибутивах с более ранними версиями glibc?

Если вы получаете ошибку типа "_version 'GLIBC_2.18' not found (required by starship)_" при использовании заранее собранного бинарного файла (например, на CentOS 6 или 7), вы можете использовать бинарный файл, скомпилированый с `musl` вместо `glibc`:

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## Почему я вижу предупреждение  `Executing command "..." timed out.`?

Starship выполняет различные команды, чтобы получить информацию, отображаемую в промпте, например версию программы или текущий git status. Чтобы быть уверенными, что starship не зависнет во время выполнения этих команд, мы поставили лимит времени, и если команда выполняется дольше лимита, starship прекратит её выполнение и выведет это предупреждение, это нормальное поведение. Временной лимит можно изменить с помощью опции [`command_timeout`key](../config/#prompt), поэтому при желании вы можете увеличить это время. Вы так же можете следовать шагам для отладки ниже, чтобы понять, какая команда влияет на время и ускорить промпт. Наконец, вы можете изменить переменную окружения `STARSHIP_LOG` `error`, чтобы спрятать это предупреждение.

## Я вижу символы, которые я не понимаю или не ожидаю, что они означают?

Если вы видите символы, которые вы не узнаете, вы можете использовать команду `starship explain`, чтобы разъяснить показываемые модули.

## Starship делает что-то неожиданное, как я могу отладить его?

You can enable the debug logs by using the `STARSHIP_LOG` env var. These logs can be very verbose so it is often useful to use the `module` command if you are trying to debug a particular module, for example, if you are trying to debug the `rust` module you could run the following command to get the trace logs and output from the module.

```sh
env STARSHIP_LOG=trace starship module rust
```

If starship is being slow you can try using the `timings` command to see if there is a particular module or command that is to blame.

```sh
env STARSHIP_LOG=trace starship timings
```

This will output the trace log and a breakdown of all modules that either took more than 1ms to execute or produced some output.

Finally if you find a bug you can use the `bug-report` command to create a GitHub issue.

```sh
starship bug-report
```

## Почему я не вижу символ в промпте?

Наиболее распространенной причиной этого является неправильная конфигурация системы. В частности, некоторые Linux дистрибутивы не предоставляют поддержку шрифта из коробки. Необходимо убедиться, что:

- Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. If `LC_ALL` is not a UTF-8 value, [you will need to change it](https://www.tecmint.com/set-system-locales-in-linux/).
- You have an emoji font installed. Most systems come with an emoji font by default, but some (notably Arch Linux) do not. You can usually install one through your system's package manager--[noto emoji](https://www.google.com/get/noto/help/emoji/) is a popular choice.
- Вы используете [Nerd Font](https://www.nerdfonts.com/).

Для тестирования системы запустите следующие команды в терминале:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

Первая строка должна создать [эмоджи змеи](https://emojipedia.org/snake/), а вторая - символ ветки [(e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

Если любой из символов не отображается корректно, ваша система все еще неправильно настроена. К сожалению, иногда сложно получить правильную конфигурацию шрифта. Пользователи в Discord могут помочь. Если оба символа отображаются верно, но вы всё ещё не видите их в starship, [отправьте bug report!](https://github.com/starship/starship/issues/new/choose)

## Как удалить Starship?

Starship is just as easy to uninstall as it is to install in the first place.

1. Remove any lines in your shell config (e.g. `~/.bashrc`) used to initialize Starship.
1. Delete the Starship binary.

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

If Starship was installed using the install script, the following command will delete the binary:

```sh
# Locate and delete the starship binary
sh -c 'rm "$(command -v 'starship')"'
```

## Как Starship без `sudo`?

Скрипт установки (`https://starship.rs/install. h`) использует `sudo` только если директория установки недоступна для записи текущим пользователем. The default installation directory is the value of the `$BIN_DIR` environment variable or `/usr/local/bin` if `$BIN_DIR` is not set. Если вместо этого выбрать директорию установки, которая доступна для записи пользователем, вы можете установить starship без `sudo`. Например, в `curl -sS https://starship.rs/install.sh | sh -s -- -b ~/.local/bin` флаг `-b` установочного скрипта используется, чтобы задать директорию установки на `~/.local/bin`.

Для неинтерактивной установки Starship не забудьте добавить опцию `-y` чтобы пропустить подтверждение. Проверьте исходник установочного скрипта, чтобы получить список всех поддерживаемых параметров установки.

При через пакетный менеджер, смотрите документацию для вашего пакетного менеджера об установке с `sudo` и без.
