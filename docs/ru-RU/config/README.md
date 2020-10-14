# Конфигурация

Чтобы начать конфигурацию Starship, создайте следующий файл: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Вся конфигурация Starship выполняется в этом файле [TOML](https://github.com/toml-lang/toml):

```toml
# Don't print a new line at the start of the prompt
add_newline = false

# Replace the "❯" symbol in the prompt with "➜"
[character]                            # The name of the module we are configuring is "character"
success_symbol = "[➜](bold green)"     # The "success_symbol" segment is being set to "➜" with the color "bold green"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

Вы можете изменить расположение файла `starship.toml` переменной окружения `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/.starship
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### Logging

By default starship logs warnings and errors into a file named `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, where the session key is corresponding to a instance of your terminal. This, however can be changed using the `STARSHIP_CACHE` environment variable:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```ps1
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### Терминология

**Модуль**: Компонент строки, дающий информацию на основе контекстной информации вашей ОС. Например, модуль "nodejs" показывает установленную версию NodeJS на вашем компьютере, если вы находитесь в директории проекта NodeJS.

**Variable**: Smaller sub-components that contains information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of NodeJS.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Format Strings

Format strings are the format that a module prints all its variables with. Most modules have an entry called `format` that configures the display format of the module. You can use texts, variables and text groups in a format string.

#### Переменная

A variable contains a `$` symbol followed by the name of the variable. The name of a variable only contains letters, numbers and `_`.

For example:

- `$version` is a format string with a variable named `version`.
- `$git_branch$git_commit` is a format string with two variables named `git_branch` and `git_commit`.
- `$git_branch $git_commit` has the two variables separated with a space.

#### Text Group

A text group is made up of two different parts.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings). You can add texts, variables, or even nested text groups in it.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). This can be used style the first part.

For example:

- `[on](red bold)` will print a string `on` with bold text colored red.
- `[⬢ $version](bold green)` will print a symbol `⬢` followed by the content of variable `version`, with bold text colored green.
- `[a [b](red) c](green)` will print `a b c` with `b` red, and `a` and `c` green.

#### Строки стиля

В Starship, большинство модулей позволяют настроить стили отображения. Это делается записью (обычно называется `style`), которая представляет собой строку, определяющую конфигурацию. Ниже приведены несколько примеров стилей строк, а также, их действия. Подробнее о полном синтаксисе можно прочитать в [расширенном разделе конфигурации](/advanced-config/).

