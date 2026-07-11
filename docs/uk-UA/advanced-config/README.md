# Розширені налаштування

While Starship is a versatile shell, sometimes you need to do more than edit
`starship.toml` to get it to do certain things. Ця сторінка містить деякі з найбільш
докладних методів налаштувань, які використовуються у starship.

> [!WARNING] Конфігурації в цьому розділі можуть бути змінені в майбутніх випусках Starship.

## TransientPrompt у PowerShell

Можна замінити попередній командний рядок на власний. Це корисно у випадках, коли вся інформація у ньому не завжди потрібна. To enable
this, run `Enable-TransientPrompt` in the shell session. To make it permanent, put
this statement in your `$PROFILE`. Transience can be disabled on-the-fly with
`Disable-TransientPrompt`.

Типово, ліва частина вводу буде замінена на `>`. To customize this,
define a new function called `Invoke-Starship-TransientFunction`. For example, to
display Starship's `character` module here, you would do

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt та TransientRightPrompt в Cmd

Clink дозволяє замінювати попередньо надрукований командний рядок іншим рядком. Це корисно у випадках, коли вся інформація у ньому не завжди потрібна. To enable
this, run `clink set prompt.transient <value>` where \<value\> can be one of:

- `always`: завжди замінює попередній командний рядок
- `same_dir`: замінює попередній командний рядок тільки якщо робоча тека не змінювалась
- `off`: не змінює командний рядок (тобто функцію вимкнено)

Це треба зробити лише один раз. Make the following changes to your `starship.lua`
to customize what gets displayed on the left and on the right:

- Типово, ліва частина вводу буде замінена на `>`. To customize this,
  define a new function called `starship_transient_prompt_func`. This function
  receives the current prompt as a string that you can utilize. For example, to
  display Starship's `character` module here, you would do

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- Типово, права частина вводу є порожньою. To customize this, define a new
  function called `starship_transient_rprompt_func`. This function receives the
  current prompt as a string that you can utilize. For example, to display
  the time at which the last command was started here, you would do

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt та TransientRightPrompt у Fish

Можна замінити попередній командний рядок на власний. Це корисно у випадках, коли вся інформація у ньому не завжди потрібна. To enable
this, run `enable_transience` in the shell session. To make it permanent, put
this statement in your `~/.config/fish/config.fish`. Transience can be disabled on-the-fly with
`disable_transience`.

Зверніть увагу, що у випадку Fish, перехідний командний рядок буде надруковано лише тоді, коли командний рядок не порожній та синтаксично правильний.

- Типово, ліва частина вводу буде замінена на зелений символ `❯`. To customize this,
  define a new function called `starship_transient_prompt_func`. For example, to
  display Starship's `character` module here, you would do

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- Типово, права частина вводу є порожньою. To customize this, define a new
  function called `starship_transient_rprompt_func`. For example, to display
  the time at which the last command was started here, you would do

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt та TransientRightPrompt в Bash

The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework at v0.4 or higher allows you to replace
the previous-printed prompt with custom strings. Це корисно у випадках, коли вся інформація з командного рядка не потрібна. To enable this, put this in `~/.bashrc`
`bleopt prompt_ps1_transient=<value>`:

The \<value\> here is a colon-separated list of `always`, `same-dir` and `trim`.
When `prompt_ps1_final` is empty and the option `prompt_ps1_transient` has a non-empty \<value\>,
the prompt specified by `PS1` is erased on leaving the current command line.
If \<value\> contains a field `trim`, only the last line of multiline `PS1` is
preserved and the other lines are erased. Otherwise, the command line will be
redrawn as if `PS1=` is specified. When a field `same-dir` is contained in
\<value\> and the current working directory is different from the final directory of
the previous command line, this option `prompt_ps1_transient` is ignored.

Зробіть наступні зміни у `~/.blerc` (або у `~/.config/blesh/init.sh`), щоб налаштувати, що показується ліворуч і праворуч:

- To customize what the left side of input gets replaced with, configure the
  `prompt_ps1_final` Ble.sh option. For example, to display Starship's `character`
  module here, you would do

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- To customize what the right side of input gets replaced with, configure the
  `prompt_rps1_final` Ble.sh option. For example, to display
  the time at which the last command was started here, you would do

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Власні команди pre-prompt та pre-execution в Cmd

