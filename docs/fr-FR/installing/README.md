# 🚀 Installation avancée

Pour installer starship, vous devez faire deux choses:

1. Installez le binaire **starship** sur votre ordinateur
1. Dites à votre shell d'utiliser le binaire de starship comme invite en modifiant ses scripts d'initialisation

Pour la plupart des utilisateurs, les instructions sur [la page principale](/guide/#🚀-installation) fonctionneront bien. Cependant, pour certaines plateformes plus spécialisées, des instructions différentes sont nécessaires.

Il y a tellement de plates-formes, qu'il aurait été déraisonnable de les faire apparaître dans le README principal, voici donc quelques instructions d'installation supplémentaires pour celles-ci, écrient par la commaunauté. La vôtre n'est-elle pas là ? S'il vous plaît, ajoutez-la ici pour les suivants !

## [Nix](https://nixos.wiki/wiki/Nix)

### Getting the Binary

#### Imperatively

```sh
nix-env -iA nixos.starship
```

#### Declarative, single user, via [home-manager](home-manager)

Add `pkgs.starship` to your `home.packages` in your `home.nix` file, then run

```sh
home-manager switch
```

#### Declarative, system-wide, with NixOS

Add `pkgs.starship` to `environment.packages` in your `configuration.nix`, then run

```sh
sudo nixos-rebuild switch
```

### Modifying Init Scripts

#### With Nix and home-manager, using zsh:

Add the following to `programs.zsh.initExtra` in your `home.nix` file, then run

```sh
home-manager switch
```
