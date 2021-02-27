---
home: true
heroImage: /logo.svg
heroText:
tagline: L'invite minimaliste, ultra-rapide et personnalisable à l'infini pour n'importe quel shell !
actionText: Commencez →
actionLink: ./guide/
features:
  - 
    title: Compatibilité avant tout
    details: Fonctionne sur tous les principaux shells et système d'exploitation. Utilisez-le partout !
  - 
    title: Propulsé par Rust
    details: Profiter de toute la rapidité et la sécurité de Rust pour rendre votre invite de commandes le plus rapide et fiable possible.
  - 
    title: Personnalisable
    details: Tous les petits détails sont personnalisable à votre goût, pour rendre votre invite de commandes aussi léger ou complet que le vous souhaitez.
footer: Licence ISC | Copyright © 2019-présent Contributeurs Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship : Invite Multi-Shell"
description: Starship est un invite minimaliste, ultra-rapide et hautement personnalisable pour n'importe quel shell ! Montre les informations dont vous avez besoin tout en restant élégant et minimaliste. Installation rapide disponible pour Bash, Fish, ZSH, Ion et PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

## Installation

### Prérequis

* Une police d'écriture dite "Nerd-font" installée et configurée dans votre terminal (par exemple, [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads).

**Note** : les instructions ci-dessous ne concernent que les principales plateformes supportées. Vous ne voyer pas la votre ? Regardez [cette page](https://starship.rs/fr-fr/config/).

1. Installer l'exécutable **starship** (précompilé) :

   #### Installer la dernière version

   Depuis le shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Installer via le gestionnaire de paquets

   Avec [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Avec [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Ajouter le script d’initialisation au fichier configuration de votre shell:

   #### Bash

   Ajouter ce qui suit à la fin de `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```

   #### Fish

   Ajoute ce qui suit à la fin de `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```

   #### Zsh

   Ajouter ce qui suit à la fin de `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```

   #### Powershell

   Ajoutez ce qui suit à la fin de `Microsoft.PowerShell_profile.ps1`. Vous pouvez vérifier l'emplacement de ce fichier en affichant la valeur de la variable `$PROFILE` dans le PowerShell. Le chemin est habituellement `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` ou `~/.config/powershell/Microsoft.PowerShell_profile.ps1` sous -nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```

   #### Ion

   Ajouter ce qui suit à la fin de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

   #### Elvish

   **Attention** : ne supporte elvish qu'à partir de la v0.15

   Ajouter ce qui suit à la fin de `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```

# 🤝 Contribuer

Nous acceptons des contributeurs de tous niveaux ! Si vous souhaitez participer à ce projet, commencez par ouvrir une première \[[issue](https://github.com/starship/starship/labels/%F0%9F%8C%B1%20good%20first%20issue)\] (rapport de bug/demande de fonctionnalité/etc).

Si vous ne parlez pas anglais, nous apprécierions vraiment que vous nous aidiez à mettre à jour les traductions en diverses langues. Si vous souhaitez nous aider, vous pouvez traduire la documentation (complète) en français sur le [Crowdin de Starship](https://translate.starship.rs/project/starship-prompt/fr#) ou [en d'autres langues](https://translate.starship.rs/).

Si vous réfléchissez à contribuer à starship, nous vous remercierions de lire le [Guide du Contributeur](https://github.com/starship/starship/blob/master/CONTRIBUTING.md) (en anglais). N'hésitez pas rejoindre notre serveur [Discord](https://discord.gg/8Jzqu3T) pour nous passer le bonjour ! 👋

# [Contributeurs de code](https://github.com/starship/starship/#code-contributors)

# [Contributeurs financiers](https://github.com/starship/starship/#financial-contributors)

# 💭 Inspiré par

Merci à ces travaux en amont ayant permis à starship d'être développé. :pray:

*traduction des slogans anglais :*

  * [denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt) - *Un prompt ZSH pour astronautes*.
  * [denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node) - *Thème multiplateforme robbyrussell écrit en Javascript*
  * [reujab/silver](https://github.com/reujab/silver) - *Un prompt multiplateforme et configurable du style de powerline à icones*.

# Lisense

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).

Ce projet est sous lisense (ISC](https://github.com/starship/starship/blob/master/LICENSE).

---

Plus d'informations générales sur la [page anglophone](https://github.com/starship/starship/blob/master/README.md), sur [starship.rs](https://starship.rs/fr-FR/) et dans la [documentation](https://starship.rs/fr-FR/guide/#🚀-installation).
