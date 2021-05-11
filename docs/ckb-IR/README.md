---
home: true
heroImage: /logo.svg
heroText:
tagline: promptـێکی سوکەڵە، خێرا، و بێسنور دڵخوازکراو بۆ هەر شێڵێک!
actionText: دەستپێبکە ←
actionLink: ./guide/
features:
  - 
    title: سەرەتا گونجان
    details: کاردەکات لەسەر زۆربەی شێڵە باوەکان لەسەر زۆربەی سیستەمە باوەکان. لە هەموو شوێنێک بەکاری بهێنە!
  - 
    title: Rust لە پشتە
    details: باشترینی هاوتاکانی لە خێرایی و سەلامەتی Rust بەکارئەهێنێ بۆ ئەوەی promptـەکەت خێراترین و پشت پێبەستراوین بێ.
  - 
    title: دڵخوازکراو
    details: هەموو وردەکارییەکی دڵخواز ئەکرێ بەوجۆرەی حەزت لێیە، بۆ ئەوەی promptـەکە سوکەڵە بێ و پڕ تایبەتمەندی بێت بەوجۆرەی حەزت لێیە ببێ.
footer: لەژێر مۆڵەتی ISCـە | مافی پارێزراوە © 2019-ئێستا بەژداریکەرانی Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship: یەکێ لە Promptـە شێڵ نەناسەکان"
description: 'Starship: یەکێ لە promptـە سوکەڵە، خێرا، و بێسنور دڵخوازکراوەکان بۆ هەر شێڵێک! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and PowerShell.'
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
