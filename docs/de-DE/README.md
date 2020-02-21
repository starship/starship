---
layout: Home
heroImage: /logo.svg
heroText:
tagline: The minimal, blazing-fast, and infinitely customizable prompt for any shell!
actionText: Loslegen →
actionLink: ./guide/
features:
  - 
    title: Kompatibel
    details: Läuft mit den beliebtesten Shells auf den beliebtesten Betriebssystemen. Überall einsetzbar!
  - 
    title: Rust-Powered
    details: Bringt die schnelligkeit und zuverlässigkeit von Rust in deinen Shell-prompt.
  - 
    title: Individualisierbar
    details: Jedes noch so kleine Detail kann nach Deinen Wünschen angepasst werden, um die Eingabeaufforderung so minimal oder funktionsreich zu gestalten, wie Du es möchtest.
footer: ICS lizenziert | Copyright © 2019-heute Starship-Mitwirkende
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Schnellinstallation

1. Installiere die Binärversion von **starship**:


   #### Neueste Version installieren

   With Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Installation mithilfe eines Paket-Managers

   Mit [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Mit [scoop](https://scoop.sh):

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
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Trage folgendes am Ende der `~/.config/ion/initrc` ein:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
