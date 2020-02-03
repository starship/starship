---
home: true
heroImage: /logo.svg
heroText:
tagline: The cross-shell prompt for astronauts
actionText: Commencez →
actionLink: ./guide/
features:
  - 
    title: Compatibilité d'abord
    details: Fonctionne sur tous les principaux shells et système d'exploitation. Utilisez-le partout !
  - 
    title: Propulsé par Rust
    details: Profiter de toute la rapidité et la securité de Rust, pour rendre votre prompt le plus rapide et fiable possible.
  - 
    title: Personnalisable
    details: Tous les petits détails sont personnalisable à votre goût, pour rendre votre prompt aussi léger ou complet que le vous souhaitez.
footer: ISC licencié | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and Powershell.
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

1. Ajouter le script d'initialization à la fiche config de votre shell:


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

   Ajouter ce qui suit à la fin de `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (ou `~/.config/powershell/Microsoft.PowerShell_profile.ps1` sur -Nix):

   ```sh
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Ajouter ce qui suit à la fin de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
