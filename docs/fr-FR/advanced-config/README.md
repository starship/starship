# Configuration avancée

Même si Starship est un shell polyvalent, éditer `starship.toml` ne suffit parfois pas pour faire certaines choses. Cette page détaille quelques techniques de configuration avancées utilisées dans starship.

::: warning

Les configurations dans cette section sont sujettes à modification dans les futures versions de Starship.

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

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. This function receives the current prompt as a string that you can utilize. Par exemple, pour afficher l'heure à laquelle la dernière commande a été lancée ici, vous feriez

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt et TransientRightPrompt dans Fish

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

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. Par exemple, pour afficher l'heure à laquelle la dernière commande a été lancée ici, vous feriez

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
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

Bash n'a pas de structure officielle préexec/précmd comme la plupart des autres shells. C'est pourquoi il est difficile de fournir des hooks entièrement personnalisables dans `bash`. Cependant, Starship vous permet dans une certaine mesure d'insérer vos propres fonctions dans la procédure de rendu du prompt :

- Pour exécuter une fonction personnalisée juste avant que le prompt ne soit dessiné, définissez une nouvelle fonction et assignez son nom à `starship_precmd_user_func`. Par exemple, pour dessiner une fusée avant la commande, vous feriez

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Pour exécuter une fonction personnalisée juste avant l'exécution d'une commande, vous pouvez utiliser le [ mécanisme d'interruption du signal ` DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Cependant, vous **devez** piéger le signal DEBUG _avant_ l'initialisation de Starship ! Starship peut préserver la valeur du piège DEBUG, mais si le piège est écrasé après le démarrage de Starship, certaines fonctionnalités vont casser.

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

Powershell n'a pas de système de préexec/précmd officiel comme la plupart des autres shells. C'est pourquoi il est difficile de fournir des hooks entièrement personnalisables dans `powershell`. Cependant, Starship vous permet dans une certaine mesure d'insérer vos propres fonctions dans la procédure de rendu du prompt :

Créez une fonction nommée `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Changer le titre de la fenêtre

Certaines commandes du shell changeront automatiquement le titre de la fenêtre (par exemple, pour refléter le dossier courant). Fish le fait même par défaut. Starship ne fait pas ça, mais c’est assez facile d’ajouter cette fonctionnalité à `bash`, `zsh`, `cmd` ou `powershell`.

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
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## Mettre l’invite à droite

Certains shells peuvent gérer une invite de commande à droite, sur la même ligne que l’entrée utilisateur. Starship peut définir le contenu de cet invite à droite en utilisant l’option `right_format`. N’importe quel module qui peut être utilisé dans `format` est aussi géré dans `right_format`. La variable `$all` va seulement contenir les modules qui ne sont explicitement utilisés ni dans `format`, ni dans `right_format`.

Note: l’invite à droite est une seule ligne, sur la même ligne que l’entrée. Pour aligner à droite les modules au-dessus de la ligne d’entrée d’une invite multiligne, voir le [module `fill`](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell.

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

# Un invite de continuation qui affiche deux flèches pleines
continuation_prompt = '▶▶ '
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

où `<color>` spécifie une couleur (voir ci-dessous). `fg:<color>` and `<color>` font la même chose actuellement, mais cela pourrait changer dans le futur. `inverted` inverse les couleurs d’arrière-plan et d’avant-plan. L’ordre des mots dans la chaine n’a pas d’importance.

La valeur `none` écrase toutes les autres dans une chaine si elle ne fait pas partie d’une déclaration `bg:`, donc par exemple `fg:red none fg:blue` va créer une chaine sans style. `bg:none` définit comme arrière-plan la couleur par défaut donc `fg:red bg:none` équivaut à `red` ou `fg:red` et `bg:green fg:red bg:none` équivaut aussi à `fg:red` or `red`. Cela pourrait devenir une erreur d’utiliser `none` avec d’autres mots dans le futur.

Une spécification de couleur peut être :

- Une des couleurs de terminal standard: `black` (noir), `red` (rouge), `green` (vert), `blue` (bleu), `yellow` (jaune), `purple` (violet), `cyan` (cyan), `white` (blanc). Vous pouvez éventuellement les préfixer avec `bright-` pour obtenir la version claire (par exemple `bright-white`).
- Un `#` suivi d’un nombre hexadécimal de 6 chiffres. Cela spécifie un [code de couleur RGB hexadécimal](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Un nombre entre 0 et 255. Cela spécifie un [code couleur ANSI 8-bit](https://i.stack.imgur.com/KTSQa.png).

Si plusieurs couleurs sont définies pour l’avant-plan/arrière-plan, la dernière dans le chaine sera prioritaire.

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app
