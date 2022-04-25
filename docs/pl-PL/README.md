---
home: true
heroImage: /logo.svg
heroText:
tagline: Minimalny, szybki i nieskończenie konfigurowalny wiersz poleceń dla dowolnej powłoki!
actionText: Get Started →
actionLink: ./guide/
features:
  - 
    title: Kompatybilność przede wszystkim
    details: Works on the most common shells on the most common operating systems. Use it everywhere!
  - 
    title: Napędzany językiem Rust
    details: Brings the best-in-class speed and safety of Rust, to make your prompt as quick and reliable as possible.
  - 
    title: Konfigurowalny
    details: Every little detail is customizable to your liking, to make this prompt as minimal or feature-rich as you'd like it to be.
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Wymagania wstępne

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal.

### Quick Install

1. Zainstaluj plik programu **starship**:


   #### Instalacja najnowszej wersji

   With Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Aby zaktualizować Starship, uruchom ponownie powyższy skrypt. Obecna wersja zostanie zastąpiona nową, bez modyfikowania konfiguracji Starship.


   #### Instalacja za pomocą menedżera pakietów

   With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Za pomocą [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Dodaj skrypt inicjalizacyjny do konfiguracji twojej powłoki:


   #### Bash

   Dodaj na koniec pliku `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Dodaj na koniec pliku `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Dodaj na koniec pliku `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Dodaj na koniec pliku `Microsoft.PowerShell_profile.ps1`. Możesz sprawdzić lokalizację tego pliku odczytując zmienną środowiskową `$PROFILE` w PowerShell. Zazwyczaj jest to `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` lub `~/.config/powershell/Microsoft.PowerShell_profile.ps1` na -Nixie.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Dodaj na koniec pliku `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Only elvish v0.18 or higher is supported.

   :::

   Dodaj na koniec pliku `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Dodaj na koniec pliku `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   This will change in the future. Only Nushell v0.60+ is supported.

   :::

   Run the following:
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   ```

   And add the following to the end of your Nushell configuration (find it by running `$nu.config-path`):

   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   Dodaj na koniec pliku `~/.xonshrc`:

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
