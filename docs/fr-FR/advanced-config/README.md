# Configuration avancée

Même si Starship est un shell polyvalent, éditer `starship.toml` ne suffit parfois pas pour faire certaines choses. Cette page détaille quelques techniques de configuration avancées utilisées dans starship.

> [!WARNING] The configurations in this section are subject to change in future releases of Starship.

## TransientPrompt in PowerShell

Il est possible de remplacer le prompt précédent avec une string personnalisée. Ceci est utile lorsque toutes les informations fournies par l'invité de commande ne sont pas nécessaire. Pour activer ceci, exécutez `Enable-TransientPrompt` dans la session shell. Pour que la modification soit permanente, ajoutez cette commande dans votre `$PROFILE`. Transience can be disabled on-the-fly with `Disable-TransientPrompt`.

By default, the left side of input gets replaced with `>`. To customize this, define a new function called `Invoke-Starship-TransientFunction`. For example, to display Starship's `character` module here, you would do

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt and TransientRightPrompt in Cmd

Clink allows you to replace the previous-printed prompt with custom strings. Ceci est utile lorsque toutes les informations fournies par l'invité de commande ne sont pas nécessaire. To enable this, run `clink set prompt.transient <value>` where \<value\> can be one of:

- `always`: always replace the previous prompt
- `same_dir`: replace the previous prompt only if the working directory is same
- `off`: do not replace the prompt (i.e. turn off transience)

Il est nécessaire de faire cela qu'une fois. Modifiez votre `starship.lua` pour customiser ce qui sera présenté à gauche et à droite:

- By default, the left side of input gets replaced with `>`. To customize this, define a new function called `starship_transient_prompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display Starship's `character` module here, you would do

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. This function receives the current prompt as a string that you can utilize. Par exemple, pour afficher l'heure à laquelle la dernière commande a été lancée ici, vous feriez

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt et TransientRightPrompt dans Fish

Il est possible de remplacer le prompt précédent avec une string personnalisée. Ceci est utile lorsque toutes les informations fournies par l'invité de commande ne sont pas nécessaire. To enable this, run `enable_transience` in the shell session. To make it permanent, put this statement in your `~/.config/fish/config.fish`. Transience can be disabled on-the-fly with `disable_transience`.

Note that in case of Fish, the transient prompt is only printed if the commandline is non-empty, and syntactically correct.

- By default, the left side of input gets replaced with a bold-green `❯`. To customize this, define a new function called `starship_transient_prompt_func`. For example, to display Starship's `character` module here, you would do

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. Par exemple, pour afficher l'heure à laquelle la dernière commande a été lancée ici, vous feriez

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt and TransientRightPrompt in Bash

The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework at v0.4 or higher allows you to replace the previous-printed prompt with custom strings. This is useful in cases where all the prompt information is not always needed. To enable this, put this in `~/.bashrc` `bleopt prompt_ps1_transient=<value>`:

The \<value\> here is a colon-separated list of `always`, `same-dir` and `trim`.
When `prompt_ps1_final` is empty and the option `prompt_ps1_transient` has a non-empty \<value\>, the prompt specified by `PS1` is erased on leaving the current command line.
If \<value\> contains a field `trim`, only the last line of multiline `PS1` is preserved and the other lines are erased. Otherwise, the command line will be redrawn as if `PS1=` is specified. When a field `same-dir` is contained in \<value\> and the current working directory is different from the final directory of the previous command line, this option `prompt_ps1_transient` is ignored.

Make the following changes to your `~/.blerc` (or in `~/.config/blesh/init.sh`) to customize what gets displayed on the left and on the right:

- To customize what the left side of input gets replaced with, configure the `prompt_ps1_final` Ble.sh option. For example, to display Starship's `character` module here, you would do

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- To customize what the right side of input gets replaced with, configure the `prompt_rps1_final` Ble.sh option. Par exemple, pour afficher l'heure à laquelle la dernière commande a été lancée ici, vous feriez

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Commandes pré-invite et pré-exécution personnalisées dans Cmd

Clink fournit des APIs extrêmement flexibles pour exécuter des commandes pre-invite et pre-exec dans Cmd. Il est assez simple à utiliser avec Starship. Effectuez les modifications suivantes dans votre fichier `starship.lua`, en fonction de vos besoins:

- Pour exécuter une fonction juste avant que l’invite soit dessinée, définissez une nouvelle fonction appelée `starship_preprompt_user_func`. Cette fonction reçoit l’invite courante sous la forme d’une chaine que vous pouvez utiliser. Par exemple, pour dessiner une fusée avant l’invite, vous pouvez faire

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- Pour exécuter une fonction personnalisée juste avant qu’une commande soit exécutée, définissez une nouvelle fonction appelée `starship_precmd_user_func`. Cette fonction reçoit la ligne de commande courante sous la forme d’une chaine que vous pouvez utiliser. Par exemple, pour afficher la commande sur le point d’être exécutée, vous pouvez faire

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Commandes pré-invite et pré-exécution personnalisées en Bash

Bash n'a pas de structure officielle préexec/précmd comme la plupart des autres shells.
C'est pourquoi il est difficile de fournir des hooks entièrement personnalisables dans `bash`.
Cependant, Starship vous permet dans une certaine mesure d'insérer vos propres fonctions dans la procédure de rendu du prompt :

- Pour exécuter une fonction personnalisée juste avant que le prompt ne soit dessiné, définissez une nouvelle fonction et assignez son nom à `starship_precmd_user_func`. Par exemple, pour dessiner une fusée avant la commande, vous feriez

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Pour exécuter une fonction personnalisée juste avant l'exécution d'une commande, vous pouvez utiliser le [ mécanisme d'interruption du signal ` DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/).
  Cependant, vous **devez** piéger le signal DEBUG _avant_ l'initialisation de Starship !
  Starship peut préserver la valeur du piège DEBUG, mais si le piège est écrasé après le démarrage de Starship, certaines fonctionnalités vont casser.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Capture DEBUG *avant* de lancer starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Commandes pré-invite et pré-exécution personnalisées dans PowerShell

