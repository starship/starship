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
  <a href="https://starship.rs">Website</a>
  ·
  <a href="#🚀-installation">Installation</a>
  ·
  <a href="https://starship.rs/config/">Configuration</a>
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
  <a 
    href="https://github.com/starship/starship/blob/master/docs/es-ES/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Español" /></a>
  &nbsp;
  <a 
    href="https://github.com/starship/starship/blob/master/docs/fr-FR/guide/README.md"
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

**Nhỏ gọn, cực nhanh và khả năng tuỳ chỉnh vô hạn prompt cho bất kì shell nào!**

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

- Đã cài đặt m [Nerd Font](https://www.nerdfonts.com/) và đã kích hoạt trong giao diện dòng lệnh của bạn (ví dụ, thử [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)).

### Bắt đầu

**Lưu ý**: do sự gia tăng của các nền tảng khác nhau, chỉ một tập con các nền tảng hỗ trợ được hiển thị bên dưới. Không thể tìm thấy nền tảng phù hợp của bạn? Hãy xem một [hướng dẫn bổ sung cho các nền tảng khác](https://starship.rs/installing/).

1. Cài đặt **starship** nhị phân:


   #### Cài đặt phiên bản cuối cùng


   ##### Từ bản nhịn phân có sẵn, với Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Cài đặt thông qua Trình quản lí gói


   ##### Ví dụ: [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### Với [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

2. Thêm đoạn mã khởi tạo vào tệp tin cấu hình shell của bạn:


   #### Bash

   Thêm vào cuối tệp tin `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Thêm vào cuối tệp tin `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Thêm vào cuối tệp tin `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   Thêm vào cuối tệp tin `Microsoft.PowerShell_profile.ps1`. Bạn có thể kiểm tra vị trí tệp tin này bằng việc truy xuất biến `$PROFILE` trong PowerShell. Thông thường, đường dẫn là `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` hoặc `~/.config/powershell/Microsoft.PowerShell_profile.ps1` trên -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Thêm vào cuối tệp tin `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

## 🤝 Đóng góp

Chúng tôi luôn luôn tìm kiếm những người đóng góp ở**tất cả các các mức độ về kĩ năng**! If you're looking to ease your way into the project, try out a [good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue).

If you are fluent in a non-English language, we greatly appreciate any help keeping our docs translated and up-to-date in other languages. If you would like to help, translations can be contributed on the [Starship Crowdin](https://translate.starship.rs/).

If you are interested in helping contribute to starship, please take a look at our [Contributing Guide](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Also, feel free to drop into our [Discord server](https://discord.gg/8Jzqu3T) and say hi. 👋

### Code Contributors

This project exists thanks to all the people who contribute. [[Contribute](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)].
<a href="https://github.com/starship/starship/graphs/contributors"><img src="https://opencollective.com/starship/contributors.svg?width=890&button=false" /></a>

### Financial Contributors

Become a financial contributor and help us sustain our community. [[Contribute](https://opencollective.com/starship/contribute)]

#### Individuals

<a href="https://opencollective.com/starship"><img src="https://opencollective.com/starship/individuals.svg?width=890"></a>

#### Organizations

Support this project with your organization. Your logo will show up here with a link to your website. [[Contribute](https://opencollective.com/starship/contribute)]

<a href="https://opencollective.com/starship/organization/0/website"><img src="https://opencollective.com/starship/organization/0/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/1/website"><img src="https://opencollective.com/starship/organization/1/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/2/website"><img src="https://opencollective.com/starship/organization/2/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/3/website"><img src="https://opencollective.com/starship/organization/3/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/4/website"><img src="https://opencollective.com/starship/organization/4/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/5/website"><img src="https://opencollective.com/starship/organization/5/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/6/website"><img src="https://opencollective.com/starship/organization/6/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/7/website"><img src="https://opencollective.com/starship/organization/7/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/8/website"><img src="https://opencollective.com/starship/organization/8/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/9/website"><img src="https://opencollective.com/starship/organization/9/avatar.svg"></a>

## 💭 Inspired By

Please check out these previous works that helped inspire the creation of starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - A ZSH prompt for astronauts.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - Cross-shell robbyrussell theme written in JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - A cross-shell customizable powerline-like prompt with icons.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 License

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> This project is [ISC](https://github.com/starship/starship/blob/master/LICENSE) licensed.
