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

## Cambia il titolo della finestra

Alcune shell prompt cambieranno automaticamente il titolo della finestra (ad esempio per riflettere la directory di lavoro). Fish lo fa per impostazione predefinita. Starship non lo fa, ma è abbastanza semplice aggiungere questa funzionalità a `bash` o `zsh`.

Innanzitutto, bisogna definire una funzione per il cambio del titolo della finestra (identica sia per bash che zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; IL_TUO_TITOLO_QUI \007"
}
```

Puoi usare delle variabili per personalizzare questo titolo (`$USER`, `$HOSTNAME`, e `$PWD` sono le scelte più popolari).

In `bash`, impostare questa funzione per essere la precmd Starship function:

```bash
starship_precmd_user_func="set_win_title"
```

In `zsh`, aggiungi questo `precmd_functions` all'array:

```bash
precmd_functions+=(set_win_title)
```

Se ti piace il risultato, aggiungi queste righe al tuo file shell di configurazione (`~/.bashrc` o `~/.zshrc`) per renderlo permanente.

Ad esempio, se desideri visualizzare la directory corrente nel titolo della scheda del terminale, aggiungi la seguente snippet al tuo `~/.bashrc` or `~/.zshrc`:

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

## Continuation Prompt

Some shells support a continuation prompt along with the normal prompt. This prompt is rendered instead of the normal prompt when the user has entered an incomplete statement (such as a single left parenthesis or quote).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `"[∙](bright-black) "`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Note: Continuation prompts are only available in the following shells:

  - `bash`
  - `zsh`
  - `PowerShell`

### Esempio

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled in arrows
continuation_prompt = "▶▶"
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

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
