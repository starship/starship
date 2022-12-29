---
home: true
heroImage: /logo.svg
heroText:
tagline: De minimalistische, razend snelle en oneindig aanpasbare prompt voor elke shell!
actionText: Get Started →
actionLink: ./guide/
features:
  - 
    title: Compatibility First
    details: Works on the most common shells on the most common operating systems. Use it everywhere!
  - 
    title: Rust-Powered
    details: Brings the best-in-class speed and safety of Rust, to make your prompt as quick and reliable as possible.
  - 
    title: Customizable
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

### Benodigdheden

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal.

### Quick Install

1. Installeer de **Starship** binary:


   #### Installeer de nieuwste versie

   With Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Om Starship zelf bij te werken, voer je het bovenstaande script opnieuw uit. Deze vervangt de huidige versie zonder de configuratie van Starship aan te passen.


   #### Installeren via pakketbeheerder

   With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Voeg het initscript toe aan het configuratiebestand van uw shell:


   #### Bash

   Voeg het volgende toe aan het einde van `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Add the following to the end of `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Add the following to the end of `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Add the following to the end of `Microsoft.PowerShell_profile.ps1`. You can check the location of this file by querying the `$PROFILE` variable in PowerShell. Typically the path is `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Add the following to the end of `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Only elvish v0.18 or higher is supported.

   :::

   Add the following to the end of `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Add the following to the end of `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   This will change in the future. Only Nushell v0.61+ is supported.

   :::

   Add the following to the end of your Nushell env file (find it by running `$nu.env-path` in Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   ```

   And add the following to the end of your Nushell configuration (find it by running `$nu.config-path`):

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   Add the following to the end of `~/.xonshrc`:

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
