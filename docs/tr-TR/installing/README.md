# 🚀 Gelişmiş Kurulum

Starship'i kurmak için, 2 şeye ihtiyacınız var:

1. **Starship** binary dosyalarını bilgisayarınıza alın
1. Kabuğunuza Starship binary scriptini kullanmasını söyleyin

Çoğu kullanıcı için, [ana sayfadaki](/guide/#🚀-installation) talimatlar gayet iyi çalışacaktır. Fakat, bazı özel platformlar için, farklı talimatlara ihtiyaç vardır.

Birçok platform var ki ana sisteme uymuyorlar. README.md belgesi ile diğer platformlar için bazı kurulum talimatları. Seninki burada değil mi? Anlarsan lütfen buraya ekle!

## [Chocolatey ](https://chocolatey.org)

### Ön gereklilikler

[Chocolatey kurulum sayfasına](https://chocolatey.org/install) gidin ve Chocolatey'i yüklemek için talimatları izleyin.

### Kurulum

```powershell
choco install starship
```

## [termux](https://termux.com)

### Ön gereklilikler

```sh
pkg install getconf
```

### Kurulum

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Binary Alma

#### Zorunlu olarak

```sh
nix-env -iA nixos.starship
```

#### Açıklayıcı, tek kullanıcı, via [home-manager](https://github.com/nix-community/home-manager)

`home.nix` dosyanızda, `programs.starship` modülünü etkinleştirin, ayarlarınızı ekleyin

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

sonra çalıştırın

```sh
home-manager switch
```

#### NixOS ile sistem genelinde, açıklama

Add `pkgs.starship` to `environment.systemPackages` in your `configuration.nix`, then run

```sh
sudo nixos-rebuild switch
```
