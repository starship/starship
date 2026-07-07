# Налаштування

Щоб почати налаштування starship, створіть наступний файл: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Всі налаштування для starship виконуються в цьому [TOML](https://github.com/toml-lang/toml) файлі:

```toml
# Отримання автозавершення вводу команд з config-schema.json
"$schema" = 'https://starship.rs/config-schema.json'

# Додавати порожні рядки між командними рядками
add_newline = true

# Заміняти символ '❯' в командному рядку на символ  '➜'
[character] # Назва модуля, який ми налаштовуємо – 'character'
success_symbol = '[➜](bold green)' # Частина 'success_symbol' буде використовувати символ '➜' підсвічений значенням 'bold green'

# Вимкнути модуль package, повністю приховавши його в командному рядку
[package]
disabled = true
```

### Розташування файлу налаштувань

Ви можете змінити типове розташування файлу налаштувань за допомогою змінної середовища `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

Так само і в PowerShell (Windows), можна додати цей рядок до `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Або для Cmd (Windows), додайте рядок до вашого файлу `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Ведення журналу

By default starship logs warnings and errors into a file named `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, where the session key is corresponding to an instance of your terminal.
This, however can be changed using the `STARSHIP_CACHE` environment variable:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Так само і в PowerShell (Windows), можна додати цей рядок до `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Або для Cmd (Windows), додайте рядок до вашого файлу `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Термінологія

**Module**: A component in the prompt giving information based on contextual information from your OS. Наприклад, модуль "nodejs" показує версію Node.js, яка встановлена на вашому компʼютері, якщо ваша поточна тека є проєктом Node.js.

**Variable**: Smaller sub-components that contain information provided by the module. Наприклад, змінна "version" в модулі "nodejs" містить поточну версію Node.js.

Згідно з правилами, більшість модулів мають префікс стандартного кольору термінала (наприклад, `via` в "nodejs") та порожнє місце як суфікс.

### Рядки

В синтаксисі TOML [текстові значення](https://toml.io/en/v1.0.0#string) оголошуються за допомогою лапок `'`, `"`, `'''`, or `"""`.

Наступні символи синтаксису Starship мають спеціальне призначення у форматуванні рядків і повинні бути екрановані, якщо ви хочете показувати їх як символи: `$ [ ] ( )`.

| Символ | Тип                    | Примітки                                                              |
| ------ | ---------------------- | --------------------------------------------------------------------- |
| `'`    | літерал                | менш екранований                                                      |
| `"`    | рядок                  | більш екранований                                                     |
| `'''`  | багаторядковий літерал | менш екранований                                                      |
| `"""`  | багаторядковий рядок   | більш екранований, нові рядки в оголошеннях можуть бути проігноровані |

Наприклад:

```toml
# літерал
format = '☺\☻ '

# звичайний рядок
format = "☺\\☻ "

# екрановані службові символи Starship
format = '\[\$\] '
```

При використанні символів розриву рядків можна використовувати багаторядкові оголошення.
For example, if you want to print a `$` symbol on a new line, the following values for `format` are equivalent:

```toml
# для літералів
format = '''

\$'''

# для звичайних багаторядкових рядків
format = """

\\$"""

# для звичайних рядків
format = "\n\\$"
```

У звичайних рядках, символ нового рядка можна використовувати наступним чином, щоб уникнути його показу.

```toml
format = """
line1\
line1\
line1
line2\
line2\
line2
"""
```

### Формат рядків

Формат рядків це формат, з яким модуль виводить всі змінні.
Most modules have an entry called `format` that configures the display format of the module.
Можна використовувати текст, змінні та текстові групи у форматуванні рядків.

#### Змінна

A variable contains a `$` symbol followed by the name of the variable.
Назва змінною може складатись лише з літер та цифр та символу `_`.

Наприклад:

- `'$version` – це форматований рядок зі змінною з назвою `version`.
- `$git_branch$git_commit'` – рядок з двома змінними `git_branch` і `git_commit`.
- `'$git_branch $git_commit'` має дві змінні, розділені пробілом.

#### Текстова група

Текстова група складається з двох різних частин.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings).
Ви можете додати текст, змінні або навіть вкладені текстові групи.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). Ця частина використовується для стилізації першої частини.

Наприклад:

- `'[on](red bold)'` – виведе рядок `on` жирним шрифтом підсвічений червоним.
- `'[⌘ $version](bold green)'` – виведе символ `⌘` разом з вмістом змінної  `version` жирним шрифтом підсвіченим зеленим.
- `'[a [b](red) c](green)'` – виведе `a b c`, де `b` – червона, а `a` та `c` – зелені.

#### Рядки стилів

Більшість модулів в starship дозволяють вам налаштовувати стилі їх виводу. This is done with an entry (usually called `style`) which is a string specifying the configuration. Ось деякі приклади стилів поряд з тим, що вони роблять. For details on the full syntax, consult the [advanced config guide](../advanced-config/).

