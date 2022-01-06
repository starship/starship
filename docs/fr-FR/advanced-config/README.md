# Configuration avancée

Même si Starship est un shell polyvalent, éditer `starship.toml` ne suffit parfois pas pour faire certaines choses. Cette page détaille quelques techniques de configuration avancées utilisées dans starship.

::: warning

Les configurations dans cette section sont sujettes à modification dans les futures versions de Starship.

:::

## Commandes pré-commande et pré-exécution personnalisées en Bash

Bash n'a pas de structure officielle préexec/précmd comme la plupart des autres shells. C'est pourquoi il est difficile de fournir des hooks entièrement personnalisables dans `bash`. Cependant, Starship vous permet dans une certaine mesure d'insérer vos propres fonctions dans la procédure de rendu du prompt :

- Pour exécuter une fonction personnalisée juste avant que le prompt ne soit dessiné, définissez une nouvelle fonction et assignez son nom à `starship_precmd_user_func`. Par exemple, pour dessiner une fusée avant la commande, vous feriez

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Pour exécuter une fonction personnalisée juste avant l'exécution d'une commande, vous pouvez utiliser le [ mécanisme d'interruption du signal ` DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Cependant, vous **devez** piéger le signal DEBUG *avant* l'initialisation de Starship ! Starship peut préserver la valeur du piège DEBUG, mais si le piège est écrasé après le démarrage de Starship, certaines fonctionnalités vont casser.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Pièger DEBUG *avant* l'initialisation de starship
eval $(starship init bash)
```

## Commandes pré-invite et pré-exécution personnalisées dans PowerShell

Powershell n'a pas de système de préexec/précmd officiel comme la plupart des autres shells. Because of this, it is difficult to provide fully customizable hooks in `powershell`. Cependant, Starship vous permet dans une certaine mesure d'insérer vos propres fonctions dans la procédure de rendu du prompt :

Créez une fonction nommée `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Modifier le titre des fenêtres

Certaines commandes du shell changeront automatiquement le titre de la fenêtre (par exemple, pour refléter votre répertoire de travail). Fish le fait même par défaut. Starship ne le fait pas, mais il est assez simple d'ajouter cette fonctionnalité à `bash` ou `zsh`.

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

Par exemple, si vous voulez afficher votre répertoire actuel dans le titre de l'onglet de votre terminal, ajoutez le code suivant à votre `~/.bashrc` ou `~/.zshrc`:

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

## Mettre l’invite à droite

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh.

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

## Continuation Prompt

Some shells support a continuation prompt along with the normal prompt. This prompt is rendered instead of the normal prompt when the user has entered an incomplete statement (such as a single left parenthesis or quote).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `"[∙](bright-black) "`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Note: Continuation prompts are only available in the following shells:

  - `bash`
  - `zsh`
  - `PowerShell`

### Exemple

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled in arrows
continuation_prompt = "▶▶"
```

## Chaînes de style

Les chaines de style sont une liste de mots séparés par des espaces. Les mots ne sont pas sensibles à la casse (`bold` et `boLd` sont considérés comme la même chaine). Les mots peuvent être :

  - `bold (gras)`
  - `italic (italique)`
  - `underline (souligné)`
  - `dimmed (atténué)`
  - `inverted (inversé)`
  - `bg:<color> (arrière-plan)`
  - `fg:<color> (avant-plan)`
  - `<color>`
  - `none (aucun)`

où `<color>` spécifie une couleur (voir ci-dessous). `fg:<color>` and `<color>` font la même chose actuellement, mais cela pourrait changer dans le futur. `inverted` inverse les couleurs d’arrière-plan et d’avant-plan. L’ordre des mots dans la chaine n’a pas d’importance.

La valeur `none` écrase toutes les autres dans une chaine si elle ne fait pas partie d’une déclaration `bg:`, donc par exemple `fg:red none fg:blue` va créer une chaine sans style. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
