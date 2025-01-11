# Расширенная конфигурация

Хотя Starship - это универсальная оболочка, иногда вам нужно сделать больше, чем просто редактировать `starship.toml`, для того чтобы сделать определенные вещи. Эта страница описывает некоторые из дополнительных техник конфигурации, используемых в Starship.

::: warning

Конфигурации в этом разделе могут быть изменены в будущих выпусках Starship.

:::

## TransientPrompt для PowerShell

Можно заменить предыдущий выведенный промпт на пользовательскую строку. Это полезно в тех случаях, когда весь промпт не всегда нужен. Чтобы включить, запустите `Enable-TransientPrompt` в сеансе оболочки. Чтобы включить его глобально, вставьте это в ваш `$PROFILE` Переход может быть отключён на лету с помощью `Disable-TransientPrompt`.

По умолчанию, левая сторона ввода заменяется на `>`. Для настройки определите новую функцию под названием `Invoke-Starship-TransientFunction`. Например, чтобы отобразить модуль Starship `character`, вы должны выполнить

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt и TransientRightPrompt в Cmd

Clink позволяет заменить предыдущий выведенный промпт на пользовательские строки. Это полезно в тех случаях, когда весь промпт не всегда нужен. Чтобы включить это, запустите `clink set prompt.transient <value>`, где \<value\> может быть одним из

- `always`: всегда заменять предыдущий промпт
- `same_dir`: заменять предыдущий промпт только если рабочая директория та же
- `off`: не заменять промпт (т.е. выключить переход)

Вы должны сделать это только один раз. Внесите следующие изменения в ваш `starship.lua`, чтобы настроить, что отображается слева и справа:

- По умолчанию, левая сторона ввода заменяется на `>`. Чтобы настроить это, определите новую функцию под названием `starship_transient_prompt_func`. Эта функция получает текущий промпт как строку, которую вы можете использовать. Например, чтобы отобразить здесь модуль Starship `character`, вы должны выполнить

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- По умолчанию, правая сторона ввода пуста. Для настройки определите новую функцию под названием `starship_transient_rprompt_func`. Эта функция получает текущий промпт как строку, которую вы можете использовать. Например, чтобы отобразить время, когда здесь была запущена последняя команда, вы должны выполнить

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt и TransientRightPrompt в Fish

Можно заменить предыдущий выведенный промпт на пользовательскую строку. Это полезно в тех случаях, когда весь промпт не всегда нужен. Чтобы включить это, запустите `enable_transience` в сеансе оболочки. Чтобы включить его глобально, добавьте это в ваш `~/.config/fish/config.fish`. Переход может быть отключён на лету с помощью `disable_transience`.

Обратите внимание, что в случае с Fish, временный промпт печатается только в том случае, если командная строка не пустая и синтаксически верна.

- По умолчанию, левая сторона ввода заменяется жирным зеленым `❯`. Для настройки определите новую функцию под названием `starship_transient_prompt_func`. Например, чтобы отобразить здесь модуль Starship `character`, вы должны выполнить

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- По умолчанию, правая сторона ввода пуста. Для настройки определите новую функцию под названием `starship_transient_rprompt_func`. Например, чтобы отобразить время, когда здесь была запущена последняя команда, вы должны выполнить

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt и TransientRightPrompt в Bash

