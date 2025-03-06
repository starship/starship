---
layout: дом
hero:
  image: /logo.svg
  text:
  tagline: Минималистичное, быстрое и бесконечно настраиваемое приглашение командной строки для любой оболочки!
  actions:
    - 
      theme: бренд
      text: Начало работы →
      link: ./guide/
features:
  - 
    title: Совместимость в первую очередь
    details: Работает на большинстве распространенных оболочек и наиболее распространенных операционных системах. Используйте везде!
  - 
    title: Основана на Rust
    details: Приносит наилучшую в своем классе скорость и безопасность Rust, чтобы сделать вашу оболочку как можно быстрее и надежнее.
  - 
    title: Настраиваемая
    details: Каждая маленькая деталь настраивается по вашему вкусу, чтобы сделать эту оболочку минималистичной или функциональной, как вы захотите.
footer: Под лицензией ISC | Авторское право © 2019-настоящее Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship - минимальная, быстрая и бесконечная настраиваемая командная строка для любой оболочки! Показывает нужную вам информацию, оставаясь красивой и минималистичной. Быстрая установка доступна для Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, и PowerShell.
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

### Обязательные условия

- Установленный и включенный шрифт [Nerd Font](https://www.nerdfonts.com/) в вашем терминале.

### Быстрая установка

1. Установите бинарный файл **starship**:


   #### Установка последней версии

   Через Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Чтобы обновить Starship, повторно запустите приведенный выше скрипт. Он заменит текущую версию, не затрагивая конфигурацию Starship.


   #### Установка через пакетный менеджер

   С помощью [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   С помощью [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Добавить сценарий инициализации в конфигурационный файл вашей оболочки:


   #### Bash

   Добавьте следующее в конец `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Добавьте следующее в конец `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Добавьте следующее в конец `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   Добавьте следующее в конец `Microsoft.PowerShell_profile.ps1`. Вы можете узнать расположение этого файла, запросив переменную `$PROFILE` в PowerShell. Обычно он находится в `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` или `~/.config/powershell/Microsoft.PowerShell_profile.ps1` на -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Добавьте следующее в конец `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Только elvish v0.18 или выше поддерживается.

   :::

   Добавьте следующую строку в конец `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Добавьте следующее в конец `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   Это изменится в будущем. Поддерживается только Nushell v0.96+.

   :::

   Добавьте следующее в конец вашей конфигурации Nutshell (найдите это, запустив `$nu.config-path` в Nutshell):

   ```sh
   mkdir ($nu.data-dir | path join "vendor/autoload")
   starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
   ```


   #### Xonsh

   Добавьте следующее в конец `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Вам нужно использовать [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) с помощью Cmd. Добавьте следующий файл `starship.lua` и поместите этот файл в каталог Clink скриптов:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
