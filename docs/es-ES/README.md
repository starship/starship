---
home: true
heroImage: /logo.svg
heroText:
tagline: The cross-shell prompt for astronauts
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
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and Powershell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Instalación rápida

1. Instalar el binario de **starship**:


   #### Instalar la última versión

   Con la interfaz de línea de comandos:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Instalar con un gestor de paquetes

   Con [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Con [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Añade el script de inicio al archivo de configuración de tu interfaz de línea de comandos:


   #### Bash

   Añade el siguiente código al final de `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Añade el siguiente código al final de `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Añade el siguiente código al final de `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Añade el siguiente código al final de `~\Documentos\PowerShell\Microsoft.PowerShell_profile.ps1` (o `~/.config/powershell/Microsoft.PowerShell_profile.ps1` en sistemas *nix):

   ```sh
   # ~\Documentos\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Añade el siguiente código al final de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
