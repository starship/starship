# Instalación avanzada

Para instalar Starship, necesitas hacer dos cosas:

1. Consigue el binario de **Starship** en tu ordenador
2. Decirle a tu intérprete de comandos que use el binario de Starship como su prompt modificando sus guiones de inicio

Para la mayoría de los usuarios, las instrucciones en [la página principal](../guide/#🚀-installation) funcionarán genial. Sin embargo, para algunas plataformas más especializadas, se necesitan diferentes instrucciones.

Hay tantas plataformas ahí fuera que no cabían en el README.md principal, así que aquí están algunas instrucciones de instalación para otras plataformas de la comunidad. ¿No está usted aquí? ¡Por favor, añádelo aquí si lo encuentras!

## [Chocolatey](https://chocolatey.org)

### Prerequisitos

Dirígete a la página de instalación de [Chocolatey](https://chocolatey.org/install) y sigue las instrucciones para instalar Chocolatey.

### Instalación

```powershell
choco install starship
```

## [termux](https://termux.com)

### Instalación

```sh
pkg install starship
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Instalación

En Funtoo Linux, Starship puede instalarse desde [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) a través de Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### Obtener el binario

#### Imperativamente

```sh
nix-env -iA nixos.starship
```

#### Declarativo, usuario único, a través de [home-manager](https://github.com/nix-community/home-manager)

Activa el módulo `programs.starship` en tu archivo `home.nix` y añade tus ajustes

```nix
{
  programs.starship = {
    enable = true;
    # Configuración escrita en ~/.config/starship.toml
    settings = {
      # add_newline = false;

      # character = {
      #   success_symbol = "[➜](bold green)";
      #   error_symbol = "[➜](bold red)";
      # };

      # package.disabled = true;
    };
  };
}
```

luego ejecutar

```sh
interruptor home-manager
```

#### Declarativo, en todo el sistema, con NixOS

Añade `pkgs.starship` a `environment.systemPackages` en tu `configuration.nix`, luego ejecuta

```sh
sudo nixos-rebuild switch
```
