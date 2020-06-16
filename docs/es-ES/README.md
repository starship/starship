---
home: true
heroImage: /logo.svg
heroText:
tagline: El símbolo de sistema minimalista, ultrarápido e infinitamente customizable para cualquier intérprete de comandos.
actionText: Empezar →
actionLink: ./guide/
features:
  - 
    title: Compatibilidad primero
    details: Funciona en las interfaces de líneas de comando (shells) más comunes en los sistemas operativos más comunes. ¡Úsalo donde sea!
  - 
    title: Desarrollado en Rust
    details: Obtén la mayor velocidad y seguridad de Rust, para hacer tu prompt lo más rápida y segura posible.
  - 
    title: Personalizable
    details: Puedes personalizar cada pequeño detalle a tu gusto, de manera que puedes tener una interfaz minimalista o rica en funcionalidades.
footer: Bajo una licencia ISC | Derechos de autor © 2019-presente Colaboradores de Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship: el indicador de comando multi-intérprete"
description: Starship es el símbolo de sistema minimalista, ultrarápido e infinitamente customizable para cualquier intérprete de comandos. Muestra la información que necesitas, a la par que es elegante y minimalista. Instalación rápida disponible para Bash, Fish, ZSH, Ion y PowerShell.
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
