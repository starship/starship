---
layout: home
hero:
  image: /logo.svg
  text:
  tagline: Minimalny, szybki i nieskończenie konfigurowalny wiersz poleceń dla dowolnej powłoki!
  actions:
    - 
      theme: brand
      text: Pierwsze kroki →
      link: ./guide/
features:
  - 
    title: Kompatybilność przede wszystkim
    details: Działa na większości powszechnych powłokach, na większości najpopularniejszych systemach. Używaj go wszędzie!
  - 
    title: Napędzany językiem Rust
    details: Zapewnia najlepszą w swojej klasie prędkość i bezpieczeństwo języka Rust, w celu zapewnienia jak najszybszej i niezawodnej odpowiedzi.
  - 
    title: Konfigurowalny
    details: Każdy mały detal jest konfigurowalny do Twoich preferencji, aby wiersz poleceń był tak minimalny lub tak bogaty w funkcje, jak tylko zechcesz.
footer: Licencja ISC | Copyright © 2019-obecnie Kontrybutorzy Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship: Wiersz poleceń dla dowolnej powłoki"
description: Starship to minimalny, szybki i ekstremalnie konfigurowalny wiersz poleceń dla każdej powłoki! Pokazuje informacje których potrzebujesz, pozostając elegancki i minimalny. Szybka instalacja dostępna dla powłok Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd i PowerShell.
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

### Wymagania wstępne

- Czcionka typu [Nerd Font](https://www.nerdfonts.com/) zainstalowana i włączona w twoim terminalu.

### Instalacja

1. Zainstaluj plik programu **starship**:


   #### Instalacja najnowszej wersji

   Za pomocą powłoki:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Aby zaktualizować Starship, uruchom ponownie powyższy skrypt. Obecna wersja zostanie zastąpiona nową, bez modyfikowania konfiguracji Starship.


   #### Instalacja za pomocą menedżera pakietów

   Za pomocą [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Za pomocą [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Dodaj skrypt inicjalizacyjny do konfiguracji twojej powłoki:


   #### Bash

   Dodaj na koniec pliku `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Dodaj na koniec pliku `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Dodaj na koniec pliku `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Dodaj na koniec pliku `Microsoft.PowerShell_profile.ps1`. Możesz sprawdzić lokalizację tego pliku odczytując zmienną środowiskową `$PROFILE` w PowerShell. Zazwyczaj jest to `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` lub `~/.config/powershell/Microsoft.PowerShell_profile.ps1` na -Nixie.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Dodaj na koniec pliku `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Działa tylko dla elvish v0.18 albo wyższej wersji.

   :::

   Dodaj na koniec pliku `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Dodaj na koniec pliku `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   Ten sposób zmieni się w przyszłości. Only Nushell v0.96+ is supported.

   :::

   Add the following to the end of your Nushell configuration (find it by running `$nu.config-path` in Nushell):

   ```sh
   mkdir ($nu.data-dir | path join "vendor/autoload")
   starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
   ```


   #### Xonsh

   Dodaj na koniec pliku `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Musisz użyć [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) z Cmd. Dodaj następujący kod do pliku `starship.lua` i przenieś ten plik do folderu ze skryptami Clink:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
