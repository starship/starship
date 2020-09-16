# Foire aux questions

## Quelle est la configuration utilisée dans le GIF de démonstration ?

- **Émulateur de terminal**: [iTerm2](https://iterm2.com/)
  - **Thème** : Minimal
  - **Palette de couleurs**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Police**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell** : [Fish Shell](https://fishshell.com/)
  - **Configuration**: [Dotfiles de matchai](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Invite de commande**: [Starship](https://starship.rs/)

## Est-ce que l'option globale `format` et `<module>.disabled` font la même chose ?

Oui, ils peuvent tous deux être utilisés pour désactiver les modules dans l'invite de commande. Si tout ce que vous prévoyez de faire est de désactiver les modules, `<module>.disabled` est le meilleur moyen de le faire pour ces raisons :

- Disabling modules is more explicit than omitting them from the top level `format`
- Les modules nouvellement créés seront ajoutés à l'invite de commande au fur et à mesure que Starship sera mis à jour

## La doc dit que Starship est cross-shell, mais il ne supporte pas X shell. Pourquoi ?

Étant donné la façon dont Starship est construit, il devrait être possible d'ajouter le support pour pratiquement n'importe quel shell. Le binaire de Starship est sans état et agnostique, donc tant que votre shell supporte la personnalisation rapide et l'expansion du shell, Starship peut être utilisé.

Voici un petit exemple pour que Starship fonctionne avec bash :

```sh
# Récupère le code d'état de la dernière commande exécutée
STATUS=$?

# Récupère le nombre de tâches en cours d'exécution.
NUM_JOBS=$(jobs -p | wc -l)

# Set the prompt to the output of `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

[L'implémentation Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) intégrée dans Starship est légèrement plus complexe pour permettre des fonctionnalités avancées comme le [module Durée de commande](https://starship.rs/config/#Command-Duration) et pour s'assurer que Starship est compatible avec les configurations Bash préinstallées.

Pour une liste de tous les flags acceptés par `starship prompt`, utilisez la commande suivante :

```sh
starship prompt --help
```

L'invite de commande utilisera toutes les données contextuelles fournies, mais aucun indicateur n'est "requis".

## Comment utiliser Starship sur des distributions Linux avec des versions de glibc plus ancienne ?

Si vous obtenez une erreur du type "_version 'GLIBC_2.18' not found (required by starship)_" lors de l'utilisation du binaire précompilé (par exemple sur CentOS 6 ou 7), vous pouvez utiliser un binaire compilé avec `musl` au lieu de `glibc`:

```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- --platform unknown-linux-musl
```

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

- Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. If `LC_ALL` is not a UTF-8 value, [you will need to change it](https://www.tecmint.com/set-system-locales-in-linux/).
- You have an emoji font installed. Most systems come with an emoji font by default, but some (notably Arch Linux) do not. You can usually install one through your system's package manager--[noto emoji](https://www.google.com/get/noto/help/emoji/) is a popular choice.
- You are using a [Nerd Font](https://www.nerdfonts.com/).

To test your system, run the following commands in a terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## How do I uninstall Starship?

Starship is just as easy to uninstall as it is to install in the first place.

1. Remove any lines in your shell config (e.g. `~/.bashrc`) used to initialize Starship.
1. Delete the Starship binary.

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

If Starship was installed using the `curl | bash` script, the following command will delete the binary:

```sh
# Locate and delete the starship binary
rm "$(which starship)"
```
