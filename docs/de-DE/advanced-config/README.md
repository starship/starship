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

- To run a custom function right before a command runs, you can use the [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Allerdings **muss** das DEBUG Signal *vor* der Initialisierung von Starship getrapped werden! Starship kann den Wert der DEBUG-trap speichern. Wenn der Wert der DEBUG-trap überschrieben wird nachdem Starship gestartet ist kann es zu Fehlern im Bezug auf die verwendete DEBUG-trap kommen.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG # DEBUG-Trap *bevor* Starship läuft
eval $(starship init bash)
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

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh.

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


## Style-Strings

Style-String sind Wortlisten, getrennt durch Leerzeichen. Die Wörter haben keine Groß- und Kleinschreibung (z.B. `bold` und `BoLd` werden als dieselbe Zeichenkette betrachtet). Jedes Wort kann eines der folgenden sein:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

wobei `<color>` eine Farbspezifikation ist (siehe unten). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. Die Reihenfolge der Wörter in der Liste spielt keine Rolle.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. In der Zukunft könnte die Unterstützung von `none` in Verbindung mit anderen Tokens fallen gelassen werden.

Eine Farbspezifikation kann wie folgt aussehen:

 - Einer der Standardfarben der Konsole: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Optional kann ein `bright-` vorangestellt werden um die helle Version zu erhalten (z.B. `bright-white`).
 - Eine `#` gefolgt von einer sechsstelligen Hexadezimalnummer. Dies ergibt einen [RGB hex Farbcode](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Eine Zahl zwischen 0-255. Dies ergibt einen [8-bit ANSI-Farbcode](https://i.stack.imgur.com/KTSQa.png).

Wenn mehrere Farben für Vordergrund oder Hintergrund angegeben werden, hat die letzte Farbe der Zeichenkette Priorität.
