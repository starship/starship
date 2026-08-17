# Кеңейтілген орнату

Starship-ті орнату үшін екі әрекетті орындау қажет:

1. Компьютеріңізге **starship** орындалатын файлын жүктеп орнату
1. Қабықшаңыздың инициализация скриптін өзгерту арқылы қабықшаға starship-ті промпт ретінде пайдалануды көрсету

Қолданушылардың көпшілігі үшін [басты нұсқаулық бетіндегі](../guide/#🚀-орнату) әрекеттер жеткілікті. Дегенмен, кейбір арнайы платформалар үшін басқаша орнату нұсқаулары қажет.

Платформалар саны өте көп болғандықтан, олардың барлығы негізгі README.md файлына сыймады, сондықтан мұнда қауымдастық дайындаған басқа платформаларға арналған орнату нұсқаулары жинақталған. Сіздің платформаңыз бұл тізімде жоқ па? Егер оны орнату жолын тапсаңыз, осында қосқаныңызға қуанамыз!

## [Chocolatey](https://chocolatey.org)

### Алғышарттар

[Chocolatey орнату бетіне](https://chocolatey.org/install) өтіп, Chocolatey орнату бойынша нұсқаулықты орындаңыз.

### Орнату

```powershell
choco install starship
```

## [termux](https://termux.com)

### Орнату

```sh
pkg install starship
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Орнату

Funtoo Linux жүйесінде starship-ті Portage арқылы [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) көзінен орнатуға болады:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### Орындалатын файлды алу

#### Императивті тәсілмен (Imperatively)

```sh
nix-env -iA nixos.starship
```

#### Декларативті, бір пайдаланушы үшін, [home-manager](https://github.com/nix-community/home-manager) арқылы

`home.nix` файлыңызда `programs.starship` модулін қосып, баптауларыңызды жазыңыз:

```nix
{
  programs.starship = {
    enable = true;
    # Баптаулар ~/.config/starship.toml файлына жазылады
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

содан кейін келесі пәрменді іске қосыңыз:

```sh
home-manager switch
```

#### Декларативті, бүкіл жүйе үшін, NixOS арқылы

`configuration.nix` файлыңыздағы `environment.systemPackages` тізіміне `pkgs.starship` қосыңыз,
содан соң орындаңыз:

```sh
sudo nixos-rebuild switch
```
