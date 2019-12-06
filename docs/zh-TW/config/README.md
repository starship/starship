# 設定

::: tip

🔥 「設定」現在還在建置中。 許多新的設定選項會在之後的版本釋出。

:::

為了開始設定 Starship，請建立下右檔案： `~/.config/starship.toml`.

```shell
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
    "golang",
    "java",
    "nodejs",
    "python",
    "ruby",
    "rust",
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

`aws` 模組顯示現在 AWS 的區域與概況。 這是根據 `AWS_REGION`、`AWS_DEFAULT_REGION` 與 `AWS_PROFILE` 環境變數及 `~/.aws/config` 檔案。

### 選項

| 變數                | 預設              | 說明                                                                          |
| ----------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`          | `"☁️ "`         | 顯示在目前 AWS 配置之前的符號。                                                          |
| `style`           | `"bold yellow"` | 這個模組的風格。                                                                    |
| `disabled`        | `false`         | 停用 `AWS` 模組。                                                                |
| `displayed_items` | `all`           | Choose which item to display. Possible values: [`all`, `profile`, `region`] |

### 範例

```toml
# ~/.config/starship.toml

[aws]
style = "bold blue"
symbol = "🅰 "
displayed_items = "region"
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

| 變數         | 預設              | 說明                    |
| ---------- | --------------- | --------------------- |
| `min_time` | `2`             | 會顯示的最短時間長度。           |
| `prefix`   | `took`          | 在指令持續時間正前方顯示的前綴。      |
| `style`    | `"bold yellow"` | 這個模組的風格。              |
| `disabled` | `false`         | 停用 `cmd_duration` 模組。 |

### 範例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 4
prefix = "underwent "
```

## Conda

如果有設定 `$CONDA_DEFAULT_ENV` 時，`conda` 模組顯示現在 conda 的環境。 注意：這不會抑制 conda 自己的提示字元修飾字，你可能會想執行 `conda config --set changeps1 False`。

### 選項

| 變數         | 預設             | 說明             |
| ---------- | -------------- | -------------- |
| `symbol`   | `"C "`         | 環境名稱前使用的符號。    |
| `style`    | `"bold green"` | 模組的風格。         |
| `disabled` | `false`        | 停用 `conda` 模組。 |

### 範例

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## 資料夾

`directory` 模組顯示到現在資料夾的路徑，並裁減到前三層資料夾。 你的資料夾也會被裁減到你所在的 git 儲存庫的根目錄。

如果正在使用 fish 風格的 pwd 選項，將不會隱藏被裁減的資料夾，而是會根據你在選項中設定的數字看到每一層資料夾的縮寫。

例如，給定一個右列的路徑 `~/Dev/Nix/nixpkgs/pkgs` 其中 `nixpkgs` 是儲存庫的根目錄，而且該選項被設定為 `1`。 你會看到 `~/D/N/nixpkgs/pkgs`，而在這個設定之前則是 `nixpkgs/pkgs`。

### 選項

| 變數                  | 預設            | 說明                        |
| ------------------- | ------------- | ------------------------- |
| `truncation_length` | `3`           | 到達現在資料夾的路徑中，要被裁減掉的資料夾數目。  |
| `truncate_to_repo`  | `true`        | 是否要裁減到你現在所在的 git 儲存庫的根目錄。 |
| `style`             | `"bold cyan"` | 這個模組的風格。                  |
| `disabled`          | `false`       | 停用 `directory` 模組。        |

<details>
<summary>這個模組有些進階設定選項可以控制顯示資料夾。</summary>

| 變數                          | 預設     | 說明                                   |
| --------------------------- | ------ | ------------------------------------ |
| `fish_style_pwd_dir_length` | `0`    | 當使用 fish shell 的 pwd 路徑邏輯時使用的字元數量。   |
| `use_logical_path`          | `true` | 顯示 shell (`PWD`) 提供的邏輯路徑，而不是 OS 的路徑。 |

</details>

### 範例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Dotnet

`dotnet` 模組顯示現在資料夾使用的 .NET Core SDK 的版本。 如果這個資料夾已經選定一個 SDK，則顯示這個 SDK 的版本。 如果沒有的話，則顯示最新安裝的 SDK 版本。

這個模組只會在下列檔案出現在你的現在資料夾中時，顯示在你的提示字元：`global.json`、`project.json`、`*.sln`、`*.csproj`、`*.fsproj`、`*.xproj`。 你也會需要安裝 .NET Core 文字命令工具來正確使用這個模組。

這個模組內部是使用它自己的機制來偵測版本。 一般來說這個模組有 `dotnet --version` 的兩倍快，但是它可能會在你的 .NET 專案有不尋常的資料夾結構時顯示不正確的版本。 如果精確度比速度更重要的話，你可以藉由設定模組中的 `heuristic = false` 選項來停用這個功能。

### 選項

