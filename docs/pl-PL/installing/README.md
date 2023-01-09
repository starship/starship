# Zaawansowana Instalacja

Aby zainstalować starship, musisz zrobić dwie rzeczy:

1. Pobrać plik wykonywalny **starship** na swój komputer
1. Skonfigurować swoją powłokę żeby używała pliku wykonywalnego starship jako wiersza poleceń poprzez modyfikację skryptów inicjalizacyjnych

Dla większości użytkowników instrukcje na [stronie głównej](/guide/#🚀-installation) są wystarczające. Jednakże, dla niektórych wyspecjalizowanych platform, potrzebne będą inne.

Istnieje tak wiele platform, że nie zmieściły się one w głównym pliku README.md, więc poniżej znajdują się instrukcje instalacji dla innych platform od społeczności. Nie ma tutaj Twojej? Jeśli się o tym dowiesz, dodaj je tutaj!

## [Chocolatey](https://chocolatey.org)

### Wymagania wstępne

Przejdź na stronę [Instalacja Chocolatey](https://chocolatey.org/install) i postępuj zgodnie z instrukcjami, aby zainstalować Chocolatey.

### Instalacja

```powershell
choco install starship
```

## [termux](https://termux.com)

### Wymagania wstępne

```sh
pkg install getconf
```

### Instalacja

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Instalacja

Na Funtoo Linux, starship może być zainstalowany przez Portage z [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship):

```sh
emerge app-shells/starship
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Getting the Binary

#### Imperatively

```sh
nix-env -iA nixos.starship
```

#### Declarative, single user, via [home-manager](https://github.com/nix-community/home-manager)

Enable the `programs.starship` module in your `home.nix` file, and add your settings

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

then run

```sh
home-manager switch
```

#### Declarative, system-wide, with NixOS

Add `pkgs.starship` to `environment.systemPackages` in your `configuration.nix`, then run

```sh
sudo nixos-rebuild switch
```
