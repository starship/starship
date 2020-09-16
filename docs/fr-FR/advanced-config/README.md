# Configuration avancée

Même si Starship est un shell polyvalent, éditer `starship.toml` ne suffit parfois pas pour faire certaines choses. Cette page détaille quelques techniques de configuration avancées utilisées dans starship.

::: warning

Les configurations dans cette section sont sujettes à modification dans les futures versions de Starship.

:::

## Personnalisation des commandes pré-commande et pré-exécution en Bash

Bash n'a pas de structure officielle préexec/précmd comme la plupart des autres shells. C'est pourquoi il est difficile de fournir des hooks entièrement personnalisables dans `bash`. Cependant, Starship vous donne une capacité limitée à insérer vos propres fonctions dans la procédure de rendu commande :

- Pour exécuter une fonction personnalisée juste avant que commande soit dessinée, définissez une nouvelle fonction et assignez son nom à `starship_precmd_user_func`. Par exemple, pour dessiner une fusée avant la commande, vous feriez

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Pour exécuter une fonction personnalisée juste avant l'exécution d'une commande, vous pouvez utiliser le [` DEBUG` mécanisme d'interruption ](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Cependant, vous **devez** pièger le signal DEBUG *avant* initialisation du Starship ! Starship peut préserver la valeur du piège DEBUG, mais si le piège est écrasé après le démarrage de Starship, certaines fonctionnalités vont casser.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Pièger DEBUG *avant* l'initiation de starship
eval $(starship init bash)
```

## Modifier le style des fenêtres commande

Certaines commandes du shell changeront automatiquement le titre de la fenêtre (par exemple, refléter votre répertoire de travail). Fish le fait par défaut. Starship ne le fait pas, mais il est assez simple d'ajouter cette fonctionnalité à `bash` ou `zsh`.

Tout d'abord, définir une fonction de changement de titre de fenêtre (identique en bash et zsh) :

```bash
function set_titre_fenetre(){
    echo -ne "\033]0; TON_TITRE_FENETRE_ICI \007"
}
```

Vous pouvez utiliser des variables pour personnaliser ce titre (`$USER`, `$HOSTNAME`, et `$PWD` sont des choix populaires).

Dans `bash`, définissez cette fonction comme la fonction précmd Starship :

```bash
starship_precmd_user_func="set_titre_gagnante"
```

Dans `zsh`, ajoutez ceci au tableau `precmd_functions` :

```bash
précmd_functions+=(set_titre_gagnant)
```

Si vous aimez le résultat, ajoutez ces lignes à votre fichier de configuration shell (`~/.bashrc` ou `~/.zshrc`) pour le rendre permanent.

Par exemple, si vous voulez afficher votre répertoire actuel dans votre titre d'onglets de terminal, ajoutez le snippet suivant à votre `~/.bashrc` ou `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename $PWD) \007"
}
starship_precmd_user_func="set_win_title"
```

## Chaînes de style

Les chaînes de style sont une liste de mots, séparés par des espaces. Les mots ne sont pas sensibles à la casse (c'est-à-dire `gras` et `GrAs` sont considérés comme le même mot). Chaque mot peut être l'un des suivants :

  - `bold`
  - `underline`
  - `dimmed`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

où `<color>` est un spécificateur de couleur (discuté ci-dessous). `fg:<color>` et `<color>` font actuellement la même chose, même si cela peut changer plus tard. L'ordre des mots dans le string n'a pas d'importance.

Le jeton ` none ` remplace tous les autres jetons d'une string, de sorte que, par exemple, ` fg: red none fg: blue ` créera toujours une string sans style. Il peut devenir une erreur d'utiliser `none` en conjonction avec d'autres jetons dans le futur.

Un spécificateur de couleur peut être l'un des éléments suivants :

 - Une des couleurs standard du terminal : `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Vous pouvez éventuellement les préfixer avec `bright-` pour obtenir la version lumineuse (par exemple `bright-white`).
 - Un `#` suivi d'un nombre hexadécimal de six chiffres. Ceci spécifie un [ Code hexadécimal de couleur RVB ](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Un nombre entre 0-255. Ceci spécifie un [code de couleur ANSI 8 bits](https://i.stack.imgur.com/KTSQa.png).

Si plusieurs couleurs sont spécifiées pour le premier plan/arrière-plan, la dernière dans le string prendra la priorité.
