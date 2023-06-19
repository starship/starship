---
home: true
heroImage: /logo.svg
heroText:
tagline: Простий, супер швидкий та безмежно адаптивний командний рядок для будь-якої оболонки!
actionText: Початок роботи →
actionLink: ./guide/
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

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

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

   Це зміниться в майбутньому. Підтримується лише Nushell v0.73+.

   :::

   Додайте наступне в кінець вашого файлу env (його можна знайти за допомогою змінної `$nu.env-path` в Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   Додайте наступний рядок наприкінці Вашої конфігурації Nushell (знайдіть її виконавши `$nu.config-path`):

   ```sh
   source ~/.cache/starship/init.nu
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
