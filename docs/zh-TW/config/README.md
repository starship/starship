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

You can change default `starship.toml` file location with `STARSHIP_CONFIG` environment variable:
```shell
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

## 資料夾

`directory` 模組顯示到現在資料夾的路徑，並裁減到前三層資料夾。 你的資料夾也會被裁減到你所在的 git 儲存庫的根目錄。

如果正在使用 fish 風格的 pwd 選項，將不會隱藏被裁減的資料夾，而是會根據你在選項中設定的數字看到每一層資料夾的縮寫。

例如，給定一個右列的路徑 `~/Dev/Nix/nixpkgs/pkgs` 其中 `nixpkgs` 是儲存庫的根目錄，而且該選項被設定為 `1`。 你會看到 `~/D/N/nixpkgs/pkgs`，而在這個設定之前則是 `nixpkgs/pkgs`。

### 選項

| 變數                  | 預設            | 說明                                                  |
| ------------------- | ------------- | --------------------------------------------------- |
| `truncation_length` | `3`           | 到達現在資料夾的路徑中，要被裁減掉的資料夾數目。                            |
| `truncate_to_repo`  | `true`        | 是否要裁減到你現在所在的 git 儲存庫的根目錄。                           |
| `prefix`            | `"in "`       | Prefix to display immediately before the directory. |
| `style`             | `"bold cyan"` | 這個模組的風格。                                            |
| `disabled`          | `false`       | 停用 `directory` 模組。                                  |

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

| 變數          | 預設            | 說明                           |
| ----------- | ------------- | ---------------------------- |
| `symbol`    | `"•NET "`     | 在顯示 dotnet 版本之前用的符號。         |
| `heuristic` | `true`        | 使用更快速的版本偵測法來保持 starship 的速度。 |
| `style`     | `"bold blue"` | 這個模組的風格。                     |
| `disabled`  | `false`       | 停用 `dotnet` 模組。              |

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

The `git_commit` module shows the current commit hash of the repo in your current directory.

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| 變數                   | 預設             | 說明                                               |
| -------------------- | -------------- | ------------------------------------------------ |
| `commit_hash_length` | `7`            | The length of the displayed git commit hash.     |
| `prefix`             | `"("`          | Prefix to display immediately before git commit. |
| `suffix`             | `")"`          | Suffix to display immediately after git commit.  |
| `style`              | `"bold green"` | 這個模組的風格。                                         |
| `disabled`           | `true`         | Disables the `git_commit` module.                |

### 範例

```toml
# ~/.config/starship.toml

