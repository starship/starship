# Foire aux questions

## Quelle est la configuration utilisée dans le GIF de démonstration ?

- **Émulateur de terminal**: [iTerm2](https://iterm2.com/)
  - **Thème** : Minimal
  - **Palette de couleurs**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Police**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell** : [Fish Shell](https://fishshell.com/)
  - **Configuration**: [Dotfiles de matchai](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Invite de commande**: [Starship](https://starship.rs/)

## Comment puis-je obtenir la complétion de commandes comme montré dans le GIF de démo?

L'aide à la complétion ou autocomplétion est fournie par le shell que vous avez choisi. Dans le cas de la démo, elle a été faite avec [Fish Shell](https://fishshell.com/), qui fournit des complétions par défaut. Si vous utilisez le Z Shell (zsh), vous pouvez jeter un œil à [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Est-ce que l'option globale `format` et `<module>.disabled` font la même chose ?

Oui, ils peuvent tous deux être utilisés pour désactiver les modules dans l'invite de commande. Si tout ce que vous prévoyez de faire est de désactiver les modules, `<module>.disabled` est le meilleur moyen de le faire pour ces raisons :

- Désactiver les modules est plus explicite que de les omettre du `format global`
- Les modules nouvellement créés seront ajoutés à l'invite de commande au fur et à mesure que Starship sera mis à jour

## La documentation dit que Starship est shell-agnostique. Pourquoi mon shell préféré n'est-il pas pris en charge ?

Étant donné la façon dont Starship est construit, il devrait être possible d'ajouter le support pour pratiquement n'importe quel shell. Le binaire de Starship est sans état et agnostique, donc tant que votre shell supporte la personnalisation rapide et l'expansion du shell, Starship peut être utilisé.

Voici un petit exemple pour que Starship fonctionne avec bash :

```sh
# Récupère le code d'état de la dernière commande exécutée
STATUS=$?

# Récupère le nombre de tâches en cours d'exécution.
NUM_JOBS=$(jobs -p | wc -l)

# Définit l'invite de commande a `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

L' [implémentation Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) intégrée à Starship est légèrement plus complexe pour permettre des fonctionnalités avancées comme le [module de durée de commande](https://starship.rs/config/#command-duration) et pour s'assurer que Starship est compatible avec les configurations Bash pré-installées.

Pour une liste de tous les flags acceptés par `starship prompt`, utilisez la commande suivante :

```sh
starship prompt --help
```

L'invite de commande utilisera toutes les données contextuelles fournies, mais aucun indicateur n'est "requis".

## Comment utiliser Starship sur des distributions Linux avec des versions de glibc plus ancienne ?

Si vous obtenez une erreur du type "_version 'GLIBC_2.18' not found (required by starship)_" lors de l'utilisation du binaire précompilé (par exemple sur CentOS 6 ou 7), vous pouvez utiliser un binaire compilé avec `musl` au lieu de `glibc`:

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## Pourquoi je vois des avertissements `Executing command "..." timed out.`?

Starship exécute différentes commandes pour obtenir les informations à afficher dans l'invite, par exemple la version d'un programme ou l'état actuel de git. Pour s’assurer que starship ne soit pas bloqué par l’exécution de ces commandes, nous mettons une limite de temps. Si une commande dépasse cette limite, starship va arrêter l’exécution de la commande et afficher l’avertissement ci-dessus, c’est un comportement attendu. Cette limite de temps est configurable en utilisant la [clé `command_timeout`](/config/#prompt) si vous souhaitez l’augmenter. Vous pouvez également suivre les étapes de débogage ci-dessous pour voir quelle commande est lente et voir si vous pouvez l’optimiser. Enfin, vous pouvez définir la variable `STARSHIP_LOG` à `error` pour masquer ces avertissements.

## Je vois des symboles que je ne comprends pas et auxquels je ne m'attendais pas, que signifient-t-ils ?

Si vous voyez des symboles que vous ne reconnaissez pas, vous pouvez utiliser `starship explain` pour obtenir des explications concernant les modules actuellement affichés.

## Starship fait quelque chose d’inattendu, comment puis-je le déboguer ?

Vous pouvez activer les journaux de débogage en utilisant la variable d’environnement `STARSHIP_LOG`. Ces journaux peuvent être très verbeux, donc il est souvent utile d'utiliser la commande `module` si vous essayez de déboguer un module particulier ; par exemple, si vous essayez de déboguer le module `rust`, vous pouvez exécuter la commande suivante pour récupérer les journaux de suivi et les sorties du module.

```sh
env STARSHIP_LOG=trace starship module rust
```

Si starship est lent, vous pouvez essayer d’utiliser la commande `timings` pour voir si un module ou une commande particulière est à blâmer.

```sh
env STARSHIP_LOG=trace starship timings
```

Cela affichera le journal de suivi et un détail de tous les modules qui ont soit pris plus d’1ms pour s’exécuter, soit affiché quelque chose.

Finalement, si vous trouvez un bug, vous pouvez utiliser la commande `bug-report` pour créer un ticket GitHub.

```sh
starship bug-report
```

## Pourquoi ne vois-je pas de glyphe dans mon invite?

La cause la plus commune est la mauvaise configuration du système. Certaines distributions Linux ne sont pas équipées du support de police 'out of the box'. Vous devez vous assurer que:

- Votre locale est définie à une valeur UTF-8, comme `de_DE.UTF-8` ou `ja_JP.UTF-8`. Si `LC_ALL` n'est pas une valeur UTF-8, [vous aurez besoin de la modifier](https://www.tecmint.com/set-system-locales-in-linux/).
- Vous avez une police emoji installée. La plupart des systèmes sont fournis avec une police emoji par défaut, mais certains (notamment Arch Linux) ne le font pas. Vous pouvez habituellement en installer un par le biais du gestionnaire de paquets de votre système -[noto emoji](https://www.google.com/get/noto/help/emoji/) est un choix populaire.
- Vous utilisez une police [Nerd Font](https://www.nerdfonts.com/).

Pour tester votre système, exécutez les commandes suivantes dans un terminal :

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

La première ligne doit produire un emoji [serpent](https://emojipedia.org/snake/), tandis que la seconde doit produire un symbole [de branche powerline (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

Si l'un ou l'autre des symboles ne parvient pas à s'afficher correctement, votre système est toujours mal configuré. Malheureusement, il est parfois difficile d'obtenir une configuration correcte. Les utilisateurs sur Discord peuvent être en mesure d'aider. Si les deux symboles s'affichent correctement, mais vous ne les voyez toujours pas dans starship, [soumettez un rapport de bogue !](https://github.com/starship/starship/issues/new/choose)

## Comment désinstaller Starship ?

Starship est tout aussi facile à désinstaller qu'à installer.

1. Supprimez les lignes de la configuration de votre shell (par exemple `~/.bashrc`) utilisées pour initialiser Starship.
1. Supprimez l'exécutable de Starship.

Si Starship a été installé à l'aide d'un gestionnaire de paquets, veuillez vous référer à leur documentation pour les instructions de désinstallation.

Si Starship a été installé en utilisant le script d'installation, la commande suivante supprimera l'exécutable :

```sh
# Trouver et supprimer le binaire starship
sh -c 'rm "$(command -v 'starship')"'
```
