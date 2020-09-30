# 0.45への移行

Starship v0.45.0は、v1.0.0の準備として互換性の無い変更を含むリリースになります。 私たちはより多くのカスタマイズを可能にするために、プロンプトでの設定方法にいくつかの大きな変更を加えました。

このガイドは、互換性のない変更を説明することを意図しています。

## `prompt_order`をルートレベルの`format`に置換

v0.45.0以前は、`prompt_order` はStarshipによってレンダリングされる順序でモジュール名の配列を指定できるようになっていました。

Starship v0.45.0は代わりに  `format` を指定できるようになり、モジュール自体の外側でプロンプトをカスタマイズ可能になります。

**v0.45.0以前の設定例**

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

**v0.45.0での設定例**

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

## `prefix`と `suffix` モジュールが`format`を置換

v0.45.0以前では、モジュールのレンダリング方法をスタイリングするために、 `prefix` や`suffix`の指定可能なモジュールがありました。

Starship v0.45.0 は代わりに `format` の値を受け付け、モジュールのレンダリング方法をさらにカスタマイズすることができます。 接頭辞と接尾辞を定義する代わりに、コンテキストベースの変数については、モジュールの出力を表現するフォーマット文字列の中から変数を置き換えることができるようになりました。

**v0.45.0以前の設定例**

```toml
[cmd_duration]
prefix = "took "
```

**v0.45.0での設定例**

```toml
[cmd_duration]
# $duration – コマンド実行時間 (例: "15s")
# $style    – デフォルトのモジュールスタイル (例: "bold yellow")
format = "took [$duration]($style)"
```

### 影響を受けるモジュール

#### Character

| 削除されたプロパティ              | 再配置              |
| ----------------------- | ---------------- |
| `symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**デフォルト設定への変更**

```diff
[character]
-- symbol = "❯"
-- error_symbol = "✖"
-- use_symbol_for_status = true
-- vicmd_symbol = "❮"
++ success_symbol = "[❯](bold green) "
++ error_symbol = "[❯](bold red) "
++ vicmd_symbol = "[❮](bold green)"
```

Previously, the `use_symbol_for_status` property was used to configure the prompt to show the `error_symbol` when the last command resulted in a non-zero status code.

With the release of v0.45.0, we now always use `error_symbol` after non-zero status codes, unifying `use_symbol_for_status` and `error_symbol` properties.

To configure the prompt to use the older `use_symbol_for_status = true` configuration, add the following to your config file:

```toml
[character]
error_symbol = "[✖](bold red) "
```

#### コマンド実行時間

| 削除されたプロパティ | 再配置      |
| ---------- | -------- |
| `prefix`   | `format` |

**デフォルト設定への変更**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style)"
```

#### Directory

| 削除されたプロパティ | 再配置      |
| ---------- | -------- |
| `prefix`   | `format` |

**デフォルト設定への変更**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$lock_symbol]($lock_style)"
```

#### Environment Variable

| 削除されたプロパティ | 再配置      |
| ---------- | -------- |
| `prefix`   | `format` |
| `suffix`   | `format` |

**デフォルト設定への変更**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git Commit

| 削除されたプロパティ | 再配置      |
| ---------- | -------- |
| `prefix`   | `format` |
| `suffix`   | `format` |

**デフォルト設定への変更**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = "[\\($hash\\)]($style) "
```

#### Git の状態

| 削除されたプロパティ        | 再配置      |
| ----------------- | -------- |
| `prefix`          | `format` |
| `suffix`          | `format` |
| `show_sync_count` | `format` |

**デフォルト設定への変更**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = "([$all_status$ahead_behind] )"
```

Previously, the `show_sync_count` property was used to configure the prompt to show the number of commits the branch was ahead or behind the remote branch.

With the release of v0.45.0, this has been replaced with the

To configure the prompt to use the older `show_sync_count = true` configuration, set the following to your config file:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

| 削除されたプロパティ | 再配置      |
| ---------- | -------- |
| `prefix`   | `format` |
| `suffix`   | `format` |

**デフォルト設定への変更**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| 削除されたプロパティ | 再配置      |
| ---------- | -------- |
| `label`    | `format` |
| `prefix`   | `format` |
| `suffix`   | `format` |

**デフォルト設定への変更**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol\\[$env\\]]($style) "
```

#### Time

| 削除されたプロパティ | 再配置           |
| ---------- | ------------- |
| `format`   | `time_format` |

**デフォルト設定への変更**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style)
```

#### Custom Commands

| 削除されたプロパティ | 再配置      |
| ---------- | -------- |
| `prefix`   | `format` |
| `suffix`   | `format` |

**デフォルト設定への変更**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
