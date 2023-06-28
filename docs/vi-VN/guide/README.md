<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Cross-shell prompt"
 />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/actions/workflow/status/starship/starship/workflow.yml?branch=master&label=workflow&style=flat-square"
      alt="Trạng thái GitHub Actions workflow"
 /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Phiên bản Crates.io"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Trạng thái đóng gói" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Trò chuyện trên Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Theo dõi @StarshipPrompt trên Twitter"
 /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Website</a>
  ·
  <a href="#🚀-installation">Cài đặt</a>
  ·
  <a href="https://starship.rs/config/">Cấu hình</a>
</p>

<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png"
      alt="Tiếng Anh"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Tiếng Đức"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/es-ES/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Tiếng Tây Ban Nha"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/fr-FR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Thiếng Pháp"
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
      alt="Tiếng Nhật"
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
      alt="Tiếng Nga"
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
      alt="Tiếng Trung giản thể"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="Tiếng Trung hiện đại"
 /></a>
</p>

[![SWUbanner](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine)

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship với iTerm2 and chủ đề Snazzy"
  width="50%"
  align="right"
 />

**Nhỏ gọn, cực nhanh, và khả năng tuỳ chỉnh vô hạn prompt cho bất kì shell nào!**

- **Nhanh:** nó có tốc độ nhanh – _thực sự_ nhanh! 🚀
- **Khả năng tuỳ chỉnh:** cấu hình mọi thứ trên prompt của bạn.
- **Độ phổ biến:** làm việc trên mọi shell, bất kì hệ điều hành nào.
- **Thông minh:** hiển thị thông tin liên quan dưới một cái nhìn.
- **Tính năng phong phú:** hỗ trợ tất cả các công cụ yêu thích của bạn.
- **Dễ dàng:** cài đặt nhanh chóng – bắt đầu sử dụng nó trong vài phút.

<p align="center">
<a href="https://starship.rs/config/"><strong>Khám phá tài liệu của Starship&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 Cài đặt

### Yêu cầu

- Bạn phải có [Nerd Font](https://www.nerdfonts.com/) được cài đặt và kích hoạt trên terminal (ví dụ, bạn có thể thử [FiraCode NerdFont](https://www.nerdfonts.com/font-downloads)).

### Bước 1. Cài Đặt Starship

Chọn hệ điều hành của bạn trong danh sách bên dưới để xem hướng dẫn cài đặt:

<details>
<summary>Android</summary>

Cài đặt Starship bằng một package manager bất kì:

| Repository                                                                        | Instructions           |
| --------------------------------------------------------------------------------- | ---------------------- |
| [Termux](https://github.com/termux/termux-packages/tree/master/packages/starship) | `pkg install starship` |

</details>

<details>
<summary>BSD</summary>

Cài đặt Starship bằng một package manager bất kì:

| Distribution | Repository                                               | Instructions                      |
| ------------ | -------------------------------------------------------- | --------------------------------- |
| **_Any_**    | **[crates.io](https://crates.io/crates/starship)**       | `cargo install starship --locked` |
| FreeBSD      | [FreshPorts](https://www.freshports.org/shells/starship) | `pkg install starship`            |
| NetBSD       | [pkgsrc](https://pkgsrc.se/shells/starship)              | `pkgin install starship`          |

</details>

<details>
<summary>Linux</summary>

Cài đặt phiên bản mới nhất cho hệ điều hành của bạn:

```sh
curl -sS https://starship.rs/install.sh | sh
```

Hoặc là, cài đặt Starship bằng một package manager bất kì:

| Distribution       | Repository                                                                                      | Instructions                                                                   |
| ------------------ | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| **_Any_**          | **[crates.io](https://crates.io/crates/starship)**                                              | `cargo install starship --locked`                                              |
| _Any_              | [conda-forge](https://anaconda.org/conda-forge/starship)                                        | `conda install -c conda-forge starship`                                        |
| _Any_              | [Linuxbrew](https://formulae.brew.sh/formula/starship)                                          | `brew install starship`                                                        |
| _Any_              | [Snapcraft](https://snapcraft.io/starship)                                                      | `snap install --edge starship`                                                 |
| Alpine Linux 3.13+ | [Alpine Linux Packages](https://pkgs.alpinelinux.org/packages?name=starship)                    | `apk add starship`                                                             |
| Arch Linux         | [Arch Linux Extra](https://archlinux.org/packages/extra/x86_64/starship)                        | `pacman -S starship`                                                           |
| CentOS 7+          | [Copr](https://copr.fedorainfracloud.org/coprs/atim/starship)                                   | `dnf copr enable atim/starship` <br /> `dnf install starship` |
| Gentoo             | [Gentoo Packages](https://packages.gentoo.org/packages/app-shells/starship)                     | `emerge app-shells/starship`                                                   |
| Manjaro            |                                                                                                 | `pacman -S starship`                                                           |
| NixOS              | [nixpkgs](https://github.com/NixOS/nixpkgs/blob/master/pkgs/tools/misc/starship/default.nix)    | `nix-env -iA nixpkgs.starship`                                                 |
| Void Linux         | [Void Linux Packages](https://github.com/void-linux/void-packages/tree/master/srcpkgs/starship) | `xbps-install -S starship`                                                     |

</details>

<details>
<summary>macOS</summary>

Cài phiên bản mới nhất cho hệ điều hành của bạn:

```sh
curl -sS https://starship.rs/install.sh | sh
```

Hoặc là, cài đặt Starship bằng một package manager bất kì:

| Repository                                               | Instructions                            |
| -------------------------------------------------------- | --------------------------------------- |
| **[crates.io](https://crates.io/crates/starship)**       | `cargo install starship --locked`       |
| [conda-forge](https://anaconda.org/conda-forge/starship) | `conda install -c conda-forge starship` |
| [Homebrew](https://formulae.brew.sh/formula/starship)    | `brew install starship`                 |
| [MacPorts](https://ports.macports.org/port/starship)     | `port install starship`                 |

</details>

<details>
<summary>Windows</summary>

Cài đặt phiên bản mới nhất cho hệ điều hành của bạn với MSI-installers từ [phần release](https://github.com/starship/starship/releases/latest).

Cài đặt Starship bằng một package manager bất kì:

| Repository                                                                                   | Instructions                            |
| -------------------------------------------------------------------------------------------- | --------------------------------------- |
| **[crates.io](https://crates.io/crates/starship)**                                           | `cargo install starship --locked`       |
| [Chocolatey](https://community.chocolatey.org/packages/starship)                             | `choco install starship`                |
| [conda-forge](https://anaconda.org/conda-forge/starship)                                     | `conda install -c conda-forge starship` |
| [Scoop](https://github.com/ScoopInstaller/Main/blob/master/bucket/starship.json)             | `scoop install starship`                |
| [winget](https://github.com/microsoft/winget-pkgs/tree/master/manifests/s/Starship/Starship) | `winget install --id Starship.Starship` |

</details>

### Bước 2. Thiết lập shell của bạn để dùng Starship

Cấu hình shell của bạn để chạy starship. Bạn có thể chọn từ danh sách bên dưới:

<details>
<summary>Bash</summary>

Thêm đoạn sau vào cuối tệp tin `~/.bashrc`:

```sh
eval "$(starship init bash)"
```

</details>

<details>
<summary>Cmd</summary>

Bạn cần phải dùng [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) với Cmd. Tạo một file như đường dẫn `%LocalAppData%/clink/starship.lua` với nội dung như sau:

```lua
load(io.popen('starship init cmd'):read("*a"))()
```

</details>

<details>
<summary>Elvish</summary>

Thêm đoạn sau vào cuối tệp tin `~/.elvish/rc.elv`:

```sh
eval (starship init elvish)
```

Lưu ý: Chỉ hỗ trợ Elvish v0.18+

</details>

<details>
<summary>Fish</summary>

Thêm đoạn sau vào cuối tệp tin `~/.config/fish/config.fish`:

```fish
starship init fish | source
```

</details>

<details>
<summary>Ion</summary>

Thêm đoạn sau vào cuối tệp tin `~/.config/ion/initrc`:

```sh
eval $(starship init ion)
```

</details>

<details>
<summary>Nushell</summary>

Thêm đoạn code dưới đây vào cuối file Nushell env của bạn (Bạn có thể tìm đường dẫn tới file Nushell env bằng cách chạy `$nu.env-path` trong Nushell):

```sh
mkdir ~/.cache/starship
starship init nu | save -f ~/.cache/starship/init.nu
```

Thêm đoạn code sau vào cuối file cấu hình Nushell (bạn có thể tìm đường dẫn tới file cấu hình Nushell bằng cách chạy câu lệnh `$nu.config-path`):

```sh
use ~/.cache/starship/init.nu
```

Lưu ý: Chỉ hỗ trợ Nushell v0.78+

</details>

<details>
<summary>PowerShell</summary>

Thêm đoạn code sau vào cuối file cấu hình PowerShell (bạn có thể tìm đường dẫn tới file cấu hình PowerShell bằng cách chạy câu lệnh `$PROFILE`):

```powershell
Invoke-Expression (&starship init powershell)
```

</details>

<details>
<summary>Tcsh</summary>

Thêm đoạn sau vào cuối tệp tin `~/.tcshrc`:

```sh
eval `starship init tcsh`
```

</details>

<details>
<summary>Xonsh</summary>

Thêm dòng này vào cuối của file `~/.xonshrc`:

```python
execx($(starship init xonsh))
```

</details>

<details>
<summary>Zsh</summary>

Thêm đoạn sau vào cuối tệp tin `~/.zshrc`:

```sh
eval "$(starship init zsh)"
```

</details>

### Bước 3. Cấu hình Starship

Khởi tạo một shell mới, và bạn sẽ thấy một chiếc shell mới tinh, đẹp lung linh. Nếu bạn hài lòng với cấu hình mặc định thì giờ là lúc mà bạn nên tận hưởng chiếc shell mới của mình!

Nếu bạn muốn tùy chỉnh Starship nhiều hơn nữa:

- **[Cấu hình](https://starship.rs/config/)** – học cách cấu hình Starship để tùy chỉnh prompt theo ý bạn

- **[Cấu hình mẫu](https://starship.rs/presets/)** – lấy cảm hứng từ cấu hình của những người khác

## 🤝 Đóng góp

Chúng tôi luôn luôn tìm kiếm những cộng tác viên ở **tất cả các các mức độ về kĩ năng**! Nếu bạn đang tìm kiếm cách dễ dàng để tham gia vào dự án, thử một [good issue đầu tiên](https://github.com/starship/starship/labels/🌱%20good%20first%20issue).

Nếu bạn thành thạo một ngôn ngữ không phải Tiếng Anh, chúng tôi đánh giá cao mọi sự giúp đỡ của bạn trong việc dịch tài liệu của chúng tôi và cập nhật các ngôn ngữ khác. Nếu bạn muốn giúp đỡ, những bản dịch có thể được đóng góp trên [Starship Crowdin](https://translate.starship.rs/).

Nếu bạn thích thú trong việc giúp đỡ đóng góp cho starship, xin hãy xem [Hướng dẫn đóng góp](https://github.com/starship/starship/blob/master/CONTRIBUTING.md) của chúng tôi. Ngoài ra, vui lòng truy cập vào [máy chủ Discord](https://discord.gg/8Jzqu3T) và nói xin chào. 👋

## Cảm hứng

Xin hãy xem qua những công việc này trước đây, những thứ đã giúp truyền cảm hứng để tạo ra starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** – A ZSH prompt for astronauts.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** – Cross-shell robbyrussell theme written in JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** – A cross-shell customizable powerline-like prompt with icons.

## Tài trợ

Hỗ trợ project này bằng việc [trở thành nhà tài trợ](https://github.com/sponsors/starship). Tên hoặc logo của nhà tài trợ sẽ được hiển thị với một liên kết dẫn tới trang web của họ.

**Nhà tài trợ ủng hộ**

- [Appwrite](https://appwrite.io/)

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 Giấy phép

Bản quyền © 2019-nay, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> Dự án này được [ISC](https://github.com/starship/starship/blob/master/LICENSE) cấp phép.
