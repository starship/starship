---
home: true
heroImage: /logo.svg
heroText:
tagline: Nhỏ gọn, cực nhanh, và khả năng tuỳ chỉnh vô hạn prompt cho bất kì shell nào!
actionText: Bắt đầu →
actionLink: ./guide/
features:
  - 
    title: Khả năng tương thích
    details: Những công việc trên đa số các shell phổ biến trên đa số các hệ điều hành phổ biến. Sử dụng nó ở mọi nơi!
  - 
    title: Sức mạnh của Rust
    details: Mang lại tốc độ và độ an toàn tốt nhất của Rust, giúp prompt của bạn thực thi nhanh chóng và đáng tin cậy nhất có thể.
  - 
    title: Khả năng tuỳ biến
    details: Every little detail is customizable to your liking, to make this prompt as minimal or feature-rich as you'd like it to be.
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Quick Install

1. Cài đặt **starship** nhị phân:


   #### Cài đặt phiên bản cuối cùng

   With Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Cài đặt thông qua Trình quản lí gói

   With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   With [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Thêm đoạn mã khởi tạo vào tệp tin cấu hình shell của bạn:


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


   #### Powershell

   Add the following to the end of `Microsoft.PowerShell_profile.ps1`. You can check the location of this file by querying the `$PROFILE` variable in PowerShell. Thông thường, đường dẫn là `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` hoặc `~/.config/powershell/Microsoft.PowerShell_profile.ps1` trên -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Thêm vào cuối tệp tin `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
