# 迁移到 v0.45.0

Starship v0.45.0 是一个含有破坏性变化的版本，它将为大版本v1.0.0作准备。 我们围绕如何迅速进行配置作了一些重大改变，以便能够更大程度地定制化。 我们围绕如何迅速进行配置作了一些重大改变，以便能够更大程度地定制化。

本指南将指明版本的破坏性变化。

## `propt_order` 已被root-level `format`代替

Previously to v0.45.0, `prompt_order` would accept an array of module names in the order which they should be rendered by Starship.

Starship v0.45.0 instead accepts a `format` value, allowing for customization of the prompt outside of the modules themselves.

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

## Module `prefix` and `suffix` have been replaced by `format`

Previously to v0.45.0, some modules would accept `prefix` and/or `suffix` in order to stylize the way that modules are rendered.

Starship v0.45.0 instead accepts a `format` value, allowing for further customization of how modules are rendered. Instead of defining a prefix and suffix for the context-based variables, the variables can now be substituted from within a format string, which represents the module's output.

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
format = "took [$duration]($style) "
```

### 受影响的模块

#### 字符

| 被移除的属性                  | 取而代之的是           |
| ----------------------- | ---------------- |
| `符号`                    | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**更改默认配置**

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

Previously, the `use_symbol_for_status` property was used to configure the prompt to show the `error_symbol` when the last command resulted in a non-zero status code.

With the release of v0.45.0, we now always use `error_symbol` after non-zero status codes, unifying `use_symbol_for_status` and `error_symbol` properties.

To configure the prompt to use the older `use_symbol_for_status = true` configuration, add the following to your config file:

```toml
[character]
error_symbol = "[✖](bold red)"
```

_Note:_ The `character` element automatically adds a space after, so unlike the other `format` strings, we specifically do not add one in the above examples.

#### Command Duration 命令执行时长

| 被移除的属性   | 取而代之的是   |
| -------- | -------- |
| `prefix` | `format` |

**更改默认配置**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Directory 目录

| 被移除的属性   | 取而代之的是   |
| -------- | -------- |
| `prefix` | `format` |

**更改默认配置**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### 环境变量

| 被移除的属性   | 取而代之的是   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**更改默认配置**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git Commit

| 被移除的属性   | 取而代之的是   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**更改默认配置**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git Status

| 被移除的属性            | 取而代之的是   |
| ----------------- | -------- |
| `prefix`          | `format` |
| `suffix`          | `format` |
| `show_sync_count` | `format` |

**更改默认配置**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Previously, the `show_sync_count` property was used to configure the prompt to show the number of commits the branch was ahead or behind the remote branch.

With the release of v0.45.0, this has been replaced with three separate properties, `ahead`, `behind`, and `diverged`.

To configure the prompt to use the older `show_sync_count = true` configuration, set the following to your config file:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

| 被移除的属性   | 取而代之的是   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**更改默认配置**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| 被移除的属性   | 取而代之的是   |
| -------- | -------- |
| `label`  | `format` |
| `prefix` | `format` |
| `suffix` | `format` |

**更改默认配置**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### 时间

| 被移除的属性   | 取而代之的是        |
| -------- | ------------- |
| `format` | `time_format` |

**更改默认配置**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### 自定义命令

| 被移除的属性   | 取而代之的是   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**更改默认配置**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
