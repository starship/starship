# 🚀 Installazione Avanzata

Per installare starship, è necessario fare due cose:

1. Ottieni il binario **starship** sul tuo computer
1. Indica alla tua shell di usare il binario starship come prompt modificando i suoi script in init

Per la maggior parte degli utenti, le istruzioni nella [pagina principale](/guide/#🚀-installation) funzioneranno bene. Tuttavia, per alcune piattaforme più specializzate, sono necessarie istruzioni diverse.

Ci sono così tante piattaforme là fuori che non sono tutte inserite nel README.md principale, così ecco alcune istruzioni di installazione per altre piattaforme della community. La tua non c'è? Per favore, aggiungilo qui se capisci che manca!

## [Chocolatey](https://chocolatey.org)

### Prerequisiti

Vai alla pagina di installazione [Chocolatey](https://chocolatey.org/install) e segui le istruzioni per installare Chocolatey.

### Installazione

```powershell
choco install starship
```

## [termux](https://termux.com)

### Prerequisiti

```sh
pkg install getconf
```

### Installazione

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Installazione

On Funtoo Linux, starship can be installed from [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) via Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Getting the Binary

#### Imperativo

```sh
nix-env -iA nixos.starship
```

#### Dichiarativo, singolo utente, tramite [home-manager](https://github.com/nix-community/home-manager)

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

#### Dichiarativa, di sistema, con NixOS

Add `pkgs.starship` to `environment.systemPackages` in your `configuration.nix`, then run

```sh
sudo nixos-rebuild switch
```
