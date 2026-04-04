# Розширені налаштування

Хоча Starship і універсальна оболонка, іноді необхідно зробити більше ніж просто змінити `star.toml`, щоб можна було робити певні речі. Ця сторінка містить деякі з найбільш докладних методів налаштувань, які використовуються у starship.

> [!WARNING] Конфігурації в цьому розділі можуть бути змінені в майбутніх випусках Starship.

## TransientPrompt у PowerShell

Можна замінити попередній командний рядок на власний. Це корисно у випадках, коли вся інформація у ньому не завжди потрібна. Щоб увімкнути це, запустіть `Enable-TransientPrompt` в сеансі оболонки. Щоб зробити цю поведінку постійною, додайте цю команду у ваш `$PROFILE`. Перехідність можна вимкнути на льоту за допомогою `Disable-TransientPrompt`.

Типово, ліва частина вводу буде замінена на `>`. Щоб налаштувати це, створіть нову функцію з назвою `Invoke-Starship-TransientFunction`. Наприклад, щоб показати тут модуль Starship `character`, вам потрібно

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt та TransientRightPrompt в Cmd

Clink дозволяє замінювати попередньо надрукований командний рядок іншим рядком. Це корисно у випадках, коли вся інформація з командного рядка не потрібна. Щоб увімкнути це, виконайте `clink set prompt.transient <value>`, де \<value\> може бути одним з:

- `always`: завжди замінює попередній командний рядок
- `same_dir`: замінює попередній командний рядок тільки якщо робоча тека не змінювалась
- `off`: не змінює командний рядок (тобто функцію вимкнено)

Це треба зробити лише один раз. Зробіть наступні зміни у `starship.lua`, щоб налаштувати, що показується ліворуч і праворуч:

- Типово, ліва частина вводу буде замінена на `>`. Щоб налаштувати це, створіть нову функцію з назвою `starship_transient_prompt_func`. Ця функція отримує поточний текст командного рядка, з яким ви зможете працювати. Наприклад, щоб показати тут модуль Starship `character`, вам потрібно

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- Типово, права частина вводу є порожньою. Щоб кастомізувати її, створіть нову функцію з назвою `starship_transient_rprompt_func`. Ця функція отримує поточний текст командного рядка, з яким ви зможете працювати. Наприклад, щоб показати час, коли була запущена остання команда, ви можете зробити

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt та TransientRightPrompt у Fish

Можна замінити попередньо надрукований командний рядок на власний. Це корисно у випадках, коли вся інформація з командного рядка не потрібна. Щоб увімкнути це, запустіть `enable_transience` в сеансі оболонки. Щоб зробити цю змінну постійною, додайте цей вираз до ваших налаштувань `~/.config/fish/config.fish`. Перехідність може бути вимкнена за допомогою `disable_transience`.

Зверніть увагу, що у випадку Fish, перехідний командний рядок буде надруковано лише тоді, коли командний рядок не порожній та синтаксично правильний.

- Типово, ліва частина вводу буде замінена на зелений символ `❯`. Щоб кастомізувати її, створіть нову функцію з назвою `starship_transient_prompt_func`. Наприклад, щоб показати тут модуль Starship `character`, вам потрібно

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- Типово, права частина вводу є порожньою. Щоб кастомізувати її, створіть нову функцію з назвою `starship_transient_rprompt_func`. Наприклад, щоб показати час, коли була запущена остання команда, ви можете зробити

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt та TransientRightPrompt в Bash

