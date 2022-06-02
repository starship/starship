---
home: true
heroImage: /logo.svg
heroText:
tagline: Prompt yang minimal, super cepat, dan dapat disesuaikan tanpa batas untuk shell apa pun!
actionText: Mari Mulai →
actionLink: ./guide/
features:
  - 
    title: Mengutamakan Kompabilitas
    details: Dapat bekerja di shells yang paling biasa dengan sistem operasi yang paling biasa. Bisa digunakan di mana saja!
  - 
    title: Dibuat Dengan Rust
    details: Menggunakan kecepatan dan keamanan dari Rust, untuk membuat prompt kamu bekerja secepat dan seandal mungkin.
  - 
    title: Dapat Dikustomisasi
    details: Semua detail kecil yang ada dapat kamu atur sesukamu, entah untuk membuatnya sebagai prompt yang seminimal mungkin atau mungkin kaya akan fitur sesuai keinginanmu.
footer: Berlisensi ISC | Hak Cipta © 2019-sekarang Kontributor Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship: Prompt Lintas Shell"
description: Starship merupakan sebuah prompt yang minimal, super cepat, dan sangat bisa untuk dikustomisasi untuk shell apapun! Bisa menampilkan informasi yang kamu butuhkan, namun tetap bisa tampil dengan ramping dan minimal. Pemasangan singkat tersedia untuk Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, dan PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Prasyarat

- [Nerd Font](https://www.nerdfonts.com/) yang telah terinstalasi dan telah diaktifkan di dalam terminal.

### Instalasi Sederhana

1. Instalasi dengan menggunakan binary **starship**:


   #### Pasang Versi Terbaru

   Dengan Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Untuk memperbarui Starship, jalankan kembali skrip di atas. Hal ini dapat memperbarui versi yang terpasang tanpa mengubah konfigurasi Starship.


   #### Pasang Melalui Package Manager

   Dengan [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Dengan [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Tambahkan skrip init ke dalam file konfigurasi Shell:


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

   Tambahkan skrip berikut pada baris akhir `Microsoft.PowerShell_profile.ps1`. Kamu dapat mengecek lokasi file tersebut dengan mencari tahu keberadaan variabel `$PROFILE` di dalam PowerShell. Biasanya, lokasi file tersebut berada di `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` atau `~/.config/powershell/Microsoft.PowerShell_profile.ps1` pada -Nix.

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

   ::: warning

   Only elvish v0.18 or higher is supported.

   :::

   Tambahkan skrip berikut pada baris akhir `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Tambahkan skrip berikut pada baris akhir `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   This will change in the future. Hanya Nushell v0.61+ yang menerima dukungan.

   :::

   Add the following to to the end of your Nushell env file (find it by running `$nu.env-path` in Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   ```

   Lalu tambahkan baris berikut di baris terakhir konfigurasi Nushell (temukan dengan menjalankan `$nu.config-path`):

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   Tambahkan skrip berikut pada baris akhir `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Kamu perlu menggunakan [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) pada Cmd. Tambahkan baris berikut ke dalam `starship.lua` lalu taruhlah file berikut ke dalam direktori Clink:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
