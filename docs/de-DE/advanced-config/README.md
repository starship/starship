# Erweiterte Konfiguration

Auch wenn Starship eine vielseitige Shell ist, reichen manche Konfigurationen in der `starship.toml` nicht aus, um manche Sachen zu erreichen. Diese Seite beschreibt einige fortgeschrittene Konfigurationen für Starship.

> [!WARNING] Die Konfigurationen in diesem Abschnitt können in zukünftigen Releases von Starship Änderungen unterliegen.

## TransientPrompt in PowerShell

Es ist möglich, die zuvor geprintete Prompt mit einem benutzerdefinierten String zu ersetzen. Das ist in Fällen nützlich, in denen nicht immer die ganze Information der Prompt gebraucht wird. Führe `Enable-TransientPrompt` in deiner Shell-Session aus, um dieses Verhalten zu aktivieren. Füge das Statement in dein `$PROFILE` ein, um diese Funktion dauerhaft zu aktivieren. Transience can be disabled on-the-fly with `Disable-TransientPrompt`.

By default, the left side of input gets replaced with `>`. To customize this, define a new function called `Invoke-Starship-TransientFunction`. For example, to display Starship's `character` module here, you would do

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt and TransientRightPrompt in Cmd

Clink allows you to replace the previous-printed prompt with custom strings. Das ist in Fällen nützlich, in denen nicht immer die ganze Information der Prompt gebraucht wird. To enable this, run `clink set prompt.transient <value>` where \<value\> can be one of:

- `always`: always replace the previous prompt
- `same_dir`: replace the previous prompt only if the working directory is same
- `off`: do not replace the prompt (i.e. turn off transience)

Sie müssen dies nur einmal tun. Make the following changes to your `starship.lua` to customize what gets displayed on the left and on the right:

- By default, the left side of input gets replaced with `>`. To customize this, define a new function called `starship_transient_prompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display Starship's `character` module here, you would do

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- Standardmäßig ist die rechte Seite de Eingabe leer. To customize this, define a new function called `starship_transient_rprompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display the time at which the last command was started here, you would do

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt and TransientRightPrompt in Fish

Es ist möglich, die zuvor geprintete Prompt mit einem benutzerdefinierten String zu ersetzen. Das ist in Fällen nützlich, in denen nicht immer die ganze Information der Prompt gebraucht wird. To enable this, run `enable_transience` in the shell session. To make it permanent, put this statement in your `~/.config/fish/config.fish`. Transience can be disabled on-the-fly with `disable_transience`.

Note that in case of Fish, the transient prompt is only printed if the commandline is non-empty, and syntactically correct.

- By default, the left side of input gets replaced with a bold-green `❯`. To customize this, define a new function called `starship_transient_prompt_func`. For example, to display Starship's `character` module here, you would do

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- Standardmäßig ist die rechte Seite de Eingabe leer. To customize this, define a new function called `starship_transient_rprompt_func`. For example, to display the time at which the last command was started here, you would do

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## „TransientPrompt“ und „TransientRightPrompt“ in Bash

The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework at v0.4 or higher allows you to replace the previous-printed prompt with custom strings. This is useful in cases where all the prompt information is not always needed. To enable this, put this in `~/.bashrc` `bleopt prompt_ps1_transient=<value>`:

The \<value\> here is a colon-separated list of `always`, `same-dir` and `trim`. When `prompt_ps1_final` is empty and the option `prompt_ps1_transient` has a non-empty \<value\>, the prompt specified by `PS1` is erased on leaving the current command line. If \<value\> contains a field `trim`, only the last line of multiline `PS1` is preserved and the other lines are erased. Otherwise, the command line will be redrawn as if `PS1=` is specified. When a field `same-dir` is contained in \<value\> and the current working directory is different from the final directory of the previous command line, this option `prompt_ps1_transient` is ignored.

Make the following changes to your `~/.blerc` (or in `~/.config/blesh/init.sh`) to customize what gets displayed on the left and on the right:

- To customize what the left side of input gets replaced with, configure the `prompt_ps1_final` Ble.sh option. For example, to display Starship's `character` module here, you would do

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- To customize what the right side of input gets replaced with, configure the `prompt_rps1_final` Ble.sh option. For example, to display the time at which the last command was started here, you would do

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Custom pre-prompt and pre-execution Commands in Cmd

Clink provides extremely flexible APIs to run pre-prompt and pre-exec commands in Cmd shell. It is fairly simple to use with Starship. Make the following changes to your `starship.lua` file as per your requirements:

- To run a custom function right before the prompt is drawn, define a new function called `starship_preprompt_user_func`. This function receives the current prompt as a string that you can utilize. For example, to draw a rocket before the prompt, you would do

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- To run a custom function right before a command is executed, define a new function called `starship_precmd_user_func`. This function receives the current commandline as a string that you can utilize. For example, to print the command that's about to be executed, you would do

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Benutzerdefinierte Pre-Prompt- und Pre-Execution-Befehle in der Bash

