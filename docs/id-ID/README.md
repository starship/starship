---
layout: home
hero:
  image: /logo.svg
  text: null
  tagline: Prompt yang minimal, super cepat, dan dapat disesuaikan tanpa batas untuk shell apa pun!
  actions:
    - theme: brand
      text: Mari Mulai →
      link: ./guide/
features:
  - title: Kompatibilitas Yang Utama
    details: Dapat berfungsi di shells standar dengan sistem operasi yang paling biasa. Pakai di mana saja!
  - title: Dibuat dengan Rust
    details: Menggunakan kecepatan dan keamanan dari Rust, untuk membuat prompt kamu bekerja secepat dan seandal mungkin.
  - title: Dapat Dikustomisasi
    details: Kamu dapat mengatur semua detail kecil dengan sesukamu, entah itu untuk membuatnya sebagai prompt yang seminimal mungkin atau kaya akan fitur yang kamu mau.
footer: Berlisensi ISC | Hak Cipta © 2019-sekarang Kontributor Starship

# Used for the description meta tag, for SEO
metaTitle: "Starship: Prompt Lintas Shell"
description: Starship merupakan sebuah prompt yang minimal, super cepat, dan sangat bisa untuk dikustomisasi untuk shell apapun! Bisa menampilkan informasi yang kamu butuhkan, namun tetap bisa tampil dengan ramping dan minimal. Instalasi sederhana tersedia untuk Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, dan PowerShell.
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

### Prasyarat

- [Nerd Font](https://www.nerdfonts.com/) yang sudah terpasang dan berjalan di dalam terminalmu.

### Instalasi Sederhana

1. Instalasi dengan menggunakan binary **starship**:

   #### Pasang Versi Terbaru

   Dengan Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Untuk memperbarui Starship, jalankan ulang skrip di atas. Hal ini akan memperbarui versi yang terpasang tanpa mengubah konfigurasi Starship.

   #### Pasang Melalui Package Manager

   Dengan [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

2. Tambahkan skrip init ke dalam file konfigurasi Shell:

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

   #### PowerShell

   Add the following to the end of `Microsoft.PowerShell_profile.ps1`. You can check the location of this file by querying the `$PROFILE` variable in PowerShell. Typically the path is `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix.

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

   > [!WARNING] Only elvish v0.18 or higher is supported.

   Add the following to the end of `~/.config/elvish/rc.elv` (`%AppData%\elvish\rc.elv` on Windows):

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```

   For elvish versions prior to v0.21.0 the config file might instead be `~/.elvish/rc.elv`

   #### Tcsh

   Tambahkan skrip berikut pada baris akhir `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```

   #### Nushell

   > [!WARNING]> This will change in the future.
   > > Hanya Nushell v0.96+ yang menerima dukungan.

   Add the following to the end of your Nushell configuration (find it by running `$nu.config-path` in Nushell):

   ```sh
   mkdir ($nu.data-dir | path join "vendor/autoload")
   starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
   ```

   #### Xonsh

   Tambahkan skrip berikut pada baris akhir `~/.xonshrc`:

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
