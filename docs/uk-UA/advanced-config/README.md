# Розширені налаштування

Хоча Starship і універсальна оболонка, іноді необхідно зробити більше ніж просто змінити `star.toml`, щоб можна було робити певні речі. Ця сторінка містить деякі з найбільш докладних методів налаштувань, які використовуються у starship.

::: warning

Налаштування у цьому розділі можуть змінюватись у майбутніх релізах Starship.

:::

## TransientPrompt у PowerShell

Можна замінити попередній командний рядок на власний. Це корисно у випадках, коли вся інформація у ньому не завжди потрібна. Щоб увімкнути це, запустіть `Enable-TransientPrompt` в сеансі оболонки. Щоб зробити цю поведінку постійною, додайте цю команду у ваш `$PROFILE`. Перехідність можна вимкнути на льоту за допомогою `Disable-TransientPrompt`.

Типово, ліва частина вводу буде замінена на `>`. Щоб налаштувати це, створіть нову функцію з назвою `Invoke-Starship-TransientFunction`. Наприклад, щоб показати тут модуль Starship `character`, вам потрібно

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt та TransientRightPrompt в Cmd

Clink дозволяє замінювати попередньо надрукований командний рядок іншим рядком. Це корисно у випадках, коли вся інформація з командного рядка не потрібна. Щоб увімкнути це, виконайте `clink set prompt.transient <value>`, де \<value\> може бути одним з:

- `always`: завжди замінює попередній командний рядок
- `same_dir`: замінює попередній командний рядок тільки якщо робоча тека не змінювалась
- `off`: не змінює командний рядок (тобто функцію вимкнено)

Це треба зробити лише один раз. Зробіть наступні зміни у `starship.lua`, щоб налаштувати, що показується ліворуч і праворуч:

- Типово, ліва частина вводу буде замінена на `>`. Щоб налаштувати це, створіть нову функцію з назвою `starship_transient_prompt_func`. Ця функція отримує поточний текст командного рядка, з яким ви зможете працювати. Наприклад, щоб показати тут модуль Starship `character`, вам потрібно

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- Типово, права частина вводу є порожньою. Щоб кастомізувати її, створіть нову функцію з назвою `starship_transient_rprompt_func`. Ця функція отримує поточний текст командного рядка, з яким ви зможете працювати. Наприклад, щоб показати час, коли була запущена остання команда, ви можете зробити

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt та TransientRightPrompt у Fish

Можна замінити попередньо надрукований командний рядок на власний. Це корисно у випадках, коли вся інформація з командного рядка не потрібна. Щоб увімкнути це, запустіть `enable_transience` в сеансі оболонки. Щоб зробити цю змінну постійною, додайте цей вираз до ваших налаштувань `~/.config/fish/config.fish`. Перехідність може бути вимкнена за допомогою `disable_transience`.

Зверніть увагу, що у випадку Fish, перехідний командний рядок буде надруковано лише тоді, коли командний рядок не порожній та синтаксично правильний.

- Типово, ліва частина вводу буде замінена на зелений символ `❯`. Щоб кастомізувати її, створіть нову функцію з назвою `starship_transient_prompt_func`. Наприклад, щоб показати тут модуль Starship `character`, вам потрібно

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- Типово, права частина вводу є порожньою. Щоб кастомізувати її, створіть нову функцію з назвою `starship_transient_rprompt_func`. Наприклад, щоб показати час, коли була запущена остання команда, ви можете зробити

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt та TransientRightPrompt в Bash

The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework at v0.4 or higher allows you to replace the previous-printed prompt with custom strings. Це корисно у випадках, коли вся інформація з командного рядка не потрібна. Для увімкнення цього додайте до `~/.bashrc` рядок `bleopt prompt_ps1_transient=<value>`:

\<value\> тут  – це розділений двокрапкою список `always`, `same-dir` та `trim`. Якщо `prompt_ps1_final` порожній і цей параметр має не пусте значення, командний рядок, вказаний у `PS1` буде стертий при виході з поточного командного рядка. Якщо значення містить поле `trim`, тільки останній рядок багаторядкового `PS1` буде збережений, а інші вилучені. В іншому випадку командний рядок буде встановлено перестворено, якщо вказано `PS1=`. Коли поле `same-dir` міститься у значені та поточна тека є відмінною від останньої теки у попередньому виводі командного рядка, параметр `prompt_ps1_transient` не враховується.

Зробіть наступні зміни у `~/.bashrc`, щоб налаштувати, що показується ліворуч і праворуч:

- Для налаштування того, чим замінюється ліва частина вводу, налаштуйте параметр `prompt_ps1_final`. Наприклад, щоб показати тут модуль Starship `character`, вам потрібно

```bash
bleopt prompt_ps1_final="$(starship module character)"
```

- Для налаштування того, чим замінюється права частина вводу, налаштуйте параметр `prompt_rps1_final`. Наприклад, щоб показати час, коли була запущена остання команда, ви можете зробити

```bash
bleopt prompt_rps1_final="$(starship module time)"
```

## Власні команди pre-prompt та pre-execution в Cmd

Clink забезпечує надзвичайно гнучкий API для виконання команд pre-prompt і pre-exec в Cmd. Його досить просто використовувати в Starship. Зробіть наступні зміни у вашому `starship.lua` відповідно до ваших вимог:

- Для запуску власних функцій прямо перед виводом командного рядка, визначте нову функцію з назвою `starship_preprompt_user_func`. Ця функція отримує поточний текст командного рядка, з яким ви зможете працювати. Наприклад, щоб показати ракету перед командним рядком, ви можете зробити наступне

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- Для запуску власних функцій прямо перед виконанням команди, визначте нову функцію з назвою `starship_precmd_user_func`. Ця функція отримує поточний текст команди, з яким ви зможете працювати. Наприклад, для виводу команди, яка буде виконана, вам треба зробити

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Власні команди pre-prompt та pre-execution в Bash

Bash не має офіційної системи preexec/precmd, як більшість інших оболонок. Через це важко забезпечити повністю настроювані гачки для цього в `bash`. Однак, Starship дає можливість вставити свої власні функції в процедуру виводу командного рядка:

