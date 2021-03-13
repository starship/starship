# 設定

為了開始設定 Starship，請建立下右檔案： `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

所有關於 Starship 的設定都在這個 [TOML](https://github.com/toml-lang/toml) 檔案內：

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

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
```

### Logging

By default starship logs warnings and errors into a file named `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, where the session key is corresponding to a instance of your terminal. This, however can be changed using the `STARSHIP_CACHE` environment variable:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### 術語

**模組 (Module)**： 提示字元中的一個元件，基於你的作業系統提供的背景資訊來提供訊息。 舉例來說，如果你現在的資料夾是一個 NodeJS 專案，"nodejs" 模組會顯示出現在安裝在你的電腦上的 NodeJS 版本。

**Variable**: Smaller sub-components that contain information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of NodeJS.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Format Strings

Format strings are the format that a module prints all its variables with. Most modules have an entry called `format` that configures the display format of the module. You can use texts, variables and text groups in a format string.

#### 變數

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
- `[⌘ $version](bold green)` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `[a [b](red) c](green)` will print `a b c` with `b` red, and `a` and `c` green.

#### 風格字串

Starship 內大多數的模組允許你設定他們的顯示風格。 這要透過一個條目 (通常叫做 `style`)，這個條目使用一個字串來進行設定。 這裡給幾個風格字串的例子，以及這些字串的功用。 對於完整語法的詳細說明，請參照 [進階設定指南](/advanced-config/)。

- `"fg:green bg:blue"` 在一個藍色背景上設定綠色文字
- `"bg:blue fg:bright-green"` 在一個藍色背景上設定亮綠色文字
- `"bold fg:27"` 設定具有 [ANSI 顏色](https://i.stack.imgur.com/KTSQa.png) 27 號的粗體文字
- `"underline bg:#bf5700"` 在一個燒橙色背景上設定有底線的文字
- `"bold italic fg:purple"` 設定粗體、斜體且紫色的文字
- `""` 明確地關閉所有風格

注意風格產出的樣子取決於你的終端機模擬器。 例如，有些終端機模擬器會提升顏色的亮度而不是讓文字變粗體，而且有些色彩主題對一般與加亮顏色使用的是相同色碼。 除此之外，為了要有斜體字，你的終端機一定要支援斜體。

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

## 提示字元

以下是針對提示字元內容的設定。

### 選項

| Option         | 預設                           | 說明                                                    |
| -------------- | ---------------------------- | ----------------------------------------------------- |
| `format`       | [連結](#default-prompt-format) | Configure the format of the prompt.                   |
| `scan_timeout` | `30`                         | Timeout for starship to scan files (in milliseconds). |
| `add_newline`  | `true`                       | Inserts blank line between shell prompts.             |

### 範例

```toml
# ~/.config/starship.toml

# Use custom format
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10

# Disable the blank line at the start of the prompt
add_newline = false
```

### Default Prompt Format

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. 預設如下：

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
$kotlin\
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

`aws` 模組顯示現在 AWS 的區域與概況。 這是根據 `AWS_REGION`、`AWS_DEFAULT_REGION` 與 `AWS_PROFILE` 環境變數及 `~/.aws/config` 檔案。

從 `AWS_VAULT`讀取而使用 [aws-vault](https://github.com/99designs/aws-vault) 這個設定檔

When using [awsu](https://github.com/kreuzwerker/awsu) the profile is read from the `AWSU_PROFILE` env var.

### 選項

| Option           | 預設                                                  | 說明                         |
| ---------------- | --------------------------------------------------- | -------------------------- |
| `format`         | `'on [$symbol($profile )(\($region\) )]($style)'` | The format for the module. |
| `symbol`         | `"☁️ "`                                             | 顯示在目前 AWS 配置之前的符號。         |
| `region_aliases` |                                                     | 除了AWS名稱外，顯示區域別名表           |
| `style`          | `"bold yellow"`                                     | 這個模組的風格。                   |
| `disabled`       | `false`                                             | 停用 `AWS` 模組。               |

### Variables

| 變數        | 範例               | 說明                                   |
| --------- | ---------------- | ------------------------------------ |
| region    | `ap-northeast-1` | The current AWS region               |
| profile   | `astronauts`     | The current AWS profile              |
| symbol    |                  | Mirrors the value of option `symbol` |
| style\* |                  | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Examples

#### Display everything

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

## 電池

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### 選項

| Option               | 預設                                | 說明                         |
| -------------------- | --------------------------------- | -------------------------- |
| `full_symbol`        | `""`                             | 當電池充飽時顯示的符號。               |
| `charging_symbol`    | `""`                             | 當電池正在充電時顯示的符號。             |
| `discharging_symbol` | `""`                             | 當電池正在放電時顯示的符號。             |
| `unknown_symbol`     | `""`                             | 當電池狀態不明時顯示的符號。             |
| `empty_symbol`       | `""`                             | 當電池沒電時顯示的符號。               |
| `format`             | `"[$symbol$percentage]($style) "` | The format for the module. |
| `display`            | [連結](#battery-display)            | 顯示的門檻與模組的風格。               |
| `disabled`           | `false`                           | 停用 `battery` 模組。           |


### 範例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### 電池顯示

The `display` configuration option is used to define when the battery indicator should be shown (threshold) and what it looks like (style). If no `display` is provided. 預設如下：

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### 選項

The `display` option is an array of the following table.

| Option      | 說明          |
| ----------- | ----------- |
| `threshold` | 顯示選項的上界。    |
| `style`     | 顯示選項使用時的風格。 |

#### 範例

```toml
[[battery.display]]  # 0% 到 10% 電量之間時，使用 "bold red" 風格
threshold = 10
style = "bold red"

