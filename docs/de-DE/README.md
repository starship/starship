---
home: true
heroImage: /logo.svg
actionText: Loslegen →
actionLink: /guide/
footer: ICS lizenziert | Copyright © 2019-heute Starship-Mitwirkende
---

<div class="features">
  <div class="feature">
    <h2>Kompatibel</h2>
    <p>Läuft mit den beliebtesten Shells auf den beliebtesten Betriebssystemen. Überall einsetzbar!</p>
  </div>
  <div class="feature">
    <h2>Antrieben von Rust</h2>
    <p>Bringt die branchenweit beste Geschwindigkeit und Sicherheit von Rust mit, um Deine Eingaben so schnell und zuverlässig wie nur möglich zu machen.</p>
  </div>
  <div class="feature">
    <h2>Individualisierbar</h2>
    <p>Jedes noch so kleine Detail kann nach Deinen Wünschen angepasst werden, um die Eingabeaufforderung so minimal oder funktionsreich zu gestalten, wie Du es möchtest.</p>
  </div>
</div>

<div class="center">
  <video class="demo-video" autoplay muted loop>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Schnellinstallation

1. Installiere die Binärversion von **starship**:

   **[Lade die vorkompilierte Binärversion herunter](https://github.com/starship/starship/releases)**, wenn du keine der unten gelisteten Plattformen verwendest.


   #### Homebrew

   ```sh
   $ brew install starship
   ```


   #### Rust (v1.33 oder neuer)

   ```sh
   $ cargo install starship
   ```


   #### Arch Linux (AUR)

   Starship ist via AUR unter dem Namen `starship` verfügbar. Installiere es mittels `yay` oder einem AUR-Helfer deiner Wahl.

   ```sh
   $ yay -S starship
   ```


   #### Nix (instabil)

   ```sh
   $ nix-env --install starship
   ```


   #### Termux

   ```sh
   $ pkg install starship
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

   eval (starship init fish)
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
