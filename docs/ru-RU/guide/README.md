<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Cross-shell prompt" />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/starship/starship/Main workflow/master?label=workflow&style=flat-square"
      alt="GitHub Actions workflow status" /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Crates.io version" /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Packaging status" /></a
><br />
  <a href="https://discord.gg/8Jzqu3T"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Chat on Discord" /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Follow @StarshipPrompt on Twitter" /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Сайт</a>
  ·
  <a href="#-installation">Установка</a>
  ·
  <a href="https://starship.rs/config/">Конфигурация</a>
</p>

<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png"
      alt="English" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ja-JP/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-jp.png"
      alt="日本語" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="繁體中文" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png"
      alt="Русский" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Deutsch" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-CN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-cn.png"
      alt="简体中文" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/es"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Español" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/fr"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Français" /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship with iTerm2 and the Snazzy theme"
  width="50%"
  align="right" />


**Минимальная, быстрая и бесконечная настраиваемая командная строка для любой оболочки!**


- **Быстрая:** она быстрая – _очень-очень_ быстрая! 🚀
- **Настраиваемая:** настройте каждый элемент вашей командной строки.
- **Универсальная:** работает с любой оболочкой, на любой операционной системе.
- **Умная:** сразу показывает соответствующую информацию.
- **Богатая функциями:** поддержка всех ваших любимых инструментов.
- **Легкая:** быстро установить - начните использовать ее в считанные минуты.

<p align="center">
<a href="https://starship.rs/"><strong>Изучите документацию Starship&nbsp;&nbsp;▶</strong></a>
</p>

## 🚀 Установка

### Требования

- Установленный и включённый [шрифт Powerline](https://github.com/powerline/fonts) (например, [Fira Code](https://github.com/tonsky/FiraCode)).

### Начало работы

1. Установите двоичный файл **starship**:


   #### Установить последнюю версию


   ##### Из прекомпилированного двоичного файла, с Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   ##### Из источника на [crates.io](https://crates.io/):

   ```sh
   cargo install starship
   ```


   #### Установить через менеджер пакетов


   ##### С [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### С [Scoop](https://scoop.sh):

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
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Добавьте следующее в конец `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

## 🤝 Помощь

Мы всегда ищем помощь людей **всех уровней навыков**! Если вы хотите облегчить свой путь к проекту, посмотрите хорошие первые ошибки ([first good issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue)).

Если вы хотите помочь в создании Starship, пожалуйста, ознакомьтесь с нашим [Руководством по содействию](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Кроме того, заходите на наш [Discord сервер](https://discord.gg/8Jzqu3T) и поздоровайтесь. 👋

## 💭 Вдохновления

Пожалуйста, ознакомьтесь с этими предыдущими работами, которые помогли вдохновить создание Starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - командная строка ZSH для астронавтов.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - меж-оболочная тема robbyrussell, написаная на JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - меж-оболочная настраиваемая командная строка с иконками.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 Лицензия

Авторское право © 2019-настоящее, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> Этот проект лицензирован под лицензией [ISC](https://github.com/starship/starship/blob/master/LICENSE).
