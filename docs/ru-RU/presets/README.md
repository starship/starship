# Предустановки

Ниже приведена коллекция предустановок конфигураций сообщества для Starship. Если вы хотите поделиться своей предустановкой, пожалуйста, [создайте PR](https://github.com/starship/starship/edit/master/docs/presets/README.md), обновляя этот файл! 😊

## Символы Шрифта Nerd Font

Эта предустановка не меняет ничего кроме символов, используемых для каждого модуля. Если эмодзи вам не по душе, это может порадовать ваш глаз!

![Скриншот предустановки Nerd Font Symbols](/presets/nerd-font-symbols.png)

### Требования

- Установленный и включенный шрифт [Nerd Font](https://www.nerdfonts.com/) вашем терминале (пример использует Fira Code Nerd Font)

### Конфигурация

```toml
[aws]
symbol = " "

[battery]
full_symbol = ""
charging_symbol = ""
discharging_symbol = ""

[conda]
symbol = " "

[git_branch]
symbol = " "

[golang]
symbol = " "

[hg_branch]
symbol = " "

[java]
symbol = " "

[memory_usage]
symbol = " "

[nodejs]
symbol = " "

[package]
symbol = " "

[php]
symbol = " "

[python]
symbol = " "

[ruby]
symbol = " "

[rust]
symbol = " "
```
