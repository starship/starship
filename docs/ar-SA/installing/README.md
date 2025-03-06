# Advanced Installation

لثتبيت starship تحتاج للقيام بشيئين هما:

1. ثبت ملفات **starship** على جهازك
1. تنبيه موجه الأوامر بإن يقوم بجعل سطر الأوامر ل starship وذلك بتعديل كود الإبتداء

For most users, the instructions on [the main page](../guide/#🚀-installation) will work great. لكن، من أجل الاستخدام المتقدم، هناك حاجة لتوجيهات أخرى.

هناك العديد من الحالات التي لا تلبي المعلومات في ملف README.md احتياجها ولذلك هذه بعض إرشادات التثبيت الإضافية مقدمة من مجتمع starship. إذا كانت لديك ملاحظة وقمت بحلها ولم تجد هذا الحل لها ضمن الحلول التالية، الرجاء أضفها هنا!

## [Chocolatey](https://chocolatey.org)

### المتطلبات الأساسية

إذهب إلى  [ صفحة تثبيت Chocolatey   ](https://chocolatey.org/install)  و اتبع الإرشادات لتثبيت البرنامج.

### التثبيت

```powershell
choco install starship
```

## [termux](https://termux.com)

### المتطلبات الأساسية

```sh
pkg install getconf
```

### التثبيت

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### التثبيت

يمكن تثبيت starship في Funtoo linux باستخدام  [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) via Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### احصل على ملفات الباينري

#### بشكل مباشر

```sh
nix-env -iA nixos.starship
```

#### بشكل تصريحي، من أجل مستخدم واحد، عبر [home-manager](https://github.com/nix-community/home-manager)

مكن كود`programs.starship` في  ملف`home.nix` و أضف إلى الإعدادات الإعدادات التالية

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

ثم بعد ذلك شغل

```sh
home-manager switch
```

#### بشكل تصريحي، لعدة مستخدمين

أضف `pkgs.starship` إلى `environment.systemPackages` في  `configuration.nix`, بعد ذلك شغل

```sh
sudo nixos-rebuild switch
```
