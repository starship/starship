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

Стандартно starship виводить попередження та інформацію про помилки у файл `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, де ключ сесії відповідає екземпляру вашого термінала. Це, однак, може бути змінено за допомогою змінної середовища `STARSHIP_CACHE`:

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

**Модуль**: Компонент командного рядка, що надає інформацію на основі контексту з вашої ОС. Наприклад, модуль "nodejs" показує версію Node.js, яка встановлена на вашому компʼютері, якщо ваша поточна тека є проєктом Node.js.

**Змінна**: менші субкомпоненти, що містять інформацію, що надається модулем. Наприклад, змінна "version" в модулі "nodejs" містить поточну версію Node.js.

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

При використанні символів розриву рядків можна використовувати багаторядкові оголошення. Наприклад, якщо ви хочете вивести символ `$` на новий рядок, такі значення для `format` є еквівалентними:

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

Формат рядків це формат, з яким модуль виводить всі змінні. У більшості модулів є запис з назвою `format`, який налаштовує формат виводу модуля. Можна використовувати текст, змінні та текстові групи у форматуванні рядків.

#### Змінна

Змінна починається з символу `$`, за яким йде назва змінної. Назва змінною може складатись лише з літер та цифр та символу `_`.

Наприклад:

- `'$version` – це форматований рядок зі змінною з назвою `version`.
- `$git_branch$git_commit'` – рядок з двома змінними `git_branch` і `git_commit`.
- `'$git_branch $git_commit'` має дві змінні, розділені пробілом.

#### Текстова група

Текстова група складається з двох різних частин.

