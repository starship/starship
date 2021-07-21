# 🚀 進階安裝

要安裝 starship，你必須做兩件事：

1. 下載 **starship** 執行檔到你的電腦裡
1. 修改 shell 的初始化腳本，讓其使用 starship 作為提示字元

對大部分的使用者來說，在[主頁面](/guide/#🚀-installation)的安裝指引皆足以正常運作。 然而，對於一些較特別的平台，我們需要不同的安裝指引。

有太多平台不適用於主要的 README.md 檔案了，所以這裡有一些來自社群的其他平台的安裝指引。 Is yours not here? Please do add it here if you figure it out!

## [Chocolatey](https://chocolatey.org)

### 先決要求

Head over to the [Chocolatey installation page](https://chocolatey.org/install) and follow the instructions to install Chocolatey.

### Installation

```powershell
choco install starship
```

## [termux](https://termux.com)

### 先決要求

```sh
pkg install getconf
```

### Installation

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --bin-dir /data/data/com.termux/files/usr/bin
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

#### Declarative, system-wide, with NixOS

Add `pkgs.starship` to `environment.systemPackages` in your `configuration.nix`, then run

```sh
sudo nixos-rebuild switch
```
