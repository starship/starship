<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship - меж-оболочная командная строка"
 />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/starship/starship/Main workflow/master?label=workflow&style=flat-square"
      alt="Статус GitHub Actions"
 /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Версия Crates.io"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Статус упаковки" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Чат в Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Подпишитесь на @StarshipPrompt в Twitter"
 /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Сайт</a>
  ·
  <a href="#🚀-installation">Установка</a>
  ·
  <a href="https://starship.rs/config/">Конфигурация</a>
</p>

<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png"
      alt="English"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Deutsch"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/es-ES/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Español"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/fr-FR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Français"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ja-JP/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-jp.png"
      alt="日本語"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/pt-BR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-br.png"
      alt="Português do Brasil"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png"
      alt="Русский"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/vi-VN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-vn.png"
      alt="Tiếng Việt"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-CN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-cn.png"
      alt="简体中文"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="繁體中文"
 /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship в iTerm2 с темой Snazzy"
  width="50%"
  align="right"
 />

**Минималистичная, быстрая и бесконечно настраиваемая командная строка для любой оболочки!**

- **Быстрая:** она быстрая – _очень-очень_ быстрая! 🚀
- **Настраиваемая:** настройте каждый элемент вашей командной строки.
- **Универсальная:** работает с любой оболочкой, на любой операционной системе.
- **Умная:** сразу показывает соответствующую информацию.
- **Богатая функциями:** поддержка всех ваших любимых инструментов.
- **Легкая:** быстро установить - начните использовать ее в считанные минуты.

<p align="center">
<a href="https://starship.rs/config/"><strong>Изучите документацию Starship&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 Установка

### Требования

- Установленный и включённый [шрифт Powerline](https://www.nerdfonts.com/) (например, [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)).

### Начало работы

**Note**: due to the proliferation of different platforms, only a subset of supported platforms are shown below. Can't see yours? Have a look at the [extra platform instructions](https://starship.rs/installing/).

1. Установите двоичный файл **starship**:


   #### Установить последнюю версию


   ##### Из прекомпилированного двоичного файла, с Shell:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```

   Для обновления Starship перезапустите этот скрипт. Он заменит текущую версию без изменения конфигурации.

   **Примечание** - Значения по умолчанию из установочного скрипта могут быть переопределены. Для уточнения смотрите встроенную справку.

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --help
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

2. Добавить сценарий инициализации в конфигурационный файл вашей оболочки:


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

   ```powershell
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Добавьте следующее в конец `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   **Внимание** Поддерживается только elvish v0.15 или выше. Добавьте следующую строку в конец `~/.elvish/rc.elv`:

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


   #### Xonsh

   Add the following to the end of `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Nushell

   **Warning** This will change in the future. Only nu version v0.33 or higher is supported. Add the following to your nu config file. You can check the location of this file by running `config path` in nu.

   ```toml
   startup = [
    "mkdir ~/.cache/starship",
    "starship init nu | save ~/.cache/starship/init.nu",
    "source ~/.cache/starship/init.nu"
   ]
   prompt = "starship_prompt"
   ```

## 🤝 Помощь

Мы всегда ищем помощь людей **всех уровней навыков**! Если вы хотите облегчить свой путь к проекту, посмотрите хорошие первые ошибки ([first good issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue)).

Если Вы свободно владеете иностранным языком отличным от английского, мы высоко оценим любую помощь в переводе нашей документации на другие языки и поддержании ее в актуальном состоянии. Если вы хотите помочь, переводы могут быть сделаны на платформе [Starship Crowdin](https://translate.starship.rs/).

Если вы хотите помочь в создании Starship, пожалуйста, ознакомьтесь с нашим [Руководством по содействию](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Кроме того, заходите на наш [Discord сервер](https://discord.gg/8Jzqu3T) и поздоровайтесь. 👋

## 💭 Вдохновления

Пожалуйста, ознакомьтесь с этими предыдущими работами, которые помогли вдохновить создание Starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - командная строка ZSH для астронавтов.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - меж-оболочная тема robbyrussell, написаная на JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - меж-оболочная настраиваемая командная строка с иконками.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Иконка ракеты Starship">
</p>

## 📝 Лицензия

Авторское право © 2019-настоящее, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> Этот проект [ISC](https://github.com/starship/starship/blob/master/LICENSE) лицензирован.
