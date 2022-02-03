---
home: true
heroImage: /logo.svg
heroText:
tagline: The minimal, blazing-fast, and infinitely customizable prompt for any shell!
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

### Prerequisites

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal.

### 빠른 설치

1. **starship** 바이너리 설치:


   #### 최근 버전 설치

   With Shell:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   Starship을 업데이트하고 싶은 경우에도 위의 스크립트를 실행시키면 됩니다. Starship의 설정은 변경되지 않고 버전만 최근 버전으로 대체될 것입니다.


   #### 패키지 매니저를 이용한 설치

   [Homebrew](https://brew.sh/)를 통한 설치:

   ```sh
   brew install starship
   ```

   [Scoop](https://scoop.sh)을 통한 설치:

   ```powershell
   scoop install starship
   ```

1. 쉘 설정 파일에 init 스크립트 추가:


   #### Bash

   `~/.bashrc`에 아래 라인을 추가

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   `~/.config/fish/config.fish`에 아래 라인을 추가

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   `~/.zshrc`에 아래 라인을 추가

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   `Microsoft.PowerShell_profile.ps1`의 끝부분에 아래 내용을 추가. 해당 설정파일은 파워쉘에서 `$PROFILE` 변수 확인을 통해 확인 가능. 일반적으로 해당 파일은 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` 혹은 -Nix의 경우 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`에 위치.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   아래의 라인을 `~/.config/ion/initrc` 마지막에 추가:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning Only elvish v0.17 or higher is supported. :::

   아래의 라인을 `~/.elvish/rc.elv` 마지막에 추가:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   아래의 라인을 `~/.tcshrc` 마지막에 추가:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning This will change in the future. Only nu version v0.33 or higher is supported. ::: Add the following to your nu config file. You can check the location of this file by running `config path` in nu.

   ```toml
   startup = [
     "mkdir ~/.cache/starship",
     "starship init nu | save ~/.cache/starship/init.nu",
     "source ~/.cache/starship/init.nu",
   ]
   prompt = "starship_prompt"
   ```


   #### Xonsh

   Add the following to the end of `~/.xonshrc`:

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
