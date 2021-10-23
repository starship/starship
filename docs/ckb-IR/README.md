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
description: 'Starship: یەکێ لە promptـە سوکەڵە، خێرا، و بێسنور دڵخوازکراوەکان بۆ هەر شێڵێک! ئەو زانیارییانە پشان دەدات کە پێویستە، لەوکاتەیا بە ئارامی و سوکەڵەیی ئەمێنێتەوە. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, and PowerShell.'
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### پێشمەرجەکان

- [فۆنتێکی Nerd](https://www.nerdfonts.com/) دامەزراوە و چالاککراوە لە تێرمیناڵەکەتا.

### دامەزراندنی خێرا

1. باینەری **starship** دابمەزرێنە:


   #### کۆتا وەشان دابمەزرێنە

   لەگەڵ شێڵ:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   بۆ نوێکردنەوەی Starship خۆی، نووسینەکەی سەرەوە ڕەن بکەوە. وەشانی ئێستا دەگۆڕێت بێ ئەوەی دەسکاری ڕێکخستنەکانی Starship بکات.


   #### دابمەزرێنە لەڕێگەی بەڕێوبەری گورزەوە

   لەگەڵ [Homebrew](https://brew.sh/):

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

   ئەمەی خوارەوە زیادبکە لە کۆتایی `Microsoft.PowerShell_profile.ps1`. ئەتوانی شوێنی ئەم فایلە ببینیتەوە بە سەیرکردنی گۆڕاوی `$PROFILE` لە PowerShell. ئاسایی ڕێڕەوەکە بریتیە لە `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` یان `/.config/powershell/Microsoft.PowerShell_profile.ps1` لەسەر -Nix.

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

   ::: ئاگادارکردنەوە تەنها elvish v0.15 یان بەرزتر پشتگیری کراوە. :::

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


   #### Nushell

   ::: warning This will change in the future. Only nu version v0.33 or higher is supported. ::: Add the following to your nu config file. You can check the location of this file by running `config path` in nu.

   ```toml
   startup = [
    "mkdir ~/.cache/starship",
    "starship init nu | save ~/.cache/starship/init.nu",
    "source ~/.cache/starship/init.nu"
   ]
   prompt = "starship_prompt"
   ```


   #### Xonsh

   Add the following to the end of `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```
