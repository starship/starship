# Расширенная конфигурация

Хотя Starship - это универсальная оболочка, иногда вам нужно сделать больше, чем просто редактировать `starship.toml`, для того чтобы сделать определенные вещи. Эта страница описывает некоторые из дополнительных техник конфигурации, используемых в Starship.

::: warning

Конфигурации в этом разделе могут быть изменены в будущих выпусках Starship.

:::

## Пользовательские команды перед командной строкой и перед запуском Bash

Bash не имеет формальной среды preexec/precmd, как и большинство других оболочек. Из-за этого трудно предоставить полностью настраиваемые хуки в `bash`. Тем не менее, Starship дает вам ограниченную возможность вставить собственные функции в процедуру отображения подсказки:

- Чтобы запустить пользовательскую функцию прямо перед отображением подсказки, определите новую функцию и затем назначьте ей имя `starship_precmd_user_func`. Например, чтобы нарисовать ракету перед появлением подсказки, сделайте

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Тем не менее, вы **должны** поймать сигнал DEBUG *перед* инициализацией Starship! Starship может сохранить значение ловушки DEBUG, но если ловушка перезаписана после запуска Starship, некоторая функциональность сломается.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
eval $(starship init bash)
```

## Изменение заголовка окна

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish даже делает это по умолчанию. Starship не делает этого, но достаточно легко добавить эту функциональность к `bash` или `zsh`.

Сначала задайте функцию изменения заголовка окна (идентичную в bash и zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

Вы можете использовать переменные для настройки этого заголовка (`$USER`, `$HOSTNAME`, и `$PWD` являются популярными вариантами).

В `bash`, установите эту функцию как функцию precmd в Starship:

```bash
starship_precmd_user_func="set_win_title"
```

В `zsh`, добавьте это в массив `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zshrc`) to make it permanent.

Например, если вы хотите отобразить ваш текущий каталог в заголовке вкладки терминала, добавьте следующие строки в `~/. bashrc` или `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh.

### Пример

```toml
# ~/.config/starship.toml

# A minimal left prompt
format = """$character"""

# move the rest of the prompt to the right
right_format = """$all"""
```

Produces a prompt like the following:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```


## Строки стиля

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - Некоторые из стандартных цветов терминалов: `black`, `red`, `green`, `blue`, `gellow`, `purple`, `cyan`, `white`. Вы можете по желанию добавить префикс `bright-`, чтобы получить яркую версию (например, `bright-white`).
 - `#`, за которой следует шестизначное шестнадцатеричное число. Это определяет [шестнадцатеричный код цвета RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Число от 0 до 255. Это определяет [8-битный код цвета ANSI](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