| 變數          | 預設            | 說明                                                    |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"•NET "`     | 在顯示 dotnet 版本之前用的符號。                                  |
| `heuristic` | `true`        | Use faster version detection to keep starship snappy. |
| `style`     | `"bold blue"` | The style for the module.                             |
| `disabled`  | `false`       | 停用 `dotnet` 模組。                                       |

### 範例

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## 環境變數

`env_var`模組顯示一個選擇的環境變數的現在數值。 這個模組只在下列條件其中之一達到時顯示：

- `variable` 設定選項符合一個存在的環境變數。
- 沒有設定 `variable` 選項，但是有設定 `default` 選項。

### 選項

| 變數         | 預設               | 說明                   |
| ---------- | ---------------- | -------------------- |
| `symbol`   |                  | 顯示在變數數值之前的符號。        |
| `variable` |                  | 要顯示的環境變數。            |
| `default`  |                  | 在選擇的變數值沒有定義時，顯示的預設值。 |
| `prefix`   | `""`             | 在變數值正前方顯示的前綴。        |
| `suffix`   | `""`             | 在變數值正後方顯示的後綴。        |
| `style`    | `"dimmed black"` | 這個模組的風格。             |
| `disabled` | `false`          | 停用 `env_var` 模組。     |

### 範例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Git 分支

`git_branch` 模組顯示現在的資料夾中使用中的儲存庫的分支。

### 選項

| 變數                  | 預設              | 說明                               |
| ------------------- | --------------- | -------------------------------- |
| `symbol`            | `" "`          | 在你現在資料夾之中的儲存庫的分支名稱前使用的符號。        |
| `truncation_length` | `2^63 - 1`      | 裁減一個 git 分支到 X 字素 (grapheme)。    |
| `truncation_symbol` | `"…"`           | 用來指示分支名稱被縮減的符號。 你可以用 "" 來表示不要顯示。 |
| `style`             | `"bold purple"` | 這個模組的風格。                         |
| `disabled`          | `false`         | 停用 `git_branch` 模組。              |

### 範例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the active branch of the repo in your current directory.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 選項

| 變數                   | 預設             | 說明                                               |
| -------------------- | -------------- | ------------------------------------------------ |
| `commit_hash_length` | `7`            | The length of the displayed git commit hash.     |
| `prefix`             | `(`            | Prefix to display immediately before git commit. |
| `suffix`             | `)`            | Suffix to display immediately after git commit.  |
| `style`              | `"bold green"` | The style for the module.                        |
| `disabled`           | `true`         | Disables the `git_commit` module.                |

### 範例

```toml
# ~/.config/starship.toml

[git_commit]
disabled = false
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
| `style`            | `"bold yellow"`    | The style for the module.                                                                                        |
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

