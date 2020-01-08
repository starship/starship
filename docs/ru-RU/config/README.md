# Конфигурация

::: tip

🔥 Конфигурация все еще находится в стадии разработки. Много новых опций будут доступны в будущих версиях.

:::

Чтобы начать конфигурацию Starship, создайте следующий файл: `~/.config/starship.toml`.

```shell
$ mkdir -p ~/.config && touch ~/.config/starship.toml
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
```shell
export STARSHIP_CONFIG=~/.starship
```

### Терминология

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of NodeJS that is currently installed on your computer, if your current directory is a NodeJS project.

**Segment**: Smaller sub-components that compose a module. For example, the "symbol" segment in the "nodejs" module contains the character that is shown before the version number (⬢ by default).

Here is the representation of the node module. In the following example, "symbol" and "version" are segments within it. Every module also has a prefix and suffix that are the default terminal color.

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### Стиль строк

Most modules in starship allow you to configure their display styles. This is done with an entry (usually called `style`) which is a string specifying the configuration. Here are some examples of style strings along with what they do. For details on the full syntax, consult the [advanced config guide](/advanced-config/).

- `"fg:green bg:blue"` устанавливает зеленый текст на синем фоне
- `"bg:blue fg:bright-green"` устанавливает ярко-зеленый текст на синем фоне
- `"bold fg:27"` устанавливает жирный текст с [цветом ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` устанавливает подчёркиваемый текст цвета сожженного апельсина
- `"bold italic fg:purple"` устанавливает жирный фиолетовый текст
- `""` выключает все стили

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors. Also, to get italic text, your terminal must support italics.

## Командная строка

This is the list of prompt-wide configuration options.

### Опции