- `'fg:green bg:blue'` – зелений текст на блакитному фоні
- `'bg:blue fg:bright-green'` – яскраво зелений текст на блакитному фоні
- `'bold fg:27'` – жирний текст з [ANSI кольором](https://i.stack.imgur.com/KTSQa.png) 27
- `'underline bg:#bf5700'` – підкреслений текст на яскраво помаранчевому фоні
- `'bold italic fg:purple'` – жирний курсив пурпурового кольору
- `''` – явним чином вимикає всі стилі

Зверніть увагу, що те, як виглядатиме стиль, як буде залежати від вашого емулятора термінала. Наприклад, деякі емулятори терміналів будуть виділяти кольори замість показу жирного тексту, а також деякі кольорові теми використовують однакові значення для нормального і яскравого кольорів. Також, щоб отримати курсив, ваш термінал має містити його підтримку.

#### Умовне форматування

Рядок з умовним форматуванням, огорнутий в круглі дужки `(` та `)`, не буде показуватись якщо змінні в середині не містять значень.

Наприклад:

- `'(@$region)'` – не буде показуватись, якщо змінна `region` дорівнює `None` чи є порожнім рядком, в іншому випадку вона буде показана разом з `@`, за яким йде значення регіону.
- `'(якийсь текст)'` ніколи не буде показуватись, через те, що немає змінних, загорнутих в дужки.
- When `$combined` is a shortcut for `\[$a$b\]`, `'($combined)'` will show nothing only if `$a` and `$b` are both `None`.
  This works the same as `'(\[$a$b\] )'`.

### Негативний збіг

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. Вони отримують перелік рядків для порівняння, чи в них є збіги, чи – немає. Символ для перевірки "негативного збігу"  (негативний збіг – те що не має мати збігу з вказаним значенням) – '!' знак оклику, вказується на початку параметра, що перевіряється. The presence of _any_ negative indicator in the directory
will result in the module not being matched.

Розширення зіставляються як із символами після останньої крапки в назві файлу, так і з
символами після першої крапки в назві файлу. For example, `foo.bar.tar.gz` will be matched
against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Файли, назва яких починається з крапки, взагалі не вважаються такими, що мають розширення.

Щоб побачити, як це працює на практиці, ви можете шукати збіг для файлів TypeScript, але не для MPEG Transport Stream, таким чином:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Командний рядок

Це перелік параметрів налаштувань, що використовуються для всього командного рядка.

### Параметри

| Параметр          | Стандартно                     | Опис                                                                                                                                                                                                                                   |
| ----------------- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | Налаштовує формат командного рядка.                                                                                                                                                                                    |
| `right_format`    | `''`                           | Дивіться [Увімкнути вивід праворуч](../advanced-config/#enable-right-prompt)                                                                                                                                                           |
| `scan_timeout`    | `30`                           | Тайм-аут для сканування файлів (у мілісекундах).                                                                                                                                                    |
| `command_timeout` | `500`                          | Тайм-аут для команд, виконаних starship (у мілісекундах).                                                                                                                                           |
| `add_newline`     | `true`                         | Вставити порожній рядок між командними рядками в оболонці.                                                                                                                                                             |
| `palette`         | `''`                           | Встановлює кольорову палітру використовуючи `palettes`.                                                                                                                                                                |
| `palettes`        | `{}`                           | Collection of color palettes that assign [colors](../advanced-config/#style-strings) to user-defined names. Зверніть увагу, що кольорові палітри не можуть посилатися на їх власні визначення кольору. |
| `follow_symlinks` | `true`                         | Перевіряти символічні посилання чи вони посилаються на теки; використовується в таких модулях як git.                                                                                                                  |

> [!TIP] Порада Якщо у вас є символічні посилання до мережевих файлових систем, встановіть `follow_symlinks` у `false`.

### Приклад

```toml
# ~/.config/starship.toml

# Використовуємо власний формат
format = '''
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) '''

# Чекаємо 10 мілісекунд, поки starship перевірить файли в поточній теці.
scan_timeout = 10

# Не додавати порожній рядок на початку командного рядка
add_newline = false

# Встановлює власну кольорову палітру 'foo'
palette = 'foo'

# Визначає власні кольори
[palettes.foo]
# Перевизначає наявний колір
blue = '21'
# Визначає новий колір
mustard = '#af8700'
```

### Стандартний формат командного рядка {#default-prompt-format}

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. Стандартне значення є таким, як зазначено нижче:

```toml
format = '$all'

# Є еквівалентом до
format = """
$username\
$hostname\
$localip\
$shlvl\
$singularity\
$kubernetes\
$nats\
$directory\
$vcsh\
$fossil_branch\
$fossil_metrics\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$hg_state\
$pijul_channel\
$docker_context\
$package\
$bun\
$c\
$cmake\
$cobol\
$cpp\
$daml\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$fennel\
$fortran\
$gleam\
$golang\
$gradle\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$lua\
$maven\
$mojo\
$nim\
$nodejs\
$ocaml\
$odin\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$quarto\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$solidity\
$swift\
$terraform\
$typst\
$vlang\
$vagrant\
$xmake\
$zig\
$buf\
$guix_shell\
$nix_shell\
$conda\
$pixi\
$meson\
$spack\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
$direnv\
$env_var\
$mise\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$container\
$netns\
$os\
$shell\
$character"""
```

If you just want to extend the default format, you can use `$all`;
modules you explicitly add to the format will not be duplicated. Напр.

```toml
# Перемістити назву теки на другий рядок
format = '$all$directory$character'
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials.
The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process`, `sso_start_url`, or `sso_session` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice.
If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

Під час використання [aws-vault](https://github.com/99designs/aws-vault) профіль зчитується зі змінної `AWS_VAULT`, а дата закінчення терміну дії облікових даних зчитується зі змінної `AWS_SESSION_EXPIRATION`.

Під час використання [awsu](https://github.com/kreuzwerker/awsu) профіль читається зі змінної `AWSU_PROFILE`.

Під час використання [AWSume](https://awsu.me) профіль зчитується зі змінної `AWSUME_PROFILE`, а дата закінчення терміну дії облікових даних зчитується зі змінної `AWSUME_EXPIRATION`.

Під час використання [saml2aws](https://github.com/Versent/saml2aws) інформація про закінчення терміну дії, отримана з `~/.aws/credentials`, повертається до ключа `x_security_token_expires`.

Під час використання [aws-sso-cli](https://github.com/synfinatic/aws-sso-cli) профіль читається зі змінної `AWS_SSO_PROFILE`.

### Параметри

| Параметр            | Стандартно                                                            | Опис                                                                                                                                 |
| ------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | Формат модуля.                                                                                                       |
| `символ`            | `'☁️ '`                                                               | Символ, який використовується під час показу перед поточним профілем AWS.                                            |
| `region_aliases`    | `{}`                                                                  | Таблиця псевдонімів регіону для показу на додачу до назви AWS.                                                       |
| `profile_aliases`   | `{}`                                                                  | Таблиця псевдонімів профілю для показу на додачу до назви AWS.                                                       |
| `style`             | `'bold yellow'`                                                       | Стиль модуля.                                                                                                        |
| `expiration_symbol` | `'X'`                                                                 | Символ, який показується, коли закінчився термін дії тимчасових облікових даних.                                     |
| `disabled`          | `false`                                                               | Вимикає модуль `AWS`.                                                                                                |
| `force_display`     | `false`                                                               | Якщо `true`, інформація показується, навіть якщо `credentials`, `credential_process` або `sso_start_url` не вказано. |

### Змінні

| Змінна   | Приклад          | Опис                                     |
| -------- | ---------------- | ---------------------------------------- |
| region   | `ap-northeast-1` | Поточний регіон AWS                      |
| profile  | `astronauts`     | Поточний профіль AWS                     |
| duration | `2h27m20s`       | Термін дії тимчасових облікових даних    |
| symbol   |                  | Віддзеркалює значення параметра `symbol` |
| style\*  |                  | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклади

#### Показати все

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
```

#### Показати регіон

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$region]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
```

#### Показати профіль

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$profile]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

The `azure` module shows the current Azure Subscription. This is based on showing the name of the default subscription or the username, as defined in the `~/.azure/azureProfile.json` file.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Змінна                 | Стандартно                               | Опис                                                                                       |
| ---------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`               | `'on [$symbol($subscription)]($style) '` | Формат для модуля Azure.                                                   |
| `символ`               | `'󰠅 '`                                  | Символ, який використовується.                                             |
| `style`                | `'blue bold'`                            | Стиль, який використовується.                                              |
| `disabled`             | `true`                                   | Вимикає модуль `azure`.                                                    |
| `subscription_aliases` | `{}`                                     | Таблиця псевдонімів підписки для показу на додачу до назви підписки Azure. |

### Приклади

#### Показ назви підписки

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = '󰠅 '
style = 'blue bold'
```

#### Показ імені користувача

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "󰠅 "
style = "blue bold"
```

#### Показ псевдонімів підписки

```toml
# ~/.config/starship.toml

[azure.subscription_aliases]
very-long-subscription-name = 'vlsn'
```

## Battery

The `battery` module shows how charged the device's battery is and its current charging status.
Модуль показується лише тоді, коли заряд акумулятора пристрою нижче 10%.

### Параметри

| Параметр             | Стандартно                        | Опис                                                 |
| -------------------- | --------------------------------- | ---------------------------------------------------- |
| `full_symbol`        | `'󰁹 '`                           | Символ, повного заряду батареї.      |
| `charging_symbol`    | `'󰂄 '`                           | Символ процесу заряджання.           |
| `discharging_symbol` | `'󰂃 '`                           | Символ, коли батарея розряджається.  |
| `unknown_symbol`     | `'󰂑 '`                           | Символ, коли стан батареї невідомий. |
| `empty_symbol`       | `'󰂎 '`                           | Символ повністю розрядженої батареї. |
| `format`             | `'[$symbol$percentage]($style) '` | Формат модуля.                       |
| `display`            | [link](#battery-display)          | Граничні значення і стиль модуля.    |
| `disabled`           | `false`                           | Вимикає модуль `battery`.            |

### Приклад

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Показ модуля Battery

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style).
If no `display` is provided. Стандартне значення є таким, як зазначено нижче:

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

Типові значення для опції `charging_symbol` і `discharging_symbol` є відповідно типовими значеннями параметрів `charging_symbol` та `discharging_symbol` `батареї`.

#### Параметри

Параметр `display` є масивом з наступної таблиці.

| Параметр             | Стандартно   | Опис                                                                                                                                                     |
| -------------------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | Верхня межа для параметра.                                                                                                               |
| `style`              | `'red bold'` | Стиль, який використовується, якщо параметр використовується.                                                                            |
| `charging_symbol`    |              | Необов'язковий символ, що показується якщо параметр display використовується, стандартно використовується параметр `charging_symbol`.    |
| `discharging_symbol` |              | Необов'язковий символ, що показується якщо параметр display використовується, стандартно використовується параметр `discharging_symbol`. |

#### Приклад

```toml
[[battery.display]] # стиль 'bold red' та discharging_symbol, якщо заряд між 0% та 10%
threshold = 10
style = 'bold red'

[[battery.display]] # стиль 'bold yellow' та символ 💦, якщо заряд між 10% та 30%
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦'

# якщо рівень заряду понад 30%, індикатор заряду не показується
```

## Buf

The `buf` module shows the currently installed version of [Buf](https://buf.build). By default, the module is shown if the current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### Параметри

| Параметр            | Стандартно                                      | Опис                                                         |
| ------------------- | ----------------------------------------------- | ------------------------------------------------------------ |
| `format`            | `'with [$symbol($version )]($style)'`           | Формат для модуля `buf`.                     |
| `version_format`    | `'v${raw}'`                                     | Формат версії.                               |
| `символ`            | `'🐃 '`                                         | Символ, який знаходиться перед версією Buf.  |
| `detect_extensions` | `[]`                                            | Які розширення повинні запускати цей модуль. |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Які імена файлів мають запускати цей модуль. |
| `detect_folders`    | `[]`                                            | В яких теках цей модуль має запускатись.     |
| `style`             | `'bold blue'`                                   | Стиль модуля.                                |
| `disabled`          | `false`                                         | Вимкнути модуль `elixir`.                    |

### Змінні

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| `version` | `v1.0.0` | Версія `buf`                             |
| `символ`  |          | Віддзеркалює значення параметра `symbol` |
| `style`\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime.
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `bun.lock`
- Поточна тека містить файл `bun.lockb`
- Поточна тека містить файл `bunfig.toml`

### Параметри

| Параметр            | Стандартно                                 | Опис                                                                                                     |
| ------------------- | ------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🥟 '`                                    | Формат рядка, що представляє символ Bun.                                                 |
| `detect_extensions` | `[]`                                       | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                       | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold red'`                               | Стиль модуля.                                                                            |
| `disabled`          | `false`                                    | Вимикає модуль `bun`.                                                                    |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v0.1.4` | Версія `bun`                             |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

#### Налаштуйте формат

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

The `c` module shows some information about your C compiler. By default
the module will be shown if the current directory contains a `.c` or `.h`
file.

### Параметри

| Параметр            | Стандартно                                                                    | Опис                                                                                                     |
| ------------------- | ----------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | Формат рядка модуля.                                                                     |
| `version_format`    | `'v${raw}'`                                                                   | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'C '`                                                                        | Символ, який знаходиться перед інформацією про компілятор                                                |
| `detect_extensions` | `['c', 'h']`                                                                  | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                                                          | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                                                          | В яких теках цей модуль має запускатись.                                                 |
| `команди`           | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | Як виявити компілятор                                                                                    |
| `style`             | `'bold 149'`                                                                  | Стиль модуля.                                                                            |
| `disabled`          | `false`                                                                       | Вимикає модуль `c`.                                                                      |

### Змінні

| Змінна  | Приклад                                | Опис                                     |
| ------- | -------------------------------------- | ---------------------------------------- |
| name    | clang                                  | Назва компілятора                        |
| version | 13.0.0 | Версія компілятора                       |
| symbol  |                                        | Віддзеркалює значення параметра `symbol` |
| style   |                                        | Віддзеркалює значення параметра `style`  |

### Команди

Параметр `commands` отримує список команд для визначення версії та назви компілятора.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship спробує виконати кожну команду, поки не отримає результат в STDOUT.

Якщо C-компілятор не підтримується цим модулем, ви можете зробити [запит на GitHub](https://github.com/starship/starship/issues/new/choose).

### Приклад

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## CPP

The `cpp` module shows some information about your `C++` compiler. By default,
the module will be shown if the current directory contains a `.cpp`, `.hpp`, or other `C++`-related files.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр            | Стандартно                                                                       | Опис                                                                                                     |
| ------------------- | -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | Формат рядка модуля.                                                                     |
| `version_format`    | `'v${raw}'`                                                                      | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'C++ '`                                                                         | Символ, який знаходиться перед інформацією про компілятор                                                |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                                                             | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                                                             | В яких теках цей модуль має запускатись.                                                 |
| `команди`           | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | Як виявити компілятор                                                                                    |
| `style`             | `'bold 149'`                                                                     | Стиль модуля.                                                                            |
| `disabled`          | `true`                                                                           | Вимикає модуль `cpp`.                                                                    |

### Змінні

| Змінна  | Приклад                                | Опис                                     |
| ------- | -------------------------------------- | ---------------------------------------- |
| name    | clang++                                | Назва компілятора                        |
| version | 13.0.0 | Версія компілятора                       |
| symbol  |                                        | Віддзеркалює значення параметра `symbol` |
| style   |                                        | Віддзеркалює значення параметра `style`  |

### Команди

Параметр `commands` отримує список команд для визначення версії та назви компілятора.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycpp', '--version']`. Starship спробує виконати кожну команду, поки не отримає результат в STDOUT.

Якщо компілятор C++ компілятор не підтримується цим модулем, ви можете зробити [запит на GitHub](https://github.com/starship/starship/issues/new/choose).

### Приклад

```toml
# ~/.config/starship.toml

[cpp]
disabled = false
format = 'via [$name $version]($style)'
```

## Character

Модуль `character` показує символ (зазвичай стрілку) поруч з текстом, який вводиться в терміналі.

Символ підкаже вам, чи була остання команда успішною, чи ні. Це можна зробити двома способами:

- зміною кольору (`red`/`green`)
- зміною символу (`❯`/`✖`)

Стандартно відбувається зміна кольору. If you also want to change its shape take a
look at [this example](#with-custom-error-shape).

> [!WARNING]
> `vimcmd_symbol` is only supported in cmd, fish and zsh.
> `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol`
> are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

### Параметри

| Параметр                    | Стандартно           | Опис                                                                                                                     |
| --------------------------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `format`                    | `'$symbol '`         | Формат рядка, що знаходиться перед введенням тексту.                                                     |
| `success_symbol`            | `'[❯](bold green)'`  | Формат рядка, що знаходиться перед введенням тексту, якщо попередня команда була успішно виконана.       |
| `error_symbol`              | `'[❯](bold red)'`    | Формат рядка, що знаходиться перед введенням тексту, якщо попередня команда не була успішно виконана.    |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | Формат рядка, що знаходиться перед введенням тексту, якщо оболонка перебуває у звичайному режимі vim.    |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | Формат рядка, що знаходиться перед введенням тексту, якщо оболонка перебуває у режимі vim `replace_one`. |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | Формат рядка, що знаходиться перед введенням тексту, якщо оболонка перебуває у режимі заміни vim.        |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | Формат рядка, що знаходиться перед введенням тексту, якщо оболонка перебуває у візуальному режимі vim.   |
| `disabled`                  | `false`              | Вимикає модуль `character`.                                                                              |

### Змінні

| Змінна | Приклад | Опис                                                                                                                               |
| ------ | ------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| symbol |         | Віддзеркалює значення `success_symbol`, `error_symbol`, `vimcmd_symbol` або `vimcmd_replace_one_symbol` і подібне. |

### Приклади

#### З власним значком помилки

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[✗](bold red) '
```

#### Без власного значка помилки

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[➜](bold red) '
```

#### Значок vim

```toml
# ~/.config/starship.toml

[character]
vimcmd_symbol = '[V](bold green) '
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `CMakeLists.txt`
- Поточна тека містить файл `CMakeCache.txt`

### Параметри

| Параметр            | Стандартно                             | Опис                                                                                                     |
| ------------------- | -------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`   | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                            | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'△ '`                                 | Символ, який йде перед версією cmak.                                                     |
| `detect_extensions` | `[]`                                   | Які розширення повинні запускати цей модуль                                                              |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | Назви файлів, які активують модуль                                                                       |
| `detect_folders`    | `[]`                                   | Назви тек, що активують модуль                                                                           |
| `style`             | `'bold blue'`                          | Стиль модуля.                                                                            |
| `disabled`          | `false`                                | Вимикає модуль `cmake`.                                                                  |

### Змінні

| Змінна  | Приклад   | Опис                                     |
| ------- | --------- | ---------------------------------------- |
| version | `v3.17.3` | Версія cmake                             |
| symbol  |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL.
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файли `.cob` або `.COB`
- Поточна тека містить файли  `.cbl` або `.CBL`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `символ`            | `'⚙️ '`                              | Символ, який знаходиться перед версією COBOL.                                            |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | Стиль модуля.                                                                            |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `disabled`          | `false`                              | Вимикає модуль `cobol`.                                                                  |

### Змінні

| Змінна  | Приклад    | Опис                                     |
| ------- | ---------- | ---------------------------------------- |
| version | `v3.1.2.0` | Версія `cobol`                           |
| symbol  |            | Віддзеркалює значення параметра `symbol` |
| style\* |            | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

## Command Duration – час виконання

The `cmd_duration` module shows how long the last command took to execute.
The module will be shown only if the command took longer than two seconds, or
the `min_time` config value, if it exists.

> [!WARNING] Попередження Не перехоплюйте trap DEBUG у Bash
>
> Якщо ви запускаєте Starship у `bash`, не вмикайте `DEBUG trap` після запуску `eval $(starship init $0)`, бо цей модуль **не працюватиме**.

Bash users who need preexec-like functionality can use
[rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec).
Simply define the arrays `preexec_functions` and `precmd_functions` before
running `eval $(starship init $0)`, and then proceed as normal.

### Параметри

| Параметр               | Стандартно                    | Опис                                                                                                                                                                                                                                   |
| ---------------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Найменший час виконання для показу (в мілісекундах).                                                                                                                                                |
| `show_milliseconds`    | `false`                       | Показувати мілісекунди на додачу для секунд.                                                                                                                                                                           |
| `format`               | `'took [$duration]($style) '` | Формат модуля.                                                                                                                                                                                                         |
| `style`                | `'bold yellow'`               | Стиль модуля.                                                                                                                                                                                                          |
| `disabled`             | `false`                       | Вимикає модуль `cmd_duration`.                                                                                                                                                                                         |
| `show_notifications`   | `false`                       | Показувати сповіщення на робочому столі після закінчення команди.                                                                                                                                                      |
| `min_time_to_notify`   | `45_000`                      | Найменший час виконання для сповіщення (в мілісекундах).                                                                                                                                            |
| `notification_timeout` |                               | Тривалість показу сповіщення (у мілісекундах). Якщо не налаштовано, час очікування сповіщень визначатиметься демоном. Не всі демони сповіщень підтримують цю опцію. |

### Змінні

| Змінна   | Приклад  | Опис                                    |
| -------- | -------- | --------------------------------------- |
| duration | `16m40s` | Час, витрачений на виконання команди    |
| style\*  |          | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

Модуль `conda` показує інформацію про поточне оточення [Conda](https://docs.conda.io/en/latest/), якщо змінна `$CONDA_DEFAULT_ENV` встановлена.

> [!TIP]
> This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.
> If you use [pixi](https://pixi.sh), you can disable pixi's prompt modifier by running `pixi config set shell.change-ps1 false`.

### Параметри

| Параметр            | Стандартно                             | Опис                                                                                                                                                                                                                                                        |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `символ`            | `'🅒 '`                                | Символ що передує назві оточення.                                                                                                                                                                                                           |
| `style`             | `'bold green'`                         | Стиль модуля.                                                                                                                                                                                                                               |
| `format`            | `'via [$symbol$environment]($style) '` | Формат модуля.                                                                                                                                                                                                                              |
| `ignore_base`       | `true`                                 | Ігнорувати середовище `base`.                                                                                                                                                                                                               |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | Які змінні середовища повинні запускати цей модуль. Якщо це середовище pixi, цей модуль стандартно не запускається.                                                                                                         |
| `disabled`          | `false`                                | Вимикає модуль `conda`.                                                                                                                                                                                                                     |

### Змінні

| Змінна      | Приклад      | Опис                                     |
| ----------- | ------------ | ---------------------------------------- |
| environment | `astronauts` | Поточне середовище conda                 |
| symbol      |              | Віддзеркалює значення параметра `symbol` |
| style\*     |              | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Container

Модуль `container` показує символ та назву контейнера, коли ви перебуваєте в ньому.

### Параметри

| Параметр   | Стандартно                         | Опис                                                    |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `символ`   | `'⬢'`                              | Символ, що показується під час перебування в контейнері |
| `style`    | `'bold red dimmed'`                | Стиль модуля.                           |
| `format`   | `'[$symbol \[$name\]]($style) '` | Формат модуля.                          |
| `disabled` | `false`                            | Вимикає модуль `container`.             |

### Змінні

| Змінна  | Приклад             | Опис                                     |
| ------- | ------------------- | ---------------------------------------- |
| name    | `fedora-toolbox:35` | Назва контейнера                         |
| symbol  |                     | Віддзеркалює значення параметра `symbol` |
| style\* |                     | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `shard.yml`
- Поточна тека містить файл `.cr`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `символ`            | `'🔮 '`                              | Символ, який знаходиться перед версією crystal.                                          |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | Стиль модуля.                                                                            |
| `detect_extensions` | `['cr']`                             | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['shard.yml']`                      | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `disabled`          | `false`                              | Вимикає модуль `crystal`.                                                                |

### Змінні

| Змінна  | Приклад   | Опис                                     |
| ------- | --------- | ---------------------------------------- |
| version | `v0.32.1` | Версія `crystal`                         |
| symbol  |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers)
SDK version when you are in the root directory of your Daml project. The `sdk-version` in
the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION`
environment variable.
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `daml.yaml`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'Λ '`                               | Формат рядка, що представляє символ Daml                                                                 |
| `style`             | `'bold cyan'`                        | Стиль модуля.                                                                            |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['daml.yaml']`                      | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `disabled`          | `false`                              | Вимикає модуль `daml`.                                                                   |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v2.2.0` | Версія `daml`                            |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `.dart`
- Поточна тека містить файл `.dart_tool`
- Поточна тека містить файли `pubspec.yaml`, `pubspec.yml` або `pubspec.lock`

### Параметри

| Параметр            | Стандартно                                        | Опис                                                                                                     |
| ------------------- | ------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`              | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                       | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🎯 '`                                           | Формат рядка, що представляє символ Dart                                                                 |
| `detect_extensions` | `['dart']`                                        | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `['.dart_tool']`                                  | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold blue'`                                     | Стиль модуля.                                                                            |
| `disabled`          | `false`                                           | Вимикає модуль `dart`.                                                                   |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v2.8.4` | Версія `dart`                            |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файли  `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` чи `deps.js`

### Параметри

| Параметр            | Стандартно                                                                           | Опис                                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                                                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🦕 '`                                                                              | Формат рядка, що представляє символ Deno                                                                 |
| `detect_extensions` | `[]`                                                                                 | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                                                                 | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'green bold'`                                                                       | Стиль модуля.                                                                            |
| `disabled`          | `false`                                                                              | Вимикає модуль `deno`.                                                                   |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v1.8.3` | Версія `deno`                            |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Directory

The `directory` module shows the path to your current directory, truncated to
three parent folders. Шлях до теки також буде скорочений до кореня git-репозиторію, в якому ви перебуваєте.

Якщо використовується параметр `fish_style_pwd_dir_length`, замість того, щоб приховувати скорочений шлях, ви побачите скорочену назву кожної теки в залежності від числа, яке ви вказали для цього параметра.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root,
and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before
it would have been `nixpkgs/pkgs`.

### Параметри

| Параметр                 | Стандартно                                                                                                                   | Опис                                                                                                                                              |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | Кількість батьківських тек, до яких слід скоротити шлях до поточної теку.                                                         |
| `truncate_to_repo`       | `true`                                                                                                                       | Скорочувати чи ні шлях до кореня git-репозиторію, коли ви перебуваєте в ньому.                                                    |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | Формат модуля.                                                                                                                    |
| `style`                  | `'bold cyan'`                                                                                                                | Стиль модуля.                                                                                                                     |
| `disabled`               | `false`                                                                                                                      | Вимикає модуль `directory`.                                                                                                       |
| `read_only`              | `'🔒'`                                                                                                                       | Символ, який вказує, що поточна тека доступна лише для читання.                                                                   |
| `read_only_style`        | `'red'`                                                                                                                      | Стиль символу для елементів read only.                                                                                            |
| `truncation_symbol`      | `''`                                                                                                                         | Символ на початку урізаних шляхів. напр: '…/'                                                                     |
| `before_repo_root_style` |                                                                                                                              | Стиль частини  шляху, що передує кореню git-репозиторію. The default value is equivalent to `style`.              |
| `repo_root_style`        |                                                                                                                              | Стиль кореня git-репозиторію. The default value is equivalent to `style`.                                         |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | Формат git-репозиторію, коли задані `before_repo_root_style` та `repo_root_style`.                                                |
| `home_symbol`            | `'~'`                                                                                                                        | Символ, що позначає домашню теку.                                                                                                 |
| `use_os_path_sep`        | `true`                                                                                                                       | Використовувати розділювач шляхів, що притаманній вашій OS, замість того, щоб завжди використовувати `/` (напр `\` у Windows) |

<details><summary>Цей модуль має декілька додаткових опцій, які контролюють показ шляхів до тек.</summary>

| Додатковий параметр         | Стандартно | Опис                                                                                                                                                                                                   |
| --------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `substitutions`             |            | Масив або таблиця замін, які необхідно внести в шлях.                                                                                                                                  |
| `fish_style_pwd_dir_length` | `0`        | Кількість символів, які використовуються при застосуванні логіки шляху fish shell pwd.                                                                                                 |
| `use_logical_path`          | `true`     | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network
prefixes or development directories of Java. Зауважте, що це відключить стиль fish у PWD. Приймається масив наступних пар ключ/значення:

| Значення | Тип     | Опис                                                            |
| -------- | ------- | --------------------------------------------------------------- |
| `from`   | String  | Значення для заміни                                             |
| `to`     | String  | Заміна цього значення, якщо знайдено                            |
| `regex`  | Boolean | (Опціонально) Чи є `from` регулярним виразом |

By using `regex = true`, you can use [Rust's regular expressions](https://docs.rs/regex/latest/regex/#syntax) in `from`.
Наприклад, ви можете замінити кожну косу риску, окрім першої за допомогою:

```toml
substitutions = [
  { from = "^/", to = "<root>/", regex = true },
  { from = "/", to = " | " },
  { from = "^<root>", to = "/", regex = true },
]
```

Це замінить `/var/log` на `/ | var | log`.

Старий синтаксис все ще працює, хоча він не підтримує регулярні вирази:

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero,
the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path
`/built/this/city/on/rock/and/roll`, which would normally be displayed as `rock/and/roll`, would be displayed as
`/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with
a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Змінні

| Змінна  | Приклад               | Опис                                    |
| ------- | --------------------- | --------------------------------------- |
| path    | `'D:/Projects'`       | Поточний шлях до теки                   |
| style\* | `'black bold dimmed'` | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

<details><summary>Git-репозиторії мають додаткові змінні.</summary>

Розгляньмо шлях `/path/to/home/git_repo/src/lib`

| Змінна                                                     | Приклад               | Опис                                    |
| ---------------------------------------------------------- | --------------------- | --------------------------------------- |
| before_root_path | `≠`                   | Шлях до кореневої теки git              |
| repo_root                             | `'git_repo'`          | Назва кореневої теки git                |
| path                                                       | `'/src/lib'`          | Залишок шляху                           |
| style                                                      | `'black bold dimmed'` | Віддзеркалює значення параметра `style` |
| repo_root_style  | `'underline white'`   | Стиль кореневої теки git                |

</details>

### Приклад

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Direnv

The `direnv` module shows the status of the current rc file if one is present. The status includes the path to the rc file, whether it is loaded, and whether it has been allowed by `direnv`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр            | Стандартно                             | Опис                                                                                |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | Формат модуля.                                                      |
| `символ`            | `'direnv '`                            | Символ, що показується перед direnv context.                        |
| `style`             | `'bold orange'`                        | Стиль модуля.                                                       |
| `disabled`          | `true`                                 | Вимикає модуль `direnv`.                                            |
| `detect_extensions` | `[]`                                   | Які розширення повинні запускати цей модуль.                        |
| `detect_files`      | `['.envrc']`                           | Які імена файлів мають запускати цей модуль.                        |
| `detect_folders`    | `[]`                                   | В яких теках цей модуль має запускатись.                            |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Які змінні середовища повинні запускати цей модуль.                 |
| `allowed_msg`       | `'allowed'`                            | Повідомлення, що показується коли використання rc-файлу дозволене.  |
| `not_allowed_msg`   | `'not allowed'`                        | Повідомлення, що показується коли використання rc-файлу заборонене. |
| `denied_msg`        | `'denied'`                             | Повідомлення, що показується коли використання rc-файлу заборонене. |
| `loaded_msg`        | `'loaded'`                             | Повідомлення, що показується коли rc-файл завантажений.             |
| `unloaded_msg`      | `'not loaded'`                         | Повідомлення, що показується коли rc-файл не завантажений.          |

### Змінні

| Змінна                       | Приклад             | Опис                                                      |
| ---------------------------- | ------------------- | --------------------------------------------------------- |
| loaded                       | `loaded`            | Чи завантажений rc-файл.                  |
| allowed                      | `denied`            | Чи дозволене використання rc-файлу.       |
| rc_path | `/home/test/.envrc` | Шлях до rc-файлу.                         |
| symbol                       |                     | Віддзеркалює значення параметра `symbol`. |
| style\*                      | `red bold`          | Віддзеркалює значення параметра `style`.  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Docker Context

Модуль `docker_context` показує поточний [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) якщо його не встановлено у `default` чи `desktop-linux`, або якщо змінні середовища `DOCKER_MACHINE_NAME`, `DOCKER_HOST` або `DOCKER_CONTEXT` встановлені (оскільки вони призначені для перевизначення контексту).

### Параметри

| Параметр            | Стандартно                                                                                   | Опис                                                                                                                         |
| ------------------- | -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                                                           | Формат модуля.                                                                                               |
| `символ`            | `'🐳 '`                                                                                      | Символ, який знаходиться перед Docker context.                                                               |
| `only_with_files`   | `true`                                                                                       | Показувати, лише коли є збіг                                                                                                 |
| `detect_extensions` | `[]`                                                                                         | Які розширення мають запускати цей модуль (потрібно щоб в `only_with_files` було true).   |
| `detect_files`      | `['compose.yml', 'compose.yaml', 'docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Які імена файлів мають запускати цей модуль (потрібно щоб в `only_with_files` було true). |
| `detect_folders`    | `[]`                                                                                         | Які теки мають запускати цей модуль (потрібно щоб в `only_with_files` було true).         |
| `style`             | `'blue bold'`                                                                                | Стиль модуля.                                                                                                |
| `disabled`          | `false`                                                                                      | Вимикає модуль `docker_context`.                                                                             |

### Змінні

| Змінна  | Приклад        | Опис                                     |
| ------- | -------------- | ---------------------------------------- |
| context | `test_context` | Поточний docker context                  |
| symbol  |                | Віддзеркалює значення параметра `symbol` |
| style\* |                | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. Якщо SDK закріплена в поточній теці, показується закріплена версія. В іншому випадку модуль покаже останню встановлену версію SDK.

Стандартно модуль буде показаний в командному рядку, коли в теці присутні один чи більше наступних файлів:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

Вам також знадобиться .NET Core SDK, встановлений для того, щоб використовувати його правильно.

Всередині, цей модуль використовує власний механізм для виявлення версій. Typically it is twice as fast
as running `dotnet --version`, but it may show an incorrect version if your .NET project has an
unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by
setting `heuristic = false` in the module options.

Модуль також показуватиме Target Framework Monamework (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>), коли у поточній теці є файл `.csproj`.

### Параметри

| Параметр            | Стандартно                                                                                              | Опис                                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                          | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                                                                             | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'.NET '`                                                                                               | Символ, який знаходиться перед версією dotnet.                                           |
| `heuristic`         | `true`                                                                                                  | Використовувати швидший алгоритм визначення версії, щоб ваш starship залишався швидким.  |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                                                                                    | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold blue'`                                                                                           | Стиль модуля.                                                                            |
| `disabled`          | `false`                                                                                                 | Вимикає модуль `dotnet`.                                                                 |

### Змінні

| Змінна  | Приклад          | Опис                                                         |
| ------- | ---------------- | ------------------------------------------------------------ |
| version | `v3.1.201`       | Версія `dotnet` sdk                                          |
| tfm     | `netstandard2.0` | Псевдонім Target Framework, на який націлено поточний проєкт |
| symbol  |                  | Віддзеркалює значення параметра `symbol`                     |
| style\* |                  | Віддзеркалює значення параметра `style`                      |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `mix.exs`.

### Параметри

| Параметр            | Стандартно                                                  | Опис                                                                                                     |
| ------------------- | ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                                 | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'💧 '`                                                     | Символ, який знаходиться перед версією Elixir/Erlang.                                    |
| `detect_extensions` | `[]`                                                        | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['mix.exs']`                                               | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                                        | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold purple'`                                             | Стиль модуля.                                                                            |
| `disabled`          | `false`                                                     | Вимикає модуль `elixir`.                                                                 |

### Змінні

| Змінна                           | Приклад | Опис                                     |
| -------------------------------- | ------- | ---------------------------------------- |
| version                          | `v1.10` | Версія `elixir`                          |
| otp_version |         | Версія otp `elixir`                      |
| symbol                           |         | Віддзеркалює значення параметра `symbol` |
| style\*                          |         | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `elm.json`
- Поточна тека містить файл `elm-package.json`
- Поточна тека містить файл `.elm-version`
- Поточна тека містить файл `elm-stuff`
- The current directory contains `.elm` files

### Параметри

| Параметр            | Стандартно                                         | Опис                                                                                                     |
| ------------------- | -------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`               | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                        | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🌳 '`                                            | Формат рядка, що представляє символ Elm.                                                 |
| `detect_extensions` | `['elm']`                                          | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `['elm-stuff']`                                    | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'cyan bold'`                                      | Стиль модуля.                                                                            |
| `disabled`          | `false`                                            | Вимикає модуль `elm`.                                                                    |

### Змінні

| Змінна  | Приклад   | Опис                                     |
| ------- | --------- | ---------------------------------------- |
| version | `v0.19.1` | Версія `elm`                             |
| symbol  |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variables.
Модуль показується, якщо виконується будь-яка з наступних умов:

- Параметр `variable` відповідає наявній змінній середовища
- Параметр `variable` не визначено, але є параметр `default`

> [!TIP]
> The order in which env_var modules are shown can be individually set by including
> `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `env_var` module will simply show all env_var modules in the order they were defined.

> [!TIP]
> Multiple environmental variables can be displayed by using a `.`. (see example)
> If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.
>
> Приклад: наступна конфігурація показуватиме значення змінної середовища USER
>
> ```toml
>
> # ~/.config/starship.toml
>
> [env_var.USER] default = 'unknown user' ```
> ```

### Параметри

| Параметр     | Стандартно                            | Опис                                                                                    |
| ------------ | ------------------------------------- | --------------------------------------------------------------------------------------- |
| `символ`     | `""`                                  | Символ, який знаходиться перед значенням variable.                      |
| `змінна`     |                                       | Змінна середовища для показу.                                           |
| `стандартно` |                                       | Стандартне значення буде показане, якщо змінні для показу не визначені. |
| `format`     | `"with [$symbol$env_value]($style) "` | Формат модуля.                                                          |
| `опис`       | `"<env_var module>"`                  | Опис модуля, який показується під час запуску `starship explain`.       |
| `disabled`   | `false`                               | Вимикає модуль `env_var`.                                               |
| `style`      | `"black bold dimmed"`                 | Стиль модуля.                                                           |

### Змінні

| Змінна                         | Приклад                                                      | Опис                                             |
| ------------------------------ | ------------------------------------------------------------ | ------------------------------------------------ |
| env_value | `Windows NT` (якщо _variable_ буде `$OS`) | Значення змінної оточення з параметра `variable` |
| symbol                         |                                                              | Віддзеркалює значення параметра `symbol`         |
| style\*                        |                                                              | Віддзеркалює значення параметра `style`          |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[env_var]
variable = 'SHELL'
default = 'unknown shell'
```

Показ кількох змінних оточення:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = 'SHELL'
default = 'unknown shell'
[env_var.USER]
default = 'unknown user'
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `rebar.config`.
- Поточна тека містить файл `erlang.mk`.

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `' '`                               | Символ, який знаходиться перед версією erlang.                                           |
| `style`             | `'bold red'`                         | Стиль модуля.                                                                            |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `disabled`          | `false`                              | Вимикає модуль `erlang`.                                                                 |

### Змінні

| Змінна  | Приклад   | Опис                                     |
| ------- | --------- | ---------------------------------------- |
| version | `v22.1.3` | Версія `erlang`                          |
| symbol  |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

The `fennel` module shows the currently installed version of [Fennel](https://fennel-lang.org).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `.fnl`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🧅 '`                              | Символ, який знаходиться перед версією fennel.                                           |
| `style`             | `'bold green'`                       | Стиль модуля.                                                                            |
| `detect_extensions` | `['fnl']`                            | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `disabled`          | `false`                              | Вимикає модуль `fennel`.                                                                 |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v1.2.1` | Версія `fennel`                          |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are
present in a line they will split the space evenly between them. Це корисно для форматування інших модулів.

### Параметри

| Параметр   | Стандартно     | Опис                                                                |
| ---------- | -------------- | ------------------------------------------------------------------- |
| `символ`   | `'.'`          | Символ, який використовується для заповнення рядка. |
| `style`    | `'bold black'` | Стиль модуля.                                       |
| `disabled` | `false`        | Вимикає модуль `fill`                                               |

### Приклад

```toml
# ~/.config/starship.toml
format = 'AA $fill BB $fill CC'

[fill]
symbol = '-'
style = 'bold green'
```

Створить рядок, що виглядатиме наступним чином:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Fortran

Модуль `fortran` показує поточну встановлену версію Fortran.

### Параметри

| Параметр            | Стандартно                                                                                                                  | Опис                                                                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `символ`            | `' '`                                                                                                                      | Символ, який знаходиться перед версією Fortran.                                          |
| `format`            | `'via [$symbol($version )]($style)'`                                                                                        | Формат модуля.                                                                           |
| `version_format`    | `'${raw}'`                                                                                                                  | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold purple'`                                                                                                             | Стиль модуля.                                                                            |
| `detect_extensions` | `['f', 'F', 'for', 'FOR', 'ftn', 'FTN', 'f77', 'F77', 'f90', 'F90', 'f95', 'F95','f03', 'F03', 'f08', 'F08', 'f18', 'F18']` | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['fpm.toml']`                                                                                                              | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                                                                                                        | В яких теках цей модуль має запускатись.                                                 |
| `команди`           | `[ [ 'gfortran', '--version' ], [ 'flang', '--version' ], [ 'flang-new', '--version' ] ]`                                   | Як виявити компілятор                                                                                    |
| `disabled`          | `false`                                                                                                                     | Вимикає модуль `fortran`.                                                                |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| name    | gfortran | Назва компілятора                        |
| version | `14.2.0` | Версія компілятора Fortran               |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Команди

Параметр `commands` отримує список команд для визначення версії та назви компілятора.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['myfortran', '--version']`. Starship спробує виконати кожну команду, поки не отримає результат в STDOUT.

Якщо компілятор Fortran не підтримується цим модулем, ви можете зробити [запит на GitHub](https://github.com/starship/starship/).

## Fossil Branch

Модуль `fossil_branch` показує назву активної гілки у вашій поточній теці.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр            | Стандартно                       | Опис                                                                                                                                        |
| ------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | Формат модуля. Use `'$branch'` to refer to the current branch name.                                         |
| `символ`            | `' '`                           | Символ, що використовується перед назвою гілки у вашій поточній теці.                                                       |
| `style`             | `'bold purple'`                  | Стиль модуля.                                                                                                               |
| `truncation_length` | `2^63 - 1`                       | Скорочує назву гілки Fossil до `N` графем                                                                                                   |
| `truncation_symbol` | `'…'`                            | Символ, що використовується для позначення назви гілки, яка була скорочена. You can use `''` for no symbol. |
| `disabled`          | `true`                           | Вимикає модуль `fossil_branch`.                                                                                             |

### Змінні

| Змінна  | Приклад | Опис                                     |
| ------- | ------- | ---------------------------------------- |
| branch  | `trunk` | Поточна гілка Fossil                     |
| symbol  |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Fossil Metrics

The `fossil_metrics` module will show the number of added and deleted lines in the check-out in your current directory. Потрібна версія Fossil не нижче v2.14 (2021-01-20).

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр             | Стандартно                                                   | Опис                                                         |
| -------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Формат модуля.                               |
| `added_style`        | `'bold green'`                                               | Стиль для показу кількості доданих рядків.   |
| `deleted_style`      | `'bold red'`                                                 | Стиль для показу кількості видалених рядків. |
| `only_nonzero_diffs` | `true`                                                       | Показувати стан лише для змінених елементів. |
| `disabled`           | `true`                                                       | Вимикає модуль `fossil_metrics`.             |

### Змінні

| Змінна                               | Приклад | Опис                                            |
| ------------------------------------ | ------- | ----------------------------------------------- |
| added                                | `1`     | Поточна кількість доданих рядків                |
| deleted                              | `2`     | Поточна кількість видалених рядків              |
| added_style\*   |         | Віддзеркалює значення параметра `added_style`   |
| deleted_style\* |         | Віддзеркалює значення параметра `deleted_style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI.
This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.
The `CLOUDSDK_CORE_PROJECT` and `CLOUDSDK_COMPUTE_REGION` environment variables, when set, override the `project` and `region` values from the active configuration, mirroring the behavior of `gcloud` itself.

Коли модуль увімкнено, він завжди буде активним, якщо не встановлено параметр `detect_env_vars`, в такому випадку модуль буде активним лише коли буде встановлено одну зі змінних середовища.

### Параметри

| Параметр          | Стандартно                                                 | Опис                                                                                      |
| ----------------- | ---------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Формат модуля.                                                            |
| `символ`          | `'☁️  '`                                                   | Символ, який використовується під час показу перед поточним профілем GCP. |
| `region_aliases`  | `{}`                                                       | Таблиця псевдонімів регіону для показу на додачу до назви GCP.            |
| `project_aliases` | `{}`                                                       | Таблиця псевдонімів проєкту для показу на додачу до назви GCP.            |
| `detect_env_vars` | `[]`                                                       | Які змінні середовища повинні запускати цей модуль                                        |
| `style`           | `'bold blue'`                                              | Стиль модуля.                                                             |
| `disabled`        | `false`                                                    | Вимикає модуль `gcloud`.                                                  |

### Змінні

| Змінна  | Приклад       | Опис                                                           |
| ------- | ------------- | -------------------------------------------------------------- |
| region  | `us-central1` | Поточний регіон GCP                                            |
| account | `foo`         | Поточний профіль GCP                                           |
| domain  | `example.com` | Поточний домен профілю GCP                                     |
| project |               | Поточний проєкт GCP                                            |
| active  | `стандартно`  | Назва активної конфігурації з `~/.config/gcloud/active_config` |
| symbol  |               | Віддзеркалює значення параметра `symbol`                       |
| style\* |               | Віддзеркалює значення параметра `style`                        |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклади

#### Показувати обліковий запис та проєкт

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
```

#### Показувати тільки назву активної конфігурації

```toml
# ~/.config/starship.toml

[gcloud]
format = '[$symbol$active]($style) '
style = 'bold yellow'
```

#### Показувати обліковий запис та псевдо регіону

```toml
# ~/.config/starship.toml

[gcloud]
symbol = '️🇬️ '
[gcloud.region_aliases]
us-central1 = 'uc1'
asia-northeast1 = 'an1'
```

#### Показувати обліковий запис та псевдо проєкту

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = 'vlpn'
```

## Git Branch

Модуль `git_branch` показує активну гілку репозиторію у вашій поточній теці.

### Параметри

| Параметр             | Стандартно                                        | Опис                                                                                                                                        |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Показує назву віддаленої відстежуваної гілки, навіть якщо вона збігається з назвою локальної гілки.                         |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | Формат модуля. Use `'$branch'` to refer to the current branch name.                                         |
| `символ`             | `' '`                                            | Формат рядка, що представляє символ гілки git.                                                                              |
| `style`              | `'bold purple'`                                   | Стиль модуля.                                                                                                               |
| `truncation_length`  | `2^63 - 1`                                        | Скорочує назву гілки git до `N` графем.                                                                                     |
| `truncation_symbol`  | `'…'`                                             | Символ, що використовується для позначення назви гілки, яка була скорочена. You can use `''` for no symbol. |
| `only_attached`      | `false`                                           | Показувати назву гілки тільки коли вона не у відʼєднаному від `HEAD` стані.                                                 |
| `ignore_branches`    | `[]`                                              | Перелік назв, які не треба показувати. Корисно для 'master' або 'main'.                                     |
| `ignore_bare_repo`   | `false`                                           | Не показувати, коли в голих репозиторіях.                                                                                   |
| `disabled`           | `false`                                           | Вимикає модуль `git_branch`.                                                                                                |

### Змінні

| Змінна                             | Приклад  | Опис                                                                                                                                                      |
| ---------------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| branch                             | `master` | Назва поточної гілки, показується `HEAD`, якщо зараз немає поточної гілки (напр. git detached `HEAD`). |
| remote_name   | `origin` | Назва віддаленої гілки.                                                                                                                   |
| remote_branch | `master` | Назва гілки, що відстежується у `remote_name`.                                                                                            |
| symbol                             |          | Віддзеркалює значення параметра `symbol`                                                                                                                  |
| style\*                            |          | Віддзеркалює значення параметра `style`                                                                                                                   |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[git_branch]
symbol = '🌱 '
truncation_length = 4
truncation_symbol = ''
ignore_branches = ['master', 'main']
```

## Git Commit

Модуль `git_commit` показує поточний хеш коміту, а також теґ (якщо він є) репозиторію у вашій поточній теці.

### Параметри

| Параметр             | Стандартно                     | Опис                                                                                                                         |
| -------------------- | ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| `commit_hash_length` | `7`                            | Довжина хешу коміта.                                                                                         |
| `format`             | `'[\($hash$tag\)]($style) '` | Формат модуля.                                                                                               |
| `style`              | `'bold green'`                 | Стиль модуля.                                                                                                |
| `only_detached`      | `true`                         | Показувати хеш коміту тільки коли `HEAD` у відʼєднаному стані                                                                |
| `tag_disabled`       | `true`                         | Вимикає показ теґів в модулі `git_commit`.                                                                   |
| `tag_max_candidates` | `0`                            | Впродовж скількох комітів показувати теґ. Стандартно дозволяється тільки безпосередній збіг. |
| `tag_symbol`         | `' 🏷  '`                      | Символ теґу                                                                                                                  |
| `disabled`           | `false`                        | Вимикає модуль `git_commit`.                                                                                 |

### Змінні

| Змінна  | Приклад   | Опис                                                                 |
| ------- | --------- | -------------------------------------------------------------------- |
| hash    | `b703eb3` | Хэш коміту git                                                       |
| tag     | `v1.0.0`  | Назва теґу, якщо увімкнено показ інформації про теґ. |
| style\* |           | Віддзеркалює значення параметра `style`                              |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git State

The `git_state` module will show in directories which are part of a git
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc. Якщо є інформація про прогрес (наприклад, REBASING 3/10), ця інформація також буде показана.

### Параметри

| Параметр       | Стандартно                                                      | Опис                                                                                         |
| -------------- | --------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | Формат рядка під час процесу `rebase`.                                       |
| `merge`        | `'MERGING'`                                                     | Формат рядка під час процесу `merge`.                                        |
| `revert`       | `'REVERTING'`                                                   | Формат рядка під час процесу `revert`.                                       |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | Формат рядка під час процесу `cherry-pick`.                                  |
| `bisect`       | `'BISECTING'`                                                   | Формат рядка під час процесу `bisect`.                                       |
| `am`           | `'AM'`                                                          | Формат рядка під час процесу  `apply-mailbox` (`git am`). |
| `am_or_rebase` | `'AM/REBASE'`                                                   | Формат рядка під час процесу  `apply-mailbox` або  `rebase`.                 |
| `style`        | `'bold yellow'`                                                 | Стиль модуля.                                                                |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Формат модуля.                                                               |
| `disabled`     | `false`                                                         | Вимикає модуль `git_state`.                                                  |

### Змінні

| Змінна                                | Приклад    | Опис                                    |
| ------------------------------------- | ---------- | --------------------------------------- |
| state                                 | `REBASING` | Поточний стан репозиторію               |
| progress_current | `1`        | Прогрес поточної операції               |
| progress_total   | `2`        | Загальний прогрес операції              |
| style\*                               |            | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Git Metrics

Модуль `git_metrics` покаже кількість доданих та видалених рядків у поточному репозиторії git.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр             | Стандартно                                                   | Опис                                                         |
| -------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| `added_style`        | `'bold green'`                                               | Стиль для показу кількості доданих рядків.   |
| `deleted_style`      | `'bold red'`                                                 | Стиль для показу кількості видалених рядків. |
| `only_nonzero_diffs` | `true`                                                       | Показувати стан лише для змінених елементів. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Формат модуля.                               |
| `disabled`           | `true`                                                       | Вимикає модуль `git_metrics`.                |
| `ignore_submodules`  | `false`                                                      | Ігнорувати зміни в субмодулях                                |

### Змінні

| Змінна                               | Приклад | Опис                                            |
| ------------------------------------ | ------- | ----------------------------------------------- |
| added                                | `1`     | Поточна кількість доданих рядків                |
| deleted                              | `2`     | Поточна кількість видалених рядків              |
| added_style\*   |         | Віддзеркалює значення параметра `added_style`   |
| deleted_style\* |         | Віддзеркалює значення параметра `deleted_style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git Status

Модуль `git_status` показує символ, що описує стан репозиторію в поточній теці.

> [!TIP]
> The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment.
> You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

### Параметри

| Параметр               | Стандартно                                      | Опис                                                                                                                                                       |
| ---------------------- | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`               | `'([\[$all_status$ahead_behind\]]($style) )'` | Стандартний формат `git_status`                                                                                                                            |
| `conflicted`           | `'='`                                           | Формат, що показується, коли у цій гілці виникають конфлікти злиття.                                                                       |
| `ahead`                | `'⇡'`                                           | Формат, що показується, коли ця гілка випереджає гілку, що відстежується.                                                                  |
| `behind`               | `'⇣'`                                           | Формат, що показується, коли ця гілка позаду гілки, що відстежується.                                                                      |
| `diverged`             | `'⇕'`                                           | Формат, що показується, коли ця гілка відхиляється гілки, що відстежується.                                                                |
| `up_to_date`           | `''`                                            | Формат, що показується, коли ця гілка відповідає гілці, що відстежується.                                                                  |
| `untracked`            | `'?'`                                           | Формат, що показується, коли в робочій теці є невідстежувані файли.                                                                        |
| `stashed`              | `'\$'`                                         | Формат, що показується, коли для локального репозиторію існує stash.                                                                       |
| `modified`             | `'!'`                                           | Формат, що показується при змінах у файлах у робочій теці.                                                                                 |
| `staged`               | `'+'`                                           | Формат, що показується, коли до stage додано новий файл.                                                                                   |
| `renamed`              | `'»'`                                           | Формат, що показується, коли до stage додано перейменований файл.                                                                          |
| `deleted`              | `'✘'`                                           | Формат, що показується, коли до stage додано вилучений файл.                                                                               |
| `typechanged`          | `""`                                            | Формат, що показується, коли до stage додано файл зі зміненим типом.                                                                       |
| `style`                | `'bold red'`                                    | Стиль модуля.                                                                                                                              |
| `ignore_submodules`    | `false`                                         | Ігнорувати зміни в субмодулях.                                                                                                             |
| `worktree_added`       | `""`                                            | Формат, що показується при додаванні нового файлу до робочої теки.                                                                         |
| `worktree_deleted`     | `""`                                            | Формат, що показується при вилученні файлу з робочої теки.                                                                                 |
| `worktree_modified`    | `""`                                            | Формат, що показується при зміні файлу в робочій теці.                                                                                     |
| `worktree_typechanged` | `""`                                            | Формат, що показується, коли в робочій теці файлу було змінено тип.                                                                        |
| `index_added`          | `""`                                            | Формат, що показується, коли до stage додано новий файл.                                                                                   |
| `index_deleted`        | `""`                                            | Формат, що показується, коли файл було вилучено з stage.                                                                                   |
| `index_modified`       | `""`                                            | Формат, що показується, коли файл в stage було змінено.                                                                                    |
| `index_typechanged`    | `""`                                            | Формат, що показується, коли до stage додано файл зі зміненим типом.                                                                       |
| `disabled`             | `false`                                         | Вимикає модуль `git_status`.                                                                                                               |
| `windows_starship`     |                                                 | Використовуйте цей (Linux) шлях до виконуваного файлу у Windows для показу `git_status` у випадку шляхів Windows у WSL. |
| `use_git_executable`   | `false`                                         | Не використовуйте `gitoxide` для обчислення статусу, натомість використовуйте виконуваний файл `git`.                                      |

### Змінні

Наступні змінні можуть бути використані у `format`:

| Змінна                 | Опис                                                                                                                |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `all_status`           | Скорочення для  `$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`.        |
| `ahead_behind`         | Показує `diverged`, `ahead`, `behind` чи `up_to_date` в залежності від поточного стану репозиторію. |
| `conflicted`           | Показує `conflicted`, коли поточна гілка має конфлікт злиття.                                       |
| `untracked`            | Показує `untracked` коли в робочій теці є файли що ще не включені до відстеження у репозиторії.     |
| `stashed`              | Показує `stashed` за наявності stash у локальному репозиторії.                                      |
| `modified`             | Показує `modified` коли в робочій теці є змінені файли.                                             |
| `staged`               | Показує `staged`, коли нові фали були додані до простору staging.                                   |
| `renamed`              | Показує `renamed` коли перейменовані файли було додано до простору staging.                         |
| `deleted`              | Показує `deleted` коли інформація про видалення файлів була додана до простору staging.             |
| `typechanged`          | Показує `typechanged` коли інформація про файл була змінена у просторі staging.                     |
| `worktree_added`       | Показує `worktree_added`, коли новий фал будо додано до робочої теки.                               |
| `worktree_deleted`     | Показує `worktree_deleted`, коли фал було вилучено з робочої теки.                                  |
| `worktree_modified`    | Показує `worktree_modified `, коли фал було змінено в робочій теці.                                 |
| `worktree_typechanged` | Показує `worktree_typechanged`, коли тип файлу було змінено в робочій теці.                         |
| `index_added`          | Показує `index_added`, коли новий файл було додано до простору staging.                             |
| `index_deleted`        | Показує `index_deleted`, коли файл було вилучено з stage.                                           |
| `index_modified`       | Показує `index_modified`, коли файл було змінено в stage.                                           |
| `index_typechanged`    | Показує `index_typechanged`, коли тип файлу було змінено в stage.                                   |
| style\*                | Віддзеркалює значення параметра `style`                                                                             |

\*: Ця змінна може бути використана лише як частина стилю рядка

Наступні змінні можуть бути використані у `diverged`:

| Змінна         | Опис                                                             |
| -------------- | ---------------------------------------------------------------- |
| `ahead_count`  | Кількість комітів на яку поточна гілка випереджає відстежувану   |
| `behind_count` | Кількість комітів на яку поточна гілка відстає від відстежуваної |

Наступні змінні можуть використовуватись в `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed`, `deleted`, `typechanged`, `worktree_added`, `worktree_deleted`, `worktree_modified`, `worktree_typechanged`, `index_added`, `index_deleted`, `index_modified` та `index_typechanged`:

| Змінна  | Опис                     |
| ------- | ------------------------ |
| `count` | Показує кількість файлів |

### Приклад

```toml
# ~/.config/starship.toml

[git_status]
conflicted = '🏳'
ahead = '🏎💨'
behind = '😰'
diverged = '😵'
up_to_date = '✓'
untracked = '🤷'
stashed = '📦'
modified = '📝'
staged = '[++\($count\)](green)'
renamed = '👅'
deleted = '🗑'
```

Показує кількість комітів ahead/behind

```toml
# ~/.config/starship.toml

[git_status]
ahead = '⇡${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
behind = '⇣${count}'
```

Використання виконавчого файлу Windows Starship у Windows у шляхах WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```

## Gleam

The `gleam` module shows the currently installed version of [Gleam](https://gleam.run/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `gleam.toml`
- Поточна тека містить файли з розширенням `.gleam`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'⭐ '`                               | Формат рядка, що представляє символ Gleam.                                               |
| `detect_extensions` | `['gleam']`                          | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['gleam.toml']`                     | Які імена файлів мають запускати цей модуль.                                             |
| `style`             | `'bold #FFAFF3'`                     | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Вимикає модуль `gleam`.                                                                  |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v1.0.0` | Версія `gleam`                           |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `go.mod`
- Поточна тека містить файл `go.sum`
- Поточна тека містить файл `go.work`
- Поточна тека містить файл `glide.yaml`
- Поточна тека містить файл `Gopkg.yml`
- Поточна тека містить файл `Gopkg.lock`
- Поточна тека містить файл `.go-version`
- Поточна тека містить теку `Godeps`
- Поточна тека містить файл `.go`

### Параметри

| Параметр            | Стандартно                                                                                | Опис                                                                                                                    |
| ------------------- | ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | Формат модуля.                                                                                          |
| `version_format`    | `'v${raw}'`                                                                               | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch`                |
| `символ`            | `'🐹 '`                                                                                   | Формат рядка, що представляє символ Go.                                                                 |
| `detect_extensions` | `['go']`                                                                                  | Які розширення повинні запускати цей модуль.                                                            |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Які імена файлів мають запускати цей модуль.                                                            |
| `detect_folders`    | `['Godeps']`                                                                              | В яких теках цей модуль має запускатись.                                                                |
| `style`             | `'bold cyan'`                                                                             | Стиль модуля.                                                                                           |
| `not_capable_style` | `'bold red'`                                                                              | Стиль модуля, коли директиви go з файлу go.mod не збігаються з встановленою версією Go. |
| `disabled`          | `false`                                                                                   | Вимикає модуль `golang`.                                                                                |

### Змінні

| Змінна                           | Приклад   | Опис                                                                                                                                                                        |
| -------------------------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version                          | `v1.12.1` | Версія `go`                                                                                                                                                                 |
| mod_version | `1.16`    | `go` version requirement as set in the go directive of `go.mod`. Will only show if the version requirement does not match the `go` version. |
| symbol                           |           | Віддзеркалює значення параметра `symbol`                                                                                                                                    |
| style\*                          |           | Віддзеркалює значення параметра `style`                                                                                                                                     |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

### Використання `mod_version`

```toml
# ~/.config/starship.toml

[golang]
format = 'via [$symbol($version )($mod_version )]($style)'
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment.
Модуль буде показано, коли ви перебуваєте в середовищі guix-shell.

### Параметри

| Параметр   | Стандартно                 | Опис                                                            |
| ---------- | -------------------------- | --------------------------------------------------------------- |
| `format`   | `'via [$symbol]($style) '` | Формат модуля.                                  |
| `символ`   | `'🐃 '`                    | Формат рядка, що представляє символ guix-shell. |
| `style`    | `'yellow bold'`            | Стиль модуля.                                   |
| `disabled` | `false`                    | Вимикає модуль `guix_shell`.                    |

### Змінні

| Змінна  | Приклад | Опис                                     |
| ------- | ------- | ---------------------------------------- |
| symbol  |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

Модуль `gradle` показує версію [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) що використовується в теці проєкту.

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить теку `gradle/wrapper/gradle-wrapper.properties`.
- Поточна тека містить файли `.gradle` або `.gradle.kts`.

Модуль `gradle` може лише зчитувати версію Gradle Wrapper з вашого файлу налаштувань, ми не запускаємо на виконання вашу обгортку з міркувань безпеки.

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🅶 '`                              | Формат рядка, що представляє символ Gradle.                                              |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `['gradle']`                         | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold bright-cyan'`                 | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Вимикає модуль `gradle`.                                                                 |
| `recursive`         | `false`                              | Дозволяє рекурсивний пошук теки `gradle`.                                                |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v7.5.1` | Версія `gradle`                          |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

## Haskell

Модуль `haskell` знаходить поточну версію GHC та/або Stack snapshot.

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `stack.yaml`
- Поточна тека містить файли `.hs`, `.cabal` або `.hs-boot`

### Параметри

| Параметр            | Стандартно                           | Опис                                                         |
| ------------------- | ------------------------------------ | ------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                               |
| `символ`            | `'λ '`                               | Формат рядка, що представляє символ Haskell                  |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Які розширення повинні запускати цей модуль. |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Які імена файлів мають запускати цей модуль. |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.     |
| `style`             | `'bold purple'`                      | Стиль модуля.                                |
| `disabled`          | `false`                              | Вимикає модуль `haskell`.                    |

### Змінні

| Змінна                             | Приклад     | Опис                                                                                    |
| ---------------------------------- | ----------- | --------------------------------------------------------------------------------------- |
| version                            |             | `ghc_version` або `snapshot` в залежності від того, чи є поточний проєкт проєктом Stack |
| snapshot                           | `lts-18.12` | Поточний обраний Stack snapshot                                                         |
| ghc\_version | `9.2.1`     | Встановлена версія GHC                                                                  |
| symbol                             |             | Віддзеркалює значення параметра `symbol`                                                |
| style\*                            |             | Віддзеркалює значення параметра `style`                                                 |

\*: Ця змінна може бути використана лише як частина стилю рядка

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- Поточна тека містить теку `.haxelib` або `haxe_libraries`
- Поточна тека містить файли `.hx` або `.hxml`

### Параметри

| Параметр            | Стандартно                                                                                      | Опис                                                                                                     |
| ------------------- | ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                                                                     | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | В яких теках цей модуль має запускатись.                                                 |
| `символ`            | `'⌘ '`                                                                                          | Формат рядка, що представляє символ Haxe.                                                |
| `style`             | `'bold fg:202'`                                                                                 | Стиль модуля.                                                                            |
| `disabled`          | `false`                                                                                         | Вимикає модуль `haxe`.                                                                   |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v4.2.5` | Версія `haxe`                            |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `helmfile.yaml`
- Поточна тека містить файл `Chart.yaml`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `символ`            | `'⎈ '`                               | Формат рядка, що представляє символ Helm.                                                |
| `style`             | `'bold white'`                       | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Вимикає модуль `helm`.                                                                   |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v3.1.1` | Версія `helm`                            |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Hostname

Модуль `hostname` показує назву хосту.

### Параметри

| Параметр          | Стандартно                             | Опис                                                                                                                                                                                 |
| ----------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only`        | `true`                                 | Показувати назву хоста лише при підключенні через SSH.                                                                                                               |
| `ssh_symbol`      | `'🌐 '`                                | Формат рядка для показу символу підключення до SSH-сеансу.                                                                                                           |
| `trim_at`         | `'.'`                                  | Рядок, у якому назва хоста буде обрізано після першого збігу. `'.'` will stop after the first dot. `''` will disable any truncation. |
| `detect_env_vars` | `[]`                                   | Які змінні середовища повинні запускати цей модуль.                                                                                                                  |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | Формат модуля.                                                                                                                                                       |
| `style`           | `'bold dimmed green'`                  | Стиль модуля.                                                                                                                                                        |
| `disabled`        | `false`                                | Вимикає модуль `hostname`.                                                                                                                                           |
| `aliases`         | `{}`                                   | Переводить системні імена хостів у щось інше. If `trim_at` is specified, only the first part will be matched and replaced.                           |

### Змінні

| Змінна                          | Приклад    | Опис                                                          |
| ------------------------------- | ---------- | ------------------------------------------------------------- |
| hostname                        | `computer` | Назва хосту                                                   |
| style\*                         |            | Віддзеркалює значення параметра `style`                       |
| ssh_symbol | `'🌏 '`    | Символ, який буде показаний, під час підʼєднання до SSH сесії |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклади

#### Завжди показувати hostname

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

#### Приховувати hostname для віддалених сеансів tmux

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
detect_env_vars = ['!TMUX', 'SSH_CONNECTION']
disabled = false
```

#### Замінити імʼя хосту псевдонімом

```toml
# ~/.config/starship.toml
[hostname]
aliases = { "Max's MacBook Pro" = "home" }
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- В поточній теці містяться файли `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot` або `.sdkmanrc`
- Поточна тека містить файли з розширеннями `.java`, `.class`, `.gradle`, `.jar`, `.clj` або `.cljc`

### Параметри

| Параметр            | Стандартно                                                                                                            | Опис                                                                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                                                                                           | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                                                                                                  | В яких теках цей модуль має запускатись.                                                 |
| `символ`            | `'☕ '`                                                                                                                | Формат рядка, що представляє символ Java                                                                 |
| `style`             | `'red dimmed'`                                                                                                        | Стиль модуля.                                                                            |
| `disabled`          | `false`                                                                                                               | Вимикає модуль `java`.                                                                   |

### Змінні

| Змінна  | Приклад | Опис                                     |
| ------- | ------- | ---------------------------------------- |
| version | `v14`   | Версія `java`                            |
| symbol  |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## Jobs

The `jobs` module shows the current number of jobs running.
Модуль показуватиметься лише у випадку наявності фонових завдань.
The module will show the number of jobs running if there are at least
2 jobs, or more than the `number_threshold` config value, if it exists.
The module will show a symbol if there is at least 1 job, or more than the
`symbol_threshold` config value, if it exists. You can set both values
to 0 in order to _always_ show the symbol and number of jobs, even if there are
0 jobs running.

Стандартний функціонал:

- 0 завдань -> нічого не показується.
- 1 завдання -> показується `symbol`.
- 2 чи більше завдань -> показується `symbol` + `число`.

> [!WARNING] Попередження Цей модуль не підтримується на tcsh.

> [!WARNING]
> The `threshold` option is deprecated, but if you want to use it,
> the module will show the number of jobs running if there is more than 1 job, or
> more than the `threshold` config value, if it exists. If `threshold` is set to 0,
> then the module will also show when there are 0 jobs running.

### Параметри

| Параметр           | Стандартно                    | Опис                                                                                            |
| ------------------ | ----------------------------- | ----------------------------------------------------------------------------------------------- |
| `threshold`\*      | `1`                           | Показувати кількість завдань, якщо вони перевищують значення.                   |
| `symbol_threshold` | `1`                           | Показувати символ `symbol`, якщо кількість завдань не менше `symbol_threshold`. |
| `number_threshold` | `2`                           | Показувати кількість завдань, якщо їх кількість не менша за `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | Формат модуля.                                                                  |
| `символ`           | `'✦'`                         | Змінна для визначення символу `symbol`.                                         |
| `style`            | `'bold blue'`                 | Стиль модуля.                                                                   |
| `disabled`         | `false`                       | Вимикає модуль `jobs`.                                                          |

\*: Цей параметр застарів, використовуйте параметри `number_threshold` та `symbol_threshold` замість цього.

### Змінні

| Змінна  | Приклад | Опис                                     |
| ------- | ------- | ---------------------------------------- |
| number  | `1`     | Кількість завдань                        |
| symbol  |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклади

```toml
# ~/.config/starship.toml

[jobs]
символ = '+ '
number_threshold = 4
symbol_threshold = 0
```

#### Зміна процесу групування поведінки у fish

When using the Fish shell, Starship counts **job groups** instead of individual process IDs by default. Це запобігає надмірному підрахунку, коли конвеєр має кілька процесів, але тільки одну призупинену групу. Щоб повернутися до старого підрахунку на основі PID, додайте наступне до конфігурації оболонки:

```fish
set -g __starship_fish_use_job_groups "false"
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `Project.toml`
- Поточна тека містить файл `Manifest.toml`
- Поточна тека містить файл `.jl`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `символ`            | `'ஃ '`                               | Формат рядка, що представляє символ Julia.                                               |
| `style`             | `'bold purple'`                      | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Вимикає модуль `julia`.                                                                  |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v1.4.0` | Версія `julia`                           |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файли, `.kt` або `.kts`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                      |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                            |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch`  |
| `detect_extensions` | `['kt', 'kts']`                      | Які розширення повинні запускати цей модуль.                                              |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                                              |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                  |
| `символ`            | `'🅺 '`                              | Формат рядка, що представляє символ Kotlin.                                               |
| `style`             | `'bold blue'`                        | Стиль модуля.                                                                             |
| `kotlin_binary`     | `'kotlin'`                           | Налаштовує бінарний файл kotlin, який Starship буде використовувати для отримання версії. |
| `disabled`          | `false`                              | Вимикає модуль `kotlin`.                                                                  |

### Змінні

| Змінна  | Приклад   | Опис                                     |
| ------- | --------- | ---------------------------------------- |
| version | `v1.4.21` | Версія `kotlin`                          |
| symbol  |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[kotlin]
symbol = '🅺 '
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = 'kotlinc'
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file.
The namespace needs to be set in the kubeconfig file, this can be done via
`kubectl config set-context starship-context --namespace astronaut`.
Similarly, the user and cluster can be set with `kubectl config set-context starship-context --user starship-user`
and `kubectl config set-context starship-context --cluster starship-cluster`.
If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.
>
> Коли модуль увімкнено, він завжди буде активним, якщо будь-який з параметрів `detect_env_vars`, `detect_extensions`, `detect_files` або `detect_folders` встановлені,  модуль буде активним тільки в теках, що відповідають умовам або якщо одна зі змінних середовища була встановлена.

### Параметри

> [!WARNING]
> The `context_aliases` and `user_aliases` options are deprecated. Use `contexts` and the corresponding `context_alias`
> and `user_alias` options instead.

| Параметр            | Стандартно                                           | Опис                                                                      |
| ------------------- | ---------------------------------------------------- | ------------------------------------------------------------------------- |
| `символ`            | `'☸ '`                                               | Символ, що показується перед Кластером.                   |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | Формат модуля.                                            |
| `style`             | `'cyan bold'`                                        | Стиль модуля.                                             |
| `context_aliases`\* | `{}`                                                 | Таблиця контекстних псевдонімів.                          |
| `user_aliases`\*    | `{}`                                                 | Таблиця псевдонімів користувача.                          |
| `detect_extensions` | `[]`                                                 | Які розширення повинні запускати цей модуль.              |
| `detect_files`      | `[]`                                                 | Які імена файлів мають запускати цей модуль.              |
| `detect_folders`    | `[]`                                                 | В яких теках цей модуль має запускатись.                  |
| `detect_env_vars`   | `[]`                                                 | Які змінні середовища повинні запускати цей модуль                        |
| `contexts`          | `[]`                                                 | Кастомізовані стилі та символи для конкретних контекстів. |
| `disabled`          | `true`                                               | Вимикає модуль `kubernetes`.                              |

\*: Цей параметр є застарілими, додайте `contexts`, відповідно, `context_alias` та `user_alias`, натомість.

Для налаштування стилю модуля для конкретних середовищ використовуйте наступну конфігурацію як частину списку `contexts`:

| Змінна            | Опис                                                                                                                                      |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `context_pattern` | **Обовʼязково** Регулярний вираз, що повертає збіг з назвою поточного контексту Kubernetes.                               |
| `user_pattern`    | Регулярний вираз, що відповідає поточному імені користувача Kubernetes.                                                   |
| `context_alias`   | Псевдонім контексту для показу замість назви повного контексту.                                                           |
| `user_alias`      | Псевдонім користувача для показу замість повного імені користувача.                                                       |
| `style`           | Стиль для модуля, при використанні цього контексту. Якщо не вказано, використовуватиметься стиль модуля.  |
| `символ`          | Символ для модуля при використанні цього контексту. Якщо не вказано, використовуватиметься символ модуля. |

Note that all regular expression are anchored with `^<pattern>$` and so must match the whole string. The `*_pattern`
regular expressions may contain capture groups, which can be referenced in the corresponding alias via `$name` and `$N`
(see example below and the
[rust Regex::replace() documentation](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

### Змінні

| Змінна    | Приклад              | Опис                                               |
| --------- | -------------------- | -------------------------------------------------- |
| context   | `starship-context`   | Поточна назва kubernetes context                   |
| namespace | `starship-namespace` | Якщо встановлено, поточний простір імен kubernetes |
| user      | `starship-user`      | Якщо встановлено, поточний користувач kubernetes   |
| cluster   | `starship-cluster`   | Якщо встановлено, поточний кластер kubernetes      |
| symbol    |                      | Віддзеркалює значення параметра `symbol`           |
| style\*   |                      | Віддзеркалює значення параметра `style`            |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
contexts = [
  { context_pattern = "dev.local.cluster.k8s", style = "green", symbol = "💔 " },
]
```

Показує модуль лише у теках, що містять файл `k8s`.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Kubernetes Context спец налаштування

Параметр `contexts` використовується для налаштування того, як виглядає назва контексту Kubernetes (стиль та символ), якщо назва збігається з визначеною регулярним виразом.

```toml
# ~/.config/starship.toml

[[kubernetes.contexts]]
# стиль "bold red" + типовий символ, коли назва поточного контексту Kubernetes збігається з "production" *та* поточний користувач
# збігається з "admin_user"
context_pattern = "production"
user_pattern = "admin_user"
style = "bold red"
context_alias = "prod"
user_alias = "admin"

[[kubernetes.contexts]]
# стиль "green" + інший символ, коли назва поточного контексту Kubernetes містить openshift
context_pattern = ".*openshift.*"
style = "green"
symbol = "💔 "
context_alias = "openshift"

[[kubernetes.contexts]]
# Використання груп
# Контекст з GKE, AWS та інших хмарних постачальників зазвичай має додаткову інформацію, наприклад регіон/зону.
# Наступний елемент збігається з форматом GKE format (`gke_projectname_zone_cluster-name`)
# та змінює кожний відповідний kube context на більш зрозумілий формат (`gke-cluster-name`):
context_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
context_alias = "gke-$cluster"
```

## Line Break

Модуль `line_break` розділяє командний рядок на два рядки.

### Параметри

| Параметр   | Стандартно | Опис                                                                       |
| ---------- | ---------- | -------------------------------------------------------------------------- |
| `disabled` | `false`    | Вимикає модуль `line_break`, перемикає вивід в один рядок. |

### Приклад

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

Модуль `localip` показує IPv4 адресу основного мережевого інтерфейсу.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр   | Стандартно                | Опис                                                                 |
| ---------- | ------------------------- | -------------------------------------------------------------------- |
| `ssh_only` | `true`                    | Показувати IP адресу лише при підключенні через SSH. |
| `format`   | `'[$localipv4]($style) '` | Формат модуля.                                       |
| `style`    | `'bold yellow'`           | Стиль модуля.                                        |
| `disabled` | `true`                    | Вимикає модуль `localip`.                            |

### Змінні

| Змінна    | Приклад                                                      | Опис                                    |
| --------- | ------------------------------------------------------------ | --------------------------------------- |
| localipv4 | 192.168.1.13 | Містить основну адресу IPv4             |
| style\*   |                                                              | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `.lua-version`
- Поточна тека містить теку `lua`
- Поточна тека містить файли з розширенням `.lua`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🌙 '`                              | Формат рядка, що представляє символ Lua.                                                 |
| `detect_extensions` | `['lua']`                            | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['.lua-version']`                   | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `['lua']`                            | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold blue'`                        | Стиль модуля.                                                                            |
| `lua_binary`        | `'lua'`                              | Налаштовує бінарний файл lua, який Starship буде використовувати для отримання версії.   |
| `disabled`          | `false`                              | Вимикає модуль `lua`.                                                                    |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v5.4.0` | Версія `lua`                             |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Maven

The `maven` module indicates the presence of a Maven project in the current directory. If the [Maven Wrapper](https://maven.apache.org/wrapper/) is enabled, the Maven version will be parsed from `.mvn/wrapper/maven-wrapper.properties` and shown.

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `pom.xml`.
- Поточна тека містить файл `.mvn/wrapper/maven-wrapper.properties`.

Якщо ви використовуєте альтернативний синтаксис POM (наприклад, `pom.hocon`), додайте його імʼя файлу до `detect_files`.

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🅼 '`                              | Формат рядка, що представляє символ Maven.                                               |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['pom.xml']`                        | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `['.mvn']`                           | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold bright-cyan'`                 | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Вимикає модуль `maven`.                                                                  |
| `recursive`         | `false`                              | Дозволяє рекурсивний пошук теки `.mvn`.                                                  |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v3.2.0` | Версія `maven`                           |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

## Використання памʼяті {#memory-usage}

Модуль `memory_usage` показує поточне використання оперативної памʼяті та памʼяті файлу підкачки.

Стандартно використання файлу підкачки показується якщо його розмір не є нульовим.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр    | Стандартно                                     | Опис                                                                             |
| ----------- | ---------------------------------------------- | -------------------------------------------------------------------------------- |
| `threshold` | `75`                                           | Приховати використання памʼяті, якщо не перевищено цей відсоток. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | Формат модуля.                                                   |
| `символ`    | `'🐏'`                                         | Символ, який знаходиться перед значенням використання памʼяті.   |
| `style`     | `'bold dimmed white'`                          | Стиль модуля.                                                    |
| `disabled`  | `true`                                         | Вимикає модуль `memory_usage`.                                   |

### Змінні

| Змінна                            | Приклад       | Опис                                                     |
| --------------------------------- | ------------- | -------------------------------------------------------- |
| ram                               | `31GiB/65GiB` | Використана/загальна памʼять.            |
| ram_pct      | `48%`         | Відсоток завантаженості памʼяті системи. |
| swap\*\*                          | `1GiB/4GiB`   | Розмір файлу підкачки.                   |
| swap_pct\*\* | `77%`         | Процент завантаженості файлу підкачки.   |
| symbol                            | `🐏`          | Віддзеркалює значення параметра `symbol`                 |
| style\*                           |               | Віддзеркалює значення параметра `style`                  |

\*: Цю змінну можна використовувати лише як частину стилю рядка \*\*: Інформація щодо файлів SWAP показується лише у разі наявності в поточній системі

### Приклад

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = ' '
style = 'bold dimmed green'
```

## Meson

Модуль `meson` показує поточний стан оточення розробки Meson.

Стандартно показується назва проєкту Meson, якщо встановлено `$MESON_DEVENV`.

### Параметри

| Параметр            | Стандартно                         | Опис                                                                                                                                           |
| ------------------- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Скорочує назву проєкту до `N` графем.                                                                                          |
| `truncation_symbol` | `'…'`                              | Символ, що використовується для позначення назви проєкту, який було скорочено. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | Формат модуля.                                                                                                                 |
| `символ`            | `'⬢ '`                             | Символ, який знаходиться перед назвою проєкту.                                                                                 |
| `style`             | `'blue bold'`                      | Стиль модуля.                                                                                                                  |
| `disabled`          | `false`                            | Вимкнути модуль `meson`.                                                                                                       |

### Змінні

| Змінна  | Приклад    | Опис                                     |
| ------- | ---------- | ---------------------------------------- |
| project | `starship` | Поточна назва проєкту з Meson            |
| symbol  | `🐏`       | Віддзеркалює значення параметра `symbol` |
| style\* |            | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```

## Mercurial Branch

Модуль `hg_branch` показує активну гілку та вершину репозиторію у вашій поточній теці.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр            | Стандартно                                | Опис                                                                                                  |
| ------------------- | ----------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `символ`            | `' '`                                    | Символ, що використовується перед закладкою hg чи назвою гілки у вашій поточній теці. |
| `style`             | `'bold purple'`                           | Стиль модуля.                                                                         |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | Формат модуля.                                                                        |
| `truncation_length` | `2^63 - 1`                                | Скорочує назву гілки/вершини до `N` графем                                                            |
| `truncation_symbol` | `'…'`                                     | Символ, що використовується для позначення назви гілки, яка була скорочена.           |
| `disabled`          | `true`                                    | Вимикає модуль `hg_branch`.                                                           |

### Змінні

| Змінна  | Приклад   | Опис                                     |
| ------- | --------- | ---------------------------------------- |
| branch  | `master`  | Поточна гілка mercurial                  |
| topic   | `feature` | Поточна вершина mercurial                |
| symbol  |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Mercurial State

Модуль `hg_state` показуватиметься в теках, які є частиною сховища mercurial, і в яких виконується операція, наприклад: _REBASING_, _BISECTING_ тощо.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр     | Стандартно                  | Опис                                                       |
| ------------ | --------------------------- | ---------------------------------------------------------- |
| `merge`      | `'MERGING'`                 | Формат рядка під час процесу `merge`.      |
| `rebase`     | `'REBASING'`                | Формат рядка під час процесу `rebase`.     |
| `update`     | `'UPDATING'`                | Формат рядка під час процесу `update`.     |
| `bisect`     | `'BISECTING'`               | Формат рядка під час процесу `bisect`.     |
| `shelve`     | `'SHELVING'`                | Формат рядка під час процесу `shelve`.     |
| `graft`      | `'GRAFTING'`                | Формат рядка під час процесу `graft`.      |
| `transplant` | `'TRANSPLANTING'`           | Формат рядка під час процесу `transplant`. |
| `histedit`   | `'HISTEDITING'`             | Формат рядка під час процесу `histedit`.   |
| `style`      | `'bold yellow'`             | Стиль модуля.                              |
| `format`     | `'\([$state]($style)\) '` | Формат модуля.                             |
| `disabled`   | `true`                      | Вимикає модуль `hg_state`.                 |

### Змінні

| Змінна                                | Приклад    | Опис                                    |
| ------------------------------------- | ---------- | --------------------------------------- |
| state                                 | `REBASING` | Поточний стан репозиторію               |
| progress_current | `1`        | Прогрес поточної операції               |
| progress_total   | `2`        | Загальний прогрес операції              |
| style\*                               |            | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

## Mise

Модуль `mise` показує поточний стан mise, про який повідомляє запуск `mise doctor`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр            | Стандартно                                                           | Опис                                                                      |
| ------------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `символ`            | `'mise '`                                                            | Символ, який використовується перед показом стану _mise_. |
| `style`             | `'bold purple'`                                                      | Стиль модуля.                                             |
| `format`            | `'on [$symbol$health]($style) '`                                     | Формат модуля.                                            |
| `detect_extensions` | `[]`                                                                 | Які розширення повинні запускати цей модуль.              |
| `detect_files`      | `['mise.toml', 'mise.local.toml', '.mise.toml', '.mise.local.toml']` | Які імена файлів мають запускати цей модуль.              |
| `detect_folders`    | `['.mise']`                                                          | В яких теках цей модуль має запускатись.                  |
| `healthy_symbol`    | `healthy`                                                            | Повідомлення, яке показується, коли _mise_ справний.      |
| `unhealthy_symbol`  | `unhealthy`                                                          | Повідомлення, яке показується, коли _mise_ несправний.    |
| `disabled`          | `true`                                                               | Вимикає модуль `mise`.                                    |

### Змінні

| Змінна  | Приклад   | Опис                                     |
| ------- | --------- | ---------------------------------------- |
| health  | `healthy` | Стан справності _mise_                   |
| symbol  |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[mise]
health = 'ready'
```

## Mojo

Модуль `mojo` показує поточну версію встановленої мови програмування [Mojo](https://www.modular.com/mojo)

### Параметри

| Параметр            | Стандартно                            | Опис                                                         |
| ------------------- | ------------------------------------- | ------------------------------------------------------------ |
| `format`            | `'with [$symbol($version )]($style)'` | Формат модуля.                               |
| `символ`            | `'🔥 '`                               | Символ, який знаходиться перед версією Mojo. |
| `style`             | `'bold 208'`                          | Стиль модуля.                                |
| `disabled`          | `false`                               | Вимикає модуль `mojo`.                       |
| `detect_extensions` | `['mojo', '🔥']`                      | Які розширення повинні запускати цей модуль. |
| `detect_files`      | `[]`                                  | Які імена файлів мають запускати цей модуль. |
| `detect_folders`    | `[]`                                  | В яких теках цей модуль має запускатись.     |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `24.4.0` | Версія `mojo`                            |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )($hash )]($style)'
```

## NATS

Модуль `nats` показує назву поточного контексту [NATS](https://nats.io).

### Параметри

| Параметр   | Стандартно                 | Опис                                                                                                    |
| ---------- | -------------------------- | ------------------------------------------------------------------------------------------------------- |
| `символ`   | `'✉️ '`                    | Символ, що використовується перед контекстом NATS (типово порожнім). |
| `style`    | `'bold purple'`            | Стиль модуля.                                                                           |
| `format`   | `'[$symbol$name]($style)'` | Формат модуля.                                                                          |
| `disabled` | `false`                    | Вимикає модуль `nats`.                                                                  |

### Змінні

| Змінна  | Приклад     | Опис                                     |
| ------- | ----------- | ---------------------------------------- |
| name    | `localhost` | Назва контексту NATS                     |
| symbol  |             | Віддзеркалює значення параметра `symbol` |
| style\* |             | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## Мережевий простір імен {#network-namespace}

The `netns` module shows the current network namespace.
This uses `ip netns identify` to get the network namespace, so only network namespaces mounted at `/var/run/netns` will be detected.

### Параметри

| Параметр   | Стандартно                        | Опис                                                                                                          |
| ---------- | --------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`   | `'[$symbol \[$name\]]($style)'` | Формат модуля.                                                                                |
| `символ`   | `'🛜 '`                           | Символ, що використовується перед простором імен мережі (типово порожньо). |
| `style`    | `'blue bold dimmed'`              | Стиль модуля.                                                                                 |
| `disabled` | `false`                           | Вимикає модуль `netns`.                                                                       |

### Змінні

| Змінна  | Приклад    | Опис                                     |
| ------- | ---------- | ---------------------------------------- |
| name    | `my-netns` | Імʼя поточного простору імен мережі      |
| symbol  |            | Віддзеркалює значення параметра `symbol` |
| style\* |            | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `nim.cfg`
- Поточна тека містить файли з розширенням `.nim`
- Поточна тека містить файли з розширенням `.nims`
- Поточна тека містить файли з розширенням `.nimble`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля                                                                                            |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'👑 '`                              | Символ, який знаходиться перед версією Nim.                                              |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['nim.cfg']`                        | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold yellow'`                      | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Вимикає модуль `nim`.                                                                    |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v1.2.0` | Версія `nimc`                            |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment.
Модуль буде показано, коли ви перебуваєте в середовищі nix-shell.

### Параметри

| Параметр      | Стандартно                                     | Опис                                                                                               |
| ------------- | ---------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | Формат модуля.                                                                     |
| `символ`      | `'❄️ '`                                        | Формат рядка, що представляє символ nix-shell.                                     |
| `style`       | `'bold blue'`                                  | Стиль модуля.                                                                      |
| `impure_msg`  | `'impure'`                                     | Формат рядка, який показується, коли оболонка (impure) нечиста. |
| `pure_msg`    | `'pure'`                                       | Формат рядка, який показується, коли оболонка (pure) чиста.     |
| `unknown_msg` | `''`                                           | Формат рядка, у випадку, коли стан невідомий.                                      |
| `disabled`    | `false`                                        | Вимикає модуль `nix_shell`.                                                        |
| `heuristic`   | `false`                                        | Намагається визначити новий `nix shell`-стиль евристичними методами.               |

### Змінні

| Змінна  | Приклад | Опис                                     |
| ------- | ------- | ---------------------------------------- |
| state   | `pure`  | Стан nix-shell                           |
| name    | `lorri` | Назва nix-shell                          |
| level   | `1`     | Віддзеркалює значення параметра `symbol` |
| symbol  |         | Віддзеркалює значення параметра `style`  |
| style\* |         | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = '[impure shell](bold red)'
pure_msg = '[pure shell](bold green)'
unknown_msg = '[unknown shell](bold yellow)'
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `package.json`
- Поточна тека містить файл `.node-version`
- Поточна тека містить файл `.nvmrc`
- Поточна тека містить теку `node_modules`
- Поточна тека містить файли з розширеннями `.js`, `.mjs` або `.cjs`
- Поточна тека містить файли з розширеннями `.ts`, `.mts` чи `.cts`

Additionally, the module will be hidden by default if the directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `bunfig.toml`, `bun.lock`, or `bun.lockb` file, overriding the above conditions.

### Параметри

| Параметр            | Стандартно                                    | Опис                                                                                                                             |
| ------------------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | Формат модуля.                                                                                                   |
| `version_format`    | `'v${raw}'`                                   | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch`                         |
| `символ`            | `' '`                                        | Формат рядка, що представляє символ Node.js.                                                     |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | Які розширення повинні запускати цей модуль.                                                                     |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | Які імена файлів мають запускати цей модуль.                                                                     |
| `detect_folders`    | `['node_modules']`                            | В яких теках цей модуль має запускатись.                                                                         |
| `style`             | `'bold green'`                                | Стиль модуля.                                                                                                    |
| `disabled`          | `false`                                       | Вимикає модуль `nodejs`.                                                                                         |
| `not_capable_style` | `'bold red'`                                  | Стиль для модуля, коли версія рушія у package.json не відповідає версії Node.js. |

### Змінні

| Змінна                               | Приклад    | Опис                                                                                                                                                                                      |
| ------------------------------------ | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version                              | `v13.12.0` | Версія `node`                                                                                                                                                                             |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbol                               |            | Віддзеркалює значення параметра `symbol`                                                                                                                                                  |
| style\*                              |            | Віддзеркалює значення параметра `style`                                                                                                                                                   |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файли з розширенням `.opam` або теку `_opam`
- Поточна тека містить теку `esy.lock`
- Поточна тека містить файли `dune` або `dune-project`
- Поточна тека містить файли `jbuild` чи `jbuild-ignore`
- Поточна тека містить файл `.merlin`
- Поточна тека містить файли з розширеннями `.ml`, `.mli`, `.re` або `.rei`

### Параметри

| Параметр                  | Стандартно                                                                 | Опис                                                                                                     |
| ------------------------- | -------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | Формат рядка модуля.                                                                     |
| `version_format`          | `'v${raw}'`                                                                | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`                  | `'🐫 '`                                                                    | Символ, який знаходиться перед версією OCaml.                                            |
| `global_switch_indicator` | `''`                                                                       | Формат рядка для глобального перемикача OPAM.                                            |
| `local_switch_indicator`  | `'*'`                                                                      | Формат рядка для локального перемикача OPAM.                                             |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | В яких теках цей модуль має запускатись.                                                 |
| `style`                   | `'bold yellow'`                                                            | Стиль модуля.                                                                            |
| `disabled`                | `false`                                                                    | Вимикає модуль `ocaml`.                                                                  |

### Змінні

| Змінна                                | Приклад      | Опис                                                            |
| ------------------------------------- | ------------ | --------------------------------------------------------------- |
| version                               | `v4.10.0`    | Версія `ocaml`                                                  |
| switch_name      | `my-project` | Поточний перемикач OPAM                                         |
| switch_indicator |              | Віддзеркалює значення `indicator` для поточного перемикача OPAM |
| symbol                                |              | Віддзеркалює значення параметра `symbol`                        |
| style\*                               |              | Віддзеркалює значення параметра `style`                         |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Odin

The `odin` module shows the currently installed version of [Odin](https://odin-lang.org/). By default the module will be shown if the current directory contains a `.odin` file.

### Параметри

| Параметр            | Стандартно                           | Опис                                                         |
| ------------------- | ------------------------------------ | ------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                               |
| `show_commit`       | `false`                              | Показує коміт як частину версії.             |
| `символ`            | `'Ø '`                               | Символ, який знаходиться перед версією Odin. |
| `style`             | `'bold bright-blue'`                 | Стиль модуля.                                |
| `disabled`          | `false`                              | Вимикає модуль `odin`.                       |
| `detect_extensions` | `['odin']`                           | Які розширення повинні запускати цей модуль. |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль. |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.     |

### Змінні

| Змінна  | Приклад       | Опис                                     |
| ------- | ------------- | ---------------------------------------- |
| version | `dev-2024-03` | Версія `odin`                            |
| symbol  |               | Віддзеркалює значення параметра `symbol` |
| style\* |               | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool.
By default the module will be shown if the current directory contains a `.rego` file.

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🪖  '`                             | Формат рядка, що представляє символ OPA.                                                 |
| `detect_extensions` | `['rego']`                           | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold blue'`                        | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Вимикає модуль `opa`.                                                                    |

### Змінні

| Змінна  | Приклад   | Опис                                     |
| ------- | --------- | ---------------------------------------- |
| version | `v0.44.0` | Версія `opa`                             |
| symbol  |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module
only active when the `OS_CLOUD` env var is set, in which case it will read
`clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files).
щоб отримати поточний проєкт для використання.

### Параметри

| Параметр   | Стандартно                                      | Опис                                                                                          |
| ---------- | ----------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | Формат модуля.                                                                |
| `символ`   | `'☁️ '`                                         | Символ, який використовується під час показу перед поточною хмарою OpenStack. |
| `style`    | `'bold yellow'`                                 | Стиль модуля.                                                                 |
| `disabled` | `false`                                         | Вимикає модуль `openstack`.                                                   |

### Змінні

| Змінна  | Приклад | Опис                                     |
| ------- | ------- | ---------------------------------------- |
| cloud   | `corp`  | Поточна хмара OpenStack                  |
| project | `dev`   | Поточний проєкт OpenStack                |
| symbol  |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## OS

The `os` module shows the current operating system.
OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

> [!WARNING] Попередження Crate [os_info](https://lib.rs/crates/os_info), що використовується цим модулем, може бути не точним в деяких системах.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр   | Стандартно            | Опис                                                             |
| ---------- | --------------------- | ---------------------------------------------------------------- |
| `format`   | `'[$symbol]($style)'` | Формат модуля.                                   |
| `style`    | `'bold white'`        | Стиль модуля.                                    |
| `disabled` | `true`                | Вимикає модуль `os`.                             |
| `symbols`  |                       | Таблиця символів для кожної операційної системи. |

`symbols` allows you to define arbitrary symbols to display for each operating system type.
Типи операційних систем не визначені вашою конфігурацією, використовують стандартну таблицю символів, дивись нижче.
На цю мить усі операційні системи, що підтримуються модулем, перераховані нижче.
If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# Це таблиця стандартних символів.
[os.symbols]
AIX = "➿ "
Alpaquita = "🔔 "
AlmaLinux = "💠 "
Alpine = "🏔️ "
ALTLinux = "Ⓐ "
Amazon = "🙂 "
Android = "🤖 "
AOSC = "🐱 "
Arch = "🎗️ "
Artix = "🎗️ "
Bluefin = "🐟 "
CachyOS = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Elementary = "🍏 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Ios = "📱 "
InstantOS = "⏲️ "
Kali = "🐉 "
Linux = "🐧 "
Mabox = "📦 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
Nobara = "🎩 "
OpenBSD = "🐡 "
OpenCloudOS = "☁️ "
openEuler = "🦉 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
PikaOS = "🐤 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
RockyLinux = "💠 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Ultramarine = "🔷 "
Unknown = "❓ "
Uos = "🐲 "
Void = " "
Windows = "🪟 "
Zorin = "🔹 "
```

### Змінні

| Змінна   | Приклад      | Опис                                                                  |
| -------- | ------------ | --------------------------------------------------------------------- |
| symbol   | `🎗️`        | Поточний символ операційної системи з розширеного параметра `symbols` |
| name     | `Arch Linux` | Назва поточної операційної системи                                    |
| type     | `Arch`       | Тип поточної операційної системи                                      |
| codename |              | Поточна кодова назва операційної системи, за наявності                |
| edition  |              | Поточна редакція операційної системи, за наявності                    |
| version  |              | Поточна версія операційної системи, за наявності                      |
| style\*  |              | Віддзеркалює значення параметра `style`                               |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[os]
format = "on [($name )]($style)"
style = "bold blue"
disabled = false

[os.symbols]
Windows = " "
Arch = "Arch is the best! "
```

## Package Version

The `package` module is shown when the current directory is the repository for a
package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`,
`poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `galaxy`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) — версія пакунка `npm` отримується з `package.json` з поточної теки
- [**JSR**](https://jsr.io/) — версія пакунка `jsr` отримана з файлів `jsr.json`/`jsr.jsonc` чи `deno.json`/`deno.jsonc` у поточній теці
- [**Cargo**](https://doc.rust-lang.org/cargo/) — версія пакунка `cargo` отримується з `Cargo.toml` з поточної теки
- [**Nimble**](https://github.com/nim-lang/nimble) — версія пакунка `nimble` з файлу `*.nimble` з поточної теки, отримана командою `nimble dump`
- [**Poetry**](https://python-poetry.org/) — версія пакунка  `poetry` отримується з `pyproject.toml` з поточної теки
- [**Python**](https://www.python.org) — версія пакунка `python` отримана із [PEP 621](https://peps.python.org/pep-0621/) сумісних файлів `pyproject.toml` чи `setup.cfg` у поточній теці
- [**Composer**](https://getcomposer.org/) — версія пакунка `composer` отримується з `composer.json` з поточної теки
- [**Gradle**](https://gradle.org/) — версія пакунка `gradle` отримується з `build.gradle` з поточної теки
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) — версія пакунка отримується з `Project.toml` з поточної теки
- [**Mix**](https://hexdocs.pm/mix/) — версія пакунка `mix` отримується з `mix.exs` з поточної теки
- [**Helm**](https://helm.sh/docs/helm/helm_package/) — версія чарту `helm` отримується з `Chart.yaml` з поточної теки
- [**Maven**](https://maven.apache.org/) — версія пакунка `maven` отримується з `pom.xml` з поточної теки
- [**Meson**](https://mesonbuild.com/) — версія пакунка `meson` отримується з `meson.build` з поточної теки
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) — версія пакунка `shards` отримується з `shard.yml` з поточної теки
- [**Galaxy**](https://galaxy.ansible.com/) — версія пакунка `galaxy` отримується з `galaxy.yml` з поточної теки
- [**V**](https://vlang.io) — версія пакунка `vlang` отримується з `v.mod` з поточної теки
- [**SBT**](https://scala-sbt.org) — версія пакунка `sbt` отримується з `build.sbt` з поточної теки
- [**Daml**](https://www.digitalasset.com/developers) — версія пакунка `daml` отримується з `daml.yaml` з поточної теки
- [**Dart**](https://pub.dev/) — версія пакунка `dart` отримується з `pubspec.yaml` з поточної теки

> ⚠️ Показується версія пакунка, сирці якого знаходяться у вашій поточній теці, а не у вашому менеджері пакунків.

### Параметри

| Параметр          | Стандартно                        | Опис                                                                                                     |
| ----------------- | --------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`          | `'is [$symbol$version]($style) '` | Формат модуля.                                                                           |
| `символ`          | `'📦 '`                           | Символ, який знаходиться перед версією пакунка.                                          |
| `version_format`  | `'v${raw}'`                       | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | Стиль модуля.                                                                            |
| `display_private` | `false`                           | Вмикає показ версій для приватних пакунків.                                              |
| `disabled`        | `false`                           | Вимикає модуль `package`.                                                                |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v1.0.0` | Версія вашого пакунка                    |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файли `Makefile.PL` чи `Build.PL`
- Поточна тека містить файли `cpanfile` або `cpanfile.snapshot`
- Поточна тека містить файли, `META.json` або `META.yml`
- Поточна тека містить файл `.perl-version`
- Поточна тека містить файли `.pl`, `.pm` або `.pod`

### Параметри

| Параметр            | Стандартно                                                                                               | Опис                                                                                                     |
| ------------------- | -------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | Формат рядка модуля.                                                                     |
| `version_format`    | `'v${raw}'`                                                                                              | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🐪 '`                                                                                                  | Символ, який знаходиться перед версією Perl                                                              |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                                                                                     | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold 149'`                                                                                             | Стиль модуля.                                                                            |
| `disabled`          | `false`                                                                                                  | Вимикає модуль `perl`.                                                                   |

### Змінні

| Змінна  | Приклад   | Опис                                     |
| ------- | --------- | ---------------------------------------- |
| version | `v5.26.1` | Версія `perl`                            |
| symbol  |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `composer.json`
- Поточна тека містить файл `.php-version`
- Поточна тека містить файл `.php`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🐘 '`                              | Символ, який знаходиться перед версією PHP.                                              |
| `detect_extensions` | `['php']`                            | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['composer.json', '.php-version']`  | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'147 bold'`                         | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Вимикає модуль `php`.                                                                    |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v7.3.8` | Версія `php`                             |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pijul Channel

Модуль `pijul_channel` показує активний канал репозиторію у вашій поточній теці.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр            | Стандартно                        | Опис                                                                                        |
| ------------------- | --------------------------------- | ------------------------------------------------------------------------------------------- |
| `символ`            | `' '`                            | Символ, що використовується перед каналом pijul у вашій поточній теці.      |
| `style`             | `'bold purple'`                   | Стиль модуля.                                                               |
| `format`            | `'on [$symbol$channel]($style) '` | Формат модуля.                                                              |
| `truncation_length` | `2^63 - 1`                        | Скорочує назву каналу pijul до `N` графем                                                   |
| `truncation_symbol` | `'…'`                             | Символ, що використовується для позначення назви гілки, яка була скорочена. |
| `disabled`          | `true`                            | Вимикає модуль `pijul`.                                                     |

## Pixi

Модуль `pixi` показує встановлену версію [pixi](https://pixi.sh), а також активоване середовище, якщо `$PIXI_ENVIRONMENT_NAME` встановлено.

> [!TIP] Порада Це не пригнічує власний модифікатор підказки pixi, ви можете виконати `pixi config set shell.change-ps1 false`.

### Параметри

| Параметр                   | Стандартно                                                | Опис                                                                                                                      |
| -------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `format`                   | `'via [$symbol($version )(\($environment\) )]($style)'` | Формат модуля.                                                                                            |
| `version_format`           | `'v${raw}'`                                               | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch`. |
| `символ`                   | `'🧚 '`                                                   | Символ що передує назві оточення.                                                                         |
| `style`                    | `'yellow bold'`                                           | Стиль модуля.                                                                                             |
| `show_default_environment` | `true`                                                    | Чи вказувати, що у вашому проєкті активовано середовище `default`.                                        |
| `pixi_binary`              | `['pixi']`                                                | Налаштовує бінарний файл pixi, який Starship буде використовувати для отримання версії.                   |
| `detect_extensions`        | `[]`                                                      | Які розширення повинні запускати цей модуль.                                                              |
| `detect_files`             | `['pixi.toml']`                                           | Які імена файлів мають запускати цей модуль.                                                              |
| `detect_folders`           | `[]`                                                      | В яких теках цей модуль має запускатись.                                                                  |
| `disabled`                 | `false`                                                   | Вимикає модуль `pixi`.                                                                                    |

### Змінні

| Змінна                            | Приклад      | Опис                                     |
| --------------------------------- | ------------ | ---------------------------------------- |
| version                           | `v0.33.0`    | Версія `pixi`                            |
| environment                       | `py311`      | Поточне середовище pixi                  |
| project_name | `my-project` | Віддзеркалює значення параметра `symbol` |
| symbol                            |              | Віддзеркалює значення параметра `style`  |
| style                             |              | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
# ~/.config/starship.toml

[pixi]
format = '[$symbol$environment](yellow) '
```

## Pulumi

Модуль `pulumi` показує імʼя поточного користувача та версію обраного [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/).

> [!TIP]
> By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms).
> If you still want to enable it, [follow the example shown below](#with-pulumi-version).

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- У поточній теці є або `Pulumi.yaml` або `Pulumi.yml`
- Батьківська тека містить або `Pulumi.yaml` або `Pulumi.yml`, якщо для `search_upwards` не встановлено значення `false`

### Параметри

| Параметр         | Стандартно                                   | Опис                                                                                                     |
| ---------------- | -------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | Формат рядка модуля.                                                                     |
| `version_format` | `'v${raw}'`                                  | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`         | `' '`                                       | Формат рядка перед стеком Pulumi.                                                        |
| `style`          | `'bold 5'`                                   | Стиль модуля.                                                                            |
| `search_upwards` | `true`                                       | Дозволяє шукати файли налаштування pulumi у батьківських теках.                          |
| `disabled`       | `false`                                      | Вимикає модуль `pulumi`.                                                                 |

### Змінні

| Змінна   | Приклад    | Опис                                     |
| -------- | ---------- | ---------------------------------------- |
| version  | `v0.12.24` | Версія `pulumi`                          |
| stack    | `dev`      | Поточний стек Pulumi                     |
| username | `alice`    | Поточне імʼя користувача Pulumi          |
| symbol   |            | Віддзеркалює значення параметра `symbol` |
| style\*  |            | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

#### З версією Pulumi

```toml
# ~/.config/starship.toml

[pulumi]
format = '[🛥 ($version )$stack]($style) '
```

#### Без версії Pulumi

```toml
# ~/.config/starship.toml
[pulumi]
symbol = '🛥 '
format = '[$symbol$stack]($style) '
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version.
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `spago.dhall`
- Поточна тека містить файл `spago.yaml`
- Поточна тека містить файл `spago.lock`
- Поточна тека містить файли з розширенням `.purs`

### Параметри

| Параметр            | Стандартно                                    | Опис                                                                                                     |
| ------------------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                   | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'<=> '`                                      | Символ, який знаходиться перед версією PureScript.                                       |
| `detect_extensions` | `['purs']`                                    | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                          | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold white'`                                | Стиль модуля.                                                                            |
| `disabled`          | `false`                                       | Вимикає модуль `purescript`.                                                             |

### Змінні

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `0.13.5` | Версія `purescript`                      |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

Модуль `python` показує поточну встановлену версію [Python](https://www.python.org/) і поточне [віртуальне середовище Python](https://docs.python.org/tutorial/venv.html), якщо воно активоване.

If `pyenv_version_name` is set to `true`, it will display the pyenv version
name. Otherwise, it will display the version number from `python --version`.

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `.python-version`
- Поточна тека містить файл `Pipfile`
- Поточна тека містить файл `__init__.py`
- Поточна тека містить файл `pyproject.toml`
- Поточна тека містить файл `requirements.txt`
- Поточна тека містить файл `setup.py`
- Поточна тека містить файл `tox.ini`
- Поточна тека містить файл `.py`.
- Поточна тека містить файли з розширенням `.ipynb`.
- Віртуальне середовище активовано

### Параметри

| Параметр             | Стандартно                                                                                                   | Опис                                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Формат модуля.                                                                           |
| `version_format`     | `'v${raw}'`                                                                                                  | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`             | `'🐍 '`                                                                                                      | Формат рядка, що представляє символ Python                                                               |
| `style`              | `'yellow bold'`                                                                                              | Стиль модуля.                                                                            |
| `pyenv_version_name` | `false`                                                                                                      | Використовувати pyenv для отримання версії Python                                                        |
| `pyenv_prefix`       | `'pyenv'`                                                                                                    | Префікс перед версією pyenv, показується якщо pyenv використовується                                     |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Налаштовує бінарні файли python, які Starship буде використовувати для отримання версії. |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                            | Які розширення повинні запускати цей модуль                                                              |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Назви файлів, які активують модуль                                                                       |
| `detect_folders`     | `[]`                                                                                                         | Назви тек, що активують модуль                                                                           |
| `generic_venv_names` | `[]`                                                                                                         | Які імена venv слід замінити на імʼя батьківської теки.                                  |
| `disabled`           | `false`                                                                                                      | Вимикає модуль `python`.                                                                 |

> [!TIP]
> The `python_binary` variable accepts either:
>
> - a string (e.g. `'python3'`),
> - a list of strings (e.g. `['python', 'python3']`)
> - a list of lists of strings, representing commands with optional arguments (e.g.
>   `[['mise', 'exec', '--', 'python'], ['python3']]`)
>
> Starship will try executing each configured command until it gets a result.
> Note you can only change the binary that Starship executes to get the version
> of Python not the arguments that are used.
>
> The default values and order for `python_binary` was chosen to first identify
> the Python version in a virtualenv/conda environments (which currently still
> add a `python`, no matter if it points to `python3` or `python2`). This has the
> side effect that if you still have a system Python 2 installed, it may be
> picked up before any Python 3 (at least on Linux Distros that always symlink
> `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but
> cannot remove the system Python 2, changing this to `'python3'` will hide any
> Python version 2, see example below.

### Змінні

| Змінна                            | Приклад         | Опис                                                                               |
| --------------------------------- | --------------- | ---------------------------------------------------------------------------------- |
| version                           | `'v3.8.1'`      | Версія `python`                                                                    |
| symbol                            | `'🐍 '`         | Віддзеркалює значення параметра `symbol`                                           |
| style                             | `'yellow bold'` | Віддзеркалює значення параметра `style`                                            |
| pyenv_prefix | `'pyenv '`      | Віддзеркалює значення параметра `pyenv_prefix`                                     |
| virtualenv                        | `'venv'`        | Поточна назва `virtualenv` або батьківська, якщо збігається з `generic_venv_names` |

### Приклад

```toml
# ~/.config/starship.toml

[python]
symbol = '👾 '
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Використання лише `python3` для отримання версії.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# Не запускати файли з розширенням py
detect_extensions = []
```

```toml
# ~/.config/starship.toml

[python]
# Potentially dangerous: `uv` can run any binary at `.venv/bin/python` without interaction
python_binary = [['uv', 'run', '--no-python-downloads', '--no-project', 'python']]
```

```toml
# ~/.config/starship.toml

[python]
# Не запускати файли з розширенням py
detect_extensions = []
```

## Quarto

The `quarto` module shows the current installed version of Quarto used in a project.

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `_quarto.yml` file
- The current directory contains any `*.qmd` file

### Змінні

| Змінна              | Приклад                              | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Версія `quarto`                                                                                          |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'⨁ '`                               | Віддзеркалює значення параметра `style`                                                                  |
| `style`             | `'bold #75AADB'`                     | Стиль модуля.                                                                            |
| `detect_extensions` | `['.qmd']`                           | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['_quarto.yml']`                    | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `disabled`          | `false`                              | Disables the `quarto` module.                                                            |

### Змінні

| Змінна  | Приклад   | Опис                                    |
| ------- | --------- | --------------------------------------- |
| version | `1.4.549` | The version of `quarto`                 |
| symbol  |           | Віддзеркалює значення параметра `style` |
| style\* |           | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). Модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Змінні

| Змінна              | Приклад                              | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Версія `R`                                                                                               |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'📐'`                               | Віддзеркалює значення параметра `style`                                                                  |
| `style`             | `'blue bold'`                        | Стиль модуля.                                                                            |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Які розширення повинні запускати цей модуль                                                              |
| `detect_files`      | `['.Rprofile']`                      | Назви файлів, які активують модуль                                                                       |
| `detect_folders`    | `['.Rproj.user']`                    | Назви тек, що активують модуль                                                                           |
| `disabled`          | `false`                              | Disables the `r` module.                                                                 |

### Приклад

| Змінна  | Приклад       | Опис                                    |
| ------- | ------------- | --------------------------------------- |
| version | `v4.0.5`      | The version of `R`                      |
| symbol  |               | Віддзеркалює значення параметра `style` |
| style   | `'blue bold'` | Віддзеркалює значення параметра `style` |

### Приклад

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Змінні

| Змінна              | Приклад                                          | Опис                                                                                                     |
| ------------------- | ------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | Версія `raku`                                                                                            |
| `version_format`    | `'v${raw}'`                                      | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🦋 '`                                          | Віддзеркалює значення параметра `symbol`                                                                 |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['META6.json']`                                 | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                             | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold 149'`                                     | Стиль модуля.                                                                            |
| `disabled`          | `false`                                          | Disables the `raku` module.                                                              |

### Приклад

| Змінна                          | Приклад | Опис                                    |
| ------------------------------- | ------- | --------------------------------------- |
| version                         | `v6.d`  | The version of `raku`                   |
| vm_version | `moar`  | The version of VM `raku` is built on    |
| symbol                          |         | Віддзеркалює значення параметра `style` |
| style\*                         |         | Віддзеркалює значення параметра `style` |

### Приклад

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/).
Модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a file with `.red` or `.reds` extension

### Змінні

| Змінна              | Приклад                              | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Версія `red`                                                                                             |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🔺 '`                              | Віддзеркалює значення параметра `style`                                                                  |
| `detect_extensions` | `['red']`                            | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'red bold'`                         | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Disables the `red` module.                                                               |

### Змінні

| Змінна  | Приклад  | Опис                                    |
| ------- | -------- | --------------------------------------- |
| version | `v2.5.1` | The version of `red`                    |
| symbol  |          | Віддзеркалює значення параметра `style` |
| style\* |          | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/).
Модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Змінні

| Змінна              | Приклад                              | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Версія `ruby`                                                                                            |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'💎 '`                              | Віддзеркалює значення параметра `style`                                                                  |
| `detect_extensions` | `['rb']`                             | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Які змінні середовища повинні запускати цей модуль.                                      |
| `style`             | `'bold red'`                         | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Disables the `ruby` module.                                                              |

### Змінні

| Змінна  | Приклад  | Опис                                                   |
| ------- | -------- | ------------------------------------------------------ |
| version | `v2.5.1` | The version of `ruby`                                  |
| symbol  |          | Віддзеркалює значення параметра `style`                |
| style\* |          | Віддзеркалює значення параметра `style`                |
| gemset  | `test`   | Опціонально, отримує назву RVM gemset. |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/).
Модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Змінні

| Змінна              | Приклад                              | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Версія `rustc`                                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🦀 '`                              | Версія toolchain                                                                                         |
| `detect_extensions` | `['rs']`                             | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['Cargo.toml']`                     | Віддзеркалює значення параметра `style`                                                                  |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold red'`                         | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Disables the `rust` module.                                                              |

### Змінні

| Змінна    | Приклад           | Опис                                         |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | Версія toolchain                             |
| symbol    |                   | Віддзеркалює значення параметра `style`      |
| style\*   |                   | Віддзеркалює значення параметра `style`      |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Змінні

| Змінна              | Приклад                                  | Опис                                                                                                     |
| ------------------- | ---------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'` | Версія `scala`                                                                                           |
| `version_format`    | `'v${raw}'`                              | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `['.metals']`                            | В яких теках цей модуль має запускатись.                                                 |
| `символ`            | `'🆂 '`                                  | Формат рядка, що представляє символ Scala.                                               |
| `style`             | `'red dimmed'`                           | Стиль модуля.                                                                            |
| `disabled`          | `false`                                  | Disables the `scala` module.                                                             |

### Змінні

| Змінна  | Приклад  | Опис                                    |
| ------- | -------- | --------------------------------------- |
| version | `2.13.5` | The version of `scala`                  |
| symbol  |          | Віддзеркалює значення параметра `style` |
| style\* |          | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

The `shell` module shows an indicator for currently used shell.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Змінні

| Змінна                 | Стандартно                | Опис                                                                                                                                       |
| ---------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `bash_indicator`       | `'bsh'`                   | Віддзеркалює значення `indicator` для поточної оболонки.                                                                   |
| `fish_indicator`       | `'fsh'`                   | Віддзеркалює значення параметра `style`.                                                                                   |
| `zsh_indicator`        | `'zsh'`                   | Формат рядка, що використовується для zsh.                                                                                 |
| `powershell_indicator` | `'psh'`                   | Формат рядка, що використовується для powershell.                                                                          |
| `pwsh_indicator`       |                           | Формат рядка, що використовується для pwsh. The default value mirrors the value of `powershell_indicator`. |
| `ion_indicator`        | `'ion'`                   | Формат рядка, що використовується для ion.                                                                                 |
| `elvish_indicator`     | `'esh'`                   | Формат рядка, що використовується для elvish.                                                                              |
| `tcsh_indicator`       | `'tsh'`                   | Формат рядка, що використовується для tcsh.                                                                                |
| `xonsh_indicator`      | `'xsh'`                   | Формат рядка, що використовується для xonsh.                                                                               |
| `cmd_indicator`        | `'cmd'`                   | Формат рядка, що використовується для cmd.                                                                                 |
| `nu_indicator`         | `'nu'`                    | Формат рядка, що використовується для nu.                                                                                  |
| `unknown_indicator`    | `''`                      | Типове значення, що буде показане, якщо оболонка не визначена.                                                             |
| `format`               | `'[$indicator]($style) '` | Формат модуля.                                                                                                             |
| `style`                | `'white bold'`            | Стиль модуля.                                                                                                              |
| `disabled`             | `true`                    | Disables the `shell` module.                                                                                               |

### Змінні

| Змінна    | Стандартно | Опис                                                                       |
| --------- | ---------- | -------------------------------------------------------------------------- |
| indicator |            | Mirrors the value of `indicator` for currently used shell. |
| style\*   |            | Віддзеркалює значення параметра `style`.                   |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклади

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = '󰈺 '
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is
set to a number and meets or exceeds the specified threshold.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Змінні

| Змінна          | Приклад                      | Опис                                                                          |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------- |
| `threshold`     | `2`                          | Поточне значення `SHLVL`                                                      |
| `format`        | `'[$symbol$shlvl]($style) '` | Віддзеркалює значення параметра `symbol`                                      |
| `символ`        | `'↕️  '`                     | Віддзеркалює значення параметра `style`                                       |
| `repeat`        | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value           |
| `style`         | `'bold yellow'`              | Стиль модуля.                                                 |
| `disabled`      | `true`                       | Disables the `shlvl` module.                                  |

### Змінні

| Змінна  | Приклад | Опис                                    |
| ------- | ------- | --------------------------------------- |
| shlvl   | `3`     | The current value of `SHLVL`            |
| symbol  |         | Віддзеркалює значення параметра `style` |
| style\* |         | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

Using `repeat` and `repeat_offset` along with `character` module, one can get
prompt like `❯❯❯` where last character is colored appropriately for return
status code and preceding characters are provided by `shlvl`.

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '[$symbol]($style)'
repeat = true
symbol = '❯'
repeat_offset = 1
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container
and `$SINGULARITY_NAME` is set.

### Змінні

| Змінна     | Приклад                          | Опис                                               |
| ---------- | -------------------------------- | -------------------------------------------------- |
| `format`   | `'[$symbol\[$env\]]($style) '` | Поточний образ Singularity                         |
| `символ`   | `''`                             | Віддзеркалює значення параметра `symbol`           |
| `style`    | `'bold dimmed blue'`             | Віддзеркалює значення параметра `style`            |
| `disabled` | `false`                          | Disables the `singularity` module. |

### Змінні

| Змінна  | Приклад      | Опис                                    |
| ------- | ------------ | --------------------------------------- |
| env     | `centos.img` | Поточний образ Singularity              |
| symbol  |              | Віддзеркалює значення параметра `style` |
| style\* |              | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Solidity

The `solidity` module shows the currently installed version of [Solidity](https://soliditylang.org/)
The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.sol` extension

### Змінні

| Змінна              | Приклад                                                      | Опис                                                                                                     |
| ------------------- | ------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                         | Версія `solidity`                                                                                        |
| `version_format`    | `'v${major}.${minor}.${patch}'`                              | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'S '`                                                       | Віддзеркалює значення параметра `style`                                                                  |
| \`compiler          | ['solc'] | Стандартний компілятор Solidity.                                                         |
| `detect_extensions` | `['sol']`                                                    | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                                         | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                                         | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold blue'`                                                | Стиль модуля.                                                                            |
| `disabled`          | `false`                                                      | Вмикає цей модуль.                                                                       |

### Змінні

| Змінна  | Приклад  | Опис                                    |
| ------- | -------- | --------------------------------------- |
| version | `v0.8.1` | The version of `solidity`               |
| symbol  |          | Віддзеркалює значення параметра `style` |
| style\* |          | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml
[solidity]
format = "via [S $version](blue bold)"
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Змінні

| Змінна              | Приклад                                | Опис                                                                                                                                                                                            |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | Кількість тек, до яких має бути скорочений шлях до середовища оточення. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `символ`            | `'🅢  '`                               | Віддзеркалює значення параметра `symbol`                                                                                                                                                        |
| `style`             | `'bold blue'`                          | Віддзеркалює значення параметра `style`                                                                                                                                                         |
| `format`            | `'via [$symbol$environment]($style) '` | Формат модуля.                                                                                                                                                                  |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                                                    |

### Змінні

| Змінна      | Приклад      | Опис                                    |
| ----------- | ------------ | --------------------------------------- |
| environment | `astronauts` | Поточне середовище spack                |
| symbol      |              | Віддзеркалює значення параметра `style` |
| style\*     |              | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Status

The `status` module displays the exit code of the previous command.
If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`.
Код status буде перетворений у 32-бітне ціле число.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Змінні

| Змінна                      | Приклад                                                                          | Опис                                                                                                                  |
| --------------------------- | -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                    | Код виходу останньої команди                                                                                          |
| `символ`                    | `'❌'`                                                                            | Код виходу останньої команди в hex                                                                                    |
| `success_symbol`            | `''`                                                                             | Код виходу останньої команди                                                                                          |
| `not_executable_symbol`     | `'🚫'`                                                                           | Значення коду, якщо не сигнал                                                                                         |
| `not_found_symbol`          | `'🔍'`                                                                           | Номер сигналу, що відповідає коду завершення, за наявності                                                            |
| `sigint_symbol`             | `'🧱'`                                                                           | Назва сигналу, що відповідає коду виходу лише в тому випадку, якщо його записано                                      |
| `signal_symbol`             | `'⚡'`                                                                            | Містить номер коду виходу, коли значення не знайдено                                                                  |
| `style`                     | `'bold red'`                                                                     | Показує коду виходу конвеєра, доступно тільки в форматі pipestatus_format                        |
| `success_style`             |                                                                                  | Віддзеркалює значення параметра `symbol`                                                                              |
| `failure_style`             |                                                                                  | Віддзеркалити значення параметра `success_style` при успішному виконанні програмі та `failure_style` в іншому випадку |
| `recognize_signal_code`     | `true`                                                                           | Вмикає сигнал на код виходу                                                                                           |
| `map_symbol`                | `false`                                                                          | Вмикає символ на код виходу                                                                                           |
| `pipestatus`                | `false`                                                                          | Вмикає звітування про pipestatus                                                                                      |
| `pipestatus_separator`      | <code>&vert;</code>                                          | Символ, що використовується для розділення сегментів конвеєра (підтримує форматування)             |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | Формат модуля, коли команда є конвеєром                                                                               |
| `pipestatus_segment_format` |                                                                                  | When specified, replaces `format` when formatting pipestatus segments                                                 |
| `disabled`                  | `true`                                                                           | Disables the `status` module.                                                                         |

### Змінні

| Змінна                              | Приклад | Опис                                                                                           |
| ----------------------------------- | ------- | ---------------------------------------------------------------------------------------------- |
| status                              | `127`   | Код виходу останньої команди                                                                   |
| hex_status     | `0x7F`  | Код виходу останньої команди в hex                                                             |
| int                                 | `127`   | Код виходу останньої команди                                                                   |
| common_meaning | `ERROR` | Значення коду, якщо не сигнал                                                                  |
| signal_number  | `9`     | Номер сигналу, що відповідає коду завершення, за наявності                                     |
| signal_name    | `KILL`  | Назва сигналу, що відповідає коду виходу лише в тому випадку, якщо його записано               |
| maybe_int      | `7`     | Містить номер коду виходу, коли значення не знайдено                                           |
| pipestatus                          |         | Показує коду виходу конвеєра, доступно тільки в форматі pipestatus_format |
| symbol                              |         | Віддзеркалює значення параметра `style`                                                        |
| style\*                             |         | Mirrors the value of option `success_style` on program success and `failure_style` otherwise   |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[status]
style = 'bg:blue'
symbol = '🔴 '
success_symbol = '🟢 SUCCESS'
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

The `sudo` module displays if sudo credentials are currently cached.
Модуль показується лише за наявності даних в кеші.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Змінні

| Змінна          | Приклад                  | Опис                                                                              |
| --------------- | ------------------------ | --------------------------------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | Віддзеркалює значення параметра `symbol`                                          |
| `символ`        | `'🧙 '`                  | Віддзеркалює значення параметра `style`                                           |
| `style`         | `'bold blue'`            | Стиль модуля.                                                     |
| `allow_windows` | `false`                  | Оскільки Windows не має sudo, для цієї Ос модуль типово вимкнено. |
| `disabled`      | `true`                   | Disables the `sudo` module.                                       |

### Змінні

| Змінна  | Приклад | Опис                                    |
| ------- | ------- | --------------------------------------- |
| symbol  |         | Віддзеркалює значення параметра `style` |
| style\* |         | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[sudo]
style = 'bold green'
symbol = '👩‍💻 '
disabled = false
```

```toml
# On windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/).
Модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Змінні

| Змінна              | Приклад                              | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Версія `swift`                                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'🐦 '`                              | Віддзеркалює значення параметра `style`                                                                  |
| `detect_extensions` | `['swift']`                          | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['Package.swift']`                  | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold 202'`                         | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Disables the `swift` module.                                                             |

### Змінні

| Змінна  | Приклад  | Опис                                    |
| ------- | -------- | --------------------------------------- |
| version | `v5.2.4` | The version of `swift`                  |
| symbol  |          | Віддзеркалює значення параметра `style` |
| style\* |          | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.
Він підтримує як Hashicorp Terraform, так і OpenTofu для виявлення версій.

> [!TIP]
> By default the Terraform/OpenTofu version is not shown, since this is slow for current versions when a lot of plugins are in use.
> If you still want to enable it, [follow the example shown below](#with-terraform-version).

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Змінні

| Змінна              | Приклад                                                 | Опис                                                                                                     |
| ------------------- | ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '`                    | Версія `terraform`                                                                                       |
| `version_format`    | `'v${raw}'`                                             | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'💠'`                                                  | Віддзеркалює значення параметра `symbol`                                                                 |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`                           | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                                    | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `['.terraform']`                                        | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'bold 105'`                                            | Стиль модуля.                                                                            |
| `disabled`          | `false`                                                 | Disables the `terraform` module.                                                         |
| `команди`           | `[ [ 'terraform', 'version' ], [ 'tofu', 'version' ] ]` | Як визначити версію Terraform.                                                           |

### Змінні

| Змінна    | Приклад      | Опис                                    |
| --------- | ------------ | --------------------------------------- |
| version   | `v0.12.24`   | The version of `terraform`              |
| workspace | `стандартно` | Поточна робоча область Terraform        |
| symbol    |              | Віддзеркалює значення параметра `style` |
| style\*   |              | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

#### Без версії Terraform {#without-terraform-version}

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$version $workspace]($style) '
```

#### Без версії Terraform {#without-terraform-version}

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$workspace]($style) '
```

## Time

The `time` module shows the current **local** time.
The `format` configuration value is used by the [`jiff`](https://crates.io/crates/jiff) crate to control how the time is displayed. Take a look [at the jiff strftime docs](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) to see what options are available.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Параметри

| Параметр          | Стандартно              | Опис                                                                                                                                                                                                                            |
| ----------------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | Формат рядка модуля.                                                                                                                                                                                            |
| `use_12hr`        | `false`                 | Вмикає 12-годинний формат                                                                                                                                                                                                       |
| `time_format`     | дивіться нижче          | The [jiff format string](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) used to format the time.                                                                                                      |
| `style`           | `'bold yellow'`         | Стиль модуля time                                                                                                                                                                                                               |
| `utc_time_offset` | `'local'`               | Встановлює зсув від UTC. Either an IANA time zone name or a range from -24 &lt; x &lt; 24. Дозволяє часові пояси із сувом 30/45 хвилин. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                                                                                                     |
| `time_range`      | `'-'`                   | Встановлює діапазон часу, протягом якого модуль показується. Час має бути зазначений у 24-годинному форматі                                                                                                     |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`.
Manually setting `time_format` will override the `use_12hr` setting.

### Змінні

| Змінна  | Приклад    | Опис                                    |
| ------- | ---------- | --------------------------------------- |
| time    | `13:08:10` | Поточний час.           |
| style\* |            | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

#### With UTC offset

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = '%T'
utc_time_offset = '-5'
time_range = '10:00:00-14:00:00'
```

#### With Timezone name

```toml
# ~/.config/starship.toml

[time]
disabled = false
time_format = '%T'
utc_time_offset = 'Europe/Berlin'
```

## Typst

The `typst` module shows the current installed version of Typst used in a project.

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `template.typ`
- The current directory contains any `*.typ` file

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'t '`                               | Формат рядка, що представляє символ Typst                                                                |
| `style`             | `'bold #0093A7'`                     | Стиль модуля.                                                                            |
| `detect_extensions` | `['.typ']`                           | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['template.typ']`                   | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `disabled`          | `false`                              | Disables the `typst` module.                                                             |

### Змінні

| Змінна                             | Приклад      | Опис                                                                 |
| ---------------------------------- | ------------ | -------------------------------------------------------------------- |
| version                            | `v0.9.0`     | The version of `typst`, alias for typst_version |
| typst_version | `стандартно` | Поточна версія Typest                                                |
| symbol                             |              | Віддзеркалює значення параметра `style`                              |
| style\*                            |              | Віддзеркалює значення параметра `style`                              |

\*: Ця змінна може бути використана лише як частина стилю рядка

## Username

The `username` module shows active user's username.
Модуль показується, якщо виконується будь-яка з наступних умов:

- Поточний користувач має права суперкористувача
- Поточний користувач не є таким же, як той, який увійшов до системи
- Користувач зараз підключений через SSH
- The variable `show_always` is set to true
- The array `detect_env_vars` contains at least the name of one environment variable, that is set

> [!TIP]
> SSH connection is detected by checking environment variables
> `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. Якщо ваш хост SSH не налаштував ці змінні, одним зі способів розвʼязання проблеми є встановлення для однієї з них фіктивного значення.

### Завжди показувати username

| Параметр          | Стандартно              | Опис                                                                       |
| ----------------- | ----------------------- | -------------------------------------------------------------------------- |
| `style_root`      | `'bold red'`            | Стиль, який використовується коли користувач є root/admin. |
| `style_user`      | `'bold yellow'`         | Стиль для звичайних користувачів.                          |
| `detect_env_vars` | `[]`                    | Які змінні середовища повинні запускати цей модуль.        |
| `format`          | `'[$user]($style) in '` | Формат модуля.                                             |
| `show_always`     | `false`                 | Always shows the `username` module.                        |
| `disabled`        | `false`                 | Disables the `username` module.                            |
| `aliases`         | `{}`                    | Переводить системні імена користувачів у щось інше.        |

### Змінні

| Змінна  | Приклад      | Опис                                                                                                        |
| ------- | ------------ | ----------------------------------------------------------------------------------------------------------- |
| `style` | `'red bold'` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`  | `'matchai'`  | Поточний користувач.                                                                        |

### Приклад

#### Параметри

```toml
# ~/.config/starship.toml

[username]
style_user = 'white bold'
style_root = 'black bold'
format = 'user: [$user]($style) '
disabled = false
show_always = true
aliases = { "corpuser034g" = "matchai" }
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `Vagrantfile` file

### Приклад

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'⍱ '`                               | Формат рядка, що представляє символ Vagrant.                                             |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['Vagrantfile']`                    | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'cyan bold'`                        | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                                           |

### Змінні

| Змінна  | Приклад          | Опис                                    |
| ------- | ---------------- | --------------------------------------- |
| version | `Vagrant 2.2.10` | The version of `Vagrant`                |
| symbol  |                  | Віддзеркалює значення параметра `style` |
| style\* |                  | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Параметри

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/).
Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Параметри

| Параметр            | Стандартно                                   | Опис                                                                                                     |
| ------------------- | -------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`         | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                                  | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'V '`                                       | Формат рядка, що представляє символ V                                                                    |
| `detect_extensions` | `['v']`                                      | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                         | В яких теках цей модуль має запускатись.                                                 |
| `style`             | `'blue bold'`                                | Стиль модуля.                                                                            |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                                             |

### Змінні

| Змінна  | Приклад | Опис                                    |
| ------- | ------- | --------------------------------------- |
| version | `v0.2`  | The version of `v`                      |
| symbol  |         | Віддзеркалює значення параметра `style` |
| style\* |         | Віддзеркалює значення параметра `style` |

### Параметри

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCS

> Note the module is enabled by default but **not** included in the default list because that would be a breaking change.
> Крім того, точний формат модуля може змінитися в майбутньому, наприклад, для обробки вирівнювання праворуч.

The `vcs` module displays the current active Version Control System (VCS).
Модуль буде показаний тільки в тому випадку, якщо налаштована VCS зараз використовується.

### Параметри

| Параметр         | Стандартно                                                  | Опис                                                                    |
| ---------------- | ----------------------------------------------------------- | ----------------------------------------------------------------------- |
| `order`          | `["git", "hg", "pijul", "fossil"]`                          | Порядок в якому потрібно шукати VCS.                    |
| `fossil_modules` | `"$fossil_branch$fossil_metrics"`                           | Модулі для показу, коли знайдено репозиторій Fossil.    |
| `git_modules`    | `"$git_branch$git_commit$git_state$git_metrics$git_status"` | Модулі для показу, коли знайдено репозиторій Git.       |
| `hg_modules`     | `"$hg_branch$hg_state"`                                     | Модулі для показу, коли знайдено репозиторій Mercurial. |
| `pijul_modules`  | `"$pijul_channel"`                                          | Модулі для показу, коли знайдено репозиторій Pijul.     |
| `disabled`       | `false`                                                     | Disables the `vcs` module.                              |

### Приклад

```toml
# ~/.config/starship.toml

[vcs]
# Шукатиме Git, а потім Pijul, якщо Git не знайдено, але не шукатиме інші VCS.
order = [
  "git",
  "pijul",
]
# Тут можна включити будь-який модуль (крім самого `$vcs`, щоб уникнути нескінченних циклів).
git_modules = "$git_branch${custom.foo}"

# Дивіться документацію для власних модулів
[custom.foo]
command = 'echo foo'
detect_files = ['foo']
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository.
Модуль показується лише в тому випадку, якщо репозиторій використовується.

### Приклад

| Параметр   | Стандартно                       | Опис                                                               |
| ---------- | -------------------------------- | ------------------------------------------------------------------ |
| `символ`   | `''`                             | Символ, який знаходиться перед назвою репозиторію. |
| `style`    | `'bold yellow'`                  | Стиль модуля.                                      |
| `format`   | `'vcsh [$symbol$repo]($style) '` | Формат модуля.                                     |
| `disabled` | `false`                          | Disables the `vcsh` module.                        |

### Змінні

| Змінна  | Приклад                                     | Опис                                    |
| ------- | ------------------------------------------- | --------------------------------------- |
| repo    | `dotfiles` if in a VCSH repo named dotfiles | Назва поточного репозиторію             |
| symbol  |                                             | Віддзеркалює значення параметра `style` |
| style\* | `black bold dimmed`                         | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

### Параметри

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## XMake

The `xmake` module shows the currently installed version of [XMake](https://xmake.io/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `xmake.lua`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'△ '`                               | Символ, який йде перед версією cmak.                                                     |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль                                                              |
| `detect_files`      | `['xmake.lua']`                      | Назви файлів, які активують модуль                                                                       |
| `detect_folders`    | `[]`                                 | Назви тек, що активують модуль                                                                           |
| `style`             | `'bold green'`                       | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Disables the `xmake` module.                                                             |

### Змінні

| Змінна  | Приклад  | Опис                                    |
| ------- | -------- | --------------------------------------- |
| version | `v2.9.5` | Версія xmake                            |
| symbol  |          | Віддзеркалює значення параметра `style` |
| style\* |          | Віддзеркалює значення параметра `style` |

\*: Ця змінна може бути використана лише як частина стилю рядка

## Zig

By default the `zig` module shows the currently installed version of [Zig](https://ziglang.org/).
Модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `.zig` file

### Приклад

| Параметр            | Стандартно                           | Опис                                                                                                     |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                           |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `символ`            | `'↯ '`                               | Символ, який знаходиться перед версією Zig.                                              |
| `style`             | `'bold yellow'`                      | Стиль модуля.                                                                            |
| `disabled`          | `false`                              | Disables the `zig` module.                                                               |
| `detect_extensions` | `['zig']`                            | Які розширення повинні запускати цей модуль.                                             |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                                             |
| `detect_folders`    | `[]`                                 | В яких теках цей модуль має запускатись.                                                 |

### Власні команди

| Змінна  | Приклад  | Опис                                    |
| ------- | -------- | --------------------------------------- |
| version | `v0.6.0` | The version of `zig`                    |
| symbol  |          | Віддзеркалює значення параметра `style` |
| style\* |          | Віддзеркалює значення параметра `style` |

Модулі показуються, якщо виконується будь-яка з наступних умов:

### Приклад

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## Власні команди

The `custom` modules show the output of some arbitrary commands.

Модулі показуються, якщо виконується будь-яка з наступних умов:

- The current directory contains a file whose name is in `detect_files`
- The current directory contains a directory whose name is in `detect_folders`
- The current directory contains a file whose extension is in `detect_extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

> [!TIP]
> Multiple custom modules can be defined by using a `.`.

> [!TIP]
> The order in which custom modules are shown can be individually set by including
> `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `custom` module will simply show all custom modules in the order they were defined.

> [!TIP]
> [Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules.
> Якщо у вас є цікавий приклад ще не розкритий там, не соромтеся, поділитися ним!

> [!WARNING]
> If `unsafe_no_escape` is enabled or prior to starship v1.20 command output is printed unescaped to the prompt.
>
> Незалежно від результату, який генерує команда, він виводиться в командний рядок у незміненому вигляді. Це означає, що якщо вивід  містить специфічні для оболонки інтерпретовані послідовності, вони можуть бути інтерпретовані безпосередньо під час виведення на екран.
> Залежно від оболонки, це може означати, що, наприклад, рядки, у зворотніх лапках, виконуються оболонкою.
> Such sequences are usually shell specific, e.g. you can write a command module that writes bash sequences,
> e.g. `\h`, but this module will not work in a fish or zsh shell.
>
> Format strings can also contain shell specific prompt sequences, e.g.
> [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html),
> [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

### Власні команди shell

| Параметр            | Стандартно                      | Опис                                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | Команда, вивід якої потрібно показувати. Команду буде передано до оболонки через stdin.                                                                                                                                                                                                                                                                             |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the `shell` returns a `0` status code from executing it.                                                                                                           |
| `require_repo`      | `false`                         | If `true`, the module will only be shown in paths containing a (git) repository. Цей параметр сам по собі не є достатньою умовою для показу модуля за відсутності інших варіантів.                                                                                                                                                               |
| `shell`             |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                                                                                                                                  |
| `опис`              | `'<custom module>'`             | Опис модуля, який показується під час запуску `starship explain`.                                                                                                                                                                                                                                                                                                                   |
| `unsafe_no_escape`  | `false`                         | Якщо встановлено, виведення команд не екранується від символів, які можуть бути інтерпретовані оболонкою.                                                                                                                                                                                                                                                                           |
| `detect_files`      | `[]`                            | Файли, які треба шукати у робочій теці для отримання збігу.                                                                                                                                                                                                                                                                                                                         |
| `detect_folders`    | `[]`                            | Теки, які треба шукати у робочій теці для отримання збігу.                                                                                                                                                                                                                                                                                                                          |
| `detect_extensions` | `[]`                            | Розширення файлів, які треба шукати у робочій теці для отримання збігу.                                                                                                                                                                                                                                                                                                             |
| `символ`            | `''`                            | Символ, який йде перед виводом команди.                                                                                                                                                                                                                                                                                                                                             |
| `style`             | `'bold green'`                  | Стиль модуля.                                                                                                                                                                                                                                                                                                                                                                       |
| `format`            | `'[$symbol($output )]($style)'` | Формат модуля.                                                                                                                                                                                                                                                                                                                                                                      |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                                                                                                      |
| `os`                |                                 | Назва операційної системи, на якій буде показано модуль (unix, linux, macos, windows, … ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                                                                                      |
| `use_stdin`         |                                 | Необов’язкове логічне значення, яке перевизначає, чи команди слід пересилати в оболонку через стандартний ввід чи як аргумент. Якщо не встановлено, типово використовується  стандартний ввід, якщо оболонка не підтримує його (cmd, nushell). Встановлення цього параметра вимикає обробку специфічних для оболонки аргументів. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                                                                                                           |

### Змінні

| Змінна  | Опис                                    |
| ------- | --------------------------------------- |
| output  | The output of `command` run in `shell`  |
| symbol  | Віддзеркалює значення параметра `style` |
| style\* | Віддзеркалює значення параметра `style` |

Команду ` command` (`коли` це застосовується) буде передано у stdin.

#### Власні команди shell

`shell` accepts a non-empty list of strings, where:

- Перший рядок — це шлях до оболонки для виконання команди.
- Наступні — інші аргументи, що передаються до оболонки.

Якщо не налаштовано, модуль повертатиметься до STARSHIP_SHELL, а потім до «sh» у Linux і «cmd /C» у Windows.

The `command` (and `when`, if applicable) will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used,
the following arguments will automatically be added: `-NoProfile -Command -`.
If `shell` is not given or only contains one element and Starship detects Cmd will be used,
the following argument will automatically be added: `/C` and `stdin` will be set to `false`.
If `shell` is not given or only contains one element and Starship detects Nushell will be used,
the following arguments will automatically be added: `-c` and `stdin` will be set to `false`.
Такої поведінки можна уникнути шляхом явного передавання аргументів до оболонки, наприклад,

```toml
shell = ['pwsh', '-Command', '-']
```

> [!WARNING]
> Make sure your custom shell configuration exits gracefully
>
> If you set a custom command, make sure that the default Shell used by starship
> will properly execute the command with a graceful exit (via the `shell`
> option).
>
> For example, PowerShell requires the `-Command` parameter to execute a one
> liner. Пропуск цього параметра може призвести до рекурсивного циклу starship, де оболонка може спробувати знову завантажити повний профіль середовища з самим starship і, отже, повторно виконати власну команду, потрапивши в нескінченний цикл.
>
> Parameters similar to `-NoProfile` in PowerShell are recommended for other
> shells as well to avoid extra loading time of a custom profile on every
> starship invocation.
>
> Наразі реалізовано автоматичне виявлення оболонок і правильне додавання параметрів, але можливо, що охоплено не всі оболонки.
> [Please open an issue](https://github.com/starship/starship/issues/new/choose)
> with shell details and starship configuration if you hit such scenario.

### Приклад

```toml
# ~/.config/starship.toml

[custom.foo]
command = 'echo foo' # shows output of command
detect_files = ['foo'] # can specify filters but wildcards are not supported
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'

[custom.time]
command = 'time /T'
detect_extensions = ['pst'] # filters *.pst files
shell = ['pwsh.exe', '-NoProfile', '-Command', '-']

[custom.time-as-arg]
command = 'time /T'
detect_extensions = ['pst'] # filters *.pst files
shell = ['pwsh.exe', '-NoProfile', '-Command']
use_stdin = false
```