Powershell n'a pas de système de préexec/précmd officiel comme la plupart des autres shells.
C'est pourquoi il est difficile de fournir des hooks entièrement personnalisables dans `powershell`.
Cependant, Starship vous permet dans une certaine mesure d'insérer vos propres fonctions dans la procédure de rendu du prompt :

Créez une fonction nommée `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Changer le titre de la fenêtre

Certaines commandes du shell changeront automatiquement le titre de la fenêtre (par exemple, pour refléter le dossier courant). Fish le fait même par défaut.
Starship ne fait pas ça, mais c’est assez facile d’ajouter cette fonctionnalité à `bash`, `zsh`, `cmd` ou `powershell`.

Tout d'abord, définissez une fonction de changement de titre de fenêtre (identique en bash et zsh) :

```bash
function set_titre_fenetre(){
    echo -ne "\033]0; VOTRE_TITRE_ICI\007"
}
```

Vous pouvez utiliser des variables pour personnaliser ce titre (`$USER`, `$HOSTNAME`, et `$PWD` sont des choix populaires).

Dans `bash`, définissez cette fonction comme la fonction précommande Starship :

```bash
starship_precmd_user_func="set_titre_fenetre"
```

Dans `zsh`, ajoutez ceci au tableau `precmd_functions` :

```bash
precmd_functions+=(set_titre_fenetre)
```

Si vous aimez le résultat, ajoutez ces lignes à votre fichier de configuration shell (`~/.bashrc` ou `~/.zshrc`) pour le rendre permanent.

Par exemple, si vous voulez afficher votre dossier courant dans le titre de l'onglet de votre terminal, ajoutez le code suivant à votre `~/.bashrc` ou `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

Pour Cmd, vous pouvez changer le titre de la fenêtre en utilisant la fonction `starship_preprompt_user_func`.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

Vous pouvez également faire la même chose avec PowerShell en créant une fonction nommée `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Mettre l’invite à droite

Certains shells peuvent gérer une invite de commande à droite, sur la même ligne que l’entrée utilisateur. Starship peut définir le contenu de cet invite à droite en utilisant l’option `right_format`. N’importe quel module qui peut être utilisé dans `format` est aussi géré dans `right_format`. La variable `$all` va seulement contenir les modules qui ne sont explicitement utilisés ni dans `format`, ni dans `right_format`.

Note: l’invite à droite est une seule ligne, sur la même ligne que l’entrée. To right align modules above the input line in a multi-line prompt, see the [`fill` module](../config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Note: The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework v0.4 or higher should be installed in order to use right prompt in bash.

### Exemple

```toml
# ~/.config/starship.toml

