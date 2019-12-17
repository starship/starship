---
home: true
heroImage: /logo.svg
actionText: Commencez →
actionLink: /guide/
footer: ISC licencié | Copyright © 2019-present Starship Contributors
---

<div class="features">
  <div class="feature">
    <h2>Compatibilité d'abord</h2>
    <p>Fonctionne sur tous les principaux shells et système d'exploitation. Utilisez-le partout !</p>
  </div>
  <div class="feature">
    <h2>Propulsé par Rust</h2>
    <p>Profiter de toute la rapidité et la securité de Rust, pour rendre votre prompt le plus rapide et fiable possible.</p>
  </div>
  <div class="feature">
    <h2>Personnalisable</h2>
    <p>Tous les petits détails sont personnalisable à votre goût, pour rendre votre prompt aussi léger ou complet que le vous souhaitez.</p>
  </div>
</div>

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Installation

1. Installer le binaire **starship** :


   #### Install Latest Version

   With Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Install via Package Manager

   With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

    With [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Add the init script to your shell's config file:


   #### Bash

   Add the following to the end of `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Add the following to the end of `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Add the following to the end of `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Add the following to the end of `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix):

   ```sh
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Add the following to the end of `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
