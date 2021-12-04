# 🚀 Erweiterte Installation

Um Starship zu installieren, musst du zwei Dinge tun:

1. Holen Sie sich das **starship** Binary auf Ihren Computer
1. Sagen Sie ihrer Shell, dass sie das Starship-Binary als Prompt benutzt, indem sie die Initialisierungs-Skripte ändern

Die Anweisungen auf [der Hauptseite](/guide/#🚀-installation) werden für die meisten Benutzer gut funktionieren. Für einige speziellere Plattformen werden jedoch andere Anweisungen benötigt.

Es gibt so viele Plattformen, dass sie nicht in das Haupt-README passen, also hier einige Installationsanweisungen für andere Plattformen von der Community. Ist Ihre nicht hier? Bitte fügen Sie es hier hinzu, wenn Sie es herausfinden!

## [Chocolatey](https://chocolatey.org)

### Voraussetzungen

Gehen Sie zur [Chocolatey's Installations-Seite](https://chocolatey.org/install) und folge den Anweisungen um Chocolatey zu installieren.

### Installation

```powershell
choco install starship
```

## [termux](https://termux.com)

### Voraussetzungen

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

#### Imperativ

```sh
nix-env -iA nixos.starship
```

#### Deklarativ, Einzel-Benutzer, über [home-manager](https://github.com/nix-community/home-manager)

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

#### Deklarativ, systemweit, mit NixOS

Add `pkgs.starship` to `environment.systemPackages` in your `configuration.nix`, then run

```sh
sudo nixos-rebuild switch
```
