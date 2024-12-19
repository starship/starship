---
layout: home
hero:
  image: /logo.svg
  text:
  tagline: আপনার টার্মিনাল এর জন্য একটি সহজ, প্রচণ্ড দ্রুত এবং অশেষভাবে কাস্টমাইজ করার  মতো সুবিধাসম্পন্ন একটি প্রম্প্ট!
  actions:
    - 
      theme: brand
      text: Get Started →
      link: ./guide/
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

### পূর্বশর্ত

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

   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Add the init script to your shell's config file:


   #### Bash

   `~/.bashrc` এর শেষে নিম্নলিখিত লাইন টি যোগ করুন:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   `~/.config/fish/config.fish` এর শেষে নিম্নলিখিত লাইন টি যোগ করুন:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   `~/.zshrc` এর শেষে নিম্নলিখিত লাইন টি যোগ করুন:

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

   `~/.config/ion/initrc` এর শেষে নিম্নলিখিত লাইন টি যোগ করুন:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Only elvish v0.18 or higher is supported.

   :::

   `~/.elvish/rc.elv` এর শেষে নিম্নলিখিত লাইন টি যোগ করুন:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   `~/.tcshrc` এর শেষে নিম্নলিখিত লাইন টি যোগ করুন:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   This will change in the future. Only Nushell v0.78+ is supported.

   :::

   আপনার Nushell env ফাইলের (Nushell এ `$nu.env-path` কমান্ডটি রান করে ফাইলটি খুঁজে বের করুন) শেষে নিম্নলিখিত লাইনগুলি যোগ করুন:

   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   এরপর আপনার Nushell কনফিগের (Nushell এ `$nu.config-path` কমান্ডটি রান করে ফাইলটি খুঁজে বের করুন) শেষে নিম্নলিখিত লাইনটি যোগ করুন:

   ```sh
   use ~/.cache/starship/init.nu
   ```


   #### Xonsh

   `~/.xonshrc` এর শেষে নিম্নলিখিত লাইন টি যোগ করুন:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   আপনাকে Cmd এর সাথে [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) ব্যবহার করতে হবে । Add the following to a file `starship.lua` and place this file in Clink scripts directory:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
