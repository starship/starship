# Часті питання

## Які налаштування використовуються в демо-GIF?

- **Емулятор термінала**: [iTerm2](https://iterm2.com/)
  - **Тема**: Minimal
  - **Схема кольорів**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Шрифт**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Оболонка**: [Fish Shell](https://fishshell.com/)
  - **Налаштування**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Командний рядок**: [Starship](https://starship.rs/)

## Як зробити завершення команди як показано в демо-GIF?

Підтримка завершення або автозаповнення, надається вашою оболонкою. In the case of the demo, the demo was done with [Fish Shell](https://fishshell.com/), which provides completions by default. If you use Z Shell (zsh), I'd suggest taking a look at [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Чи `format` та `<module>.`disabled верхнього рівня роблять теж саме?

Так, вони обидва можуть використовуватись для вимикання модулів у командному рядку. If all you plan to do is disable modules, `<module>.disabled` is the preferred way to do so for these reasons:

- Вимкнення модулів є більш явною дією аніж приховування їх у `format`
- Нові створені модулі будуть додані до командного рядка, оскільки Starship оновлюється

## В документації йдеться, що Starship є крос-оболонковим. Чому ж тоді моя оболонка не підтримується?

Через те, яким чином Starship побудований, підтримку практично будь-якої оболонки можна достатньо легко додати. Двійковий файл starship не має статусу та не залежить від оболонки, тому Starship можна використовувати, якщо ваша оболонка підтримує оперативне налаштування та розширення оболонки.

Ось маленький приклад того, як зробити так, щоб Starship працював в bash:

```sh
# Отримати код статусу виконання з останньої команди
STATUS=$?

# Отримати кількість запущених завдань.
NUM_JOBS=$(jobs -p | wc -l)

# Встановити для командного рядка вивід `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

Реалізація [Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) створена в Starship є трохи складнішою, щоб дозволити використання додаткових функцій, таких як модуль [Command Duration](https://starship.rs/config/#command-duration) і переконатися, що Starship сумісний із попередньо встановленими конфігураціями Bash.

Для отримання переліку всіх прапорів, які приймаються `starship prompt`, скористайтесь наступною командою:

```sh
starship prompt --help
```

Командний рядок використовуватиме якомога більше отриманого контексту, однак прапорці не є "обовʼязковими".

## Як запустити Starship в дистрибутивах Linux зі старими версіями glibc?

Якщо ви отримуєте помилку_версію 'GLIBC_2.18' не знайдено (потрібна для starship)_при використанні вбудованого бінарного файлу (наприклад, на CentOS 6 або 7), ви можете використовувати бінарний файл скомпільований з підтримкою `musl` замість `glibc`:

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## Чому я бачу попередження `Executing command "..." timed out.`?

Starship виконує різні команди, щоб отримати інформацію для показу в командному рядку, наприклад версію програми або поточного статусу git. Щоб переконатися, що starship не підвис, намагаючись виконати ці команди, ми встановлюємо ліміт,
якщо виконання команди займе більше часу, starship зупинить виконання команди та видасть попередження, це очікувана поведінка. This time limit is configurable using the [`command_timeout`key](../config/#prompt) so if you want you can increase the time limit. Ви можете також виконати дії для відлагодження, щоб побачити, яка команда є повільною і подивитися, чи ви можете її оптимізувати. Finally you can set the `STARSHIP_LOG` env var to
`error` to hide these warnings.

## Я бачу якісь символи, але не розумію, що вони значать?

Якщо ви бачите символи, які ви не можете розпізнати, ви можете скористатись `starship explain`, для отримання пояснень для показаних модулів.

## Starship робить щось несподіване, як я можу налагодити його?

You can enable the debug logs by using the `STARSHIP_LOG` env var. These logs
can be very verbose so it is often useful to use the `module` command if you are
trying to debug a particular module, for example, if you are trying to debug
the `rust` module you could run the following command to get the trace
logs and output from the module.

```sh
env STARSHIP_LOG=trace starship module rust
```

Якщо Starship працює повільно, ви можете спробувати використати команду `timings`, щоб побачити, чи є певний модуль або команда, які можна в цьому звинуватити.

```sh
env STARSHIP_LOG=trace starship timings
```

Це виведе журнал трасування та розбивку всіх модулів, для виконання яких знадобилося понад 1 мс, або вони дали певні результати.

Нарешті, якщо ви знайшли помилку, ви можете використати команду `bug-report`, щоб створити квиток в GitHub.

```sh
starship bug-report
```

## Чому я не бачу символу гліфа в командному рядку?

Найпоширенішою причиною цього є неправильна конфігурація системи. Зокрема, деякі дистрибутиви Linux не мають підтримки шрифтів із коробки. Ви повинні переконатись, що:

- Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. If `LC_ALL` is not a UTF-8 value,
  [you will need to change it](https://www.tecmint.com/set-system-locales-in-linux/).
- У вас встановлено шрифт для emoji. Most systems come with an emoji font by default, but
  some (notably Arch Linux) do not. You can usually install one through your system's
  package manager--[noto emoji](https://www.google.com/get/noto/help/emoji/) is a popular choice.
- Ви використовуєте [Nerd Font](https://www.nerdfonts.com/).

Щоб перевірити свою систему, виконайте наступні команди в терміналі:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

У першому рядку має бути [емодзі змії](https://emojipedia.org/snake/), у той час як другий має показати [символ гілки (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

Якщо будь-який символ не показується належним чином, ваша система все ще не налаштована правильно.
На жаль, інколи важко налаштувати правильну конфігурацію шрифту. Користувачі в Discord можуть допомогти. If both symbols display correctly, but
you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## Як видалити Starship?

Starship так само легко видалити, як і встановити.

1. Видаліть усі рядки у конфігурації оболонки (наприклад, `~/.bashrc`), які використовуються для ініціалізації Starship.
2. Видаліть двійковий файл Starship.

Якщо Starship був встановлений за допомогою менеджера пакунків, будь ласка, зверніться до документації по їх видаленню.

Якщо Starship було встановлено за допомогою сценарію встановлення, наступна команда видалить двійковий файл:

```sh
# Locate and delete the starship binary
sh -c 'rm "$(command -v 'starship')"'
```

## Як встановити Starship без `sudo`?

The shell install script (`https://starship.rs/install.sh`) only attempts to use `sudo` if the target installation directory is not writable by the current user. The default installation directory is the value of the `$BIN_DIR` environment variable or `/usr/local/bin` if `$BIN_DIR` is not set. If you instead set the installation directory to one that is writable by your user, you should be able to install starship without `sudo`. For example, `curl -sS https://starship.rs/install.sh | sh -s -- -b ~/.local/bin` uses the `-b` command line option of the install script to set the installation directory to `~/.local/bin`.

For a non-interactive installation of Starship, don't forget to add the `-y` option to skip the confirmation. Перегляньте сирці скрипту встановлення для ознайомлення зі всіма можливими параметрами.

У разі використання пакетного менеджера, ознайомтесь з документацією до нього що до можливості встановлення з застосуванням, чи без, `sudo`.