[[battery.display]]  # 10% 到 30% 電量之間時，使用 "bold yellow" 風格
threshold = 30
style = "bold yellow"

# 當電量超過 30% 時，電量指示就不會顯示出來

```

## 字元

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change it's shape take a look at [this example](#with-custom-error-shape).

::: warning `error_symbol` is not supported on elvish shell. :::

### 選項

| Option           | 預設                  | 說明                                                                               |
| ---------------- | ------------------- | -------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | The format string used before the text input.                                    |
| `success_symbol` | `"[❯](bold green)"` | The format string used before the text input if the previous command succeeded.  |
| `error_symbol`   | `"[❯](bold red)"`   | The format string used before the text input if the previous command failed.     |
| `vicmd_symbol`   | `"[❮](bold green)"` | The format string used before the text input if the shell is in vim normal mode. |
| `disabled`       | `false`             | 停用 `character` 模組。                                                               |

### Variables

| 變數     | 範例 | 說明                                                                    |
| ------ | -- | --------------------------------------------------------------------- |
| symbol |    | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

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

The `cmake` module shows the currently installed version of CMake. By default the module will be activated if any of the following conditions are met:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### 選項

| Option              | 預設                                     | 說明                                           |
| ------------------- | -------------------------------------- | -------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`   | The format for the module.                   |
| `symbol`            | `"喝 "`                                 | The symbol used before the version of cmake. |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this moudle  |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Which filenames should trigger this module   |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module     |
| `style`             | `"bold blue"`                          | 這個模組的風格。                                     |
| `disabled`          | `false`                                | Disables the `cmake` module.                 |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v3.17.3` | The version of cmake                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

## 指令持續時間

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### 選項

| Option               | 預設                            | 說明                                                    |
| -------------------- | ----------------------------- | ----------------------------------------------------- |
| `min_time`           | `2_000`                       | Shortest duration to show time for (in milliseconds). |
| `show_milliseconds`  | `false`                       | 顯示時間除了以秒為單位外，亦以毫秒顯示                                   |
| `format`             | `"took [$duration]($style) "` | The format for the module.                            |
| `style`              | `"bold yellow"`               | 這個模組的風格。                                              |
| `disabled`           | `false`                       | 停用 `cmd_duration` 模組。                                 |
| `show_notifications` | `false`                       | Show desktop notifications when command completes.    |
| `min_time_to_notify` | `45_000`                      | Shortest duration for notification (in milliseconds). |

::: tip

Showing desktop notifications requires starship to be built with `rust-notify` support. You check if your starship supports notifications by running `STARSHIP_LOG=debug starship module cmd_duration -d 60000` when `show_notifications` is set to `true`.

:::

### Variables

| 變數        | 範例       | 說明                                      |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | Mirrors the value of option `style`     |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

The `conda` module shows the current conda environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### 選項

| Option              | 預設                                     | 說明                                                                                              |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | 如果環境變數由所`conda create -p [path]`產生時，環境變數的資料夾需要截斷的數目。 `0` 表示不截斷 也請參考 [`directory`](#directory)模組 |
| `symbol`            | `"🅒 "`                                 | 環境名稱前使用的符號。                                                                                     |
| `style`             | `"bold green"`                         | 這個模組的風格。                                                                                        |
| `format`            | `"via [$symbol$environment]($style) "` | The format for the module.                                                                      |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                      |
| `disabled`          | `false`                                | 停用 `conda` 模組。                                                                                  |

### Variables

| 變數          | 範例           | 說明                                   |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current conda environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*   |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `shard.yml` 檔案
- 現在資料夾中含有一個`.cr`檔案

### 選項

| Option              | 預設                                   | 說明                                                        |
| ------------------- | ------------------------------------ | --------------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | The symbol used before displaying the version of crystal. |
| `style`             | `"bold red"`                         | 這個模組的風格。                                                  |
| `detect_extensions` | `["cr"]`                             | Which extensions should trigger this module.              |
| `detect_files`      | `["shard.yml"]`                      | Which filenames should trigger this module.               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                 |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                |
| `disabled`          | `false`                              | Disables the `crystal` module.                            |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v0.32.1` | The version of `crystal`             |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

The `dart` module shows the currently installed version of Dart. By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml`, `pubspec.yml` or `pubspec.lock` file

### 選項

| Option              | 預設                                                | 說明                                              |
| ------------------- | ------------------------------------------------- | ----------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | The format for the module.                      |
| `symbol`            | `"🎯 "`                                            | A format string representing the symbol of Dart |
| `detect_extensions` | `['dart']`                                        | Which extensions should trigger this moudle.    |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Which filenames should trigger this module.     |
| `detect_folders`    | `[".dart_tool"]`                                  | Which folders should trigger this module.       |
| `style`             | `"bold blue"`                                     | 這個模組的風格。                                        |
| `disabled`          | `false`                                           | Disables the `dart` module.                     |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v2.8.4` | The version of `dart`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## 資料夾

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### 選項

