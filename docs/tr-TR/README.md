---
home: true
heroImage: /logo.svg
heroText:
tagline: Sade, hızlı, dilediğiniz gibi özelleştirilebilen ve istenilen shell'de kullanılabilen prompt!
actions:
  - text: Kullanmaya Başlayın →
    link: ./guide/
    type: primary
features:
  -
    title: Önce Uyumluluk
    details: En yaygın işletim sistemlerindeki en yaygın shell'ler üzerinde çalışır. Use it everywhere!
  -
    title: Rust-Powered
    details: Prompt'u mümkün olduğunca hızlı ve güvenilir hale getirmek için sınıfının en iyisi Rust hızını ve güvenliğini sağlar.
  -
    title: Özelleştirilebilir
    details: Her küçük ayrıntı beğeninize göre özelleştirilebilir, böylece bu prompt'u istediğiniz kadar minimal veya zengin özelliklere sahip hale getirirsiniz.
footer: ISC Lisanslı | Telif Hakkı © 2019-günümüz Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship, her prompt için minimal, son derece hızlı ve son derece özelleştirilebilir bir shelldir! Şık ve minimal kalırken ihtiyacınız olan bilgileri gösterir. Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd ve PowerShell için hızlı kurulum mevcuttur.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Ön koşullar

- Terminalinizde bir [Nerd Font](https://www.nerdfonts.com/) yüklü ve etkin olmalı.

### Hızlı Kurulum

1. Install the **starship** binary:


   #### En Son Sürümü Yükle

   Shell ile:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Starship'in kendisini güncellemek için yukarıdaki komudu yeniden çalıştırın. Starship'in yapılandırmasına dokunmadan mevcut sürümün yerini alacak.


   #### Paket Yöneticisi ile yükleme

   Homebrew [ile](https://brew.sh/):

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Add the init script to your shell's config file:


   #### Bash

   `~/.bashrc` dosyasının sonuna ekleyin:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   `~/.config/fish/config.fish` dosyasının sonuna ekleyin:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   `~/.zshrc` dosyasının sonuna ekleyin:

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

   `~/.config/ion/initrc` dosyasının sonuna ekleyin:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

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

   ::: warning

   This will change in the future. Only Nushell v0.73+ is supported.

   :::

   Nushell env dosyanızın sonuna aşağıdakileri ekleyin (Nushell'de `$nu.env-path` komutunu çalıştırarak bulabilirsiniz):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   Aşağıdaki kodu Nushell ayarlarınızın (`$nu.config-path` komutu ile ulaşabilirsiniz) sonuna ekleyin:

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   `~/.xonshrc` dosyasının sonuna ekleyin:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Cmd ile beraber [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) kullanmalısınız. Add the following to a file `starship.lua` and place this file in Clink scripts directory:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
