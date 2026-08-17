# v0.45.0 нұсқасына көшу

Starship v0.45.0 шығарылымы үлкен v1.0.0 нұсқасына дайындық ретінде бұрынғы үйлесімділікті бұзатын өзгерістерді (breaking changes) қамтиды. Баптауларды одан әрі икемді ету үшін промптты конфигурациялау тәсіліне үлкен өзгерістер енгізілді.

Бұл нұсқаулық сізді осы өзгерістермен таныстыруға және баптауларыңызды жаңартуға арналған.

## `prompt_order` параметрi түбірлік `format` параметрімен ауыстырылды

v0.45.0 нұсқасына дейін `prompt_order` Starship көрсететін модульдер атауларының реттелген массивін қабылдайтын.

Starship v0.45.0 нұсқасында оның орнына модульдерден тыс промптты еркін пішімдеуге мүмкіндік беретін `format` мәні қолданылады.

**v0.45.0-ге дейінгі конфигурация мысалы**

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

**v0.45.0 нұсқасындағы конфигурация мысалы**

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

## Модульдердегі `prefix` және `suffix` параметрлері `format` арқылы ауыстырылды

v0.45.0 нұсқасына дейін кейбір модульдер өздерінің визуалды көрінісін өзгерту үшін `prefix` және/немесе `suffix` қабылдайтын.

Starship v0.45.0 оның орнына модульдердің қалай бейнеленетінін толық баптауға мүмкіндік беретін `format` мәнін қабылдайды. Мәнмәтіндік айнымалыларға префикс пен суффикс анықтаудың орнына, енді айнымалыларды модульдің шығысын білдіретін пішімдеу жолының ішіне тікелей қоюға болады.

**v0.45.0-ге дейінгі конфигурация мысалы**

```toml
[cmd_duration]
prefix = "took "
```

**v0.45.0 нұсқасындағы конфигурация мысалы**

```toml
[cmd_duration]
# $duration – Пәрменнің орындалу ұзақтығы (мысалы, "15s")
# $style    – Модульдің әдепкі стилі (мысалы, "bold yellow")
format = "took [$duration]($style) "
```

### Өзгеріске ұшыраған модульдер

#### Character

| Жойылған қасиет        | Оның орнына      |
| ---------------------- | ---------------- |
| `symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Әдепкі конфигурациядағы өзгерістер**

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

Бұрын `use_symbol_for_status` қасиеті соңғы пәрмен нөлден басқа қате кодымен аяқталғанда `error_symbol` көрсету үшін қолданылатын.

v0.45.0 нұсқасынан бастап, біз нөлден өзге қате кодтарынан кейін әрқашан `error_symbol` қолданамыз, бұл `use_symbol_for_status` және `error_symbol` қасиеттерін біріктірді.

Промптты бұрынғы `use_symbol_for_status = true` тәрізді жұмыс істету үшін конфигурация файлыңызға келесіні қосыңыз:

```toml
[character]
error_symbol = "[✖](bold red)"
```

_Ескертпе:_ `character` элементі өзінен кейін автоматты түрде бос орын қосады, сондықтан басқа `format` жолдарынан айырмашылығы, біз жоғарыдағы мысалдарда соңына бос орын қоспаймыз.

#### Command Duration

| Жойылған қасиет  | Оның орнына |
| ---------------- | ----------- |
| `prefix`         | `format`    |

**Әдепкі конфигурациядағы өзгерістер**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Directory

| Жойылған қасиет  | Оның орнына |
| ---------------- | ----------- |
| `prefix`         | `format`    |

**Әдепкі конфигурациядағы өзгерістер**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Environment Variable

| Жойылған қасиет  | Оның орнына |
| ---------------- | ----------- |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Әдепкі конфигурациядағы өзгерістер**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git Commit

| Жойылған қасиет  | Оның орнына |
| ---------------- | ----------- |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Әдепкі конфигурациядағы өзгерістер**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git Status

| Жойылған қасиет   | Оның орнына |
| ----------------- | ----------- |
| `prefix`          | `format`    |
| `suffix`          | `format`    |
| `show_sync_count` | `format`    |

**Әдепкі конфигурациядағы өзгерістер**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Бұрын `show_sync_count` қасиеті тармақтың қашықтағы тармақтан неше коммит алда немесе артта екенін көрсету үшін пайдаланылатын.

v0.45.0 шығарылымында бұл үш бөлек қасиетпен ауыстырылды: `ahead`, `behind` және `diverged`.

Бұрынғы `show_sync_count = true` баптауын пайдалану үшін конфигурация файлыңызға келесіні орнатыңыз:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

| Жойылған қасиет  | Оның орнына |
| ---------------- | ----------- |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Әдепкі конфигурациядағы өзгерістер**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| Жойылған қасиет  | Оның орнына |
| ---------------- | ----------- |
| `label`          | `format`    |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Әдепкі конфигурациядағы өзгерістер**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Time

| Жойылған қасиет  | Оның орнына   |
| ---------------- | ------------- |
| `format`         | `time_format` |

**Әдепкі конфигурациядағы өзгерістер**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Custom Commands

| Жойылған қасиет  | Оның орнына |
| ---------------- | ----------- |
| `prefix`         | `format`    |
| `suffix`         | `format`    |

**Әдепкі конфигурациядағы өзгерістер**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
