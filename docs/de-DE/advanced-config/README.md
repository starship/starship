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

## Fenstertitel anpassen

Manche shell prompts können den Fenstertitel ändern. Fish ist standardmäßig so konfiguriert. Starship ändert standardmäßig den Fenstertitel nicht, aber es ist sehr einfach die Funktion zu `bash` oder `zsh` hinzuzufügen.

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

Um die Funktion dauerhaft zu machen, schreiben Sie diese in die Shell-Konfigurationsdatei: (`~/.bashrc` oder `~/.zsrhc`).

## Style-Strings

Style-String sind Wortlisten, getrennt durch Leerzeichen. Die Wörter haben keine Groß- und Kleinschreibung (z.B. `bold` und `BoLd` werden als dieselbe Zeichenkette betrachtet). Jedes Wort kann eines der folgenden sein:

  - `bold`
  - `underline`
  - `dimmed`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

wobei `<color>` eine Farbspezifikation ist (siehe unten). `fg:<color>` und `<color>` tun derzeit dasselbe , das kann sich in Zukunft aber ändern. Die Reihenfolge der Wörter in der Liste spielt keine Rolle.

`None` überschreibt alle anderen Tokens in einem String, so dass z.B. der string `fg:red none fg:blue` kein Styling anzeigen wird. In der Zukunft könnte die Unterstützung von `none` in Verbindung mit anderen Tokens fallen gelassen werden.

Eine Farbspezifikation kann wie folgt aussehen:

 - Einer der Standardfarben der Konsole: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Optional kann ein `bright-` vorangestellt werden um die helle Version zu erhalten (z.B. `bright-white`).
 - Eine `#` gefolgt von einer sechsstelligen Hexadezimalnummer. Dies ergibt einen [RGB hex Farbcode](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Eine Zahl zwischen 0-255. Dies ergibt einen [8-bit ANSI-Farbcode](https://i.stack.imgur.com/KTSQa.png).

Wenn mehrere Farben für Vordergrund oder Hintergrund angegeben werden, hat die letzte Farbe der Zeichenkette Priorität.
