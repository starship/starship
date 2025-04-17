# Продвинутая установка

Чтобы установить Starship, нужно выполнить две вещи:

1. Получить бинарный файл **starship** на вашем компьютере
1. Попросите оболочку использовать бинарный файл starship в качестве его промпта, изменив его сценарии инициализации

Для большинства пользователей отлично подходят инструкции на [главной странице](../guide/#🚀-installation). Тем не менее, для некоторых специализированных платформ, нужны различные инструкции.

Есть так много платформ, что они не вписываются в главный README.md файл, поэтому здесь есть некоторые инструкции по установке для других платформ от сообщества. Здесь нет вашей? Пожалуйста, добавьте её сюда, если сможешь разобраться в этом!

## [Chocolatey](https://chocolatey.org)

### Обязательные требования

Зайдите на [страницу установки Chocolatey](https://chocolatey.org/install) и следуйте инструкциям по установке Chocolatey.

### Установка

```powershell
choco install starship
```

## [termux](https://termux.com)

### Обязательные требования

```sh
pkg install getconf
```

### Установка

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Установка

На Funtoo Linux, starship может быть установлен из <0>core-kit</0> с помощью Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### Получение бинарного файла

#### Императивно

```sh
nix-env -iA nixos.starship
```

#### Декларативно, для одного пользователя, используя [home-manager](https://github.com/nix-community/home-manager)

Включите модуль <0>programs.starship</0> в ваш файл <0>home.nix</0> и добавьте ваши настройки

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

затем запустите

```sh
home-manager switch
```

#### Декларативно, системно, с NixOS

Добавьте `pkgs.starship` в `environment.systemPackages` в вашем `configuration.nix`, затем запустите

```sh
sudo nixos-rebuild switch
```
