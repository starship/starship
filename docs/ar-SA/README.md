---
home: true
heroImage: /logo.svg
heroText:
tagline: التخصيص البسيط و السريع و الغير محدود لي ال"shell"!
actionText: البدء مع Starship ←
actionLink: ./guide/
features:
  - 
    title: التوافق أولاً
    details: يعمل على أكثر موجهات الأوامر شيوعاً في أكثر نظم التشغيل شيوعاً. استخدمه في كل مكان!
  - 
    title: Rust-Powered
    details: Brings the best-in-class speed and safety of Rust, to make your prompt as quick and reliable as possible.
  - 
    title: قابل للتخصيص
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

### المتطلبات الأساسية

- تثبيت [Nerd Font](https://www.nerdfonts.com/) وتمكينه في موجه الأوامر الخاصة بك.

### تثبيت سريع

1. تثبيت **starship**:


   #### تثبيت أحدث إصدار

   بإستخدام Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   لتحديث Starship نفسه، أعد تشغيل البرنامج النصي أعلاه. سيتم استبدال الإصدار الحالي بدون لمس تكوين Starship.


   #### التثبيت عبر مدير الحزم

   بإستخدام [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. أضف ما يلي إلى ملف تكوين موجه الأوامر الخاص بك:


   #### Bash

   أضف ما يلي إلى نهاية `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   أضف ما يلي إلى نهاية `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   أضف ما يلي إلى نهاية `~/.zshrc`:

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

   أضف ما يلي إلى نهاية `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: تحذير

   Only elvish v0.18 or higher is supported.

   :::

   أضف ما يلي إلى نهاية `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   أضف ما يلي إلى نهاية `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: تحذير

   This will change in the future. Only Nushell v0.61+ is supported.

   :::

   Add the following to to the end of your Nushell env file (find it by running `$nu.env-path` in Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   ```

   And add the following to the end of your Nushell configuration (find it by running `$nu.config-path`):

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   أضف ما يلي إلى نهاية `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   عليك بإستخدام [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) مع Cmd. Add the following to a file `starship.lua` and place this file in Clink scripts directory:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
