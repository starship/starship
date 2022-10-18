---
home: true
heroImage: /logo.svg
heroText:
tagline: Minimale, super schnelle und unendlich anpassbare Prompt für jede Shell!
actionText: Loslegen →
actionLink: ./guide/
features:
  - 
    title: Kompatibel
    details: Läuft mit den beliebtesten Shells auf den beliebtesten Betriebssystemen. Überall einsetzbar!
  - 
    title: Rust-Powered
    details: Bringt die Schnelligkeit und Zuverlässigkeit von Rust in deine Shell-Prompt.
  - 
    title: Individualisierbar
    details: Jedes noch so kleine Detail kann nach Deinen Wünschen angepasst werden, um die Eingabeaufforderung so minimal oder funktionsreich zu gestalten, wie Du es möchtest.
footer: ICS lizenziert | Copyright © 2019-heute Starship-Mitwirkende
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship ist eine minimale, super schnelle, und extrem anpassbare Prompt für jede Shell! Sie zeigt die Information, die man benötigt an, während sie schnell und minimal bleibt. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Voraussetzungen

- Eine [Nerd Font](https://www.nerdfonts.com/) installiert und aktiviert in Ihrem Terminal.

### Schnellinstallation

1. Installiere die Binärversion von **starship**:


   #### Neueste Version installieren

   Mit Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Um Starship selbst zu aktualisieren, lasse das Skript oben nochmal laufen. Es wird die vorhandene Version ersetzen, ohne die Konfiguration von Starship zu berühren.


   #### Installation mithilfe eines Paket-Managers

   Mit [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Führe den init Befehl zum Start der Shell aus:


   #### Bash

   Füge dies ans Ende von `~/.bashrc`:

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

   Füge dies ans Ende von `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Füge das folgende zum Ende von `Microsoft.PowerShell_profile.ps1` hinzu. Sie können den Speicherort dieser Datei überprüfen, indem Sie die `$PROFILE` Variable in PowerShell abfragen. Normalerweise ist der Pfad `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` oder `~/.config/powershell/Microsoft.PowerShell_profile.ps1` auf -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Trage folgendes am Ende der `~/.config/ion/initrc` ein:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Only elvish v0.18 or higher is supported.

   :::

   Trage folgendes am Ende von `~/.config/fish/rc.elv` ein:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Trage folgendes am Ende von `~/.bashrc` ein:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   This will change in the future. Only Nushell v0.61+ is supported.

   :::

   Add the following to to the end of your Nushell env file (find it by running `$nu.env-path` in Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   ```

   And add the following to the end of your Nushell configuration (find it by running `$nu.config-path`):

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   Füge folgendes an das Ende von `~/.xonshrc` hinzu:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### ⌘ Cmd

   You need to use [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) with Cmd. Add the following to a file `starship.lua` and place this file in Clink scripts directory:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