| Option              | 預設                                                 | 說明                                                    |
| ------------------- | -------------------------------------------------- | ----------------------------------------------------- |
| `truncation_length` | `3`                                                | 到達現在資料夾的路徑中，要被裁減掉的資料夾數目。                              |
| `truncate_to_repo`  | `true`                                             | 是否要裁減到你現在所在的 git 儲存庫的根目錄。                             |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | The format for the module.                            |
| `style`             | `"bold cyan"`                                      | 這個模組的風格。                                              |
| `disabled`          | `false`                                            | 停用 `directory` 模組。                                    |
| `read_only`         | `"🔒"`                                              | The symbol indicating current directory is read only. |
| `read_only_style`   | `"red"`                                            | The style for the read only symbol.                   |
| `truncation_symbol` | `""`                                               | The symbol to prefix to truncated paths. eg: "…/"     |
| `home_symbol`       | `"~"`                                              | The symbol indicating home directory.                 |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | 預設     | 說明                                                                                                                                                                     |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`    | 當使用 fish shell 的 pwd 路徑邏輯時使用的字元數量。                                                                                                                                     |
| `use_logical_path`          | `true` | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| 變數        | 範例                    | 說明                                  |
| --------- | --------------------- | ----------------------------------- |
| path      | `"D:/Projects"`       | The current directory path          |
| style\* | `"black bold dimmed"` | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### 選項

| Option              | 預設                                                            | 說明                                                                                |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | The format for the module.                                                        |
| `symbol`            | `"🐳 "`                                                        | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                        | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                          | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                          | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `"blue bold"`                                                 | 這個模組的風格。                                                                          |
| `disabled`          | `false`                                                       | Disables the `docker_context` module.                                             |

### Variables

| 變數        | 範例             | 說明                                   |
| --------- | -------------- | ------------------------------------ |
| context   | `test_context` | The current docker context           |
| symbol    |                | Mirrors the value of option `symbol` |
| style\* |                | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

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

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### 選項

| Option              | 預設                                                                                                      | 說明                                           |
| ------------------- | ------------------------------------------------------------------------------------------------------- | -------------------------------------------- |
| `format`            | `"[$symbol($version )(🎯 $tfm )]($style)"`                                                               | The format for the module.                   |
| `symbol`            | `"•NET "`                                                                                               | 在顯示 dotnet 版本之前用的符號。                         |
| `heuristic`         | `true`                                                                                                  | 使用更快速的版本偵測法來保持 starship 的速度。                 |
| `detect_extensions` | `["sln", "csproj", "fsproj", "xproj"]`                                                                  | Which extensions should trigger this module. |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Which filenames should trigger this module.  |
| `detect_folders`    | `[]`                                                                                                    | Which folders should trigger this modules.   |
| `style`             | `"bold blue"`                                                                                           | 這個模組的風格。                                     |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                |

### Variables

| 變數        | 範例               | 說明                                                                 |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Mirrors the value of option `symbol`                               |
| style\* |                  | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `mix.exs` 檔案.

### 選項

| Option              | 預設                                                          | 說明                                                              |
| ------------------- | ----------------------------------------------------------- | --------------------------------------------------------------- |
| `symbol`            | `"💧 "`                                                      | The symbol used before displaying the version of Elixir/Erlang. |
| `detect_extensions` | `[]`                                                        | Which extensions should trigger this module.                    |
| `detect_files`      | `["mix.exs"]`                                               | Which filenames should trigger this module.                     |
| `detect_folders`    | `[]`                                                        | Which folders should trigger this modules.                      |
| `style`             | `"bold purple"`                                             | 這個模組的風格。                                                        |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                               |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                   |

### Variables

| 變數          | 範例      | 說明                                   |
| ----------- | ------- | ------------------------------------ |
| version     | `v1.10` | The version of `elixir`              |
| otp_version |         | The otp version of `elixir`          |
| symbol      |         | Mirrors the value of option `symbol` |
| style\*   |         | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `elm.json` 檔案
- 現在資料夾中包含一個 `elm-package.json` 檔案
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### 選項

| Option              | 預設                                                 | 說明                                              |
| ------------------- | -------------------------------------------------- | ----------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | The format for the module.                      |
| `symbol`            | `"🌳 "`                                             | A format string representing the symbol of Elm. |
| `detect_extensions` | `["elm"]`                                          | Which extensions should trigger this module.    |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Which filenames should trigger this module.     |
| `detect_folders`    | `["elm-stuff"]`                                    | Which folders should trigger this modules.      |
| `style`             | `"cyan bold"`                                      | 這個模組的風格。                                        |
| `disabled`          | `false`                                            | Disables the `elm` module.                      |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v0.19.1` | The version of `elm`                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## 環境變數

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- `variable` 設定選項符合一個存在的環境變數。
- 沒有設定 `variable` 選項，但是有設定 `default` 選項。

### 選項

| Option     | 預設                             | 說明                         |
| ---------- | ------------------------------ | -------------------------- |
| `symbol`   |                                | 顯示在變數數值之前的符號。              |
| `variable` |                                | 要顯示的環境變數。                  |
| `default`  |                                | 在選擇的變數值沒有定義時，顯示的預設值。       |
| `format`   | `"with [$env_value]($style) "` | The format for the module. |
| `disabled` | `false`                        | 停用 `env_var` 模組。           |

### Variables

| 變數        | 範例                                          | 說明                                         |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Mirrors the value of option `symbol`       |
| style\* | `black bold dimmed`                         | Mirrors the value of option `style`        |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `rebar.config` 檔案.
- 現在資料夾中包含一個 `erlang.mk` 檔案.

### 選項

