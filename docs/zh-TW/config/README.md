# 設定

::: tip

🔥 「設定」現在還在建置中。 許多新的設定選項會在之後的版本釋出。

:::

為了開始設定 Starship，請建立下右檔案： `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

所有關於 Starship 的設定都在這個 [TOML](https://github.com/toml-lang/toml) 檔案內：

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

你可以藉由修改 `STARSHIP_CONFIG`環境變數而改變預設的`starship.toml` 檔案位置

```sh
export STARSHIP_CONFIG=~/.starship
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### 術語

**模組 (Module)**： 提示字元中的一個元件，基於你的作業系統提供的背景資訊來提供訊息。 舉例來說，如果你現在的資料夾是一個 NodeJS 專案，"nodejs" 模組會顯示出現在安裝在你的電腦上的 NodeJS 版本。

**Variable**: Smaller sub-components that contains information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of NodeJS.

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
- `[⬢ $version](bold green)` will print a symbol `⬢` followed by the content of variable `version`, with bold text colored green.
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

- $
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

### 範例

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
format = "$all"

# Use custom format
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10
```

### Default Prompt Format

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. 預設如下：

```toml
format = "\n$all"

# Which is equivalent to
format = """

$username\
$hostname\
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
$php\
$purescript\
$python\
$ruby\
$rust\
$terraform\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$env_var\
$crystal\
$cmd_duration\
$custom\
$line_break\
$jobs\
$battery\
$time\
$character"""
```

## AWS

`aws` 模組顯示現在 AWS 的區域與概況。 這是根據 `AWS_REGION`、`AWS_DEFAULT_REGION` 與 `AWS_PROFILE` 環境變數及 `~/.aws/config` 檔案。

從 `AWS_VAULT`讀取而使用 [aws-vault](https://github.com/99designs/aws-vault) 這個設定檔

### 選項

| Option           | 預設                                                   | 說明                         |
| ---------------- | ---------------------------------------------------- | -------------------------- |
| `format`         | `"on [$symbol$profile(\\($region\\))]($style) "` | The format for the module. |
| `symbol`         | `"☁️ "`                                              | 顯示在目前 AWS 配置之前的符號。         |
| `region_aliases` |                                                      | 除了AWS名稱外，顯示區域別名表           |
| `style`          | `"bold yellow"`                                      | 這個模組的風格。                   |
| `disabled`       | `false`                                              | 停用 `AWS` 模組。               |

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
format = "on [$symbol$profile(\\($region\\))]($style) "
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

`battery` 模組顯示電池的電量以及現在的充電狀態。 這個模組只會在裝置的電量低於 10% 的時候看見。

### 選項

| Option               | 預設                                | 說明                         |
| -------------------- | --------------------------------- | -------------------------- |
| `full_symbol`        | `"•"`                             | 當電池充飽時顯示的符號。               |
| `charging_symbol`    | `"⇡"`                             | 當電池正在充電時顯示的符號。             |
| `discharging_symbol` | `"⇣"`                             | 當電池正在放電時顯示的符號。             |
| `format`             | `"[$symbol$percentage]($style) "` | The format for the module. |
| `display`            | [連結](#battery-display)            | 顯示的門檻與模組的風格。               |
| `disabled`           | `false`                           | 停用 `battery` 模組。           |

<details>
<summary>也有些針對不常見的電池狀態設定的選項。</summary>

| 變數               | 說明             |
| ---------------- | -------------- |
| `unknown_symbol` | 當電池狀態不明時顯示的符號。 |
| `empty_symbol`   | 當電池沒電時顯示的符號。   |

注意：電池指示會在電池狀態`不明`或`沒電`時隱藏起來，除非你在設定之中有特別指定選項。

</details>

### 範例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### 電池顯示

`display` 設定是用來定義甚麼時候電池指示會顯示出來 (threshold)，以及它長甚麼樣子 (style)。 如果沒有提供 `display`。 預設如下：

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### 選項

`display` 選項是一個下列表格的陣列。

| 變數          | 說明          |
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

`character` 模組在你的文字輸入處旁顯示一個字元 (通常是箭頭)。

這個字元會告訴你最後的指令是成功還是失敗。 It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change it's shape take a look at [this example](#with-custom-error-shape).

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

The `cmake` module shows the currently installed version of CMake if:

- The current directory contains a `CMakeLists.txt` file

### 選項

| Option     | 預設                                 | 說明                                           |
| ---------- | ---------------------------------- | -------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                   |
| `symbol`   | `"🛆 "`                             | The symbol used before the version of cmake. |
| `style`    | `"bold blue"`                      | 這個模組的風格。                                     |
| `disabled` | `false`                            | Disables the `cmake` module.                 |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v3.17.3` | The version of cmake                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

