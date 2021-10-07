# Configurazione Avanzata

Nonostante Starship sia una shell versatile, a volte devi fare qualche modifica in più in `starship.toml` per ottenere alcune cose. Questa pagina descrive alcune tecniche di configurazione avanzate utilizzate in Starship.

::: warning

Le configurazioni in questa sezione sono soggette a modifiche nelle future versioni di Starship.

:::

## Comandi personalizzati di pre-prompt e pre-esecuzione per Bash

Bash non ha un framework preexec/precmd formale come la maggior parte delle altre shell. Per questo motivo, è difficile fornire hook completamente personalizzabile in `bash`. Tuttavia, Starship dà la limitata possibilità di inserire le tue funzioni nella procedura prompt-rendering:

- Per eseguire una funzione personalizzata a destra del prompt prima che venga disegnato, definisci una nuova funzione e assegna il suo nome a `starship_precmd_user_func`. Per esempio, per visualizzare l'icona di un razzo prima del prompt, si può usare il codice seguente

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Per eseguire una funzione personalizzata prima dell'esecuzione di un comando, è possibile utilizzare il meccanismo trappola [`DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Tuttavia, **devi** intrappolare il segnale DEBUG *prima di* inizializzare Starship! Starship può preservare il valore trappola di DEBUG, ma se la trappola viene sovrascritta dopo l'avvio di Starship, alcune funzionalità non funzioneranno.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG # Trap DEBUG *prima* di eseguire starship
eval $(starship bash)
```

## Custom pre-prompt and pre-execution Commands in PowerShell

PowerShell does not have a formal preexec/precmd framework like most other shells. Because of this, it is difficult to provide fully customizable hooks in `powershell`. Tuttavia, Starship dà la limitata possibilità di inserire le tue funzioni nella procedura prompt-rendering:

Create a function named `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Change Window Title

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish even does it by default. Starship does not do this, but it's fairly straightforward to add this functionality to `bash` or `zsh`.

First, define a window title change function (identical in bash and zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

You can use variables to customize this title (`$USER`, `$HOSTNAME`, and `$PWD` are popular choices).

In `bash`, set this function to be the precmd starship function:

```bash
starship_precmd_user_func="set_win_title"
```

In `zsh`, add this to the `precmd_functions` array:

```bash
precmd_functions+=(set_win_title)
```

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zshrc`) to make it permanent.

For example, if you want to display your current directory in your terminal tab title, add the following snippet to your `~/.bashrc` or `~/.zshrc`:

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

`right_format` is currently supported for the following shells: elvish, fish, zsh.

### Esempio

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


## Stile delle Stringhe

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

 - Uno dei colori standard del terminale: `nero`, `rosso`, `verde`, `blu`, `giallo`, `viola`, `ciano`, `bianco`. Puoi eventualmente utilizzare il prefisso `bright-` per ottenere la versione luminosa (es. `bright-white`).
 - Un `#` seguito da un valore esadecimale a sei cifre. Questo specifica un [colore esagesimale in RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Un numero compreso tra 0-255. Specifica un [codice colore ANSI a 8 bit](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