| Option              | 預設                                   | 說明                                                       |
| ------------------- | ------------------------------------ | -------------------------------------------------------- |
| `symbol`            | `" "`                               | The symbol used before displaying the version of erlang. |
| `style`             | `"bold red"`                         | 這個模組的風格。                                                 |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.             |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Which filenames should trigger this module.              |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.               |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                               |
| `disabled`          | `false`                              | Disables the `erlang` module.                            |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v22.1.3` | The version of `erlang`              |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Gcloud

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### 選項

| Option           | 預設                                               | 說明                                                              |
| ---------------- | ------------------------------------------------ | --------------------------------------------------------------- |
| `format`         | `'on [$symbol$account(\($region\))]($style) '` | The format for the module.                                      |
| `symbol`         | `"☁️ "`                                          | The symbol used before displaying the current GCP profile.      |
| `region_aliases` |                                                  | Table of region aliases to display in addition to the GCP name. |
| `style`          | `"bold blue"`                                    | 這個模組的風格。                                                        |
| `disabled`       | `false`                                          | Disables the `gcloud` module.                                   |

### Variables

| 變數        | 範例                | 說明                                                                 |
| --------- | ----------------- | ------------------------------------------------------------------ |
| region    | `us-central1`     | The current GCP region                                             |
| account   | `foo@example.com` | The current GCP profile                                            |
| project   |                   | The current GCP project                                            |
| active    | `default`         | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |                   | Mirrors the value of option `symbol`                               |
| style\* |                   | Mirrors the value of option `style`                                |

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

## Git 分支

The `git_branch` module shows the active branch of the repo in your current directory.

### 選項

| Option               | 預設                               | 說明                                                                                   |
| -------------------- | -------------------------------- | ------------------------------------------------------------------------------------ |
| `always_show_remote` | `false`                          | Shows the remote tracking branch name, even if it is equal to the local branch name. |
| `format`             | `"on [$symbol$branch]($style) "` | The format for the module. Use `"$branch"` to refer to the current branch name.      |
| `symbol`             | `" "`                           | A format string representing the symbol of git branch.                               |
| `style`              | `"bold purple"`                  | 這個模組的風格。                                                                             |
| `truncation_length`  | `2^63 - 1`                       | Truncates a git branch to X graphemes.                                               |
| `truncation_symbol`  | `"…"`                            | 用來指示分支名稱被縮減的符號。 You can use `""` for no symbol.                                      |
| `only_attached`      | `false`                          | Only show the branch name when not in a detached HEAD state.                         |
| `disabled`           | `false`                          | 停用 `git_branch` 模組。                                                                  |

### Variables

| 變數            | 範例       | 說明                                                                                                   |
| ------------- | -------- | ---------------------------------------------------------------------------------------------------- |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached HEAD). |
| remote_name   | `origin` | The remote name.                                                                                     |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                     |
| symbol        |          | Mirrors the value of option `symbol`                                                                 |
| style\*     |          | Mirrors the value of option `style`                                                                  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### 選項

| Option               | 預設                                                     | 說明                                                    |
| -------------------- | ------------------------------------------------------ | ----------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | The length of the displayed git commit hash.          |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | The format for the module.                            |
| `style`              | `"bold green"`                                         | 這個模組的風格。                                              |
| `only_detached`      | `true`                                                 | Only show git commit hash when in detached HEAD state |
| `tag_disabled`       | `true`                                                 | Disables showing tag info in `git_commit` module.     |
| `tag_symbol`         | `"🏷 "`                                                 | Tag symbol prefixing the info shown                   |
| `disabled`           | `false`                                                | Disables the `git_commit` module.                     |

### Variables

| 變數        | 範例        | 說明                                  |
| --------- | --------- | ----------------------------------- |
| hash      | `b703eb3` | The current git commit hash         |
| style\* |           | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### 選項

| Option         | 預設                                                              | 說明                                                                                      |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | 這個模組的風格。                                                                                |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | The format for the module.                                                              |
| `disabled`     | `false`                                                         | 停用 `git_state` 模組。                                                                      |

### Variables

| 變數               | 範例         | 說明                                  |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*        |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### 選項

| Option       | 預設                                              | 說明                                  |
| ------------ | ----------------------------------------------- | ----------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status` |
| `conflicted` | `"="`                                           | 這個分支有合併衝突。                          |
| `ahead`      | `"⇡"`                                           | The format of `ahead`               |
| `behind`     | `"⇣"`                                           | The format of `behind`              |
| `diverged`   | `"⇕"`                                           | The format of `diverged`            |
| `untracked`  | `"?"`                                           | The format of `untracked`           |
| `stashed`    | `"$"`                                           | The format of `stashed`             |
| `modified`   | `"!"`                                           | The format of `modified`            |
| `staged`     | `"+"`                                           | The format of `staged`              |
| `renamed`    | `"»"`                                           | The format of `renamed`             |
| `deleted`    | `"✘"`                                           | The format of `deleted`             |
| `style`      | `"bold red"`                                    | 這個模組的風格。                            |
| `disabled`   | `false`                                         | 停用 `git_status` 模組。                 |

### Variables

The following variables can be used in `format`:

| 變數             | 說明                                                                                            |
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

| 變數             | 說明                                             |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| 變數      | 說明                       |
| ------- | ------------------------ |
| `count` | Show the number of files |

### 範例

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

