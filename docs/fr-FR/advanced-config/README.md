# Configuration avancée

Même si Starship est un shell polyvalent, éditer `starship.toml` ne suffit parfois pas pour faire certaines choses. Cette page détaille quelques techniques de configuration avancées utilisées dans starship.

> [!WARNING] Les configurations de cette section sont susceptibles de changer dans les prochaines versions de Starship.

## TransientPrompt dans PowerShell

Il est possible de remplacer le prompt précédent avec une string personnalisée. Ceci est utile lorsque toutes les informations fournies par l'invité de commande ne sont pas nécessaire. Pour activer ceci, exécutez `Enable-TransientPrompt` dans la session shell. Pour que la modification soit permanente, ajoutez cette commande dans votre `$PROFILE`. La transience peut être désactivée à la volée avec `Disable-TransientPrompt`.

Par défaut, le côté gauche de l'entrée est remplacé par `>`. Pour personnaliser cela, définissez une nouvelle fonction appelée `Invoke-Starship-TransientFunction`. Par exemple, pour afficher le module `character` de Starship ici, vous feriez

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt et TransientRightPrompt dans Cmd

Clink vous permet de remplacer l'invite précédemment affichée avec des chaînes personnalisées. Ceci est utile lorsque toutes les informations fournies par l'invité de commande ne sont pas nécessaire. Pour activer cela, exécutez `clink set prompt.transient <value>` où \<value\> peut être :

- `always` : toujours remplacer l'invite précédente
- `same_dir` : remplacer l'invite précédente uniquement si le répertoire de travail est le même
- `off` : ne pas remplacer l'invite (c'est-à-dire désactiver la transience)

Il est nécessaire de faire cela qu'une fois. Modifiez votre `starship.lua` pour customiser ce qui sera présenté à gauche et à droite:

- Par défaut, le côté gauche de l'entrée est remplacé par `>`. Pour personnaliser cela, définissez une nouvelle fonction appelée `starship_transient_prompt_func`. Cette fonction reçoit l'invite courante sous forme de chaîne que vous pouvez utiliser. Par exemple, pour afficher le module `character` de Starship ici, vous feriez

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- Par défaut, le côté droit de l'entrée est vide. Pour personnaliser cela, définissez une nouvelle fonction appelée `starship_transient_rprompt_func`. Cette fonction reçoit l'invite courante sous forme de chaîne que vous pouvez utiliser. Par exemple, pour afficher l'heure à laquelle la dernière commande a été lancée ici, vous feriez

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt et TransientRightPrompt dans Fish

Il est possible de remplacer le prompt précédent avec une string personnalisée. Ceci est utile lorsque toutes les informations fournies par l'invité de commande ne sont pas nécessaire. Pour activer cela, exécutez `enable_transience` dans la session shell. Pour le rendre permanent, ajoutez cette commande dans votre `~/.config/fish/config.fish`. La transience peut être désactivée à la volée avec `disable_transience`.

Notez que dans le cas de Fish, l'invite transitoire n'est affichée que si la ligne de commande est non vide et syntaxiquement correcte.

- Par défaut, le côté gauche de l'entrée est remplacé par un `❯` vert gras. Pour personnaliser cela, définissez une nouvelle fonction appelée `starship_transient_prompt_func`. Par exemple, pour afficher le module `character` de Starship ici, vous feriez

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- Par défaut, le côté droit de l'entrée est vide. Pour personnaliser cela, définissez une nouvelle fonction appelée `starship_transient_rprompt_func`. Par exemple, pour afficher l'heure à laquelle la dernière commande a été lancée ici, vous feriez

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt et TransientRightPrompt dans Bash