| 變數                 | 預設                         | 說明                                                      |
| ------------------ | -------------------------- | ------------------------------------------------------- |
| `conflicted`       | `"="`                      | This branch has merge conflicts.                        |
| `conflicted_count` | [link](#git-status-counts) | Show and style the number of conflicts.                 |
| `ahead`            | `"⇡"`                      | This branch is ahead of the branch being tracked.       |
| `behind`           | `"⇣"`                      | This branch is behind of the branch being tracked.      |
| `diverged`         | `"⇕"`                      | This branch has diverged from the branch being tracked. |
| `untracked`        | `"?"`                      | There are untracked files in the working directory.     |
| `untracked_count`  | [link](#git-status-counts) | Show and style the number of untracked files.           |
| `stashed`          | `"$"`                      | A stash exists for the local repository.                |
| `modified`         | `"!"`                      | There are file modifications in the working directory.  |
| `modified_count`   | [link](#git-status-counts) | Show and style the number of modified files.            |
| `staged`           | `"+"`                      | A new file has been added to the staging area.          |
| `staged_count`     | [link](#git-status-counts) | Show and style the number of files staged files.        |
| `renamed`          | `"»"`                      | A renamed file has been added to the staging area.      |
| `renamed_count`    | [link](#git-status-counts) | Show and style the number of renamed files.             |
| `deleted`          | `"✘"`                      | A file's deletion has been added to the staging area.   |
| `deleted_count`    | [link](#git-status-counts) | Show and style the number of deleted files.             |
| `show_sync_count`  | `false`                    | Show ahead/behind count of the branch being tracked.    |
| `prefix`           | `[`                        | Prefix to display immediately before git status.        |
| `suffix`           | `]`                        | Suffix to display immediately after git status.         |
| `style`            | `"bold red"`               | The style for the module.                               |
| `disabled`         | `false`                    | Disables the `git_status` module.                       |

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

The `golang` module shows the currently installed version of Golang. The module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `go.mod` 檔案
- 現在資料夾中含有一個 `go.sum` 檔案
- 現在資料夾中含有一個 `glide.yaml` 檔案
- 現在資料夾中含有一個 `Gopkg.yml` 檔案
- 現在資料夾中含有一個 `Gopkg.lock` 檔案
- 現在資料夾中含有一個 `Godeps` 資料夾
- 現在資料夾中含有一個檔案具有 `.go` 副檔名

### 選項

| 變數         | 預設            | 說明                                                       |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | The style for the module.                                |
| `disabled` | `false`       | Disables the `golang` module.                            |

### 範例

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### 選項

| 變數                  | 預設              | 說明                                                                                           |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated.                                     |
| `style`             | `"bold purple"` | The style for the module.                                                                    |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### 範例

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Hostname

The `hostname` module shows the system hostname.

### 選項

| 變數         | 預設                    | 說明                                                                                                                                   |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session.                                                                                 |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.                                                                                   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.                                                                                    |
| `trim_at`  | `"."`                 | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `style`    | `"bold dimmed green"` | The style for the module.                                                                                                            |
| `disabled` | `false`               | Disables the `hostname` module.                                                                                                      |

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

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### 選項

| 變數          | 預設            | 說明                                                    |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | The style for the module.                             |
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

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 選項

| 變數         | 預設            | 說明                                                  |
| ---------- | ------------- | --------------------------------------------------- |
| `symbol`   | `"☸ "`        | The symbol used before displaying the Cluster info. |
| `style`    | `"bold blue"` | The style for the module.                           |
| `disabled` | `true`        | Disables the `kubernetes` module                    |

### 範例

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dim green"
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

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### 選項

| 變數           | 預設           | 說明                                 |
| ------------ | ------------ | ---------------------------------- |
| `use_name`   | `false`      | Display the name of the nix-shell. |
| `impure_msg` | `impure`     | Customize the "impure" msg.        |
| `pure_msg`   | `pure`       | Customize the "pure" msg.          |
| `style`      | `"bold red"` | The style for the module.          |
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

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 選項

| 變數                | 預設                    | 說明                                                            |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | Display memory usage as a percentage of the available memory. |
| `show_swap`       | `true`                | Display swap usage if total swap is non-zero.                 |
| `threshold`       | `75`                  | Hide the memory usage unless it exceeds this percentage.      |
| `symbol`          | `"🐏 "`                | The symbol used before displaying the memory usage.           |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | The style for the module.                                     |
| `disabled`        | `true`                | Disables the `memory_usage` module.                           |

### 範例

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

## Java

The `java` module shows the currently installed version of Java. The module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `pom.xml`、`build.gradle` 或 `build.sbt` 檔案
- 現在資料夾中包含一個檔案具有 `.java`、`.class` 或 `.jar` 副檔名

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

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. The module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `package.json` 檔案
- 現在資料夾中包含一個 `node_modules` 資料夾
- 現在資料夾中包含一個檔案具有 `.js` 副檔名

### 選項

| 變數         | 預設             | 說明                                                       |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | The style for the module.                                |
| `disabled` | `false`        | Disables the `nodejs` module.                            |

### 範例

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, and `poetry` packages.

- **npm** – `npm` 套件的版本是從現在資料夾中的 `package.json` 之中擷取出來的
- **cargo** – `cargo` 套件的版本是從現在資料夾中的 `Cargo.toml` 之中擷取出來的
- **poetry** – `poetry` 套件的版本是從現在資料夾中的 `pyproject.toml` 之中擷取出來的

> ⚠️ 顯示出來的版本是從你的現在資料夾之中擷取出來的，並非從套件管理員取得。

### 選項

| 變數         | 預設           | 說明                                                         |
| ---------- | ------------ | ---------------------------------------------------------- |
| `symbol`   | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`    | `"bold red"` | The style for the module.                                  |
| `disabled` | `false`      | Disables the `package` module.                             |

### 範例

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

The `php` module shows the currently installed version of PHP. The module will be shown if any of the following conditions are met:

- The current directory contains a `composer.json` file
- The current directory contains a `.php` file

### 選項

| 變數         | 預設           | 說明                                                    |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold red"` | The style for the module.                             |
| `disabled` | `false`      | Disables the `php` module.                            |

### 範例

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

### 選項

| 變數                   | 預設              | 說明                                                                          |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `style`              | `"bold yellow"` | The style for the module.                                                   |
| `disabled`           | `false`         | Disables the `python` module.                                               |

### 範例

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

### 選項

| 變數         | 預設           | 說明                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`       | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | The style for the module.                              |
| `disabled` | `false`      | Disables the `ruby` module.                            |

### 範例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust. The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Options

| Variable   | Default      | Description                                            |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`       | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | The style for the module.                              |
| `disabled` | `false`      | Disables the `rust` module.                            |

### Example

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Variable          | Default       | Description                                                                                                         |
| ----------------- | ------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`       | Enables 12 hour formatting                                                                                          |
| `format`          | see below     | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `bold yellow` | The style for the module time                                                                                       |
| `disabled`        | `true`        | Disables the `time` module.                                                                                         |
| `utc_time_offset` | `local`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### Example

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

### Options

| Variable      | Default         | Description                           |
| ------------- | --------------- | ------------------------------------- |
| `style_root`  | `"bold red"`    | The style used when the user is root. |
| `style_user`  | `"bold yellow"` | The style used for non-root users.    |
| `show_always` | `false`         | Always shows the `username` module.   |
| `disabled`    | `false`         | Disables the `username` module.       |

### Example

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
