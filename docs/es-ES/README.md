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
description: '¡Starship es el prompt minimalista, ultrarápido e infinitamente personalizable para cualquier intérprete de comandos! Muestra la información que necesitas, a la par que es elegante y minimalista. Instalación rápida disponible para Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, y PowerShell.'
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Prerequisitos

- Una [Nerd Font](https://www.nerdfonts.com/) instalada y habilitada en tu terminal.

### Instalación rápida

1. Instalar el binario de **Starship**:


   #### Instalar la última versión

   Con el intérprete de comandos:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   Para actualizar Starship, vuelve a ejecutar el guión anterior. Reemplazará la versión actual sin tocar la configuración de Starship.


   #### Instalar con un gestor de paquetes

   Con [Homebrew](https://brew.sh/):

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


   #### Nushell

   ::: warning This will change in the future. Only nu version v0.33 or higher is supported. ::: Add the following to your nu config file. You can check the location of this file by running `config path` in nu.

   ```toml
   startup = [
    "mkdir ~/.cache/starship",
    "starship init nu | save ~/.cache/starship/init.nu",
    "source ~/.cache/starship/init.nu"
   ]
   prompt = "starship_prompt"
   ```


   #### Xonsh

   Add the following to the end of `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```
