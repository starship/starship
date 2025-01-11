---
layout: home
hero:
  image: /logo.svg
  text:
  tagline: Der minimalistische, super schnelle und unendlich anpassbare Prompt für jede Shell!
  actions:
    - 
      theme: brand
      text: Loslegen →
      link: ./de-DE/guide/
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

<script setup>
import { onMounted } from 'vue'

onMounted(() => {
  const urlParams = new URLSearchParams(window.location.search)
  if (urlParams.has('uwu') || urlParams.has('kawaii')) {
    const img = document.querySelector('.VPHero .VPImage.image-src')
    img.classList.add('uwu')
    img.src = '/logo-uwu.png'
    img.alt = 'Kawaii Starship Logo by @sawaratsuki1004'
  }
})
</script>

<video class="demo-video" muted autoplay loop playsinline>
  <source src="/demo.webm" type="video/webm">
  <source src="/demo.mp4" type="video/mp4">
</video>

### Voraussetzungen

- Eine [Nerd Font](https://www.nerdfonts.com/) installiert und aktiviert in deinem Terminal.

### Schnellinstallation

1. Installiere die Binärversion von **starship**:


   #### Neueste Version installieren

   Mit Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Führe das Skript oben erneut aus, um Starship selbst zu aktualisieren. Die vorhandene Version wird ersetzt, ohne dass deine Starship-Konfiguration verloren geht.


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

   Trage folgendes am Ende der `~/.zshrc` ein:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Füge das Folgende ans Ende von `Microsoft.PowerShell_profile.ps1` an. Du kannst den Speicherort dieser Datei überprüfen, indem du die `$PROFILE` Variable in PowerShell abfragst. Normalerweise ist der Pfad `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` oder `~/.config/powershell/Microsoft.PowerShell_profile.ps1` auf -Nix.

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

   Das wird sich in Zukunft ändern. Only Nushell v0.96+ is supported.

   :::

   Add the following to the end of your Nushell configuration (find it by running `$nu.config-path` in Nushell):

   ```sh
   mkdir ($nu.data-dir | path join "vendor/autoload")
   starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
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
