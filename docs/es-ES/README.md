---
home: true
heroImage: /logo.svg
heroText:
tagline: '¡El prompt minimalista, ultrarápido e infinitamente personalizable para cualquier intérprete de comandos!'
actionText: Comenzar →
actionLink: ./guide/
features:
  - 
    title: Compatibilidad primero
    details: Funciona en los intérprete de comandos más comunes de los sistemas operativos más comunes. ¡Úsalo en todas partes!
  - 
    title: Desarrollado en Rust
    details: Obtén la mayor velocidad y seguridad de Rust, para hacer tu prompt lo más rápida y segura posible.
  - 
    title: Personalizable
    details: Puedes personalizar cada pequeño detalle a tu gusto, de manera que puedes tener un prompt minimalista o rico en funcionalidades.
footer: Bajo una licencia ISC | Derechos de autor © 2019-presente Colaboradores de Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship: el prompt multi-intérprete"
description: '¡Starship es el prompt minimalista, ultrarápido e infinitamente personalizable para cualquier intérprete de comandos! Muestra la información que necesitas, a la par que es elegante y minimalista. Instalación rápida disponible para Bash, Fish, ZSH, Ion y PowerShell.'
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Instalación rápida

1. Instalar el binario de **Starship**:


   #### Instalar la última versión

   Con el intérprete de comandos:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```
   To update the Starship itself, rerun the above script. It will replace the current version without touching Starship's configuration.


   #### Instalar con un gestor de paquetes

   With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Con [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Añade el guión de inicio al archivo de configuración de tu intérprete de comandos:


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

   Añade lo siguiente al final de `Microsoft.PowerShell_profile.ps1`. Puedes comprobar la ubicación de este archivo consultando la variable `$PROFILE` en PowerShell. Normalmente la ruta es `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` o `~/.config/powershell/Microsoft.PowerShell_profile.ps1` en -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Añade el siguiente código al final de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

   #### Elvish

   ::: advertencia Solo se admite Elvish v0.15 o superior. :::

   Añade el siguiente código al final de `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Añade el siguiente código al final de `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```
