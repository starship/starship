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

### Installation

1. Installer le binaire **starship** :


   #### Installer la dernière version

   Avec Shell:

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

   Ajoutez ce qui suit à la fin de `Microsoft.PowerShell_profile.ps1`. Vous pouvez vérifier l'emplacement de ce fichier en regardant la variable `$PROFILE` dans PowerShell. Typically the path is `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Ajouter ce qui suit à la fin de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
