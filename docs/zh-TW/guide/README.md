<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – 跨 Shell 的提示字元"
 />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/actions/workflow/status/starship/starship/workflow.yml?branch=master&label=workflow&style=flat-square"
      alt="GitHub Actions workflow status"
 /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Crates.io 版本"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Packaging status" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Chat on Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="在推特上追蹤 @StarshipPrompt"
 /></a>
  <a href="https://stand-with-ukraine.pp.ua"
    ><img
      src="https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraineFlat.svg"
      alt="Stand With Ukraine"
 /></a>
</p>

<p align="center">
  <a href="https://starship.rs">網站</a>
  ·
  <a href="#🚀-installation">安裝</a>
  ·
  <a href="https://starship.rs/config/">設定</a>
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
    href="https://github.com/starship/starship/blob/master/docs/id-ID/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-id.png"
      alt="印尼文 Bahasa"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/it-IT/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-it.png"
      alt="意大利語"
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
      alt="巴西葡萄牙語"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png"
      alt="俄語"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/uk-UA/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ua.png"
      alt="烏克蘭語"
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
  alt="Starship with iTerm2 and the Snazzy theme"
  width="50%"
  align="right"
 />

**適合任何 shell 的最小、極速、無限客製化的提示字元！**

- **效率：**很快 – _非常非常_快！ 🚀
- **客製化：**全方面設定你的提示字元。
- **通用性：**適用於任何 Shell 以及作業系統。
- **智慧：**一目了然地顯示相關資訊。
- **豐富的功能：**支援所有你喜歡的工具。
- **易用：**安裝快速 – 幾分鐘內即可開始使用。

<p align="center">
<a href="https://starship.rs/config/"><strong>探索 Starship 文件&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 安裝

### 先決要求