Clink забезпечує надзвичайно гнучкий API для виконання команд pre-prompt і pre-exec в Cmd. Його досить просто використовувати в Starship. Make the following changes
to your `starship.lua` file as per your requirements:

- To run a custom function right before the prompt is drawn, define a new
  function called `starship_preprompt_user_func`. This function receives
  the current prompt as a string that you can utilize. For example, to
  draw a rocket before the prompt, you would do

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- To run a custom function right before a command is executed, define a new
  function called `starship_precmd_user_func`. This function receives
  the current commandline as a string that you can utilize. For example, to
  print the command that's about to be executed, you would do

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Власні команди pre-prompt та pre-execution в Bash

Bash не має офіційної системи preexec/precmd, як більшість інших оболонок.
Because of this, it is difficult to provide fully customizable hooks in `bash`.
Однак, Starship дає можливість вставити свої власні функції в процедуру виводу командного рядка:

- To run a custom function right before the prompt is drawn, define a new
  function and then assign its name to `starship_precmd_user_func`. For example,
  to draw a rocket before the prompt, you would do

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the
  [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/).
  However, you **must** trap the DEBUG signal _before_ initializing Starship!
  Starship can preserve the value of the DEBUG trap, but if the trap is overwritten
  after starship starts up, some functionality will break.

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

PowerShell не має офіційної системи preexec/precmd, як більшість інших оболонок.
Because of this, it is difficult to provide fully customizable hooks in `powershell`.
Однак, Starship дає можливість вставити свої власні функції в процедуру виводу командного рядка:

Створіть функцію з назвою `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Зміна заголовка вікна

В деяких оболонках командний рядок автоматично змінює заголовок вікна (наприклад, на назву поточної теки). У Fish це стандартна поведінка.
Starship does not do this, but it's fairly straightforward to add this
functionality to `bash`, `zsh`, `cmd` or `powershell`.

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

## Командний рядок праворуч {#enable-right-prompt}

Деякі оболонки підтримують вивід командного рядка праворуч. Starship can
set the content of the right prompt using the `right_format` option. Any module that can be used
in `format` is also supported in `right_format`. The `$all` variable will only contain modules
not explicitly used in either `format` or `right_format`.

Примітка: командний рядок праворуч – це один рядок, що знаходиться праворуч у рядку вводу. To right align modules above
the input line in a multi-line prompt, see the [`fill` module](../config/#fill).

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

When using `zsh` (v5.0.5+), the shell adds a default trailing space to the right prompt. This can cause alignment issues specifically when using the Starship `$fill` module. To remove this gap, add the following to your `.zshrc`:

```zsh
ZLE_RPROMPT_INDENT=0
```

## Подовжений ввід

Деякі оболонки підтримують подовження вводу так само як і  звичайний ввід в командний рядок. Такий ввід буде показаний замість звичайного, колі користувач ввів символ продовження вводу (наприклад одну ліву дужку чи лапку).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `'[∙](bright-black) '`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Примітка: Подовжений ввід доступний лише для наступних оболонок:

- `bash`
- `zsh`
- `PowerShell`

### Рядок стану для Claude Code

```toml
# ~/.config/starship.toml

# Подовжений ввід позначається двома стрілками
continuation_prompt = '▶▶ '
```

## Рядок стану для Claude Code

Starship підтримує показ власного рядка стану під час роботи в Claude Code — інструменті командного рядка від Anthropic, призначеному для інтерактивного програмування за допомогою Claude. Ця рядок стану надає інформацію в режимі реального часу про вашу сесію Claude, зокрема про використовувану модель, використання вікна контексту та вартість сесії.

Щоб використовувати Starship як рядок стану Claude Code:

### Встановлення

Щоб використовувати Starship як рядок стану Claude Code:

1. Налаштуйте вигляд рядка стану у файлі `~/.config/starship.toml` (див. розділ [Налаштування](#configuration) нижче)

```json
{
  "statusLine": {
    "type": "command",
    "command": "starship statusline claude-code"
  }
}
```

2. Customize the statusline appearance in your `~/.config/starship.toml` (see [Configuration](#configuration) below)

### Огляд

When invoked with `starship statusline claude-code`, Starship receives Claude Code session data via stdin and renders a statusline using a dedicated profile named `claude-code`.

Типовий формат профілю:

- `claude_model`: Displays the current Claude model being used
- `claude_context`: Shows context window usage with a visual gauge
- `claude_cost`: Displays session cost and statistics

Типовий формат профілю:

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

#### Змінні

| Змінна          | Приклад                      | Опис                                                                                                                                                       |
| --------------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`        | `'[$symbol$model]($style) '` | Імʼя поточної моделі                                                                                                                                       |
| `символ`        | `'🤖 '`                      | ID моделі                                                                                                                                                  |
| `style`         | `'bold blue'`                | Віддзеркалює значення параметра `symbol`                                                                                                                   |
| `model_aliases` | `{}`                         | Звʼязує ідентифікатори моделей або їхні назви з коротшими псевдонімами. Спочатку перевіряється ідентифікатор, потім назва. |
| `disabled`      | `false`                      | Disables the `claude_model` module.                                                                                                        |