The `golang` module shows the currently installed version of Golang. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `go.mod` 檔案
- 現在資料夾中含有一個 `go.sum` 檔案
- 現在資料夾中含有一個 `glide.yaml` 檔案
- 現在資料夾中含有一個 `Gopkg.yml` 檔案
- 現在資料夾中含有一個 `Gopkg.lock` 檔案
- The current directory contains a `.go-version` file
- 現在資料夾中含有一個 `Godeps` 資料夾
- 現在資料夾中含有一個檔案具有 `.go` 副檔名

### 選項

| Option              | 預設                                                                             | 說明                                             |
| ------------------- | ------------------------------------------------------------------------------ | ---------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                           | The format for the module.                     |
| `symbol`            | `"🐹 "`                                                                         | A format string representing the symbol of Go. |
| `detect_extensions` | `["go"]`                                                                       | Which extensions should trigger this moudle.   |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Which filenames should trigger this module.    |
| `detect_folders`    | `["Godeps"]`                                                                   | Which folders should trigger this module.      |
| `style`             | `"bold cyan"`                                                                  | 這個模組的風格。                                       |
| `disabled`          | `false`                                                                        | Disables the `golang` module.                  |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v1.12.1` | The version of `go`                  |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of Helm. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `helmfile.yaml` 檔案
- The current directory contains a `Chart.yaml` file

### 選項

| Option              | 預設                                   | 說明                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                       |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.     |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Which filenames should trigger this module.      |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.       |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm. |
| `style`             | `"bold white"`                       | 這個模組的風格。                                         |
| `disabled`          | `false`                              | Disables the `helm` module.                      |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v3.1.1` | The version of `helm`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## 主機名稱

The `hostname` module shows the system hostname.

### 選項

| Option     | 預設                          | 說明                                                         |
| ---------- | --------------------------- | ---------------------------------------------------------- |
| `ssh_only` | `true`                      | 只在連接到一個 SSH session 時顯示主機名稱。                               |
| `trim_at`  | `"."`                       | 擷取出主機名稱的斷點，以第一個符合的為準。 `"."` 會讓它停在第一個點的符號。 `""` 會停用任何的截斷功能。 |
| `format`   | `"[$hostname]($style) in "` | The format for the module.                                 |
| `style`    | `"bold dimmed green"`       | 這個模組的風格。                                                   |
| `disabled` | `false`                     | 停用 `hostname` 模組。                                          |

### Variables

| 變數        | 範例 | 說明                                   |
| --------- | -- | ------------------------------------ |
| symbol    |    | Mirrors the value of option `symbol` |
| style\* |    | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### 選項

| Option              | 預設                                                                                                        | 說明                                              |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | The format for the module.                      |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Which extensions should trigger this module.    |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Which filenames should trigger this module.     |
| `detect_folders`    | `[]`                                                                                                      | Which folders should trigger this modules.      |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java |
| `style`             | `"red dimmed"`                                                                                            | 這個模組的風格。                                        |
| `disabled`          | `false`                                                                                                   | Disables the `java` module.                     |

### Variables

| 變數        | 範例    | 說明                                   |
| --------- | ----- | ------------------------------------ |
| version   | `v14` | The version of `java`                |
| symbol    |       | Mirrors the value of option `symbol` |
| style\* |       | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## 工作

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

::: warning

This module is not supported on tcsh.

:::

### 選項

| Option      | 預設                            | 說明                                               |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | 在超過指定值時顯示工作數量。                                   |
| `format`    | `"[$symbol$number]($style) "` | The format for the module.                       |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | 這個模組的風格。                                         |
| `disabled`  | `false`                       | 停用 `jobs` 模組。                                    |

### Variables

| 變數        | 範例  | 說明                                   |
| --------- | --- | ------------------------------------ |
| number    | `1` | The number of jobs                   |
| symbol    |     | Mirrors the value of option `symbol` |
| style\* |     | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### 選項

| Option              | 預設                                   | 說明                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                        |
| `detect_extensions` | `["jl"]`                             | Which extensions should trigger this module.      |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Which filenames should trigger this module.       |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.        |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia. |
| `style`             | `"bold purple"`                      | 這個模組的風格。                                          |
| `disabled`          | `false`                              | Disables the `julia` module.                      |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.4.0` | The version of `julia`               |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of Kotlin. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.kt` or a `.kts` file

### 選項

| Option              | 預設                                   | 說明                                                                            |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                    |
| `detect_extensions` | `["kt", "kts"]`                      | Which extensions should trigger this module.                                  |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                                   |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                    |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `"bold blue"`                        | 這個模組的風格。                                                                      |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v1.4.21` | The version of `kotlin`              |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

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

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 選項

| Option            | 預設                                                   | 說明                                                                    |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | The format for the module.                                            |
| `style`           | `"cyan bold"`                                        | 這個模組的風格。                                                              |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Disables the `kubernetes` module.                                     |

### Variables

| 變數        | 範例                   | 說明                                       |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Mirrors the value of option `symbol`     |
| style\* |                      | Mirrors the value of option `style`      |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## 換行

The `line_break` module separates the prompt into two lines.

### 選項

| Option     | 預設      | 說明                            |
| ---------- | ------- | ----------------------------- |
| `disabled` | `false` | 停用 `line_break` 模組，讓提示字元變成一行。 |

### 範例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

The `lua` module shows the currently installed version of Lua. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### 選項

| Option              | 預設                                   | 說明                                                                         |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                 |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | Which extensions should trigger this moudle.                               |
| `detect_files`      | `[".lua-version"]`                   | Which filenames should trigger this module.                                |
| `detect_folders`    | `["lua"]`                            | Which folders should trigger this module.                                  |
| `style`             | `"bold blue"`                        | 這個模組的風格。                                                                   |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v5.4.0` | The version of `lua`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## 記憶體使用量

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 選項

