# 🚀 高级安装

安装Starship有以下两个步骤：

1. 下载 **starship** 的可执行文件
1. 修改 shell 的初始化脚本，让 starship 显示命令提示符

大部分用户按照[主页](/guide/#🚀-installation)上的步骤安装即可， 但有一些特殊系统上的安装步骤不同。

There are so many platforms out there that they didn't fit into the main README.md file, so here are some installation instructions for other platforms from the community. Is yours not here? Please do add it here if you figure it out!

## [Chocolatey](https://chocolatey.org)

### 前置要求

按照 [Chocolatey 安装页面](https://chocolatey.org/install)上的步骤安装 Chocolatey。

### 安装

```powershell
choco install starship
```

## [termux](https://termux.com)

### 前置要求

```sh
pkg install getconf
```

### 安装

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