| Переменная     | По умолчанию                  | Описание                                                 |
| -------------- | ----------------------------- | -------------------------------------------------------- |
| `add_newline`  | `true`                        | Добавление пустой строки перед началом командной строки. |
| `prompt_order` | [link](#default-prompt-order) | Настройка порядка появления модулей командной строки.    |
| `scan_timeout` | `30`                          | Тайм-аут запуска сканирования файлов (в миллисекундах).  |

### Пример

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
add_newline = false
# Overwrite a default_prompt_order and  use custom prompt_order
prompt_order=["rust","line_break","package","line_break","character"]
# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10
```

### Порядок модулей командной строки по умолчанию

The default `prompt_order` is used to define the order in which modules are shown in the prompt, if empty or no `prompt_order` is provided. The default is as shown:

```toml
prompt_order = [
    "username",
    "hostname",
    "kubernetes",
    "directory",
    "git_branch",
    "git_commit",
    "git_state",
    "git_status",
    "hg_branch",
    "package",
    "dotnet",
    "golang",
    "java",
    "nodejs",
    "php",
    "python",
    "ruby",
    "rust",
    "terraform",
    "nix_shell",
    "conda",
    "memory_usage",
    "aws",
    "env_var",
    "cmd_duration",
    "line_break",
    "jobs",
    "battery",
    "time",
    "character",
]
```

## AWS

The `aws` module shows the current AWS region and profile. This is based on `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env var with `~/.aws/config` file.

### Опции

| Переменная        | По умолчанию    | Описание                                                         |
| ----------------- | --------------- | ---------------------------------------------------------------- |
| `symbol`          | `"☁️ "`         | Символ перед отображением текущего профиля AWS.                  |
| `displayed_items` | `all`           | Выбор элементов. Возможные значения [`all`, `profile`, `region`] |
| `region_aliases`  |                 | Таблица региона псевдонимов, отображаемая вместе с именем AWS.   |
| `style`           | `"bold yellow"` | Стиль модуля.                                                    |
| `disabled`        | `false`         | Отключение модуля `AWS`.                                         |

### Пример

```toml
# ~/.config/starship.toml

[aws]
style = "bold blue"
symbol = "🅰 "
displayed_items = "region"
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

## Батарея

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### Опции

| Переменная           | По умолчанию             | Описание                                        |
| -------------------- | ------------------------ | ----------------------------------------------- |
| `full_symbol`        | `"•"`                    | Символ, отображаемый при полной батарее.        |
| `charging_symbol`    | `"⇡"`                    | Символ, показываемый при зарядке аккумулятора.  |
| `discharging_symbol` | `"⇣"`                    | Символ, показываемый при разрядке аккумулятора. |
| `display`            | [link](#battery-display) | Порог отображения и стиль для модуля.           |
| `disabled`           | `false`                  | Отключает модуль `battery`.                     |

<details>
<summary>There are also options for some uncommon battery states.</summary>

| Переменная       | Описание                                            |
| ---------------- | --------------------------------------------------- |
| `unknown_symbol` | The symbol shown when the battery state is unknown. |
| `empty_symbol`   | The symbol shown when the battery state is empty.   |

Note: Battery indicator will be hidden if the status is `unknown` or `empty` unless you specify the option in the config.

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

The `display` configuration option is used to define when the battery indicator should be shown (threshold) and what it looks like (style). If no `display` is provided. The default is as shown:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Опции

The `display` option is an array of the following table.

| Переменная  | Описание                                                 |
| ----------- | -------------------------------------------------------- |
| `threshold` | Верхняя граница опции отображения.                       |
| `style`     | Используемый стиль, если используется опция отображения. |

#### Пример

```toml
[[battery.display]]  # "bold red" style when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" style when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"

# when capacity is over 30%, the battery indicator will not be displayed

```

## Символ

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways: by changing color (red/green) or by changing its shape (❯/✖). The latter will only be done if `use_symbol_for_status` is set to `true`.

### Опции

| Переменная              | По умолчанию   | Описание                                                                                                    |
| ----------------------- | -------------- | ----------------------------------------------------------------------------------------------------------- |
| `symbol`                | `"❯"`          | Символ, используемый перед вводом текста в командной строке.                                                |
| `error_symbol`          | `"✖"`          | Символ, используемый перед вводом текста, если предыдущая команда не удалась.                               |
| `use_symbol_for_status` | `false`        | Показывает статус ошибки путем изменения символа.                                                           |
| `vicmd_symbol`          | `"❮"`          | Символ, используемый перед вводом текста в строке, если командная строка находится в нормальном режиме vim. |
| `style_success`         | `"bold green"` | Используемый стиль, если последняя команда была успешной.                                                   |
| `style_failure`         | `"bold red"`   | Используемый стиль, если последняя команда была не успешной.                                                |
| `disabled`              | `false`        | Отключает модуль `character`.                                                                               |

### Пример

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## Длительность команды

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Опции

| Переменная          | По умолчанию    | Описание                                                             |
| ------------------- | --------------- | -------------------------------------------------------------------- |
| `min_time`          | `2_000`         | Кратчайшая продолжительность для показа времени (в миллисекундах).   |
| `show_milliseconds` | `false`         | Показывать миллисекунды в дополнение к секундам в продолжительности. |
| `prefix`            | `took`          | Префикс, отображаемый перед продолжительностью команды.              |
| `style`             | `"bold yellow"` | Стиль модуля.                                                        |
| `disabled`          | `false`         | Отключает модуль `cmd_duration`.                                     |

### Пример

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
prefix = "underwent "
```

## Конда

The `conda` module shows the current conda environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### Опции

| Переменная          | По умолчанию   | Описание                                                                                                                                                                                                     |
| ------------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `1`            | Количество каталогов, в которых путь к окружению должен быть усечен, если окружение было создано через `conda create -p [path]`. `0` означает без усечения. Также смотрите модуль [`directory`](#directory). |
| `symbol`            | `"C "`         | Символ перед названием окружения.                                                                                                                                                                            |
| `style`             | `"bold green"` | Стиль модуля.                                                                                                                                                                                                |
| `disabled`          | `false`        | Отключает модуль `conda`.                                                                                                                                                                                    |

### Пример

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## Каталог

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Опции

| Переменная          | По умолчанию  | Описание                                                                     |
| ------------------- | ------------- | ---------------------------------------------------------------------------- |
| `truncation_length` | `3`           | Количество родительских папок, к которым должен быть усечен текущий каталог. |
| `truncate_to_repo`  | `true`        | Следует или нет обрезать до корня репозитория git, в котором вы находитесь.  |
| `prefix`            | `"in "`       | Префикс, отображаемый перед папкой.                                          |
| `style`             | `"bold cyan"` | Стиль модуля.                                                                |
| `disabled`          | `false`       | Отключает модуль `directory`.                                                |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Переменная                  | По умолчанию | Описание                                                                                 |
| --------------------------- | ------------ | ---------------------------------------------------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`          | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`       | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

</details>

### Пример

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one of the following files are present in the current directory: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj`. You'll also need the .NET Core command-line tools installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

### Опции

| Переменная  | По умолчанию  | Описание                                                          |
| ----------- | ------------- | ----------------------------------------------------------------- |
| `symbol`    | `"•NET "`     | Символ перед отображением текущей версии dotnet.                  |
| `heuristic` | `true`        | Использовать быстрое определение версии, для сохранения скорости. |
| `style`     | `"bold blue"` | Стиль модуля.                                                     |
| `disabled`  | `false`       | Отключает модуль `dotnet`.                                        |

### Пример

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Переменная Окружения

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- Опция `variable` соответствует существующей переменной среды
- Опция `variable` не определена, но определена опция `default`

### Опции

| Переменная | По умолчанию     | Описание                                                           |
| ---------- | ---------------- | ------------------------------------------------------------------ |
| `symbol`   |                  | Символ, используемый перед отображением значения переменной.       |
| `variable` |                  | Отображаемая переменная окружения.                                 |
| `default`  |                  | Значение отображаемое, когда выбранная переменная не определена.   |
| `prefix`   | `""`             | Префикс, отображаемый, непосредственно перед значением переменной. |
| `suffix`   | `""`             | Префикс, отображаемый, непосредственно после значением переменной. |
| `style`    | `"dimmed black"` | Стиль модуля.                                                      |
| `disabled` | `false`          | Отключает модуль `env_var`.                                        |

### Пример

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Ветвь Git

The `git_branch` module shows the active branch of the repo in your current directory.

### Опции

| Переменная          | По умолчанию    | Описание                                                                                                        |
| ------------------- | --------------- | --------------------------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | Символ, используемый перед именем ветки репозитория в вашей текущей директории.                                 |
| `truncation_length` | `2^63 - 1`      | Отрезает ветку git до X графемов                                                                                |
| `truncation_symbol` | `"…"`           | Символ, используемый для обозначения усечения названия ветки. Вы можете использовать "", чтобы не видеть символ |
| `style`             | `"bold purple"` | Стиль модуля.                                                                                                   |
| `disabled`          | `false`         | Отключает модуль `git_branch`.                                                                                  |

### Пример

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Коммит Git

The `git_commit` module shows the current commit hash of the repo in your current directory.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Опции

| Переменная           | По умолчанию   | Описание                                       |
| -------------------- | -------------- | ---------------------------------------------- |
| `commit_hash_length` | `7`            | Длина отображаемого хэша коммита git.          |
| `prefix`             | `"("`          | Префикс, отображаемый сразу после коммита.     |
| `suffix`             | `")"`          | Суффикс, отображаемый сразу после коммита git. |
| `style`              | `"bold green"` | Стиль модуля.                                  |
| `disabled`           | `true`         | Отключает модуль `git_commit`.                 |

### Пример

```toml
# ~/.config/starship.toml

[git_commit]
disabled = false
commit_hash_length = 4
```

## Состояние Git

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Опции

| Переменная         | По умолчанию       | Описание                                                                                                      |
| ------------------ | ------------------ | ------------------------------------------------------------------------------------------------------------- |
| `rebase`           | `"REBASING"`       | Текст, отображаемый в процессе операции `rebase`.                                                             |
| `merge`            | `"MERGING"`        | Текст, отображаемый в процессе операции `merge`.                                                              |
| `revert`           | `"REVERTING"`      | Текст, отображаемый в процессе операции `revert`.                                                             |
| `cherry_pick`      | `"CHERRY-PICKING"` | Текст, отображаемый в процессе операции `cherry-pick`.                                                        |
| `bisect`           | `"BISECTING"`      | Текст, отображаемый в процессе операции `bisect`.                                                             |
| `am`               | `"AM"`             | Текст, отображаемый в процессе операции `apply-mailbox` (`git am`).                                           |
| `am_or_rebase`     | `"AM/REBASE"`      | Текст, отображаемый, когда выполняется неоднозначный процесс `apply-mailbox` или `rebase`.                    |
| `progress_divider` | `"/"`              | Символ или текст, который будет разделять текущую и общую сумму прогресса. (напр., `" из "`, для `"3 из 10"`) |
| `style`            | `"bold yellow"`    | Стиль модуля.                                                                                                 |
| `disabled`         | `false`            | Отключает модуль `git_state`.                                                                                 |

### Пример

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Статус Git

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Опции

| Переменная         | По умолчанию                 | Описание                                               |
| ------------------ | ---------------------------- | ------------------------------------------------------ |
| `conflicted`       | `"="`                        | Эта ветка имеет конфликты слияния.                     |
| `conflicted_count` | [ссылка](#git-status-counts) | Показывать в стиле количество конфликтов.              |
| `ahead`            | `"⇡"`                        | Эта ветка впереди отслеживаемой ветви.                 |
| `behind`           | `"⇣"`                        | Эта ветка позади отслеживаемой ветви.                  |
| `diverged`         | `"⇕"`                        | Эта ветка расходится от отслеживаемой ветки.           |
| `untracked`        | `"?"`                        | В рабочей директории есть неотслеженные файлы.         |
| `untracked_count`  | [ссылка](#git-status-counts) | Показывать в стиле количество неотслеженных файлов.    |
| `stashed`          | `"$"`                        | A stash exists for the local repository.               |
| `stashed_count`    | [ссылка](#git-status-counts) | Show and style the number of stashes.                  |
| `modified`         | `"!"`                        | There are file modifications in the working directory. |
| `modified_count`   | [ссылка](#git-status-counts) | Show and style the number of modified files.           |
| `staged`           | `"+"`                        | A new file has been added to the staging area.         |
| `staged_count`     | [ссылка](#git-status-counts) | Show and style the number of files staged files.       |
| `renamed`          | `"»"`                        | A renamed file has been added to the staging area.     |
| `renamed_count`    | [ссылка](#git-status-counts) | Show and style the number of renamed files.            |
| `deleted`          | `"✘"`                        | A file's deletion has been added to the staging area.  |
| `deleted_count`    | [ссылка](#git-status-counts) | Show and style the number of deleted files.            |
| `show_sync_count`  | `false`                      | Show ahead/behind count of the branch being tracked.   |
| `prefix`           | `[`                          | Prefix to display immediately before git status.       |
| `suffix`           | `]`                          | Suffix to display immediately after git status.        |
| `style`            | `"bold red"`                 | The style for the module.                              |
| `disabled`         | `false`                      | Disables the `git_status` module.                      |

#### Git Status Counts

| Переменная | По умолчанию | Описание                                               |
| ---------- | ------------ | ------------------------------------------------------ |
| `enabled`  | `false`      | Show the number of files                               |
| `style`    |              | Optionally style the count differently than the module |

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
staged.value = "++"
staged.style = "green"
staged_count.enabled = true
staged_count.style = "green"
renamed = "👅"
deleted = "🗑"
```

## Golang

The `golang` module shows the currently installed version of Golang. The module will be shown if any of the following conditions are met:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Опции

| Переменная | По умолчанию  | Описание                                                 |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | Стиль модуля.                                            |
| `disabled` | `false`       | Disables the `golang` module.                            |

### Пример

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Опции

| Переменная          | По умолчанию    | Описание                                                                                     |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated.                                     |
| `style`             | `"bold purple"` | Стиль модуля.                                                                                |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### Пример

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Hostname

The `hostname` module shows the system hostname.

### Опции

| Переменная | По умолчанию          | Описание                                                                                                                             |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session.                                                                                 |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.                                                                                   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.                                                                                    |
| `trim_at`  | `"."`                 | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `style`    | `"bold dimmed green"` | Стиль модуля.                                                                                                                        |
| `disabled` | `false`               | Disables the `hostname` module.                                                                                                      |

### Пример

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
prefix = "⟪"
suffix = "⟫"
trim_at = ".companyname.com"
disabled = false
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Опции

| Переменная  | По умолчанию  | Описание                                              |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | Стиль модуля.                                         |
| `disabled`  | `false`       | Disables the `jobs` module.                           |

### Пример

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Опции

| Переменная | По умолчанию  | Описание                                            |
| ---------- | ------------- | --------------------------------------------------- |
| `symbol`   | `"☸ "`        | The symbol used before displaying the Cluster info. |
| `style`    | `"bold blue"` | Стиль модуля.                                       |
| `disabled` | `true`        | Disables the `kubernetes` module                    |

### Пример

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dim green"
disabled = false
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Опции

| Переменная | По умолчанию | Описание                                                           |
| ---------- | ------------ | ------------------------------------------------------------------ |
| `disabled` | `false`      | Disables the `line_break` module, making the prompt a single line. |

### Пример

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### Опции

| Переменная   | По умолчанию | Описание                           |
| ------------ | ------------ | ---------------------------------- |
| `use_name`   | `false`      | Display the name of the nix-shell. |
| `impure_msg` | `"impure"`   | Customize the "impure" msg.        |
| `pure_msg`   | `"pure"`     | Customize the "pure" msg.          |
| `style`      | `"bold red"` | Стиль модуля.                      |
| `disabled`   | `false`      | Disables the `nix_shell` module.   |

### Пример

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
```

## Java

The `java` module shows the currently installed version of Java. The module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle`, `build.gradle.kts` or `build.sbt` file
- The current directory contains a file with the `.java`, `.class` or `.jar` extension

### Опции

| Переменная | По умолчанию   | Описание                                               |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"☕ "`         | The symbol used before displaying the version of Java. |
| `style`    | `"dimmed red"` | Стиль модуля.                                          |
| `disabled` | `false`        | Disables the `java` module.                            |

### Пример

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Опции

| Переменная        | По умолчанию          | Описание                                                      |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | Display memory usage as a percentage of the available memory. |
| `show_swap`       | `true`                | Display swap usage if total swap is non-zero.                 |
| `threshold`       | `75`                  | Hide the memory usage unless it exceeds this percentage.      |
| `symbol`          | `"🐏 "`                | The symbol used before displaying the memory usage.           |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | Стиль модуля.                                                 |
| `disabled`        | `true`                | Disables the `memory_usage` module.                           |

### Пример

```toml
# ~/.config/starship.toml

[memory_usage]
show_percentage = true
show_swap = true
threshold = -1
symbol = " "
separator = "/"
style = "bold dimmed green"
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. The module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### Опции

| Переменная | По умолчанию   | Описание                                                 |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | Стиль модуля.                                            |
| `disabled` | `false`        | Disables the `nodejs` module.                            |

### Пример

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, and `poetry` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Опции

| Переменная | По умолчанию | Описание                                                   |
| ---------- | ------------ | ---------------------------------------------------------- |
| `symbol`   | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`    | `"bold red"` | Стиль модуля.                                              |
| `disabled` | `false`      | Disables the `package` module.                             |

### Пример

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

The `php` module shows the currently installed version of PHP. The module will be shown if any of the following conditions are met:

- The current directory contains a `composer.json` file
- The current directory contains a `.php` file

### Опции

| Переменная | По умолчанию | Описание                                              |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold red"` | Стиль модуля.                                         |
| `disabled` | `false`      | Disables the `php` module.                            |

### Пример

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

The `python` module shows the currently installed version of Python.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name.

Otherwise, it will display the version number from `python --version` and show the current Python virtual environment if one is activated.

The module will be shown if any of the following conditions are met:

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- A virtual environment is currently activated

### Опции

| Переменная           | По умолчанию    | Описание                                                                    |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `style`              | `"bold yellow"` | Стиль модуля.                                                               |
| `disabled`           | `false`         | Disables the `python` module.                                               |

### Пример

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

The `ruby` module shows the currently installed version of Ruby. The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.rb` file

### Опции

| Переменная | По умолчанию | Описание                                               |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`       | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | Стиль модуля.                                          |
| `disabled` | `false`      | Disables the `ruby` module.                            |

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

| Переменная | По умолчанию | Описание                                               |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`       | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | Стиль модуля.                                          |
| `disabled` | `false`      | Disables the `rust` module.                            |

### Пример

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. The module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### Опции

| Переменная     | По умолчанию | Описание                                                    |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`       | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | Стиль модуля.                                               |
| `disabled`     | `false`      | Disables the `terraform` module.                            |

### Пример

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Опции

| Переменная        | По умолчанию  | Описание                                                                                                            |
| ----------------- | ------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`       | Enables 12 hour formatting                                                                                          |
| `format`          | see below     | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `bold yellow` | The style for the module time                                                                                       |
| `utc_time_offset` | `local`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |
| `disabled`        | `true`        | Disables the `time` module.                                                                                         |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### Пример

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = -5
```

## Username

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### Опции

| Переменная    | По умолчанию    | Описание                              |
| ------------- | --------------- | ------------------------------------- |
| `style_root`  | `"bold red"`    | The style used when the user is root. |
| `style_user`  | `"bold yellow"` | The style used for non-root users.    |
| `show_always` | `false`         | Always shows the `username` module.   |
| `disabled`    | `false`         | Disables the `username` module.       |

### Пример

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
