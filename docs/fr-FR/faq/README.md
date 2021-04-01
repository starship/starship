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

L'aide à la complétion ou autocomplétion est fournie par le shell que vous avez choisi. Dans le cas de la démo, elle a été faite avec [Fish Shell](https://fishshell.com/), qui fournit des complétions par défaut. Si vous utilisez le Shell Z (zsh), vous pouvez jeter un œil à [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Est-ce que l'option globale `format` et `<module>.disabled` font la même chose ?

Oui, ils peuvent tous deux être utilisés pour désactiver les modules dans l'invite de commande. Si tout ce que vous prévoyez de faire est de désactiver les modules, `<module>.disabled` est le meilleur moyen de le faire pour ces raisons :

- Désactiver les modules est plus explicite que de les omettre du `format global`
- Les modules nouvellement créés seront ajoutés à l'invite de commande au fur et à mesure que Starship sera mis à jour

## The docs say Starship is cross-shell. Why isn't my preferred shell supported?

Étant donné la façon dont Starship est construit, il devrait être possible d'ajouter le support pour pratiquement n'importe quel shell. Starship est sans état et agnostique, donc tant que votre shell supporte la personnalisation rapide et l'expansion, Starship peut être utilisé.

Voici un petit exemple pour que Starship fonctionne avec bash :

```sh
# Récupère le code d'état de la dernière commande exécutée
STATUS=$?

# Récupère le nombre de tâches en cours d'exécution.
NUM_JOBS=$(jobs -p | wc -l)

# Définit l'invite de commande a `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

The [Bash implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) built into Starship is slightly more complex to allow for advanced features like the [Command Duration module](https://starship.rs/config/#command-duration) and to ensure that Starship is compatible with pre-installed Bash configurations.

Pour une liste de tous les flags acceptés par `starship`, utilisez la commande suivante :

```sh
starship prompt --help
```

L'invite utilisera autant de contexte que possible, mais aucun paramètre n'est "requis".

## Comment utiliser Starship sur des distributions Linux avec des versions de glibc plus ancienne ?

Si vous obtenez une erreur du type "_version 'GLIBC_2.18' not found (required by starship)_" lors de l'utilisation de l'exécutable précompilé (par exemple sur CentOS 6 ou 7), vous pouvez utiliser un exécutable compilé avec `musl` au lieu de `glibc`:

```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- --platform unknown-linux-musl
```

## I see symbols I don't understand or expect, what do they mean?

If you see symbols that you don't recognise you can use `starship explain` to explain the currently showing modules.

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

- Votre locale est définie à une valeur UTF-8, comme `de_DE.UTF-8` ou `ja_JP.UTF-8`. Si `LC_ALL` n'est pas une valeur UTF-8, [vous aurez besoin de la modifier](https://www.tecmint.com/set-system-locales-in-linux/).
- Vous avez une police emoji installée. La plupart des systèmes sont fournis avec une police emoji par défaut, mais certains (notamment Arch Linux) ne le font pas. Vous pouvez habituellement en installer un par le biais du gestionnaire de paquets de votre système -[noto emoji](https://www.google.com/get/noto/help/emoji/) est un choix populaire.
- Vous utilisez une police [Nerd Font](https://www.nerdfonts.com/).

To test your system, run the following commands in a terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## How do I uninstall Starship?

Starship is just as easy to uninstall as it is to install in the first place.

1. Supprimez les lignes de la configuration de votre shell (par exemple `~/.bashrc`) utilisées pour initialiser Starship.
1. Supprimer l'exécutable de Starship.

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

If Starship was installed using the `curl | bash` script, the following command will delete the binary:

```sh
# Locate and delete the starship binary
bash -c 'rm "$(which starship)"'
```
