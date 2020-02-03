---
home: true
heroImage: /logo.svg
heroText: null
tagline: The cross-shell prompt for astronauts
actionText: Empezar →
actionLink: ./guide/
footer: Bajo una licencia ISC | Derechos de autor © 2019-presente Colaboradores de Starship
---

<div class="features">
  <div class="feature">
    <h2>Compatibilidad primero</h2>
    <p>Funciona en las interfaces de líneas de comando (shells) más comunes en los sistemas operativos más comunes. ¡Úsalo donde sea!</p>
  </div>
  <div class="feature">
    <h2>Desarrollado en Rust</h2>
    <p>Obtén la mayor velocidad y seguridad de Rust, para hacer tu prompt lo más rápida y segura posible.</p>
  </div>
  <div class="feature">
    <h2>Personalizable</h2>
    <p>Puedes personalizar cada pequeño detalle a tu gusto, de manera que puedes tener una interfaz minimalista o rica en funcionalidades.</p>
  </div>
</div>

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
