# Configuration avancée

Même si Starship est un shell polyvalent, éditer `starship.toml` ne suffit parfois pas pour faire certaines choses. Cette page détaille quelques techniques de configuration avancées utilisées dans starship.

::: warning

Les configurations dans cette section sont sujettes à modification dans les futures versions de Starship.

:::

## Commandes pré-invite et pré-exécution personnalisées dans Cmd

Clink fournit des APIs extrêmement flexibles pour exécuter des commandes pre-invite et pre-exec dans Cmd. Il est assez simple à utiliser avec Starship. Effectuez les modifications suivantes dans votre fichier `starship.lua`, en fonction de vos besoins:

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

## Modifier le titre des fenêtres

Certaines commandes du shell changeront automatiquement le titre de la fenêtre (par exemple, pour refléter le dossier courant). Fish le fait même par défaut. Starship does not do this, but it's fairly straightforward to add this functionality to `bash`, `zsh`, `cmd` or `powershell`.

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

Certains shells peuvent gérer une invite de commande à droite, sur la même ligne que l’entrée. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd.

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

Les chaînes de style sont une liste de mots, séparés par des espaces blancs. Les mots ne sont pas sensibles à la casse (` bold ` et ` boLd ` sont considérés comme la même string). Chaque mot peut être l'un des suivants :

- `bold`
- `italic (italique)`
- `underline`
- `dimmed`
- `inverted`
- `bg:<couleur>`
- `fg:<couleur>`
- `<couleur>`
- `none`

où `<color>` est un spécificateur de couleur (discuté ci-dessous). `fg:<color>` et `<color>` font actuellement la même chose, bien que cela puisse changer dans le futur. `inverted` permute les couleurs de fond et de premier plan. L'ordre des mots dans le string n'a pas d'importance.

La valeur `none` remplace toutes les autres valeurs si elle n'est pas incluse dans un spécificateur `bg:`, de sorte que par exemple `fg: red none fg:blue` créera une chaîne sans style. `bg:none` définit l'arrière plan sur la couleur par défaut, donc `fg:red bg:none` est équivalent à `red` ou `fg:red` et `bg:green fg:red bg:none` est aussi équivalent à `fg:red` ou `red`. Il peut devenir une erreur d'utiliser `none` en conjonction avec d'autres jetons dans le futur.

Un spécificateur de couleur peut être l'un des éléments suivants :

- Une des couleurs de terminal standard: `black` (noir), `red` (rouge), `green` (vert), `blue` (bleu), `yellow` (jaune), `purple` (violet), `cyan` (cyan), `white` (blanc). Vous pouvez éventuellement les préfixer avec `bright-` pour obtenir la version claire (par exemple `bright-white`).
- Un `#` suivi d'un nombre hexadécimal de six chiffres. Ceci spécifie un [ Code hexadécimal de couleur RVB ](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Un nombre entre 0 et 255. Ceci spécifie un [code de couleur ANSI 8 bits](https://i.stack.imgur.com/KTSQa.png).

Si plusieurs couleurs sont spécifiées pour le premier plan/arrière-plan, la dernière dans le string prendra la priorité.