Die Bash Shell hat, im Gegensatz zu vielen anderen Shells, kein konventionelles preexec/precmd Framework. Daher gestaltet es sich schwierig, vollständig anpassbare Hooks für `bash` anzubieten. Starship bietet daher die begrenzte Möglichkeit, eigene Funktionen in das prompt rendering Verfahren einzufügen:

- Um eine benutzerdefinierte Funktion kurz vor Anzeige der Eingabeaufforderung auszuführen, definiere eine neue Funktion und weise den Namen `starship_precmd_user_func` zu. Zum Beispiel, um vor der Eingabeaufforderung eine Rakete zu zeichnen, würden Sie Folgendes tun

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Um eine benutzerdefinierte Funktion direkt vor der Ausführung eines Befehls auszulösen, kann man den [`DEBUG` trap](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/) Mechanismus verwenden. Allerdings **muss** das DEBUG Signal _vor_ der Initialisierung von Starship getrapped werden! Starship kann den Wert der DEBUG-trap speichern. Wenn der Wert der DEBUG-trap überschrieben wird nachdem Starship gestartet ist kann es zu Fehlern im Bezug auf die verwendete DEBUG-trap kommen.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Benutzerdefinierte Pre-Prompt- und Pre-Execution-Befehle in PowerShell

PowerShell does not have a formal preexec/precmd framework like most other shells. Because of this, it is difficult to provide fully customizable hooks in `powershell`. Starship bietet daher die begrenzte Möglichkeit, eigene Funktionen in das prompt rendering Verfahren einzufügen:

Create a function named `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Fenstertitel anpassen

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish ist standardmäßig so konfiguriert. Starship does not do this, but it's fairly straightforward to add this functionality to `bash`, `zsh`, `cmd` or `powershell`.

Zuerst wird eine Funktion definiert um den Fenstertitel zu ändern ( für bash und zsh ist die Funktion identisch):

```bash
function set_win_title(){
    echo -ne "\033]0; DEIN_FENSTERTITEL_HIER \007"
}
```

Sie können Variablen verwenden, um diesen Titel anzupassen (`$USER`, `$HOSTNAME`, `$PWD`).

Für `bash` muss die Funktion als "precmd starship"-Funktion gesetzt werden:

```bash
starship_precmd_user_func="set_win_title"
```

Füge dies in `Zsh` zum `precmd_functions`-Array hinzu:

```bash
precmd_functions+=(set_win_title)
```

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zshrc`) to make it permanent.

Zum Beispiel, wenn Sie ihr aktuelles Verzeichnis im Titel Ihrer Terminal-Registerkarte anzeigen möchten, fügen Sie folgenden Code-Schnipsel zu Ihrer `~/.bashrc` oder `~/.zshrc` hinzu:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

For Cmd, you can change the window title using the `starship_preprompt_user_func` function.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

You can also set a similar output with PowerShell by creating a function named `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [`fill` module](../config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Note: The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework v0.4 or higher should be installed in order to use right prompt in bash.

### Beispiel

```toml
# ~/.config/starship.toml

# A minimal left prompt
format = """$character"""

