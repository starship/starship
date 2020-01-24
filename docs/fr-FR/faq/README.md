# Foire aux questions

## Quelle est la configuration utilisée dans le GIF de démonstration ?

- **Émulateur de terminal**: [iTerm2](https://iterm2.com/)
  - **Thème** : Minimal
  - **Palette de couleurs**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Police d'écriture**: [Fira Code](https://github.com/tonsky/FiraCode)
- **Shell** : [Fish Shell](https://fishshell.com/)
  - **Configuration**: [Dotfiles de matchai](https://github.com/matchai/dotfiles/blob/master/.config/fish/config.fish)
  - **Invite de commande**: [Starship](https://starship.rs/)

## Est-ce que `prompt_order` et `<module>.disabled` font la même chose ?

Oui, ils peuvent tous deux être utilisés pour désactiver les modules dans l'invite de commande. Si tout ce que vous prévoyez de faire est de désactiver les modules, `<module>.disabled` est le meilleur moyen de le faire pour ces raisons :

- Désactiver les modules est plus explicite que de les omettre dans le prompt_order
- Les modules nouvellement créés seront ajoutés à l'invite de commande au fur et à mesure que Starship sera mis à jour

## La doc dit que Starship est cross-shell, mais il ne supporte pas X shell. Pourquoi ?

Étant donné la façon dont Starship est construit, il devrait être possible d'ajouter le support pour pratiquement n'importe quel shell. Le binaire de Starship est sans état et agnostique, donc tant que votre shell supporte la personnalisation rapide et l'expansion du shell, Starship peut être utilisé.

Voici un petit exemple pour que Starship fonctionne avec bash :

```sh
# Récupère le code d'état de la dernière commande exécutée
STATUS=$?

# Récupère le nombre de tâches en cours d'exécution.
NUM_JOBS=$(jobs -p | wc -l)

# Définit l'invite de commande `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=NUM_JOBS)"
```

[L'implémentation Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) intégrée dans Starship est légèrement plus complexe pour permettre des fonctionnalités avancées comme le [module Durée de commande](https://starship.rs/config/#Command-Duration) et pour s'assurer que Starship est compatible avec les configurations Bash préinstallées.

Pour une liste de tous les flags acceptés par `starship prompt`, utilisez la commande suivante :

```sh
starship prompt --help
```

L'invite de commande utilisera toutes les données contextuelles fournies, mais aucun indicateur n'est "requis".