Le framework [Ble.sh](https://github.com/akinomyoga/ble.sh) en version 0.4 ou supérieure vous permet de remplacer l'invite précédemment affichée avec des chaînes personnalisées. Ceci est utile lorsque toutes les informations de l'invite ne sont pas toujours nécessaires. Pour activer cela, ajoutez ceci dans `~/.bashrc` : `bleopt prompt_ps1_transient=<value>` :

La \<value\> ici est une liste séparée par des deux-points de `always`, `same-dir` et `trim`. Quand `prompt_ps1_final` est vide et que l'option `prompt_ps1_transient` a une \<value\> non vide, l'invite spécifiée par `PS1` est effacée en quittant la ligne de commande actuelle. Si \<value\> contient le champ `trim`, seule la dernière ligne d'un `PS1` multiligne est conservée et les autres lignes sont effacées. Sinon, la ligne de commande sera redessinée comme si `PS1=` était spécifié. Quand le champ `same-dir` est contenu dans \<value\> et que le répertoire de travail actuel est différent du répertoire final de la ligne de commande précédente, cette option `prompt_ps1_transient` est ignorée.

Effectuez les modifications suivantes dans votre `~/.blerc` (ou dans `~/.config/blesh/init.sh`) pour personnaliser ce qui est affiché à gauche et à droite :

- Pour personnaliser ce par quoi le côté gauche de l'entrée est remplacé, configurez l'option Ble.sh `prompt_ps1_final`. Par exemple, pour afficher le module `character` de Starship ici, vous feriez

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- Pour personnaliser ce par quoi le côté droit de l'entrée est remplacé, configurez l'option Ble.sh `prompt_rps1_final`. Par exemple, pour afficher l'heure à laquelle la dernière commande a été lancée ici, vous feriez

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
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Mettre l’invite à droite

Certains shells peuvent gérer une invite de commande à droite, sur la même ligne que l’entrée utilisateur. Starship peut définir le contenu de cet invite à droite en utilisant l’option `right_format`. N’importe quel module qui peut être utilisé dans `format` est aussi géré dans `right_format`. La variable `$all` va seulement contenir les modules qui ne sont explicitement utilisés ni dans `format`, ni dans `right_format`.

Note : l’invite à droite est une seule ligne, sur la même ligne que l’entrée. Pour aligner à droite des modules au-dessus de la ligne d’entrée dans une invite multi-ligne, consultez le [module `fill`](../config/#fill).

`right_format` est actuellement supporté pour les shells suivants : elvish, fish, zsh, xonsh, cmd, nushell, bash.

Note : Le framework [Ble.sh](https://github.com/akinomyoga/ble.sh) en version 0.4 ou supérieure doit être installé pour utiliser l'invite à droite dans bash.

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

Starship peut définir l’invite de continuation en utilisant l’option `continuation_prompt`. L’invite par défaut est `’[∙](bright-black) ‘`.

Note: la valeur de `continuation_prompt` doit être une chaine littérale, sans variable.

Note: les invites de confirmation sont uniquement disponibles pour les shells suivants:

- `bash`
- `zsh`
- `PowerShell`

### Exemple

```toml
# ~/.config/starship.toml

# Une invite de continuation qui affiche deux flèches pleines
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

où `<color>` spécifie une couleur (voir ci-dessous). `fg:<color>` et `<color>` font la même chose actuellement, mais cela pourrait changer dans le futur. `<color>` peut également être défini à `prev_fg` ou `prev_bg` qui évalue respectivement la couleur d'avant-plan ou d'arrière-plan de l'élément précédent si disponible, ou `none` sinon. `inverted` inverse les couleurs d’arrière-plan et d’avant-plan. L’ordre des mots dans la chaine n’a pas d’importance.

La valeur `none` écrase toutes les autres dans une chaine si elle ne fait pas partie d’une déclaration `bg:`, donc par exemple `fg:red none fg:blue` va créer une chaine sans style. `bg:none` définit comme arrière-plan la couleur par défaut donc `fg:red bg:none` équivaut à `red` ou `fg:red` et `bg:green fg:red bg:none` équivaut aussi à `fg:red` or `red`. Cela pourrait devenir une erreur d’utiliser `none` avec d’autres mots dans le futur.

Une spécification de couleur peut être :

- Une des couleurs de terminal standard: `black` (noir), `red` (rouge), `green` (vert), `blue` (bleu), `yellow` (jaune), `purple` (violet), `cyan` (cyan), `white` (blanc). Vous pouvez éventuellement les préfixer avec `bright-` pour obtenir la version claire (par exemple `bright-white`).
- Un `#` suivi d’un nombre hexadécimal de 6 chiffres. Cela spécifie un [code de couleur RGB hexadécimal](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Un nombre entre 0 et 255. Cela spécifie un [code couleur ANSI 8-bit](https://i.stack.imgur.com/KTSQa.png).

Si plusieurs couleurs sont définies pour l’avant-plan/arrière-plan, la dernière dans le chaine sera prioritaire.

Toutes les chaînes de style ne seront pas affichées correctement par tous les terminaux. En particulier, les limitations connues suivantes existent :

- De nombreux terminaux désactivent le support de `blink` par défaut.
- `hidden` n'est [pas supporté sur iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` n'est pas supporté par le Terminal.app par défaut de macOS.
