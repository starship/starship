---
home: true
heroImage: /logo.svg
heroText:
tagline: Prompt yang minimal, super cepat, dan dapat disesuaikan tanpa batas untuk shell apa pun!
actionText: Mulai
actionLink: ./guide/
features:
  - 
    title: Kompabilitas Yang Utama
    details: Dapat bekerja di shells yang paling biasa dengan sistem operasi yang paling biasa. Bisa digunakan di mana saja!
  - 
    title: Dibuat Dengan Rust
    details: Menggunakan kecepatan dan keamaan dari Rust, untuk membuat prompt anda bekerja secepat mungkin dan dapat diandalkan.
  - 
    title: Dapat Diatur
    details: Semua detail kecil yang ada dapat kamu atur sesukamu, mungkin untuk membuat prompt yang seminimal mungkin atau kaya fitur seperti yang kamu inginkan.
footer: Berlisensi ISC | Hak Cipta © 2019-sekarang Kontributor Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship: Prompt Lintas Shell"
description: Starship merupakan prompt yang minimal, super cepat, dan sangat bisa diatur untuk semua shell! Menampilkan info yang dibutuhkan, namun tetap bisa tampil ramping dan minimal. Instalasi sederhana tersedia untuk Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, dan PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Prasyarat

- [Nerd Font](https://www.nerdfonts.com/) yang telah terinstalasi dan telah diaktifkan di terminal.

### Instalasi Sederhana

1. Instalasi dengan menggunakan binary **starship**:


   #### Pasang Versi Terbaru

   Dengan Shell:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   Untuk memperbarui Starship, jalankan kembali skrip di atas. Hal ini dapat memperbarui versi yang terpasang tanpa menyentuh konfigurasi Starship.


   #### Pasang Melalui Package Manager

   Dengan [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Dengan [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Tambahkan skrip init ke file konfigurasi Shell:


   #### Bash

   Tambahkan skrip berikut pada baris akhir `~/.bashrc:`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Tambahkan skrip berikut pada baris akhir `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Tambahkan skrip berikut pada baris akhir `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Tambahkan skrip berikut pada baris akhir `Microsoft.PowerShell_profile.ps1`. You can check the location of this file by querying the `$PROFILE` variable in PowerShell. Biasanya, lokasi file tersebut berada di `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` atau `~/.config/powershell/Microsoft.PowerShell_profile.ps1` pada -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Tambahkan skrip berikut pada baris akhir `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning Cuma elvish v0.15 atau yang lebih baru yang tengah didukung. :::

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
