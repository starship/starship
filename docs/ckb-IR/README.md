---
home: true
heroImage: /logo.svg
heroText:
tagline: promptـێکی سوکەڵە، خێرا، و بێسنور دڵخوازکراو بۆ هەر شێڵێک!
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
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### پێشمەرجەکان

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal.

### Quick Install

1. باینەری **starship** دابمەزرێنە:


   #### کۆتا وەشان دابمەزرێنە

   With Shell:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   بۆ نوێکردنەوەی Starship خۆی، نووسینەکەی سەرەوە ڕەن بکەوە. وەشانی ئێستا دەگۆڕێت بێ ئەوەی دەسکاری ڕێکخستنەکانی Starship بکات.


   #### دابمەزرێنە لەڕێگەی بەڕێوبەری گورزەوە

   With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   لەگەڵ [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. نوسینی init زیادبکە بۆ فایلی ڕێکخستنی شێڵەکەت:


   #### Bash

   ئەمەی خوارەوە زیادبکە لە کۆتایی `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   ئەمەی خوارەوە زیادبکە لە کۆتایی `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   ئەمەی خوارەوە زیادبکە لە کۆتایی `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Add the following to the end of `Microsoft.PowerShell_profile.ps1`. You can check the location of this file by querying the `$PROFILE` variable in PowerShell. ئاسایی ڕێڕەوەکە بریتیە لە `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` یان `/.config/powershell/Microsoft.PowerShell_profile.ps1` لەسەر -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   ئەمەی خوارەوە زیادبکە لە کۆتایی `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

   #### Elvish

   ::: warning Only elvish v0.15 or higher is supported. :::

   ئەمەی خوارەوە زیادبکە لە کۆتایی `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   ئەمەی خوارەوە زیادبکە لە کۆتایی `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```
