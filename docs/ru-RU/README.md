---
home: true
heroImage: /logo.svg
heroText:
tagline: Минималистичная, быстрая и бесконечно настраиваемая командная строка для любой оболочки!
actionText: Начало работы →
actionLink: ./guide/
features:
  - 
    title: Совместивость в первую очередь
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
description: Starship - минимальная, быстрая и бесконечная настраиваемая командная строка для любой оболочки! Показывает нужную вам информацию, оставаясь красивой и минималистичной. Быстрая установка доступна для Bash, Fish, ZSH, Ion и PowerShell.
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
   To update the Starship itself, rerun the above script. It will replace the current version without touching Starship's configuration.


   #### Установить через менеджер пакетов

   With [Homebrew](https://brew.sh/):

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

   Добавьте следующее в конец `Microsoft.PowerShell_profile.ps1`. Вы можете проверить местоположение этого файла, запросив переменную `$PROFILE` в PowerShell. Обычно он находится в `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` или `~/.config/powershell/Microsoft.PowerShell_profile.ps1` на -Nix.

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

   ::: warning Only elvish v0.15 or higher is supported. :::

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