- 至少一個[Nerd Font](https://www.nerdfonts.com/)需要已經被安裝，並且在終端模擬器中啓用 (你可以試試看[FireCode Nerd Font](https://www.nerdfonts.com/font-downloads))。

### 第一步 安裝 Starship

瀏覽各系統的安裝指示

<details>
<summary>Android</summary>

透過下列的套件管理器安裝 Starship：

| 儲存庫                                                                               | 說明                     |
| --------------------------------------------------------------------------------- | ---------------------- |
| [Termux](https://github.com/termux/termux-packages/tree/master/packages/starship) | `pkg install starship` |

</details>

<details>
<summary>BSD</summary>

透過下列的套件管理器安裝 Starship：

| 發行版本       | 儲存庫                                                      | 說明                                |
| ---------- | -------------------------------------------------------- | --------------------------------- |
| **_任一版本_** | **[crates.io](https://crates.io/crates/starship)**       | `cargo install starship --locked` |
| FreeBSD    | [FreshPorts](https://www.freshports.org/shells/starship) | `pkg install starship`            |
| NetBSD     | [pkgsrc](https://pkgsrc.se/shells/starship)              | `pkgin install starship`          |

</details>

<details>
<summary>Linux</summary>

為你的系統安裝最新版本：

```sh
curl -sS https://starship.rs/install.sh | sh
```

或者，透過下列的套件管理器安裝 Starship：

| 發行版本               | 儲存庫                                                                                             | 說明                                                                             |
| ------------------ | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| **_任一版本_**         | **[crates.io](https://crates.io/crates/starship)**                                              | `cargo install starship --locked`                                              |
| _任一版本_             | [conda-forge](https://anaconda.org/conda-forge/starship)                                        | `conda install -c conda-forge starship`                                        |
| _任一版本_             | [Linuxbrew](https://formulae.brew.sh/formula/starship)                                          | `brew install starship`                                                        |
| Alpine Linux 3.13+ | [Alpine Linux Packages](https://pkgs.alpinelinux.org/packages?name=starship)                    | `apk add starship`                                                             |
| Arch Linux         | [Arch Linux Extra](https://archlinux.org/packages/extra/x86_64/starship)                        | `pacman -S starship`                                                           |
| CentOS 7+          | [Copr](https://copr.fedorainfracloud.org/coprs/atim/starship)                                   | `dnf copr enable atim/starship` <br /> `dnf install starship` |
| Debian 13+         | [Debian Main](https://sources.debian.org/src/starship/1.22.1-1/)                                | `apt install starship`                                                         |
| Fedora 40+         | [Copr](https://copr.fedorainfracloud.org/coprs/atim/starship)                                   | `dnf copr enable atim/starship` <br /> `dnf install starship` |
| Gentoo             | [Gentoo Packages](https://packages.gentoo.org/packages/app-shells/starship)                     | `emerge app-shells/starship`                                                   |
| Manjaro            |                                                                                                 | `pacman -S starship`                                                           |
| NixOS              | [nixpkgs](https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/st/starship/package.nix)    | `nix-env -iA nixpkgs.starship`                                                 |
| openSUSE           | [OSS](https://software.opensuse.org/package/starship)                                           | `zypper in starship`                                                           |
| Ubuntu 25.04+      | [Ubuntu Universe](https://packages.ubuntu.com/source/plucky/starship)                           | `apt install starship`                                                         |
| Void Linux         | [Void Linux Packages](https://github.com/void-linux/void-packages/tree/master/srcpkgs/starship) | `xbps-install -S starship`                                                     |

</details>

<details>
<summary>macOS</summary>

為你的系統安裝最新版本：

```sh
curl -sS https://starship.rs/install.sh | sh
```

或者，透過下列的套件管理器安裝 Starship：

| 儲存庫                                                      | 說明                                      |
| -------------------------------------------------------- | --------------------------------------- |
| **[crates.io](https://crates.io/crates/starship)**       | `cargo install starship --locked`       |
| [conda-forge](https://anaconda.org/conda-forge/starship) | `conda install -c conda-forge starship` |
| [Homebrew](https://formulae.brew.sh/formula/starship)    | `brew install starship`                 |
| [MacPorts](https://ports.macports.org/port/starship)     | `port install starship`                 |

</details>

<details>
<summary>Windows</summary>

透過[發布區](https://github.com/starship/starship/releases/latest)中的 MSI 安裝程式為你的系統安裝最新版本：

透過下列的套件管理器安裝 Starship：

| 儲存庫                                                                                          | 說明                                      |
| -------------------------------------------------------------------------------------------- | --------------------------------------- |
| **[crates.io](https://crates.io/crates/starship)**                                           | `cargo install starship --locked`       |
| [Chocolatey](https://community.chocolatey.org/packages/starship)                             | `choco install starship`                |
| [conda-forge](https://anaconda.org/conda-forge/starship)                                     | `conda install -c conda-forge starship` |
| [Scoop](https://github.com/ScoopInstaller/Main/blob/master/bucket/starship.json)             | `scoop install starship`                |
| [winget](https://github.com/microsoft/winget-pkgs/tree/master/manifests/s/Starship/Starship) | `winget install --id Starship.Starship` |

</details>

### 第二步 設定您的 shell 以啟用 Starship

設定您的 shell 以啟用 Starship。 請從下列選單選取您的 shell：

<details>
<summary>Bash</summary>

將以下內容放到 `~/.bashrc` 的結尾：

```sh
eval "$(starship init bash)"
```

</details>

<details>
<summary>命令提示字元</summary>

您需要在 Cmd 中使用 [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+)。 在此路徑 `%LocalAppData%\clink\starship.lua` 建立一個檔案，並填入以下內容：

```lua
load(io.popen('starship init cmd'):read("*a"))()
```

</details>

<details>
<summary>Elvish</summary>

Add the following to the end of `~/.config/elvish/rc.elv` (`%AppData%\elvish\rc.elv` on Windows):

```sh
eval (starship init elvish)
```

注意：只支援 Elvish v0.18+ 以上的版本. For elvish versions prior to v0.21.0 the config file might instead be `~/.elvish/rc.elv`

</details>

<details>
<summary>Fish</summary>

將以下內容放到 `~/.config/fish/config.fish` 的結尾：

```fish
starship init fish | source
```

</details>

<details>
<summary>Ion</summary>

將以下內容放到 `~/.config/ion/initrc` 的結尾：

```sh
eval $(starship init ion)
```

</details>

<details>
<summary>Nushell</summary>

Add the following to the end of your Nushell configuration (find it by running `$nu.config-path` in Nushell):

```sh
mkdir ($nu.data-dir | path join "vendor/autoload")
starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
```

注意：只支援 Nushell v0.96+ 的版本

</details>

<details>
<summary>PowerShell</summary>

新增下列的內容至你的 PowerShell 設定檔最下方（執行  `$PROFILE` 找到它）：

```powershell
Invoke-Expression (&starship init powershell)
```

</details>

<details>
<summary>Tcsh</summary>

將以下內容放到 `~/.tcshrc` 的結尾：

```sh
eval `starship init tcsh`
```

</details>

<details>
<summary>Xonsh</summary>

將以下內容加到 `~/.xonshrc` 的結尾：

```python
execx($(starship init xonsh))
```

</details>

<details>
<summary>Zsh</summary>

將以下內容放到 `~/.zshrc` 的結尾：

```sh
eval "$(starship init zsh)"
```

</details>

### 第三步 設定 Starship

開啟一個新的 shell，您應該可以看到新的美麗 shell 提示字元。 若您對預設值感到滿意，盡情享受！

如果您想要更加客製化 Startship ：

- **[設定](https://starship.rs/config/)**：學習如何設定 Starship 來微調提示字元的外觀

- **[其他預設組態](https://starship.rs/presets/)**：從別人的設定當中獲得啟發

## 🤝 貢獻

我們歡迎具有**各式各樣能力**的貢獻者！ If you're looking to ease your way into the project, try out a [good first issue](https://github.com/starship/starship/issues?q=state%3Aopen%20label%3A%22%F0%9F%8C%B1%20good%20first%20issue%22).

如果您精通非英語語言，並能協助我們在文檔上的翻譯保持最新狀態，我們會非常感謝！ 如果你想要提供翻譯，可以到 [Starship Crowdin](https://translate.starship.rs/) 上提交您的翻譯。

如果你對貢獻 Starship 有興趣，請看我們的 [貢獻指南](https://github.com/starship/starship/blob/master/CONTRIBUTING.md) 。 另外，請不用客氣加入我們的 [Discord 伺服器](https://discord.gg/8Jzqu3T) 並來問候一下。 👋

## 💭 發想來自

請看之前這些幫助我們創造 Starship 的前任作品。 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** – 一個給太空人用的 ZSH 提示符號。

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** – 使用 JavaScript 寫出的跨終端機 robbyrussell 主題。

- **[reujab/silver](https://github.com/reujab/silver)** - 一個跨 shell、可客製化、像 powerline 的圖案提示符號。

## ❤️ 贊助我們

你可以[成爲一個贊助者](https://github.com/sponsors/starship)來支持這個專案！ 你的名字和頭像會在這裏顯示，並且會帶有一個前往你網站的鏈接。

## 🔒 程式碼簽章政策

Free code signing provided by [SignPath.io](https://signpath.io), certificate by [SignPath Foundation](https://signpath.org).

Code Signing Roles:

- 審查者： [Astronauts](https://github.com/orgs/starship/teams/mission-control)
- 核准者與作者： [Mission Control](https://github.com/orgs/starship/teams/mission-control)

This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 許可

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> 這個專案使用 [ISC](https://github.com/starship/starship/blob/master/LICENSE) 許可。
