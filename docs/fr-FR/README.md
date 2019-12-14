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

   **[Télécharger les archives des binaires précompilés](https://github.com/starship/starship/releases)** si vous n'utilisez pas une des plateformes citées ci-dessous.


   #### Homebrew

   ```sh
   $ brew install starship
   ```


   #### Rust (v1.38 ou plus)

   ```sh
   $ cargo install starship
   ```


   #### Arch Linux (AUR)

   Starship is disponible sur AUR sous le nom `starship`. Installer le avec `yay` ou votre client AUR favori.

   ```sh
   $ yay -S starship
   ```


   #### Nix (non stable)

   ```sh
   $ nix-env --install starship
   ```


   #### Termux

   ```sh
   $ pkg install starship
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
