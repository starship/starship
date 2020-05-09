---
home: true
heroImage: /logo.svg
heroText:
tagline: O prompt mínimo, extremamente rápido e infinitamente personalizável para qualquer shell!
actionText: Começar →
actionLink: ./guide/
features:
  - 
    title: Compatibilidade primeiro
    details: Funciona nos principais shells nos principais sistemas operacionais. Use em qualquer lugar!
  - 
    title: Poder do Rust
    details: Tenha o melhor da velocidade e segurança do Rust, para tornar seu prompt o mais rápido e confiável possível.
  - 
    title: Personalizável
    details: Cada pequeno detalhe é personalizável ao seu gosto, para tornar esse prompt o mínimo possível ou rico em recursos, como você preferir.
footer: Licenciado por ISC | Todos os direitos reservados © 2019-Atual | Contribuidores Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship: Multi Shell Prompt"
description: O Starship é o prompt mínimo, extremamente rápido e extremamente personalizável para qualquer shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Quick Install

1. Install the **starship** binary:


   #### Install Latest Version

   With Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Install via Package Manager

   With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   With [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Add the init script to your shell's config file:


   #### Bash

   Add the following to the end of `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Add the following to the end of `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Add the following to the end of `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Add the following to the end of `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix):

   ```sh
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Add the following to the end of `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
