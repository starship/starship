# 0.45への移行

Starship v0.45.0は、v1.0.0の準備として互換性の無い変更を含むリリースになります。 私たちはより多くのカスタマイズを可能にするために、プロンプトでの設定方法にいくつかの大きな変更を加えました。

このガイドは、互換性のない変更を説明することを意図しています。

## `prompt_order`がルートレベルの`format`に置換

Previously to v0.45.0, `prompt_order` would accept an array of module names in the order which they should be rendered by Starship.

Starship v0.45.0 will instead be accepting a `format` value, allowing for customization of the prompt outside of the modules themselves.

**Example pre-v0.45.0 configuration**

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

**Example v0.45.0 configuration**

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

## Module `prefix` and `suffix` will be replaced by `format`

Previously to v0.45.0, some modules would accept `prefix` and/or `suffix` in order to stylize the way that modules are rendered.

Starship v0.45.0 will instead be accepting a `format` value, allowing for further customization of how modules are rendered. Instead of defining a prefix and suffix for the context-based variables, the variables can now be substituted from within a format string, which represents the module's output.

**Example pre-v0.45.0 configuration**

```toml
[cmd_duration]
prefix = "took "
```

**Example v0.45.0 configuration**

```toml
[cmd_duration]
# $duration – The command duration (e.g. "15s")
# $style    – The default style of the module (e.g. "bold yellow")
format = "took [$duration]($style)"
```

### Affected Modules

#### 文字

| Removed Property        | Replacement      |
| ----------------------- | ---------------- |
| `symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Changes to the Default Configuration**

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

| Removed Property | Replacement |
| ---------------- | ----------- |
| `prefix`         | `format`    |

**Changes to the Default Configuration**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style)"
```

#### Directory

| Removed Property | Replacement |
| ---------------- | ----------- |
| `prefix`         | `format`    |

**Changes to the Default Configuration**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$lock_symbol]($lock_style)"
```

#### Environment Variable

| Removed Property | Replacement |
| ---------------- | ----------- |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Changes to the Default Configuration**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git Commit

| Removed Property | Replacement |
| ---------------- | ----------- |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Changes to the Default Configuration**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = "[\\($hash\\)]($style) "
```

#### Git の状態

| Removed Property  | Replacement |
| ----------------- | ----------- |
| `prefix`          | `format`    |
| `suffix`          | `format`    |
| `show_sync_count` | `format`    |

**Changes to the Default Configuration**

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

| Removed Property | Replacement |
| ---------------- | ----------- |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Changes to the Default Configuration**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| Removed Property | Replacement |
| ---------------- | ----------- |
| `label`          | `format`    |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Changes to the Default Configuration**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol\\[$env\\]]($style) "
```

#### Time

| Removed Property | Replacement   |
| ---------------- | ------------- |
| `format`         | `time_format` |

**Changes to the Default Configuration**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style)
```

#### Custom Commands

| Removed Property | Replacement |
| ---------------- | ----------- |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Changes to the Default Configuration**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
