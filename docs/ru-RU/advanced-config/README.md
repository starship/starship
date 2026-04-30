# Расширенная конфигурация

Хотя Starship - это универсальная оболочка, иногда вам нужно сделать больше, чем просто редактировать `starship.toml`, для того чтобы сделать определенные вещи. Эта страница описывает некоторые из дополнительных техник конфигурации, используемых в Starship.

> [!WARNING] The configurations in this section are subject to change in future releases of Starship.

## TransientPrompt для PowerShell

Можно заменить предыдущий выведенный промпт на пользовательскую строку. Это полезно в тех случаях, когда весь промпт не всегда нужен. Чтобы включить, запустите `Enable-TransientPrompt` в сеансе оболочки. Чтобы включить его глобально, вставьте это в ваш `$PROFILE` Переход может быть отключён на лету с помощью `Disable-TransientPrompt`.

По умолчанию, левая сторона ввода заменяется на `>`. Для настройки определите новую функцию под названием `Invoke-Starship-TransientFunction`. Например, чтобы отобразить модуль Starship `character`, вы должны выполнить

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt и TransientRightPrompt в Cmd

Clink позволяет заменить предыдущий выведенный промпт на пользовательские строки. Это полезно в тех случаях, когда весь промпт не всегда нужен. Чтобы включить это, запустите `clink set prompt.transient <value>`, где \<value\> может быть одним из

- `always`: всегда заменять предыдущий промпт
- `same_dir`: заменять предыдущий промпт только если рабочая директория та же
- `off`: не заменять промпт (т.е. выключить переход)

Вы должны сделать это только один раз. Внесите следующие изменения в ваш `starship.lua`, чтобы настроить, что отображается слева и справа:

- По умолчанию, левая сторона ввода заменяется на `>`. Чтобы настроить это, определите новую функцию под названием `starship_transient_prompt_func`. Эта функция получает текущий промпт как строку, которую вы можете использовать. Например, чтобы отобразить здесь модуль Starship `character`, вы должны выполнить

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- По умолчанию, правая сторона ввода пуста. Для настройки определите новую функцию под названием `starship_transient_rprompt_func`. Эта функция получает текущий промпт как строку, которую вы можете использовать. Например, чтобы отобразить время, когда здесь была запущена последняя команда, вы должны выполнить

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt и TransientRightPrompt в Fish

Можно заменить предыдущий выведенный промпт на пользовательскую строку. Это полезно в тех случаях, когда весь промпт не всегда нужен. Чтобы включить это, запустите `enable_transience` в сеансе оболочки. Чтобы включить его глобально, добавьте это в ваш `~/.config/fish/config.fish`. Переход может быть отключён на лету с помощью `disable_transience`.

Обратите внимание, что в случае с Fish, временный промпт печатается только в том случае, если командная строка не пустая и синтаксически верна.

- По умолчанию, левая сторона ввода заменяется жирным зеленым `❯`. Для настройки определите новую функцию под названием `starship_transient_prompt_func`. Например, чтобы отобразить здесь модуль Starship `character`, вы должны выполнить

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- По умолчанию, правая сторона ввода пуста. Для настройки определите новую функцию под названием `starship_transient_rprompt_func`. Например, чтобы отобразить время, когда здесь была запущена последняя команда, вы должны выполнить

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt и TransientRightPrompt в Bash

