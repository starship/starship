# 🚀 Instalación avanzada

Para instalar Starship, necesitas hacer dos cosas:

1. Consigue el binario de **Starship** en tu ordenador
1. Decirle a tu shell que use el binario de Starship como su prompt modificando sus guiones de inicio

Para la mayoría de los usuarios, las instrucciones en [la página principal](/guide/#🚀-installation) funcionarán genial. Sin embargo, para algunas plataformas más especializadas, se necesitan diferentes instrucciones.

Hay tantas plataformas ahí fuera que no cabían en el README.md principal, así que aquí están algunas instrucciones de instalación para otras plataformas de la comunidad. ¿No está usted aquí? ¡Por favor, añádelo aquí si lo encuentras!

## [Chocolatey](https://chocolatey.org)

### Prerequisitos

Head over to the [Chocolatey installation page](https://chocolatey.org/install) and follow the instructions to install Chocolatey.

### Installation

```powershell
choco install starship
```

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

Enable the `programs.starship` module in your `home.nix` file, and add your settings

```nix
{
  programs.starship = {
    enable = true;
    enableZshIntegration = true;
    # Configuration written to ~/.config/starship.toml
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

then run

```sh
home-manager switch
```

#### Declarativo, en todo el sistema, con NixOS

Add `pkgs.starship` to `environment.systemPackages` in your `configuration.nix`, then run

```sh
sudo nixos-rebuild switch
```