Перша частина, яка вкладена у квадратні дужки `[]` – це [форматований рядок](#format-strings). Ви можете додати текст, змінні або навіть вкладені текстові групи.

В наступній частині, що вкладена в круглі дужки `()`, міститься [опис стилю рядка](#style-strings). Ця частина використовується для стилізації першої частини.

Наприклад:

- `'[on](red bold)'` – виведе рядок `on` жирним шрифтом підсвічений червоним.
- `'[⌘ $version](bold green)'` – виведе символ `⌘` разом з вмістом змінної  `version` жирним шрифтом підсвіченим зеленим.
- `'[a [b](red) c](green)'` – виведе `a b c`, де `b` – червона, а `a` та `c` – зелені.

#### Стилі рядків

Більшість модулів в starship дозволяють вам налаштовувати стилі їх виводу. Це робиться за допомогою запису (зазвичай `style`), який є рядком, що вказує на конфігурацію. Ось деякі приклади стилів поряд з тим, що вони роблять. За деталями щодо повного синтаксису зверніться до [Довідника з розширеного налаштування](/advanced-config/).

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
- Коли `$combined` є посиланням для `\[$a$b\]`, `'($combined)'` не показуватиме нічого, лише якщо `$a` і `$b` одночасно містять `None`. Це працює так само й для `'(\[$a$b\] )'`.

### Негативний збіг

Багато модулів мають змінні `detect_extensions`, `detect_files` та `detect_folders`. Вони отримують перелік рядків для порівняння, чи в них є збіги, чи – немає. Символ для перевірки "негативного збігу"  (негативний збіг – те що не має мати збігу з вказаним значенням) – '!' знак оклику, вказується на початку параметра, що перевіряється. Наявність _будь-якого_ індикатора негативного збігу в теці призведе до того, що результат роботи модуля буде перевірятись на негативний збіг.

Розширення зіставляються як із символами після останньої крапки в назві файлу, так і з символами після першої крапки в назві файлу. Наприклад, `foo.bar.tar.gz` буде збігатись з `bar.tar.gz` і `gz` у змінній `detect_extensions`. Файли, назва яких починається з крапки, взагалі не вважаються такими, що мають розширення.

Щоб побачити, як це працює на практиці, ви можете шукати збіг для файлів TypeScript, але не для MPEG Transport Stream, таким чином:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Командний рядок

Це перелік параметрів налаштувань, що використовуються для всього командного рядка.

### Параметри

| Параметр          | Стандартно                     | Опис                                                                                                                                                                                                                |
| ----------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | Налаштовує формат командного рядка.                                                                                                                                                                                 |
| `right_format`    | `''`                           | Див [Увімкнути командний рядок праворуч](/advanced-config/#enable-right-prompt)                                                                                                                                     |
| `scan_timeout`    | `30`                           | Тайм-аут для сканування файлів (у мілісекундах).                                                                                                                                                                    |
| `command_timeout` | `500`                          | Тайм-аут для команд, виконаних starship (у мілісекундах).                                                                                                                                                           |
| `add_newline`     | `true`                         | Вставити порожній рядок між командними рядками в оболонці.                                                                                                                                                          |
| `palette`         | `''`                           | Встановлює кольорову палітру використовуючи `palettes`.                                                                                                                                                             |
| `palettes`        | `{}`                           | Колекція кольорових палітр, для призначення [кольорів](/advanced-config/#style-strings) до назв визначених користувачем. Зверніть увагу, що кольорові палітри не можуть посилатися на їх власні визначення кольору. |

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

### Стандартний формат командного рядка

Стандартне значення змінної `format` використовується для визначення вигляду командного рядка, якщо  вона пуста або змінна  `format` відсутня. Стандартне значення є таким, як зазначено нижче:

```toml
format = '$all'

# Є еквівалентом
format = """
$username\
$hostname\
$localip\
$shlvl\
$singularity\
$kubernetes\
$directory\
$vcsh\
$fossil_branch\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$pijul_channel\
$docker_context\
$package\
$c\
$cmake\
$cobol\
$daml\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$fennel\
$golang\
$guix_shell\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$gradle\
$lua\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vlang\
$vagrant\
$zig\
$buf\
$nix_shell\
$conda\
$meson\
$spack\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
$env_var\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$os\
$container\
$shell\
$character"""
```

Якщо ви просто хочете розширити стандартний формат, можете використовувати `$all`; модулі, які ви явно додали до формату, не будуть подвоєні. Напр.

```toml
# Перемістити назву теки на другий рядок
format = '$all$directory$character'
```

## AWS

Модуль `aws` показує поточний регіон AWS і профіль, а також час, що залишився до кінця сесії при використанні тимчасових облікових даних. Для виводу модуль використовує змінні `AWS_REGION`, `AWS_DEFAULT_REGION` і `AWS_PROFILE` та файли `~/.aws/config` і `~/.aws/credentials`, коли треба.

Модуль буде показуватиме профіль лише в тому випадку, якщо облікові дані знаходяться в `~/.aws/credentials` або якщо `credential_process` чи `sso_start_url` визначені в `~/.aws/config`. Крім того, достатньо буде визначити будь-яку змінну `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` або `AWS_SESSION_TOKEN`. Якщо для параметра `force_display` встановлено значення `true`, уся доступна інформація буде показуватись, навіть якщо не буде виявлено жодних облікових даних відповідно до наведених вище умов.

Під час використання [aws-vault](https://github.com/99designs/aws-vault) профіль зчитується зі змінної `AWS_VAULT`, а дата закінчення терміну дії облікових даних зчитується зі змінної `AWS_SESSION_EXPIRATION`.

Під час використання [awsu](https://github.com/kreuzwerker/awsu) профіль читається зі змінної `AWSU_PROFILE`.

Під час використання [AWSume](https://awsu.me) профіль зчитується зі змінної `AWSUME_PROFILE`, а дата закінчення терміну дії облікових даних зчитується зі змінної `AWSUME_EXPIRATION`.

Під час використання [saml2aws](https://github.com/Versent/saml2aws) інформація про закінчення терміну дії, отримана з `~/.aws/credentials`, повертається до ключа `x_security_token_expires`.

### Параметри

| Параметр            | Стандартно                                                            | Опис                                                                                                                 |
| ------------------- | --------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | Формат модуля.                                                                                                       |
| `symbol`            | `'☁️ '`                                                               | Символ, який використовується під час показу перед поточним профілем AWS.                                            |
| `region_aliases`    | `{}`                                                                  | Таблиця псевдонімів регіону для показу на додачу до назви AWS.                                                       |
| `profile_aliases`   | `{}`                                                                  | Таблиця псевдонімів профілю для показу на додачу до назви AWS.                                                       |
| `style`             | `'bold yellow'`                                                       | Стиль модуля.                                                                                                        |
| `expiration_symbol` | `X`                                                                   | Символ, який показується, коли закінчився термін дії тимчасових облікових даних.                                     |
| `disabled`          | `false`                                                               | Вимикає модуль `AWS`.                                                                                                |
| `force_display`     | `false`                                                               | Якщо `true`, інформація показується, навіть якщо `credentials`, `credential_process` або `sso_start_url` не вказано. |

### Змінні

| Змінна    | Приклад          | Опис                                     |
| --------- | ---------------- | ---------------------------------------- |
| region    | `ap-northeast-1` | Поточний регіон AWS                      |
| profile   | `astronauts`     | Поточний профіль AWS                     |
| duration  | `2h27m20s`       | Термін дії тимчасових облікових даних    |
| symbol    |                  | Віддзеркалює значення параметра `symbol` |
| style\* |                  | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

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

Модуль `azure` показує поточну підписку Azure. Інформація заснована на назві стандартної підписки або імені користувача, як визначено у файлі `~/.azure/azureProfile.json`.

### Параметри

| Змінна     | Стандартно                               | Опис                           |
| ---------- | ---------------------------------------- | ------------------------------ |
| `format`   | `'on [$symbol($subscription)]($style) '` | Формат для модуля Azure.       |
| `symbol`   | `'ﴃ '`                                   | Символ, який використовується. |
| `style`    | `'blue bold'`                            | Стиль, який використовується.  |
| `disabled` | `true`                                   | Вимикає модуль `azure`.        |

### Приклади

#### Показ назви підписки

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = 'ﴃ '
style = 'blue bold'
```

#### Показ імені користувача

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## Battery

Модуль `battery` показує наскільки заряджена батарея пристрою і стан заряджання. Модуль показується лише тоді, коли заряд акумулятора пристрою нижче 10%.

### Параметри

| Параметр             | Стандартно                        | Опис                                 |
| -------------------- | --------------------------------- | ------------------------------------ |
| `full_symbol`        | `' '`                            | Символ, повного заряду батареї.      |
| `charging_symbol`    | `' '`                            | Символ процесу заряджання.           |
| `discharging_symbol` | `' '`                            | Символ, коли батарея розряджається.  |
| `unknown_symbol`     | `' '`                            | Символ, коли стан батареї невідомий. |
| `empty_symbol`       | `' '`                            | Символ повністю розрядженої батареї. |
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

Параметр конфігурації `display` використовується для визначення, коли має показуватись індикатор заряду батареї (threshold), який символ використовуватиметься (symbol) і як він виглядає (style). Якщо параметр `display` не встановлено. Стандартне значення є таким, як зазначено нижче:

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

Типові значення для опції `charging_symbol` і `discharging_symbol` є відповідно типовими значеннями параметрів `charging_symbol` та `discharging_symbol` `батареї`.

#### Параметри

Параметр `display` є масивом з наступної таблиці.

| Параметр             | Стандартно   | Опис                                                                                                                                     |
| -------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------- |
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

Модуль `buf` показує поточну встановлену версію [Buf](https://buf.build). Типово, модуль показується, якщо всі наступні умови виконуються:

- Встановлено [`buf`](https://github.com/bufbuild/buf) CLI.
- Поточна тека містить файли налаштувань [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), чи [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml).

### Параметри

| Параметр            | Стандартно                                      | Опис                                         |
| ------------------- | ----------------------------------------------- | -------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | Формат для модуля `buf`.                     |
| `version_format`    | `'v${raw}'`                                     | Формат версії.                               |
| `symbol`            | `'🐃 '`                                          | Символ, який знаходиться перед версією Buf.  |
| `detect_extensions` | `[]`                                            | Які розширення повинні запускати цей модуль. |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Які імена файлів мають запускати цей модуль. |
| `detect_folders`    | `[]`                                            | Які теки мають запускати цей модуль.         |
| `style`             | `'bold blue'`                                   | Стиль модуля.                                |
| `disabled`          | `false`                                         | Вимкнути модуль `elixir`.                    |

### Змінні

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| `version` | `v1.0.0` | Версія `buf`                             |
| `symbol`  |          | Віддзеркалює значення параметра `symbol` |
| `style`*  |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

Модуль `bun` показує поточну встановлену версію рушія JavaScript – [bun](https://bun.sh). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `bun.lockb`
- Поточна тека містить файл `bunfig.toml`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🍞 '`                               | A format string representing the symbol of Bun.                      |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['bun.lockb', 'bunfig.toml']`       | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `style`             | `'bold red'`                         | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `bun` module.                                           |

### Змінні

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v0.1.4` | The version of `bun`                     |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

The `c` module shows some information about your C compiler. By default the module will be shown if the current directory contains a `.c` or `.h` file.

### Параметри

| Параметр            | Стандартно                                                                  | Опис                                                                 |
| ------------------- | --------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                | The format string for the module.                                    |
| `version_format`    | `'v${raw}'`                                                                 | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'C '`                                                                      | The symbol used before displaying the compiler details               |
| `detect_extensions` | `['c', 'h']`                                                                | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `[]`                                                                        | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                                                        | Which folders should trigger this module.                            |
| `commands`          | [ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ] | How to detect what the compiler is                                   |
| `style`             | `'bold 149'`                                                                | Стиль модуля.                                                        |
| `disabled`          | `false`                                                                     | Disables the `c` module.                                             |

### Змінні

| Змінна  | Приклад | Опис                                     |
| ------- | ------- | ---------------------------------------- |
| name    | clang   | The name of the compiler                 |
| version | 13.0.0  | The version of the compiler              |
| symbol  |         | Віддзеркалює значення параметра `symbol` |
| style   |         | Віддзеркалює значення параметра `style`  |

NB that `version` is not in the default format.

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).

### Приклад

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## Character

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change its shape take a look at [this example](#with-custom-error-shape).

::: warning

`vimcmd_symbol` is only supported in cmd, fish and zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

:::

### Параметри

| Параметр                    | Стандартно           | Опис                                                                                    |
| --------------------------- | -------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | The format string used before the text input.                                           |
| `success_symbol`            | `'[❯](bold green)'`  | The format string used before the text input if the previous command succeeded.         |
| `error_symbol`              | `'[❯](bold red)'`    | The format string used before the text input if the previous command failed.            |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | The format string used before the text input if the shell is in vim normal mode.        |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode. |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.       |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.        |
| `disabled`                  | `false`              | Disables the `character` module.                                                        |

### Змінні

| Змінна | Приклад | Опис                                                                                                     |
| ------ | ------- | -------------------------------------------------------------------------------------------------------- |
| symbol |         | A mirror of either `success_symbol`, `error_symbol`, `vimcmd_symbol` or `vimcmd_replace_one_symbol` etc. |

### Приклади

#### With custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[✗](bold red) '
```

#### Without custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[➜](bold red) '
```

#### With custom vim shape

```toml
# ~/.config/starship.toml

[character]
vimcmd_symbol = '[V](bold green) '
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). By default the module will be activated if any of the following conditions are met:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### Параметри

| Параметр            | Стандартно                             | Опис                                                                 |
| ------------------- | -------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`   | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                            | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'△ '`                                 | The symbol used before the version of cmake.                         |
| `detect_extensions` | `[]`                                   | Які розширення повинні запускати цей модуль                          |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | Які імена файлів мають запускати цей модуль                          |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module                             |
| `style`             | `'bold blue'`                          | Стиль модуля.                                                        |
| `disabled`          | `false`                                | Disables the `cmake` module.                                         |

### Змінні

| Змінна    | Приклад   | Опис                                     |
| --------- | --------- | ---------------------------------------- |
| version   | `v3.17.3` | The version of cmake                     |
| symbol    |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL. By default, the module will be shown if any of the following conditions are met:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `symbol`            | `'⚙️ '`                              | The symbol used before displaying the version of COBOL.              |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | Стиль модуля.                                                        |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `disabled`          | `false`                              | Disables the `cobol` module.                                         |

### Variables

| Змінна    | Приклад    | Опис                                     |
| --------- | ---------- | ---------------------------------------- |
| version   | `v3.1.2.0` | The version of `cobol`                   |
| symbol    |            | Віддзеркалює значення параметра `symbol` |
| style\* |            | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

## Command Duration

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Параметри

| Параметр               | Стандартно                    | Опис                                                                                                                                                              |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Shortest duration to show time for (in milliseconds).                                                                                                             |
| `show_milliseconds`    | `false`                       | Show milliseconds in addition to seconds for the duration.                                                                                                        |
| `format`               | `'took [$duration]($style) '` | Формат модуля.                                                                                                                                                    |
| `style`                | `'bold yellow'`               | Стиль модуля.                                                                                                                                                     |
| `disabled`             | `false`                       | Disables the `cmd_duration` module.                                                                                                                               |
| `show_notifications`   | `false`                       | Show desktop notifications when command completes.                                                                                                                |
| `min_time_to_notify`   | `45_000`                      | Shortest duration for notification (in milliseconds).                                                                                                             |
| `notification_timeout` |                               | Duration to show notification for (in milliseconds). If unset, notification timeout will be determined by daemon. Not all notification daemons honor this option. |

### Variables

| Змінна    | Приклад  | Опис                                    |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | Віддзеркалює значення параметра `style` |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### Параметри

| Параметр            | Стандартно                             | Опис                                                                                                                                                                                                        |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `'🅒 '`                                 | The symbol used before the environment name.                                                                                                                                                                |
| `style`             | `'bold green'`                         | Стиль модуля.                                                                                                                                                                                               |
| `format`            | `'via [$symbol$environment]($style) '` | Формат модуля.                                                                                                                                                                                              |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                  |
| `disabled`          | `false`                                | Disables the `conda` module.                                                                                                                                                                                |

### Variables

| Змінна      | Приклад      | Опис                                     |
| ----------- | ------------ | ---------------------------------------- |
| environment | `astronauts` | The current conda environment            |
| symbol      |              | Віддзеркалює значення параметра `symbol` |
| style\*   |              | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Container

The `container` module displays a symbol and container name, if inside a container.

### Параметри

| Параметр   | Стандартно                         | Опис                                      |
| ---------- | ---------------------------------- | ----------------------------------------- |
| `symbol`   | `'⬢'`                              | The symbol shown, when inside a container |
| `style`    | `'bold red dimmed'`                | Стиль модуля.                             |
| `format`   | `'[$symbol \[$name\]]($style) '` | Формат модуля.                            |
| `disabled` | `false`                            | Disables the `container` module.          |

### Variables

| Змінна    | Приклад             | Опис                                     |
| --------- | ------------------- | ---------------------------------------- |
| name      | `fedora-toolbox:35` | The name of the container                |
| symbol    |                     | Віддзеркалює значення параметра `symbol` |
| style\* |                     | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `shard.yml`
- The current directory contains a `.cr` file

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `symbol`            | `'🔮 '`                               | The symbol used before displaying the version of crystal.            |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | Стиль модуля.                                                        |
| `detect_extensions` | `['cr']`                             | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['shard.yml']`                      | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `disabled`          | `false`                              | Disables the `crystal` module.                                       |

### Variables

| Змінна    | Приклад   | Опис                                     |
| --------- | --------- | ---------------------------------------- |
| version   | `v0.32.1` | The version of `crystal`                 |
| symbol    |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers) SDK version when you are in the root directory of your Daml project. The `sdk-version` in the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION` environment variable. Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `daml.yaml`

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'Λ '`                               | A format string representing the symbol of Daml                      |
| `style`             | `'bold cyan'`                        | Стиль модуля.                                                        |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['daml.yaml']`                      | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `disabled`          | `false`                              | Disables the `daml` module.                                          |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v2.2.0` | The version of `daml`                    |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml`, `pubspec.yml` or `pubspec.lock` file

### Параметри

| Параметр            | Стандартно                                        | Опис                                                                 |
| ------------------- | ------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`              | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                                       | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🎯 '`                                            | A format string representing the symbol of Dart                      |
| `detect_extensions` | `['dart']`                                        | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `['.dart_tool']`                                  | Which folders should trigger this module.                            |
| `style`             | `'bold blue'`                                     | Стиль модуля.                                                        |
| `disabled`          | `false`                                           | Disables the `dart` module.                                          |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v2.8.4` | The version of `dart`                    |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Параметри

| Параметр            | Стандартно                                                              | Опис                                                                 |
| ------------------- | ----------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                    | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                                                             | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦕 '`                                                                  | A format string representing the symbol of Deno                      |
| `detect_extensions` | `[]`                                                                    | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                                                    | Which folders should trigger this module.                            |
| `style`             | `'green bold'`                                                          | Стиль модуля.                                                        |
| `disabled`          | `false`                                                                 | Disables the `deno` module.                                          |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v1.8.3` | The version of `deno`                    |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Directory

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Параметри

| Параметр                 | Стандартно                                                                                                                   | Опис                                                                                                       |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | The number of parent folders that the current directory should be truncated to.                            |
| `truncate_to_repo`       | `true`                                                                                                                       | Whether or not to truncate to the root of the git repo that you're currently in.                           |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | Формат модуля.                                                                                             |
| `style`                  | `'bold cyan'`                                                                                                                | Стиль модуля.                                                                                              |
| `disabled`               | `false`                                                                                                                      | Disables the `directory` module.                                                                           |
| `read_only`              | `'🔒'`                                                                                                                        | The symbol indicating current directory is read only.                                                      |
| `read_only_style`        | `'red'`                                                                                                                      | The style for the read only symbol.                                                                        |
| `truncation_symbol`      | `''`                                                                                                                         | The symbol to prefix to truncated paths. eg: '…/'                                                          |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. The default value is equivalent to `style`. |
| `repo_root_style`        |                                                                                                                              | The style for the root of the git repo. The default value is equivalent to `style`.                        |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                   |
| `home_symbol`            | `'~'`                                                                                                                        | The symbol indicating home directory.                                                                      |
| `use_os_path_sep`        | `true`                                                                                                                       | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows)                    |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | Стандартно | Опис                                                                                                                                                                   |
| --------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |            | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`        | The number of characters to use when applying fish shell pwd path logic.                                                                                               |
| `use_logical_path`          | `true`     | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Змінна    | Приклад               | Опис                                    |
| --------- | --------------------- | --------------------------------------- |
| path      | `'D:/Projects'`       | The current directory path              |
| style\* | `'black bold dimmed'` | Віддзеркалює значення параметра `style` |

*: Ця змінна може бути використана лише як частина стилю рядка

<details>
<summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| Змінна             | Приклад               | Опис                                    |
| ------------------ | --------------------- | --------------------------------------- |
| before_root_path | `'/path/to/home/'`    | The path before git root directory path |
| repo_root          | `'git_repo'`          | The git root directory name             |
| path               | `'/src/lib'`          | The remaining path                      |
| style              | `'black bold dimmed'` | Віддзеркалює значення параметра `style` |
| repo_root_style  | `'underline white'`   | Style for git root directory name       |

</details>

### Приклад

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### Параметри

| Параметр            | Стандартно                                                    | Опис                                                                              |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                            | Формат модуля.                                                                    |
| `symbol`            | `'🐳 '`                                                        | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                        | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                          | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `['docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                          | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `'blue bold'`                                                 | Стиль модуля.                                                                     |
| `disabled`          | `false`                                                       | Disables the `docker_context` module.                                             |

### Variables

| Змінна    | Приклад        | Опис                                     |
| --------- | -------------- | ---------------------------------------- |
| context   | `test_context` | The current docker context               |
| symbol    |                | Віддзеркалює значення параметра `symbol` |
| style\* |                | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

By default this module will only be shown in your prompt when one or more of the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>) when there is a `.csproj` file in the current directory.

### Параметри

| Параметр            | Стандартно                                                                                              | Опис                                                                 |
| ------------------- | ------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                           | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                                                                                             | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'.NET '`                                                                                               | The symbol used before displaying the version of dotnet.             |
| `heuristic`         | `true`                                                                                                  | Use faster version detection to keep starship snappy.                |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                                                                                    | Які теки мають запускати цей модуль.                                 |
| `style`             | `'bold blue'`                                                                                           | Стиль модуля.                                                        |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                                        |

### Variables

| Змінна    | Приклад          | Опис                                                               |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Віддзеркалює значення параметра `symbol`                           |
| style\* |                  | Віддзеркалює значення параметра `style`                            |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `mix.exs`.

### Параметри

| Параметр            | Стандартно                                                  | Опис                                                                 |
| ------------------- | ----------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                                    |
| `version_format`    | `'v${raw}'`                                                 | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💧 '`                                                      | The symbol used before displaying the version of Elixir/Erlang.      |
| `detect_extensions` | `[]`                                                        | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['mix.exs']`                                               | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                                        | Які теки мають запускати цей модуль.                                 |
| `style`             | `'bold purple'`                                             | Стиль модуля.                                                        |
| `disabled`          | `false`                                                     | Вимкнути модуль `elixir`.                                            |

### Variables

| Змінна      | Приклад | Опис                                     |
| ----------- | ------- | ---------------------------------------- |
| version     | `v1.10` | The version of `elixir`                  |
| otp_version |         | The otp version of `elixir`              |
| symbol      |         | Віддзеркалює значення параметра `symbol` |
| style\*   |         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `elm.json`
- Поточна тека містить файл `elm-package.json`
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains `*.elm` files

### Параметри

| Параметр            | Стандартно                                         | Опис                                                                 |
| ------------------- | -------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`               | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                                        | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🌳 '`                                             | A format string representing the symbol of Elm.                      |
| `detect_extensions` | `['elm']`                                          | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `['elm-stuff']`                                    | Які теки мають запускати цей модуль.                                 |
| `style`             | `'cyan bold'`                                      | Стиль модуля.                                                        |
| `disabled`          | `false`                                            | Disables the `elm` module.                                           |

### Variables

| Змінна    | Приклад   | Опис                                     |
| --------- | --------- | ---------------------------------------- |
| version   | `v0.19.1` | The version of `elm`                     |
| symbol    |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variables. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

::: tip

The order in which env_var modules are shown can be individually set by including `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `env_var` module will simply show all env_var modules in the order they were defined.

:::

::: tip

Multiple environmental variables can be displayed by using a `.`. (see example) If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.

Example: following configuration will display value of USER environment variable

```toml
# ~/.config/starship.toml

[env_var.USER]
default = 'unknown user'
```

:::

### Параметри

| Параметр     | Стандартно                     | Опис                                                                         |
| ------------ | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`     | `""`                           | The symbol used before displaying the variable value.                        |
| `змінна`     |                                | The environment variable to be displayed.                                    |
| `стандартно` |                                | The default value to be displayed when the selected variable is not defined. |
| `format`     | `"with [$env_value]($style) "` | Формат модуля.                                                               |
| `опис`       | `"<env_var module>"`     | The description of the module that is shown when running `starship explain`. |
| `disabled`   | `false`                        | Disables the `env_var` module.                                               |

### Variables

| Змінна    | Приклад                                     | Опис                                       |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Віддзеркалює значення параметра `symbol`   |
| style\* | `black bold dimmed`                         | Віддзеркалює значення параметра `style`    |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[env_var]
variable = 'SHELL'
default = 'unknown shell'
```

Displaying multiple environmental variables:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = 'SHELL'
default = 'unknown shell'
[env_var.USER]
default = 'unknown user'
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `rebar.config`.
- Поточна тека містить файл `erlang.mk`.

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `' '`                               | The symbol used before displaying the version of erlang.             |
| `style`             | `'bold red'`                         | Стиль модуля.                                                        |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Які теки мають запускати цей модуль.                                 |
| `disabled`          | `false`                              | Disables the `erlang` module.                                        |

### Variables

| Змінна    | Приклад   | Опис                                     |
| --------- | --------- | ---------------------------------------- |
| version   | `v22.1.3` | The version of `erlang`                  |
| symbol    |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

The `fennel` module shows the currently installed version of [Fennel](https://fennel-lang.org). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a file with the `.fnl` extension

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🧅 '`                               | The symbol used before displaying the version of fennel.             |
| `style`             | `'bold green'`                       | Стиль модуля.                                                        |
| `detect_extensions` | `[fnl]`                              | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Які теки мають запускати цей модуль.                                 |
| `disabled`          | `false`                              | Disables the `fennel` module.                                        |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v1.2.1` | The version of `fennel`                  |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are present in a line they will split the space evenly between them. This is useful for aligning other modules.

### Параметри

| Параметр   | Стандартно     | Опис                              |
| ---------- | -------------- | --------------------------------- |
| `symbol`   | `'.'`          | The symbol used to fill the line. |
| `style`    | `'bold black'` | Стиль модуля.                     |
| `disabled` | `false`        | Disables the `fill` module        |

### Приклад

```toml
# ~/.config/starship.toml
format = 'AA $fill BB $fill CC'

[fill]
symbol = '-'
style = 'bold green'
```

Produces a prompt that looks like:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Fossil Branch

The `fossil_branch` module shows the name of the active branch of the check-out in your current directory.

### Параметри

| Параметр            | Стандартно                       | Опис                                                                                     |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | Формат модуля. Use `'$branch'` to refer to the current branch name.                      |
| `symbol`            | `' '`                           | The symbol used before the branch name of the check-out in your current directory.       |
| `style`             | `'bold purple'`                  | Стиль модуля.                                                                            |
| `truncation_length` | `2^63 - 1`                       | Truncates a Fossil branch name to `N` graphemes                                          |
| `truncation_symbol` | `'…'`                            | The symbol used to indicate a branch name was truncated. You can use `''` for no symbol. |
| `disabled`          | `true`                           | Disables the `fossil_branch` module.                                                     |

### Variables

| Змінна    | Приклад | Опис                                     |
| --------- | ------- | ---------------------------------------- |
| branch    | `trunk` | The active Fossil branch                 |
| symbol    |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Параметри

| Параметр          | Стандартно                                                 | Опис                                                             |
| ----------------- | ---------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Формат модуля.                                                   |
| `symbol`          | `'☁️  '`                                                   | The symbol used before displaying the current GCP profile.       |
| `region_aliases`  | `{}`                                                       | Table of region aliases to display in addition to the GCP name.  |
| `project_aliases` | `{}`                                                       | Table of project aliases to display in addition to the GCP name. |
| `style`           | `'bold blue'`                                              | Стиль модуля.                                                    |
| `disabled`        | `false`                                                    | Disables the `gcloud` module.                                    |

### Variables

| Змінна    | Приклад       | Опис                                                               |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | The current GCP region                                             |
| account   | `foo`         | The current GCP profile                                            |
| domain    | `example.com` | The current GCP profile domain                                     |
| project   |               | The current GCP project                                            |
| active    | `стандартно`  | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |               | Віддзеркалює значення параметра `symbol`                           |
| style\* |               | Віддзеркалює значення параметра `style`                            |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклади

#### Display account and project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
```

#### Display active config name only

```toml
# ~/.config/starship.toml

[gcloud]
format = '[$symbol$active]($style) '
style = 'bold yellow'
```

#### Display account and aliased region

```toml
# ~/.config/starship.toml

[gcloud]
symbol = '️🇬️ '
[gcloud.region_aliases]
us-central1 = 'uc1'
asia-northeast1 = 'an1'
```

#### Display account and aliased project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = 'vlpn'
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Параметри

| Параметр             | Стандартно                                        | Опис                                                                                     |
| -------------------- | ------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Shows the remote tracking branch name, even if it is equal to the local branch name.     |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | Формат модуля. Use `'$branch'` to refer to the current branch name.                      |
| `symbol`             | `' '`                                            | A format string representing the symbol of git branch.                                   |
| `style`              | `'bold purple'`                                   | Стиль модуля.                                                                            |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                                                 |
| `truncation_symbol`  | `'…'`                                             | The symbol used to indicate a branch name was truncated. You can use `''` for no symbol. |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state.                           |
| `ignore_branches`    | `[]`                                              | A list of names to avoid displaying. Useful for 'master' or 'main'.                      |
| `disabled`           | `false`                                           | Disables the `git_branch` module.                                                        |

### Variables

| Змінна        | Приклад  | Опис                                                                                                   |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | Віддзеркалює значення параметра `symbol`                                                               |
| style\*     |          | Віддзеркалює значення параметра `style`                                                                |

*: Ця змінна може бути використана лише як частина стилю рядка

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

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Параметри

| Параметр             | Стандартно                     | Опис                                                                                 |
| -------------------- | ------------------------------ | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                            | The length of the displayed git commit hash.                                         |
| `format`             | `'[\($hash$tag\)]($style) '` | Формат модуля.                                                                       |
| `style`              | `'bold green'`                 | Стиль модуля.                                                                        |
| `only_detached`      | `true`                         | Only show git commit hash when in detached `HEAD` state                              |
| `tag_disabled`       | `true`                         | Disables showing tag info in `git_commit` module.                                    |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `' 🏷 '`                        | Tag symbol prefixing the info shown                                                  |
| `disabled`           | `false`                        | Disables the `git_commit` module.                                                    |

### Variables

| Змінна    | Приклад   | Опис                                         |
| --------- | --------- | -------------------------------------------- |
| hash      | `b703eb3` | The current git commit hash                  |
| tag       | `v1.0.0`  | The tag name if showing tag info is enabled. |
| style\* |           | Віддзеркалює значення параметра `style`      |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Параметри

| Параметр       | Стандартно                                                      | Опис                                                                                    |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `'MERGING'`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `'REVERTING'`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `'BISECTING'`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `'AM'`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `'AM/REBASE'`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `'bold yellow'`                                                 | Стиль модуля.                                                                           |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Формат модуля.                                                                          |
| `disabled`     | `false`                                                         | Disables the `git_state` module.                                                        |

### Variables

| Змінна           | Приклад    | Опис                                    |
| ---------------- | ---------- | --------------------------------------- |
| state            | `REBASING` | The current state of the repo           |
| progress_current | `1`        | The current operation progress          |
| progress_total   | `2`        | The total operation progress            |
| style\*        |            | Віддзеркалює значення параметра `style` |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in the current git repository.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Параметри

| Параметр             | Стандартно                                                   | Опис                                  |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `'bold green'`                                               | The style for the added count.        |
| `deleted_style`      | `'bold red'`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Формат модуля.                        |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |

### Variables

| Змінна            | Приклад | Опис                                        |
| ----------------- | ------- | ------------------------------------------- |
| added             | `1`     | The current number of added lines           |
| deleted           | `2`     | The current number of deleted lines         |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

::: tip

The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment. You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

:::

### Параметри

| Параметр            | Стандартно                                      | Опис                                                                                                        |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status`                                                                         |
| `conflicted`        | `'='`                                           | This branch has merge conflicts.                                                                            |
| `ahead`             | `'⇡'`                                           | The format of `ahead`                                                                                       |
| `behind`            | `'⇣'`                                           | The format of `behind`                                                                                      |
| `diverged`          | `'⇕'`                                           | The format of `diverged`                                                                                    |
| `up_to_date`        | `''`                                            | The format of `up_to_date`                                                                                  |
| `untracked`         | `'?'`                                           | The format of `untracked`                                                                                   |
| `stashed`           | `'$'`                                           | The format of `stashed`                                                                                     |
| `modified`          | `'!'`                                           | The format of `modified`                                                                                    |
| `staged`            | `'+'`                                           | The format of `staged`                                                                                      |
| `renamed`           | `'»'`                                           | The format of `renamed`                                                                                     |
| `deleted`           | `'✘'`                                           | The format of `deleted`                                                                                     |
| `style`             | `'bold red'`                                    | Стиль модуля.                                                                                               |
| `ignore_submodules` | `false`                                         | Ignore changes to submodules.                                                                               |
| `disabled`          | `false`                                         | Disables the `git_status` module.                                                                           |
| `windows_starship`  |                                                 | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |

### Variables

The following variables can be used in `format`:

| Змінна         | Опис                                                                                                          |
| -------------- | ------------------------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                                   |
| `ahead_behind` | Displays `diverged`, `ahead`, `behind` or `up_to_date` format string based on the current status of the repo. |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                                   |
| `untracked`    | Displays `untracked` when there are untracked files in the working directory.                                 |
| `stashed`      | Displays `stashed` when a stash exists for the local repository.                                              |
| `modified`     | Displays `modified` when there are file modifications in the working directory.                               |
| `staged`       | Displays `staged` when a new file has been added to the staging area.                                         |
| `renamed`      | Displays `renamed` when a renamed file has been added to the staging area.                                    |
| `deleted`      | Displays `deleted` when a file's deletion has been added to the staging area.                                 |
| style\*      | Віддзеркалює значення параметра `style`                                                                       |

*: Ця змінна може бути використана лише як частина стилю рядка

The following variables can be used in `diverged`:

| Змінна         | Опис                                           |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Змінна  | Опис                     |
| ------- | ------------------------ |
| `count` | Show the number of files |

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

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = '⇡${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
behind = '⇣${count}'
```

Use Windows Starship executable on Windows paths in WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `go.mod`
- Поточна тека містить файл `go.sum`
- Поточна тека містить файл `go.work`
- Поточна тека містить файл `glide.yaml`
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Параметри

| Параметр            | Стандартно                                                                                | Опис                                                                 |
| ------------------- | ----------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                                                                               | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐹 '`                                                                                    | A format string representing the symbol of Go.                       |
| `detect_extensions` | `['go']`                                                                                  | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `['Godeps']`                                                                              | Which folders should trigger this module.                            |
| `style`             | `'bold cyan'`                                                                             | Стиль модуля.                                                        |
| `disabled`          | `false`                                                                                   | Disables the `golang` module.                                        |

### Variables

| Змінна    | Приклад   | Опис                                     |
| --------- | --------- | ---------------------------------------- |
| version   | `v1.12.1` | The version of `go`                      |
| symbol    |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment. The module will be shown when inside a guix-shell environment.

### Параметри

| Параметр   | Стандартно                 | Опис                                                   |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'via [$symbol]($style) '` | Формат модуля.                                         |
| `symbol`   | `"🐃 "`                     | A format string representing the symbol of guix-shell. |
| `style`    | `"yellow bold"`            | Стиль модуля.                                          |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### Variables

| Змінна    | Приклад | Опис                                     |
| --------- | ------- | ---------------------------------------- |
| symbol    |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) currently used in the project directory.

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Формат модуля.                                                       |
| `version_format`    | `"v${raw}"`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🅶 "`                               | A format string representing the symbol of Gradle.                   |
| `detect_extensions` | `["gradle", "gradle.kts"]`           | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `["gradle"]`                         | Which folders should trigger this module.                            |
| `style`             | `"bold bright-cyan"`                 | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `gradle` module.                                        |
| `recursive`         | `false`                              | Enables recursive finding for the `gradle` directory.                |

### Variables

| Змінна  | Приклад  | Опис                                     |
| ------- | -------- | ---------------------------------------- |
| version | `v7.5.1` | The version of `gradle`                  |
| symbol  |          | Віддзеркалює значення параметра `symbol` |
| style*  |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `stack.yaml`
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### Параметри

| Параметр            | Стандартно                           | Опис                                               |
| ------------------- | ------------------------------------ | -------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                     |
| `symbol`            | `'λ '`                               | A format string representing the symbol of Haskell |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Які розширення повинні запускати цей модуль.       |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Які імена файлів мають запускати цей модуль.       |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.          |
| `style`             | `'bold purple'`                      | Стиль модуля.                                      |
| `disabled`          | `false`                              | Disables the `haskell` module.                     |

### Variables

| Змінна         | Приклад     | Опис                                                                                    |
| -------------- | ----------- | --------------------------------------------------------------------------------------- |
| version        |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot       | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbol         |             | Віддзеркалює значення параметра `symbol`                                                |
| style\*      |             | Віддзеркалює значення параметра `style`                                                 |

*: Ця змінна може бути використана лише як частина стилю рядка

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### Параметри

| Параметр            | Стандартно                                                                                      | Опис                                                                 |
| ------------------- | ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                            | Формат модуля.                                                       |
| `version_format`    | `"v${raw}"`                                                                                     | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["hx", "hxml"]`                                                                                | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `["project.xml", "Project.xml", "application.xml", "haxelib.json", "hxformat.json", ".haxerc"]` | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[".haxelib", "haxe_libraries"]`                                                                | Які теки мають запускати цей модуль.                                 |
| `symbol`            | `"⌘ "`                                                                                          | A format string representing the symbol of Helm.                     |
| `style`             | `"bold fg:202"`                                                                                 | Стиль модуля.                                                        |
| `disabled`          | `false`                                                                                         | Disables the `haxe` module.                                          |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v4.2.5` | The version of `haxe`                    |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `helmfile.yaml`
- The current directory contains a `Chart.yaml` file

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Які теки мають запускати цей модуль.                                 |
| `symbol`            | `'⎈ '`                               | A format string representing the symbol of Helm.                     |
| `style`             | `'bold white'`                       | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `helm` module.                                          |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v3.1.1` | The version of `helm`                    |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Hostname

The `hostname` module shows the system hostname.

### Параметри

| Параметр     | Стандартно                             | Опис                                                                                                                                 |
| ------------ | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only`   | `true`                                 | Only show hostname when connected to an SSH session.                                                                                 |
| `ssh_symbol` | `'🌐 '`                                 | A format string representing the symbol when connected to SSH session.                                                               |
| `trim_at`    | `'.'`                                  | String that the hostname is cut off at, after the first match. `'.'` will stop after the first dot. `''` will disable any truncation |
| `format`     | `'[$ssh_symbol$hostname]($style) in '` | Формат модуля.                                                                                                                       |
| `style`      | `'bold dimmed green'`                  | Стиль модуля.                                                                                                                        |
| `disabled`   | `false`                                | Disables the `hostname` module.                                                                                                      |

### Variables

| Змінна     | Приклад    | Опис                                                  |
| ---------- | ---------- | ----------------------------------------------------- |
| hostname   | `computer` | The hostname of the computer                          |
| style\*  |            | Віддзеркалює значення параметра `style`               |
| ssh_symbol | `'🌏 '`     | The symbol to represent when connected to SSH session |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, or `.sdkmanrc` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Параметри

| Параметр            | Стандартно                                                                                                            | Опис                                                                 |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                                                                                                           | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                                                                                                  | Які теки мають запускати цей модуль.                                 |
| `symbol`            | `'☕ '`                                                                                                                | A format string representing the symbol of Java                      |
| `style`             | `'red dimmed'`                                                                                                        | Стиль модуля.                                                        |
| `disabled`          | `false`                                                                                                               | Disables the `java` module.                                          |

### Variables

| Змінна    | Приклад | Опис                                     |
| --------- | ------- | ---------------------------------------- |
| version   | `v14`   | The version of `java`                    |
| symbol    |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to _always_ show the symbol and number of jobs, even if there are 0 jobs running.

The default functionality is:

- 0 jobs -> Nothing is shown.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

::: warning

This module is not supported on tcsh and nu.

:::

::: warning

The `threshold` option is deprecated, but if you want to use it, the module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

:::

### Параметри

| Параметр           | Стандартно                    | Опис                                                                     |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | Формат модуля.                                                           |
| `symbol`           | `'✦'`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `'bold blue'`                 | Стиль модуля.                                                            |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| Змінна    | Приклад | Опис                                     |
| --------- | ------- | ---------------------------------------- |
| number    | `1`     | The number of jobs                       |
| symbol    |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Які теки мають запускати цей модуль.                                 |
| `symbol`            | `'ஃ '`                               | A format string representing the symbol of Julia.                    |
| `style`             | `'bold purple'`                      | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `julia` module.                                         |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v1.4.0` | The version of `julia`                   |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `.kt` or a `.kts` file

### Параметри

| Параметр            | Стандартно                           | Опис                                                                          |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                                |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch`          |
| `detect_extensions` | `['kt', 'kts']`                      | Які розширення повинні запускати цей модуль.                                  |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                                  |
| `detect_folders`    | `[]`                                 | Які теки мають запускати цей модуль.                                          |
| `symbol`            | `'🅺 '`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `'bold blue'`                        | Стиль модуля.                                                                 |
| `kotlin_binary`     | `'kotlin'`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Variables

| Змінна    | Приклад   | Опис                                     |
| --------- | --------- | ---------------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`                  |
| symbol    |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

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

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-context --namespace astronaut`. Similarly the user and cluster can be set with `kubectl config set-context starship-context --user starship-user` and `kubectl config set-context starship-context --cluster starship-cluster`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

When the module is enabled it will always be active, unless any of `detect_extensions`, `detect_files` or `detect_folders` have been set in which case the module will only be active in directories that match those conditions.

:::

### Параметри

| Параметр            | Стандартно                                           | Опис                                                                  |
| ------------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`            | `'☸ '`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | Формат модуля.                                                        |
| `style`             | `'cyan bold'`                                        | Стиль модуля.                                                         |
| `context_aliases`   | `{}`                                                 | Table of context aliases to display.                                  |
| `user_aliases`      | `{}`                                                 | Table of user aliases to display.                                     |
| `detect_extensions` | `[]`                                                 | Які розширення повинні запускати цей модуль.                          |
| `detect_files`      | `[]`                                                 | Які імена файлів мають запускати цей модуль.                          |
| `detect_folders`    | `[]`                                                 | Які теки мають запускати цей модуль.                                  |
| `disabled`          | `true`                                               | Disables the `kubernetes` module.                                     |

### Variables

| Змінна    | Приклад              | Опис                                     |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | Віддзеркалює значення параметра `symbol` |
| style\* |                      | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
'dev.local.cluster.k8s' = 'dev'
'.*/openshift-cluster/.*' = 'openshift'
'gke_.*_(?P<var_cluster>[\w-]+)' = 'gke-$var_cluster'
[kubernetes.user_aliases]
'dev.local.cluster.k8s' = 'dev'
'root/.*' = 'root'
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Regex Matching

Additional to simple aliasing, `context_aliases` and `user_aliases` also supports extended matching and renaming using regular expressions.

The regular expression must match on the entire kube context, capture groups can be referenced using `$name` and `$N` in the replacement. This is more explained in the [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace) documentation.

Long and automatically generated cluster names can be identified and shortened using regular expressions:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
'.*/openshift-cluster/.*' = 'openshift'
# Or better, to rename every OpenShift cluster at once:
'.*/(?P<var_cluster>[\w-]+)/.*' = '$var_cluster'

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
'gke_.*_(?P<var_cluster>[\w-]+)' = 'gke-$var_cluster'
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Параметри

| Параметр   | Стандартно | Опис                                                               |
| ---------- | ---------- | ------------------------------------------------------------------ |
| `disabled` | `false`    | Disables the `line_break` module, making the prompt a single line. |

### Приклад

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

The `localip` module shows the IPv4 address of the primary network interface.

### Параметри

| Параметр   | Стандартно                | Опис                                                   |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `'[$localipv4]($style) '` | Формат модуля.                                         |
| `style`    | `'bold yellow'`           | Стиль модуля.                                          |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### Variables

| Змінна    | Приклад      | Опис                                    |
| --------- | ------------ | --------------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address       |
| style\* |              | Віддзеркалює значення параметра `style` |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Параметри

| Параметр            | Стандартно                           | Опис                                                                       |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                             |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch`       |
| `symbol`            | `'🌙 '`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `['lua']`                            | Які розширення повинні запускати цей модуль.                               |
| `detect_files`      | `['.lua-version']`                   | Які імена файлів мають запускати цей модуль.                               |
| `detect_folders`    | `['lua']`                            | Which folders should trigger this module.                                  |
| `style`             | `'bold blue'`                        | Стиль модуля.                                                              |
| `lua_binary`        | `'lua'`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v5.4.0` | The version of `lua`                     |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Параметри

| Параметр    | Стандартно                                      | Опис                                                     |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | Формат модуля.                                           |
| `symbol`    | `'🐏'`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `'bold dimmed white'`                           | Стиль модуля.                                            |
| `disabled`  | `true`                                          | Disables the `memory_usage` module.                      |

### Variables

| Змінна           | Приклад       | Опис                                                               |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Віддзеркалює значення параметра `symbol`                           |
| style\*        |               | Віддзеркалює значення параметра `style`                            |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

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

The `meson` module shows the current Meson developer environment status.

By default the Meson project name is displayed, if `$MESON_DEVENV` is set.

### Параметри

| Параметр            | Стандартно                         | Опис                                                                                      |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | Формат модуля.                                                                            |
| `symbol`            | `'⬢ '`                             | The symbol used before displaying the project name.                                       |
| `style`             | `'blue bold'`                      | Стиль модуля.                                                                             |
| `disabled`          | `false`                            | Disables the `meson` module.                                                              |

### Variables

| Змінна    | Приклад    | Опис                                     |
| --------- | ---------- | ---------------------------------------- |
| project   | `starship` | The current Meson project name           |
| symbol    | `🐏`        | Віддзеркалює значення параметра `symbol` |
| style\* |            | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

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

The `hg_branch` module shows the active branch and topic of the repo in your current directory.

### Параметри

| Параметр            | Стандартно                                | Опис                                                                                         |
| ------------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `' '`                                    | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `'bold purple'`                           | Стиль модуля.                                                                                |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | Формат модуля.                                                                               |
| `truncation_length` | `2^63 - 1`                                | Truncates the hg branch / topic name to `N` graphemes                                        |
| `truncation_symbol` | `'…'`                                     | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                                    | Disables the `hg_branch` module.                                                             |

### Variables

| Змінна    | Приклад   | Опис                                     |
| --------- | --------- | ---------------------------------------- |
| branch    | `master`  | The active mercurial branch              |
| topic     | `feature` | The active mercurial topic               |
| symbol    |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `nim.cfg`
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля                                                        |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'👑 '`                               | The symbol used before displaying the version of Nim.                |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['nim.cfg']`                        | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `style`             | `'bold yellow'`                      | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `nim` module.                                           |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v1.2.0` | The version of `nimc`                    |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. The module will be shown when inside a nix-shell environment.

### Параметри

| Параметр      | Стандартно                                     | Опис                                                                  |
| ------------- | ---------------------------------------------- | --------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | Формат модуля.                                                        |
| `symbol`      | `'❄️ '`                                        | A format string representing the symbol of nix-shell.                 |
| `style`       | `'bold blue'`                                  | Стиль модуля.                                                         |
| `impure_msg`  | `'impure'`                                     | A format string shown when the shell is impure.                       |
| `pure_msg`    | `'pure'`                                       | A format string shown when the shell is pure.                         |
| `unknown_msg` | `''`                                           | A format string shown when it is unknown if the shell is pure/impure. |
| `disabled`    | `false`                                        | Disables the `nix_shell` module.                                      |
| `heuristic`   | `false`                                        | Attempts to detect new `nix shell`-style shells with a heuristic.     |

### Variables

| Змінна    | Приклад | Опис                                     |
| --------- | ------- | ---------------------------------------- |
| state     | `pure`  | The state of the nix-shell               |
| name      | `lorri` | The name of the nix-shell                |
| symbol    |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

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

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `package.json`
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

### Параметри

| Параметр            | Стандартно                                 | Опис                                                                                                  |
| ------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | Формат модуля.                                                                                        |
| `version_format`    | `'v${raw}'`                                | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch`                                  |
| `symbol`            | `' '`                                     | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']` | Які розширення повинні запускати цей модуль.                                                          |
| `detect_files`      | `['package.json', '.node-version']`        | Які імена файлів мають запускати цей модуль.                                                          |
| `detect_folders`    | `['node_modules']`                         | Which folders should trigger this module.                                                             |
| `style`             | `'bold green'`                             | Стиль модуля.                                                                                         |
| `disabled`          | `false`                                    | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                                 | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variables

| Змінна    | Приклад    | Опис                                     |
| --------- | ---------- | ---------------------------------------- |
| version   | `v13.12.0` | The version of `node`                    |
| symbol    |            | Віддзеркалює значення параметра `symbol` |
| style\* |            | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Параметри

| Параметр                  | Стандартно                                                                 | Опис                                                                 |
| ------------------------- | -------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | The format string for the module.                                    |
| `version_format`          | `'v${raw}'`                                                                | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `'🐫 '`                                                                     | The symbol used before displaying the version of OCaml.              |
| `global_switch_indicator` | `''`                                                                       | The format string used to represent global OPAM switch.              |
| `local_switch_indicator`  | `'*'`                                                                      | The format string used to represent local OPAM switch.               |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | Які розширення повинні запускати цей модуль.                         |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | Which folders should trigger this module.                            |
| `style`                   | `'bold yellow'`                                                            | Стиль модуля.                                                        |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                         |

### Variables

| Змінна           | Приклад      | Опис                                                              |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Віддзеркалює значення параметра `symbol`                          |
| style\*        |              | Віддзеркалює значення параметра `style`                           |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool. By default the module will be shown if the current directory contains a `.rego` file.

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🪖  '`                              | A format string representing the symbol of OPA.                      |
| `detect_extensions` | `['rego']`                           | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `style`             | `'bold blue'`                        | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `opa` module.                                           |

### Variables

| Змінна    | Приклад   | Опис                                     |
| --------- | --------- | ---------------------------------------- |
| version   | `v0.44.0` | The version of `opa`                     |
| symbol    |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Параметри

| Параметр   | Стандартно                                      | Опис                                                           |
| ---------- | ----------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | Формат модуля.                                                 |
| `symbol`   | `'☁️ '`                                         | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `'bold yellow'`                                 | Стиль модуля.                                                  |
| `disabled` | `false`                                         | Disables the `openstack` module.                               |

### Variables

| Змінна    | Приклад | Опис                                     |
| --------- | ------- | ---------------------------------------- |
| cloud     | `corp`  | The current OpenStack cloud              |
| project   | `dev`   | The current OpenStack project            |
| symbol    |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## OS

The `os` module shows the current operating system. OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

::: warning

The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

:::

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Параметри

| Параметр   | Стандартно            | Опис                                                   |
| ---------- | --------------------- | ------------------------------------------------------ |
| `format`   | `"[$symbol]($style)"` | Формат модуля.                                         |
| `style`    | `"bold white"`        | Стиль модуля.                                          |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type. Operating system types not defined by your configuration use the default symbols table below. All operating systems currently supported by the module are listed below. If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
[os.symbols]
Alpaquita = "🔔 "
Alpine = "🏔️ "
Amazon = "🙂 "
Android = "🤖 "
Arch = "🎗️ "
Artix = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Linux = "🐧 "
Mabox = "📦 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
OpenBSD = "🐡 "
OpenCloudOS = "☁️ "
openEuler = "🦉 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Unknown = "❓ "
Windows = "🪟 "
```

### Variables

| Змінна    | Приклад      | Опис                                                               |
| --------- | ------------ | ------------------------------------------------------------------ |
| symbol    | `🎗️`         | The current operating system symbol from advanced option `symbols` |
| name      | `Arch Linux` | The current operating system name                                  |
| тип       | `Arch`       | The current operating system type                                  |
| codename  |              | The current operating system codename, if applicable               |
| edition   |              | The current operating system edition, if applicable                |
| version   |              | The current operating system version, if applicable                |
| style\* |              | Віддзеркалює значення параметра `style`                            |

*: Ця змінна може бути використана лише як частина стилю рядка

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

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Параметри

| Параметр          | Стандартно                        | Опис                                                                 |
| ----------------- | --------------------------------- | -------------------------------------------------------------------- |
| `format`          | `'is [$symbol$version]($style) '` | Формат модуля.                                                       |
| `symbol`          | `'📦 '`                            | The symbol used before displaying the version the package.           |
| `version_format`  | `'v${raw}'`                       | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | Стиль модуля.                                                        |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.            |
| `disabled`        | `false`                           | Disables the `package` module.                                       |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v1.0.0` | The version of your package              |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Параметри

| Параметр            | Стандартно                                                                                               | Опис                                                                 |
| ------------------- | -------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | The format string for the module.                                    |
| `version_format`    | `'v${raw}'`                                                                                              | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐪 '`                                                                                                   | The symbol used before displaying the version of Perl                |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                                                                                     | Which folders should trigger this module.                            |
| `style`             | `'bold 149'`                                                                                             | Стиль модуля.                                                        |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                          |

### Variables

| Змінна    | Приклад   | Опис                                     |
| --------- | --------- | ---------------------------------------- |
| version   | `v5.26.1` | The version of `perl`                    |
| symbol    |           | Віддзеркалює значення параметра `symbol` |
| style\* |           | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `composer.json`
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐘 '`                               | The symbol used before displaying the version of PHP.                |
| `detect_extensions` | `['php']`                            | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['composer.json', '.php-version']`  | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `style`             | `'147 bold'`                         | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `php` module.                                           |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v7.3.8` | The version of `php`                     |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pijul Channel

The `pijul_channel` module shows the active channel of the repo in your current directory.

### Параметри

| Параметр            | Стандартно                        | Опис                                                                                 |
| ------------------- | --------------------------------- | ------------------------------------------------------------------------------------ |
| `symbol`            | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | Стиль модуля.                                                                        |
| `format`            | `'on [$symbol$channel]($style) '` | Формат модуля.                                                                       |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                    |
| `truncation_symbol` | `'…'`                             | The symbol used to indicate a branch name was truncated.                             |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

::: tip

By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). If you still want to enable it, [follow the example shown below](#with-pulumi-version).

:::

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Параметри

| Параметр         | Стандартно                                   | Опис                                                                 |
| ---------------- | -------------------------------------------- | -------------------------------------------------------------------- |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | The format string for the module.                                    |
| `version_format` | `'v${raw}'`                                  | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `' '`                                       | A format string shown before the Pulumi stack.                       |
| `style`          | `'bold 5'`                                   | Стиль модуля.                                                        |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.       |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                        |

### Variables

| Змінна    | Приклад    | Опис                                     |
| --------- | ---------- | ---------------------------------------- |
| version   | `v0.12.24` | The version of `pulumi`                  |
| stack     | `dev`      | The current Pulumi stack                 |
| username  | `alice`    | The current Pulumi username              |
| symbol    |            | Віддзеркалює значення параметра `symbol` |
| style\* |            | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

#### With Pulumi Version

```toml
# ~/.config/starship.toml

[pulumi]
format = '[🛥 ($version )$stack]($style) '
```

#### Without Pulumi version

```toml
# ~/.config/starship.toml
[pulumi]
symbol = '🛥 '
format = '[$symbol$stack]($style) '
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- Поточна тека містить файл `spago.dhall`
- The current directory contains a file with the `.purs` extension

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'<=> '`                       | The symbol used before displaying the version of PureScript.         |
| `detect_extensions` | `['purs']`                           | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['spago.dhall']`                    | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `style`             | `'bold white'`                       | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `purescript` module.                                    |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `0.13.5` | The version of `purescript`              |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- Поточна тека містить файл `pyproject.toml`
- Поточна тека містить файл `requirements.txt`
- Поточна тека містить файл `setup.py`
- Поточна тека містить файл `tox.ini`
- The current directory contains a file with the `.py` extension.
- A virtual environment is currently activated

### Параметри

| Параметр             | Стандартно                                                                                                   | Опис                                                                                   |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Формат модуля.                                                                         |
| `version_format`     | `'v${raw}'`                                                                                                  | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch`                   |
| `symbol`             | `'🐍 '`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `'yellow bold'`                                                                                              | Стиль модуля.                                                                          |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                        |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `['py']`                                                                                                     | Які розширення повинні запускати цей модуль                                            |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Які імена файлів мають запускати цей модуль                                            |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                               |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                          |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `'python3'` will hide any Python version 2, see example below.

:::

### Variables

| Змінна       | Приклад         | Опис                                       |
| ------------ | --------------- | ------------------------------------------ |
| version      | `'v3.8.1'`      | The version of `python`                    |
| symbol       | `'🐍 '`          | Віддзеркалює значення параметра `symbol`   |
| style        | `'yellow bold'` | Віддзеркалює значення параметра `style`    |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `'venv'`        | The current `virtualenv` name              |

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
# Only use the `python3` binary to get the version.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

```toml
# ~/.config/starship.toml

[python]
# Display the version of python from inside a local venv.
#
# Note this will only work when the venv is inside the project and it will only
# work in the directory that contains the venv dir but maybe this is ok?
python_binary = ['./venv/bin/python', 'python', 'python3', 'python2']
```

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'📐'`                                | A format string representing the symbol of R.                        |
| `style`             | `'blue bold'`                        | Стиль модуля.                                                        |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Які розширення повинні запускати цей модуль                          |
| `detect_files`      | `['.Rprofile']`                      | Які імена файлів мають запускати цей модуль                          |
| `detect_folders`    | `['.Rproj.user']`                    | Which folders should trigger this module                             |
| `disabled`          | `false`                              | Disables the `r` module.                                             |

### Variables

| Змінна  | Приклад       | Опис                                     |
| ------- | ------------- | ---------------------------------------- |
| version | `v4.0.5`      | The version of `R`                       |
| symbol  |               | Віддзеркалює значення параметра `symbol` |
| style   | `'blue bold'` | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Параметри

| Параметр            | Стандартно                                       | Опис                                                                 |
| ------------------- | ------------------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | The format string for the module.                                    |
| `version_format`    | `'v${raw}'`                                      | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦋 '`                                           | The symbol used before displaying the version of Raku                |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['META6.json']`                                 | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                             | Which folders should trigger this module.                            |
| `style`             | `'bold 149'`                                     | Стиль модуля.                                                        |
| `disabled`          | `false`                                          | Disables the `raku` module.                                          |

### Variables

| Змінна     | Приклад | Опис                                     |
| ---------- | ------- | ---------------------------------------- |
| version    | `v6.d`  | The version of `raku`                    |
| vm_version | `moar`  | The version of VM `raku` is built on     |
| symbol     |         | Віддзеркалює значення параметра `symbol` |
| style\*  |         | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.red` or `.reds` extension

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🔺 '`                               | A format string representing the symbol of Red.                      |
| `detect_extensions` | `['red']`                            | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `style`             | `'red bold'`                         | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `red` module.                                           |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v2.5.1` | The version of `red`                     |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💎 '`                               | A format string representing the symbol of Ruby.                     |
| `detect_extensions` | `['rb']`                             | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Which environment variables should trigger this module.              |
| `style`             | `'bold red'`                         | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `ruby` module.                                          |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v2.5.1` | The version of `ruby`                    |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦀 '`                               | A format string representing the symbol of Rust                      |
| `detect_extensions` | `['rs']`                             | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['Cargo.toml']`                     | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `style`             | `'bold red'`                         | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `rust` module.                                          |

### Variables

| Змінна    | Приклад           | Опис                                         |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | Віддзеркалює значення параметра `symbol`     |
| style\* |                   | Віддзеркалює значення параметра `style`      |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Параметри

| Параметр            | Стандартно                               | Опис                                                                 |
| ------------------- | ---------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                              | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `['.metals']`                            | Які теки мають запускати цей модуль.                                 |
| `symbol`            | `'🆂 '`                                   | A format string representing the symbol of Scala.                    |
| `style`             | `'red dimmed'`                           | Стиль модуля.                                                        |
| `disabled`          | `false`                                  | Disables the `scala` module.                                         |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `2.13.5` | The version of `scala`                   |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Параметри

| Параметр               | Стандартно                | Опис                                                         |
| ---------------------- | ------------------------- | ------------------------------------------------------------ |
| `bash_indicator`       | `'bsh'`                   | A format string used to represent bash.                      |
| `fish_indicator`       | `'fsh'`                   | A format string used to represent fish.                      |
| `zsh_indicator`        | `'zsh'`                   | A format string used to represent zsh.                       |
| `powershell_indicator` | `'psh'`                   | A format string used to represent powershell.                |
| `ion_indicator`        | `'ion'`                   | A format string used to represent ion.                       |
| `elvish_indicator`     | `'esh'`                   | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `'tsh'`                   | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `'xsh'`                   | A format string used to represent xonsh.                     |
| `cmd_indicator`        | `'cmd'`                   | A format string used to represent cmd.                       |
| `nu_indicator`         | `'nu'`                    | A format string used to represent nu.                        |
| `unknown_indicator`    | `''`                      | The default value to be displayed when the shell is unknown. |
| `format`               | `'[$indicator]($style) '` | Формат модуля.                                               |
| `style`                | `'white bold'`            | Стиль модуля.                                                |
| `disabled`             | `true`                    | Disables the `shell` module.                                 |

### Variables

| Змінна    | Стандартно | Опис                                                       |
| --------- | ---------- | ---------------------------------------------------------- |
| indicator |            | Mirrors the value of `indicator` for currently used shell. |
| style\* |            | Віддзеркалює значення параметра `style`.                   |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклади

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ''
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Параметри

| Параметр    | Стандартно                   | Опис                                                          |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `'[$symbol$shlvl]($style) '` | Формат модуля.                                                |
| `symbol`    | `'↕️  '`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `'bold yellow'`              | Стиль модуля.                                                 |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Variables

| Змінна    | Приклад | Опис                                     |
| --------- | ------- | ---------------------------------------- |
| shlvl     | `3`     | The current value of `SHLVL`             |
| symbol    |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### Параметри

| Параметр   | Стандартно                       | Опис                                             |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Формат модуля.                                   |
| `symbol`   | `''`                             | A format string displayed before the image name. |
| `style`    | `'bold dimmed blue'`             | Стиль модуля.                                    |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

| Змінна    | Приклад      | Опис                                     |
| --------- | ------------ | ---------------------------------------- |
| env       | `centos.img` | The current Singularity image            |
| symbol    |              | Віддзеркалює значення параметра `symbol` |
| style\* |              | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Параметри

| Параметр            | Стандартно                             | Опис                                                                                                                                           |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `'🅢  '`                                | The symbol used before the environment name.                                                                                                   |
| `style`             | `'bold blue'`                          | Стиль модуля.                                                                                                                                  |
| `format`            | `'via [$symbol$environment]($style) '` | Формат модуля.                                                                                                                                 |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                   |

### Variables

| Змінна      | Приклад      | Опис                                     |
| ----------- | ------------ | ---------------------------------------- |
| environment | `astronauts` | The current spack environment            |
| symbol      |              | Віддзеркалює значення параметра `symbol` |
| style\*   |              | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Status

The `status` module displays the exit code of the previous command. If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`. The status code will cast to a signed 32-bit integer.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Параметри

| Параметр                    | Стандартно                                                                         | Опис                                                                  |
| --------------------------- | ---------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                      | The format of the module                                              |
| `symbol`                    | `'❌'`                                                                              | The symbol displayed on program error                                 |
| `success_symbol`            | `''`                                                                               | The symbol displayed on program success                               |
| `not_executable_symbol`     | `'🚫'`                                                                              | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `'🔍'`                                                                              | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `'🧱'`                                                                              | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `'⚡'`                                                                              | The symbol displayed on any signal                                    |
| `style`                     | `'bold red'`                                                                       | Стиль модуля.                                                         |
| `recognize_signal_code`     | `true`                                                                             | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                            | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                            | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                          | The symbol used to separate pipestatus segments (supports formatting) |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style)'` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                    | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                             | Disables the `status` module.                                         |

### Variables

| Змінна         | Приклад | Опис                                                                                       |
| -------------- | ------- | ------------------------------------------------------------------------------------------ |
| status         | `127`   | The exit code of the last command                                                          |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                   |
| int            | `127`   | The exit code of the last command                                                          |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                        |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                            |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                       |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                               |
| pipestatus     |         | Rendering of in pipeline programs' exit codes, this is only available in pipestatus_format |
| symbol         |         | Віддзеркалює значення параметра `symbol`                                                   |
| style\*      |         | Віддзеркалює значення параметра `style`                                                    |

*: Ця змінна може бути використана лише як частина стилю рядка

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

The `sudo` module displays if sudo credentials are currently cached. The module will only be shown if credentials are cached.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Параметри

| Параметр        | Стандартно               | Опис                                                    |
| --------------- | ------------------------ | ------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | The format of the module                                |
| `symbol`        | `'🧙 '`                   | The symbol displayed when credentials are cached        |
| `style`         | `'bold blue'`            | Стиль модуля.                                           |
| `allow_windows` | `false`                  | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                   | Disables the `sudo` module.                             |

### Variables

| Змінна    | Приклад | Опис                                     |
| --------- | ------- | ---------------------------------------- |
| symbol    |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

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

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐦 '`                               | A format string representing the symbol of Swift                     |
| `detect_extensions` | `['swift']`                          | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['Package.swift']`                  | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `style`             | `'bold 202'`                         | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `swift` module.                                         |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v5.2.4` | The version of `swift`                   |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.

::: tip

By default the Terraform version is not shown, since this is slow for current versions of Terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

:::

Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '` | The format string for the module.                                    |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💠'`                                | A format string shown before the terraform workspace.                |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`        | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `['.terraform']`                     | Which folders should trigger this module.                            |
| `style`             | `'bold 105'`                         | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `terraform` module.                                     |

### Variables

| Змінна    | Приклад      | Опис                                     |
| --------- | ------------ | ---------------------------------------- |
| version   | `v0.12.24`   | The version of `terraform`               |
| workspace | `стандартно` | The current Terraform workspace          |
| symbol    |              | Віддзеркалює значення параметра `symbol` |
| style\* |              | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

#### With Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $version$workspace]($style) '
```

#### Without Terraform version

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $workspace]($style) '
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Параметри

| Параметр          | Стандартно              | Опис                                                                                                                               |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | The format string for the module.                                                                                                  |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `'bold yellow'`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `'local'`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `'-'`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Змінна    | Приклад    | Опис                                    |
| --------- | ---------- | --------------------------------------- |
| time      | `13:08:10` | The current time.                       |
| style\* |            | Віддзеркалює значення параметра `style` |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = '%T'
utc_time_offset = '-5'
time_range = '10:00:00-14:00:00'
```

## Username

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- The current user is root/admin
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Параметри

| Параметр      | Стандартно              | Опис                                        |
| ------------- | ----------------------- | ------------------------------------------- |
| `style_root`  | `'bold red'`            | The style used when the user is root/admin. |
| `style_user`  | `'bold yellow'`         | The style used for non-root users.          |
| `format`      | `'[$user]($style) in '` | Формат модуля.                              |
| `show_always` | `false`                 | Always shows the `username` module.         |
| `disabled`    | `false`                 | Disables the `username` module.             |

### Variables

| Змінна  | Приклад      | Опис                                                                                        |
| ------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style` | `'red bold'` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`  | `'matchai'`  | The currently logged-in user ID.                                                            |

### Приклад

```toml
# ~/.config/starship.toml

[username]
style_user = 'white bold'
style_root = 'black bold'
format = 'user: [$user]($style) '
disabled = false
show_always = true
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a `Vagrantfile` file

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⍱ '`                               | A format string representing the symbol of Vagrant.                  |
| `detect_extensions` | `[]`                                 | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['Vagrantfile']`                    | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |
| `style`             | `'cyan bold'`                        | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                       |

### Variables

| Змінна    | Приклад          | Опис                                     |
| --------- | ---------------- | ---------------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`                 |
| symbol    |                  | Віддзеркалює значення параметра `symbol` |
| style\* |                  | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). Типово, модуль показується, якщо виконується будь-яка з наступних умов:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Параметри

| Параметр            | Стандартно                                   | Опис                                                                 |
| ------------------- | -------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`         | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                                  | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'V '`                                       | A format string representing the symbol of V                         |
| `detect_extensions` | `['v']`                                      | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                         | Which folders should trigger this module.                            |
| `style`             | `'blue bold'`                                | Стиль модуля.                                                        |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                         |

### Variables

| Змінна    | Приклад | Опис                                     |
| --------- | ------- | ---------------------------------------- |
| version   | `v0.2`  | The version of `v`                       |
| symbol    |         | Віддзеркалює значення параметра `symbol` |
| style\* |         | Віддзеркалює значення параметра `style`  |

### Приклад

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### Параметри

| Параметр   | Стандартно                       | Опис                                                   |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   | `''`                             | The symbol used before displaying the repository name. |
| `style`    | `'bold yellow'`                  | Стиль модуля.                                          |
| `format`   | `'vcsh [$symbol$repo]($style) '` | Формат модуля.                                         |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variables

| Змінна    | Приклад                                     | Опис                                     |
| --------- | ------------------------------------------- | ---------------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name               |
| symbol    |                                             | Віддзеркалює значення параметра `symbol` |
| style\* | `black bold dimmed`                         | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## Zig

By default the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### Параметри

| Параметр            | Стандартно                           | Опис                                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Формат модуля.                                                       |
| `version_format`    | `'v${raw}'`                          | Формат версії. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'↯ '`                               | The symbol used before displaying the version of Zig.                |
| `style`             | `'bold yellow'`                      | Стиль модуля.                                                        |
| `disabled`          | `false`                              | Disables the `zig` module.                                           |
| `detect_extensions` | `['zig']`                            | Які розширення повинні запускати цей модуль.                         |
| `detect_files`      | `[]`                                 | Які імена файлів мають запускати цей модуль.                         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                            |

### Variables

| Змінна    | Приклад  | Опис                                     |
| --------- | -------- | ---------------------------------------- |
| version   | `v0.6.0` | The version of `zig`                     |
| symbol    |          | Віддзеркалює значення параметра `symbol` |
| style\* |          | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

### Приклад

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `detect_files`
- The current directory contains a directory whose name is in `detect_folders`
- The current directory contains a file whose extension is in `detect_extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

::: warning Command output is printed unescaped to the prompt

Whatever output the command generates is printed unmodified in the prompt. This means if the output contains special sequences that are interpreted by your shell they will be expanded when displayed. These special sequences are shell specific, e.g. you can write a command module that writes bash sequences, e.g. `\h`, but this module will not work in a fish or zsh shell.

Format strings can also contain shell specific prompt sequences, e.g. [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

:::

### Параметри

| Параметр            | Стандартно                      | Опис                                                                                                                                                                                                                                                                                          |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                                                                                                                                 |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the command returns a `0` status code.                                                                          |
| `require_repo`      | `false`                         | If `true`, the module will only be shown in paths containing a (git) repository. This option alone is not sufficient display condition in absence of other options.                                                                                                                           |
| `shell`             |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                            |
| `description`       | `'<custom module>'`       | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                  |
| `detect_files`      | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                         |
| `detect_folders`    | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                   |
| `detect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                    |
| `symbol`            | `''`                            | The symbol used before displaying the command output.                                                                                                                                                                                                                                         |
| `style`             | `'bold green'`                  | Стиль модуля.                                                                                                                                                                                                                                                                                 |
| `format`            | `'[$symbol($output )]($style)'` | Формат модуля.                                                                                                                                                                                                                                                                                |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                |
| `os`                |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                 |
| `use_stdin`         |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                     |

### Variables

| Змінна    | Опис                                     |
| --------- | ---------------------------------------- |
| output    | The output of shell command in `shell`   |
| symbol    | Віддзеркалює значення параметра `symbol` |
| style\* | Віддзеркалює значення параметра `style`  |

*: Ця змінна може бути використана лише як частина стилю рядка

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to 'sh' on Linux, and 'cmd /C' on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. If `shell` is not given or only contains one element and Starship detects Cmd will be used, the following argument will automatically be added: `/C` and `stdin` will be set to `false`. If `shell` is not given or only contains one element and Starship detects Nushell will be used, the following arguments will automatically be added: `-c` and `stdin` will be set to `false`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ['pwsh', '-Command', '-']
```

::: warning Make sure your custom shell configuration exits gracefully

If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).

For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.

Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.

Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

:::

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