[Ble.sh](https://github.com/akinomyoga/ble.sh) v0.4 або вище дозволяє замінювати попередньо надрукований командний рядок іншим рядком. Це корисно у випадках, коли вся інформація з командного рядка не потрібна. Для увімкнення цього додайте до `~/.bashrc` рядок `bleopt prompt_ps1_transient=<value>`:

\<value\> тут  – це розділений двокрапкою список `always`, `same-dir` та `trim`. Якщо `prompt_ps1_final` порожній і параметр `prompt_ps1_transient` має не пусте значення \<value\>, командний рядок, вказаний у `PS1` буде стертий при виході з поточного командного рядка. Якщо \<value\> містить поле `trim`, тільки останній рядок багаторядкового `PS1` буде збережений, а інші вилучені. В іншому випадку командний рядок буде встановлено перестворено, якщо вказано `PS1=`. Коли поле `same-dir` міститься у \<value\> та поточна тека є відмінною від останньої теки у попередньому виводі командного рядка, параметр `prompt_ps1_transient` не враховується.

Зробіть наступні зміни у `~/.blerc` (або у `~/.config/blesh/init.sh`), щоб налаштувати, що показується ліворуч і праворуч:

- Для налаштування того, чим замінюється ліва частина вводу, налаштуйте параметр `prompt_ps1_final`. Наприклад, щоб показати тут модуль Starship `character`, вам потрібно

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- Для налаштування того, чим замінюється права частина вводу, налаштуйте параметр `prompt_rps1_final`. Наприклад, щоб показати час, коли була запущена остання команда, ви можете зробити

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Власні команди pre-prompt та pre-execution в Cmd

Clink забезпечує надзвичайно гнучкий API для виконання команд pre-prompt і pre-exec в Cmd. Його досить просто використовувати в Starship. Зробіть наступні зміни у вашому `starship.lua` відповідно до ваших вимог:

- Для запуску власних функцій прямо перед виводом командного рядка, визначте нову функцію з назвою `starship_preprompt_user_func`. Ця функція отримує поточний текст командного рядка, з яким ви зможете працювати. Наприклад, щоб показати ракету перед командним рядком, ви можете зробити наступне

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- Для запуску власних функцій прямо перед виконанням команди, визначте нову функцію з назвою `starship_precmd_user_func`. Ця функція отримує поточний текст команди, з яким ви зможете працювати. Наприклад, для виводу команди, яка буде виконана, вам треба зробити

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Власні команди pre-prompt та pre-execution в Bash

Bash не має офіційної системи preexec/precmd, як більшість інших оболонок. Через це важко забезпечити повністю настроювані гачки для цього в `bash`. Однак, Starship дає можливість вставити свої власні функції в процедуру виводу командного рядка:

- Для запуску власних функцій прямо перед виводом командного рядка, визначте нову функцію з назвою `starship_precmd_user_func`. Наприклад, щоб показати ракету перед командним рядком, ви можете зробити наступне

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Щоб запустити власну функцію прямо перед запуском команди, можна використати механізм [`DEBUG` trap](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Однак, ви **повинні** перехопити сигнал DEBUG _перед_ ініціалізацією Starship! Starship може зберегти значення перехоплення DEBUG, але якщо перехоплення буде перезаписане після початку запуску, деякі функції не працюватимуть.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Власні команди pre-prompt та pre-execution в PowerShell

PowerShell не має офіційної системи preexec/precmd, як більшість інших оболонок. Через це важко забезпечити повністю настроювані гачки для цього в `powershell`. Однак, Starship дає можливість вставити свої власні функції в процедуру виводу командного рядка:

Створіть функцію з назвою `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Зміна заголовка вікна

В деяких оболонках командний рядок автоматично змінює заголовок вікна (наприклад, на назву поточної теки). У Fish це стандартна поведінка. Starship цього не робить, але це досить просто зробити у `bash`, `zsh`, `cmd` або `powershell`.

Спочатку визначте функцію зміни заголовка вікна (в bash і zsh – однаково):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

Ви можете використовувати змінні для налаштування цього заголовка (`$USER`, `$HOSTNAME`, та `$PWD` – є досить популярними).

У `bash` встановіть цю функцію як функцію precmd у starship:

```bash
starship_precmd_user_func="set_win_title"
```

У `zsh` додайте наступне до масиву `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

Якщо вам подобається результат, додайте ці рядки до файлу налаштування оболонки (`~/.bashrc` чи `~/.zshrc`), щоб зробити його постійним.

Наприклад, якщо ви хочете показати вашу поточну теку у заголовку вкладці термінала, додайте наступний сніпет до вашого `~/.bashrc` або `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

Для Cmd, ви можете змінити заголовок вікна за допомогою функції `starship_preprompt_user_func`.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

Ви також можете встановити подібний вивід у PowerShell, створивши функцію з назвою `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Командний рядок праворуч

Деякі оболонки підтримують вивід командного рядка праворуч. Starship може встановити вміст правої частини командного рядка за допомогою параметра `right_format`. Будь-який модуль, який можна використовувати у `format`, також підтримується у `right_format`. Змінна `$all` міститиме лише модулі, які явно не використовується, а ні в `format`, а ні в `right_format`.

Примітка: командний рядок праворуч – це один рядок, що знаходиться праворуч у рядку вводу. Щоб вирівняти модулі праворуч над рядком введення в багаторядковому запиті, перегляньте [модуль `fill`](../config/#fill).

`right_format` наразі підтримується для таких оболонок: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Примітка: фреймворк [Ble.sh](https://github.com/akinomyoga/ble.sh) v0.4 або вище має бути встановлений для того, щоб використовувати розташування командного рядка в bash праворуч.

### Приклад

```toml
# ~/.config/starship.toml

# Мінімум ліворуч
format = """$character"""

# решту командного рядка перенесемо праворуч
right_format = """$all"""
```

Отримаємо командний рядок наступного виду:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Подовжений ввід

Деякі оболонки підтримують подовження вводу так само як і  звичайний ввід в командний рядок. Такий ввід буде показаний замість звичайного, колі користувач ввів символ продовження вводу (наприклад одну ліву дужку чи лапку).

У Starship можна встановити показ продовженого вводу за допомогою параметра `continuation_prompt`. Стандартний командний рядок — `'[∙](bright-black) '`.

Примітка: `continuation_prompt` слід встановити на літеральний рядок без жодних змінних.

Примітка: Подовжений ввід доступний лише для наступних оболонок:

- `bash`
- `zsh`
- `PowerShell`

### Приклад

```toml
# ~/.config/starship.toml

# Подовжений ввід позначається двома стрілками
continuation_prompt = '▶▶ '
```

## Statusline for Claude Code

Starship supports displaying a custom statusline when running inside Claude Code, Anthropic's CLI tool for interactive coding with Claude. This statusline provides real-time information about your Claude session, including the model being used, context window usage, and session costs.

For more information about the Claude Code statusline feature, see the [Claude Code statusline documentation](https://code.claude.com/docs/en/statusline).

### Setup

To use Starship as your Claude Code statusline:

1. Run `/statusline` in Claude Code and ask it to configure Starship, or manually add the following to your `.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "starship statusline claude-code"
  }
}
```

2. Customize the statusline appearance in your `~/.config/starship.toml` (see [Configuration](#configuration) below)

### Overview

When invoked with `starship statusline claude-code`, Starship receives Claude Code session data via stdin and renders a statusline using a dedicated profile named `claude-code`.

The profile includes three specialized modules:

- `claude_model`: Displays the current Claude model being used
- `claude_context`: Shows context window usage with a visual gauge
- `claude_cost`: Displays session cost and statistics

The default profile format is:

```toml
[profiles]
claude-code = "$claude_model$git_branch$claude_context$claude_cost"
```

### Налаштування

You can customize the Claude Code statusline by modifying the `claude-code` profile and individual module configurations in your `~/.config/starship.toml`:

```toml
# ~/.config/starship.toml

# Customize the claude-code profile
[profiles]
claude-code = "$claude_model$claude_context$claude_cost"

# Configure individual modules
[claude_model]
format = "[$symbol$model]($style) "
symbol = "🤖 "
style = "bold blue"

[claude_context]
format = "[$gauge $percentage]($style) "
gauge_width = 10

[claude_cost]
format = "[$symbol$cost]($style) "
symbol = "💰 "
```

### Claude Model

The `claude_model` module displays the current Claude model being used in the session.

#### Параметри

| Параметр        | Стандартно                   | Опис                                                                                      |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------------------- |
| `format`        | `'[$symbol$model]($style) '` | Формат модуля.                                                                            |
| `symbol`        | `'🤖 '`                       | The symbol shown before the model name.                                                   |
| `style`         | `'bold blue'`                | Стиль модуля.                                                                             |
| `model_aliases` | `{}`                         | Map of model IDs or display names to shorter aliases. Checks ID first, then display name. |
| `disabled`      | `false`                      | Disables the `claude_model` module.                                                       |

#### Змінні

| Змінна    | Приклад             | Опис                                     |
| --------- | ------------------- | ---------------------------------------- |
| model     | `Claude 3.5 Sonnet` | The display name of the current model    |
| model_id  | `claude-3-5-sonnet` | The model ID                             |
| symbol    |                     | Віддзеркалює значення параметра `symbol` |
| style\* |                     | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

#### Приклади

```toml
# ~/.config/starship.toml

# Basic customization
[claude_model]
format = "on [$symbol$model]($style) "
symbol = "🧠 "
style = "bold cyan"

# Using model aliases for vendor-specific model names
# You can alias by model ID or display name
[claude_model.model_aliases]
# Alias by vendor model ID (e.g. AWS Bedrock)
"global.anthropic.claude-sonnet-4-5-20250929-v1:0" = "Sonnet 4.5"
# Alias by display name
"Claude Sonnet 4.5 (Vendor Proxy)" = "Sonnet"
```

### Claude Context

The `claude_context` module displays context window usage as a percentage and visual gauge. The style automatically changes based on configurable thresholds.

#### Параметри

| Параметр               | Стандартно                        | Опис                                               |
| ---------------------- | --------------------------------- | -------------------------------------------------- |
| `format`               | `'[$gauge $percentage]($style) '` | Формат модуля.                                     |
| `symbol`               | `''`                              | The symbol shown before the gauge.                 |
| `gauge_width`          | `5`                               | The width of the gauge in characters.              |
| `gauge_full_symbol`    | `'█'`                             | The symbol used for filled segments of the gauge.  |
| `gauge_partial_symbol` | `'▒'`                             | The symbol used for partial segments of the gauge. |
| `gauge_empty_symbol`   | `'░'`                             | The symbol used for empty segments of the gauge.   |
| `display`              | [дивіться нижче](#display)        | Threshold and style configurations.                |
| `disabled`             | `false`                           | Disables the `claude_context` module.              |

##### Display

The `display` option is an array of objects that define thresholds and styles for different usage levels. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Параметр    | Стандартно   | Опис                                                                     |
| ----------- | ------------ | ------------------------------------------------------------------------ |
| `threshold` | `0.0`        | The minimum context windows usage percentage to match this configuration |
| `style`     | `bold green` | The value of `style` if this display configuration is matched            |
| `hidden`    | `false`      | Hide this module if this the configuration is matched.                   |

```toml
[[claude_context.display]]
threshold = 0
hidden = true

[[claude_context.display]]
threshold = 30
style = "bold green"

[[claude_context.display]]
threshold = 60
style = "bold yellow"

[[claude_context.display]]
threshold = 80
style = "bold red"
```

#### Змінні

| Змінна                       | Приклад | Опис                                                  |
| ---------------------------- | ------- | ----------------------------------------------------- |
| gauge                        | `██▒░░` | Visual representation of context usage                |
| percentage                   | `65%`   | Context usage as a percentage                         |
| input_tokens                 | `45.2k` | Total input tokens in conversation                    |
| output_tokens                | `12.3k` | Total output tokens in conversation                   |
| curr_input_tokens          | `5.1k`  | Input tokens from most recent API call                |
| curr_output_tokens         | `1.2k`  | Output tokens from most recent API call               |
| curr_cache_creation_tokens | `1.5k`  | Cache creation tokens from most recent API call       |
| curr_cache_read_tokens     | `23.4k` | Cache read tokens from most recent API call           |
| total_tokens                 | `200k`  | Total context window size                             |
| symbol                       |         | Віддзеркалює значення параметра `symbol`              |
| style\*                    |         | Mirrors the style from the matching display threshold |

\*: Ця змінна може бути використана лише як частина стилю рядка

#### Приклади

**Minimal gauge-only display**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$gauge]($style) "
gauge_width = 10
```

**Detailed token information**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$percentage ($input_tokens in / $output_tokens out)]($style) "
```

**Custom gauge symbols**

```toml
# ~/.config/starship.toml

[claude_context]
gauge_full_symbol = "▰"
gauge_partial_symbol = ""
gauge_empty_symbol = "▱"
gauge_width = 10
format = "[$gauge]($style) "
```

**Custom thresholds**

```toml
# ~/.config/starship.toml

[[claude_context.display]]
threshold = 0
style = "bold green"

[[claude_context.display]]
threshold = 50
style = "bold yellow"

[[claude_context.display]]
threshold = 75
style = "bold orange"

[[claude_context.display]]
threshold = 90
style = "bold red"
```

### Claude Cost

The `claude_cost` module displays the total cost of the current Claude Code session in USD. Like `claude_context`, it supports threshold-based styling.

#### Параметри

| Параметр   | Стандартно                         | Опис                                |
| ---------- | ---------------------------------- | ----------------------------------- |
| `format`   | `'[$symbol(\\$$cost)]($style) '` | Формат модуля.                      |
| `symbol`   | `'💰 '`                             | The symbol shown before the cost.   |
| `display`  | [дивіться нижче](#display-1)       | Threshold and style configurations. |
| `disabled` | `false`                            | Disables the `claude_cost` module.  |

##### Display

The `display` option is an array of objects that define cost thresholds and styles. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Параметр    | Стандартно   | Опис                                                          |
| ----------- | ------------ | ------------------------------------------------------------- |
| `threshold` | `0.0`        | The minimum cost in USD to match this configuration           |
| `style`     | `bold green` | The value of `style` if this display configuration is matched |
| `hidden`    | `false`      | Hide this module if this configuration is matched.            |

**Default configuration:**

```toml
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 1.0
style = "bold yellow"

[[claude_cost.display]]
threshold = 5.0
style = "bold red"
```

#### Змінні

| Змінна        | Приклад  | Опис                                                  |
| ------------- | -------- | ----------------------------------------------------- |
| cost          | `1.23`   | Total session cost in USD (formatted to 2 decimals)   |
| duration      | `1m 30s` | Total session duration                                |
| api_duration  | `45s`    | Total API call duration                               |
| lines_added   | `1.2k`   | Total lines of code added                             |
| lines_removed | `500`    | Total lines of code removed                           |
| symbol        |          | Віддзеркалює значення параметра `symbol`              |
| style\*     |          | Mirrors the style from the matching display threshold |

\*: Ця змінна може бути використана лише як частина стилю рядка

#### Приклади

```toml
# ~/.config/starship.toml

# Cost with code change statistics
[claude_cost]
format = "[$symbol$cost (+$lines_added -$lines_removed)]($style) "

# Hide module until cost exceeds $0.10
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 0.10
style = "bold yellow"

[[claude_cost.display]]
threshold = 2.0
style = "bold red"

# Show duration information
[claude_cost]
format = "[$symbol$cost ($duration)]($style) "
```

## Рядки стилів

Рядки стилю — список слів, розділених пробілами. Слова не чутливі до регістру (наприклад `bold` і `BoLd` вважаються однаковими). Кожне слово може бути одним з наступних:

- `bold`
- `italic`
- `underline`
- `dimmed`
- `inverted`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

де `<color>` є специфікацією кольору (обговорюється нижче). `fg:<color>` та `<color>` на цей час роблять те саме, хоча це може змінитися в майбутньому. `<color>` може також мати значення `prev_fg` або `prev_bg`, які обчислюють колір тексту або фону попереднього елемента відповідно, якщо він доступний, або `none` у протилежному випадку. `inverted` замінює кольори тла і тексту. Порядок слів у рядку не має значення.

Токен `none` перевизначає всі інші токени у рядку, якщо він не є частиною `bg:`, так `fg:red none fg:blue` все одно створить рядок без стилізування. `bg:none` встановлює типовий колір фону, таким чином, `fg:red bg:none` еквівалентно `red` або `fg:red`, а `bg:green fg:red fg:none` також еквівалентно `fg:red` або `red`. Використання `none` у поєднанні з іншими токенами в майбутньому може стати помилкою.

Визначення кольору може бути одним з наступних:

- Один за стандартних кольорів термінала: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Ви можете опціонально додавати префікс `bright-`, щоб отримати яскраву версію (наприклад, `bright-white`).
- `#` за яким йде шестизначний шістнадцятковий код кольору. Цей код вказує на [шістнадцятковий код RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Число від 0-255. Число визначає [8-бітний код кольору ANSI](https://i.stack.imgur.com/KTSQa.png).

Якщо для тексту та фону задано кілька кольорів, останній в рядку буде мати вищий пріоритет.

Не кожен рядок стилю буде правильно показуватись у кожному терміналі. Зокрема, існують такі відомі примхи:

- Багато терміналів стандартно вмикають підтримку `blink`.
- `hidden` [не підтримується в iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` не підтримується стандартно в macOS Terminal.app.