# Une invite minimale à gauche
format = """$character"""

# déplace le reste de l’invite à droite
right_format = """$all"""
```

Génère l’invite suivante:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

When using `zsh` (v5.0.5+), the shell adds a default trailing space to the right prompt. This can cause alignment issues specifically when using the Starship `$fill` module. To remove this gap, add the following to your `.zshrc`:

```zsh
ZLE_RPROMPT_INDENT=0
```

## Invite de continuation

Certains shells gèrent une invite de continuation en plus de l’invite normale. Cette invite est affichée à la place de l’invite normale quand l’utilisateur a entré une expression incomplète (par exemple, une parenthèse gauche ou une apostrophe seule).

Starship peut définir l’invite de continuation en utilisant l’option `continuation_prompt`. The default prompt is `'[∙](bright-black) '`.

Note: la valeur de `continuation_prompt` doit être une chaine littérale, sans variable.

Note: les invites de confirmation sont uniquement disponibles pour les shells suivants:

- `bash`
- `zsh`
- `PowerShell`

### Exemple

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

### Configuration

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

#### Options

| Option          | Défaut                       | Description                                                                                                               |
| --------------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `format`        | `'[$symbol$model]($style) '` | Format du module.                                                                                         |
| `symbole`       | `'🤖 '`                      | The symbol shown before the model name.                                                                   |
| `style`         | `'bold blue'`                | Le style pour le module.                                                                                  |
| `model_aliases` | `{}`                         | Map of model IDs or display names to shorter aliases. Checks ID first, then display name. |
| `disabled`      | `false`                      | Disables the `claude_model` module.                                                                       |

#### Variables

| Variable                      | Exemple             | Description                            |
| ----------------------------- | ------------------- | -------------------------------------- |
| model                         | `Claude 3.5 Sonnet` | The display name of the current model  |
| model_id | `claude-3-5-sonnet` | The model ID                           |
| symbole                       |                     | Reflète la valeur de l'option `symbol` |
| style\*                       |                     | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'un style

#### Exemples

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

#### Options

| Option                 | Défaut                            | Description                                                        |
| ---------------------- | --------------------------------- | ------------------------------------------------------------------ |
| `format`               | `'[$gauge $percentage]($style) '` | Format du module.                                  |
| `symbole`              | `''`                              | The symbol shown before the gauge.                 |
| `gauge_width`          | `5`                               | The width of the gauge in characters.              |
| `gauge_full_symbol`    | `'█'`                             | The symbol used for filled segments of the gauge.  |
| `gauge_partial_symbol` | `'▒'`                             | The symbol used for partial segments of the gauge. |
| `gauge_empty_symbol`   | `'░'`                             | The symbol used for empty segments of the gauge.   |
| `display`              | [voir plus bas](#display)         | Threshold and style configurations.                |
| `disabled`             | `false`                           | Disables the `claude_context` module.              |

##### Display

The `display` option is an array of objects that define thresholds and styles for different usage levels. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Option      | Défaut       | Description                                                              |
| ----------- | ------------ | ------------------------------------------------------------------------ |
| `threshold` | `0.0`        | The minimum context windows usage percentage to match this configuration |
| `style`     | `bold green` | The value of `style` if this display configuration is matched            |
| `hidden`    | `false`      | Hide this module if this the configuration is matched.   |

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

| Variable                                                                                  | Exemple | Description                                           |
| ----------------------------------------------------------------------------------------- | ------- | ----------------------------------------------------- |
| gauge                                                                                     | `██▒░░` | Visual representation of context usage                |
| percentage                                                                                | `65%`   | Context usage as a percentage                         |
| input_tokens                                                         | `45.2k` | Total input tokens in conversation                    |
| output_tokens                                                        | `12.3k` | Total output tokens in conversation                   |
| curr_input_tokens                               | `5.1k`  | Input tokens from most recent API call                |
| curr_output_tokens                              | `1.2k`  | Output tokens from most recent API call               |
| curr_cache_creation_tokens | `1.5k`  | Cache creation tokens from most recent API call       |
| curr_cache_read_tokens     | `23.4k` | Cache read tokens from most recent API call           |
| total_tokens                                                         | `200k`  | Total context window size                             |
| symbole                                                                                   |         | Reflète la valeur de l'option `symbol`                |
| style\*                                                                                   |         | Mirrors the style from the matching display threshold |

\* : Cette variable ne peut être utilisée que comme partie d'un style

#### Exemples

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

#### Options

| Option     | Défaut                             | Description                                         |
| ---------- | ---------------------------------- | --------------------------------------------------- |
| `format`   | `'[$symbol(\\$$cost)]($style) '` | Format du module.                   |
| `symbole`  | `'💰 '`                            | The symbol shown before the cost.   |
| `display`  | [voir plus bas](#display-1)        | Threshold and style configurations. |
| `disabled` | `false`                            | Disables the `claude_cost` module.  |

##### Display

The `display` option is an array of objects that define cost thresholds and styles. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Option      | Défaut       | Description                                                        |
| ----------- | ------------ | ------------------------------------------------------------------ |
| `threshold` | `0.0`        | The minimum cost in USD to match this configuration                |
| `style`     | `bold green` | The value of `style` if this display configuration is matched      |
| `hidden`    | `false`      | Hide this module if this configuration is matched. |

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

| Variable                           | Exemple  | Description                                                            |
| ---------------------------------- | -------- | ---------------------------------------------------------------------- |
| cost                               | `1.23`   | Total session cost in USD (formatted to 2 decimals) |
| duration                           | `1m 30s` | Total session duration                                                 |
| api_duration  | `45s`    | Total API call duration                                                |
| lines_added   | `1.2k`   | Total lines of code added                                              |
| lines_removed | `500`    | Total lines of code removed                                            |
| symbole                            |          | Reflète la valeur de l'option `symbol`                                 |
| style\*                            |          | Mirrors the style from the matching display threshold                  |

\* : Cette variable ne peut être utilisée que comme partie d'un style

#### Exemples

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

## Chaînes de style

Les chaines de style sont une liste de mots séparés par des espaces. Les mots ne sont pas sensibles à la casse (`bold` et `boLd` sont considérés comme la même chaine). Les mots peuvent être :

- `bold (gras)`
- `italic (italique)`
- `underline (souligné)`
- `dimmed (atténué)`
- `inverted (inversé)`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color> (arrière-plan)`
- `fg:<color> (avant-plan)`
- `<color>`
- `none (aucun)`

