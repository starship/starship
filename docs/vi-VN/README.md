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
    details: Mọi chi tiết nhỏ được tuỳ biến theo ý thích của bạn, giúp prompt này nhỏ nhất có thể hoặc phong phú về tính năng như bạn muốn.
footer: Cấp phép bởi ISC | Bản quyền © 2019-nay Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship là prompt nhỏ, cực nhanh, và khả năng tuỳ biến mạnh mẽ cho bất kì shell nào! Hiển thị thông tin bạn cần, trong khi vẫn giữ cho đẹp và nhỏ gọn. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Yêu cầu

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal.

### Cài đặt nhanh chóng

1. Cài đặt **starship** nhị phân:


   #### Cài đặt phiên bản cuối cùng

   Với Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Để cập nhật chính Starship, hãy chạy lại đoạn script bên trên. Nó sẽ thay thế phiên bản hiện tại mà không hề thay đổi gì những cài đặt của Starship trước đó.


   #### Cài đặt thông qua Trình quản lí gói

   Với [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Thêm đoạn mã khởi tạo vào tệp tin cấu hình shell của bạn:


   #### Bash

   Thêm đoạn sau vào cuối tệp tin `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Thêm đoạn sau vào cuối tệp tin `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Thêm đoạn sau vào cuối tệp tin `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Thêm đoạn sau vào cuối tệp tin `Microsoft.PowerShell_profile.ps1`. Bạn có thể kiểm tra vị trí tệp tin này bằng việc truy xuất biến `$PROFILE` trong PowerShell. Thông thường, đường dẫn là `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` hoặc `~/.config/powershell/Microsoft.PowerShell_profile.ps1` trên -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Thêm đoạn sau vào cuối tệp tin `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Only elvish v0.18 or higher is supported.

   :::

   Thêm đoạn sau vào cuối tệp tin `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Thêm đoạn sau vào cuối tệp tin `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   This will change in the future. Only Nushell v0.78+ is supported.

   :::

   Thêm đoạn code dưới đây vào cuối file Nushell env của bạn (Bạn có thể tìm đường dẫn tới file Nushell env bằng cách chạy `$nu.env-path` trong Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   And add the following to the end of your Nushell configuration (find it by running `$nu.config-path`):

   ```sh
   use ~/.cache/starship/init.nu
   ```


   #### Xonsh

   Thêm dòng này vào cuối của file `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   You need to use [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) with Cmd. Add the following to a file `starship.lua` and place this file in Clink scripts directory:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