- `"fg:green bg:blue"` устанавливает зеленый текст на синем фоне
- `"bg:blue fg:bright-green"` устанавливает ярко-зеленый текст на синем фоне
- `"bold fg:27"` устанавливает жирный текст с [цветом ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` устанавливает подчёркиваемый текст цвета сожженного апельсина
- `"bold italic fg:purple"` устанавливает жирный фиолетовый текст
- `""` выключает все стили

Обратите внимание на то, что, вид стиля зависит от вашего эмулятора терминала. Например, некоторые эмуляторы терминала будут использовать яркие цвета вместо жирного текста, и некоторые цветовые темы используют одинаковые значение для обычных и ярких цветов. Также, чтобы получить курсивный текст, ваш терминал должен поддерживать курсив.

#### Conditional Format Strings

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

For example:

- `(@$region)` will show nothing if the variable `region` is `None`, otherwise `@` followed by the value of region.
- `(some text)` will always show nothing since there are no variables wrapped in the braces.
- When `$all` is a shortcut for `\[$a$b\]`, `($all)` will show nothing only if `$a` and `$b` are both `None`. This works the same as `(\[$a$b\] )`.

#### Escapable characters

The following symbols have special usage in a format string. If you want to print the following symbols, you have to escape them with a backslash (`\`).

- \$
- \\
- [
- ]
- (
- )

Note that `toml` has [its own escape syntax](https://github.com/toml-lang/toml#user-content-string). It is recommended to use a literal string (`''`) in your config. If you want to use a basic string (`""`), pay attention to escape the backslash `\`.

For example, when you want to print a `$` symbol on a new line, the following configs for `format` are equivalent:

```toml
# with basic string
format = "\n\\$"

# with multiline basic string
format = """

\\$"""

# with literal string
format = '''

\$'''
```

## Командная строка

Ниже находится список опций, применяющихся для всей командной строки.

### Опции

| Option         | По умолчанию                     | Описание                                                 |
| -------------- | -------------------------------- | -------------------------------------------------------- |
| `format`       | [ссылка](#default-prompt-format) | Configure the format of the prompt.                      |
| `scan_timeout` | `30`                             | Тайм-аут запуска сканирования файлов (в миллисекундах).  |
| `add_newline`  | `true`                           | Добавление пустой строки перед началом командной строки. |

### Пример

```toml
# ~/.config/starship.toml

# Use custom format
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10

# Disable the newline at the start of the prompt
add_newline = false
```

### Default Prompt Format

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
$env_var\
$crystal\
$cmd_duration\
$custom\
$line_break\
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

| Option           | По умолчанию                                     | Описание                                                       |
| ---------------- | ------------------------------------------------ | -------------------------------------------------------------- |
| `format`         | `'on [$symbol$profile(\($region\))]($style) '` | The format for the module.                                     |
| `symbol`         | `"☁️ "`                                          | Символ перед отображением текущего профиля AWS.                |
| `region_aliases` |                                                  | Таблица региона псевдонимов, отображаемая вместе с именем AWS. |
| `style`          | `"bold yellow"`                                  | Стиль модуля.                                                  |
| `disabled`       | `false`                                          | Отключение модуля `AWS`.                                       |

### Variables

| Переменная | Пример           | Описание                             |
| ---------- | ---------------- | ------------------------------------ |
| region     | `ap-northeast-1` | The current AWS region               |
| profile    | `astronauts`     | The current AWS profile              |
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

| Option               | По умолчанию                      | Описание                                        |
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

| Option      | Описание                                                 |
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

| Option           | По умолчанию        | Описание                                                                         |
| ---------------- | ------------------- | -------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | The format string used before the text input.                                    |
| `success_symbol` | `"[❯](bold green)"` | The format string used before the text input if the previous command succeeded.  |
| `error_symbol`   | `"[❯](bold red)"`   | The format string used before the text input if the previous command failed.     |
| `vicmd_symbol`   | `"[❮](bold green)"` | The format string used before the text input if the shell is in vim normal mode. |
| `disabled`       | `false`             | Отключает модуль `character`.                                                    |

### Variables

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

The `cmake` module shows the currently installed version of CMake if:

- The current directory contains a `CMakeLists.txt` file

### Опции

| Option     | По умолчанию                       | Описание                                     |
| ---------- | ---------------------------------- | -------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                   |
| `symbol`   | `"🛆 "`                             | The symbol used before the version of cmake. |
| `style`    | `"bold blue"`                      | Стиль модуля.                                |
| `disabled` | `false`                            | Disables the `cmake` module.                 |

### Variables

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

| Option              | По умолчанию                  | Описание                                                             |
| ------------------- | ----------------------------- | -------------------------------------------------------------------- |
| `min_time`          | `2_000`                       | Кратчайшая продолжительность для показа времени (в миллисекундах).   |
| `show_milliseconds` | `false`                       | Показывать миллисекунды в дополнение к секундам в продолжительности. |
| `format`            | `"took [$duration]($style) "` | The format for the module.                                           |
| `style`             | `"bold yellow"`               | Стиль модуля.                                                        |
| `disabled`          | `false`                       | Отключает модуль `cmd_duration`.                                     |

### Variables

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

Модуль `conda` показывает текущее окружение conda, если `$CONDA_DEFAULT_ENV` присвоено значение.

::: tip

Это не подавляет модификатор командной строки самой conda. Возможно, вы захотите запустить `conda config --set changeps1 False`.

:::

### Опции

| Option              | По умолчанию                       | Описание                                                                                                                                                                                                     |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `1`                                | Количество каталогов, в которых путь к окружению должен быть усечен, если окружение было создано через `conda create -p [path]`. `0` означает без усечения. Также смотрите модуль [`directory`](#directory). |
| `symbol`            | `"🅒 "`                             | Символ перед названием окружения.                                                                                                                                                                            |
| `style`             | `"bold green"`                     | Стиль модуля.                                                                                                                                                                                                |
| `format`            | `"[$symbol$environment]($style) "` | The format for the module.                                                                                                                                                                                   |
| `ignore_base`       | `true`                             | Ignores `base` environment when activated.                                                                                                                                                                   |
| `disabled`          | `false`                            | Отключает модуль `conda`.                                                                                                                                                                                    |

### Variables

| Переменная  | Пример       | Описание                             |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current conda environment        |
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

Модуль `crystal` показывает установленную версию Crystal. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `shard.yml`
- Текущий каталог содержит файл `.cr`

### Опции

| Option     | По умолчанию                       | Описание                                                |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `symbol`   | `"🔮 "`                             | Символ, используемый перед отображением версии crystal. |
| `style`    | `"bold red"`                       | Стиль модуля.                                           |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                              |
| `disabled` | `false`                            | Отключает модуль `crystal`.                             |

### Variables

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

The `dart` module shows the currently installed version of Dart. Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml` or `pubspec.lock` file

### Опции

| Option     | По умолчанию                       | Описание                                        |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🎯 "`                             | A format string representing the symbol of Dart |
| `style`    | `"bold blue"`                      | Стиль модуля.                                   |
| `disabled` | `false`                            | Disables the `dart` module.                     |

### Variables

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

Модуль `directory` показывает путь к вашей текущей директории, усеченной до трех родительских папок. Ваш каталог также будет отсечен до корня git репозитория, в котором вы находитесь.

При использовании стиля оболочки fish, вместо скрытия усеченного каталога, вы увидите укороченное имя каталога, зависимое от числа символов вы установите для этой опции.

Например, возьмем `~/Dev/Nix/nixpkgs/pkgs` где `nixpkgs` является корневым репозиторием, и в опции установлено `1`. Вы увидите `~/D/N/nixpkgs/pkgs`, а до этого было бы `nixpkgs/pkgs`.

### Опции

| Option              | По умолчанию                                       | Описание                                                                     |
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
<summary>Этот модуль имеет несколько расширенных опций конфигурации, которые контролируют отображение каталога.</summary>

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

`fish_style_pwd_dir_length` взаимодействует со стандартными параметрами усечения, которые могут сначала показаться странными: если он не равен нулю, элементы пути, который обычно усекается, вместо этого отображаются с указанным количеством символов. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

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

Модуль `docker_context` показывает текущий активный [контекст Docker](https://docs.docker.com/engine/context/working-with-contexts/), если он не установлен как `default`.

### Опции

| Option            | По умолчанию                       | Описание                                                                                |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | The format for the module.                                                              |
| `symbol`          | `"🐳 "`                             | The symbol used before displaying the Docker context.                                   |
| `style`           | `"blue bold"`                      | Стиль модуля.                                                                           |
| `only_with_files` | `false`                            | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `disabled`        | `true`                             | Disables the `docker_context` module.                                                   |

### Variables

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

Модуль `dotnet` показывает соответствующую версию .NET Core SDK для текущего каталога. Если SDK был закреплен в текущей директории, будет показана закрепленная версия. В противном случае модуль отображает последнюю установленную версию SDK.

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

Внутренне этот модуль использует свой собственный механизм определения версий. Обычно он в два раза быстрее, чем `dotnet --version`, но он может показывать неправильную версию, если ваш .NET проект имеет необычный формат каталога. Если точность важнее, чем скорость, вы можете отключить механизм опцией `heuristic = false` в настройках модуля.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Опции

| Option      | По умолчанию                             | Описание                                                          |
| ----------- | ---------------------------------------- | ----------------------------------------------------------------- |
| `format`    | `"v[$symbol$version( 🎯 $tfm)]($style) "` | The format for the module.                                        |
| `symbol`    | `"•NET "`                                | Символ перед отображением текущей версии dotnet.                  |
| `heuristic` | `true`                                   | Использовать быстрое определение версии, для сохранения скорости. |
| `style`     | `"bold blue"`                            | Стиль модуля.                                                     |
| `disabled`  | `false`                                  | Отключает модуль `dotnet`.                                        |

### Variables

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

Модуль `elixir` показывает установленную версию Elixir и Erlang/OTP. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `mix.exs`.

### Опции

| Option     | По умолчанию                                              | Описание                                                      |
| ---------- | --------------------------------------------------------- | ------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                    | Символ, используемый перед отображением версии Elixir/Erlang. |
| `style`    | `"bold purple"`                                           | Стиль модуля.                                                 |
| `format`   | `'via [$symbol$version \(OTP $otp_version\)]($style) '` | The format for the module elixir.                             |
| `disabled` | `false`                                                   | Отключает модуль `elixir`.                                    |

### Variables

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

Модуль `elm` показывает установленную версию Elm. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `elm.json`
- Текущий каталог содержит файл `elm-package.json`
- Текущий каталог содержит файл `.elm-version`
- Текущий каталог содержит папку `elm-stuff`
- Текущий каталог содержит файлы `*.elm`

### Опции

| Option     | По умолчанию                       | Описание                                        |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🌳 "`                             | A format string representing the symbol of Elm. |
| `style`    | `"cyan bold"`                      | Стиль модуля.                                   |
| `disabled` | `false`                            | Отключает модуль `elm`.                         |

### Variables

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

Модуль `env_var` отображает текущее значение выбранной переменной окружения. Модуль будет показан только в том случае, если любое из следующих условий соблюдено:

- Опция `variable` соответствует существующей переменной среды
- Опция `variable` не определена, но определена опция `default`

### Опции

| Option     | По умолчанию                   | Описание                                                         |
| ---------- | ------------------------------ | ---------------------------------------------------------------- |
| `symbol`   |                                | Символ, используемый перед отображением значения переменной.     |
| `variable` |                                | Отображаемая переменная окружения.                               |
| `default`  |                                | Значение отображаемое, когда выбранная переменная не определена. |
| `format`   | `"with [$env_value]($style) "` | The format for the module.                                       |
| `disabled` | `false`                        | Отключает модуль `env_var`.                                      |

### Variables

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

The `erlang` module shows the currently installed version of Erlang/OTP. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `rebar.config`.
- Текущий каталог содержит файл `erlang.mk`.

### Опции

| Option     | По умолчанию                       | Описание                                                 |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`                             | The symbol used before displaying the version of erlang. |
| `style`    | `"bold red"`                       | Стиль модуля.                                            |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                               |
| `disabled` | `false`                            | Disables the `erlang` module.                            |

### Variables

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

| Option           | По умолчанию                                     | Описание                                                        |
| ---------------- | ------------------------------------------------ | --------------------------------------------------------------- |
| `format`         | `'on [$symbol$account(\($region\))]($style) '` | The format for the module.                                      |
| `symbol`         | `"☁️ "`                                          | The symbol used before displaying the current GCP profile.      |
| `region_aliases` |                                                  | Table of region aliases to display in addition to the GCP name. |
| `style`          | `"bold blue"`                                    | Стиль модуля.                                                   |
| `disabled`       | `false`                                          | Disables the `gcloud` module.                                   |

### Variables

| Переменная | Пример            | Описание                                                           |
| ---------- | ----------------- | ------------------------------------------------------------------ |
| region     | `us-central1`     | The current GCP region                                             |
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

Модуль `git_branch` показывает активную ветку репозитория в вашем текущей директории.

### Опции

| Option              | По умолчанию                     | Описание                                                                                      |
| ------------------- | -------------------------------- | --------------------------------------------------------------------------------------------- |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module. Use `"$branch"` to refer to the current branch name.               |
| `symbol`            | `" "`                           | A format string representing the symbol of git branch.                                        |
| `style`             | `"bold purple"`                  | Стиль модуля.                                                                                 |
| `truncation_length` | `2^63 - 1`                       | Отрезает ветку git до X графемов.                                                             |
| `truncation_symbol` | `"…"`                            | Символ, используемый для обозначения усечения названия ветки. You can use `""` for no symbol. |
| `disabled`          | `false`                          | Отключает модуль `git_branch`.                                                                |

### Variables

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

Модуль `git_commit` показывает хэш текущего коммита репозитория в вашем текущем каталоге.

### Опции

| Option               | По умолчанию               | Описание                                                                |
| -------------------- | -------------------------- | ----------------------------------------------------------------------- |
| `commit_hash_length` | `7`                        | Длина отображаемого хэша коммита git.                                   |
| `format`             | `'[\($hash\)]($style) '` | The format for the module.                                              |
| `style`              | `"bold green"`             | Стиль модуля.                                                           |
| `only_detached`      | `true`                     | Показывать хэш коммита git, только находясь в состоянии отделённой HEAD |
| `disabled`           | `false`                    | Отключает модуль `git_commit`.                                          |

### Variables

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
```

## Состояние Git

Модуль `git_state` будет отображаться в директориях, являющимися частью репозитория git, и там, где выполняется операция, такие как: _REBASING_, _BISECTING_, и т. д. Если есть информация о прогрессе (например, REBASING 3/10), эта информация также будет показана.

### Опции

| Option         | По умолчанию                                                    | Описание                                                                                |
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

### Variables

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

Модуль `git_status` отображает символы, представляющие состояние репозитория в вашей текущей директории.

### Опции

| Option       | По умолчанию                                    | Описание                            |
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

### Variables

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

Показывать счетчик впереди/позади для отслеживаемой ветки

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

Модуль `golang` показывает установленную версию Golang. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `go.mod`
- Текущий каталог содержит файл `go.sum`
- Текущий каталог содержит файл `glide.yaml`
- Текущий каталог содержит файл `Gopkg.yml`
- Текущий каталог содержит файл `Gopkg.lock`
- The current directory contains a `.go-version` file
- Текущий каталог содержит папку `Godeps`
- Текущий каталог содержит файл с расширением `.go`

### Опции

| Option     | По умолчанию                       | Описание                                       |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                     |
| `symbol`   | `"🐹 "`                             | A format string representing the symbol of Go. |
| `style`    | `"bold cyan"`                      | Стиль модуля.                                  |
| `disabled` | `false`                            | Отключает модуль `golang`.                     |

### Variables

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

The `helm` module shows the currently installed version of Helm. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `helmfile.yaml`
- The current directory contains a `Chart.yaml` file

### Опции

| Option     | По умолчанию                       | Описание                                         |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"⎈ "`                             | A format string representing the symbol of Helm. |
| `style`    | `"bold white"`                     | Стиль модуля.                                    |
| `disabled` | `false`                            | Disables the `helm` module.                      |

### Variables

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

Модуль `hostname` отображает имя системы (хоста).

### Опции

| Option     | По умолчанию                | Описание                                                                                                                                   |
| ---------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                      | Показывать имя хоста только при подключении к SSH-сессии.                                                                                  |
| `trim_at`  | `"."`                       | Символы, по которую имя хоста будет сокращено после первого совпадения. `"."` остановится после первой точки. `""` отключит любое усечение |
| `format`   | `"[$hostname]($style) in "` | The format for the module.                                                                                                                 |
| `style`    | `"bold dimmed green"`       | Стиль модуля.                                                                                                                              |
| `disabled` | `false`                     | Отключает модуль `hostname`.                                                                                                               |

### Variables

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

Модуль `java` показывает установленную версию Java. Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### Опции

| Option     | По умолчанию                           | Описание                                        |
| ---------- | -------------------------------------- | ----------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | The format for the module.                      |
| `symbol`   | `"☕ "`                                 | A format string representing the symbol of Java |
| `style`    | `"red dimmed"`                         | Стиль модуля.                                   |
| `disabled` | `false`                                | Отключает модуль `java`.                        |

### Variables

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

Модуль `jobs` отображает текущее количество запущенных работ. Модуль будет показан только если работы выполняются в фоне. Модуль покажет количество запущенных задач при наличии более чем 1 задачи, или больше, чем значение конфигурации `threshold`, если оно существует.

### Опции

| Option      | По умолчанию                  | Описание                                         |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | Показывать количество задач, если превышено.     |
| `format`    | `"[$symbol$number]($style) "` | The format for the module.                       |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | Стиль модуля.                                    |
| `disabled`  | `false`                       | Отключает модуль `jobs`.                         |

### Variables

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

The `julia` module shows the currently installed version of Julia. Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Опции

| Option     | По умолчанию                       | Описание                                          |
| ---------- | ---------------------------------- | ------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                        |
| `symbol`   | `"ஃ "`                             | A format string representing the symbol of Julia. |
| `style`    | `"bold purple"`                    | Стиль модуля.                                     |
| `disabled` | `false`                            | Disables the `julia` module.                      |

### Variables

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

По умолчанию этот модуль отключен. Чтобы включить его, установите `disabled` на `false` в файле конфигурации.

:::

### Опции

| Option            | По умолчанию                                         | Описание                                                              |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | The format for the module.                                            |
| `style`           | `"cyan bold"`                                        | Стиль модуля.                                                         |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Отключает модуль `kubernetes`.                                        |

### Variables

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

Модуль `line_break` разделяет командную строку на две строки.

### Опции

| Option     | По умолчанию | Описание                                                                 |
| ---------- | ------------ | ------------------------------------------------------------------------ |
| `disabled` | `false`      | Отключает модуль `line_break`, отображая командную строку в одну строку. |

### Пример

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Использование памяти

Модуль `memory_usage` отображает текущую системную память и использование подкачки.

По умолчанию использование подкачки отображается, если общая сумма подкачки системы не равна нулю.

::: tip

По умолчанию этот модуль отключен. Чтобы включить его, установите `disabled` на `false` в файле конфигурации.

:::

### Опции

| Option      | По умолчанию                                  | Описание                                                           |
| ----------- | --------------------------------------------- | ------------------------------------------------------------------ |
| `threshold` | `75`                                          | Скрывать использование памяти, если она не превышает этот процент. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | The format for the module.                                         |
| `symbol`    | `"🐏"`                                         | Символ, используемый перед отображением использования памяти.      |
| `style`     | `"bold dimmed white"`                         | Стиль модуля.                                                      |
| `disabled`  | `true`                                        | Отключает модуль `memory_usage`.                                   |

### Variables

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
show_percentage = true
show_swap = true
threshold = -1
symbol = " "
separator = "/"
style = "bold dimmed green"
```

## Ветка Mercurial

Модуль `hg_branch` показывает активную ветку репозитория в вашем текущем каталоге.

### Опции

| Option              | По умолчанию                     | Описание                                                                                 |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | Символ, используемый перед закладкой hg или именем ветки репозитория в текущем каталоге. |
| `style`             | `"bold purple"`                  | Стиль модуля.                                                                            |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.                                                               |
| `truncation_length` | `2^63 - 1`                       | Обрезает имя ветки hg до X графемов                                                      |
| `truncation_symbol` | `"…"`                            | Символ, используемый для обозначения усечения названия ветки.                            |
| `disabled`          | `true`                           | Отключает модуль `hg_branch`.                                                            |

### Variables

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

The `nim` module shows the currently installed version of Nim. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `nim.cfg`
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Опции

| Option     | По умолчанию                       | Описание                                              |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module                             |
| `symbol`   | `"👑 "`                             | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"`                    | Стиль модуля.                                         |
| `disabled` | `false`                            | Disables the `nim` module.                            |

### Variables

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

Модуль `nix_shell` показывает окружение nix-shell. Модуль будет показываться внутри среды nix-shell.

### Опции

| Option       | По умолчанию                                   | Описание                                              |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | The format for the module.                            |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | Стиль модуля.                                         |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Отключает модуль `nix_shell`.                         |

### Variables

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

Модуль `nodejs` показывает установленную версию NodeJS. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `package.json`
- The current directory contains a `.node-version` file
- Текущий каталог содержит каталог `node_modules`
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Опции

| Option     | По умолчанию                       | Описание                                           |
| ---------- | ---------------------------------- | -------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                         |
| `symbol`   | `"⬢ "`                             | A format string representing the symbol of NodeJS. |
| `style`    | `"bold green"`                     | Стиль модуля.                                      |
| `disabled` | `false`                            | Отключает модуль `nodejs`.                         |

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

## Версия пакета

Модуль `package` отображается, когда текущий каталог является репозиторием для пакета и показывает его текущую версию. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- **npm** – Версия пакета `npm` из файла `package.json` в текущем каталоге
- **cargo** – Версия пакета `cargo` из файла `Cargo.toml` в текущем каталоге
- **poetry** – Версия пакета `poetry` из файла `pyproject.toml` в текущем каталоге
- **composer** – Версия пакета `composer` из `composer.json` в текущем каталоге
- **gradle** – Версия пакета `gradle` извлечена из `build.gradle`
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present
- **helm** - The `helm` chart version is extracted from the `Chart.yaml` present
- **maven** - The `maven` package version is extracted from the `pom.xml` present

> ⚠ Показана версия пакета, исходный код которого находится в текущем каталоге, а не в менеджере пакетов.

### Опции

| Option            | По умолчанию                       | Описание                                                  |
| ----------------- | ---------------------------------- | --------------------------------------------------------- |
| `format`          | `"via [$symbol$version]($style) "` | The format for the module.                                |
| `symbol`          | `"📦 "`                             | Символ, используемый перед отображением версии пакета.    |
| `style`           | `"bold 208"`                       | Стиль модуля.                                             |
| `display_private` | `false`                            | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                            | Отключает модуль `package`.                               |

### Variables

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

## OCaml

The `ocaml` module shows the currently installed version of OCaml. Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Опции

| Option     | По умолчанию                       | Описание                                                |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                       |
| `symbol`   | `"🐫 "`                             | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"`                    | Стиль модуля.                                           |
| `disabled` | `false`                            | Disables the `ocaml` module.                            |

### Variables

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

## Perl

The `perl` module shows the currently installed version of Perl. Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Опции

| Option     | По умолчанию                       | Описание                                              |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                     |
| `symbol`   | `"🐪 "`                             | The symbol used before displaying the version of Perl |
| `style`    | `"bold 149"`                       | Стиль модуля.                                         |
| `disabled` | `false`                            | Disables the `perl` module.                           |

### Variables

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

Модуль `php` показывает установленную версию PHP. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `composer.json`
- Текущий каталог содержит файл `.php-version`
- Текущий каталог содержит файл `.php`

### Опции

| Option     | По умолчанию                       | Описание                                            |
| ---------- | ---------------------------------- | --------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                          |
| `symbol`   | `"🐘 "`                             | Символ, используемый перед отображением версии PHP. |
| `style`    | `"147 bold"`                       | Стиль модуля.                                       |
| `disabled` | `false`                            | Отключает модуль `php`.                             |

### Variables

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

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `.python-version`
- Текущий каталог содержит файл `requirements.txt`
- Текущий каталог содержит файл `pyproject.toml`
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- Текущий каталог содержит файл `Pipfile`
- Текущий каталог содержит файл `tox.ini`
- Текущий каталог содержит файл `setup.py`
- The current directory contains a `__init__.py` file
- Виртуальная среда в данный момент активирована

### Опции

| Option               | По умолчанию                                                              | Описание                                                                      |
| -------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}${version}( \($virtualenv\))]($style) '` | The format for the module.                                                    |
| `symbol`             | `"🐍 "`                                                                    | A format string representing the symbol of Python                             |
| `style`              | `"yellow bold"`                                                           | Стиль модуля.                                                                 |
| `pyenv_version_name` | `false`                                                                   | Использовать pyenv для получения версии Python                                |
| `pyenv_prefix`       | `pyenv`                                                                   | Prefix before pyenv version display, only used if pyenv is used               |
| `scan_for_pyfiles`   | `true`                                                                    | If false, Python files in the current directory will not show this module.    |
| `python_binary`      | `python`                                                                  | Configures the python binary that Starship executes when getting the version. |
| `disabled`           | `false`                                                                   | Disables the `python` module.                                                 |

