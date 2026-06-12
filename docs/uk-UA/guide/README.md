<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/main/media/logo.png"
    alt="Starship – Cross-shell prompt"
  />
</p>

<p align="center"><a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/actions/workflow/status/starship/starship/workflow.yml?branch=master&label=workflow&style=flat-square"
      alt="Статус дій GitHub"
 /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Версія Crates.io"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Стан пакування" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Чат в Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Слідкуйте за @StarshipPrompt на Twitter"
 /></a>
  <a href="https://stand-with-ukraine.pp.ua"
    ><img
      src="https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraineFlat.svg"
      alt="Підтримуйте Україну"
 /></a>
</p>

<p align="center"><a href="https://starship.rs">Вебсайт</a>
  ·
  <a href="#🚀-installation">Встановлення</a>
  ·
  <a href="https://starship.rs/config/">Налаштування</a>
</p>

<p align="center"><a href="https://github.com/starship/starship/blob/master/README.md"
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
    href="https://github.com/starship/starship/blob/master/docs/id-ID/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-id.png"
      alt="Bahasa Indonesia"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/it-IT/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-it.png"
      alt="Italiano"
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
    href="https://github.com/starship/starship/blob/master/docs/uk-UA/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ua.png"
      alt="Українська"
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

> [!WARNING]
> **The default branch has been renamed from `master` to `main`.**
> If you have a local clone, update it by running:
>
> ```sh
> git branch -m master main
> git fetch origin
> git branch -u origin/main main
> git remote set-head origin -a
> ```

<img
src="https://raw.githubusercontent.com/starship/starship/main/media/demo.gif"
alt="Starship with iTerm2 and the Snazzy theme"
width="50%"
align="right"
/>

**The minimal, blazing-fast, and infinitely customizable prompt for any shell!**

- **Fast:** it's fast – _really really_ fast! 🚀
- **Customizable:** configure every aspect of your prompt.
- **Universal:** works on any shell, on any operating system.
- **Intelligent:** shows relevant information at a glance.
- **Feature rich:** support for all your favorite tools.
- **Easy:** quick to install – start using it in minutes.

<p align="center"><a href="https://starship.rs/config/"><strong>Explore the Starship docs&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## Передумови

### Передумови

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal (for example, try the [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)).

### Крок 1. Встановіть Starship

Оберіть вашу операційну систему зі списку нижче для ознайомлення з інструкцією зі встановлення:

<details><summary>Android</summary>

Встановіть Starship використовуючи будь-який з наступних менеджерів пакунків:

| Репозиторій | Команда для встановлення |
| ----------- | ------------------------ |
| [Termux]    | `pkg install starship`   |

</details>

<details><summary>BSD</summary>

Встановіть Starship використовуючи будь-який з наступних менеджерів пакунків:

| Дистрибутив | Репозиторій     | Команда для встановлення          |
| ----------- | --------------- | --------------------------------- |
| **_Any_**   | **[crates.io]** | `cargo install starship --locked` |
| FreeBSD     | [FreshPorts]    | `pkg install starship`            |
| NetBSD      | [pkgsrc]        | `pkgin install starship`          |

</details>

<details><summary>Linux</summary>

Встановіть останню версію для вашої системи:

```sh
curl -sS https://starship.rs/install.sh | sh
```

Як варіант, можете встановити Starship скориставшись будь-яким з наступних менеджерів пакунків:

