# 高度なインストール

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
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### インストール

Funtoo Linuxでは、Portageの[core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship)からstarship をインストールできます。

```sh
emerge app-shells/starship
```

## [Nix](https://nixos.wiki/wiki/Nix)

### バイナリの取得

#### 命令的な手法

```sh
nix-env -iA nixos.starship
```

#### 単一ユーザーで[home-manager](https://github.com/nix-community/home-manager)を使う宣言的な手法

`home.nix`ファイルで`programs.starship`を有効にして、設定を追加してください。

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

そして、次を実行してください

```sh
home-manager switch
```

#### NixOSでシステム全体に適用する宣言的な手法

`configuration.nix`で`environment.systemPackages`に`pkgs.starship`を追加して、次を実行してください

```sh
sudo nixos-rebuild switch
```
