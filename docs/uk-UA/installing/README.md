# Розширене встановлення

Щоб установити starship, Ви мусите зробити дві речі:

1. Завантажити бінарний файл **starship** на Ваш комп'ютер
1. Сказати вашій консолі використовувати бінарний файл starship у якості командного рядка, змінивши його скрипти ініціалізації

Для більшості користувачів чудово підійдуть інструкції на [головній сторінці](/guide/#🚀-installation). Однак, для деяких більш спеціалізованих платформ потрібні інші інструкції.

Існує так багато платформ, що вони не вписуються в основний файл README.md, тож ось деякі інструкції зі встановлення від спільноти для інших платформ. Вашої тут немає? Будь ласка, додайте її сюди, якщо дізнаєтесь!

## [Chocolatey](https://chocolatey.org)

### Передумови

Перейдіть на [сторінку встановлення Chocolatey](https://chocolatey.org/install) та дотримуйтесь інструкцій, щоб установити Chocolatey.

### Установлення

```powershell
choco install starship
```

## [termux](https://termux.com)

### Передумови

```sh
pkg install getconf
```

### Установлення

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Установлення

У Funtoo Linux starship можна встановити з [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) через Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Завантаження бінарного файлу

#### Імперативно

```sh
nix-env -iA nixos.starship
```

#### Декларативно, для одного користувача, через [home-manager](https://github.com/nix-community/home-manager)

Увімкніть модуль `programs.starship` у Вашому файлі `home.nix` та додайте ваші налаштування

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

потім виконайте

```sh
home-manager switch
```

#### Декларативно, для цілої системи, з NixOS

Додайте `pkgs.starship` до `environment.systemPackages` у Вашій `configuration.nix`, після чого виконайте

```sh
sudo nixos-rebuild switch
```