| Дистрибутив                        | Репозиторій             | Команда для встановлення                                      |
| ---------------------------------- | ----------------------- | ------------------------------------------------------------- |
| **_Any_**                          | **[crates.io]**         | `cargo install starship --locked`                             |
| _Any_                              | [conda-forge]           | `conda install -c conda-forge starship`                       |
| _Any_                              | [Linuxbrew]             | `brew install starship`                                       |
| Alpine Linux 3.13+ | [Alpine Linux Packages] | `apk add starship`                                            |
| Arch Linux                         | [Arch Linux Extra]      | `pacman -S starship`                                          |
| CentOS 7+                          | [Copr]                  | `dnf copr enable atim/starship` <br /> `dnf install starship` |
| Debian 13+                         | [Debian Main]           | `apt install starship`                                        |
| Fedora 40+                         | [Copr]                  | `dnf copr enable atim/starship` <br /> `dnf install starship` |
| Gentoo                             | [Gentoo Packages]       | `emerge app-shells/starship`                                  |
| Manjaro                            |                         | `pacman -S starship`                                          |
| NixOS                              | [nixpkgs]               | `nix-env -iA nixpkgs.starship`                                |
| openSUSE                           | [OSS]                   | `zypper in starship`                                          |
| Ubuntu 25.04+      | [Ubuntu Universe]       | `apt install starship`                                        |
| Void Linux                         | [Void Linux Packages]   | `xbps-install -S starship`                                    |

</details>

<details><summary>macOS</summary>

Встановіть останню версію для вашої системи:

```sh
curl -sS https://starship.rs/install.sh | sh
```

Як варіант, можете встановити Starship скориставшись будь-яким з наступних менеджерів пакунків:

| Репозиторій     | Команда для встановлення                |
| --------------- | --------------------------------------- |
| **[crates.io]** | `cargo install starship --locked`       |
| [conda-forge]   | `conda install -c conda-forge starship` |
| [Homebrew]      | `brew install starship`                 |
| [MacPorts]      | `port install starship`                 |

</details>

<details><summary>Windows</summary>

Встановіть Starship використовуючи будь-який з наступних менеджерів пакунків:

Встановіть Starship використовуючи будь-який з наступних менеджерів пакунків:

| Репозиторій     | Команда для встановлення                |
| --------------- | --------------------------------------- |
| **[crates.io]** | `cargo install starship --locked`       |
| [Chocolatey]    | `choco install starship`                |
| [conda-forge]   | `conda install -c conda-forge starship` |
| [Scoop]         | `scoop install starship`                |
| [winget]        | `winget install --id Starship.Starship` |

</details>

### Крок 2. Налаштуйте оболонку для використання Starship

Налаштуйте свою оболонку, щоб ініціалізувати starship. Виберіть ваш варіант зі списку:

<details><summary>Bash</summary>

Додайте наступний рядок наприкінці `~/.bashrc`:

```sh
eval "$(starship init bash)"
```

</details>

<details><summary>Cmd</summary>

You need to use [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) with Cmd.
Create a file at this path `%LocalAppData%\clink\starship.lua` with the following contents:

```lua
load(io.popen('starship init cmd'):read("*a"))()
```

</details>

<details><summary>Elvish</summary>

Додайте наступне до кінця `~/.config/elvish/rc.elv` (`%AppData%\elvish\rc.elv` у Windows):

```sh
eval (starship init elvish)
```

Примітка: Підтримується лише Elvish v0.18+. Для версії elvish до v0.21.0 файл конфігурації може бути `~/.elvish/rc.elv` натомість

</details>

<details><summary>Fish</summary>

Додайте наступний рядок наприкінці `~/.config/fish/config.fish`:

```fish
starship init fish | source
```

</details>

<details><summary>Ion</summary>

Додайте наступний рядок наприкінці `~/.config/ion/initrc`:

```sh
eval $(starship init ion)
```

</details>

<details><summary>Nushell</summary>

Додайте наступний рядок наприкінці налаштувань Nushell (знайдіть її за допомоги `$nu.config-path` в Nushell):

```sh
mkdir ($nu.data-dir | path join "vendor/autoload")
starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
```

Примітка: Підтримується лише Nushell v0.96+

</details>

<details><summary>PowerShell</summary>

Add the following to the end of your PowerShell configuration (find it by running `$PROFILE`):

```powershell
Invoke-Expression (&starship init powershell)
```

</details>

<details><summary>Tcsh</summary>

Додайте наступний рядок наприкінці `~/.tcshrc`:

```sh
eval `starship init tcsh`
```

</details>

<details><summary>Xonsh</summary>

Додайте наступний рядок наприкінці `~/.xonshrc`:

