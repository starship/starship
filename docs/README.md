---
home: true
heroImage: /logo.svg
actionText: Get Started →
actionLink: /guide/
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
---

<div class="features">
  <div class="feature">
    <h2>Compatibility First</h2>
    <p>Works on the most common shells on the most common operating systems. Use it everywhere!</p>
  </div>
  <div class="feature">
    <h2>Rust-Powered</h2>
    <p>Brings the best-in-class speed and safety of Rust, to make your prompt as quick and reliable as possible.</p>
  </div>
  <div class="feature">
    <h2>Customizable</h2>
    <p>Every little detail is customizable to your liking, to make this prompt as minimal or feature-rich as you'd like it to be.</p>
  </div>
</div>

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Quick Install

1. Install the **starship** binary:

   #### Install Latest Version

   ##### With Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```

   #### Install via Package Manager

   ##### With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   ##### With [Scoop](https://scoop.sh):

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

   Add the following to the end of `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix):

   ```sh
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```

   #### Ion

   Add the following to the end of `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
