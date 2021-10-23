# 🚀 高度なインストール

Starship をインストールするには、以下の2つのことを行う必要があります。

1. ** starship ** のバイナリをコンピューター上に取得する
1. Init スクリプトを修正することで、Starship バイナリをプロンプトとして使用するようシェルに指示する

ほとんどのユーザーの場合、[メインページ](/guide/#🚀-installation)の説明でうまく動作します。 しかし、より特殊なプラットフォームでは、別の操作が必要になることがあります。

プラットフォームは無数に存在するため、メインの README.md ファイルには書ききれません。そこで、このページでは、コミュニティにより提供された他のプラットフォーム向けのインストール方法について紹介します。 あなたが使用しているプラットフォームが見当たりませんか？ その場合は是非、見つけた方法をここに追加してください。

## [Chocolatey](https://chocolatey.org)

### 必要なもの

[Chocolatey インストールページ](https://chocolatey.org/install)にアクセスし、Chocolatey のインストール手順に従ってください。

### インストール

```powershell
choco install starship
```

## [termux](https://termux.com)

### 必要なもの

```sh
pkg install getconf
```

### インストール

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Nix](https://nixos.wiki/wiki/Nix)

### バイナリの取得

#### Imperatively

```sh
nix-env -iA nixos.starship
```

#### Declarative, single user, via [home-manager](https://github.com/nix-community/home-manager)

`home.nix`ファイルで`programs.starship`を有効にして、設定を追加してください。

```nix
{
  programs.starship = {
    enable = true;
    enableZshIntegration = true;
    # ~/.config/starship.toml に書き込まれる設定
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

そして、次を実行してください

```sh
home-manager switch
```

#### Declarative, system-wide, with NixOS

`configuration.nix`で`environment.systemPackages`に`pkgs.starship`を追加して、次を実行してください

```sh
sudo nixos-rebuild switch
```
