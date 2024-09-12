# 遷移版本至 v0.45.0

Starship v0.45.0 包含了破壞性的變更，這個變更是為了大的 v1.0.0. 做準備 我們圍繞著如何在提示上完成設定進行了一些重大的更改，以允許更大程度的客製化。

這個指南目的在引導您走過一次這些破壞性的變更

## `prompt_order` 已經被根層級的 `format` 所取代

v0.45.0 以前, `prompt_order` 將會依照 Starship 渲染的順序來接受模組名稱的陣列

取而代之的是 Starship v0.45.0 會接受 `format` 值，這個值允許在模組本身之外自訂提示

**pre-v0.45.0 的設定範例**

```toml
prompt_order = [
  "username",
  "hostname",
  "directory",
  "git_branch",
  "git_commit",
  "git_state",
  "git_status",
  "cmd_duration",
  "custom",
  "line_break",
  "jobs",
  "battery",
  "time",
  "character",
]
```

** v0.45.0 的設定範例**

```toml
format = """\
  $username\
  $hostname\
  $directory\
  $git_branch\
  $git_commit\
  $git_state\
  $git_status\
  $cmd_duration\
  $custom\
  $line_break\
  $jobs\
  $battery\
  $time\
  $character\
  """
```

## 模組 `prefix` 以及 `suffix` 已經被 `format` 所取代

v0.45.0 版之前，有些模組會接受  `prefix` 且/或 `suffix`，以便使得模組呈現的方式更為風格化

Starship v0.45.0 取而代之的接受了 `format` 的值，允許進一步客製模組的渲染方式 現在可以從表示模組輸出的格式字串中取代變數，而不是基於上下文的變數定義前綴以及後綴

**pre-v0.45.0 的設定範例**

```toml
[cmd_duration]
prefix = "took "
```

** v0.45.0 的設定範例**

```toml
[cmd_duration]
# $duration – The command duration (e.g. "15s")
# $style    – The default style of the module (e.g. "bold yellow")
format = "took [$duration]($style) "
```

### 受影響的模組

#### 字元

| 已移除的屬性                  | 取代屬性             |
| ----------------------- | ---------------- |
| `symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**預設設定的異動**

```diff
[character]
-- symbol = "❯"
-- error_symbol = "✖"
-- use_symbol_for_status = true
-- vicmd_symbol = "❮"
++ success_symbol = "[❯](bold green)"
++ error_symbol = "[❯](bold red)"
++ vicmd_symbol = "[❮](bold green)"
```

在之前 `use_symbol_for_status` 屬性會被用於設定提示字元在最後一個指令執行的結果為非 0 的狀態代碼時，會顯示 `error_symbol`

隨著 v0.45.0 版本的發布，我們現在都只會在非零狀態代碼之後使用 `error_symbol`，統一 `use_symbol_for_status` 以及 `error_symbol` 屬性

如果要設定提示字元使用舊的 `use_symbol_for_status = true` 設定，請將以下設定加入您的設定檔案中:

```toml
[character]
error_symbol = "[✖](bold red)"
```

_Note:_ ，`character` 元素會自動附加一個空格, 所以與設定值 `format` 字串不同, 我們上面的例子中刻意沒有加入這個設定

#### 指令持續時間

| 已移除的屬性   | 取代屬性     |
| -------- | -------- |
| `prefix` | `format` |

**預設設定的異動**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### 資料夾

| 已移除的屬性   | 取代屬性     |
| -------- | -------- |
| `prefix` | `format` |

**預設設定的異動**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### 環境變數

| 已移除的屬性   | 取代屬性     |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**預設設定的異動**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git 提交

| 已移除的屬性   | 取代屬性     |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**預設設定的異動**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git 狀態

| 已移除的屬性            | 取代屬性     |
| ----------------- | -------- |
| `prefix`          | `format` |
| `suffix`          | `format` |
| `show_sync_count` | `format` |

**預設設定的異動**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

在之前的版本 `show_sync_count` 屬性是被用於設定提示字元顯示分之在遠端分支之前或之後所 commit 的數量

在 v0.45.0 的版本，這個屬性已經被三個分開的屬性所取代，分別是 `ahead`、`behind` 以及 `diverged`

為了能夠讓題是字元能夠使用舊的  `show_sync_count = true` 設定，請將以下內容設定至您的設定檔當中

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### 主機名稱

| 已移除的屬性   | 取代屬性     |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**預設設定的異動**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| 已移除的屬性   | 取代屬性     |
| -------- | -------- |
| `label`  | `format` |
| `prefix` | `format` |
| `suffix` | `format` |

**預設設定的異動**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### 時間

| 已移除的屬性   | 取代屬性          |
| -------- | ------------- |
| `format` | `time_format` |

**預設設定的異動**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### 自訂指令

| 已移除的屬性   | 取代屬性     |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**預設設定的異動**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
