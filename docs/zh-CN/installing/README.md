# 高级安装

安装 Starship 需要两步：

1. 下载 **Starship** 的可执行文件
1. 修改 shell 的初始化脚本，将 Starship 设置为命令提示符

大部分用户按照[主页](/guide/#🚀-installation)上的步骤安装即可， 但有一些特殊系统上的安装步骤不同。

现有平台众多，README 中无法全部展示，所以这里是社区中对其他平台 的一些安装说明。 找不到你的平台？ 如果你知道怎么安装，请添加到这里！

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

#### 声明式、单用户，使用 [home-manager](https://github.com/nix-community/home-manager)

请将以下内容添加到 `home.nix` 文件中以启用 `programs.starship` 模块

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

#### 声明式，全局，使用 NixOS

请将 `pkgs.starship` 添加至 `configuration.nix` 文件中的 `environment.systemPackages` 下，并运行

```sh
sudo nixos-rebuild switch
```