| Option      | 預設                                            | 說明                         |
| ----------- | --------------------------------------------- | -------------------------- |
| `threshold` | `75`                                          | 將記憶體使用量隱藏，除非使用量超過指定值。      |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | The format for the module. |
| `symbol`    | `"🐏"`                                         | 顯示在記憶體使用量之前的符號。            |
| `style`     | `"bold dimmed white"`                         | 這個模組的風格。                   |
| `disabled`  | `true`                                        | 停用 `memory_usage` 模組。      |

### Variables

| 變數               | 範例            | 說明                                                                 |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Mirrors the value of option `symbol`                               |
| style\*        |               | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

### 範例

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

### 選項

| Option              | 預設                               | 說明                                                                                           |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | 這個模組的風格。                                                                                     |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.                                                                   |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`                            | 用來指示分支名稱被縮減的符號。                                                                              |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| branch    | `master` | The active mercurial branch          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `nim.cfg` 檔案
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### 選項

| Option              | 預設                                   | 說明                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module                             |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim. |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Which extensions should trigger this moudle.          |
| `detect_files`      | `["nim.cfg"]`                        | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.             |
| `style`             | `"bold yellow"`                      | 這個模組的風格。                                              |
| `disabled`          | `false`                              | Disables the `nim` module.                            |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.2.0` | The version of `nimc`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### 選項

| Option       | 預設                                             | 說明                                                    |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | The format for the module.                            |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | 這個模組的風格。                                              |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | 停用 `nix_shell` 模組。                                    |

### Variables

| 變數        | 範例      | 說明                                   |
| --------- | ------- | ------------------------------------ |
| state     | `pure`  | The state of the nix-shell           |
| name      | `lorri` | The name of the nix-shell            |
| symbol    |         | Mirrors the value of option `symbol` |
| style\* |         | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `package.json` 檔案
- The current directory contains a `.node-version` file
- 現在資料夾中包含一個 `node_modules` 資料夾
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### 選項

| Option              | 預設                                   | 說明                                                                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                                            |
| `symbol`            | `" "`                               | A format string representing the symbol of NodeJS.                                                    |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Which extensions should trigger this moudle.                                                          |
| `detect_files`      | `["package.json", ".node-version"]`  | Which filenames should trigger this module.                                                           |
| `detect_folders`    | `["node_modules"]`                   | Which folders should trigger this module.                                                             |
| `style`             | `"bold green"`                       | 這個模組的風格。                                                                                              |
| `disabled`          | `false`                              | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in Packages.json does not match the NodeJS version. |

###  Variables

| 變數        | 範例         | 說明                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v13.12.0` | The version of `node`                |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### 選項

| Option              | 預設                                                               | 說明                                                      |
| ------------------- | ---------------------------------------------------------------- | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                             | The format string for the module.                       |
| `symbol`            | `"🐫 "`                                                           | The symbol used before displaying the version of OCaml. |
| `detect_extensions` | `["opam", "ml", "mli", "re", "rei"]`                             | Which extensions should trigger this moudle.            |
| `detect_files`      | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]` | Which filenames should trigger this module.             |
| `detect_folders`    | `["_opam", "esy.lock"]`                                          | Which folders should trigger this module.               |
| `style`             | `"bold yellow"`                                                  | 這個模組的風格。                                                |
| `disabled`          | `false`                                                          | Disables the `ocaml` module.                            |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v4.10.0` | The version of `ocaml`               |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### 選項

| Option     | 預設                                                  | 說明                                                             |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | The format for the module.                                     |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | 這個模組的風格。                                                       |
| `disabled` | `false`                                             | Disables the `OpenStack` module.                               |

### Variables

| 變數        | 範例     | 說明                                   |
| --------- | ------ | ------------------------------------ |
| cloud     | `corp` | The current OpenStack cloud          |
| project   | `dev`  | The current OpenStack project        |
| symbol    |        | Mirrors the value of option `symbol` |
| style\* |        | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## 套件版本

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- **npm** – `npm` 套件的版本是從現在資料夾中的 `package.json` 之中擷取出來的
- **cargo** – `cargo` 套件的版本是從現在資料夾中的 `Cargo.toml` 之中擷取出來的
- **poetry** – `poetry` 套件的版本是從現在資料夾中的 `pyproject.toml` 之中擷取出來的
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present
- **helm** - The `helm` chart version is extracted from the `Chart.yaml` present
- **maven** - The `maven` package version is extracted from the `pom.xml` present
- **meson** - The `meson` package version is extracted from the `meson.build` present

> ⚠️ 顯示出來的版本是從你的現在資料夾之中擷取出來的，並非從套件管理員取得。

### 選項

| Option            | 預設                                | 說明                                                        |
| ----------------- | --------------------------------- | --------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | The format for the module.                                |
| `symbol`          | `"📦 "`                            | 顯示在套件的版本之前的符號。                                            |
| `style`           | `"bold 208"`                      | 這個模組的風格。                                                  |
| `display_private` | `false`                           | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                           | 停用 `package` 模組。                                          |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.0.0` | The version of your package          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of Perl. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### 選項

