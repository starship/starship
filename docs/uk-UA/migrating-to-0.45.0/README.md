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

Starship v0.45.0 instead accepts a `format` value, allowing for further customization of how modules are rendered. Замість визначення префікса та суфікса для контекстних змінних тепер змінні можна замінити з рядка формату, який представляє вихідні дані модуля.

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
| `символ`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Зміни до стандартної конфігурації**

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

Раніше параметр `use_symbol_for_status` використовувався для налаштування запиту на показ `error_symbol`, коли остання команда призводила до ненульового коду стану.

З випуском версії 0.45.0 ми тепер завжди використовуємо `error_symbol` після ненульових кодів стану, об’єднуючи параметри `use_symbol_for_status` і `error_symbol`.

Щоб налаштувати запит на використання старішої конфігурації `use_symbol_for_status = true`, додайте наступне до свого файлу налаштувань:

```toml
[character]
error_symbol = "[✖](bold red)"
```

_Примітка:_ Елемент `character` автоматично додає пробіл після, тому, на відміну від інших рядків `format`, ми спеціально не додаємо його в наведених вище прикладах.

#### Command Duration – час виконання

| Вилучено | Заміна   |
| -------- | -------- |
| `prefix` | `format` |

**Зміни до стандартної конфігурації**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Directory

| Вилучено | Заміна   |
| -------- | -------- |
| `prefix` | `format` |

**Зміни до стандартної конфігурації**

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

**Зміни до стандартної конфігурації**

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

**Зміни до стандартної конфігурації**

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

**Зміни до стандартної конфігурації**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Раніше параметр `show_sync_count` використовувався для налаштування запиту на показ кількості комітів, на які гілка була попереду або позаду віддаленої гілки.

З випуском версії 0.45.0 це було замінено трьома окремими параметрами: `ahead`, `behind` і `diverged`.

Щоб налаштувати командний рядок на використання старішої конфігурації `show_sync_count = true`, встановіть у файлі налаштувань таке:

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

**Зміни до стандартної конфігурації**

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

**Зміни до стандартної конфігурації**

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

**Зміни до стандартної конфігурації**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Власні команди

| Вилучено | Заміна   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**Зміни до стандартної конфігурації**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
