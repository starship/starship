# 🚀 Instalación avanzada

Para instalar Starship, necesitas hacer dos cosas:

1. Consigue el binario de **Starship** en tu ordenador
1. Decirle a tu shell que use el binario de Starship como su prompt modificando sus guiones de inicio

Para la mayoría de los usuarios, las instrucciones en [la página principal](/guide/#🚀-installation) funcionarán genial. Sin embargo, para algunas plataformas más especializadas, se necesitan diferentes instrucciones.

Hay tantas plataformas ahí fuera que no cabían en el README.md principal, así que aquí están algunas instrucciones de instalación para otras plataformas de la comunidad. ¿No está usted aquí? ¡Por favor, añádelo aquí si lo encuentras!
## [termux](https://termux.com)
### Prerequisitos
```sh
pkg install getconf
```

### Installation
```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- -b /data/data/com.termux/files/usr/bin
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Getting the Binary

#### Imperativamente

```sh
nix-env -iA nixos.starship
```

#### Declarativo, usuario único, a través de [home-manager](home-manager)

Añade `pkgs.starship` a tu `home.packages` en tu archivo `home.nix` y luego ejecuta

```sh
interruptor home-manager
```

#### Declarativo, en todo el sistema, con NixOS

Añade `pkgs.starship` a `environment.packages` en tu `configuration.nix`, luego ejecuta

```sh
sudo nixos-rebuild switch
```

### Modifying Init Scripts

#### Con Nix y home-manager, usando zsh:

Añade lo siguiente a `programs.zsh.initExtra` en tu archivo `home.nix` y luego ejecutar

```sh
interruptor home-manager
```
