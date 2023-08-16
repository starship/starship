# 進階安裝

要安裝 starship，你必須做兩件事：

1. 下載 **starship** 執行檔到你的電腦裡
1. 修改 shell 的初始化腳本，讓其使用 starship 作為提示字元

對大部分的使用者來說，在[主頁面](/guide/#🚀-installation)的安裝指引皆足以正常運作。 然而，對於一些較特別的平台，我們需要不同的安裝指引。

有太多平台不適用於主要的 README.md 檔案了，所以這裡有一些來自社群的其他平台的安裝指引。 你的平台不在這裡嗎？ 如果你找出怎麼安裝，請幫我們加上你的指引！

## [Chocolatey](https://chocolatey.org)

### 先決要求

前往[Chocolatey 安裝頁面](https://chocolatey.org/install)，並且跟隨指引來安裝 Chocolatey。

### 安裝

```powershell
choco install starship
```

## [termux](https://termux.com)

### 先決要求

```sh
pkg install getconf
```

### 安裝

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### 安裝

On Funtoo Linux, starship can be installed from [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) via Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://nixos.wiki/wiki/Nix)

### 獲得執行檔

#### 命令式

```sh
nix-env -iA nixos.starship
```

#### 聲明式、單個使用者，使用 [home-manager](https://github.com/nix-community/home-manager)

在 `home.nix` 檔案中，啟用 `programs.starship` 模組，並且添加你的設定：

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

然後執行

```sh
home-manager switch
```

#### 聲明式、全系統，使用 NixOS

將 `pkgs.starship` 添加至 `configuration.nix` 檔案內的 `environment.systemPackages` 下，並且執行

```sh
sudo nixos-rebuild switch
```