Фреймворк [Ble.sh](https://github.com/akinomyoga/ble.sh) в версии 0.4 или выше позволяет заменить выведенный ранее промпт, пользовательскими строками. Это полезно в тех случаях, когда весть промпт не всегда нужен. Чтобы включить это, добавьте в `~/.bashrc` `bleopt prompt_ps1_transient=<value>`:

Значение \<value\> здесь - это разделённый двоеточиями список из `always`, `same-dir` и `trim`.
Когда `prompt_ps1_final` пуст, а опция `prompt_ps1_transient` имеет непустое значение \<value\>, промпт, заданный в `PS1`, стирается при выходе из текущей командной строки.
Если \<value\> содержит поле `trim`, сохраняется только последняя строка многострочного `PS1`, а остальные строки стираются. Иначе командная строка будет перерисована, как если бы было задано `PS1=`. Когда поле `same-dir` присутствует в \<value\>, и текущий рабочий каталог отличается от конечного каталога предыдущей командной строки, эта опция `prompt_ps1_transient` игнорируется.

Внесите следующие изменения в свой `~/.blerc` (или в `~/.config/blesh/init.sh `), чтобы настроить отображение слева и справа:

- Чтобы настроить, на что будет заменена левая часть ввода, настройте параметр `prompt_ps1_final` Ble.sh. Например, чтобы отобразить здесь символ Starship's `character` модуля здесь, вы бы сделали

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- Чтобы настроить, на что будет заменена правая часть ввода, настройте параметр `prompt_rps1_final` Ble.sh. Например, чтобы отобразить время, когда здесь была запущена последняя команда, вы должны выполнить

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Пользовательские команды предварительного промпта и предварительного выполнения в Cmd

Clink предоставляет чрезвычайно гибкие API-интерфейсы для запуска команд предварительного промпта и предварительного выполнения в оболочке Cmd. Он довольно прост в использовании со Starship. Внесите следующие изменения в свой файл `starship.lua` в соответствии с вашими требованиями:

- Чтобы запустить пользовательскую функцию непосредственно перед отображением промпта, определите новую функцию с именем `starship_preprompt_user_func`. Эта функция получает текущий промпт в виде строки, которую вы можете использовать. Например, чтобы нарисовать ракету перед промптом, вы должны сделать

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- Чтобы запустить пользовательскую функцию непосредственно перед выполнением команды, определите новую функцию с именем `starship_precmd_user_func`. Эта функция получает текущую командную строку в виде строки, которую вы можете использовать. Например, чтобы напечатать команду, которая вот-вот будет выполнена, вы должны сделать

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Пользовательские команды перед командной строкой и перед запуском Bash

Bash не имеет формальной среды preexec/precmd, как и большинство других оболочек. Из-за этого трудно предоставить полностью настраиваемые хуки в `bash`. Тем не менее, Starship дает вам ограниченную возможность вставить собственные функции в процедуру отображения подсказки:

- Чтобы запустить пользовательскую функцию прямо перед отображением подсказки, определите новую функцию и затем назначьте ей имя `starship_precmd_user_func`. Например, чтобы нарисовать ракету перед появлением подсказки, сделайте

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Чтобы запустить пользовательскую функцию непосредственно перед выполнением команды, вы можете использовать механизм ловушки [`DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Тем не менее, вы **должны** поймать сигнал DEBUG _перед_ инициализацией Starship! Starship может сохранить значение ловушки DEBUG, но если ловушка перезаписана после запуска Starship, некоторая функциональность сломается.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Ловушка DEBUG *перед* запуском starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Пользовательские команды предварительного промпта и предварительного выполнения в PowerShell

PowerShell не имеет формальной платформы preexec/precmd, как большинство других оболочек. Из-за этого трудно предоставить полностью настраиваемые хуки в `powershell`. Тем не менее, Starship дает вам ограниченную возможность вставить собственные функции в процедуру отображения подсказки:

Создайте функцию с именем `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Изменение заголовка окна

Некоторые промпты командной строки автоматически изменят название окна для вас (например, чтобы оно отражало ваш рабочий каталог). Fish даже делает это по умолчанию. Starship этого не делает, но довольно просто добавить эту функциональность в `bash`, `zsh`, `cmd` или `powershell`.

Сначала задайте функцию изменения заголовка окна (идентичную в bash и zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

Вы можете использовать переменные для настройки этого заголовка (`$USER`, `$HOSTNAME`, и `$PWD` являются популярными вариантами).

В `bash`, установите эту функцию как функцию precmd в Starship:

```bash
starship_precmd_user_func="set_win_title"
```

В `zsh`, добавьте это в массив `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

Если вам нравится результат, добавьте эти строки в свой файл конфигурации оболочки (`~/.bashrc` или `~/.zshrc`), чтобы сделать его постоянным.

Например, если вы хотите отобразить ваш текущий каталог в заголовке вкладки терминала, добавьте следующие строки в `~/. bashrc` или `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

Для Cmd вы можете изменить заголовок окна, используя функцию `starship_preprompt_user_func`.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

Вы также можете настроить аналогичный вывод с помощью PowerShell, создав функцию с именем `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Включить Правый Промпт

Некоторые оболочки поддерживают правый промпт, который отображается в той же строке, что и входные данные. Starship может задать содержание правого промпта, используя опцию `right_format`. Любой модуль, который может быть использован в `format`, также поддерживается в `right_format`. Переменная `$all` будет содержать только модули, явно не используемые ни в `format`, ни в `right_format`.

Примечание: Правые промпты состоят из одной строки, следующий за местоположением ввода. Чтобы выровнять модули по правому краю над строкой ввода в многострочном промпте, ознакомьтесь с [`fill` module](../config/#fill).

`right_format` в настоящее время поддерживается для следующих оболочек: elvish, fish, zsh, xonsh, cmd, nutshell, bash.

Примечание: [Ble.sh ](https://github.com/akinomyoga/ble.sh) используя правый промпт в bash необходимо установить фреймворк версии 0.4 или выше.

### Пример

```toml
# ~/.config/starship.toml

# Минимальный левый промпт 
format = """$character"""

# переместите остальную часть промпта вправо
right_format = """$all"""
```

Выдает промпт, подобный следующему:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Продолжение Промпта

Некоторые оболочки поддерживают промпт продолжения наряду с обычным промптом. Этот промпт отображается вместо обычного промпта, когда пользователь вводит неполный оператор (например, единственную левую круглую скобку или кавычку).

Starship может настроить промпт на продолжение, используя опцию `continuation_prompt`. По умолчанию промпт `'[∙](bright-black) '`.

Примечание: `continuation_prompt` должно быть изменено на строку без каких-либо переменных.

Примечание: Дальнейшие подсказки доступны только в следующих оболочках:

- `bash`
- `zsh`
- `PowerShell`

### Пример

```toml
# ~/.config/starship.toml

# Запрос на продолжение, в котором отображаются две заполненные стрелки
continuation_prompt = '▶️▶️ '
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

### Конфигурация

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

#### Опции

| Параметр        | По умолчанию                 | Описание                                                                                  |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------------------- |
| `format`        | `'[$symbol$model]($style) '` | Формат модуля.                                                                            |
| `symbol`        | `'🤖 '`                       | The symbol shown before the model name.                                                   |
| `style`         | `'bold blue'`                | Стиль модуля.                                                                             |
| `model_aliases` | `{}`                         | Map of model IDs or display names to shorter aliases. Checks ID first, then display name. |
| `disabled`      | `false`                      | Disables the `claude_model` module.                                                       |

#### Переменные

| Переменная | Пример              | Описание                              |
| ---------- | ------------------- | ------------------------------------- |
| model      | `Claude 3.5 Sonnet` | The display name of the current model |
| model_id   | `claude-3-5-sonnet` | The model ID                          |
| symbol     |                     | Отражает значение параметра `symbol`  |
| style\*  |                     | Отражает значение параметра `style`   |

\*: Эта переменная может использоваться только в качестве части строки style

#### Примеры

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

#### Опции

| Параметр               | По умолчанию                      | Описание                                           |
| ---------------------- | --------------------------------- | -------------------------------------------------- |
| `format`               | `'[$gauge $percentage]($style) '` | Формат модуля.                                     |
| `symbol`               | `''`                              | The symbol shown before the gauge.                 |
| `gauge_width`          | `5`                               | The width of the gauge in characters.              |
| `gauge_full_symbol`    | `'█'`                             | The symbol used for filled segments of the gauge.  |
| `gauge_partial_symbol` | `'▒'`                             | The symbol used for partial segments of the gauge. |
| `gauge_empty_symbol`   | `'░'`                             | The symbol used for empty segments of the gauge.   |
| `display`              | [см. ниже](#display)              | Threshold and style configurations.                |
| `disabled`             | `false`                           | Disables the `claude_context` module.              |

##### Display

The `display` option is an array of objects that define thresholds and styles for different usage levels. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Параметр    | По умолчанию | Описание                                                                 |
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

#### Переменные

| Переменная                   | Пример  | Описание                                              |
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
| symbol                       |         | Отражает значение параметра `symbol`                  |
| style\*                    |         | Mirrors the style from the matching display threshold |

\*: Эта переменная может использоваться только в качестве части строки style

#### Примеры

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

#### Опции

| Параметр   | По умолчанию                       | Описание                            |
| ---------- | ---------------------------------- | ----------------------------------- |
| `format`   | `'[$symbol(\\$$cost)]($style) '` | Формат модуля.                      |
| `symbol`   | `'💰 '`                             | The symbol shown before the cost.   |
| `display`  | [см. ниже](#display-1)             | Threshold and style configurations. |
| `disabled` | `false`                            | Disables the `claude_cost` module.  |

##### Display

The `display` option is an array of objects that define cost thresholds and styles. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Параметр    | По умолчанию | Описание                                                      |
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

#### Переменные

| Переменная    | Пример   | Описание                                              |
| ------------- | -------- | ----------------------------------------------------- |
| cost          | `1.23`   | Total session cost in USD (formatted to 2 decimals)   |
| duration      | `1m 30s` | Total session duration                                |
| api_duration  | `45s`    | Total API call duration                               |
| lines_added   | `1.2k`   | Total lines of code added                             |
| lines_removed | `500`    | Total lines of code removed                           |
| symbol        |          | Отражает значение параметра `symbol`                  |
| style\*     |          | Mirrors the style from the matching display threshold |

\*: Эта переменная может использоваться только в качестве части строки style

#### Примеры

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

## Строки стиля

Строки стиля - это список слов, разделенных пробелами. Слова не чувствительны к регистру (то есть `bold` и `BoLd` считаются одной строкой). Каждое слово может быть одним из следующих:

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

где `<color>` является цветовым спецификатором (обсуждается ниже). `fg:<color>` и `<color>` в настоящее время выполняет то же самое, хотя в будущем это может измениться. `<color>` также может быть установлено значение `prev_fg` или `prev_bg`, которое соответствует цвету переднего плана или фона предыдущего элемента соответственно, если доступно, или `none` в противном случае. `inverted` меняет местами цвета фона и переднего плана. Порядок слов в строке не имеет значения.

Токен `none` переопределяет все остальные токены в строке, если он не является частью спецификатора `bg:` так, например, `fg:red none fg:blue` все равно создаст строку без стиля. `bg:none` устанавливает цвет по умолчанию на цвет `fg:red bg:none` эквивалентно `red` или `fg:red` и `bg:green fg:red bg:none` также эквивалентно `fg:red` или `red`. Использование `none` в сочетании с другими токенами может стать ошибкой в будущем.

Цветовой спецификатор может быть одним из следующих:

- Один из стандартных цветов терминалов: `black`, `red`, `green`, `blue`, `gellow`, `purple`, `cyan`, `white`. При желании вы можете добавить к ним префикс `bright-`, чтобы получить версию bright (например, `bright-white`).
- `#`, за которой следует шестизначное шестнадцатеричное число. Это определяет [шестнадцатеричный код цвета RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Число от 0 до 255. Это определяет [8-битный код цвета ANSI](https://i.stack.imgur.com/KTSQa.png).

Если для переднего плана/фона задано несколько цветов, то последняя из строк будет иметь приоритет.

Не все строки стиля будут корректно отображаться в терминале. В частности, существуют следующие известные ошибки:

- Во многих терминалах по умолчанию отключена поддержка `blink`.
- `hidden` [не поддерживается в iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` по умолчанию не поддерживается в macOS Terminal.app.
