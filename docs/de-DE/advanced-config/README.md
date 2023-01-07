# Erweiterte Konfiguration

Auch wenn Starship eine vielseitige Shell ist, reichen manche Konfigurationen in der `starship.toml` nicht aus, um erweiterte Einstellungen vorzunehmen. Diese Seite beschreibt einige fortgeschrittene Konfigurationen für Starship.

::: warning

Die hier beschriebenen Konfigurationen werden sich mit kommenden Updates von Starship verändern.

:::

## TransientPrompt in PowerShell

It is possible to replace the previous-printed prompt with a custom string. This is useful in cases where all the prompt information is not always needed. To enable this, run `Enable-TransientPrompt` in the shell session. To make it permanent, put this statement in your `$PROFILE`. Transience can be disabled on-the-fly with `Disable-TransientPrompt`.

By default, the left side of input gets replaced with `>`. To customize this, define a new function called `Invoke-Starship-TransientFunction`. For example, to display Starship's `character` module here, you would do

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt and TransientRightPrompt in Cmd

Clink allows you to replace the previous-printed prompt with custom strings. This is useful in cases where all the prompt information is not always needed. To enable this, run `clink set prompt.transient <value>` where \<value\> can be one of:

- `always`: always replace the previous prompt
- `same_dir`: replace the previous prompt only if the working directory is same
- `off`: do not replace the prompt (i.e. turn off transience)

You need to do this only once. Make the following changes to your `starship.lua` to customize what gets displayed on the left and on the right:

- By default, the left side of input gets replaced with `>`. To customize this, define a new function called `starship_transient_prompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display Starship's `character` module here, you would do

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display the time at which the last command was started here, you would do

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt and TransientRightPrompt in Fish

It is possible to replace the previous-printed prompt with a custom string. This is useful in cases where all the prompt information is not always needed. To enable this, run `enable_transience` in the shell session. To make it permanent, put this statement in your `~/.config/fish/config.fish`. Transience can be disabled on-the-fly with `disable_transience`.

Note that in case of Fish, the transient prompt is only printed if the commandline is non-empty, and syntactically correct.

- By default, the left side of input gets replaced with a bold-green `❯`. To customize this, define a new function called `starship_transient_prompt_func`. For example, to display Starship's `character` module here, you would do

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. For example, to display the time at which the last command was started here, you would do

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
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

- Um eine benutzerdefinierte Funktion kurz vor Anzeige der Eingabeaufforderung auszuführen, definiere eine neue Funktion und weise den Namen `starship_precmd_user_func` zu. Um beispielsweise eine Rakete anzuzeigen, bevor die Eingabeaufforderung erscheint, würde man folgendes tun

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

## Custom pre-prompt and pre-execution Commands in PowerShell

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

Zum Beispiel, wenn sie ihr aktuelles Verzeichnis als Terminal Title anzeigen wollen, fügen Sie folgenden Code-Schnipsel zu ihrer `~/.bashrc` oder `~/.zshrc` hinzu:

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
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [`fill` module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell.

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

Starship kann das Aussehen des Fortsetzungs-Prompts mit der `continuation_prompt` Option einstellen. The default prompt is `"[∙](bright-black) "`.

Hinweis: Die `continuation_prompt` Anweisung sollte auf einen literalen String ohne Variablen gesetzt werden.

Hinweis: Fortsetzungs-Prompts sind nur für folgende Shells verfügbar:

- `bash`
- `zsh`
- `PowerShell`

### Beispiel

```toml
# ~/.config/starship.toml

# Ein Fortsetzungs-Prompt der 2 ausgefüllte Pfeile darstellt
continuation_prompt = "▶▶"
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

wobei `<color>` eine Farbspezifikation ist (siehe unten). `fg:<color>` und `<color>` tun derzeit dasselbe, das kann sich in Zukunft aber ändern. `inverted` tauscht Hinter- und Vordergrundfarben. Die Reihenfolge der Wörter in der Liste spielt keine Rolle.

`none` überschreibt alle anderen Tokens in einem String wenn es nicht ein Teil einer `bg:` Zeichenkette ist, so dass z.B. über die Zeichenkette `fg:red none fg:blue` kein Styling mehr anzeigt wird. `bg:none` setzt den Hintergrund auf die Standardfarbe, so `fg:red bg:none` entspricht `rot` oder `fg:red` und `bg:green fg:red bg:none` entspricht auch `fg:red` oder `rot`. In der Zukunft könnte die Unterstützung von `none` in Verbindung mit anderen Tokens fallen gelassen werden.

Eine Farbspezifikation kann wie folgt aussehen:

- One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
- Eine `#` gefolgt von einer sechsstelligen Hexadezimalnummer. Dies ergibt einen [RGB hex Farbcode](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Eine Zahl zwischen 0-255. Dies ergibt einen [8-bit ANSI-Farbcode](https://i.stack.imgur.com/KTSQa.png).

Wenn mehrere Farben für Vordergrund oder Hintergrund angegeben werden, hat die letzte Farbe der Zeichenkette Priorität.

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app
