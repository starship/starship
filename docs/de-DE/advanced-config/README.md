# Erweiterte Konfiguration

Auch wenn Starship eine vielseitige Shell ist, reichen manche Konfigurationen in der `starship.toml` nicht aus, um erweiterte Einstellungen vorzunehmen. Diese Seite beschreibt einige fortgeschrittene Konfigurationen für Starship.

::: Warnung

Die hier beschriebenen Konfigurationen werden sich mit kommenden Updates von Starship verändern.

:::

## Benutzerdefinierte Pre-Prompt- und Pre-Execution-Befehle in der Bash

Die Bash Shell hat, im Gegensatz zu vielen anderen Shells, kein konventionelles preexec/precmd Framework. Daher gestaltet es sich schwierig, vollständig anpassbare Hooks für `bash` anzubieten. Starship bietet daher die begrenzte Möglichkeit, eigene Funktionen in das prompt rendering Verfahren einzufügen:

- Um eine benutzerdefinierte Funktion kurz vor Anzeige der Eingabeaufforderung auszuführen, definiere eine neue Funktion und weise den Namen `starship_precmd_user_func` zu. Um beispielsweise eine Rakete anzuzeigen, bevor die Eingabeaufforderung erscheint, würde man folgendes tun

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Um eine benutzerdefinierte Funktion direkt vor der Ausführung eines Befehls auszulösen, kann man den [`DEBUG` trap](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/) Mechanismus verwenden. Allerdings **muss** das DEBUG Signal *vor* der Initialisierung von Starship getrapped werden! Starship kann den Wert der DEBUG-trap speichern. Wenn der Wert der DEBUG-trap überschrieben wird nachdem Starship gestartet ist kann es zu Fehlern im Bezug auf die verwendete DEBUG-trap kommen.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG # DEBUG-Trap *bevor* Starship läuft
eval $(starship init bash)
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

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish ist standardmäßig so konfiguriert. Starship ändert standardmäßig den Fenstertitel nicht, aber es ist sehr einfach die Funktion zu `bash` oder `zsh` hinzuzufügen.

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

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh.

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

## Continuation Prompt

Some shells support a continuation prompt along with the normal prompt. This prompt is rendered instead of the normal prompt when the user has entered an incomplete statement (such as a single left parenthesis or quote).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `"[❯](bold yellow)"`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Note: Continuation prompts are only available in the following shells:

  - `bash`
  - `zsh`
  - `PowerShell`

### Beispiel

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled in arrows
continuation_prompt = "▶▶"
```

## Style-Strings

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
