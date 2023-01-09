# 高级安装

安装Starship有以下两个步骤：

1. 下载 **starship** 的可执行文件
1. 修改 shell 的初始化脚本，让 starship 显示命令提示符

大部分用户按照[主页](/guide/#🚀-installation)上的步骤安装即可， 但有一些特殊系统上的安装步骤不同。

现有平台众多，README 中无法全部展示，所以这里是社区中对其他平台 的一些安装说明。 还没找到您的平台？ 如果您找到它 ，请在这里添加它！

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
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### 安装

在 Funtom Linux 上，可以通过 Portact 从 [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) 安装启动：

```sh
emerge app-shells/starship
```

## [Nix](https://nixos.wiki/wiki/Nix)

### 获取二进制文件

#### 命令

```sh
nix-env -iA nixos.starship
```

#### Declarative, single user, via [home-manager](https://github.com/nix-community/home-manager)

在您的 `home.nix` 文件中启用 `programs.starship` 模块，并添加您的设置

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

然后运行

```sh
home-manager switch
```

#### Declarative, system-wide, with NixOS

在您的 `configuration.nix`中，将 `pkgs.starship` 添加到 `environment.systemPackages`，然后运行

```sh
sudo nixos-rebuild switch
```
