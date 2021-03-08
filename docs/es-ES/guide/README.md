<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship - Prompt multi intérprete de comandos" />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/starship/starship/Main workflow/master?label=workflow&style=flat-square"
      alt="Estado del flujo de trabajo de GitHub Actions" /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Versión de Crates.io" /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Estado de empaquetado" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Chat en Discord" /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Sigue a @StarshipPrompt en Twitter" /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Sitio Web</a>
  ·
<a href="#🚀-installation">Instalación</a>
  ·
<a href="https://starship.rs/config/">Configuración</a>
</p>

<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png"
      alt="English" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ja-JP/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-jp.png"
      alt="日本語" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="繁體中文" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png"
      alt="Русский" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Deutsch" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-CN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-cn.png"
      alt="简体中文" /></a>
  &nbsp;
  <a 
    href="https://github.com/starship/starship/blob/master/docs/es-ES/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Español" /></a>
  &nbsp;
  <a 
    href="https://github.com/starship/starship/blob/master/docs/fr-FR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Francés" /></a>
  &nbsp;
  <a 
    href="https://github.com/starship/starship/blob/master/docs/vi-VN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-vn.png"
      alt="Tiếng Việt" /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship con iTerm 2 y el tema Snazzy"
  width="50%"
  align="right" />

**¡El prompt minimalista, ultrarápido e infinitamente personalizable para cualquier intérprete de comandos!**

- **Rápido:** es rápido – _realmente_ rápido! 🚀
- **Personalizable:** configura cada parte de tu prompt.
- **Universal:** funciona en cualquier intérprete de comandos, en cualquier sistema operativo.
- **Inteligente:** muestra información relevante de un vistazo.
- **Repleto de funciones**: con soporte para tus herramientas favoritas.
- **Fácil:** rápido de instalar – empieza a usarlo en minutos.

<p align="center">
<a href="https://starship.rs/config/"><strong>Explora la documentación de Starship&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 Instalación

### Prerequisitos

- Una [Nerd Font](https://www.nerdfonts.com/) instalada y habilitada en tu terminal (por ejemplo, prueba [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)).

### Comenzar

**Nota**: debido a la proliferación de diferentes plataformas, solo un subconjunto de plataformas soportadas se muestra a continuación. ¿No puedes ver el tuyo? Echa un vistazo a las [instrucciones adicionales de la plataforma](https://starship.rs/installing/).

1. Instala el binario de **Starship**:


   #### Instalar la última versión


   ##### Desde un binario preconstruido, con el intérprete de comandos:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Instalar con un gestor de paquetes


   ##### Ejemplo: [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### Con [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

2. Añade el script de inicio al archivo de configuración de tu interfaz de línea de comandos:


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


   #### PowerShell

   Añade lo siguiente al final de `Microsoft.PowerShell_profile.ps1`. Puedes comprobar la ubicación de este archivo consultando la variable `$PROFILE` en PowerShell. Normalmente la ruta es `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` o `~/.config/powershell/Microsoft.PowerShell_profile.ps1` en -Nix.

   ```powershell
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Añade el siguiente código al final de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   **Advertencia** Sólo se admite Elvish v0.15 o superior. Añade el siguiente código al final de `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```

## 🤝 Colaborando

¡Siempre estamos buscando colaboradores de **todos los niveles y habilidades**! Si estas buscando una manera fácil de ayudar este proyecto, puedes intentar resolver un problema con la etiqueta "[good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue)".

Si hablas con fluidez en un idioma que no sea inglés, agradecemos mucho cualquier ayuda para mantener nuestros documentos traducidos y actualizados en otros idiomas. Si quieres ayudar, puedes contribuir con las traducciones en el [Crowdin de Starship](https://translate.starship.rs/).

Si quieres ayudar a colaborar a Starship, por favor mira nuestra [Guía de Colaboradores](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Además, siéntete libre de entrar en nuestro [servidor de Discord](https://discord.gg/8Jzqu3T) y di "¡Hola!". 👋

### Desarrolladores

Este proyecto existe gracias a todas las personas que han ayudado. [[Contribuir](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)].
<a href="https://github.com/starship/starship/graphs/contributors"><img src="https://opencollective.com/starship/contributors.svg?width=890&button=false" /></a>

### Financiadores

Conviértete en un financiador y ayúdanos a mantener nuestra comunidad. [[Contribuir](https://opencollective.com/starship/contribute)]

#### Personas

<a href="https://opencollective.com/starship"><img src="https://opencollective.com/starship/individuals.svg?width=890"></a>

#### Organizaciones

Apoya este proyecto con tu organización. Su logo se mostrará aquí con un enlace a su sitio web. [[Contribuir](https://opencollective.com/starship/contribute)]

<a href="https://opencollective.com/starship/organization/0/website"><img src="https://opencollective.com/starship/organization/0/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/1/website"><img src="https://opencollective.com/starship/organization/1/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/2/website"><img src="https://opencollective.com/starship/organization/2/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/3/website"><img src="https://opencollective.com/starship/organization/3/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/4/website"><img src="https://opencollective.com/starship/organization/4/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/5/website"><img src="https://opencollective.com/starship/organization/5/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/6/website"><img src="https://opencollective.com/starship/organization/6/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/7/website"><img src="https://opencollective.com/starship/organization/7/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/8/website"><img src="https://opencollective.com/starship/organization/8/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/9/website"><img src="https://opencollective.com/starship/organization/9/avatar.svg"></a>

## 💭 Inspirado por

Por favor, revisa estos trabajos previos que ayudaron a inspirar la creación de starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - Una prompt ZSH para astronautas.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - robbyrussel, tema multi intérprete de comandos escrito en JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - Una prompt multi intérprete de comandos personalizable, basada en Powerline con iconos.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Icono de cohete de Starship">
</p>

## 📝 Licencia

Derechos de autor © 2019-presente, [Colaboradores de Starship](https://github.com/starship/starship/graphs/contributors).<br />Este proyecto está bajo una licencia [ISC](https://github.com/starship/starship/blob/master/LICENSE).
