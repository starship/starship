---
home: true
heroImage: /logo.svg
heroText:
tagline: Il prompt minimalista, super veloce e infinitamente personalizzabile per qualsiasi shell!
actions:
  text: Inizia →
  link: ./guide/
  type: primary
features:
  -
    title: Prima la compatibilità
    details: Funziona sulle shell e sui sistemi operativi più comuni. Usalo ovunque!
  -
    title: Scritto in Rust
    details: Sfrutta la velocità e sicurezza di Rust, per rendere il tuo prompt il più veloce e il più affidabile.
  -
    title: Personalizzabile
    details: Ogni più piccolo dettaglio è personalizzabile a piacere, per rendere questo prompt minimalista o ricco di tutte le funzionalità che desideri.
footer: Licenza ISC | Copyright © 2019-present Starship Collaboratori
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship è il prompt minimalista, super veloce ed estremamente personalizzabile per qualsiasi shell! Mostra le informazioni di cui hai bisogno, rimanendo elegante e minimale. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Prerequisiti

- Un [ Nerd Font ](https://www.nerdfonts.com/) installato e abilitato nel tuo terminale.

### Installazione Veloce

1. Installa il binario **starship**:


   #### Installa l'ultima Versione

   Con Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Per aggiornare Starship stesso, riavviare lo script sopra. Sostituirà la versione corrente senza toccare la configurazione di Starship.


   #### Installa via Package Manager

   Con [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Aggiungi lo script di inizializzazione al file di configurazione della shell:


   #### Bash

   Aggiungi quanto segue alla fine di `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Aggiungi quanto segue alla fine di `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Aggiungi quanto segue alla fine di `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Aggiungi quanto segue alla fine di `Microsoft.PowerShell_profile.ps1`. Puoi controllare la posizione di questo file interrogando la variabile `$PROFILE` in PowerShell. Tipicamente il percorso è `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` oppure `~/.config/powershell/Microsoft.PowerShell_profile.ps1` su -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Aggiungi quanto segue alla fine di `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Only elvish v0.18 or higher is supported.

   :::

   Aggiungi quanto segue alla fine di `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Aggiungi quanto segue alla fine di `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   This will change in the future. Only Nushell v0.73+ is supported.

   :::

   Add the following to the end of your Nushell env file (find it by running `$nu.env-path` in Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   And add the following to the end of your Nushell configuration (find it by running `$nu.config-path`):

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   Aggiungi quanto segue alla fine di `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   You need to use [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) with Cmd. Add the following to a file `starship.lua` and place this file in Clink scripts directory:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
