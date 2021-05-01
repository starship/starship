# Конфигурация

Чтобы начать конфигурацию Starship, создайте следующий файл: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Вся конфигурация Starship выполняется в этом файле [TOML](https://github.com/toml-lang/toml):

```toml
# Inserts a blank line between shell prompts
add_newline = true

# Replace the "❯" symbol in the prompt with "➜"
[character]                            # The name of the module we are configuring is "character"
success_symbol = "[➜](bold green)"     # The "success_symbol" segment is being set to "➜" with the color "bold green"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

You can change default configuration file location with `STARSHIP_CONFIG` environment variable:

```sh
export STARSHIP_CONFIG=~/.starship/config.toml
```

Аналогично в PowerShell (Windows) следует добавить эту строку в `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
```

### Логгирование (Запись действий)

По умолчанию в starship записываются предупреждения и ошибки в файл с именем `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, где ключ сессии соответствует экземпляру терминала. Это, однако, может быть изменено с помощью переменной окружения `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Аналогично в PowerShell (Windows) следует добавить эту строку в `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### Терминология

**Модуль**: Компонент строки, дающий информацию на основе контекстной информации вашей ОС. For example, the "nodejs" module shows the version of Node.js that is currently installed on your computer, if your current directory is a Node.js project.

**Variable**: Smaller sub-components that contain information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of Node.js.

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
- `[⌘ $version](bold green)` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
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

| Параметр          | По умолчанию                     | Описание                                                     |
| ----------------- | -------------------------------- | ------------------------------------------------------------ |
| `format`          | [ссылка](#default-prompt-format) | Настройка форматирования оболочки.                           |
| `scan_timeout`    | `30`                             | Тайм-аут запуска сканирования файлов (в миллисекундах).      |
| `command_timeout` | `500`                            | Timeout for commands executed by starship (in milliseconds). |
| `add_newline`     | `true`                           | Inserts blank line between shell prompts.                    |

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

# Disable the blank line at the start of the prompt
add_newline = false
```

### Формат оболочки по умолчанию

Формат по умолчанию `format` используется для определения формата подсказки (prompt), если `format` пустой или отсутствует. Значение по умолчанию:

```toml
format = "$all"

# Which is equivalent to
format = """
$username\
$hostname\
$shlvl\
$kubernetes\
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$kotlin\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vagrant\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$env_var\
$crystal\
$custom\
$cmd_duration\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

## AWS

Модуль `aws` показывает текущий регион и профиль AWS. Основано на `AWS_REGION`, `AWS_DEFAULT_REGION`, и `AWS_PROFILE` переменных окружения и файле`~/.aws/config`.

При использовании [aws-vault](https://github.com/99designs/aws-vault) профиль читается из переменной среды `AWS_VAULT`.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var.

### Опции

| Параметр         | По умолчанию                                        | Описание                                                       |
| ---------------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`         | `'on [$symbol($profile )(\($region\) )]($style)'` | Формат модуля.                                                 |
| `symbol`         | `"☁️ "`                                             | Символ перед отображением текущего профиля AWS.                |
| `region_aliases` |                                                     | Таблица региона псевдонимов, отображаемая вместе с именем AWS. |
| `style`          | `"bold yellow"`                                     | Стиль модуля.                                                  |
| `disabled`       | `false`                                             | Disables the `aws` module.                                     |

### Переменные

| Переменная | Пример           | Описание                             |
| ---------- | ---------------- | ------------------------------------ |
| регион     | `ap-northeast-1` | Текущий регион AWS                   |
| профиль    | `astronauts`     | Текущий профиль AWS                  |
| symbol     |                  | Отражает значение параметра `symbol` |
| style\*  |                  | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Примеры

#### Отобразить все

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Отобразить регион

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

#### Отобразить профиль

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

| Параметр             | По умолчанию                      | Описание                                                |
| -------------------- | --------------------------------- | ------------------------------------------------------- |
| `full_symbol`        | `" "`                            | Символ, отображаемый при полной батарее.                |
| `charging_symbol`    | `" "`                            | Символ, показываемый при зарядке аккумулятора.          |
| `discharging_symbol` | `" "`                            | Символ, показываемый при разрядке аккумулятора.         |
| `unknown_symbol`     | `" "`                            | Символ, отображаемый при неизвестном состоянии батареи. |
| `empty_symbol`       | `" "`                            | Символ, отображаемый при пустом состоянии батареи.      |
| `format`             | `"[$symbol$percentage]($style) "` | Формат модуля.                                          |
| `display`            | [ссылка](#battery-display)        | Порог отображения и стиль для модуля.                   |
| `disabled`           | `false`                           | Отключает модуль `battery`.                             |

### Пример

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### Отображение батареи

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style). Если `display` не предоставлено. Значение по умолчанию:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Опции

Опция `display` представляет собой массив следующей таблицы.

| Параметр             | По умолчанию | Описание                                                                                                  |
| -------------------- | ------------ | --------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | Верхняя граница опции отображения.                                                                        |
| `style`              | `bold red`   | Используемый стиль, если используется опция отображения.                                                  |
| `charging_symbol`    | `-`          | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` | `-`          | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### Пример

```toml
[[battery.display]]  # "bold red" style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"
discharging_symbol = 💦

# when capacity is over 30%, the battery indicator will not be displayed

```

## Символ

Модуль `character` показывает символ (обычно, стрелка) рядом с вводимым текстом в терминале.

Символ показывает, была ли последняя команда успешной или нет. It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change its shape take a look at [this example](#with-custom-error-shape).

::: warning `error_symbol` is not supported on elvish shell. :::

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

### Примеры

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

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). By default the module will be activated if any of the following conditions are met:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### Опции

| Параметр            | По умолчанию                           | Описание                                                                  |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`   | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"△ "`                                 | The symbol used before the version of cmake.                              |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this module                               |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Which filenames should trigger this module                                |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module                                  |
| `style`             | `"bold blue"`                          | Стиль модуля.                                                             |
| `disabled`          | `false`                                | Disables the `cmake` module.                                              |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v3.17.3` | The version of cmake                 |
| symbol     |           | Отражает значение параметра `symbol` |
| style\*  |           | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

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
| `format`             | `"took [$duration]($style) "` | Формат модуля.                                                       |
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
| style\*  |          | Отражает значение параметра `style`     |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Конда

Модуль `conda` показывает текущее окружение conda, если `$CONDA_DEFAULT_ENV` присвоено значение.

::: tip

Это не подавляет модификатор командной строки самой conda. Возможно, вы захотите запустить `conda config --set changeps1 False`.

:::

### Опции

| Параметр            | По умолчанию                           | Описание                                                                                                                                                                                                     |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `1`                                    | Количество каталогов, в которых путь к окружению должен быть усечен, если окружение было создано через `conda create -p [path]`. `0` означает без усечения. Также смотрите модуль [`directory`](#directory). |
| `symbol`            | `"🅒 "`                                 | Символ перед названием окружения.                                                                                                                                                                            |
| `style`             | `"bold green"`                         | Стиль модуля.                                                                                                                                                                                                |
| `format`            | `"via [$symbol$environment]($style) "` | Формат модуля.                                                                                                                                                                                               |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                   |
| `disabled`          | `false`                                | Отключает модуль `conda`.                                                                                                                                                                                    |

### Переменные

| Переменная  | Пример       | Описание                             |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current conda environment        |
| symbol      |              | Отражает значение параметра `symbol` |
| style\*   |              | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/). By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `shard.yml`
- Текущий каталог содержит файл `.cr`

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | Символ, используемый перед отображением версии crystal.                   |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | Стиль модуля.                                                             |
| `detect_extensions` | `["cr"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["shard.yml"]`                      | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Отключает модуль `crystal`.                                               |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v0.32.1` | The version of `crystal`             |
| symbol     |           | Отражает значение параметра `symbol` |
| style\*  |           | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml`, `pubspec.yml` or `pubspec.lock` file

### Опции

| Параметр            | По умолчанию                                      | Описание                                                                  |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | A format string representing the symbol of Dart                           |
| `detect_extensions` | `["dart"]`                                        | Which extensions should trigger this module.                              |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".dart_tool"]`                                  | Which folders should trigger this module.                                 |
| `style`             | `"bold blue"`                                     | Стиль модуля.                                                             |
| `disabled`          | `false`                                           | Disables the `dart` module.                                               |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v2.8.4` | The version of `dart`                |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/). By default the module will be shown if any of the following conditions are met:
- Текущий каталог содержит файл `mod.ts`, `mod.js`, `deps.ts` или `deps.js`

### Опции

| Параметр            | По умолчанию                                 | Описание                                                                  |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                                     | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                       | A format string representing the symbol of Deno                           |
| `detect_extensions` | `[]`                                         | Which extensions should trigger this module.                              |
| `detect_files`      | `["mod.ts", "mod.js", "deps.ts", "deps.js"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                         | Which folders should trigger this module.                                 |
| `style`             | `"green bold"`                               | Стиль модуля.                                                             |
| `disabled`          | `false`                                      | Disables the `deno` module.                                               |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v1.8.3` | The version of `deno`                |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

### Пример

```toml
# ~/.config/starship.toml

[deno]
format = "via [🦕 $version](green bold) "
```

## Каталог

Модуль `directory` показывает путь к вашей текущей директории, усеченной до трех родительских папок. Ваш каталог также будет отсечен до корня git репозитория, в котором вы находитесь.

При использовании стиля оболочки fish, вместо скрытия усеченного каталога, вы увидите укороченное имя каталога, зависимое от числа символов вы установите для этой опции.

Например, возьмем `~/Dev/Nix/nixpkgs/pkgs` где `nixpkgs` является корневым репозиторием, и в опции установлено `1`. Вы увидите `~/D/N/nixpkgs/pkgs`, а до этого было бы `nixpkgs/pkgs`.

### Опции

| Параметр            | По умолчанию                                       | Описание                                                                     |
| ------------------- | -------------------------------------------------- | ---------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                | Количество родительских папок, к которым должен быть усечен текущий каталог. |
| `truncate_to_repo`  | `true`                                             | Следует или нет обрезать до корня репозитория git, в котором вы находитесь.  |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | Формат модуля.                                                               |
| `style`             | `"bold cyan"`                                      | Стиль модуля.                                                                |
| `disabled`          | `false`                                            | Отключает модуль `directory`.                                                |
| `read_only`         | `"🔒"`                                              | The symbol indicating current directory is read only.                        |
| `read_only_style`   | `"red"`                                            | The style for the read only symbol.                                          |
| `truncation_symbol` | `""`                                               | The symbol to prefix to truncated paths. eg: "…/"                            |
| `home_symbol`       | `"~"`                                              | The symbol indicating home directory.                                        |

<details>
<summary>Этот модуль имеет несколько расширенных опций конфигурации, которые контролируют отображение каталога.</summary>

| Advanced Option             | По умолчанию | Описание                                                                                                                                                               |
| --------------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |              | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`          | Количество символов, используемых при использовании логики создания пути из fish.                                                                                      |
| `use_logical_path`          | `true`       | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` взаимодействует со стандартными параметрами усечения, которые могут сначала показаться странными: если он не равен нулю, элементы пути, который обычно усекается, вместо этого отображаются с указанным количеством символов. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Переменные

| Переменная | Пример                | Описание                            |
| ---------- | --------------------- | ----------------------------------- |
| path       | `"D:/Projects"`       | The current directory path          |
| style\*  | `"black bold dimmed"` | Отражает значение параметра `style` |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Контекст Docker

Модуль `docker_context` показывает текущий активный [контекст Docker](https://docs.docker.com/engine/context/working-with-contexts/), если он не установлен как `default`.

### Опции

| Параметр            | По умолчанию                                                  | Описание                                                                          |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | Формат модуля.                                                                    |
| `symbol`            | `"🐳 "`                                                        | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                        | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                          | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                          | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `"blue bold"`                                                 | Стиль модуля.                                                                     |
| `disabled`          | `false`                                                       | Disables the `docker_context` module.                                             |

### Переменные

| Переменная | Пример         | Описание                             |
| ---------- | -------------- | ------------------------------------ |
| context    | `test_context` | The current docker context           |
| symbol     |                | Отражает значение параметра `symbol` |
| style\*  |                | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. Если SDK был закреплен в текущей директории, будет показана закрепленная версия. В противном случае модуль отображает последнюю установленную версию SDK.

By default this module will only be shown in your prompt when one or more of the following files are present in the current directory:

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

Внутренне этот модуль использует свой собственный механизм определения версий. Обычно он в два раза быстрее, чем `dotnet --version`, но он может показывать неправильную версию, если ваш .NET проект имеет необычный формат каталога. Если точность важнее, чем скорость, вы можете отключить механизм опцией `heuristic = false` в настройках модуля.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Опции

| Параметр            | По умолчанию                                                                                            | Описание                                                                  |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"[$symbol($version )(🎯 $tfm )]($style)"`                                                               | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                                                                                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | Символ перед отображением текущей версии dotnet.                          |
| `heuristic`         | `true`                                                                                                  | Использовать быстрое определение версии, для сохранения скорости.         |
| `detect_extensions` | `["sln", "csproj", "fsproj", "xproj"]`                                                                  | Which extensions should trigger this module.                              |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                    | Which folders should trigger this modules.                                |
| `style`             | `"bold blue"`                                                                                           | Стиль модуля.                                                             |
| `disabled`          | `false`                                                                                                 | Отключает модуль `dotnet`.                                                |

### Переменные

| Переменная | Пример           | Описание                                                           |
| ---------- | ---------------- | ------------------------------------------------------------------ |
| version    | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm        | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol     |                  | Отражает значение параметра `symbol`                               |
| style\*  |                  | Отражает значение параметра `style`                                |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/). By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `mix.exs`.

### Опции

| Параметр            | По умолчанию                                                | Описание                                                                  |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                                         |
| `version_format`    | `v{raw}`                                                    | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | Символ, используемый перед отображением версии Elixir/Erlang.             |
| `detect_extensions` | `[]`                                                        | Which extensions should trigger this module.                              |
| `detect_files`      | `["mix.exs"]`                                               | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                        | Which folders should trigger this modules.                                |
| `style`             | `"bold purple"`                                             | Стиль модуля.                                                             |
| `disabled`          | `false`                                                     | Отключает модуль `elixir`.                                                |

### Переменные

| Переменная  | Пример  | Описание                             |
| ----------- | ------- | ------------------------------------ |
| version     | `v1.10` | The version of `elixir`              |
| otp_version |         | The otp version of `elixir`          |
| symbol      |         | Отражает значение параметра `symbol` |
| style\*   |         | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/). By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `elm.json`
- Текущий каталог содержит файл `elm-package.json`
- Текущий каталог содержит файл `.elm-version`
- Текущий каталог содержит папку `elm-stuff`
- Текущий каталог содержит файлы `*.elm`

### Опции

| Параметр            | По умолчанию                                       | Описание                                                                  |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                                           | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | A format string representing the symbol of Elm.                           |
| `detect_extensions` | `["elm"]`                                          | Which extensions should trigger this module.                              |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `["elm-stuff"]`                                    | Which folders should trigger this modules.                                |
| `style`             | `"cyan bold"`                                      | Стиль модуля.                                                             |
| `disabled`          | `false`                                            | Отключает модуль `elm`.                                                   |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v0.19.1` | The version of `elm`                 |
| symbol     |           | Отражает значение параметра `symbol` |
| style\*  |           | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Переменная Окружения

Модуль `env_var` отображает текущее значение выбранной переменной окружения. Модуль будет показан только в том случае, если любое из следующих условий соблюдено:

- Опция `variable` соответствует существующей переменной среды
- Опция `variable` не определена, но определена опция `default`

### Опции

| Параметр   | По умолчанию                   | Описание                                                         |
| ---------- | ------------------------------ | ---------------------------------------------------------------- |
| `symbol`   |                                | Символ, используемый перед отображением значения переменной.     |
| `variable` |                                | Отображаемая переменная окружения.                               |
| `default`  |                                | Значение отображаемое, когда выбранная переменная не определена. |
| `format`   | `"with [$env_value]($style) "` | Формат модуля.                                                   |
| `disabled` | `false`                        | Отключает модуль `env_var`.                                      |

### Переменные

| Переменная | Пример                                      | Описание                                   |
| ---------- | ------------------------------------------- | ------------------------------------------ |
| env_value  | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol     |                                             | Отражает значение параметра `symbol`       |
| style\*  | `black bold dimmed`                         | Отражает значение параметра `style`        |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `rebar.config`.
- Текущий каталог содержит файл `erlang.mk`.

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | The symbol used before displaying the version of erlang.                  |
| `style`             | `"bold red"`                         | Стиль модуля.                                                             |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `disabled`          | `false`                              | Disables the `erlang` module.                                             |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v22.1.3` | The version of `erlang`              |
| symbol     |           | Отражает значение параметра `symbol` |
| style\*  |           | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Опции

| Параметр         | По умолчанию                                               | Описание                                                        |
| ---------------- | ---------------------------------------------------------- | --------------------------------------------------------------- |
| `format`         | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Формат модуля.                                                  |
| `symbol`         | `"☁️  "`                                                   | The symbol used before displaying the current GCP profile.      |
| `region_aliases` |                                                            | Table of region aliases to display in addition to the GCP name. |
| `style`          | `"bold blue"`                                              | Стиль модуля.                                                   |
| `disabled`       | `false`                                                    | Disables the `gcloud` module.                                   |

### Переменные

| Переменная | Пример        | Описание                                                           |
| ---------- | ------------- | ------------------------------------------------------------------ |
| регион     | `us-central1` | The current GCP region                                             |
| account    | `foo`         | The current GCP profile                                            |
| domain     | `example.com` | The current GCP profile domain                                     |
| project    |               | The current GCP project                                            |
| active     | `default`     | The active config name written in `~/.config/gcloud/active_config` |
| symbol     |               | Отражает значение параметра `symbol`                               |
| style\*  |               | Отражает значение параметра `style`                                |

\*: Эта переменная может использоваться только в качестве части строки style

### Примеры

#### Display account and project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
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

Модуль `git_branch` показывает активную ветку репозитория в вашем текущей директории.

### Опции

| Параметр             | По умолчанию                     | Описание                                                                                      |
| -------------------- | -------------------------------- | --------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Shows the remote tracking branch name, even if it is equal to the local branch name.          |
| `format`             | `"on [$symbol$branch]($style) "` | Формат модуля. Use `"$branch"` to refer to the current branch name.                           |
| `symbol`             | `" "`                           | A format string representing the symbol of git branch.                                        |
| `style`              | `"bold purple"`                  | Стиль модуля.                                                                                 |
| `truncation_length`  | `2^63 - 1`                       | Truncates a git branch to `N` graphemes.                                                      |
| `truncation_symbol`  | `"…"`                            | Символ, используемый для обозначения усечения названия ветки. You can use `""` for no symbol. |
| `only_attached`      | `false`                          | Only show the branch name when not in a detached `HEAD` state.                                |
| `disabled`           | `false`                          | Отключает модуль `git_branch`.                                                                |

### Переменные

| Переменная    | Пример   | Описание                                                                                               |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | Отражает значение параметра `symbol`                                                                   |
| style\*     |          | Отражает значение параметра `style`                                                                    |

\*: Эта переменная может использоваться только в качестве части строки style

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

| Параметр             | По умолчанию                                           | Описание                                                |
| -------------------- | ------------------------------------------------------ | ------------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | Длина отображаемого хэша коммита git.                   |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | Формат модуля.                                          |
| `style`              | `"bold green"`                                         | Стиль модуля.                                           |
| `only_detached`      | `true`                                                 | Only show git commit hash when in detached `HEAD` state |
| `tag_disabled`       | `true`                                                 | Disables showing tag info in `git_commit` module.       |
| `tag_symbol`         | `"🏷 "`                                                 | Tag symbol prefixing the info shown                     |
| `disabled`           | `false`                                                | Отключает модуль `git_commit`.                          |

### Переменные

| Переменная | Пример    | Описание                            |
| ---------- | --------- | ----------------------------------- |
| hash       | `b703eb3` | The current git commit hash         |
| style\*  |           | Отражает значение параметра `style` |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Состояние Git

Модуль `git_state` будет отображаться в директориях, являющимися частью репозитория git, и там, где выполняется операция, такие как: _REBASING_, _BISECTING_, и т. д. Если есть информация о прогрессе (например, REBASING 3/10), эта информация также будет показана.

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
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Формат модуля.                                                                          |
| `disabled`     | `false`                                                         | Отключает модуль `git_state`.                                                           |

### Переменные

| Переменная       | Пример     | Описание                            |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*        |            | Отражает значение параметра `style` |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Статус Git

Модуль `git_status` отображает символы, представляющие состояние репозитория в вашей текущей директории.

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
| style\*      | Отражает значение параметра `style`                                                           |

\*: Эта переменная может использоваться только в качестве части строки style

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

Показывать счетчик впереди/позади для отслеживаемой ветки

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

The `golang` module shows the currently installed version of [Golang](https://golang.org/). By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `go.mod`
- Текущий каталог содержит файл `go.sum`
- Текущий каталог содержит файл `glide.yaml`
- Текущий каталог содержит файл `Gopkg.yml`
- Текущий каталог содержит файл `Gopkg.lock`
- The current directory contains a `.go-version` file
- Текущий каталог содержит папку `Godeps`
- Текущий каталог содержит файл с расширением `.go`

### Опции

| Параметр            | По умолчанию                                                                   | Описание                                                                  |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                           | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                                                                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                         | A format string representing the symbol of Go.                            |
| `detect_extensions` | `["go"]`                                                                       | Which extensions should trigger this module.                              |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `["Godeps"]`                                                                   | Which folders should trigger this module.                                 |
| `style`             | `"bold cyan"`                                                                  | Стиль модуля.                                                             |
| `disabled`          | `false`                                                                        | Отключает модуль `golang`.                                                |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v1.12.1` | The version of `go`                  |
| symbol     |           | Отражает значение параметра `symbol` |
| style\*  |           | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `helmfile.yaml`
- The current directory contains a `Chart.yaml` file

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm.                          |
| `style`             | `"bold white"`                       | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Disables the `helm` module.                                               |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v3.1.1` | The version of `helm`                |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Имя хоста

Модуль `hostname` отображает имя системы (хоста).

### Опции

| Параметр   | По умолчанию                | Описание                                                                                                                                   |
| ---------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                      | Показывать имя хоста только при подключении к SSH-сессии.                                                                                  |
| `trim_at`  | `"."`                       | Символы, по которую имя хоста будет сокращено после первого совпадения. `"."` остановится после первой точки. `""` отключит любое усечение |
| `format`   | `"[$hostname]($style) in "` | Формат модуля.                                                                                                                             |
| `style`    | `"bold dimmed green"`       | Стиль модуля.                                                                                                                              |
| `disabled` | `false`                     | Отключает модуль `hostname`.                                                                                                               |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| symbol     |        | Отражает значение параметра `symbol` |
| style\*  |        | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

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

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Опции

| Параметр            | По умолчанию                                                                                              | Описание                                                                  |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                      | Which folders should trigger this modules.                                |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java                           |
| `style`             | `"red dimmed"`                                                                                            | Стиль модуля.                                                             |
| `disabled`          | `false`                                                                                                   | Отключает модуль `java`.                                                  |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| version    | `v14`  | The version of `java`                |
| symbol     |        | Отражает значение параметра `symbol` |
| style\*  |        | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Задачи

Модуль `jobs` отображает текущее количество запущенных работ. Модуль будет показан только если работы выполняются в фоне. Модуль покажет количество запущенных задач при наличии более чем 1 задачи, или больше, чем значение конфигурации `threshold`, если оно существует. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

::: warning

This module is not supported on tcsh.

:::

### Опции

| Параметр    | По умолчанию                  | Описание                                         |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | Показывать количество задач, если превышено.     |
| `format`    | `"[$symbol$number]($style) "` | Формат модуля.                                   |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | Стиль модуля.                                    |
| `disabled`  | `false`                       | Отключает модуль `jobs`.                         |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| number     | `1`    | The number of jobs                   |
| symbol     |        | Отражает значение параметра `symbol` |
| style\*  |        | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.                         |
| `style`             | `"bold purple"`                      | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Disables the `julia` module.                                              |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v1.4.0` | The version of `julia`               |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.kt` or a `.kts` file

### Опции

| Параметр            | По умолчанию                         | Описание                                                                      |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                                |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch`     |
| `detect_extensions` | `["kt", "kts"]`                      | Which extensions should trigger this module.                                  |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                                   |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                    |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `"bold blue"`                        | Стиль модуля.                                                                 |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v1.4.21` | The version of `kotlin`              |
| symbol     |           | Отражает значение параметра `symbol` |
| style\*  |           | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = "kotlinc"
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

По умолчанию этот модуль отключен. Чтобы включить его, установите `disabled` на `false` в файле конфигурации.

:::

### Опции

| Параметр          | По умолчанию                                         | Описание                                                              |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | Формат модуля.                                                        |
| `style`           | `"cyan bold"`                                        | Стиль модуля.                                                         |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Отключает модуль `kubernetes`.                                        |

### Переменные

| Переменная | Пример               | Описание                                 |
| ---------- | -------------------- | ---------------------------------------- |
| context    | `starship-cluster`   | The current kubernetes context           |
| namespace  | `starship-namespace` | If set, the current kubernetes namespace |
| symbol     |                      | Отражает значение параметра `symbol`     |
| style\*  |                      | Отражает значение параметра `style`      |

\*: Эта переменная может использоваться только в качестве части строки style

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

Модуль `line_break` разделяет командную строку на две строки.

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

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Опции

| Параметр            | По умолчанию                         | Описание                                                                   |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                             |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch`  |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | Which extensions should trigger this module.                               |
| `detect_files`      | `[".lua-version"]`                   | Which filenames should trigger this module.                                |
| `detect_folders`    | `["lua"]`                            | Which folders should trigger this module.                                  |
| `style`             | `"bold blue"`                        | Стиль модуля.                                                              |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v5.4.0` | The version of `lua`                 |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Использование памяти

Модуль `memory_usage` отображает текущую системную память и использование подкачки.

По умолчанию использование подкачки отображается, если общая сумма подкачки системы не равна нулю.

::: tip

По умолчанию этот модуль отключен. Чтобы включить его, установите `disabled` на `false` в файле конфигурации.

:::

### Опции

| Параметр    | По умолчанию                                    | Описание                                                           |
| ----------- | ----------------------------------------------- | ------------------------------------------------------------------ |
| `threshold` | `75`                                            | Скрывать использование памяти, если она не превышает этот процент. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | Формат модуля.                                                     |
| `symbol`    | `"🐏"`                                           | Символ, используемый перед отображением использования памяти.      |
| `style`     | `"bold dimmed white"`                           | Стиль модуля.                                                      |
| `disabled`  | `true`                                          | Отключает модуль `memory_usage`.                                   |

### Переменные

| Переменная       | Пример        | Описание                                                           |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Отражает значение параметра `symbol`                               |
| style\*        |               | Отражает значение параметра `style`                                |

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

## Ветка Mercurial

Модуль `hg_branch` показывает активную ветку репозитория в вашем текущем каталоге.

### Опции

| Параметр            | По умолчанию                     | Описание                                                                                 |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | Символ, используемый перед закладкой hg или именем ветки репозитория в текущем каталоге. |
| `style`             | `"bold purple"`                  | Стиль модуля.                                                                            |
| `format`            | `"on [$symbol$branch]($style) "` | Формат модуля.                                                                           |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                            |
| `truncation_symbol` | `"…"`                            | Символ, используемый для обозначения усечения названия ветки.                            |
| `disabled`          | `true`                           | Отключает модуль `hg_branch`.                                                            |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| branch     | `master` | The active mercurial branch          |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `nim.cfg`
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля                                                             |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.                     |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Which extensions should trigger this module.                              |
| `detect_files`      | `["nim.cfg"]`                        | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold yellow"`                      | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Disables the `nim` module.                                                |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v1.2.0` | The version of `nimc`                |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. Модуль будет показываться внутри среды nix-shell.

### Опции

| Параметр     | По умолчанию                                   | Описание                                              |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | Формат модуля.                                        |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | Стиль модуля.                                         |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Отключает модуль `nix_shell`.                         |

### Переменные

| Переменная | Пример  | Описание                             |
| ---------- | ------- | ------------------------------------ |
| state      | `pure`  | The state of the nix-shell           |
| name       | `lorri` | The name of the nix-shell            |
| symbol     |         | Отражает значение параметра `symbol` |
| style\*  |         | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `package.json`
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- Текущий каталог содержит каталог `node_modules`
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Опции

| Параметр            | По умолчанию                         | Описание                                                                                              |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                                                        |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                             |
| `symbol`            | `" "`                               | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Which extensions should trigger this module.                                                          |
| `detect_files`      | `["package.json", ".node-version"]`  | Which filenames should trigger this module.                                                           |
| `detect_folders`    | `["node_modules"]`                   | Which folders should trigger this module.                                                             |
| `style`             | `"bold green"`                       | Стиль модуля.                                                                                         |
| `disabled`          | `false`                              | Отключает модуль `nodejs`.                                                                            |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in package.json does not match the Node.js version. |

### Переменные

| Переменная | Пример     | Описание                             |
| ---------- | ---------- | ------------------------------------ |
| version    | `v13.12.0` | The version of `node`                |
| symbol     |            | Отражает значение параметра `symbol` |
| style\*  |            | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Опции

| Параметр                  | По умолчанию                                                               | Описание                                                                  |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | The format string for the module.                                         |
| `version_format`          | `v{raw}`                                                                   | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml.                   |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch.                   |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.                    |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Which extensions should trigger this module.                              |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Which filenames should trigger this module.                               |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Which folders should trigger this module.                                 |
| `style`                   | `"bold yellow"`                                                            | Стиль модуля.                                                             |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                              |

### Переменные

| Переменная       | Пример       | Описание                                                          |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Отражает значение параметра `symbol`                              |
| style\*        |              | Отражает значение параметра `style`                               |

\*: Эта переменная может использоваться только в качестве части строки style

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
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | Формат модуля.                                                 |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | Стиль модуля.                                                  |
| `disabled` | `false`                                             | Disables the `openstack` module.                               |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| cloud      | `corp` | The current OpenStack cloud          |
| project    | `dev`  | The current OpenStack project        |
| symbol     |        | Отражает значение параметра `symbol` |
| style\*  |        | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Версия пакета

Модуль `package` отображается, когда текущий каталог является репозиторием для пакета и показывает его текущую версию. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present
- [**julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present
- [**mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present
- [**helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present
- [**maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present
- [**meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present

> ⚠ Показана версия пакета, исходный код которого находится в текущем каталоге, а не в менеджере пакетов.

### Опции

| Параметр          | По умолчанию                      | Описание                                                  |
| ----------------- | --------------------------------- | --------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | Формат модуля.                                            |
| `symbol`          | `"📦 "`                            | Символ, используемый перед отображением версии пакета.    |
| `style`           | `"bold 208"`                      | Стиль модуля.                                             |
| `display_private` | `false`                           | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                           | Отключает модуль `package`.                               |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v1.0.0` | The version of your package          |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Опции

| Параметр            | По умолчанию                                                                                             | Описание                                                                  | **** |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- | ---- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                                         |      |
| `version_format`    | `v{raw}`                                                                                                 | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |      |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl                     |      |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Which extensions should trigger this module.                              |      |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Which filenames should trigger this module.                               |      |
| `detect_folders`    | `[]`                                                                                                     | Which folders should trigger this module.                                 |      |
| `style`             | `"bold 149"`                                                                                             | Стиль модуля.                                                             |      |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                               |      |

### Переменные

| Переменная | Пример    | Описание                             |
| ---------- | --------- | ------------------------------------ |
| version    | `v5.26.1` | The version of `perl`                |
| symbol     |           | Отражает значение параметра `symbol` |
| style\*  |           | Отражает значение параметра `style`  |

### Пример

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `composer.json`
- Текущий каталог содержит файл `.php-version`
- The current directory contains a `.php` extension

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | Символ, используемый перед отображением версии PHP.                       |
| `detect_extensions` | `["php"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `["composer.json", ".php-version"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"147 bold"`                         | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Отключает модуль `php`.                                                   |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v7.3.8` | The version of `php`                 |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `spago.dhall`
- The current directory contains a file with the `.purs` extension

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript.              |
| `detect_extensions` | `["purs"]`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `["spago.dhall"]`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold white"`                       | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Disables the `purescript` module.                                         |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `0.13.5` | The version of `purescript`          |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит файл `.python-version`
- Текущий каталог содержит файл `Pipfile`
- The current directory contains a `__init__.py` file
- Текущий каталог содержит файл `pyproject.toml`
- Текущий каталог содержит файл `requirements.txt`
- Текущий каталог содержит файл `setup.py`
- Текущий каталог содержит файл `tox.ini`
- Текущий каталог содержит файл с расширением `.py`.
- Виртуальная среда в данный момент активирована

### Опции

| Параметр             | По умолчанию                                                                                                 | Описание                                                                               |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Формат модуля.                                                                         |
| `version_format`     | `v{raw}`                                                                                                     | The version format. Available vars are `raw`, `major`, `minor`, & `patch`              |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | Стиль модуля.                                                                          |
| `pyenv_version_name` | `false`                                                                                                      | Использовать pyenv для получения версии Python                                         |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3, "python2"]`                                                                            | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `["py"]`                                                                                                     | Which extensions should trigger this module                                            |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Which filenames should trigger this module                                             |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                               |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                          |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Переменные

| Переменная   | Пример          | Описание                                   |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Отражает значение параметра `symbol`       |
| style        | `"yellow bold"` | Отражает значение параметра `style`        |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |

### Пример

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = "python3"
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a file with `.red` or `.reds` extension

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                           |
| `detect_extensions` | `["red"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"red bold"`                         | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Disables the `red` module.                                                |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v2.5.1` | The version of `red`                 |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `Gemfile`
- Текущий каталог содержит файл `.ruby-version`
- Текущий каталог содержит файл `.rb`

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                          |
| `detect_extensions` | `["rb"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold red"`                         | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Отключает модуль `ruby`.                                                  |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v2.5.1` | The version of `ruby`                |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `Cargo.toml`
- Текущий каталог содержит файл с расширением `.rs`

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                           |
| `detect_extensions` | `["rs"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Cargo.toml"]`                     | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold red"`                         | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Отключает модуль `rust`.                                                  |

### Переменные

| Переменная | Пример            | Описание                             |
| ---------- | ----------------- | ------------------------------------ |
| version    | `v1.43.0-nightly` | The version of `rustc`               |
| symbol     |                   | Отражает значение параметра `symbol` |
| style\*  |                   | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Опции

| Параметр            | По умолчанию                             | Описание                                                                  |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                                 | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Which extensions should trigger this module.                              |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".metals"]`                            | Which folders should trigger this modules.                                |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.                         |
| `style`             | `"red dimmed"`                           | Стиль модуля.                                                             |
| `disabled`          | `false`                                  | Disables the `scala` module.                                              |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `2.13.5` | The version of `scala`               |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

По умолчанию этот модуль отключен. Чтобы включить его, установите `disabled` на `false` в файле конфигурации.

:::

### Опции

| Параметр               | По умолчанию | Описание                                                     |
| ---------------------- | ------------ | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`        | A format string used to represent bash.                      |
| `fish_indicator`       | `fsh`        | A format string used to represent fish.                      |
| `zsh_indicator`        | `zsh`        | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`        | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`        | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`        | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.                      |
| `unknown_indicator`    |              | The default value to be displayed when the shell is unknown. |
| `format`               | `$indicator` | Формат модуля.                                               |
| `disabled`             | `true`       | Disables the `shell` module.                                 |

### Переменные

| Переменная | По умолчанию | Описание                                                   |
| ---------- | ------------ | ---------------------------------------------------------- |
| indicator  |              | Mirrors the value of `indicator` for currently used shell. |

### Примеры

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
disabled = false
```

## SHLVL

The `shlvl` module shows the current `SHLVL` ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Опции

| Параметр    | По умолчанию                 | Описание                                                      |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | Формат модуля.                                                |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | Стиль модуля.                                                 |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Переменные

| Переменная | Пример | Описание                             |
| ---------- | ------ | ------------------------------------ |
| shlvl      | `3`    | The current value of `SHLVL`         |
| symbol     |        | Отражает значение параметра `symbol` |
| style\*  |        | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### Опции

| Параметр   | По умолчанию                     | Описание                                         |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Формат модуля.                                   |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | Стиль модуля.                                    |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Переменные

| Переменная | Пример       | Описание                             |
| ---------- | ------------ | ------------------------------------ |
| env        | `centos.img` | The current Singularity image        |
| symbol     |              | Отражает значение параметра `symbol` |
| style\*  |              | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

По умолчанию этот модуль отключен. Чтобы включить его, установите `disabled` на `false` в файле конфигурации.

:::

::: warning This module is not supported on elvish shell. :::

### Опции

| Параметр                | По умолчанию                  | Описание                                             |
| ----------------------- | ----------------------------- | ---------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "` | The format of the module                             |
| `symbol`                | `"✖"`                         | The symbol displayed on program error                |
| `not_executable_symbol` | `"🚫"`                         | The symbol displayed when file isn't executable      |
| `not_found_symbol`      | `"🔍"`                         | The symbol displayed when the command can't be found |
| `sigint_symbol`         | `"🧱"`                         | The symbol displayed on SIGINT (Ctrl + c)            |
| `signal_symbol`         | `"⚡"`                         | The symbol displayed on any signal                   |
| `style`                 | `"bold red"`                  | Стиль модуля.                                        |
| `recognize_signal_code` | `true`                        | Enable signal mapping from exit code                 |
| `map_symbol`            | `false`                       | Enable symbols mapping from exit code                |
| `disabled`              | `true`                        | Disables the `status` module.                        |

### Переменные

| Переменная     | Пример  | Описание                                                             |
| -------------- | ------- | -------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                    |
| int            | `127`   | The exit code of the last command                                    |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                  |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled      |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found         |
| symbol         |         | Отражает значение параметра `symbol`                                 |
| style\*      |         | Отражает значение параметра `style`                                  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false

```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift                          |
| `detect_extensions` | `["swift"]`                          | Which extensions should trigger this module.                              |
| `detect_files`      | `["Package.swift"]`                  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold 202"`                         | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Disables the `swift` module.                                              |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v5.2.4` | The version of `swift`               |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.

::: tip

By default the Terraform version is not shown, since this is slow for current versions of Terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

:::

By default the module will be shown if any of the following conditions are met:

- Текущий каталог содержит папку `.terraform`
- Current directory contains a file with the `.tf` or `.hcl` extensions

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                                         |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                     |
| `detect_extensions` | `["tf", "hcl"]`                      | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".terraform"]`                     | Which folders should trigger this module.                                 |
| `style`             | `"bold 105"`                         | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Отключает модуль `terraform`.                                             |

### Переменные

| Переменная | Пример     | Описание                             |
| ---------- | ---------- | ------------------------------------ |
| version    | `v0.12.24` | The version of `terraform`           |
| workspace  | `default`  | The current Terraform workspace      |
| symbol     |            | Отражает значение параметра `symbol` |
| style\*  |            | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

#### With Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without Terraform version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Время

Модуль `time` показывает текущее **локальное** время. Значение конфигурации `format` используется пакетом [`chrono`](https://crates.io/crates/chrono) для контроля того, как отображается время. Ознакомьтесь с [документацией chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html), чтобы увидеть доступные параметры.

::: tip

По умолчанию этот модуль отключен. Чтобы включить его, установите `disabled` на `false` в файле конфигурации.

:::

### Опции

| Параметр          | По умолчанию            | Описание                                                                                                                                                      |
| ----------------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                                             |
| `use_12hr`        | `false`                 | Включить 12-часовое форматирование                                                                                                                            |
| `time_format`     | см. ниже                | [Строка формата chrono](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html), используемая для форматирования времени.                             |
| `style`           | `"bold yellow"`         | Стиль модуля времени                                                                                                                                          |
| `utc_time_offset` | `"local"`               | Устанавливает смещение UTC. Range from -24 &lt; x &lt; 24. Разрешает числам с плавающей точкой встраивать 30/45-минутное смещение временной зоны. |
| `disabled`        | `true`                  | Отключает модуль `time`.                                                                                                                                      |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                                                         |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Иначе по умолчанию используется `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Переменные

| Переменная | Пример     | Описание                            |
| ---------- | ---------- | ----------------------------------- |
| время      | `13:08:10` | The current time.                   |
| style\*  |            | Отражает значение параметра `style` |

\*: Эта переменная может использоваться только в качестве части строки style

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

## Имя пользователя

Модуль `username` показывает имя текущего пользователя. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий пользователь - root
- Текущий пользователь отличается от залогиненного
- Пользователь подключен к SSH-сессии
- Переменная `show_always` равна true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Опции

| Параметр      | По умолчанию            | Описание                                                |
| ------------- | ----------------------- | ------------------------------------------------------- |
| `style_root`  | `"bold red"`            | Стиль, используемый для пользователя root.              |
| `style_user`  | `"bold yellow"`         | Стиль, используемый для всех пользователей, кроме root. |
| `format`      | `"[$user]($style) in "` | Формат модуля.                                          |
| `show_always` | `false`                 | Всегда показывать модуль `username`.                    |
| `disabled`    | `false`                 | Отключает модуль `username`.                            |

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

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Vagrantfile` file

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.                       |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["Vagrantfile"]`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"cyan bold"`                        | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                            |

### Переменные

| Переменная | Пример           | Описание                             |
| ---------- | ---------------- | ------------------------------------ |
| version    | `Vagrant 2.2.10` | The version of `Vagrant`             |
| symbol     |                  | Отражает значение параметра `symbol` |
| style\*  |                  | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### Опции

| Параметр   | По умолчанию                     | Описание                                               |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   |                                  | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | Стиль модуля.                                          |
| `format`   | `"vcsh [$symbol$repo]($style) "` | Формат модуля.                                         |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Переменные

| Переменная | Пример                                      | Описание                             |
| ---------- | ------------------------------------------- | ------------------------------------ |
| repo       | `dotfiles` if in a VCSH repo named dotfiles | The active repository name           |
| symbol     |                                             | Отражает значение параметра `symbol` |
| style\*  | `black bold dimmed`                         | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

### Пример

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a `.zig` file

### Опции

| Параметр            | По умолчанию                         | Описание                                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                            |
| `version_format`    | `v{raw}`                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.                     |
| `style`             | `"bold yellow"`                      | Стиль модуля.                                                             |
| `disabled`          | `false`                              | Disables the `zig` module.                                                |
| `detect_extensions` | `["zig"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |

### Переменные

| Переменная | Пример   | Описание                             |
| ---------- | -------- | ------------------------------------ |
| version    | `v0.6.0` | The version of `zig`                 |
| symbol     |          | Отражает значение параметра `symbol` |
| style\*  |          | Отражает значение параметра `style`  |

\*: Эта переменная может использоваться только в качестве части строки style

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

| Параметр      | По умолчанию                    | Описание                                                                                                                   |
| ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                         |
| `описание`    | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                  | Стиль модуля.                                                                                                              |
| `format`      | `"[$symbol($output )]($style)"` | Формат модуля.                                                                                                             |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                             |

### Переменные

| Переменная | Описание                               |
| ---------- | -------------------------------------- |
| output     | The output of shell command in `shell` |
| symbol     | Отражает значение параметра `symbol`   |
| style\*  | Отражает значение параметра `style`    |

\*: Эта переменная может использоваться только в качестве части строки style

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
