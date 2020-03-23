# 設定

::: tip

🔥 「設定」現在還在建置中。 許多新的設定選項會在之後的版本釋出。

:::

為了開始設定 Starship，請建立下右檔案： `~/.config/starship.toml`.

```sh
$ mkdir -p ~/.config && touch ~/.config/starship.toml
```

所有關於 Starship 的設定都在這個 [TOML](https://github.com/toml-lang/toml) 檔案內：

```toml
# 不要在提示字元的開頭換行
add_newline = false

# 把提示字元中的 "❯" 符號換成 "➜"
[character]      # 我們正在設定的模組叫做 "character"
symbol = "➜"     #  設定 "symbol" 區段為 "➜"

# 關閉 package 模組，把它完全從提示字元藏起來
[package]
disabled = true
```

You can change default `starship.toml` file location with `STARSHIP_CONFIG` environment variable:
```sh
export STARSHIP_CONFIG=~/.starship
```

### 術語

**模組 (Module)**： 提示字元中的一個元件，基於你的作業系統提供的背景資訊來提供訊息。 舉例來說，如果你現在的資料夾是一個 NodeJS 專案，"nodejs" 模組會顯示出現在安裝在你的電腦上的 NodeJS 版本。

**區段 (Segment)**： 組成一個模組的子元件。 舉例來說，"nodejs" 模組內的 "symbol" 區段包含了一個會顯示在版本編號之前的字元 (預設是 ⬢)。

這是一個 node 模組的表示法。 在下面的例子裡，"symbol" 跟 "version" 都是模組內的區段。 每個模組也包含了使用預設終端機顏色的一個前綴 (prefix) 跟一個後綴 (suffix)。

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### 風格字串

Starship 內大多數的模組允許你設定他們的顯示風格。 這要透過一個條目 (通常叫做 `style`)，這個條目使用一個字串來進行設定。 這裡給幾個風格字串的例子，以及這些字串的功用。 對於完整語法的詳細說明，請參照 [進階設定指南](/advanced-config/)。

- `"fg:green bg:blue"` 在一個藍色背景上設定綠色文字
- `"bg:blue fg:bright-green"` 在一個藍色背景上設定亮綠色文字
- `"bold fg:27"` 設定具有 [ANSI 顏色](https://i.stack.imgur.com/KTSQa.png) 27 號的粗體文字
- `"underline bg:#bf5700"` 在一個燒橙色背景上設定有底線的文字
- `"bold italic fg:purple"` 設定粗體、斜體且紫色的文字
- `""` 明確地關閉所有風格

注意風格產出的樣子取決於你的終端機模擬器。 例如，有些終端機模擬器會提升顏色的亮度而不是讓文字變粗體，而且有些色彩主題對一般與加亮顏色使用的是相同色碼。 除此之外，為了要有斜體字，你的終端機一定要支援斜體。

## 提示字元

以下是針對提示字元內容的設定。

### 選項

| 變數             | 預設                          | 說明                                                    |
| -------------- | --------------------------- | ----------------------------------------------------- |
| `add_newline`  | `true`                      | 在提示字元前面加上換行字元。                                        |
| `prompt_order` | [連結](#default-prompt-order) | 調整各個提示字元模組的顯示順序。                                      |
| `scan_timeout` | `30`                        | Timeout for starship to scan files (in milliseconds). |

### 範例

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
add_newline = false
# Overwrite a default_prompt_order and  use custom prompt_order
prompt_order=["rust","line_break","package","line_break","character"]
# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10
```

### 預設的提示字元順序

預設 `prompt_order` 是用來在 `prompt_order` 為空時或者沒有提供時定義模組顯示在提示字元的順序。 預設如下：

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
    "elixir",
    "elm",
    "golang",
    "haskell",
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
    "crystal",
    "cmd_duration",
    "line_break",
    "jobs",
    "battery",
    "time",
    "character",
]
```

## AWS

`aws` 模組顯示現在 AWS 的區域與概況。 這是根據 `AWS_REGION`、`AWS_DEFAULT_REGION` 與 `AWS_PROFILE` 環境變數及 `~/.aws/config` 檔案。

### 選項

| 變數                | 預設              | 說明                                                                          |
| ----------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`          | `"☁️ "`         | 顯示在目前 AWS 配置之前的符號。                                                          |
| `displayed_items` | `all`           | Choose which item to display. Possible values: [`all`, `profile`, `region`] |
| `region_aliases`  |                 | Table of region aliases to display in addition to the AWS name.             |
| `style`           | `"bold yellow"` | 這個模組的風格。                                                                    |
| `disabled`        | `false`         | 停用 `AWS` 模組。                                                                |

### 範例

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

## 電池

`battery` 模組顯示電池的電量以及現在的充電狀態。 這個模組只會在裝置的電量低於 10% 的時候看見。

### 選項

| 變數                   | 預設                     | 說明               |
| -------------------- | ---------------------- | ---------------- |
| `full_symbol`        | `"•"`                  | 當電池充飽時顯示的符號。     |
| `charging_symbol`    | `"⇡"`                  | 當電池正在充電時顯示的符號。   |
| `discharging_symbol` | `"⇣"`                  | 當電池正在放電時顯示的符號。   |
| `display`            | [連結](#battery-display) | 顯示的門檻與模組的風格。     |
| `disabled`           | `false`                | 停用 `battery` 模組。 |

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

這個字元會告訴你最後的指令是成功還是失敗。 他會用兩種方式告訴你：改變他的顏色 (紅色/綠色) 或是改變他的形狀 (❯/✖)。 後者只會在 `use_symbol_for_status` 被設定為 `true` 時出現。

### 選項

| 變數                      | 預設             | 說明                                        |
| ----------------------- | -------------- | ----------------------------------------- |
| `symbol`                | `"❯"`          | 使用在提示字元文字輸入處前的符號。                         |
| `error_symbol`          | `"✖"`          | 如果前一個指令失敗時，使用在文字輸入處前的符號。                  |
| `use_symbol_for_status` | `false`        | 是否透過改變符號來提示錯誤狀態。                          |
| `vicmd_symbol`          | `"❮"`          | 如果 shell 正在 vim 正常模式內，在提示字元的文字輸入處前的使用的符號。 |
| `style_success`         | `"bold green"` | 最後的指令成功時使用的風格。                            |
| `style_failure`         | `"bold red"`   | 最後的指令失敗時使用的風格。                            |
| `disabled`              | `false`        | 停用 `character` 模組。                        |

### 範例

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## 指令持續時間

`cmd_duration` 模組顯示最後一個指令執行所花費的時間。 這個模組只會在指令花費超過兩秒或是有設定 `min_time` 時，超過設定值時出現。

::: warning 不要在 Bash 中設置 DEBUG trap

如果你在 `bash` 中使用 Starship，不要在執行 `eval $(starship init $0)` 之後設置 `DEBUG` trap，不然這個模組**會**壞掉。

:::

想使用類似 preexec 功能的 Bash 使用者可以 [rcaloras 的 bash_preexec 框架](https://github.com/rcaloras/bash-preexec)。 只要在 `eval $(starship init $0)` 之前簡單地定義 `preexec_functions` 與 `precmd_functions` 兩個陣列，然後就可以照常進行。

### 選項

| 變數                  | 預設              | 說明                                                         |
| ------------------- | --------------- | ---------------------------------------------------------- |
| `min_time`          | `2_000`         | Shortest duration to show time for (in milliseconds).      |
| `show_milliseconds` | `false`         | Show milliseconds in addition to seconds for the duration. |
| `prefix`            | `took`          | 在指令持續時間正前方顯示的前綴。                                           |
| `style`             | `"bold yellow"` | 這個模組的風格。                                                   |
| `disabled`          | `false`         | 停用 `cmd_duration` 模組。                                      |

### 範例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
prefix = "underwent "
```

## Conda

如果有設定 `$CONDA_DEFAULT_ENV` 時，`conda` 模組顯示現在 conda 的環境。

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### 選項

| 變數                  | 預設             | 說明                                                                                                                                                                                                          |
| ------------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`            | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `"C "`         | 環境名稱前使用的符號。                                                                                                                                                                                                 |
| `style`             | `"bold green"` | 這個模組的風格。                                                                                                                                                                                                    |
| `disabled`          | `false`        | 停用 `conda` 模組。                                                                                                                                                                                              |

### 範例

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### 選項

| 變數         | 預設           | 說明                                                        |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`       | The symbol used before displaying the version of crystal. |
| `style`    | `"bold red"` | 這個模組的風格。                                                  |
| `disabled` | `false`      | Disables the `crystal` module.                            |

### 範例

```toml
# ~/.config/starship.toml

[crystal]
symbol = "✨ "
style = "bold blue"
```

## Directory

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### 選項

| 變數                  | 預設            | 說明                                                                               |
| ------------------- | ------------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`           | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`  | `true`        | Whether or not to truncate to the root of the git repo that you're currently in. |
| `prefix`            | `"in "`       | Prefix to display immediately before the directory.                              |
| `style`             | `"bold cyan"` | 這個模組的風格。                                                                         |
| `disabled`          | `false`       | Disables the `directory` module.                                                 |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| 變數                          | 預設     | 說明                                                                                       |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`    | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true` | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### 範例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one of the following files are present in the current directory: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj`. You'll also need the .NET Core command-line tools installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

### 選項

| 變數          | 預設            | 說明                                                       |
| ----------- | ------------- | -------------------------------------------------------- |
| `symbol`    | `"•NET "`     | The symbol used before displaying the version of dotnet. |
| `heuristic` | `true`        | Use faster version detection to keep starship snappy.    |
| `style`     | `"bold blue"` | 這個模組的風格。                                                 |
| `disabled`  | `false`       | Disables the `dotnet` module.                            |

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

- The current directory contains a `mix.exs` file.

### 選項

| 變數         | 預設      | 說明                                                              |
| ---------- | ------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`  | The symbol used before displaying the version of Elixir/Erlang. |
| `disabled` | `false` | Disables the `elixir` module.                                   |

### 範例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### 選項

| 變數         | 預設            | 說明                                                    |
| ---------- | ------------- | ----------------------------------------------------- |
| `symbol`   | `"🌳 "`        | The symbol used before displaying the version of Elm. |
| `style`    | `"bold cyan"` | 這個模組的風格。                                              |
| `disabled` | `false`       | Disables the `elm` module.                            |


### 範例

```toml
# ~/.config/starship.toml

[elm]
symbol = " "
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### 選項

| 變數         | 預設               | 說明                                                                           |
| ---------- | ---------------- | ---------------------------------------------------------------------------- |
| `symbol`   |                  | The symbol used before displaying the variable value.                        |
| `variable` |                  | The environment variable to be displayed.                                    |
| `default`  |                  | The default value to be displayed when the selected variable is not defined. |
| `prefix`   | `""`             | Prefix to display immediately before the variable value.                     |
| `suffix`   | `""`             | Suffix to display immediately after the variable value.                      |
| `style`    | `"dimmed black"` | 這個模組的風格。                                                                     |
| `disabled` | `false`          | Disables the `env_var` module.                                               |

### 範例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### 選項

| 變數                  | 預設              | 說明                                                                                    |
| ------------------- | --------------- | ------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the branch name of the repo in your current directory.         |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                 |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol |
| `style`             | `"bold purple"` | 這個模組的風格。                                                                              |
| `disabled`          | `false`         | Disables the `git_branch` module.                                                     |

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

| 變數                   | 預設             | 說明                                                    |
| -------------------- | -------------- | ----------------------------------------------------- |
| `commit_hash_length` | `7`            | The length of the displayed git commit hash.          |
| `prefix`             | `"("`          | Prefix to display immediately before git commit.      |
| `suffix`             | `")"`          | Suffix to display immediately after git commit.       |
| `style`              | `"bold green"` | 這個模組的風格。                                              |
| `only_detached`      | `true`         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`        | Disables the `git_commit` module.                     |

### 範例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### 選項

| 變數                 | 預設                 | 說明                                                                                                               |
| ------------------ | ------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `rebase`           | `"REBASING"`       | The text displayed when a `rebase` is in progress.                                                               |
| `merge`            | `"MERGING"`        | The text displayed when a `merge` is in progress.                                                                |
| `revert`           | `"REVERTING"`      | The text displayed when a `revert` is in progress.                                                               |
| `cherry_pick`      | `"CHERRY-PICKING"` | The text displayed when a `cherry-pick` is in progress.                                                          |
| `bisect`           | `"BISECTING"`      | The text displayed when a `bisect` is in progress.                                                               |
| `am`               | `"AM"`             | The text displayed when an `apply-mailbox` (`git am`) is in progress.                                            |
| `am_or_rebase`     | `"AM/REBASE"`      | The text displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.                                 |
| `progress_divider` | `"/"`              | The symbol or text which will separate the current and total progress amounts. (e.g., `" of "`, for `"3 of 10"`) |
| `style`            | `"bold yellow"`    | 這個模組的風格。                                                                                                         |
| `disabled`         | `false`            | Disables the `git_state` module.                                                                                 |

### 範例

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### 選項

| 變數                 | 預設                       | 說明                                                      |
| ------------------ | ------------------------ | ------------------------------------------------------- |
| `conflicted`       | `"="`                    | This branch has merge conflicts.                        |
| `conflicted_count` | [連結](#git-status-counts) | Show and style the number of conflicts.                 |
| `ahead`            | `"⇡"`                    | This branch is ahead of the branch being tracked.       |
| `behind`           | `"⇣"`                    | This branch is behind of the branch being tracked.      |
| `diverged`         | `"⇕"`                    | This branch has diverged from the branch being tracked. |
| `untracked`        | `"?"`                    | There are untracked files in the working directory.     |
| `untracked_count`  | [連結](#git-status-counts) | Show and style the number of untracked files.           |
| `stashed`          | `"$"`                    | A stash exists for the local repository.                |
| `stashed_count`    | [連結](#git-status-counts) | Show and style the number of stashes.                   |
| `modified`         | `"!"`                    | There are file modifications in the working directory.  |
| `modified_count`   | [連結](#git-status-counts) | Show and style the number of modified files.            |
| `staged`           | `"+"`                    | A new file has been added to the staging area.          |
| `staged_count`     | [連結](#git-status-counts) | Show and style the number of files staged files.        |
| `renamed`          | `"»"`                    | A renamed file has been added to the staging area.      |
| `renamed_count`    | [連結](#git-status-counts) | Show and style the number of renamed files.             |
| `deleted`          | `"✘"`                    | A file's deletion has been added to the staging area.   |
| `deleted_count`    | [連結](#git-status-counts) | Show and style the number of deleted files.             |
| `show_sync_count`  | `false`                  | Show ahead/behind count of the branch being tracked.    |
| `prefix`           | `[`                      | Prefix to display immediately before git status.        |
| `suffix`           | `]`                      | Suffix to display immediately after git status.         |
| `style`            | `"bold red"`             | 這個模組的風格。                                                |
| `disabled`         | `false`                  | Disables the `git_status` module.                       |

#### Git Status Counts

| 變數        | 預設      | 說明                                                     |
| --------- | ------- | ------------------------------------------------------ |
| `enabled` | `false` | Show the number of files                               |
| `style`   |         | Optionally style the count differently than the module |

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
staged.value = "++"
staged.style = "green"
staged_count.enabled = true
staged_count.style = "green"
renamed = "👅"
deleted = "🗑"
```

## Golang

The `golang` module shows the currently installed version of Golang. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### 選項

| 變數         | 預設            | 說明                                                       |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | 這個模組的風格。                                                 |
| `disabled` | `false`       | Disables the `golang` module.                            |

### 範例

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `stack.yaml` file

### 選項

| 變數         | 預設           | 說明                                                        |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"λ "`       | The symbol used before displaying the version of Haskell. |
| `style`    | `"bold red"` | 這個模組的風格。                                                  |
| `disabled` | `false`      | Disables the `haskell` module.                            |


### 範例

```toml
# ~/.config/starship.toml

[haskell]
symbol = " "
```

## 主機名稱

`hostname` 模組顯示系統的主機名稱。

### 選項

| 變數         | 預設                    | 說明                                                         |
| ---------- | --------------------- | ---------------------------------------------------------- |
| `ssh_only` | `true`                | 只在連接到一個 SSH session 時顯示主機名稱。                               |
| `prefix`   | `""`                  | 在主機名稱正前方顯示的前綴。                                             |
| `suffix`   | `""`                  | 在主機名稱正後方顯示的後綴。                                             |
| `trim_at`  | `"."`                 | 擷取出主機名稱的斷點，以第一個符合的為準。 `"."` 會讓它停在第一個點的符號。 `""` 會停用任何的截斷功能。 |
| `style`    | `"bold dimmed green"` | 這個模組的風格。                                                   |
| `disabled` | `false`               | 停用 `hostname` 模組。                                          |

### 範例

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
prefix = "⟪"
suffix = "⟫"
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `pom.xml`, `build.gradle.kts` or `build.sbt` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### 選項

| 變數         | 預設             | 說明                                                     |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"☕ "`         | The symbol used before displaying the version of Java. |
| `style`    | `"dimmed red"` | 這個模組的風格。                                               |
| `disabled` | `false`        | Disables the `java` module.                            |

### 範例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### 選項

| 變數          | 預設            | 說明                                                    |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | 這個模組的風格。                                              |
| `disabled`  | `false`       | Disables the `jobs` module.                           |

### 範例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| 變數         | 預設            | 說明                                                  |
| ---------- | ------------- | --------------------------------------------------- |
| `symbol`   | `"☸ "`        | The symbol used before displaying the Cluster info. |
| `style`    | `"bold blue"` | 這個模組的風格。                                            |
| `disabled` | `true`        | Disables the `kubernetes` module                    |

### 範例

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dimmed green"
disabled = false
```

## Line Break

The `line_break` module separates the prompt into two lines.

### 選項

| 變數         | 預設      | 說明                                                                 |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### 範例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| 變數                | 預設                    | 說明                                                            |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | Display memory usage as a percentage of the available memory. |
| `show_swap`       | `true`                | Display swap usage if total swap is non-zero.                 |
| `threshold`       | `75`                  | Hide the memory usage unless it exceeds this percentage.      |
| `symbol`          | `"🐏 "`                | The symbol used before displaying the memory usage.           |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | 這個模組的風格。                                                      |
| `disabled`        | `true`                | Disables the `memory_usage` module.                           |

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

| 變數                  | 預設              | 說明                                                                                           |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated.                                     |
| `style`             | `"bold purple"` | 這個模組的風格。                                                                                     |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### 範例

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### 選項

| 變數           | 預設           | 說明                                 |
| ------------ | ------------ | ---------------------------------- |
| `use_name`   | `false`      | Display the name of the nix-shell. |
| `impure_msg` | `"impure"`   | Customize the "impure" msg.        |
| `pure_msg`   | `"pure"`     | Customize the "pure" msg.          |
| `style`      | `"bold red"` | 這個模組的風格。                           |
| `disabled`   | `false`      | Disables the `nix_shell` module.   |

### 範例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
```

## NodeJS

`nodejs` 模組顯示現在安裝的 NodeJS 版本。 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `package.json` 檔案
- 現在資料夾中包含一個 `node_modules` 資料夾
- 現在資料夾中包含一個檔案具有 `.js` 副檔名

### 選項

| 變數         | 預設             | 說明                   |
| ---------- | -------------- | -------------------- |
| `symbol`   | `"⬢ "`         | 在顯示 NodeJS 版本之前用的符號。 |
| `style`    | `"bold green"` | 這個模組的風格。             |
| `disabled` | `false`        | 停用 `nodejs` 模組。      |

### 範例

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## 套件版本

The `package` 模組在現在資料夾是一個套件的儲藏庫時出現，並顯示他的現在版本。 The module currently supports `npm`, `cargo`, `poetry`, `composer`, and `gradle` packages.

- **npm** – `npm` 套件的版本是從現在資料夾中的 `package.json` 之中擷取出來的
- **cargo** – `cargo` 套件的版本是從現在資料夾中的 `Cargo.toml` 之中擷取出來的
- **poetry** – `poetry` 套件的版本是從現在資料夾中的 `pyproject.toml` 之中擷取出來的
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present

> ⚠️ 顯示出來的版本是從你的現在資料夾之中擷取出來的，並非從套件管理員取得。

### 選項

| 變數         | 預設           | 說明               |
| ---------- | ------------ | ---------------- |
| `symbol`   | `"📦 "`       | 顯示在套件的版本之前的符號。   |
| `style`    | `"bold red"` | 這個模組的風格。         |
| `disabled` | `false`      | 停用 `package` 模組。 |

### 範例

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

The `php` module shows the currently installed version of PHP. 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `composer.json` 檔案
- The current directory contains a `.php` file

### 選項

| 變數         | 預設           | 說明                                                    |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold red"` | 這個模組的風格。                                              |
| `disabled` | `false`      | Disables the `php` module.                            |

### 範例

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

`python` 模組顯示現在安裝的 Python 版本

如果 `pyenv_version_name` 的值為 `true`, 會顯示 pyenv 內的版本名稱

要不然就會顯示 `python -version` 的版本和有啟用的 Python 虛擬環境版本

這個模組在下列其中一個條件達成時顯示：

- 目前資料夾中有一個 `.python-version` 檔案
- 目前資料夾中有一個 `requirements.txt` 檔案
- 目前資料夾中有一個 `pyproject.toml` 檔案
- 目前資料夾中有一個 `.py` 副檔名的檔案
- 目前資料夾中有一個 `Pipfile` 檔案
- 目前資料夾中有一個 `tox.ini` 檔案
- A virtual environment is currently activated

### 選項

| 變數                   | 預設              | 說明                                            |
| -------------------- | --------------- | --------------------------------------------- |
| `symbol`             | `"🐍 "`          | 顯示在 Python 版本之前的符號。                           |
| `pyenv_version_name` | `false`         | 使用 pyenv 取得 Python 的版本。                       |
| `pyenv_prefix`       | `"pyenv "`      | 顯示在 pyenv 版本之前的前綴 (預設顯示是 `pyenv MY_VERSION`)。 |
| `style`              | `"bold yellow"` | 這個模組的風格。                                      |
| `disabled`           | `false`         | 停用 `python` 模組。                               |

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
- 目前資料夾中有一個 `.rb` 檔案

### 選項

| 變數         | 預設           | 說明                |
| ---------- | ------------ | ----------------- |
| `symbol`   | `"💎 "`       | 顯示在 Ruby 版本之前的符號。 |
| `style`    | `"bold red"` | 這個模組的風格。          |
| `disabled` | `false`      | 停用 `ruby` 模組。     |

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

| 變數         | 預設           | 說明                |
| ---------- | ------------ | ----------------- |
| `symbol`   | `"🦀 "`       | 顯示在 Rust 版本之前的符號。 |
| `style`    | `"bold red"` | 這個模組的風格。          |
| `disabled` | `false`      | 停用 `rust` 模組。     |

### 範例

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

:::

### 選項

| 變數         | 預設                   | 說明                                               |
| ---------- | -------------------- | ------------------------------------------------ |
| `label`    | `""`                 | Prefix before the image name display.            |
| `prefix`   | `"["`                | Prefix to display immediately before image name. |
| `suffix`   | `"]"`                | Suffix to display immediately after image name.  |
| `symbol`   | `""`                 | The symbol used before the image name.           |
| `style`    | `"bold dimmed blue"` | 這個模組的風格。                                         |
| `disabled` | `false`              | Disables the `singularity` module.               |

### 範例

```toml
# ~/.config/starship.toml

[singularity]
symbol = "📦 "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### 選項

| 變數             | 預設           | 說明                                                          |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`       | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | 這個模組的風格。                                                    |
| `disabled`     | `false`      | Disables the `terraform` module.                            |

### 範例

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## 時間

`time` 模組顯示目前的**當地**時間. `format` 設定值被 [`chrono`](https://crates.io/crates/chrono) crate 用來控制時間如何顯示。 請看 [chrono 的 strftime 文件](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)來了解有那些選項可以使用。

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| 變數                | 預設              | 說明                                                                                     |
| ----------------- | --------------- | -------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | 啟用 12 小時格式。                                                                            |
| `format`          | 請看下列            | 用來顯示時間的 [chrono 格式字串](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)。 |
| `style`           | `"bold yellow"` | 這個模組的時間的風格。                                                                            |
| `utc_time_offset` | `"local"`       | 設定相對於 UTC 的時差。 範圍 -24 < x < 24。 允許使用浮點數來表示 30/45 分鐘時差的時區。                              |
| `disabled`        | `true`          | 停用 `time` 模組。                                                                          |

如果 `use_12hr` 是 `true` 的話，`format` 會被預設為 `"%r"`。 不然的話，它會被預設為 `"%T"`。 手動設定 `format` 的設定值會覆寫 `use_12hr` 的設定。

### 範例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
```

## 使用者名稱

`username` 模組顯示現在使用中的使用者名稱。 這個模組在下列其中一個條件達成時顯示：

- 目前使用者為 root
- 目前使用者並非登入時的使用者
- 使用者透過 SSH session 進行連線
- 變數 `show_always` 被設為 true

### 選項

| 變數            | 預設              | 說明                  |
| ------------- | --------------- | ------------------- |
| `style_root`  | `"bold red"`    | 使用者為 root 時使用的風格。   |
| `style_user`  | `"bold yellow"` | 非 root 使用者時使用的風格。   |
| `show_always` | `false`         | 總是顯示 `username` 模組。 |
| `disabled`    | `false`         | 停用 `username` 模組。   |

### 範例

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
