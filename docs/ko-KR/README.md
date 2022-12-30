---
home: true
heroImage: /logo.svg
heroText:
tagline: 간결하고 화끈하게 빠르며 무제한으로 커스터마이징이 가능한 프롬프트. 어떤 쉘에서든 사용할 수 있습니다!
actionText: Get Started →
actionLink: ./guide/
features:
  - 
    title: Compatibility First
    details: Works on the most common shells on the most common operating systems. Use it everywhere!
  - 
    title: Rust-Powered
    details: Brings the best-in-class speed and safety of Rust, to make your prompt as quick and reliable as possible.
  - 
    title: Customizable
    details: Every little detail is customizable to your liking, to make this prompt as minimal or feature-rich as you'd like it to be.
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### 준비 사항

- 터미널에 [Nerd Font](https://www.nerdfonts.com/)가 설치되어 있고 사용 가능해야 합니다.

### 빠른 설치

1. **starship** 바이너리 설치:


   #### 최근 버전 설치

   With Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Starship을 업데이트하고 싶은 경우에도 위의 스크립트를 실행시키면 됩니다. Starship의 설정은 변경되지 않고 버전만 최근 버전으로 대체될 것입니다.


   #### 패키지 매니저를 이용한 설치

   [Homebrew](https://brew.sh/)를 통한 설치:

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. 쉘 설정 파일에 init 스크립트 추가:


   #### Bash

   `~/.bashrc`의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   `~/.config/fish/config.fish`의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   `~/.zshrc`의 끝부분에 아래 라인을 추가

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   `Microsoft.PowerShell_profile.ps1`의 끝부분에 아래 내용을 추가합니다. 해당 설정파일은 파워쉘에서 `$PROFILE` 변수 확인을 통해 확인 가능합니다. 일반적으로 해당 파일은 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` 혹은 -Nix의 경우 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`에 있습니다.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   `~/.config/ion/initrc` 의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Only elvish v0.18 or higher is supported.

   :::

   `~/.elvish/rc.elv` 의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   `~/.tcshrc` 의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   This will change in the future. Only Nushell v0.73+ is supported.

   :::

   Add the following to the end of your Nushell env file (find it by running `$nu.env-path` in Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   And add the following to the end of your Nushell configuration (find it by running `$nu.config-path`):

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   `~/.xonshrc` 의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Cmd를 이용하려면 [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) 를 사용해야 합니다. `starship.lua` 파일에 아래의 라인을 추가하고 파일을 Clink scripts 폴더에 저장합니다.

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
