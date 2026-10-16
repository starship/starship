# Installazione Avanzata

Per installare starship, è necessario fare due cose:

1. Ottieni il binario **starship** sul tuo computer
2. Indica alla tua shell di usare il binario starship come prompt modificando i suoi script in init

For most users, the instructions on [the main page](../guide/#🚀-installation) will work great. Tuttavia, per alcune piattaforme più specializzate, sono necessarie istruzioni diverse.

Ci sono così tante piattaforme là fuori che non sono tutte inserite nel README.md principale, così ecco alcune istruzioni di installazione per altre piattaforme della community. La tua non c'è? Per favore, aggiungilo qui se capisci che manca!

## [Chocolatey](https://chocolatey.org)

### Prerequisiti

Vai alla pagina di installazione [Chocolatey](https://chocolatey.org/install) e segui le istruzioni per installare Chocolatey.

### Installazione

```powershell
choco install starship
```

## [termux](https://termux.com)

### Installazione

```sh
pkg install starship
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Installazione

On Funtoo Linux, starship can be installed from [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) via Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### Ottenere il Binario

#### Imperativo

```sh
nix-env -iA nixos.starship
```

#### Dichiarativo, singolo utente, tramite [home-manager](https://github.com/nix-community/home-manager)

Abilita il modulo `programs.starship` nel tuo file `home.nix` e aggiungi le tue impostazioni

```nix
{
  programs.starship = {
    enable = true;
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

poi eseguire

```sh
home-manager switch
```

#### Dichiarativa, di sistema, con NixOS

Aggiungi `pkgs.starship` a `environment.systemPackages` in `configuration.nix`, poi esegui

```sh
sudo nixos-rebuild switch
```
