# Миграция на v0.45.0

Starship v0.45.0 - это релиз, содержащий важные изменения в рамках подготовки к выпуску большой v1.0.0. Мы внесли несколько существенных изменений в то, как выполняется настройка в промпте, чтобы обеспечить большую степень настройки.

Это руководство предназначено для того, чтобы ознакомить вас с основными изменениями.

## `prompt_order` был заменен на корневой уровень `format`

Ранее, до v0.45.0, `prompt_order` принимал массив имен модулей в том порядке, в котором они должны были отображаться Starship.

Вместо Starship v0.45.0 принимает значение `format`, что позволяет настраивать промпт вне самих модулей.

**Пример конфигурации pre-v0.45.0**

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

**Пример конфигурации v0.45.0**

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

## Модуль `prefix` и `suffix` были заменены на `format`

Ранее, до v0.45.0, некоторые модули принимали `prefix` и/или `suffix`, чтобы стилизовать способ отображения модулей.

Вместо Starship v0.45.0 принимает значение `format`, что позволяет дополнительно настроить отображение модулей. Вместо определения префикса и суффикса для контекстных переменных, теперь можно заменить переменные из строки формата, которая представляет вывод модуля.

**Пример конфигурации pre-v0.45.0**

```toml
[cmd_duration]
prefix = "took "
```

**Пример конфигурации v0.45.0**

```toml
[cmd_duration]
# $duration – The command duration (e.g. "15s")
# $style    – The default style of the module (e.g. "bold yellow")
format = "took [$duration]($style) "
```

### Затронутые модули

#### Символ

| Удалено                 | Замена           |
| ----------------------- | ---------------- |
| `symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Изменения в конфигурации по умолчанию**

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

Ранее свойство `use_symbol_for_status` использовалось для настройки промпта на отображение `error_symbol`, когда результатом выполнения последней команды был ненулевой код состояния.

С выходом v0.45.0 мы теперь всегда используем `error_symbol` после ненулевых кодов состояния, объединяя свойства `use_symbol_for_status` и `error_symbol`.

Чтобы настроить промпт на использование более старой конфигурации `use_symbol_for_status = true`, добавьте в свой конфигурационный файл следующее:

```toml
[character]
error_symbol = "[✖](bold red)"
```

_Примечание:_ После элемента `character` автоматически добавляется пробел, поэтому, в отличие от других строк `format`, мы специально не добавляем его в приведенных выше примерах.

#### Длительность команды

| Удалено  | Замена   |
| -------- | -------- |
| `prefix` | `format` |

**Изменения в конфигурации по умолчанию**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Каталог

| Удалено  | Замена   |
| -------- | -------- |
| `prefix` | `format` |

**Изменения в конфигурации по умолчанию**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Переменная Окружения

| Удалено  | Замена   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**Изменения в конфигурации по умолчанию**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) " 
```

#### Коммит Git

| Удалено  | Замена   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**Изменения в конфигурации по умолчанию**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Статус Git

| Удалено           | Замена   |
| ----------------- | -------- |
| `prefix`          | `format` |
| `suffix`          | `format` |
| `show_sync_count` | `format` |

**Изменения в конфигурации по умолчанию**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Ранее свойство `show_sync_count` использовалось для настройки промпта на отображение количества коммитов, которые ветвь выполняла перед удаленной ветвью или позади нее.

С выпуском v0.45.0 это было заменено тремя отдельными свойствами: `ahead`, `behind` и `diverged`.

Чтобы настроить промпт на использование более старой конфигурации `show_sync_count = true`, задайте в вашем конфигурационном файле следующее:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Имя хоста

| Удалено  | Замена   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**Изменения в конфигурации по умолчанию**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| Удалено  | Замена   |
| -------- | -------- |
| `label`  | `format` |
| `prefix` | `format` |
| `suffix` | `format` |

**Изменения в конфигурации по умолчанию**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Время

| Удалено  | Замена        |
| -------- | ------------- |
| `format` | `time_format` |

**Изменения в конфигурации по умолчанию**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Пользовательские команды

| Удалено  | Замена   |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**Изменения в конфигурации по умолчанию**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
