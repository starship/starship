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
description: Starship ist eine minimale, super schnelle, und extrem anpassbare Prompt für jede Shell! Sie zeigt die Information, die man benötigt an, während sie schnell und minimal bleibt. Schnell-Installation verfügbar für Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, und PowerShell.
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
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   Um Starship selbst zu aktualisieren, lasse das Skript oben nochmal laufen. Es wird die vorhandene Version ersetzen, ohne die Konfiguration von Starship zu berühren.


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

   ::: warning Nur elvish v0.15 oder höher ist unterstützt. :::

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

   ::: warning Dies wird sich in der Zukunft ändern. Es wird nur nu Version v0.33 oder höher unterstützt. ::: Füge Folgendes zu deiner nu Konfigurationsdatei hinzu. Du kannst den Speicherort dieser Datei überprüfen, indem du `config path` in nu ausführst.

   ```toml
   startup = [
    "mkdir ~/.cache/starship",
    "starship init nu | save ~/.cache/starship/init.nu",
    "source ~/.cache/starship/init.nu"
   ]
   prompt = "starship_prompt"
   ```


   #### Xonsh

   Füge folgendes an das Ende von `~/.xonshrc` hinzu:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```
