---
home: true
heroImage: /logo.svg
heroText:
tagline: Il prompt minimalista, super veloce e infinitamente personalizzabile per qualsiasi shell!
actionText: Inizia →
actionLink: ./guide/
features:
  - 
    title: Prima la compatibilità
    details: Funziona sulle shell e sui sistemi operativi più comuni. Usalo ovunque!
  - 
    title: Generato in Rust
    details: Sfrutta la velocità e sicurezza migliori di Rust, per rendere il tuo prompt il più veloce e il più affidabile.
  - 
    title: Personalizzabile
    details: Ogni più piccolo dettaglio è personalizzabile a piacere, per rendere questo messaggio prompt minimalista o ricco delle funzionalità che desideri.
footer: Licenza ISC | Copyright © 2019-present Starship Collaboratori
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship è il prompt minimalista, super veloce ed estremamente personalizzabile per qualsiasi shell! Mostra le informazioni di cui hai bisogno, rimanendo elegante e minimale. Installazione rapida disponibile per Bash, Fish, ZSH, Ion e PowerShell.
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
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   Per aggiornare Starship stesso, riavviare lo script sopra. Sostituirà la versione corrente senza toccare la configurazione di Starship.


   #### Installa con Package Manager

   Con [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Con [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
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

   Aggiungi quanto segue alla fine di `Microsoft.PowerShell_profile.ps1`. Puoi controllare la posizione di questo file interrogando la variabile `$PROFILE` in PowerShell. In genere il percorso è `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` oppure `~/.config/powershell/Microsoft.PowerShell_profile.ps1` in -Nix.

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

   ::: warning È supportato solo elvish v0.15 o superiore. :::

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