| Option              | 預設                                                                                                       | 說明                                                    |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                     |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Which extensions should trigger this moudle.          |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                                                                                     | Which folders should trigger this module.             |
| `style`             | `"bold 149"`                                                                                             | 這個模組的風格。                                              |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                           |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v5.26.1` | The version of `perl`                |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

### 範例

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of PHP. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `composer.json` 檔案
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### 選項

| Option              | 預設                                   | 說明                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                            |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP. |
| `detect_extensions` | `["php"]`                            | Which extensions should trigger this moudle.          |
| `detect_files`      | `["composer.json", ".php-version"]`  | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.             |
| `style`             | `"147 bold"`                         | 這個模組的風格。                                              |
| `disabled`          | `false`                              | Disables the `php` module.                            |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v7.3.8` | The version of `php`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `spago.dhall` 檔案
- The current directory contains a file with the `.purs` extension

### 選項

| Option              | 預設                                   | 說明                                                           |
| ------------------- | ------------------------------------ | ------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                   |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript. |
| `detect_extensions` | `["purs"]`                           | Which extensions should trigger this moudle.                 |
| `detect_files`      | `["spago.dhall"]`                    | Which filenames should trigger this module.                  |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                    |
| `style`             | `"bold white"`                       | 這個模組的風格。                                                     |
| `disabled`          | `false`                              | Disables the `purescript` module.                            |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `0.13.5` | The version of `purescript`          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

By default the module will be shown if any of the following conditions are met:

- 目前資料夾中有一個 `.python-version` 檔案
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- A virtual environment is currently activated

### 選項

| Option               | 預設                                                                                                           | 說明                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\))]($style)'`                                   | The format for the module.                                                             |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | 這個模組的風格。                                                                               |
| `pyenv_version_name` | `false`                                                                                                      | 使用 pyenv 取得 Python 的版本。                                                                |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3, "python2"]`                                                                            | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `[".py"]`                                                                                                    | Which extensions should trigger this moudle                                            |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Which filenames should trigger this module                                             |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                               |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                          |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Variables

| 變數           | 範例              | 說明                                         |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Mirrors the value of option `symbol`       |
| style        | `"yellow bold"` | Mirrors the value of option `style`        |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |


### 範例

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

## Ruby

By default the `ruby` module shows the currently installed version of Ruby. The module will be shown if any of the following conditions are met:

- 目前資料夾中有一個 `Gemfile` 檔案
- The current directory contains a `.ruby-version` file
- 目前資料夾中有一個 `.rb` 檔案

### 選項

| Option              | 預設                                   | 說明                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                       |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby. |
| `detect_extensions` | `["rb"]`                             | Which extensions should trigger this module.     |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Which filenames should trigger this module.      |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.        |
| `style`             | `"bold red"`                         | 這個模組的風格。                                         |
| `disabled`          | `false`                              | Disables the `ruby` module.                      |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v2.5.1` | The version of `ruby`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of Rust. The module will be shown if any of the following conditions are met:

- 目前資料夾中有一個 `Cargo.toml` 檔案
- 現在資料夾中包含一個檔案具有 `.rs` 副檔名

### 選項

| Option              | 預設                                   | 說明                                              |
| ------------------- | ------------------------------------ | ----------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                      |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust |
| `detect_extensions` | `["rs"]`                             | Which extensions should trigger this module.    |
| `detect_files`      | `["Cargo.toml"]`                     | Which filenames should trigger this module.     |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.       |
| `style`             | `"bold red"`                         | 這個模組的風格。                                        |
| `disabled`          | `false`                              | Disables the `rust` module.                     |

### Variables

| 變數        | 範例                | 說明                                   |
| --------- | ----------------- | ------------------------------------ |
| version   | `v1.43.0-nightly` | The version of `rustc`               |
| symbol    |                   | Mirrors the value of option `symbol` |
| style\* |                   | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 選項

| Option                 | 預設           | 說明                                            |
| ---------------------- | ------------ | --------------------------------------------- |
| `bash_indicator`       | `bsh`        | A format string used to represent bash.       |
| `fish_indicator`       | `fsh`        | A format string used to represent fish.       |
| `zsh_indicator`        | `zsh`        | A format string used to represent zsh.        |
| `powershell_indicator` | `psh`        | A format string used to represent powershell. |
| `ion_indicator`        | `ion`        | A format string used to represent ion.        |
| `elvish_indicator`     | `esh`        | A format string used to represent elvish.     |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.       |
| `format`               | `$indicator` | The format for the module.                    |
| `disabled`             | `true`       | Disables the `shell` module.                  |

### Variables

| 變數        | 預設 | 說明                                                         |
| --------- | -- | ---------------------------------------------------------- |
| indicator |    | Mirrors the value of `indicator` for currently used shell. |

### Examples
```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
disabled = false
```

## SHLVL

The `shlvl` module shows the current SHLVL ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### 選項

| Option      | 預設                           | 說明                                                          |
| ----------- | ---------------------------- | ----------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                          |
| `format`    | `"[$symbol$shlvl]($style) "` | The format for the module.                                  |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the SHLVL.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current SHLVL amount. |
| `style`     | `"bold yellow"`              | 這個模組的風格。                                                    |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                |

### Variables

| 變數        | 範例  | 說明                                   |
| --------- | --- | ------------------------------------ |
| shlvl     | `3` | The current value of SHLVL           |
| symbol    |     | Mirrors the value of option `symbol` |
| style\* |     | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### 選項

