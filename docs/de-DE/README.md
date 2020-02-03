---
home: true
heroImage: /logo.svg
heroText: null
tagline: The cross-shell prompt for astronauts
actionText: Loslegen →
actionLink: ./guide/
footer: ICS lizenziert | Copyright © 2019-heute Starship-Mitwirkende
---

<div class="features">
  <div class="feature">
    <h2>Kompatibel</h2>
    <p>Läuft mit den beliebtesten Shells auf den beliebtesten Betriebssystemen. Überall einsetzbar!</p>
  </div>
  <div class="feature">
    <h2>Rust-Powered</h2>
    <p>Bringt die schnelligkeit und zuverlässigkeit von Rust in deinen Shell-prompt.</p>
  </div>
  <div class="feature">
    <h2>Individualisierbar</h2>
    <p>Jedes noch so kleine Detail kann nach Deinen Wünschen angepasst werden, um die Eingabeaufforderung so minimal oder funktionsreich zu gestalten, wie Du es möchtest.</p>
  </div>
</div>

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Schnellinstallation

1. Installiere die Binärversion von **starship**:


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

1. Füge das init-Skript zur Konfigurationsdatei deiner Shell hinzu:


   #### Bash

   Trage folgendes am Ende der `~/.bashrc` ein:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Trage folgendes am Ende der `~/.config/fish/config.fish` ein:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Trage folgendes am Ende der `~/.zshrc` ein:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Trage folgendes in das Powershell-Profil ($Profile) ein. Standardmäßig gespeichert unter: `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` auf Windows, oder `~/.config/powershell/Microsoft.PowerShell_profile.ps1` auf -Nix:

   ```sh
   # notepad $PROFILE

# ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Add the following to the end of `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