[git_commit]
disabled = false
commit_hash_length = 4
```

## Git State

`git_state` 模組會顯示在 git 儲存庫中的資料夾內，以及會在有作業正在進行時顯示，像是：_REBASING_、_BISECTING_ 等等。 如果有進展的資訊 (像是 REBASING 3/10)，也會一併顯示出來。

### 選項

| 變數                 | 預設                 | 說明                                                 |
| ------------------ | ------------------ | -------------------------------------------------- |
| `rebase`           | `"REBASING"`       | `rebase` 進行中顯示的文字。                                 |
| `merge`            | `"MERGING"`        | `merge` 進行中顯示的文字。                                  |
| `revert`           | `"REVERTING"`      | `revert` 進行中顯示的文字。                                 |
| `cherry_pick`      | `"CHERRY-PICKING"` | `cherry-pick` 進行中顯示的文字。                            |
| `bisect`           | `"BISECTING"`      | `bisect` 進行中顯示的文字。                                 |
| `am`               | `"AM"`             | `apply-mailbox` (`git am`) 進行中顯示的文字。               |
| `am_or_rebase`     | `"AM/REBASE"`      | 當不容易分辨是 `apply-mailbox` 或 `rebase` 正在進行中時顯示的文字。    |
| `progress_divider` | `"/"`              | 用來分開現在與總共進度量的符號。 (例如：`" of "` 會得到 `"3 of 10"` 的效果) |
| `style`            | `"bold yellow"`    | 這個模組的風格。                                           |
| `disabled`         | `false`            | 停用 `git_state` 模組。                                 |

### 範例

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git Status

`git_status` 模組顯示用來表示現在資料夾之中儲存庫狀態的符號。

### 選項

| 變數                 | 預設                       | 說明                                               |
| ------------------ | ------------------------ | ------------------------------------------------ |
| `conflicted`       | `"="`                    | 這個分支有合併衝突。                                       |
| `conflicted_count` | [連結](#git-status-counts) | Show and style the number of conflicts.          |
| `ahead`            | `"⇡"`                    | 這個分支超前正在追蹤的分支。                                   |
| `behind`           | `"⇣"`                    | 這個分支落後正在追蹤的分支。                                   |
| `diverged`         | `"⇕"`                    | 這個分支偏離正在追蹤的分支。                                   |
| `untracked`        | `"?"`                    | 工作資料夾中有沒有追蹤的檔案。                                  |
| `untracked_count`  | [連結](#git-status-counts) | Show and style the number of untracked files.    |
| `stashed`          | `"$"`                    | 本地儲存庫有 stash。                                    |
| `stashed_count`    | [連結](#git-status-counts) | Show and style the number of stashes.            |
| `modified`         | `"!"`                    | 工作資料夾中有修改過的檔案。                                   |
| `modified_count`   | [連結](#git-status-counts) | Show and style the number of modified files.     |
| `staged`           | `"+"`                    | 一個新檔案被加入了暫存區 (staging area)。                     |
| `staged_count`     | [連結](#git-status-counts) | Show and style the number of files staged files. |
| `renamed`          | `"»"`                    | 一個被改名的檔案被加入了暫存區 (staging area)。                  |
| `renamed_count`    | [連結](#git-status-counts) | Show and style the number of renamed files.      |
| `deleted`          | `"✘"`                    | 一個刪除檔案的動作被加入了暫存區 (staging area)。                 |
| `deleted_count`    | [連結](#git-status-counts) | Show and style the number of deleted files.      |
| `show_sync_count`  | `false`                  | 顯示超前/落後追蹤的分支的數量。                                 |
| `prefix`           | `[`                      | 在 git 狀態正前方顯示的前綴。                                |
| `suffix`           | `]`                      | 在 git 狀態正後方顯示的後綴。                                |
| `style`            | `"bold red"`             | 這個模組的風格。                                         |
| `disabled`         | `false`                  | 停用 `git_status` 模組。                              |

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

`golang` 模組顯示現在安裝的 Golang 版本。 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中含有一個 `go.mod` 檔案
- 現在資料夾中含有一個 `go.sum` 檔案
- 現在資料夾中含有一個 `glide.yaml` 檔案
- 現在資料夾中含有一個 `Gopkg.yml` 檔案
- 現在資料夾中含有一個 `Gopkg.lock` 檔案
- 現在資料夾中含有一個 `Godeps` 資料夾
- 現在資料夾中含有一個檔案具有 `.go` 副檔名

### 選項

| 變數         | 預設            | 說明                  |
| ---------- | ------------- | ------------------- |
| `symbol`   | `"🐹 "`        | 顯示在 Golang 版本之前的符號。 |
| `style`    | `"bold cyan"` | 這個模組的風格。            |
| `disabled` | `false`       | 停用 `golang` 模組。     |

### 範例

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. 這個模組在下列其中一個條件達成時顯示：

- 目前資料夾中有一個 `stack.yaml` 檔案

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
symbol = "λx.x "
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### 選項

| 變數                  | 預設              | 說明                                                                                           |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | 用來指示分支名稱被縮減的符號。                                                                              |
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

## 工作

`jobs` 模組顯示現在正在執行中的工作。 這個模組只會在有背景工作正在執行時顯示。 這個模組會在工作數量超過一個，或者有設定 `threshold` 時且數量超過設定值時，顯示工作的數量。

### 選項

| 變數          | 預設            | 說明             |
| ----------- | ------------- | -------------- |
| `symbol`    | `"✦"`         | 在顯示工作數量之前用的符號。 |
| `threshold` | `1`           | 在超過指定值時顯示工作數量。 |
| `style`     | `"bold blue"` | 這個模組的風格。       |
| `disabled`  | `false`       | 停用 `jobs` 模組。  |

### 範例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Kubernetes

顯示現在 Kubernetes 主體名稱以及從 kubeconfig 檔案來的名稱空間 (如果有設定的話)。 The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. 如果有設定 `$KUBECONFIG` 環境變數，這個模組就會使用設定值；如果沒有，它就會使用 `~/.kube/config`。

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| 變數         | 預設            | 說明                       |
| ---------- | ------------- | ------------------------ |
| `symbol`   | `"☸ "`        | 顯示在叢集 (cluster) 資訊之前的符號。 |
| `style`    | `"bold blue"` | 這個模組的風格。                 |
| `disabled` | `true`        | 停用 `kubernetes` 模組。      |

### 範例

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dim green"
disabled = false
```

