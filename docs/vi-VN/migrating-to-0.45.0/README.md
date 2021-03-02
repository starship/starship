# Tích hợp sang v0.45.0

Starship v0.45.0 là một bản phát hành chứa nhiều thay đổi trong việc chuẩn bị cho bản thay đổi lớn v1.0.0. Chúng tôi tạo một vài thay đổi xung quanh cách cấu hình được thực hiện trên dáu nhắc lệnh, cho phép tùy biến theo góc độ tốt hơn.

Hướng dẫn này nhằm hướng dẫn bạn vượt qua những sự thay đổi.

## `prompt_order` được thay thế boiwr một root-level `format`

Từ trước đế v0.45.0, `prompt_order` sẽ chấp nhận một mảng các tên mô đun theo thứ tự mà chúng nên được render bởi Starship.

Starship v0.45.0 thay vì chấp nhận một giá trị `format`, nó cho phép tùy biến dấu nhắc lệnh bên ngoài chính các mô đun đó.

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

### Affected Modules

#### Character

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

*Note:* The `character` element automatically adds a space after, so unlike the other `format` strings, we specifically do not add one in the above examples.

#### Command Duration

| Removed Property | Replacement |
| ---------------- | ----------- |
| `prefix`         | `format`    |

**Changes to the Default Configuration**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Đường dẫn

| Removed Property | Replacement |
| ---------------- | ----------- |
| `prefix`         | `format`    |

**Changes to the Default Configuration**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
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
++ format = '[\($hash\)]($style) '
```

#### Git Status

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
++ format = '[$symbol\[$env\]]($style) '
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
++ format = "at 🕙[$time]($style) "
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