```python
execx($(starship init xonsh))
```

</details>

<details><summary>Zsh</summary>

Додайте наступний рядок наприкінці `~/.zshrc`:

```sh
eval "$(starship init zsh)"
```

</details>

### Крок 3. Налаштуйте starship

Запустіть новий екземпляр вашої оболонки і ви побачите новий яскравий командний рядок.
Якщо ви задоволені налаштуваннями, насолоджуйтесь!

Якщо ви бажаєте додатково налаштувати Starship:

- **[Configuration](https://starship.rs/config/)** – learn how to configure Starship to tweak your prompt to your liking

- **[Presets](https://starship.rs/presets/)** – get inspired by the pre-built configuration of others

## 🤝 Участь

We are always looking for contributors of **all skill levels**! If you're looking to ease your way into the project, try out a [good first issue](https://github.com/starship/starship/issues?q=state%3Aopen%20label%3A%22%F0%9F%8C%B1%20good%20first%20issue%22).

Якщо ви вільно володієте мовою відмінною від англійської, ми будемо дуже вдячні, якщо ви допоможете в перекладі документації та її підтримці в актуальному стані. If you would like to help, translations can be contributed on the [Starship Crowdin](https://translate.starship.rs/).

If you are interested in helping contribute to starship, please take a look at our [Contributing Guide](https://github.com/starship/starship/blob/main/CONTRIBUTING.md). Also, feel free to drop into our [Discord server](https://discord.gg/8Jzqu3T) and say hi. 👋

## 💭 Подяки

Будь ласка, перегляньте роботи, які надихнули на створення starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** – A ZSH prompt for astronauts.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** – Cross-shell robbyrussell theme written in JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** – A cross-shell customizable powerline-like prompt with icons.

## ❤️ Спонсори

Support this project by [becoming a sponsor](https://github.com/sponsors/starship). Ваше імʼя або логотип показуватимуться тут з посиланням на ваш сайт.

## 🔒 Політика Підпису коду

Ролі:

Ролі:

- Reviewers: [Astronauts](https://github.com/orgs/starship/teams/astronauts)
- Approvers and Authors: [Mission Control](https://github.com/orgs/starship/teams/mission-control)

Ця програма не передаватиме жодної інформації до інших мережевих систем, якщо користувач або особа, яка її встановлює чи експлуатує, не зробить на це спеціального запиту.

<p align="center"><br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/main/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 Ліцензія

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br>
This project is [ISC](https://github.com/starship/starship/blob/main/LICENSE) licensed.

[alpine linux packages]: https://pkgs.alpinelinux.org/packages?name=starship
[arch linux extra]: https://archlinux.org/packages/extra/x86_64/starship
[chocolatey]: https://community.chocolatey.org/packages/starship
[conda-forge]: https://anaconda.org/conda-forge/starship
[copr]: https://copr.fedorainfracloud.org/coprs/atim/starship
[crates.io]: https://crates.io/crates/starship
[debian main]: https://sources.debian.org/src/starship/1.22.1-1/
[freshports]: https://www.freshports.org/shells/starship
[gentoo packages]: https://packages.gentoo.org/packages/app-shells/starship
[linuxbrew]: https://formulae.brew.sh/formula/starship
[homebrew]: https://formulae.brew.sh/formula/starship
[macports]: https://ports.macports.org/port/starship
[nixpkgs]: https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/st/starship/package.nix
[OSS]: https://software.opensuse.org/package/starship
[pkgsrc]: https://pkgsrc.se/shells/starship
[scoop]: https://github.com/ScoopInstaller/Main/blob/master/bucket/starship.json
[SignPath Foundation]: https://signpath.org
[SignPath.io]: https://signpath.io
[termux]: https://github.com/termux/termux-packages/tree/master/packages/starship
[ubuntu universe]: https://packages.ubuntu.com/source/plucky/starship
[void linux packages]: https://github.com/void-linux/void-packages/tree/master/srcpkgs/starship
[winget]: https://github.com/microsoft/winget-pkgs/tree/master/manifests/s/Starship/Starship
