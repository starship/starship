# Instalasi Lanjutan

Untuk memasang starship, ada dua hal yang perlu anda lakukan:

1. Dapatkan binary **starship** di komputer anda
1. Atur shell anda untuk menggunakan binary tersebut sebagai prompt dengan mengedit init scripts nya

Untuk kebayakan pengguna, instruksi pada [halaman utama](../guide/#🚀-installation) akan berjalan lancar. Namun, untuk sebagian platfrom khusus, instruksi khusus diperlukan.

There are so many platforms out there that they didn't fit into the main README.md file, so here are some installation instructions for other platforms from the community. Is yours not here? Please do add it here if you figure it out!

## [Chocolatey](https://chocolatey.org)

### Prasyarat

Head over to the [Chocolatey installation page](https://chocolatey.org/install) and follow the instructions to install Chocolatey.

### Pemasangan

```powershell
choco install starship
```

## [termux](https://termux.com)

### Prasyarat

```sh
pkg install getconf
```

### Pemasangan

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Pemasangan

On Funtoo Linux, starship can be installed from [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) via Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

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
