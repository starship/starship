# Міграція на v0.45.0

Starship v0.45.0 — це реліз, що містить останні зміни, на шляху до версії v1.0.0. Ми внесли кілька суттєвих змін у те, як виконується налаштування командного рядка, щоб забезпечити більший ступінь налаштувань.

Цей посібник має на меті ознайомити вас із критичними змінами.

## `prompt_order` було замінено на `format` кореневого рівня

Раніше до версії 0.45.0 `prompt_order` приймав масив імен модулів у порядку, в якому їх мав показувати Starship.

Натомість Starship v0.45.0 приймає значення `format`, що дозволяє налаштувати командний рядок поза межами самих модулів.

**Приклад налаштувань pre-v0.45.0**

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

**Приклад налаштувань v0.45.0**

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

## Модулі `prefix` та `suffix` були замінені на  `format`

До версії 0.45.0 деякі модулі приймали `prefix` та/або `suffix`, щоб стилізувати спосіб показу модулів.

Натомість Starship v0.45.0 приймає значення `format`, що дозволяє додатково налаштовувати спосіб показу модулів. Замість визначення префікса та суфікса для контекстних змінних тепер змінні можна замінити з рядка формату, який представляє вихідні дані модуля.

**Приклад налаштувань pre-v0.45.0**

```toml
[cmd_duration]
prefix = "took "
```

**Приклад налаштувань v0.45.0**

```toml
[cmd_duration]
# $duration – The command duration (e.g. "15s")
# $style    – The default style of the module (e.g. "bold yellow")
format = "took [$duration]($style) "
```

### Вражені модулі

#### Character

| Вилучено                | Заміна           |
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

_Note:_ The `character` element automatically adds a space after, so unlike the other `format` strings, we specifically do not add one in the above examples.

#### Command Duration – час виконання

| Вилучено | Заміна   |
| -------- | -------- |
| `prefix` | `format` |

**Changes to the Default Configuration**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Directory

| Вилучено | Заміна   |
| -------- | -------- |
| `prefix` | `format` |

**Changes to the Default Configuration**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Environment Variable

| Вилучено | Заміна   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**Changes to the Default Configuration**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git Commit

| Вилучено | Заміна   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**Changes to the Default Configuration**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git Status

| Вилучено          | Заміна   |
| ----------------- | -------- |
| `prefix`          | `format` |
| `suffix`          | `format` |
| `show_sync_count` | `format` |

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

| Вилучено | Заміна   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**Changes to the Default Configuration**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| Вилучено | Заміна   |
| -------- | -------- |
| `label`  | `format` |
| `prefix` | `format` |
| `suffix` | `format` |

**Changes to the Default Configuration**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Time

| Вилучено | Заміна        |
| -------- | ------------- |
| `format` | `time_format` |

**Changes to the Default Configuration**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Custom Commands

| Вилучено | Заміна   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**Changes to the Default Configuration**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