### Variables

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

Модуль `ruby` показывает установленную версию Ruby. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `Gemfile`
- Текущий каталог содержит файл `.ruby-version`
- Текущий каталог содержит файл `.rb`

### Опции

| Option     | По умолчанию                       | Описание                                         |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"💎 "`                             | A format string representing the symbol of Ruby. |
| `style`    | `"bold red"`                       | Стиль модуля.                                    |
| `disabled` | `false`                            | Отключает модуль `ruby`.                         |

### Variables

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

Модуль `rust` показывает установленную версию Rust. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `Cargo.toml`
- Текущий каталог содержит файл с расширением `.rs`

### Опции

| Option     | По умолчанию                       | Описание                                        |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🦀 "`                             | A format string representing the symbol of Rust |
| `style`    | `"bold red"`                       | Стиль модуля.                                   |
| `disabled` | `false`                            | Отключает модуль `rust`.                        |

### Variables

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

| Option      | По умолчанию                 | Описание                                |
| ----------- | ---------------------------- | --------------------------------------- |
| `threshold` | `2`                          | Display threshold.                      |
| `format`    | `"[$symbol$shlvl]($style) "` | The format for the module.              |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the SHLVL. |
| `style`     | `"bold yellow"`              | Стиль модуля.                           |
| `disabled`  | `true`                       | Disables the `shlvl` module.            |