| Option     | 預設                               | 說明                                               |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | The format for the module.                       |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | 這個模組的風格。                                         |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

| 變數        | 範例           | 說明                                   |
| --------- | ------------ | ------------------------------------ |
| env       | `centos.img` | The current singularity image        |
| symbol    |              | Mirrors the value of option `symbol` |
| style\* |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

::: warning This module is not supported on elvish shell. :::

### 選項

| Option                  | 預設                            | 說明                                                   |
| ----------------------- | ----------------------------- | ---------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "` | The format of the module                             |
| `symbol`                | `"✖"`                         | The symbol displayed on program error                |
| `not_executable_symbol` | `"🚫"`                         | The symbol displayed when file isn't executable      |
| `not_found_symbol`      | `"🔍"`                         | The symbol displayed when the command can't be found |
| `sigint_symbol`         | `"🧱"`                         | The symbol displayed on SIGINT (Ctrl + c)            |
| `signal_symbol`         | `"⚡"`                         | The symbol displayed on any signal                   |
| `style`                 | `"bold red"`                  | 這個模組的風格。                                             |
| `recognize_signal_code` | `true`                        | Enable signal mapping from exit code                 |
| `map_symbol`            | `false`                       | Enable symbols mapping from exit code                |
| `disabled`              | `true`                        | Disables the `status` module.                        |

### Variables

| 變數             | 範例      | 說明                                                                   |
| -------------- | ------- | -------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                    |
| int            | `127`   | The exit code of the last command                                    |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                  |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled      |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found         |
| symbol         |         | Mirrors the value of option `symbol`                                 |
| style\*      |         | Mirrors the value of option `style`                                  |

\*: This variable can only be used as a part of a style string

### 範例

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

By default the `swift` module shows the currently installed version of Swift. The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### 選項

| Option              | 預設                                   | 說明                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                       |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift |
| `detect_extensions` | `["swift"]`                          | Which extensions should trigger this moudle.     |
| `detect_files`      | `["Package.swift"]`                  | Which filenames should trigger this module.      |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.        |
| `style`             | `"bold 202"`                         | 這個模組的風格。                                         |
| `disabled`          | `false`                              | Disables the `swift` module.                     |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v5.2.4` | The version of `swift`               |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version.

::: tip

By default the terraform version is not shown, since this is slow for current versions of terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-version).

:::

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

### 選項

| Option              | 預設                                   | 說明                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace. |
| `detect_extensions` | `["tf", "hcl"]`                      | Which extensions should trigger this module.          |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.           |
| `detect_folders`    | `[".terraform"]`                     | Which folders should trigger this module.             |
| `style`             | `"bold 105"`                         | 這個模組的風格。                                              |
| `disabled`          | `false`                              | Disables the `terraform` module.                      |

### Variables

| 變數        | 範例         | 說明                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v0.12.24` | The version of `terraform`           |
| workspace | `default`  | The current terraform workspace      |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

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

## 時間

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 選項

| Option            | 預設                      | 說明                                                                                                                                 |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                  |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| 變數        | 範例         | 說明                                  |
| --------- | ---------- | ----------------------------------- |
| time      | `13:08:10` | The current time.                   |
| style\* |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 範例

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

- 目前使用者為 root
- 目前使用者並非登入時的使用者
- 使用者透過 SSH session 進行連線
- 變數 `show_always` 被設為 true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### 選項

| Option        | 預設                      | 說明                                    |
| ------------- | ----------------------- | ------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root. |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.    |
| `format`      | `"[$user]($style) in "` | The format for the module.            |
| `show_always` | `false`                 | Always shows the `username` module.   |
| `disabled`    | `false`                 | Disables the `username` module.       |

### Variables

| 變數      | 範例           | 說明                                                                                          |
| ------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style` | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`  | `"matchai"`  | The currently logged-in user ID.                                                            |

### 範例

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

The `vagrant` module shows the currently installed version of Vagrant. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Vagrantfile` file

### 選項

| Option              | 預設                                   | 說明                                                  |
| ------------------- | ------------------------------------ | --------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                          |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant. |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.        |
| `detect_files`      | `["Vagrantfile"]`                    | Which filenames should trigger this module.         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.           |
| `style`             | `"cyan bold"`                        | 這個模組的風格。                                            |
| `disabled`          | `false`                              | Disables the `Vagrant` module.                      |

### Variables

| 變數        | 範例               | 說明                                   |
| --------- | ---------------- | ------------------------------------ |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`             |
| symbol    |                  | Mirrors the value of option `symbol` |
| style\* |                  | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## Zig

By default the the `zig` module shows the currently installed version of Zig. The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### 選項

| Option              | 預設                                   | 說明                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig. |
| `style`             | `"bold yellow"`                      | 這個模組的風格。                                              |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                            |
| `disabled`          | `false`                              | Disables the `zig` module.                            |
| `detect_extensions` | `["zig"]`                            | Which extensions should trigger this module.          |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.             |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v0.6.0` | The version of `zig`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 範例

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

### 選項

| Option        | 預設                              | 說明                                                                                                                         |
| ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                  | 這個模組的風格。                                                                                                                   |
| `format`      | `"[$symbol($output )]($style)"` | The format for the module.                                                                                                 |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                             |

### Variables

| 變數        | 說明                                     |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Mirrors the value of option `symbol`   |
| style\* | Mirrors the value of option `style`    |

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

### 範例

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
