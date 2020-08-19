<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship - Prompt multi interfaz de línea de comandos" />
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
  <a href="https://discord.gg/8Jzqu3T"
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
  <a href="https://translate.starship.rs/project/starship-prompt/es"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Español" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/fr"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Francés" /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship con iTerm 2 y el tema Snazzy"
  width="50%"
  align="right" />


**El símbolo del sistema minimalista, ultrarápido e infinitamente customizable para cualquier intérprete de comandos!**


- **Rápido:** es rápido – _muy muy_ rápido! 🚀
- **Personalizable:** configura cada parte de tu intérprete de comandos.
- **Universal:** funciona en cualquier intérprete de comandos, en cualquier sistema operativo.
- **Inteligente:** muestra información relevante de un vistazo.
- **Repleto de funciones**: con soporte para tus herramientas favoritas.
- **Fácil:** rápido de instalar – empieza a usarlo en minutos.

<p align="center">
<a href="https://starship.rs/config/"><strong>Explora la documentación de Starship&nbsp;&nbsp;</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 Instalación

### Prerequisitos

- Una [Nerd Font](https://www.nerdfonts.com/) instalada y habilitada en tu terminal (por ejemplo, prueba [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)).

### Comenzar

1. Instala el binario de **starship**:


   #### Instalar la última versión


   ##### Desde un binario precontruido, con Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   ##### Desde el código fuente en [crates.io](https://crates.io/):

   ```sh
   cargo install starship
   ```


   #### Instalar vía un gestor de paquetes


   ##### Con [Homebew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### Con [Scoop](https://scoop.sh):

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


   #### PowerShell

   Añade el siguiente código al final de `~\Documentos\PowerShell\Microsoft.PowerShell_profile.ps1` (o `~/.config/powershell/Microsoft.PowerShell_profile.ps1` en *nix):

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Añade el siguiente código al final de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

## 🤝 Colaborando

¡Siempre estamos buscando por colaboradores de **cualquier nivel**! Si estas buscando una manera fácil de ayudar este proyecto, puedes intentar resolver un problema con la etiqueta "[good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue)".

Si quieres ayudar a colaborar a starship, por favor mira nuestra [Guía de Colaboradores](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Además, siéntete libre de entrar en nuestro [servidor de Discord](https://discord.gg/8Jzqu3T) y di "¡Hola!". 👋

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

Por favor, revisa estos proyectos que inspiraron la creación de starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - Una prompt ZSH para astronautas.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - robbyrussel, tema multi interfaz de línea de comandos escrito en JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - Una prompt con iconos, personalizable y multi interfaz de línea de comandos basada en PowerLine.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Icono de Starship">
</p>

## 📝 Licencia

Copyright © 2019-actualidad, [Creadores de Starship](https://github.com/starship/starship/graphs/contributors).<br /> Este proyecto está bajo una licencia [ISC](https://github.com/starship/starship/blob/master/LICENSE).