### Variables

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

Модуль `singularity` показывает текущий образ singularity, если внутри контейнера и `$SINGULARITY_NAME` установлена.

### Опции

| Option     | По умолчанию                     | Описание                                         |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | The format for the module.                       |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | Стиль модуля.                                    |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

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

## Swift

The `swift` module shows the currently installed version of Swift. Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Опции

| Option     | По умолчанию                       | Описание                                         |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"🐦 "`                             | A format string representing the symbol of Swift |
| `style`    | `"bold 202"`                       | Стиль модуля.                                    |
| `disabled` | `false`                            | Disables the `swift` module.                     |

### Variables

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

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

По умолчанию этот модуль отключен. Чтобы включить его, установите `disabled` на `false` в файле конфигурации. :::

### Опции

| Option     | По умолчанию               | Описание                                               |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `[$symbol$status]($style)` | The format of the module                               |
| `symbol`   | `"✖"`                      | A format string representing the symbol for the status |
| `style`    | `"bold red"`               | Стиль модуля.                                          |
| `disabled` | `true`                     | Disables the `status` module.                          |

### Variables

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

## Terraform

Модуль `terraform` показывает выбранную рабочую область и версию terraform. По умолчанию, версия terraform не показана, так как это медленно на текущих версиях terraform, при использовании большого количества плагинов. If you still want to enable it, [follow the example shown below](#with-version). Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит папку `.terraform`
- Текущий каталог содержит файл с расширением `.tf`

### Опции

| Option     | По умолчанию                         | Описание                                              |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`   | `"💠 "`                               | A format string shown before the terraform workspace. |
| `style`    | `"bold 105"`                         | Стиль модуля.                                         |
| `disabled` | `false`                              | Отключает модуль `terraform`.                         |

