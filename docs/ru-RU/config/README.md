# Конфигурация

Чтобы начать конфигурацию Starship, создайте следующий файл: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Вся конфигурация Starship выполняется в этом файле [TOML](https://github.com/toml-lang/toml):

```toml
# Не добавлять пустую строку в начале ввода
add_newline = false

# Поменять символ "❯" на символ "➜"
[character]      # Имя настраемого модуля - "character"
symbol = "➜"      # Сегменту "symbol" присваеваем значение "➜"

# Отключить модуль пакетов, полностью скрывая его из терминала
[package]
disabled = true
```

Вы можете изменить расположение файла `starship.toml` переменной окружения `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/.starship
```

Аналогично в PowerShell (Windows) следует добавить эту строку в `$PROFILE`:

```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### Логгирование (Запись действий)

По умолчанию в starship записываются предупреждения и ошибки в файл с именем `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, где ключ сессии соответствует экземпляру терминала. Это, однако, может быть изменено с помощью переменной окружения `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Аналогично в PowerShell (Windows) следует добавить эту строку в `$PROFILE`:

```ps1
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### Терминология

**Модуль**: Компонент строки, дающий информацию на основе контекстной информации вашей ОС. Например, модуль "nodejs" показывает установленную версию NodeJS на вашем компьютере, если вы находитесь в директории проекта NodeJS.

**Переменные**: Маленькие подкомпоненты, содержащие информацию, предоставленную модулем. Например, переменная "version" в модуле "nodejs" содержит текущую версию NodeJS.

По традициям, большинство модулей имеют префикс цвета терминала по умолчанию (например, `через` в "узлах") и пустое пространство как суффикс.

### Форматирование строк

Формат строк - это формат, с которым модуль печатает все переменные. Большинство модулей имеют запись `формата`, который настраивает формат отображения модуля. Вы можете использовать тексты, переменные и группы текста в строке формата.

#### Переменная

Переменная содержит символ `$`, за которым следует имя переменной. Имя переменной содержит только буквы, цифры и `_`.

Например:

- `$version` это строка формата с именем `версии`.
- `$git_branch$git_commit` это строка формата с двумя переменными `git_branch` и `git_commit`.
- `$git_branch $git_commit` имеет две переменные, разделенные пробелом.

#### Группа текста

Текстовая группа состоит из двух различных частей.

Первая часть, которая заключена в `[]`, это [формат строки](#format-strings). Вы можете добавлять в него тексты, переменные, или даже вложенные текстовые группы.

Во второй части, которая заключена в `()`, это строка стиля [](#style-strings). Это может быть использовано в стиле первой части.

Например:

- `[on](red bold)` будет печатать строку `on` жирным текстом красного цвета.
- `[⬢ $version](bold green)` будет печатать символ `⬢` за которым следует содержимое переменной `версии`, с жирным шрифтом зеленого цвета.
- `[a [b](red) c](green)` будет печатать `a b c` с `b` красного и `a` и `c` зеленого цвета соответсвенно.

#### Строки стиля

В Starship, большинство модулей позволяют настроить стили отображения. Это делается записью (обычно называется `style`), которая представляет собой строку, определяющую конфигурацию. Ниже приведены несколько примеров стилей строк, а также, их действия. Подробнее о полном синтаксисе можно прочитать в [расширенном разделе конфигурации](/advanced-config/).

- `"fg:green bg:blue"` устанавливает зеленый текст на синем фоне
- `"bg:blue fg:bright-green"` устанавливает ярко-зеленый текст на синем фоне
- `"bold fg:27"` устанавливает жирный текст с [цветом ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` устанавливает подчёркиваемый текст цвета сожженного апельсина
- `"bold italic fg:purple"` устанавливает жирный фиолетовый текст
- `""` выключает все стили

Обратите внимание на то, что, вид стиля зависит от вашего эмулятора терминала. Например, некоторые эмуляторы терминала будут использовать яркие цвета вместо жирного текста, и некоторые цветовые темы используют одинаковые значение для обычных и ярких цветов. Также, чтобы получить курсивный текст, ваш терминал должен поддерживать курсив.

#### Строки условного формата

Строка условного формата, завернутая в `(` и `)` не будет отображаться, если все переменные внутри являются пустыми.

Например:

- `(@$region)` ничего не будет показывать, если переменная `регион` в значении `None`, иначе `@` будет использовать значение этой переменной.
- `(некоторый текст)` всегда не показывает ничего, поскольку в скобках нет переменных.
- Когда `$all` является ярлыком для `\[$a$b\]`, `($all)` будет показываться только если `$a` и `$b` не являются `None`. Это работает так же, как `(\[$a$b\] )`.

#### Символ экранирования

Следующие символы имеют особое значение в формате. Если вы хотите напечатать следующие символы, вам нужно экранировать их обратной косой чертой (`\`).

- \$
- \\
- [
- ]
- (
- )

Обратите внимание, что `toml` имеет [свой собственный синтаксис для экранирования](https://github.com/toml-lang/toml#user-content-string). В конфигурации рекомендуется использовать строку с знаком (`''`). Если вы хотите использовать базовую строку (`""`), обратите внимание на то, чтобы избежать обратной косой черты `\`.

Например, когда вы хотите напечатать символ `$` на новой строке, следующие настройки для `формата` эквивалентны:

```toml
# с базовой строкой
format = "\n\\$"

# с многолинейной строкой
format = """

\\$"""

# со строкой, состоящей из литералов
format = '''

\$'''
```

## Командная строка

Ниже находится список опций, применяющихся для всей командной строки.

### Опции

| Параметр       | По умолчанию                     | Описание                                                 |
| -------------- | -------------------------------- | -------------------------------------------------------- |
| `format`       | [ссылка](#default-prompt-format) | Настройка форматирования оболочки.                       |
| `scan_timeout` | `30`                             | Тайм-аут запуска сканирования файлов (в миллисекундах).  |
| `add_newline`  | `true`                           | Добавление пустой строки перед началом командной строки. |

### Пример

```toml
# ~/.config/starship.toml

# Собственное форматирование оболочки
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Подождите 10 милисекунд пока starship прочитает файлы в этой директории.
scan_timeout = 10

# Выключить новую строку в начале запроса
add_newline = false
```

### Формат оболочки по умолчанию

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. Значение по умолчанию:

```toml
format = "$all"

# Which is equivalent to
format = """
$username\
$hostname\
$shlvl\
$kubernetes\
$directory\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
$ruby\
$rust\
$swift\
$terraform\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$env_var\
$crystal\
$cmd_duration\
$custom\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$character"""
```

## AWS

Модуль `aws` показывает текущий регион и профиль AWS. Основано на `AWS_REGION`, `AWS_DEFAULT_REGION`, и `AWS_PROFILE` переменных окружения и файле`~/.aws/config`.

При использовании [aws-vault](https://github.com/99designs/aws-vault) профиль читается из переменной среды `AWS_VAULT`.

### Опции

| Параметр         | По умолчанию                                     | Описание                                                       |
| ---------------- | ------------------------------------------------ | -------------------------------------------------------------- |
| `format`         | `'on [$symbol$profile(\($region\))]($style) '` | The format for the module.                                     |
| `symbol`         | `"☁️ "`                                          | Символ перед отображением текущего профиля AWS.                |
| `region_aliases` |                                                  | Таблица региона псевдонимов, отображаемая вместе с именем AWS. |
| `style`          | `"bold yellow"`                                  | Стиль модуля.                                                  |
| `disabled`       | `false`                                          | Отключение модуля `AWS`.                                       |

### Переменные

| Переменная | Пример           | Описание                             |
| ---------- | ---------------- | ------------------------------------ |
| регион     | `ap-northeast-1` | The current AWS region               |
| профиль    | `астронавты`     | The current AWS profile              |
| symbol     |                  | Mirrors the value of option `symbol` |
| style\*  |                  | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Examples

#### Display everything

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$profile(\($region\))]($style) '
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Display region

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$region]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Display profile

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
```

## Батарея

Модуль `battery` показывает насколько заряжена батарея девайса и статус зарядки на данный момент. Модуль виден только, если заряд батареи устройства меньше 10%.

### Опции

| Параметр             | По умолчанию                      | Описание                                        |
| -------------------- | --------------------------------- | ----------------------------------------------- |
| `full_symbol`        | `"•"`                             | Символ, отображаемый при полной батарее.        |
| `charging_symbol`    | `"⇡"`                             | Символ, показываемый при зарядке аккумулятора.  |
| `discharging_symbol` | `"⇣"`                             | Символ, показываемый при разрядке аккумулятора. |
| `format`             | `"[$symbol$percentage]($style) "` | The format for the module.                      |
| `display`            | [ссылка](#battery-display)        | Порог отображения и стиль для модуля.           |
| `disabled`           | `false`                           | Отключает модуль `battery`.                     |

<details>
<summary>Также, есть опции для некоторых нетипичных состояний батареи.</summary>

| Переменная       | Описание                                                |
| ---------------- | ------------------------------------------------------- |
| `unknown_symbol` | Символ, отображаемый при неизвестном состоянии батареи. |
| `empty_symbol`   | Символ, отображаемый при пустом состоянии батареи.      |

Примечание: Индикатор батареи будет скрыт при состоянии `unknown` или `empty`, если вы не указали параметр в настройках.

</details>

### Пример

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Отображение батареи

Параметр `display` используется для определения того, когда индикатор батареи должен быть показан (threshhold) и как он выглядит (style). Если `display` не предоставлено. Значение по умолчанию:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Опции

Опция `display` представляет собой массив следующей таблицы.

| Параметр    | Описание                                                 |
| ----------- | -------------------------------------------------------- |
| `threshold` | Верхняя граница опции отображения.                       |
| `style`     | Используемый стиль, если используется опция отображения. |

#### Пример

```toml
[[battery.display]] # стиль "bold red" (жирный красный) если заряд между 0% и 10%
threshold = 10
style = "bold red"

[[battery.display]] # стиль "bold yellow" (жирный желтый) если заряд между 10% и 30%
threshold = 30
style = "bold yellow"

# когда заряд батареи больше 30%, индикатор батареи скрыт

```

## Символ

Модуль `character` показывает символ (обычно, стрелка) рядом с вводимым текстом в терминале.

Символ показывает, была ли последняя команда успешной или нет. It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change it's shape take a look at [this example](#with-custom-error-shape).

### Опции

| Параметр         | По умолчанию        | Описание                                                                         |
| ---------------- | ------------------- | -------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | The format string used before the text input.                                    |
| `success_symbol` | `"[❯](bold green)"` | The format string used before the text input if the previous command succeeded.  |
| `error_symbol`   | `"[❯](bold red)"`   | The format string used before the text input if the previous command failed.     |
| `vicmd_symbol`   | `"[❮](bold green)"` | The format string used before the text input if the shell is in vim normal mode. |
| `disabled`       | `false`             | Отключает модуль `character`.                                                    |

### Переменные

| Переменная | Пример | Описание                                                              |
| ---------- | ------ | --------------------------------------------------------------------- |
| symbol     |        | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

### Examples

#### With custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Without custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### With custom vim shape

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

The `cmake` module shows the currently installed version of CMake if any of the following conditions are met:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### Опции

| Параметр   | По умолчанию                       | Описание                                     |
| ---------- | ---------------------------------- | -------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                   |
| `symbol`   | `"喝 "`                             | The symbol used before the version of cmake. |
| `style`    | `"bold blue"`                      | Стиль модуля.                                |
| `disabled` | `false`                            | Disables the `cmake` module.                 |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v3.17.3` | The version of cmake                 |
| symbol     |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

## Длительность команды

Модуль `cmd_duration` показывает время исполнения последней команды. Модуль будет показан только, если команда заняла более двух секунд, или если задан параметр `min_time`.

::: warning Не подключайте ловушку DEBUG к Bash

Если вы испоьзуете Starship в `bash`, не подключайте ловушку `DEBUG` после запуска `eval $(starship init $0)`, иначе этот модуль сломается.

:::

Пользователи Bash, которым нужна функциональность, подобная preexec, могут использовать [фреймворк bash_preexec от rcaloras](https://github.com/rcaloras/bash-preexec). Просто определите массивы `preexec_functions` и `precmd_functions` перед запуском `eval $(starship init $0)`, а затем продолжайте нормально.

### Опции

| Параметр             | По умолчанию                  | Описание                                                             |
| -------------------- | ----------------------------- | -------------------------------------------------------------------- |
| `min_time`           | `2_000`                       | Кратчайшая продолжительность для показа времени (в миллисекундах).   |
| `show_milliseconds`  | `false`                       | Показывать миллисекунды в дополнение к секундам в продолжительности. |
| `format`             | `"took [$duration]($style) "` | The format for the module.                                           |
| `style`              | `"bold yellow"`               | Стиль модуля.                                                        |
| `disabled`           | `false`                       | Отключает модуль `cmd_duration`.                                     |
| `show_notifications` | `false`                       | Show desktop notifications when command completes.                   |
| `min_time_to_notify` | `45_000`                      | Shortest duration for notification (in milliseconds).                |

::: tip

Showing desktop notifications requires starship to be built with `rust-notify` support. You check if your starship supports notifications by running `STARSHIP_LOG=debug starship module cmd_duration -d 60000` when `show_notifications` is set to `true`.

:::

### Переменные

| Переменная | Пример   | Описание                                |
| ---------- | -------- | --------------------------------------- |
| duration   | `16m40s` | The time it took to execute the command |
| style\*  |          | Mirrors the value of option `style`     |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Конда

The `conda` module shows the current conda environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### Опции

| Параметр            | По умолчанию                       | Описание                                                                                                                                                                                                     |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `1`                                | Количество каталогов, в которых путь к окружению должен быть усечен, если окружение было создано через `conda create -p [path]`. `0` означает без усечения. Также смотрите модуль [`directory`](#directory). |
| `symbol`            | `"🅒 "`                             | Символ перед названием окружения.                                                                                                                                                                            |
| `style`             | `"bold green"`                     | Стиль модуля.                                                                                                                                                                                                |
| `format`            | `"[$symbol$environment]($style) "` | The format for the module.                                                                                                                                                                                   |
| `ignore_base`       | `true`                             | Ignores `base` environment when activated.                                                                                                                                                                   |
| `disabled`          | `false`                            | Отключает модуль `conda`.                                                                                                                                                                                    |

### Переменные

| Переменная  | Пример       | Описание                             |
| ----------- | ------------ | ------------------------------------ |
| environment | `астронавты` | The current conda environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*   |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. The module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `shard.yml`
- Текущий каталог содержит файл `.cr`

### Опции

| Параметр   | По умолчанию                       | Описание                                                |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `symbol`   | `"🔮 "`                             | Символ, используемый перед отображением версии crystal. |
| `style`    | `"bold red"`                       | Стиль модуля.                                           |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                              |
| `disabled` | `false`                            | Отключает модуль `crystal`.                             |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v0.32.1` | The version of `crystal`             |
| symbol     |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

The `dart` module shows the currently installed version of Dart. The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml` or `pubspec.lock` file

### Опции

| Параметр   | По умолчанию                       | Описание                                        |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🎯 "`                             | A format string representing the symbol of Dart |
| `style`    | `"bold blue"`                      | Стиль модуля.                                   |
| `disabled` | `false`                            | Disables the `dart` module.                     |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v2.8.4` | The version of `dart`                |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Каталог

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Опции

| Параметр            | По умолчанию                                       | Описание                                                                     |
| ------------------- | -------------------------------------------------- | ---------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                | Количество родительских папок, к которым должен быть усечен текущий каталог. |
| `truncate_to_repo`  | `true`                                             | Следует или нет обрезать до корня репозитория git, в котором вы находитесь.  |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | The format for the module.                                                   |
| `style`             | `"bold cyan"`                                      | Стиль модуля.                                                                |
| `disabled`          | `false`                                            | Отключает модуль `directory`.                                                |
| `read_only`         | `"🔒"`                                              | The symbol indicating current directory is read only.                        |
| `read_only_style`   | `"red"`                                            | The style for the read only symbol.                                          |
| `truncation_symbol` | `""`                                               | The symbol to prefix to truncated paths. eg: "…/"                            |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | По умолчанию | Описание                                                                          |
| --------------------------- | ------------ | --------------------------------------------------------------------------------- |
| `substitutions`             |              | A table of substitutions to be made to the path.                                  |
| `fish_style_pwd_dir_length` | `0`          | Количество символов, используемых при использовании логики создания пути из fish. |
| `use_logical_path`          | `true`       | Отображает логический путь от оболочки (`PWD`) вместо пути от ОС.                 |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Переменные

| Переменная | Пример                | Описание                            |
| ---------- | --------------------- | ----------------------------------- |
| path       | `"D:/Projects"`       | The current directory path          |
| style\*  | `"black bold dimmed"` | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Контекст Docker

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### Опции

| Параметр          | По умолчанию                       | Описание                                                                                |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | The format for the module.                                                              |
| `symbol`          | `"🐳 "`                             | The symbol used before displaying the Docker context.                                   |
| `style`           | `"blue bold"`                      | Стиль модуля.                                                                           |
| `only_with_files` | `false`                            | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `disabled`        | `true`                             | Disables the `docker_context` module.                                                   |

### Переменные

| Переменная | Пример         | Описание                             |
| ---------- | -------------- | ------------------------------------ |
| context    | `test_context` | The current docker context           |
| symbol     |                | Mirrors the value of option `symbol` |
| style\*  |                | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one or more of the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.sln`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Опции

| Параметр    | По умолчанию                             | Описание                                                          |
| ----------- | ---------------------------------------- | ----------------------------------------------------------------- |
| `format`    | `"v[$symbol$version( 🎯 $tfm)]($style) "` | The format for the module.                                        |
| `symbol`    | `"•NET "`                                | Символ перед отображением текущей версии dotnet.                  |
| `heuristic` | `true`                                   | Использовать быстрое определение версии, для сохранения скорости. |
| `style`     | `"bold blue"`                            | Стиль модуля.                                                     |
| `disabled`  | `false`                                  | Отключает модуль `dotnet`.                                        |

### Переменные

| Переменная | Пример           | Описание                                                           |
| ---------- | ---------------- | ------------------------------------------------------------------ |
| version    | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm        | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol     |                  | Mirrors the value of option `symbol`                               |
| style\*  |                  | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. The module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `mix.exs`.

### Опции

| Параметр   | По умолчанию                                              | Описание                                                      |
| ---------- | --------------------------------------------------------- | ------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                    | Символ, используемый перед отображением версии Elixir/Erlang. |
| `style`    | `"bold purple"`                                           | Стиль модуля.                                                 |
| `format`   | `'via [$symbol$version \(OTP $otp_version\)]($style) '` | The format for the module elixir.                             |
| `disabled` | `false`                                                   | Отключает модуль `elixir`.                                    |

### Переменные

| Переменная  | Пример  | Описание                             |
| ----------- | ------- | ------------------------------------ |
| version     | `v1.10` | The version of `elixir`              |
| otp_version |         | The otp version of `elixir`          |
| symbol      |         | Mirrors the value of option `symbol` |
| style\*   |         | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. The module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `elm.json`
- Текущий каталог содержит файл `elm-package.json`
- Текущий каталог содержит файл `.elm-version`
- Текущий каталог содержит папку `elm-stuff`
- Текущий каталог содержит файлы `*.elm`

### Опции

| Параметр   | По умолчанию                       | Описание                                        |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🌳 "`                             | A format string representing the symbol of Elm. |
| `style`    | `"cyan bold"`                      | Стиль модуля.                                   |
| `disabled` | `false`                            | Отключает модуль `elm`.                         |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v0.19.1` | The version of `elm`                 |
| symbol     |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Переменная Окружения

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- Опция `variable` соответствует существующей переменной среды
- Опция `variable` не определена, но определена опция `default`

### Опции

| Параметр   | По умолчанию                   | Описание                                                         |
| ---------- | ------------------------------ | ---------------------------------------------------------------- |
| `symbol`   |                                | Символ, используемый перед отображением значения переменной.     |
| `variable` |                                | Отображаемая переменная окружения.                               |
| `default`  |                                | Значение отображаемое, когда выбранная переменная не определена. |
| `format`   | `"with [$env_value]($style) "` | The format for the module.                                       |
| `disabled` | `false`                        | Отключает модуль `env_var`.                                      |

### Переменные

| Переменная | Пример                                      | Описание                                   |
| ---------- | ------------------------------------------- | ------------------------------------------ |
| env_value  | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol     |                                             | Mirrors the value of option `symbol`       |
| style\*  | `black bold dimmed`                         | Mirrors the value of option `style`        |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. The module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `rebar.config`.
- Текущий каталог содержит файл `erlang.mk`.

### Опции

| Параметр   | По умолчанию                       | Описание                                                 |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `symbol`   | `" "`                             | The symbol used before displaying the version of erlang. |
| `style`    | `"bold red"`                       | Стиль модуля.                                            |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                               |
| `disabled` | `false`                            | Disables the `erlang` module.                            |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v22.1.3` | The version of `erlang`              |
| symbol     |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Gcloud

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Опции

| Параметр         | По умолчанию                                     | Описание                                                        |
| ---------------- | ------------------------------------------------ | --------------------------------------------------------------- |
| `format`         | `'on [$symbol$account(\($region\))]($style) '` | The format for the module.                                      |
| `symbol`         | `"☁️ "`                                          | The symbol used before displaying the current GCP profile.      |
| `region_aliases` |                                                  | Table of region aliases to display in addition to the GCP name. |
| `style`          | `"bold blue"`                                    | Стиль модуля.                                                   |
| `disabled`       | `false`                                          | Disables the `gcloud` module.                                   |

### Переменные

| Переменная | Пример            | Описание                                                           |
| ---------- | ----------------- | ------------------------------------------------------------------ |
| регион     | `us-central1`     | The current GCP region                                             |
| account    | `foo@example.com` | The current GCP profile                                            |
| project    |                   | The current GCP project                                            |
| active     | `default`         | The active config name written in `~/.config/gcloud/active_config` |
| symbol     |                   | Mirrors the value of option `symbol`                               |
| style\*  |                   | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string

### Examples

#### Display account and project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(\($project\))]($style) '
```

#### Display active config name only

```toml
# ~/.config/starship.toml

[gcloud]
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Display account and aliased region

```toml
# ~/.config/starship.toml

[gcloud]
symbol = "️🇬️ "
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"
```

## Ветвь Git

The `git_branch` module shows the active branch of the repo in your current directory.

### Опции

| Параметр            | По умолчанию                     | Описание                                                                                      |
| ------------------- | -------------------------------- | --------------------------------------------------------------------------------------------- |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module. Use `"$branch"` to refer to the current branch name.               |
| `symbol`            | `" "`                           | A format string representing the symbol of git branch.                                        |
| `style`             | `"bold purple"`                  | Стиль модуля.                                                                                 |
| `truncation_length` | `2^63 - 1`                       | Отрезает ветку git до X графемов.                                                             |
| `truncation_symbol` | `"…"`                            | Символ, используемый для обозначения усечения названия ветки. You can use `""` for no symbol. |
| `disabled`          | `false`                          | Отключает модуль `git_branch`.                                                                |

### Переменные

| Переменная | Пример   | Описание                                                                                             |
| ---------- | -------- | ---------------------------------------------------------------------------------------------------- |
| branch     | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached HEAD). |
| symbol     |          | Mirrors the value of option `symbol`                                                                 |
| style\*  |          | Mirrors the value of option `style`                                                                  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Коммит Git

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Опции

| Параметр             | По умолчанию                                           | Описание                                                                |
| -------------------- | ------------------------------------------------------ | ----------------------------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | Длина отображаемого хэша коммита git.                                   |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | The format for the module.                                              |
| `style`              | `"bold green"`                                         | Стиль модуля.                                                           |
| `only_detached`      | `true`                                                 | Показывать хэш коммита git, только находясь в состоянии отделённой HEAD |
| `tag_disabled`       | `true`                                                 | Disables showing tag info in `git_commit` module.                       |
| `tag_symbol`         | `"🏷 "`                                                 | Tag symbol prefixing the info shown                                     |
| `disabled`           | `false`                                                | Disables the `git_commit` module.                                       |

### Переменные

| Переменная | Пример    | Описание                            |
| ---------- | --------- | ----------------------------------- |
| hash       | `b703eb3` | The current git commit hash         |
| style\*  |           | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Состояние Git

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Опции

| Параметр       | По умолчанию                                                    | Описание                                                                                |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | Стиль модуля.                                                                           |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | The format for the module.                                                              |
| `disabled`     | `false`                                                         | Отключает модуль `git_state`.                                                           |

### Переменные

| Переменная       | Пример     | Описание                            |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*        |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Статус Git

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Опции

| Параметр     | По умолчанию                                    | Описание                            |
| ------------ | ----------------------------------------------- | ----------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status` |
| `conflicted` | `"="`                                           | Эта ветка имеет конфликты слияния.  |
| `ahead`      | `"⇡"`                                           | The format of `ahead`               |
| `behind`     | `"⇣"`                                           | The format of `behind`              |
| `diverged`   | `"⇕"`                                           | The format of `diverged`            |
| `untracked`  | `"?"`                                           | The format of `untracked`           |
| `stashed`    | `"$"`                                           | The format of `stashed`             |
| `modified`   | `"!"`                                           | The format of `modified`            |
| `staged`     | `"+"`                                           | The format of `staged`              |
| `renamed`    | `"»"`                                           | The format of `renamed`             |
| `deleted`    | `"✘"`                                           | The format of `deleted`             |
| `style`      | `"bold red"`                                    | Стиль модуля.                       |
| `disabled`   | `false`                                         | Отключает модуль `git_status`.      |

### Переменные

The following variables can be used in `format`:

| Переменная     | Описание                                                                                      |
| -------------- | --------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                   |
| `ahead_behind` | Displays `diverged` `ahead` or `behind` format string based on the current status of the repo |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                   |
| `untracked`    | Displays `untracked` when there are untracked files in the working directory.                 |
| `stashed`      | Displays `stashed` when a stash exists for the local repository.                              |
| `modified`     | Displays `modified` when there are file modifications in the working directory.               |
| `staged`       | Displays `staged` when a new file has been added to the staging area.                         |
| `renamed`      | Displays `renamed` when a renamed file has been added to the staging area.                    |
| `deleted`      | Displays `deleted` when a file's deletion has been added to the staging area.                 |
| style\*      | Mirrors the value of option `style`                                                           |

\*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Переменная     | Описание                                       |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Переменная | Описание                   |
| ---------- | -------------------------- |
| `count`    | Показать количество файлов |

### Пример

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
untracked = "🤷‍"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

The `golang` module shows the currently installed version of Golang. The module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `go.mod`
- Текущий каталог содержит файл `go.sum`
- Текущий каталог содержит файл `glide.yaml`
- Текущий каталог содержит файл `Gopkg.yml`
- Текущий каталог содержит файл `Gopkg.lock`
- The current directory contains a `.go-version` file
- Текущий каталог содержит папку `Godeps`
- Текущий каталог содержит файл с расширением `.go`

### Опции

| Параметр   | По умолчанию                       | Описание                                       |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                     |
| `symbol`   | `"🐹 "`                             | A format string representing the symbol of Go. |
| `style`    | `"bold cyan"`                      | Стиль модуля.                                  |
| `disabled` | `false`                            | Отключает модуль `golang`.                     |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v1.12.1` | The version of `go`                  |
| symbol     |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of Helm. The module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `helmfile.yaml`
- The current directory contains a `Chart.yaml` file

### Опции

| Параметр   | По умолчанию                       | Описание                                         |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"⎈ "`                             | A format string representing the symbol of Helm. |
| `style`    | `"bold white"`                     | Стиль модуля.                                    |
| `disabled` | `false`                            | Disables the `helm` module.                      |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v3.1.1` | The version of `helm`                |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Имя хоста

The `hostname` module shows the system hostname.

### Опции

| Параметр   | По умолчанию                | Описание                                                                                                                                   |
| ---------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                      | Показывать имя хоста только при подключении к SSH-сессии.                                                                                  |
| `trim_at`  | `"."`                       | Символы, по которую имя хоста будет сокращено после первого совпадения. `"."` остановится после первой точки. `""` отключит любое усечение |
| `format`   | `"[$hostname]($style) in "` | The format for the module.                                                                                                                 |
| `style`    | `"bold dimmed green"`       | Стиль модуля.                                                                                                                              |
| `disabled` | `false`                     | Отключает модуль `hostname`.                                                                                                               |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| symbol     |        | Mirrors the value of option `symbol` |
| style\*  |        | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java. The module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### Опции

| Параметр   | По умолчанию                           | Описание                                        |
| ---------- | -------------------------------------- | ----------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | The format for the module.                      |
| `symbol`   | `"☕ "`                                 | A format string representing the symbol of Java |
| `style`    | `"red dimmed"`                         | Стиль модуля.                                   |
| `disabled` | `false`                                | Отключает модуль `java`.                        |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| version    | `v14`  | The version of `java`                |
| symbol     |        | Mirrors the value of option `symbol` |
| style\*  |        | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Задачи

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Опции

| Параметр    | По умолчанию                  | Описание                                         |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | Показывать количество задач, если превышено.     |
| `format`    | `"[$symbol$number]($style) "` | The format for the module.                       |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | Стиль модуля.                                    |
| `disabled`  | `false`                       | Отключает модуль `jobs`.                         |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| number     | `1`    | The number of jobs                   |
| symbol     |        | Mirrors the value of option `symbol` |
| style\*  |        | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. The module will be shown if any of the following conditions are met:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Опции

| Параметр   | По умолчанию                       | Описание                                          |
| ---------- | ---------------------------------- | ------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                        |
| `symbol`   | `"ஃ "`                             | A format string representing the symbol of Julia. |
| `style`    | `"bold purple"`                    | Стиль модуля.                                     |
| `disabled` | `false`                            | Disables the `julia` module.                      |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v1.4.0` | The version of `julia`               |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Опции

| Параметр          | По умолчанию                                         | Описание                                                              |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | The format for the module.                                            |
| `style`           | `"cyan bold"`                                        | Стиль модуля.                                                         |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Отключает модуль `kubernetes`.                                        |

### Переменные

| Переменная | Пример               | Описание                                 |
| ---------- | -------------------- | ---------------------------------------- |
| context    | `starship-cluster`   | The current kubernetes context           |
| namespace  | `starship-namespace` | If set, the current kubernetes namespace |
| symbol     |                      | Mirrors the value of option `symbol`     |
| style\*  |                      | Mirrors the value of option `style`      |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Перевод Строки

The `line_break` module separates the prompt into two lines.

### Опции

| Параметр   | По умолчанию | Описание                                                                 |
| ---------- | ------------ | ------------------------------------------------------------------------ |
| `disabled` | `false`      | Отключает модуль `line_break`, отображая командную строку в одну строку. |

### Пример

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

The `lua` module shows the currently installed version of Lua. The module will be shown if any of the following conditions are met:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Опции

| Параметр     | По умолчанию                       | Описание                                                                   |
| ------------ | ---------------------------------- | -------------------------------------------------------------------------- |
| `format`     | `"via [$symbol$version]($style) "` | The format for the module.                                                 |
| `symbol`     | `"🌙 "`                             | A format string representing the symbol of Lua.                            |
| `style`      | `"bold blue"`                      | Стиль модуля.                                                              |
| `lua_binary` | `"lua"`                            | Configures the lua binary that Starship executes when getting the version. |
| `disabled`   | `false`                            | Disables the `lua` module.                                                 |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v5.4.0` | The version of `lua`                 |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Опции

| Параметр    | По умолчанию                                  | Описание                                                 |
| ----------- | --------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                          | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | The format for the module.                               |
| `symbol`    | `"🐏"`                                         | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                         | Стиль модуля.                                            |
| `disabled`  | `true`                                        | Disables the `memory_usage` module.                      |

### Переменные

| Переменная       | Пример        | Описание                                                           |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Mirrors the value of option `symbol`                               |
| style\*        |               | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

### Пример

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Опции

| Параметр            | По умолчанию                     | Описание                                                                                     |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | Стиль модуля.                                                                                |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.                                                                   |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`                            | Символ, используемый для обозначения усечения названия ветки.                                |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| branch     | `master` | The active mercurial branch          |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. The module will be shown if any of the following conditions are met:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Опции

| Параметр   | По умолчанию                       | Описание                                              |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module                             |
| `symbol`   | `"👑 "`                             | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"`                    | Стиль модуля.                                         |
| `disabled` | `false`                            | Disables the `nim` module.                            |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v1.2.0` | The version of `nimc`                |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### Опции

| Параметр     | По умолчанию                                   | Описание                                              |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | The format for the module.                            |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | Стиль модуля.                                         |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### Переменные

| Переменная | Пример  | Описание                             |
| ---------- | ------- | ------------------------------------ |
| state      | `pure`  | The state of the nix-shell           |
| name       | `lorri` | The name of the nix-shell            |
| symbol     |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. The module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Опции

| Параметр   | По умолчанию                       | Описание                                           |
| ---------- | ---------------------------------- | -------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                         |
| `symbol`   | `"⬢ "`                             | A format string representing the symbol of NodeJS. |
| `style`    | `"bold green"`                     | Стиль модуля.                                      |
| `disabled` | `false`                            | Disables the `nodejs` module.                      |

###  Variables

| Переменная | Пример     | Описание                             |
| ---------- | ---------- | ------------------------------------ |
| version    | `v13.12.0` | The version of `node`                |
| symbol     |            | Mirrors the value of option `symbol` |
| style\*  |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Опции

| Параметр   | По умолчанию                       | Описание                                                |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                       |
| `symbol`   | `"🐫 "`                             | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"`                    | Стиль модуля.                                           |
| `disabled` | `false`                            | Disables the `ocaml` module.                            |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v4.10.0` | The version of `ocaml`               |
| symbol     |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Опции

| Параметр   | По умолчанию                                        | Описание                                                       |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | The format for the module.                                     |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | Стиль модуля.                                                  |
| `disabled` | `false`                                             | Disables the `OpenStack` module.                               |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| cloud      | `corp` | The current OpenStack cloud          |
| project    | `dev`  | The current OpenStack project        |
| symbol     |        | Mirrors the value of option `symbol` |
| style\*  |        | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present
- **helm** - The `helm` chart version is extracted from the `Chart.yaml` present
- **maven** - The `maven` package version is extracted from the `pom.xml` present

> ⚠ Показана версия пакета, исходный код которого находится в текущем каталоге, а не в менеджере пакетов.

### Опции

| Параметр          | По умолчанию                       | Описание                                                   |
| ----------------- | ---------------------------------- | ---------------------------------------------------------- |
| `format`          | `"via [$symbol$version]($style) "` | The format for the module.                                 |
| `symbol`          | `"📦 "`                             | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"`                       | Стиль модуля.                                              |
| `display_private` | `false`                            | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`                            | Disables the `package` module.                             |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v1.0.0` | The version of your package          |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of Perl. The module will be shown if any of the following conditions are met:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Опции

| Параметр   | По умолчанию                       | Описание                                              |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                     |
| `symbol`   | `"🐪 "`                             | The symbol used before displaying the version of Perl |
| `style`    | `"bold 149"`                       | Стиль модуля.                                         |
| `disabled` | `false`                            | Disables the `perl` module.                           |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v5.26.1` | The version of `perl`                |
| symbol     |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

### Пример

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of PHP. The module will be shown if any of the following conditions are met:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### Опции

| Параметр   | По умолчанию                       | Описание                                              |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `symbol`   | `"🐘 "`                             | The symbol used before displaying the version of PHP. |
| `style`    | `"147 bold"`                       | Стиль модуля.                                         |
| `disabled` | `false`                            | Disables the `php` module.                            |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v7.3.8` | The version of `php`                 |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. The module will be shown if any of the following conditions are met:

- The current directory contains a `spago.dhall` file
- The current directory contains a \*.purs files

### Опции

| Параметр   | По умолчанию                       | Описание                                                     |
| ---------- | ---------------------------------- | ------------------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                                   |
| `symbol`   | `"<=> "`                     | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"`                     | Стиль модуля.                                                |
| `disabled` | `false`                            | Disables the `purescript` module.                            |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `0.13.5` | The version of `purescript`          |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

The module will be shown if any of the following conditions are met:

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- The current directory contains a `setup.py` file
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### Опции

| Параметр             | По умолчанию                                                              | Описание                                                                      |
| -------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}${version}( \($virtualenv\))]($style) '` | The format for the module.                                                    |
| `symbol`             | `"🐍 "`                                                                    | A format string representing the symbol of Python                             |
| `style`              | `"yellow bold"`                                                           | Стиль модуля.                                                                 |
| `pyenv_version_name` | `false`                                                                   | Use pyenv to get Python version                                               |
| `pyenv_prefix`       | `pyenv`                                                                   | Prefix before pyenv version display, only used if pyenv is used               |
| `scan_for_pyfiles`   | `true`                                                                    | If false, Python files in the current directory will not show this module.    |
| `python_binary`      | `python`                                                                  | Configures the python binary that Starship executes when getting the version. |
| `disabled`           | `false`                                                                   | Disables the `python` module.                                                 |

### Переменные

| Переменная   | Пример          | Описание                                   |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Mirrors the value of option `symbol`       |
| style        | `"yellow bold"` | Mirrors the value of option `style`        |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |


### Пример

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

Using the `python3` binary to get the version.

Note - The `python_binary` variable changes the binary that Starship executes to get the version of Python, it doesn't change the arguments that are used.

```toml
# ~/.config/starship.toml

[python]
python_binary = "python3"
```

## Ruby

The `ruby` module shows the currently installed version of Ruby. The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Опции

| Параметр   | По умолчанию                       | Описание                                         |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"💎 "`                             | A format string representing the symbol of Ruby. |
| `style`    | `"bold red"`                       | Стиль модуля.                                    |
| `disabled` | `false`                            | Disables the `ruby` module.                      |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v2.5.1` | The version of `ruby`                |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust. The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Опции

| Параметр   | По умолчанию                       | Описание                                        |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🦀 "`                             | A format string representing the symbol of Rust |
| `style`    | `"bold red"`                       | Стиль модуля.                                   |
| `disabled` | `false`                            | Disables the `rust` module.                     |

### Переменные

| Переменная | Пример            | Описание                             |
| ---------- | ----------------- | ------------------------------------ |
| version    | `v1.43.0-nightly` | The version of `rustc`               |
| symbol     |                   | Mirrors the value of option `symbol` |
| style\*  |                   | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## SHLVL

The `shlvl` module shows the current SHLVL ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Опции

| Параметр    | По умолчанию                 | Описание                                |
| ----------- | ---------------------------- | --------------------------------------- |
| `threshold` | `2`                          | Display threshold.                      |
| `format`    | `"[$symbol$shlvl]($style) "` | The format for the module.              |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the SHLVL. |
| `style`     | `"bold yellow"`              | Стиль модуля.                           |
| `disabled`  | `true`                       | Disables the `shlvl` module.            |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| shlvl      | `3`    | The current value of SHLVL           |
| symbol     |        | Mirrors the value of option `symbol` |
| style\*  |        | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### Опции

| Параметр   | По умолчанию                     | Описание                                         |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | The format for the module.                       |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | Стиль модуля.                                    |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Переменные

| Переменная | Пример       | Описание                             |
| ---------- | ------------ | ------------------------------------ |
| env        | `centos.img` | The current singularity image        |
| symbol     |              | Mirrors the value of option `symbol` |
| style\*  |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file. :::

### Опции

| Параметр   | По умолчанию               | Описание                                               |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `[$symbol$status]($style)` | The format of the module                               |
| `symbol`   | `"✖"`                      | A format string representing the symbol for the status |
| `style`    | `"bold red"`               | Стиль модуля.                                          |
| `disabled` | `true`                     | Disables the `status` module.                          |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| status     | `127`  | The exit code of the last command    |
| symbol     |        | Mirrors the value of option `symbol` |
| style\*  |        | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "💣 "
format = '[\[$symbol$status\]]($style) '
disabled = false

```

## Swift

The `swift` module shows the currently installed version of Swift. The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Опции

| Параметр   | По умолчанию                       | Описание                                         |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"🐦 "`                             | A format string representing the symbol of Swift |
| `style`    | `"bold 202"`                       | Стиль модуля.                                    |
| `disabled` | `false`                            | Disables the `swift` module.                     |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v5.2.4` | The version of `swift`               |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-version). The module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

### Опции

| Параметр   | По умолчанию                         | Описание                                              |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`   | `"💠 "`                               | A format string shown before the terraform workspace. |
| `style`    | `"bold 105"`                         | Стиль модуля.                                         |
| `disabled` | `false`                              | Disables the `terraform` module.                      |

### Переменные

| Переменная | Пример     | Описание                             |
| ---------- | ---------- | ------------------------------------ |
| version    | `v0.12.24` | The version of `terraform`           |
| workspace  | `default`  | The current terraform workspace      |
| symbol     |            | Mirrors the value of option `symbol` |
| style\*  |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

#### With Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Время

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Опции

| Параметр          | По умолчанию            | Описание                                                                                                                           |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                  |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Переменные

| Переменная | Пример     | Описание                            |
| ---------- | ---------- | ----------------------------------- |
| time       | `13:08:10` | The current time.                   |
| style\*  |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### Опции

| Параметр      | По умолчанию            | Описание                              |
| ------------- | ----------------------- | ------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root. |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.    |
| `format`      | `"[$user]($style) in "` | The format for the module.            |
| `show_always` | `false`                 | Always shows the `username` module.   |
| `disabled`    | `false`                 | Disables the `username` module.       |

### Переменные

| Переменная | Пример       | Описание                                                                                    |
| ---------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`    | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`     | `"matchai"`  | The currently logged-in user ID.                                                            |

### Пример

```toml
# ~/.config/starship.toml

[username]
style_user = "white bold"
style_root = "black bold"
format = "user: [$user]($style) "
disabled = false
show_always = true
```

## Zig

The `zig` module shows the currently installed version of Zig. The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### Опции

| Параметр   | По умолчанию                       | Описание                                              |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`                             | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"`                    | Стиль модуля.                                         |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `disabled` | `false`                            | Disables the `zig` module.                            |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v0.6.0` | The version of `zig`                 |
| symbol     |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Пример

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `files`
- The current directory contains a directory whose name is in `directories`
- The current directory contains a file whose extension is in `extensions`
- The `when` command returns 0

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

### Опции

| Параметр      | По умолчанию                  | Описание                                                                                                                   |
| ------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                               | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
| `when`        |                               | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                               | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"`     | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                          | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                          | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                          | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                          | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                | Стиль модуля.                                                                                                              |
| `format`      | `"[$symbol$output]($style) "` | The format for the module.                                                                                                 |
| `disabled`    | `false`                       | Disables this `custom` module.                                                                                             |

### Переменные

| Переменная | Описание                               |
| ---------- | -------------------------------------- |
| output     | The output of shell command in `shell` |
| symbol     | Mirrors the value of option `symbol`   |
| style\*  | Mirrors the value of option `style`    |

\*: This variable can only be used as a part of a style string

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ["pwsh", "-Command", "-"]
```

::: warning Make sure your custom shell configuration exits gracefully

If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).

For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.

Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.

Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

:::

### Пример

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
