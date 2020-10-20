# v0.45への移行

Starship v0.45.0は、v1.0.0の準備として互換性の無い変更を含むリリースになります。 私たちはより多くのカスタマイズを可能にするために、プロンプトでの設定方法にいくつかの大きな変更を加えました。

このガイドは、互換性のない変更を説明することを意図しています。

## `prompt_order`をルートレベルの`format`に置換

Previously to v0.45.0, `prompt_order` would accept an array of module names in the order which they should be rendered by Starship.

Starship v0.45.0 instead accepts a `format` value, allowing for customization of the prompt outside of the modules themselves.

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

## Module `prefix` and `suffix` have been replaced by `format`

Previously to v0.45.0, some modules would accept `prefix` and/or `suffix` in order to stylize the way that modules are rendered.

Starship v0.45.0 instead accepts a `format` value, allowing for further customization of how modules are rendered. Instead of defining a prefix and suffix for the context-based variables, the variables can now be substituted from within a format string, which represents the module's output.

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
format = "took [$duration]($style) "
```

### 影響を受けるモジュール

#### Character

| 削除されたプロパティ              | 置換後              |
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
++ success_symbol = "[❯](bold green)"
++ error_symbol = "[❯](bold red)"
++ vicmd_symbol = "[❮](bold green)"
```

以前は `use_symbol_for_status` プロパティを使用して、最後のコマンドがステータスコードをゼロにした場合、 `error_symbol` を表示するようにプロンプトを設定しました。

v0.45.0 のリリースでは、ステータスコードがゼロでないときに `error_symbol` を常に使用するようになりました。 `use_symbol_for_status` と `error_symbol` プロパティを統合します。

以前の `use_symbol_for_status = true` 設定を使用するようにプロンプトを設定するには、次の設定ファイルを追加します。

```toml
[character]
error_symbol = "[✖](bold red)"
```

*Note:* The `character` element automatically adds a space after, so unlike the other `format` strings, we specifically do not add one in the above examples.

#### Command Duration

| 削除されたプロパティ | 置換後      |
| ---------- | -------- |
| `prefix`   | `format` |

**デフォルト設定への変更**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Directory

| 削除されたプロパティ | 置換後      |
| ---------- | -------- |
| `prefix`   | `format` |

**デフォルト設定への変更**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Environment Variable

| 削除されたプロパティ | 置換後      |
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

| 削除されたプロパティ | 置換後      |
| ---------- | -------- |
| `prefix`   | `format` |
| `suffix`   | `format` |

**デフォルト設定への変更**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git Status

| 削除されたプロパティ        | 置換後      |
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
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

以前は `show_sync_count` プロパティを使用して、 ブランチが先行またはリモートブランチの後ろにあるコミット数を表示するようにプロンプトを設定していました。

With the release of v0.45.0, this has been replaced with three separate properties, `ahead`, `behind`, and `diverged`.

以前の `show_sync_count = true` 設定を使用するようにプロンプトを構成するには、次の設定ファイルを設定します。

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

| 削除されたプロパティ | 置換後      |
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

| 削除されたプロパティ | 置換後      |
| ---------- | -------- |
| `label`    | `format` |
| `prefix`   | `format` |
| `suffix`   | `format` |

**デフォルト設定への変更**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Time

| 削除されたプロパティ | 置換後           |
| ---------- | ------------- |
| `format`   | `time_format` |

**デフォルト設定への変更**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Custom Commands

| 削除されたプロパティ | 置換後      |
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