#### Змінні

| Змінна                        | Приклад             | Опис                                    |
| ----------------------------- | ------------------- | --------------------------------------- |
| model                         | `Claude 3.5 Sonnet` | Імʼя поточної моделі                    |
| model_id | `claude-3-5-sonnet` | ID моделі                               |
| symbol                        |                     | Віддзеркалює значення параметра `style` |
| style\*                       |                     | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

#### Приклади

```toml
# ~/.config/starship.toml

# Базові налаштування
[claude_model]
format = "on [$symbol$model]($style) "
symbol = "🧠 "
style = "bold cyan"

# Використання псевдонімів для моделей із власними назвами постачальників
# Аліаси можна створити за ідентифікатором моделі або за її назвою
[claude_model.model_aliases]
# Аліас за ID моделі від постачальника (напр. AWS Bedrock)
"global.anthropic.claude-sonnet-4-5-20250929-v1:0" = "Sonnet 4.5"
# Аліас за назвою
"Claude Sonnet 4.5 (Vendor Proxy)" = "Sonnet"
```

### Claude Context

The `claude_context` module displays context window usage as a percentage and visual gauge. Стиль автоматично змінюється на основі налаштованих порогів.

#### Параметри

| Параметр               | Стандартно                        | Опис                                                                                        |
| ---------------------- | --------------------------------- | ------------------------------------------------------------------------------------------- |
| `format`               | `'[$gauge $percentage]($style) '` | Формат модуля.                                                              |
| `символ`               | `''`                              | Символ, який показується перед індикатором.                                 |
| `gauge_width`          | `5`                               | Ширина індикатора в символах.                                               |
| `gauge_full_symbol`    | `'█'`                             | Символ, який використовується для заповнених сегментів індикатора.          |
| `gauge_partial_symbol` | `'▒'`                             | Символ, який використовується для частково заповнених сегментів індикатора. |
| `gauge_empty_symbol`   | `'░'`                             | Символ, який використовується для не заповнених сегментів індикатора.       |
| `display`              | [see below](#display)             | Налаштування порогів та стилів.                                             |
| `disabled`             | `false`                           | Disables the `claude_context` module.                                       |

##### Display

The `display` option is an array of objects that define thresholds and styles for different usage levels. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Параметр    | Стандартно   | Опис                                                                                |
| ----------- | ------------ | ----------------------------------------------------------------------------------- |
| `threshold` | `0.0`        | Мінімальний відсоток використання контекстних вікон, що відповідає цій конфігурації |
| `style`     | `bold green` | The value of `style` if this display configuration is matched                       |
| `hidden`    | `false`      | Приховати цей модуль, якщо він збігається з конфігурацією.          |

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

| Змінна                                                                                    | Приклад | Опис                                                                      |
| ----------------------------------------------------------------------------------------- | ------- | ------------------------------------------------------------------------- |
| gauge                                                                                     | `██▒░░` | Візуальне представлення використання контексту                            |
| percentage                                                                                | `65%`   | Використання контексту у відсотках                                        |
| input_tokens                                                         | `45.2k` | Загальна кількість input-токенів, використаних у розмові                  |
| output_tokens                                                        | `12.3k` | Загальна кількість output-токенів, використаних у розмові                 |
| curr_input_tokens                               | `5.1k`  | Кількість input-токенів в останньому API-виклику                          |
| curr_output_tokens                              | `1.2k`  | Кількість output-токенів в останньому API-виклику                         |
| curr_cache_creation_tokens | `1.5k`  | Кеш з токенів створення з останнього виклику API                          |
| curr_cache_read_tokens     | `23.4k` | Кеш з токенів читання з останнього виклику API                            |
| total_tokens                                                         | `200k`  | Загальний розмір контекстного вікна                                       |
| symbol                                                                                    |         | Віддзеркалює значення параметра `style`                                   |
| style\*                                                                                   |         | Повторює стиль, що збігається з відповідним пороговим значення для показу |

**Тільки індикатор**

#### Приклади

**Докладні відомості про використання токенів**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$gauge]($style) "
gauge_width = 10
```

**Власні символи для індикатора**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$percentage ($input_tokens in / $output_tokens out)]($style) "
```