Фреймворк [Ble.sh](https://github.com/akinomyoga/ble.sh) в версии 0.4 или выше позволяет заменить выведенный ранее промпт, пользовательскими строками. Это полезно в тех случаях, когда весть промпт не всегда нужен. Чтобы включить это, добавьте в `~/.bashrc` `bleopt prompt_ps1_transient=<value>`:

` здесь - это разделённый двоеточиями список, состоящий из  <value\>always</1>, <1>same-dir</1> и <1>trim</1>.
Когда <value\>prompt_ps1_final</0> пуст, а опция <0>prompt_ps1_transient</0> - не пустой \<code>, промпт, указанный <0>PS1</0> удаляется при выходе из текущей командной строки.
Если \<value\> содержит поле <code>trim`, сохраняется только последняя строка многострочного `PS1`, а остальные линии стираются. Иначе командная строка будет перерисована, будто `PS1=` установлено. When a field `same-dir` is contained in \<value\> and the current working directory is different from the final directory of the previous command line, this option `prompt_ps1_transient` is ignored.

Make the following changes to your `~/.blerc` (or in `~/.config/blesh/init.sh`) to customize what gets displayed on the left and on the right:

- To customize what the left side of input gets replaced with, configure the `prompt_ps1_final` Ble.sh option. For example, to display Starship's `character` module here, you would do

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- To customize what the right side of input gets replaced with, configure the `prompt_rps1_final` Ble.sh option. Например, чтобы отобразить время, когда здесь была запущена последняя команда, вы должны выполнить

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Custom pre-prompt and pre-execution Commands in Cmd

Clink provides extremely flexible APIs to run pre-prompt and pre-exec commands in Cmd shell. It is fairly simple to use with Starship. Make the following changes to your `starship.lua` file as per your requirements:

- To run a custom function right before the prompt is drawn, define a new function called `starship_preprompt_user_func`. This function receives the current prompt as a string that you can utilize. For example, to draw a rocket before the prompt, you would do

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- To run a custom function right before a command is executed, define a new function called `starship_precmd_user_func`. This function receives the current commandline as a string that you can utilize. For example, to print the command that's about to be executed, you would do

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Пользовательские команды перед командной строкой и перед запуском Bash

Bash не имеет формальной среды preexec/precmd, как и большинство других оболочек. Из-за этого трудно предоставить полностью настраиваемые хуки в `bash`. Тем не менее, Starship дает вам ограниченную возможность вставить собственные функции в процедуру отображения подсказки:

- Чтобы запустить пользовательскую функцию прямо перед отображением подсказки, определите новую функцию и затем назначьте ей имя `starship_precmd_user_func`. Например, чтобы нарисовать ракету перед появлением подсказки, сделайте

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Тем не менее, вы **должны** поймать сигнал DEBUG _перед_ инициализацией Starship! Starship может сохранить значение ловушки DEBUG, но если ловушка перезаписана после запуска Starship, некоторая функциональность сломается.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Custom pre-prompt and pre-execution Commands in PowerShell

PowerShell does not have a formal preexec/precmd framework like most other shells. Из-за этого трудно предоставить полностью настраиваемые хуки в `powershell`. Тем не менее, Starship дает вам ограниченную возможность вставить собственные функции в процедуру отображения подсказки:

Create a function named `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Изменение заголовка окна

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish даже делает это по умолчанию. Starship does not do this, but it's fairly straightforward to add this functionality to `bash`, `zsh`, `cmd` or `powershell`.

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

For Cmd, you can change the window title using the `starship_preprompt_user_func` function.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

You can also set a similar output with PowerShell by creating a function named `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [`fill` module](../config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Note: The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework v0.4 or higher should be installed in order to use right prompt in bash.

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

## Continuation Prompt

Some shells support a continuation prompt along with the normal prompt. This prompt is rendered instead of the normal prompt when the user has entered an incomplete statement (such as a single left parenthesis or quote).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `'[∙](bright-black) '`.

Примечание: `continuation_prompt` должно быть изменено на строку без каких-либо переменных.

Примечание: Дальнейшие подсказки доступны только в следующих оболочках:

- `bash`
- `zsh`
- `PowerShell`

### Пример

```toml
# ~/.config/starship.toml

# Запрос на продолжение, в котором отображаются две заполненные стрелки
continuation_prompt = '▶️▶️ '
```

## Строки стиля

Строки стиля - это список слов, разделенных пробелами. Слова не чувствительны к регистру (то есть `bold` и `BoLd` считаются одной строкой). Каждое слово может быть одним из следующих:

- `bold`
- `italic`
- `underline`
- `dimmed`
- `инвертировано`
- `мерцать`
- `скрытый`
- `зачеркнуто`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

где `<color>` является цветовым спецификатором (обсуждается ниже). `fg:<color>` и `<color>` в настоящее время выполняет то же самое, хотя в будущем это может измениться. `<color>` также может быть установлено значение `prev_fg` или `prev_bg`, которое соответствует цвету переднего плана или фона предыдущего элемента соответственно, если доступно, или `нет` в противном случае. `инвертировано` меняет местами цвета фона и переднего плана. Порядок слов в строке не имеет значения.

Токен `none` переопределяет все остальные токены в строке, если он не является частью спецификатора `bg:` так, например, `fg:red none fg:blue` все равно создаст строку без стиля. `bg:none` устанавливает цвет по умолчанию на цвет `fg:red bg:none` эквивалентно `red` или `fg:red` и `bg:green fg:red bg:none` также эквивалентно `fg:red` или `red`. Использование `none` в сочетании с другими токенами может стать ошибкой в будущем.

Цветовой спецификатор может быть одним из следующих:

- Один из стандартных цветов терминалов: `black`, `red`, `green`, `blue`, `gellow`, `purple`, `cyan`, `white`. При желании вы можете добавить к ним префикс `bright-`, чтобы получить версию bright (например, `bright-white`).
- `#`, за которой следует шестизначное шестнадцатеричное число. Это определяет [шестнадцатеричный код цвета RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Число от 0 до 255. Это определяет [8-битный код цвета ANSI](https://i.stack.imgur.com/KTSQa.png).

Если для переднего плана/фона задано несколько цветов, то последняя из строк будет иметь приоритет.

Не все строки стиля будут корректно отображаться в терминале. В частности, существуют следующие известные ошибки:

- Во многих терминалах по умолчанию отключена поддержка `blink`.
- `hidden` [не поддерживается в iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` по умолчанию не поддерживается в macOS Terminal.app.
