---
layout: home
hero:
  image: /logo.svg
  text:
  tagline: Простий, супер швидкий та безмежно адаптивний командний рядок для будь-якої оболонки!
  actions:
    - 
      theme: brand
      text: Початок роботи →
      link: ./guide/
features:
  - 
    title: Сумісність на першому місці
    details: Працює з більшістю оболонок у всіх популярних операційних системах. Можна використовувати будь-де!
  - 
    title: Rust під капотом
    details: Використовує найкращу в класі швидкість та безпеку застосунків створених за допомогою Rust, що робить ваш командний рядок швидким та надійним.
  - 
    title: Персоналізація
    details: Кожна дрібничка налаштовується відповідно до ваших потреб, щоб зробити командний рядок аскетичним чи багатофункціональним, таким, яким ви б хотіли б його бачити.
footer: Ліцензія ISC | Авторське право © 2019-по сьогодні Учасники Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Простий, супер швидкий та безмежно адаптивний командний рядок для будь-якої оболонки! Показує потрібну інформацію залишаючись блискучим та мінімальним. Швидке встановлення доступне для Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd і PowerShell.
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

### Вимоги

- A Встановлений та увімкнений шрифт [Nerd Font](https://www.nerdfonts.com/) у вашому терміналі.

### Швидке встановлення

1. Встановіть **starship**:


   #### Встановлення останньої версії

   З оболонки:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Для оновлення Starship перезапустіть команду вказану вище. Це допоможе замінити поточну версію не чіпаючи налаштувань Starship.


   #### Встановлення за допомогою пакетних менеджерів

   [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Додайте init скрипт до конфігураційного файлу оболонки:


   #### Bash

   Додайте наступний рядок наприкінці `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Додайте наступний рядок наприкінці `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Додайте наступний рядок наприкінці `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Додайте наступний рядок в кінець `Microsoft.PowerShell_profile.ps1`. Ви можете перевірити розташування цього файлу, звернувшись до змінної `$PROFILE` у PowerShell. Зазвичай це `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` або `~/.config/powershell/Microsoft.PowerShell_profile.ps1` у -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Додайте наступний рядок наприкінці `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Підтримуються лише elv0.18 або вище.

   :::

   Додайте наступний рядок наприкінці `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Додайте наступний рядок наприкінці `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   Це зміниться в майбутньому. Only Nushell v0.96+ is supported.

   :::

   Add the following to the end of your Nushell configuration (find it by running `$nu.config-path` in Nushell):

   ```sh
   mkdir ($nu.data-dir | path join "vendor/autoload")
   starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
   ```


   #### Xonsh

   Додайте наступний рядок наприкінці `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Вам потрібно використовувати [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) разом з Cmd. Додайте наступне у файл `starship.lua` і розмістіть цей файл у теці скриптів Cline:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