où `<color>` spécifie une couleur (voir ci-dessous). `fg:<color>` and `<color>` font la même chose actuellement, mais cela pourrait changer dans le futur.
`<color>` can also be set to `prev_fg` or `prev_bg` which evaluates to the previous item's foreground or background color respectively if available or `none` otherwise.
`inverted` inverse les couleurs d’arrière-plan et d’avant-plan. L’ordre des mots dans la chaine n’a pas d’importance.

La valeur `none` écrase toutes les autres dans une chaine si elle ne fait pas partie d’une déclaration `bg:`, donc par exemple `fg:red none fg:blue` va créer une chaine sans style. `bg:none` définit comme arrière-plan la couleur par défaut donc `fg:red bg:none` équivaut à `red` ou `fg:red` et `bg:green fg:red bg:none` équivaut aussi à `fg:red` or `red`. Cela pourrait devenir une erreur d’utiliser `none` avec d’autres mots dans le futur.

Une spécification de couleur peut être :

- Une des couleurs de terminal standard: `black` (noir), `red` (rouge), `green` (vert), `blue` (bleu), `yellow` (jaune), `purple` (violet), `cyan` (cyan), `white` (blanc). Vous pouvez éventuellement les préfixer avec `bright-` pour obtenir la version claire (par exemple `bright-white`).
- Un `#` suivi d’un nombre hexadécimal de 6 chiffres. Cela spécifie un [code de couleur RGB hexadécimal](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Un nombre entre 0 et 255. Cela spécifie un [code couleur ANSI 8-bit](https://i.stack.imgur.com/KTSQa.png).

Si plusieurs couleurs sont définies pour l’avant-plan/arrière-plan, la dernière dans le chaine sera prioritaire.

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default.
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app.