## 換行

`line_break` 模組將提示字元分成兩行。

### 選項

| 變數         | 預設      | 說明                            |
| ---------- | ------- | ----------------------------- |
| `disabled` | `false` | 停用 `line_break` 模組，讓提示字元變成一行。 |

### 範例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Nix-shell

`nix_shell` 模組顯示 nix-shell 環境。 這個模組會在 nix-shell 環境中顯示。

### 選項

| 變數           | 預設           | 說明                 |
| ------------ | ------------ | ------------------ |
| `use_name`   | `false`      | 顯示 nix-shell 的名稱。  |
| `impure_msg` | `"impure"`   | 自定義「impure」訊息。     |
| `pure_msg`   | `"pure"`     | 自定義「pure」訊息。       |
| `style`      | `"bold red"` | 這個模組的風格。           |
| `disabled`   | `false`      | 停用 `nix_shell` 模組。 |

### 範例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
```

## Java

`java` 模組顯示現在安裝的 Java 版本。 這個模組在下列其中一個條件達成時顯示：

- 現在資料夾中包含一個 `pom.xml`、`build.gradle.kts` 或 `build.sbt` 檔案
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### 選項

| 變數         | 預設             | 說明                |
| ---------- | -------------- | ----------------- |
| `symbol`   | `"☕ "`         | 顯示在 Java 版本之前的符號。 |
| `style`    | `"dimmed red"` | 這個模組的風格。          |
| `disabled` | `false`        | 停用 `java` 模組。     |

### 範例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## 記憶體使用量

`memory_usage` 模組顯示現在系統記憶體與 swap 的使用量。

預設 swap 使用量會在系統總 swap 使用量不為 0 時顯示出來。

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| 變數                | 預設                    | 說明                                                            |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | 以剩餘記憶體佔有的百分比的方式顯示記憶體使用狀況。                                     |
| `show_swap`       | `true`                | 如果總 swap 使用量不為零的話，顯示 swap 使用量                                 |
| `threshold`       | `75`                  | 將記憶體使用量隱藏，除非使用量超過指定值。                                         |
| `symbol`          | `"🐏 "`                | 顯示在記憶體使用量之前的符號。                                               |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | 這個模組的風格。                                                      |
| `disabled`        | `true`                | 停用 `memory_usage` 模組。                                         |

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

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. The module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### 選項

| 變數         | 預設             | 說明                                                       |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | 這個模組的風格。                                                 |
| `disabled` | `false`        | Disables the `nodejs` module.                            |

### 範例

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

> ⚠️ 顯示出來的版本是從你的現在資料夾之中擷取出來的，並非從套件管理員取得。

### 選項

| 變數         | 預設           | 說明                                                         |
| ---------- | ------------ | ---------------------------------------------------------- |
| `symbol`   | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`    | `"bold red"` | 這個模組的風格。                                                   |
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

The `ruby` module shows the currently installed version of Ruby. 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `Gemfile` file
- The current directory contains a `.rb` file

### 選項

| 變數         | 預設           | 說明                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`       | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | 這個模組的風格。                                               |
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

### 選項

| 變數         | 預設           | 說明                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`       | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | The style for the module.                              |
| `disabled` | `false`      | Disables the `rust` module.                            |

### 範例

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. The module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### 選項

| 變數             | 預設           | 說明                                                          |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`       | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | The style for the module.                                   |
| `disabled`     | `false`      | Disables the `terraform` module.                            |

### 範例

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

### 選項

| 變數                | 預設            | 說明                                                                                                                  |
| ----------------- | ------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`       | Enables 12 hour formatting                                                                                          |
| `format`          | see below     | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `bold yellow` | The style for the module time                                                                                       |
| `utc_time_offset` | `local`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |
| `disabled`        | `true`        | Disables the `time` module.                                                                                         |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### 範例

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