- Для запуску власних функцій прямо перед виводом командного рядка, визначте нову функцію з назвою `starship_precmd_user_func`. Наприклад, щоб показати ракету перед командним рядком, ви можете зробити наступне

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Щоб запустити власну функцію прямо перед запуском команди, можна використати механізм [`DEBUG` trap](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Однак, ви **повинні** перехопити сигнал DEBUG _перед_ ініціалізацією Starship! Starship може зберегти значення перехоплення DEBUG, але якщо перехоплення буде перезаписане після початку запуску, деякі функції не працюватимуть.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Власні команди pre-prompt та pre-execution в PowerShell

PowerShell не має офіційної системи preexec/precmd, як більшість інших оболонок. Через це важко забезпечити повністю настроювані гачки для цього в `powershell`. Однак, Starship дає можливість вставити свої власні функції в процедуру виводу командного рядка:

Створіть функцію з назвою `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Зміна заголовка вікна

В деяких оболонках командний рядок автоматично змінює заголовок вікна (наприклад, на назву поточної теки). У Fish це стандартна поведінка. Starship цього не робить, але це досить просто зробити у `bash`, `zsh`, `cmd` або `powershell`.

Спочатку визначте функцію зміни заголовка вікна (в bash і zsh – однаково):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

Ви можете використовувати змінні для налаштування цього заголовка (`$USER`, `$HOSTNAME`, та `$PWD` – є досить популярними).

У `bash` встановіть цю функцію як функцію precmd у starship:

```bash
starship_precmd_user_func="set_win_title"
```

У `zsh` додайте наступне до масиву `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

Якщо вам подобається результат, додайте ці рядки до файлу налаштування оболонки (`~/.bashrc` чи `~/.zshrc`), щоб зробити його постійним.

Наприклад, якщо ви хочете показати вашу поточну теку у заголовку вкладці термінала, додайте наступний сніпет до вашого `~/.bashrc` або `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

Для Cmd, ви можете змінити заголовок вікна за допомогою функції `starship_preprompt_user_func`.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

Ви також можете встановити подібний вивід у PowerShell, створивши функцію з назвою `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Командний рядок праворуч

Деякі оболонки підтримують вивід командного рядка праворуч. Starship може встановити вміст правої частини командного рядка за допомогою параметра `right_format`. Будь-який модуль, який можна використовувати у `format`, також підтримується у `right_format`. Змінна `$all` міститиме лише модулі, які явно не використовується, а ні в `format`, а ні в `right_format`.

Примітка: командний рядок праворуч – це один рядок, що знаходиться праворуч у рядку вводу. Щоб вирівняти модулі праворуч над рядком введення в багаторядковому запиті, перегляньте [модуль `fill`](../config/#fill).

`right_format` наразі підтримується для таких оболонок: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Note: The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework v0.4 or higher should be installed in order to use right prompt in bash.

### Приклад

```toml
# ~/.config/starship.toml

# Мінімум ліворуч
format = """$character"""

# решту командного рядка перенесемо праворуч
right_format = """$all"""
```

Отримаємо командний рядок наступного виду:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Подовжений ввід

Деякі оболонки підтримують подовження вводу так само як і  звичайний ввід в командний рядок. Такий ввід буде показаний замість звичайного, колі користувач ввів символ продовження вводу (наприклад одну ліву дужку чи лапку).

У Starship можна встановити показ продовженого вводу за допомогою параметра `continuation_prompt`. Стандартний командний рядок — `'[∙](bright-black) '`.

Примітка: `continuation_prompt` слід встановити на літеральний рядок без жодних змінних.

Примітка: Подовжений ввід доступний лише для наступних оболонок:

- `bash`
- `zsh`
- `PowerShell`

### Приклад

```toml
# ~/.config/starship.toml

# Подовжений ввід позначається двома стрілками
continuation_prompt = '▶▶ '
```

## Рядки стилів

Рядки стилю — список слів, розділених пробілами. Слова не чутливі до регістру (наприклад `bold` і `BoLd` вважаються однаковими). Кожне слово може бути одним з наступних:

- `bold`
- `italic`
- `underline`
- `dimmed`
- `inverted`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

де `<color>` є специфікацією кольору (обговорюється нижче). `fg:<color>` та `<color>` на цей час роблять те саме, хоча це може змінитися в майбутньому. `inverted` замінює кольори тла і тексту. Порядок слів у рядку не має значення.

Токен `none` перевизначає всі інші токени у рядку, якщо він не є частиною `bg:`, так `fg:red none fg:blue` все одно створить рядок без стилізування. `bg:none` встановлює типовий колір фону, таким чином, `fg:red bg:none` еквівалентно `red` або `fg:red`, а `bg:green fg:red fg:none` також еквівалентно `fg:red` або `red`. Використання `none` у поєднанні з іншими токенами в майбутньому може стати помилкою.

Визначення кольору може бути одним з наступних:

- Один за стандартних кольорів термінала: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Ви можете опціонально додавати префікс `bright-`, щоб отримати яскраву версію (наприклад, `bright-white`).
- `#` за яким йде шестизначний шістнадцятковий код кольору. Цей код вказує на [шістнадцятковий код RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Число від 0-255. Число визначає [8-бітний код кольору ANSI](https://i.stack.imgur.com/KTSQa.png).

Якщо для тексту та фону задано кілька кольорів, останній в рядку буде мати вищий пріоритет.

Не кожен рядок стилю буде правильно показуватись у кожному терміналі. Зокрема, існують такі відомі примхи:

- Багато терміналів стандартно вмикають підтримку `blink`
- `hidden` [не підтримується в iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` не підтримується стандартно в macOS Terminal.app
