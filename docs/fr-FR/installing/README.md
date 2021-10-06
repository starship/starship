# 🚀 Installation avancée

Pour installer starship, vous devez faire deux choses:

1. Installez le binaire **starship** sur votre ordinateur
1. Dire à votre shell d'utiliser le binaire de starship comme invite en modifiant ses scripts d'initialisation

Pour la plupart des utilisateurs, les instructions sur [la page principale](/guide/#🚀-installation) fonctionneront bien. Cependant, pour certaines plateformes plus spécialisées, des instructions différentes sont nécessaires.

Il y a tellement de plates-formes, qu'il aurait été déraisonnable de les faire apparaître dans le README principal, voici donc quelques instructions d'installation supplémentaires pour celles-ci, écrites par la communauté. La vôtre n'est pas là ? S'il vous plaît, ajoutez-la ici pour les suivants !

## [Chocolatey](https://chocolatey.org)

### Pré-requis

Rendez-vous sur la [page d'installation de Chocolatey](https://chocolatey.org/install) et suivez leurs instructions pour installer Chocolatey.

### Installation

```powershell
choco install starship
```

## [termux](https://termux.com)

### Pré-requis

```sh
pkg install getconf
```

### Installation

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Installation

On Funtoo Linux, starship can be installed from [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) via Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Getting the Binary

#### Impératif

```sh
nix-env -iA nixos.starship
```

#### Déclaration, utilisateur unique, via [home-manager](https://github.com/nix-community/home-manager)

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

#### Déclaration, au niveau du système, avec NixOS

Add `pkgs.starship` to `environment.systemPackages` in your `configuration.nix`, then run

```sh
sudo nixos-rebuild switch
```
