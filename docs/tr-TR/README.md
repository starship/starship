---
home: true
heroImage: /logo.svg
heroText:
tagline: The minimal, blazing-fast, and infinitely customizable prompt for any shell!
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

### Ön koşullar

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal.

### Quick Install

1. Install the **starship** binary:


   #### Install Latest Version

   With Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   To update the Starship itself, rerun the above script. It will replace the current version without touching Starship's configuration.


   #### Install via Package Manager

   With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   With [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Add the init script to your shell's config file:


   #### Bash

   Add the following to the end of `~/.bashrc`:

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

   ::: uyarı

   Only elvish v0.18 or higher is supported.

   :::

   `~/.elvish/rc.elv` dosyasının sonuna ekleyin:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   `~/.tcshrc` dosyasının sonuna ekleyın:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: uyarı

   This will change in the future. Only Nushell v0.60+ is supported.

   :::

   Aşağıdaki komutu çalıştırın:
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   ```

   Aşağıdaki kodu Nushell ayarlarınızın (`$nu.config-path` komutu ile ulaşabilirsiniz) sonuna ekleyin:

   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   `~/.xonshrc` dosyasının sonuna ekleyin:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Cmd ıle beraber [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) kullanmalısınız. Add the following to a file `starship.lua` and place this file in Clink scripts directory:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
