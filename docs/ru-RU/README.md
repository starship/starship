---
home: true
heroImage: /logo.svg
heroText:
tagline: The cross-shell prompt for astronauts
actionText: Начало работы →
actionLink: ./ru-RU/guide/
features:
  - 
    title: Совместивость в первую очередь
    details: Работает на большинстве распространенных оболочек и наиболее распространенных операционных системах. Используйте везде!
  - 
    title: Основана на Rust
    details: Приносит наилучшую в своем классе скорость и безопасность Rust, чтобы сделать вашу подсказку как можно быстрее и надежнее.
  - 
    title: Настраиваемая
    details: Каждая маленькая деталь настраивается по вашему вкусу, чтобы сделать эту подсказку минимальной или функциональной, как вы захотите.
footer: Под лицензией ISC | Авторское право © 2019-настоящее Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and Powershell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Быстрая установка

1. Установите двоичный файл **starship**:


   #### Установить последнюю версию

   Через Bash:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Установить через менеджер пакетов

   С [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   С [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
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

   Добавьте следующее в конец `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (или `~/.config/powershell/Microsoft.PowerShell_profile.ps1` на -Nix):

   ```sh
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Добавьте следующее в конец `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