# move the rest of the prompt to the right
right_format = """$all"""
```

Produces a prompt like the following:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Fortsetzungsprompt

Einige Shells unterstützen einen speziellen Fortsetzungsprompt zusätzlich zum normalen Prompt. Dieser Prompt wird anstelle des normalen Prompts ausgegeben, wenn der Benutzer ein unvollständiges Kommando eingegeben hat (etwa wie eine einzelne linke Klammer oder ein einzelnes Anführungszeichen).

Starship kann das Aussehen des Fortsetzungs-Prompts mit der `continuation_prompt` Option einstellen. The default prompt is `'[∙](bright-black) '`.

Hinweis: Die `continuation_prompt` Anweisung sollte auf einen literalen String ohne Variablen gesetzt werden.

Hinweis: Fortsetzungs-Prompts sind nur für folgende Shells verfügbar:

- `bash`
- `zsh`
- `PowerShell`

### Beispiel

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled-in arrows
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

### Konfiguration

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

#### Optionen

| Option          | Standartwert                 | Beschreibung                                                                              |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------------------- |
| `format`        | `'[$symbol$model]($style) '` | Das Format für das Modul.                                                                 |
| `Symbol`        | `'🤖 '`                       | The symbol shown before the model name.                                                   |
| `style`         | `'bold blue'`                | Stil für dieses Modul.                                                                    |
| `model_aliases` | `{}`                         | Map of model IDs or display names to shorter aliases. Checks ID first, then display name. |
| `disabled`      | `false`                      | Disables the `claude_model` module.                                                       |

#### Variables

| Variable  | Beispiel            | Beschreibung                          |
| --------- | ------------------- | ------------------------------------- |
| model     | `Claude 3.5 Sonnet` | The display name of the current model |
| model_id  | `claude-3-5-sonnet` | The model ID                          |
| Symbol    |                     | Spiegelt den Wert der Option `symbol` |
| style\* |                     | Spiegelt den Wert der Option `style`  |

\*: This variable can only be used as a part of a style string

#### Beispiele

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

#### Optionen

| Option                 | Standartwert                      | Beschreibung                                       |
| ---------------------- | --------------------------------- | -------------------------------------------------- |
| `format`               | `'[$gauge $percentage]($style) '` | Das Format für das Modul.                          |
| `Symbol`               | `''`                              | The symbol shown before the gauge.                 |
| `gauge_width`          | `5`                               | The width of the gauge in characters.              |
| `gauge_full_symbol`    | `'█'`                             | The symbol used for filled segments of the gauge.  |
| `gauge_partial_symbol` | `'▒'`                             | The symbol used for partial segments of the gauge. |
| `gauge_empty_symbol`   | `'░'`                             | The symbol used for empty segments of the gauge.   |
| `display`              | [siehe unten](#display)           | Threshold and style configurations.                |
| `disabled`             | `false`                           | Disables the `claude_context` module.              |

##### Display

The `display` option is an array of objects that define thresholds and styles for different usage levels. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Option      | Standartwert | Beschreibung                                                             |
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

#### Variables

| Variable                     | Beispiel | Beschreibung                                          |
| ---------------------------- | -------- | ----------------------------------------------------- |
| gauge                        | `██▒░░`  | Visual representation of context usage                |
| percentage                   | `65%`    | Context usage as a percentage                         |
| input_tokens                 | `45.2k`  | Total input tokens in conversation                    |
| output_tokens                | `12.3k`  | Total output tokens in conversation                   |
| curr_input_tokens          | `5.1k`   | Input tokens from most recent API call                |
| curr_output_tokens         | `1.2k`   | Output tokens from most recent API call               |
| curr_cache_creation_tokens | `1.5k`   | Cache creation tokens from most recent API call       |
| curr_cache_read_tokens     | `23.4k`  | Cache read tokens from most recent API call           |
| total_tokens                 | `200k`   | Total context window size                             |
| Symbol                       |          | Spiegelt den Wert der Option `symbol`                 |
| style\*                    |          | Mirrors the style from the matching display threshold |

\*: This variable can only be used as a part of a style string

#### Beispiele

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

#### Optionen

| Option     | Standartwert                       | Beschreibung                        |
| ---------- | ---------------------------------- | ----------------------------------- |
| `format`   | `'[$symbol(\\$$cost)]($style) '` | Das Format für das Modul.           |
| `Symbol`   | `'💰 '`                             | The symbol shown before the cost.   |
| `display`  | [siehe unten](#display-1)          | Threshold and style configurations. |
| `disabled` | `false`                            | Disables the `claude_cost` module.  |

##### Display

The `display` option is an array of objects that define cost thresholds and styles. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Option      | Standartwert | Beschreibung                                                  |
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

#### Variables

| Variable      | Beispiel | Beschreibung                                          |
| ------------- | -------- | ----------------------------------------------------- |
| cost          | `1.23`   | Total session cost in USD (formatted to 2 decimals)   |
| duration      | `1m 30s` | Total session duration                                |
| api_duration  | `45s`    | Total API call duration                               |
| lines_added   | `1.2k`   | Total lines of code added                             |
| lines_removed | `500`    | Total lines of code removed                           |
| Symbol        |          | Spiegelt den Wert der Option `symbol`                 |
| style\*     |          | Mirrors the style from the matching display threshold |

\*: This variable can only be used as a part of a style string

#### Beispiele

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

## Style-Strings

Stil-Zeichenketten sind eine Liste von Wörtern, getrennt durch Leerzeichen. Die Wörter haben keine Groß- und Kleinschreibung (z.B. `bold` und `BoLd` werden als dieselbe Zeichenkette betrachtet). Jedes Wort kann eines der folgenden sein:

- `fett`
- `kursiv`
- `unterstrichen`
- `gedimmt`
- `invertiert`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `keins`

wobei `<color>` eine Farbspezifikation ist (siehe unten). `fg:<color>` und `<color>` tun derzeit dasselbe, das kann sich in Zukunft aber ändern. `<color>` can also be set to `prev_fg` or `prev_bg` which evaluates to the previous item's foreground or background color respectively if available or `none` otherwise. `inverted` tauscht Hinter- und Vordergrundfarben. Die Reihenfolge der Wörter in der Liste spielt keine Rolle.

`none` überschreibt alle anderen Tokens in einem String wenn es nicht ein Teil einer `bg:` Zeichenkette ist, so dass z.B. über die Zeichenkette `fg:red none fg:blue` kein Styling mehr anzeigt wird. `bg:none` setzt den Hintergrund auf die Standardfarbe, so `fg:red bg:none` entspricht `rot` oder `fg:red` und `bg:green fg:red bg:none` entspricht auch `fg:red` oder `rot`. In der Zukunft könnte die Unterstützung von `none` in Verbindung mit anderen Tokens fallen gelassen werden.

Eine Farbspezifikation kann wie folgt aussehen:

- One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
- Eine `#` gefolgt von einer sechsstelligen Hexadezimalnummer. Dies ergibt einen [RGB hex Farbcode](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Eine Zahl zwischen 0-255. Dies ergibt einen [8-bit ANSI-Farbcode](https://i.stack.imgur.com/KTSQa.png).

Wenn mehrere Farben für Vordergrund oder Hintergrund angegeben werden, hat die letzte Farbe der Zeichenkette Priorität.

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default.
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app.