### Variables

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

Модуль `time` показывает текущее **локальное** время. Значение конфигурации `format` используется пакетом [`chrono`](https://crates.io/crates/chrono) для контроля того, как отображается время. Ознакомьтесь с [документацией chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html), чтобы увидеть доступные параметры.

::: tip

По умолчанию этот модуль отключен. Чтобы включить его, установите `disabled` на `false` в файле конфигурации.

:::

### Опции

| Option            | По умолчанию            | Описание                                                                                                                                                      |
| ----------------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                                             |
| `use_12hr`        | `false`                 | Включить 12-часовое форматирование                                                                                                                            |
| `time_format`     | см. ниже                | [Строка формата chrono](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html), используемая для форматирования времени.                             |
| `style`           | `"bold yellow"`         | Стиль модуля времени                                                                                                                                          |
| `utc_time_offset` | `"local"`               | Устанавливает смещение UTC. Range from -24 &lt; x &lt; 24. Разрешает числам с плавающей точкой встраивать 30/45-минутное смещение временной зоны. |
| `disabled`        | `true`                  | Отключает модуль `time`.                                                                                                                                      |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                                                         |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Иначе по умолчанию используется `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Переменная | Пример     | Описание                            |
| ---------- | ---------- | ----------------------------------- |
| время      | `13:08:10` | The current time.                   |
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

## Имя пользователя

Модуль `username` показывает имя текущего пользователя. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий пользователь - root
- Текущий пользователь отличается от залогиненного
- Пользователь подключен к SSH-сессии
- Переменная `show_always` равна true

### Опции

| Option        | По умолчанию            | Описание                                                |
| ------------- | ----------------------- | ------------------------------------------------------- |
| `style_root`  | `"bold red"`            | Стиль, используемый для пользователя root.              |
| `style_user`  | `"bold yellow"`         | Стиль, используемый для всех пользователей, кроме root. |
| `format`      | `"[$user]($style) in "` | The format for the module.                              |
| `show_always` | `false`                 | Всегда показывать модуль `username`.                    |
| `disabled`    | `false`                 | Отключает модуль `username`.                            |

### Variables

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

The `zig` module shows the currently installed version of Zig. Модуль будет показан, если любое из следующих условий соблюдено:

- The current directory contains a `.zig` file

### Опции

| Option     | По умолчанию                       | Описание                                              |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`                             | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"`                    | Стиль модуля.                                         |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `disabled` | `false`                            | Disables the `zig` module.                            |

### Variables

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

| Option        | По умолчанию                  | Описание                                                                                                                   |
| ------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                               | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
| `when`        |                               | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                               | [See below](#custom-command-shell)                                                                                         |
| `описание`    | `"<custom module>"`     | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                          | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                          | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                          | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                          | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                | Стиль модуля.                                                                                                              |
| `format`      | `"[$symbol$output]($style) "` | The format for the module.                                                                                                 |
| `disabled`    | `false`                       | Disables this `custom` module.                                                                                             |

### Variables

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

## PureScript

The `purescript` module shows the currently installed version of PureScript version. Модуль будет показан, если любое из следующих условий соблюдено:

- Текущий каталог содержит файл `spago.dhall`
- The current directory contains a \*.purs files

### Опции

| Option     | По умолчанию                       | Описание                                                     |
| ---------- | ---------------------------------- | ------------------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                                   |
| `symbol`   | `"<=> "`                     | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"`                     | Стиль модуля.                                                |
| `disabled` | `false`                            | Disables the `purescript` module.                            |

### Variables

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
