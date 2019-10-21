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

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `underline`
  - `dimmed`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing , though this may change in the future. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string, so that e.g. `fg:red none fg:blue` will still create a string with no styling. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
