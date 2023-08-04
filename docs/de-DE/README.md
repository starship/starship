---
home: true
heroImage: /logo.svg
heroText:
tagline: Minimale, super schnelle und unendlich anpassbare Prompt für jede Shell!
actionText: Loslegen →
actionLink: ./de-DE/guide/
features:
  - 
    title: Kompatibel
    details: Läuft mit den beliebtesten Shells auf den beliebtesten Betriebssystemen. Überall einsetzbar!
  - 
    title: Rust-Powered
    details: Bringt die Schnelligkeit und Sicherheit von Rust in deine Shell-Prompt.
  - 
    title: Individualisierbar
    details: Jedes noch so kleine Detail kann nach Deinen Wünschen angepasst werden, um die Eingabeaufforderung so minimal oder funktionsreich zu gestalten, wie Du es möchtest.
footer: ICS lizenziert | Copyright © 2019-heute Starship-Mitwirkende
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship ist eine minimale, super schnelle, und extrem anpassbare Prompt für jede Shell! Sie zeigt die Information, die man benötigt an, während sie schnell und minimal bleibt. Schnell-Installation verfügbar für Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, und PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Voraussetzungen

- Eine [Nerd Font](https://www.nerdfonts.com/) installiert und aktiviert in deinem Terminal.

### Schnellinstallation

1. Installiere die Binärversion von **starship**:


   #### Neueste Version installieren

   Mit Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Um Starship selbst zu aktualisieren, führe das Skript oben erneut aus. Die vorhandene Version wird ersetzt, ohne das deine Konfiguration von Starship verloren geht.


   #### Installation mithilfe eines Paket-Managers

   Mit [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```
   Mit [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
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

   Trage das folgende am Ende von `Microsoft.PowerShell_profile.ps1` ein. Du kannst den Speicherort dieser Datei überprüfen, indem du die `$PROFILE` Variable in PowerShell abfragst. Der Pfat lautet normalerweise `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` unter Windows und `~/.config/powershell/Microsoft.PowerShell_profile.ps1` auf -Nix.

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

   Es wird nur elvish v0.18 oder höher unterstützt.

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

   Das wird sich in Zukunft ändern. Nur Nushell v0.78+ wird unterstützt.

   :::

   Add the following to the end of your Nushell env file (find it by running `$nu.env-path` in Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   Und füge folgendes am Ende deiner Nushell-Konfiguration hinzu (du findest diese, indem du folgenden Befehl in Nushell ausführst `$nu.config-path`):

   ```sh
   use ~/.cache/starship/init.nu
   ```


   #### Xonsh

   Füge folgendes an das Ende von `~/.xonshrc` hinzu:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### ⌘ Cmd

   Du musst [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) mit Cmd verwenden. Trage folgendes in eine neue Datei namens `starship.lua` hinzu und lege diese Datei im Clink Scripts Verzeichnis ab:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