## 指令持續時間

`cmd_duration` 模組顯示最後一個指令執行所花費的時間。 這個模組只會在指令花費超過兩秒或是有設定 `min_time` 時，超過設定值時出現。

::: warning 不要在 Bash 中設置 DEBUG trap

如果你在 `bash` 中使用 Starship，不要在執行 `eval $(starship init $0)` 之後設置 `DEBUG` trap，不然這個模組**會**壞掉。

:::

想使用類似 preexec 功能的 Bash 使用者可以 [rcaloras 的 bash_preexec 框架](https://github.com/rcaloras/bash-preexec)。 只要在 `eval $(starship init $0)` 之前簡單地定義 `preexec_functions` 與 `precmd_functions` 兩個陣列，然後就可以照常進行。

### 選項

| Option              | 預設                            | 說明                                                    |
| ------------------- | ----------------------------- | ----------------------------------------------------- |
| `min_time`          | `2_000`                       | Shortest duration to show time for (in milliseconds). |
| `show_milliseconds` | `false`                       | 顯示時間除了以秒為單位外，亦以毫秒顯示                                   |
| `format`            | `"took [$duration]($style) "` | The format for the module.                            |
| `style`             | `"bold yellow"`               | 這個模組的風格。                                              |
| `disabled`          | `false`                       | 停用 `cmd_duration` 模組。                                 |

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

如果有設定 `$CONDA_DEFAULT_ENV` 時，`conda` 模組顯示現在 conda 的環境。

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### 選項

| Option              | 預設                                 | 說明                                                                                              |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                | 如果環境變數由所`conda create -p [path]`產生時，環境變數的資料夾需要截斷的數目。 `0` 表示不截斷 也請參考 [`directory`](#directory)模組 |
| `symbol`            | `"🅒 "`                             | 環境名稱前使用的符號。                                                                                     |
| `style`             | `"bold green"`                     | 這個模組的風格。                                                                                        |
| `format`            | `"[$symbol$environment]($style) "` | The format for the module.                                                                      |
| `disabled`          | `false`                            | 停用 `conda` 模組。                                                                                  |

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

`crystal` 模組顯示現在所安裝的Crystal版本 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中含有一個 `shard.yml` 檔案
- 現在資料夾中含有一個`.cr`檔案

### 選項

| Option     | 預設                                 | 說明                                                        |
| ---------- | ---------------------------------- | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`                             | The symbol used before displaying the version of crystal. |
| `style`    | `"bold red"`                       | 這個模組的風格。                                                  |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                                |
| `disabled` | `false`                            | Disables the `crystal` module.                            |

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

## 資料夾

`directory` 模組顯示到現在資料夾的路徑，並裁減到前三層資料夾。 你的資料夾也會被裁減到你所在的 git 儲存庫的根目錄。

如果正在使用 fish 風格的 pwd 選項，將不會隱藏被裁減的資料夾，而是會根據你在選項中設定的數字看到每一層資料夾的縮寫。

例如，給定一個右列的路徑 `~/Dev/Nix/nixpkgs/pkgs` 其中 `nixpkgs` 是儲存庫的根目錄，而且該選項被設定為 `1`。 你會看到 `~/D/N/nixpkgs/pkgs`，而在這個設定之前則是 `nixpkgs/pkgs`。

### 選項

| 變數                       | 預設                                              | 說明                                                    |
| ------------------------ | ----------------------------------------------- | ----------------------------------------------------- |
| `truncation_length`      | `3`                                             | 到達現在資料夾的路徑中，要被裁減掉的資料夾數目。                              |
| `truncate_to_repo`       | `true`                                          | 是否要裁減到你現在所在的 git 儲存庫的根目錄。                             |
| `format`                 | `"[$path]($style)[$lock_symbol]($lock_style) "` | The format for the module.                            |
| `style`                  | `"bold cyan"`                                   | 這個模組的風格。                                              |
| `disabled`               | `false`                                         | 停用 `directory` 模組。                                    |
| `read_only_symbol`       | `"🔒"`                                           | The symbol indicating current directory is read only. |
| `read_only_symbol_style` | `"red"`                                         | The style for the read only symbol.                   |

<details>
<summary>這個模組有些進階設定選項可以控制顯示資料夾。</summary>

| Advanced Option             | 預設     | 說明                                               |
| --------------------------- | ------ | ------------------------------------------------ |
| `substitutions`             |        | A table of substitutions to be made to the path. |
| `fish_style_pwd_dir_length` | `0`    | 當使用 fish shell 的 pwd 路徑邏輯時使用的字元數量。               |
| `use_logical_path`          | `true` | 顯示 shell (`PWD`) 提供的邏輯路徑，而不是 OS 的路徑。             |

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
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### 選項

| Option            | 預設                                 | 說明                                                                                      |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | The format for the module.                                                              |
| `symbol`          | `"🐳 "`                             | The symbol used before displaying the Docker context.                                   |
| `style`           | `"blue bold"`                      | 這個模組的風格。                                                                                |
| `only_with_files` | `false`                            | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `disabled`        | `true`                             | Disables the `docker_context` module.                                                   |

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

`dotnet` 模組顯示現在資料夾使用的 .NET Core SDK 的版本。 如果這個資料夾已經選定一個 SDK，則顯示這個 SDK 的版本。 如果沒有的話，則顯示最新安裝的 SDK 版本。

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

這個模組內部是使用它自己的機制來偵測版本。 一般來說這個模組有 `dotnet --version` 的兩倍快，但是它可能會在你的 .NET 專案有不尋常的資料夾結構時顯示不正確的版本。 如果精確度比速度更重要的話，你可以藉由設定模組中的 `heuristic = false` 選項來停用這個功能。

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### 選項

| Option      | 預設                                       | 說明                           |
| ----------- | ---------------------------------------- | ---------------------------- |
| `format`    | `"v[$symbol$version( 🎯 $tfm)]($style) "` | The format for the module.   |
| `symbol`    | `"•NET "`                                | 在顯示 dotnet 版本之前用的符號。         |
| `heuristic` | `true`                                   | 使用更快速的版本偵測法來保持 starship 的速度。 |
| `style`     | `"bold blue"`                            | 這個模組的風格。                     |
| `disabled`  | `false`                                  | 停用 `dotnet` 模組。              |

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

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `mix.exs` 檔案.

### 選項

| Option     | 預設                                                            | 說明                                                              |
| ---------- | ------------------------------------------------------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                        | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"`                                               | 這個模組的風格。                                                        |
| `format`   | `"via [$symbol$version \\(OTP $otp_version\\)]($style) "` | The format for the module elixir.                               |
| `disabled` | `false`                                                       | Disables the `elixir` module.                                   |

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

The `elm` module shows the currently installed version of Elm. 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `elm.json` 檔案
- 現在資料夾中包含一個 `elm-package.json` 檔案
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### 選項

| Option     | 預設                                 | 說明                                              |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🌳 "`                             | A format string representing the symbol of Elm. |
| `style`    | `"cyan bold"`                      | 這個模組的風格。                                        |
| `disabled` | `false`                            | Disables the `elm` module.                      |

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

`env_var`模組顯示一個選擇的環境變數的現在數值。 這個模組只在下列條件其中之一達到時顯示：

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
| env_value | `Windows NT` (if *variable* would be `$OS`) | The environment value of option `variable` |
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

The `erlang` module shows the currently installed version of Erlang/OTP. 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `rebar.config` 檔案.
- 現在資料夾中包含一個 `erlang.mk` 檔案.

### 選項

| Option     | 預設                                 | 說明                                                       |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`                             | The symbol used before displaying the version of erlang. |
| `style`    | `"bold red"`                       | 這個模組的風格。                                                 |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                               |
| `disabled` | `false`                            | Disables the `erlang` module.                            |

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

## Git 分支

`git_branch` 模組顯示現在的資料夾中使用中的儲存庫的分支。

### 選項

| Option              | 預設                               | 說明                                                                               |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------- |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.  Use `"$branch"` to refer to the current branch name. |
| `symbol`            | `" "`                           | A format string representing the symbol of git branch.                           |
| `style`             | `"bold purple"`                  | 這個模組的風格。                                                                         |
| `truncation_length` | `2^63 - 1`                       | Truncates a git branch to X graphemes.                                           |
| `truncation_symbol` | `"…"`                            | 用來指示分支名稱被縮減的符號。 You can use `""` for no symbol.                                  |
| `disabled`          | `false`                          | 停用 `git_branch` 模組。                                                              |

### Variables

| 變數        | 範例       | 說明                                                                                                   |
| --------- | -------- | ---------------------------------------------------------------------------------------------------- |
| branch    | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached HEAD). |
| symbol    |          | Mirrors the value of option `symbol`                                                                 |
| style\* |          | Mirrors the value of option `style`                                                                  |

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

The `git_commit` module shows the current commit hash of the repo in your current directory.

### 選項

| Option               | 預設                             | 說明                                                    |
| -------------------- | ------------------------------ | ----------------------------------------------------- |
| `commit_hash_length` | `7`                            | The length of the displayed git commit hash.          |
| `format`             | `"[\\($hash\\)]($style) "` | The format for the module.                            |
| `style`              | `"bold green"`                 | 這個模組的風格。                                              |
| `only_detached`      | `true`                         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`                        | Disables the `git_commit` module.                     |

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
```

## Git State

`git_state` 模組會顯示在 git 儲存庫中的資料夾內，以及會在有作業正在進行時顯示，像是：_REBASING_、_BISECTING_ 等等。 如果有進展的資訊 (像是 REBASING 3/10)，也會一併顯示出來。

### 選項

| Option         | 預設                                                                  | 說明                                                                                      |
| -------------- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                        | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                         | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                       | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                                  | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                       | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                              | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                       | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                     | 這個模組的風格。                                                                                |
| `format`       | `"[\\($state( $progress_current/$progress_total)\\)]($style) "` | The format for the module.                                                              |
| `disabled`     | `false`                                                             | 停用 `git_state` 模組。                                                                      |

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
format = "[\\($state( $progress_current of $progress_total)\\)]($style) "
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Status

`git_status` 模組顯示用來表示現在資料夾之中儲存庫狀態的符號。

### 選項

| Option            | 預設                                              | 說明                                  |
| ----------------- | ----------------------------------------------- | ----------------------------------- |
| `format`          | "([\[$all_status$ahead_behind\]]($style) )" | The default format for `git_status` |
| `conflicted`      | `"="`                                           | 這個分支有合併衝突。                          |
| `ahead`           | `"⇡"`                                           | The format of `ahead`               |
| `behind`          | `"⇣"`                                           | The format of `behind`              |
| `diverged`        | `"⇕"`                                           | The format of `diverged`            |
| `untracked`       | `"?"`                                           | The format of `untracked`           |
| `stashed`         | `"$"`                                           | The format of `stashed`             |
| `modified`        | `"!"`                                           | The format of `modified`            |
| `staged`          | `"+"`                                           | The format of `staged`              |
| `renamed`         | `"»"`                                           | The format of `renamed`             |
| `deleted`         | `"✘"`                                           | The format of `deleted`             |
| `show_sync_count` | `false`                                         | 顯示超前/落後追蹤的分支的數量。                    |
| `style`           | `"bold red"`                                    | 這個模組的風格。                            |
| `disabled`        | `false`                                         | 停用 `git_status` 模組。                 |

### Variables

The following variables can be used in `format`:

| 變數             | 說明                                                                                            |
| -------------- | --------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                   |
| `ahead_behind` | Displays `diverged` `ahead` or `behind` format string based on the current status of the repo |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                   |
| `untracked`    | Displays `untracked`  when there are untracked files in the working directory.                |
| `stashed`      | Displays `stashed`    when a stash exists for the local repository.                           |
| `modified`     | Displays `modified`   when there are file modifications in the working directory.             |
| `staged`       | Displays `staged`     when a new file has been added to the staging area.                     |
| `renamed`      | Displays `renamed`    when a renamed file has been added to the staging area.                 |
| `deleted`      | Displays `deleted`    when a file's deletion has been added to the staging area.              |
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

## Golang

`golang` 模組顯示現在安裝的 Golang 版本。 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中含有一個 `go.mod` 檔案
- 現在資料夾中含有一個 `go.sum` 檔案
- 現在資料夾中含有一個 `glide.yaml` 檔案
- 現在資料夾中含有一個 `Gopkg.yml` 檔案
- 現在資料夾中含有一個 `Gopkg.lock` 檔案
- The current directory contains a `.go-version` file
- 現在資料夾中含有一個 `Godeps` 資料夾
- 現在資料夾中含有一個檔案具有 `.go` 副檔名

### 選項

| Option     | 預設                                 | 說明                                             |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                     |
| `symbol`   | `"🐹 "`                             | A format string representing the symbol of Go. |
| `style`    | `"bold cyan"`                      | 這個模組的風格。                                       |
| `disabled` | `false`                            | 停用 `golang` 模組。                                |

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

The `helm` module shows the currently installed version of Helm. 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `helmfile.yaml` 檔案
- The current directory contains a `Chart.yaml` file

### 選項

| Option     | 預設                                 | 說明                                               |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"⎈ "`                             | A format string representing the symbol of Helm. |
| `style`    | `"bold white"`                     | 這個模組的風格。                                         |
| `disabled` | `false`                            | Disables the `helm` module.                      |

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

`hostname` 模組顯示系統的主機名稱。

### 選項

| Option     | 預設                          | 說明                                                         |
| ---------- | --------------------------- | ---------------------------------------------------------- |
| `ssh_only` | `true`                      | 只在連接到一個 SSH session 時顯示主機名稱。                               |
| `trim_at`  | `"."`                       | 擷取出主機名稱的斷點，以第一個符合的為準。 `"."` 會讓它停在第一個點的符號。 `""` 會停用任何的截斷功能。 |
| `format`   | `"on [$hostname]($style) "` | The format for the module.                                 |
| `style`    | `"bold dimmed green"`       | 這個模組的風格。                                                   |
| `disabled` | `false`                     | 停用 `hostname` 模組。                                          |

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

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

`java` 模組顯示現在安裝的 Java 版本。 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### 選項

| Option     | 預設                                     | 說明                                              |
| ---------- | -------------------------------------- | ----------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | The format for the module.                      |
| `symbol`   | `"☕ "`                                 | A format string representing the symbol of Java |
| `style`    | `"red dimmed"`                         | 這個模組的風格。                                        |
| `disabled` | `false`                                | 停用 `java` 模組。                                   |

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

`jobs` 模組顯示現在正在執行中的工作。 這個模組只會在有背景工作正在執行時顯示。 這個模組會在工作數量超過一個，或者有設定 `threshold` 時且數量超過設定值時，顯示工作的數量。

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

The `julia` module shows the currently installed version of Julia. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### 選項

| Option     | 預設                                 | 說明                                                |
| ---------- | ---------------------------------- | ------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                        |
| `symbol`   | `"ஃ "`                             | A format string representing the symbol of Julia. |
| `style`    | `"bold purple"`                    | 這個模組的風格。                                          |
| `disabled` | `false`                            | Disables the `julia` module.                      |

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

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| Option                  | 預設                                                       | 說明                                                                    |
| ----------------------- | -------------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`                | `"☸ "`                                                   | A format string representing the symbol displayed before the Cluster. |
| `format`                | `"on [$symbol$context( \\($namespace\\))]($style) "` | The format for the module.                                            |
| `style`                 | `"cyan bold"`                                            | 這個模組的風格。                                                              |
| `namespace_spaceholder` | `none`                                                   | The value to display if no namespace was found.                       |
| `context_aliases`       |                                                          | Table of context aliases to display.                                  |
| `disabled`              | `true`                                                   | Disables the `kubernetes` module.                                     |

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
format = "on [⛵ $context \\($namespace\\)](dimmed green) "
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## 換行

`line_break` 模組將提示字元分成兩行。

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

## 記憶體使用量

`memory_usage` 模組顯示現在系統記憶體與 swap 的使用量。

預設 swap 使用量會在系統總 swap 使用量不為 0 時顯示出來。

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

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

| 變數            | 範例            | 說明                                                                 |
| ------------- | ------------- | ------------------------------------------------------------------ |
| ram           | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct       | `48%`         | The percentage of the current system memory.                       |
| swap\**     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\** | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol        | `🐏`           | Mirrors the value of option `symbol`                               |
| style\*     |               | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

### 範例

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

The `nim` module shows the currently installed version of Nim. 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `nim.cfg` 檔案
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### 選項

| Option     | 預設                                 | 說明                                                    |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module                             |
| `symbol`   | `"👑 "`                             | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"`                    | 這個模組的風格。                                              |
| `disabled` | `false`                            | Disables the `nim` module.                            |

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

`nix_shell` 模組顯示 nix-shell 環境。 這個模組會在 nix-shell 環境中顯示。

### 選項

| Option       | 預設                                                 | 說明                                                    |
| ------------ | -------------------------------------------------- | ----------------------------------------------------- |
| `format`     | `"via [$symbol$state( \\($name\\))]($style) "` | The format for the module.                            |
| `symbol`     | `"❄️  "`                                           | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                      | 這個模組的風格。                                              |
| `impure_msg` | `"impure"`                                         | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                           | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                            | 停用 `nix_shell` 模組。                                    |

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
format = "via [☃️ $state( \\($name\\))](bold blue) "
```

## NodeJS

`nodejs` 模組顯示現在安裝的 NodeJS 版本。 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `package.json` 檔案
- The current directory contains a `.node-version` file
- 現在資料夾中包含一個 `node_modules` 資料夾
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### 選項

| Option     | 預設                                 | 說明                                                 |
| ---------- | ---------------------------------- | -------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                         |
| `symbol`   | `"⬢ "`                             | A format string representing the symbol of NodeJS. |
| `style`    | `"bold green"`                     | 這個模組的風格。                                           |
| `disabled` | `false`                            | 停用 `nodejs` 模組。                                    |

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

## 套件版本

The `package` 模組在現在資料夾是一個套件的儲藏庫時出現，並顯示他的現在版本。 The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** – `npm` 套件的版本是從現在資料夾中的 `package.json` 之中擷取出來的
- **cargo** – `cargo` 套件的版本是從現在資料夾中的 `Cargo.toml` 之中擷取出來的
- **poetry** – `poetry` 套件的版本是從現在資料夾中的 `pyproject.toml` 之中擷取出來的
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠️ 顯示出來的版本是從你的現在資料夾之中擷取出來的，並非從套件管理員取得。

### 選項

| Option            | 預設                                 | 說明                                                        |
| ----------------- | ---------------------------------- | --------------------------------------------------------- |
| `format`          | `"via [$symbol$version]($style) "` | The format for the module.                                |
| `symbol`          | `"📦 "`                             | 顯示在套件的版本之前的符號。                                            |
| `style`           | `"bold 208"`                       | 這個模組的風格。                                                  |
| `display_private` | `false`                            | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                            | 停用 `package` 模組。                                          |

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

## OCaml

The `ocaml` module shows the currently installed version of OCaml. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### 選項

| Option     | 預設                                 | 說明                                                      |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                       |
| `symbol`   | `"🐫 "`                             | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"`                    | 這個模組的風格。                                                |
| `disabled` | `false`                            | Disables the `ocaml` module.                            |

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

## PHP

The `php` module shows the currently installed version of PHP. 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `composer.json` 檔案
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### 選項

| Option     | 預設                                 | 說明                                                    |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `symbol`   | `"🐘 "`                             | The symbol used before displaying the version of PHP. |
| `style`    | `"147 bold"`                       | 這個模組的風格。                                              |
| `disabled` | `false`                            | Disables the `php` module.                            |

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

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

這個模組在下列其中一個條件達成時顯示：

- 目前資料夾中有一個 `.python-version` 檔案
- 目前資料夾中有一個 `requirements.txt` 檔案
- 目前資料夾中有一個 `pyproject.toml` 檔案
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- 目前資料夾中有一個 `Pipfile` 檔案
- 目前資料夾中有一個 `tox.ini` 檔案
- 現在資料夾中包含一個 `setup.py` 檔案
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### 選項

| Option               | 預設                                                             | 說明                                                                         |
| -------------------- | -------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `format`             | `"via [${symbol}${version}( \\($virtualenv\\))]($style) "` | The format for the module.                                                 |
| `symbol`             | `"🐍 "`                                                         | A format string representing the symbol of Python                          |
| `style`              | `"yellow bold"`                                                | 這個模組的風格。                                                                   |
| `pyenv_version_name` | `false`                                                        | 使用 pyenv 取得 Python 的版本。                                                    |
| `scan_for_pyfiles`   | `true`                                                         | If false, Python files in the current directory will not show this module. |
| `disabled`           | `false`                                                        | 停用 `python` 模組。                                                            |

### Variables

| 變數         | 範例              | 說明                                   |
| ---------- | --------------- | ------------------------------------ |
| version    | `"v3.8.1"`      | The version of `python`              |
| symbol     | `"🐍 "`          | Mirrors the value of option `symbol` |
| style      | `"yellow bold"` | Mirrors the value of option `style`  |
| virtualenv | `"venv"`        | The current `virtualenv` name        |

<details>
<summary>This module has some advanced configuration options.</summary>

| 變數              | 預設       | 說明                                                                            |
| --------------- | -------- | ----------------------------------------------------------------------------- |
| `python_binary` | `python` | Configures the python binary that Starship executes when getting the version. |

The `python_binary` variable changes the binary that Starship executes to get the version of Python, it doesn't change the arguments that are used.

```toml
# ~/.config/starship.toml

[python]
python_binary = "python3"
```

</details>

### 範例

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

`ruby` 模組顯示現在安裝的 Ruby 版本。 這個模組在下列其中一個條件達成時顯示：

- 目前資料夾中有一個 `Gemfile` 檔案
- The current directory contains a `.ruby-version` file
- 目前資料夾中有一個 `.rb` 檔案

### 選項

| Option     | 預設                                 | 說明                                               |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"💎 "`                             | A format string representing the symbol of Ruby. |
| `style`    | `"bold red"`                       | 這個模組的風格。                                         |
| `disabled` | `false`                            | 停用 `ruby` 模組。                                    |

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

`rust` 模組顯示現在安裝的 Rust 版本。 這個模組在下列其中一個條件達成時顯示：

- 目前資料夾中有一個 `Cargo.toml` 檔案
- 現在資料夾中包含一個檔案具有 `.rs` 副檔名

### 選項

| Option     | 預設                                 | 說明                                              |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🦀 "`                             | A format string representing the symbol of Rust |
| `style`    | `"bold red"`                       | 這個模組的風格。                                        |
| `disabled` | `false`                            | 停用 `rust` 模組。                                   |

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

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### 選項

| Option     | 預設                                   | 說明                                               |
| ---------- | ------------------------------------ | ------------------------------------------------ |
| `format`   | `"[$symbol\\[$env\\]]($style) "` | The format for the module.                       |
| `symbol`   | `""`                                 | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`                 | 這個模組的風格。                                         |
| `disabled` | `false`                              | Disables the `singularity` module.               |

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
format = "[📦 \\[$env\\]]($style) "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-version). 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### 選項

| Option     | 預設                                   | 說明                                                    |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`   | `"💠 "`                               | A format string shown before the terraform workspace. |
| `style`    | `"bold 105"`                         | 這個模組的風格。                                              |
| `disabled` | `false`                              | Disables the `terraform` module.                      |

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

`time` 模組顯示目前的**當地**時間. `format` 設定值被 [`chrono`](https://crates.io/crates/chrono) crate 用來控制時間如何顯示。 請看 [chrono 的 strftime 文件](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)來了解有那些選項可以使用。

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| Option            | 預設                      | 說明                                                                                                    |
| ----------------- | ----------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                     |
| `use_12hr`        | `false`                 | 啟用 12 小時格式。                                                                                           |
| `time_format`     | 請看下列                    | 用來顯示時間的 [chrono 格式字串](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)。                |
| `style`           | `"bold yellow"`         | 這個模組的時間的風格。                                                                                           |
| `utc_time_offset` | `"local"`               | 設定相對於 UTC 的時差。 Range from -24 &lt; x &lt; 24. 允許使用浮點數來表示 30/45 分鐘時差的時區。                   |
| `disabled`        | `true`                  | 停用 `time` 模組。                                                                                         |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. 不然的話，它會被預設為 `"%T"`。 Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| 變數        | 範例         | 說明                                  |
| --------- | ---------- | ----------------------------------- |
| 時間        | `13:08:10` | The current time.                   |
| style\* |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[\\[ $time \\]]($style) "
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## 使用者名稱

`username` 模組顯示現在使用中的使用者名稱。 這個模組在下列其中一個條件達成時顯示：

- 目前使用者為 root
- 目前使用者並非登入時的使用者
- 使用者透過 SSH session 進行連線
- 變數 `show_always` 被設為 true

### 選項

| Option        | 預設                       | 說明                         |
| ------------- | ------------------------ | -------------------------- |
| `style_root`  | `"bold red"`             | 使用者為 root 時使用的風格。          |
| `style_user`  | `"bold yellow"`          | 非 root 使用者時使用的風格。          |
| `format`      | `"via [$user]($style) "` | The format for the module. |
| `show_always` | `false`                  | 總是顯示 `username` 模組。        |
| `disabled`    | `false`                  | 停用 `username` 模組。          |

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

## Zig

The `zig` module shows the currently installed version of Zig. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `.zig` file

### 選項

| Option     | 預設                                 | 說明                                                    |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`                             | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"`                    | 這個模組的風格。                                              |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `disabled` | `false`                            | Disables the `zig` module.                            |

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

The order in which custom modules are shown can be individually set by setting `custom.foo` in `prompt_order`. By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

### 選項

| Option        | 預設                            | 說明                                                                                                                         |
| ------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                               | The command whose output should be printed.                                                                                |
| `when`        |                               | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                               | [See below](#custom-command-shell)                                                                                         |
| `說明`          | `"<custom module>"`     | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                          | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                          | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                          | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                          | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                | 這個模組的風格。                                                                                                                   |
| `format`      | `"[$symbol$output]($style) "` | The format for the module.                                                                                                 |
| `disabled`    | `false`                       | Disables this `custom` module.                                                                                             |

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
prefix = " transcending "

[custom.time]
command = "time /T"
files = ["*.pst"]
prefix = "transcending "
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `spago.dhall` 檔案
- The current directory contains a \*.purs files

### 選項

| Option     | 預設                                 | 說明                                                           |
| ---------- | ---------------------------------- | ------------------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                                   |
| `symbol`   | `"<=> "`                     | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"`                     | 這個模組的風格。                                                     |
| `disabled` | `false`                            | Disables the `purescript` module.                            |

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
