# Розширене встановлення

Щоб встановити starship, Ви мусите зробити дві речі:

1. Завантажити бінарний файл **starship** на Ваш компʼютер
2. Вказати у вашій оболонці, що ви бажаєте використовувати бінарний файл starship у вигляді командного рядка, змінивши скрипти ініціалізації

For most users, the instructions on [the main page](../guide/#🚀-installation) will work great. Однак,
для деяких більш спеціалізованих платформ потрібні інші інструкції.

Існує так багато платформ, що вони не вписуються в основний файл README.md, тож ось деякі інструкції зі встановлення від спільноти для інших платформ. Вашої тут немає? Будь ласка, додайте її сюди, якщо дізнаєтесь!

## [Chocolatey](https://chocolatey.org)

### Вимоги

Перейдіть на [сторінку встановлення Chocolatey](https://chocolatey.org/install) та дотримуйтесь інструкцій, щоб установити Chocolatey.

### Встановлення

```powershell
choco install starship
```

## [termux](https://termux.com)

### Встановлення

```sh
pkg install starship
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Встановлення

У Funtoo Linux starship можна встановити з [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) через Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### Завантаження бінарного файлу

#### Примусово

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

Додайте `pkgs.starship` до `environment.systemPackages` у налаштування `configuration.nix`, після чого виконайте

```sh
sudo nixos-rebuild switch
```