**Власні порогові значення**

```toml
# ~/.config/starship.toml

[claude_context]
gauge_full_symbol = "▰"
gauge_partial_symbol = ""
gauge_empty_symbol = "▱"
gauge_width = 10
format = "[$gauge]($style) "
```

**Власні порогові значення**

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

| Параметр   | Стандартно                         | Опис                                                         |
| ---------- | ---------------------------------- | ------------------------------------------------------------ |
| `format`   | `'[$symbol(\\$$cost)]($style) '` | Формат модуля.                               |
| `символ`   | `'💰 '`                            | Символ, який показується перед сумою витрат. |
| `display`  | [see below](#display-1)            | Налаштування порогів та стилів.              |
| `disabled` | `false`                            | Disables the `claude_cost` module.           |

##### Display

The `display` option is an array of objects that define cost thresholds and styles. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Параметр    | Стандартно   | Опис                                                                       |
| ----------- | ------------ | -------------------------------------------------------------------------- |
| `threshold` | `0.0`        | Мінімальна вартість у доларах США для такої конфігурації                   |
| `style`     | `bold green` | The value of `style` if this display configuration is matched              |
| `hidden`    | `false`      | Приховати цей модуль, якщо він збігається з конфігурацією. |

**Стандартні налаштування:**

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

| Змінна                             | Приклад  | Опис                                                                      |
| ---------------------------------- | -------- | ------------------------------------------------------------------------- |
| cost                               | `1.23`   | Загальна вартість сесії у доларах (з центами)          |
| duration                           | `1m 30s` | Загальна тривалість сеансу                                                |
| api_duration  | `45s`    | Загальна тривалість API-виклику                                           |
| lines_added   | `1.2k`   | Загальна кількість доданих рядків коду                                    |
| lines_removed | `500`    | Загальна кількість вилучених рядків коду                                  |
| symbol                             |          | Віддзеркалює значення параметра `style`                                   |
| style\*                            |          | Повторює стиль, що збігається з відповідним пороговим значення для показу |

\*: Ця змінна може бути використана лише як частина стилю рядка

#### Рядки стилів {#style-strings}

```toml
# ~/.config/starship.toml

# Витрати зі статистикою змін у коді
[claude_cost]
format = "[$symbol$cost (+$lines_added -$lines_removed)]($style) "

# Приховати модуль, доки вартість не перевищить $0,10
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 0.10
style = "bold yellow"

[[claude_cost.display]]
threshold = 2.0
style = "bold red"

# Показувати інформацію про тривалість роботи
[claude_cost]
format = "[$symbol$cost ($duration)]($style) "
```

## Рядки стилів

Рядки стилю — список слів, розділених пробілами. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Кожне слово може бути одним з наступних:

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

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future.
`<color>` can also be set to `prev_fg` or `prev_bg` which evaluates to the previous item's foreground or background color respectively if available or `none` otherwise.
`inverted` swaps the background and foreground colors. Порядок слів у рядку не має значення.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

Якщо для тексту та фону задано кілька кольорів, останній в рядку буде мати вищий пріоритет.

- One of the standard terminal colors: `black`, `red`, `green`, `blue`,
  `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these
  with `bright-` to get the bright version (e.g. `bright-white`).
- A `#` followed by a six-digit hexadecimal number. This specifies an
  [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Число від 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

Якщо для тексту та фону задано кілька кольорів, останній в рядку буде мати вищий пріоритет.

Не кожен рядок стилю буде правильно показуватись у кожному терміналі. Зокрема, існують такі відомі примхи:

- Many terminals disable support for `blink` by default.
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app.
