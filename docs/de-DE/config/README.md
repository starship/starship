# Konfiguration

Um mit der Konfiguration von Starship zu beginnen, muss eine leere Datei in diesem Pfad erstellt werden: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Die gesamte Konfiguration von Starship erfolgt in dieser [TOML](https://github.com/toml-lang/toml)-Datei:

```toml
# Get editor completions based on the config schema
"$schema" = 'https://starship.rs/config-schema.json'

# Inserts a blank line between shell prompts
add_newline = true

# Replace the "❯" symbol in the prompt with "➜"
[character] # The name of the module we are configuring is "character"
success_symbol = "[➜](bold green)" # The "success_symbol" segment is being set to "➜" with the color "bold green"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

Die voreingestellte Konfigurations-Datei kann mit der `STARSHIP_CONFIG` Umgebungsvariable verändert werden. Hier z. Bsp. für die BASH shell, hinzuzufügen zur ~/. bashrc:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

Äquivalent ist in der Windows PowerShell diese Zeile zum `$PROFILE` hinzuzufügen:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Protokollierung

Standardmäßig protokolliert Starship Warnungen und Fehler in einer Datei names `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, wobei der session key zu der Instanz deines Terminals korrespondiert. Das kann jedoch durch die Nutzung der `STARSHIP_CACHE` Umgebungsvariable verändert werden:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Für die Windows PowerShell diese Zeile zum `$PROFILE` hinzufügen:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminologie

**Module**: Eine Komponente in der Konsole, die auf kontextualisierte Informationen des OS basiert. Beispielsweise zeigt das "nodejs" Module die Version von Node.js, welches derzeitig auf deinem Computer installiert ist, an, falls dein derzeitiger Ordner ein Node.js Projekt ist.

**Variable**: Eine kleinere Unterkomponente, welche Information des Moduls enthält. Zum Beispiel enthält die Variable "version" im Modul "nodejs" die aktuelle Version von Node.js.

Die meisten Module haben einen Präfix der standardmäßigen terminal-Farbe (z.B. `via` in "nodejs") und ein Leerzeichen als Suffix.

### Formatierte Strings

Formatierte Strings sind das Format, mit dem ein Modul all seine Variablen ausgibt. Die meisten Module haben den Eintrag `format`, welcher das anzeige-Format des Moduls konfiguriert. Du kannst Texte, Variablen und Textgruppen in einem formatierten string benutzen.

#### Variable

Eine Variable besteht aus dem `$` Symbol und dem Namen der Variable. The name of a variable can only contain letters, numbers and `_`.

Hier sind ein paar Beispiele:

- `$version` ist ein formatierter string mit dem Namen `version`.
- `$git_branch$git_commit` ist ein formatierter string mit zwei Variablen, nämlich `git_branch` und `git_commit`.
- `$git_branch $git_commit` hat die zwei Variablen mit einem Leerzeichen getrennt.

#### Textgruppe

Eine Textgruppe besteht aus zwei verschiedenen Teilen.

Der erste Teil, welcher innerhalb eckiger Klammern `[]` ist, ist ein [formatierter string](#format-strings). Du kannst ihm Texte, Variablen oder sogar verschachtelte Textgruppen hinzufügen.

Der zweite Teil ist innerhalb normaler Klammern `()` und ist ein[style string](#style-strings). Dies kann verwendet werden, um den ersten Teil zu gestalten.

Hier sind ein paar Beispiele:

- `[on](red bold)` wird einen string `on` mit fettgedrucktem roten Text ausgeben.
- `[⌘ $version](bold green)` wird ein Symbol `⌘` gefolgt von dem Inhalt der Variable `version` mit fettgedrucktem grünen Text ausgeben.
- `[a [b](red) c](green)` wird `a b c` ausgeben, wobei `b` rot ist und `a` und `c` grün sind.

#### Style-Strings

Die meisten Module in Starship lassen dich den Darstellungsstil verändern. Dies passiert meistens an einem bestimmten Eintrag (gewöhnlich `style` genannt), der einen String mit den Einstellungen darstellt. Es folgen ein paar Beispiele für solche Strings zusammen mit Beschreibungen was sie bewirken. Details zur vollen Syntax findest du im [Erweiterten Konfigurationshandbuch](/advanced-config/).

- `"fg:green bg:blue"` setzt grünen Text auf einem blauen Hintergrund
- `"bg:blue fg:bright-green"` setzt hell-grünen Text auf einen blauen Hintergrund
- `"bold fg:27"` setzt färbt fettgedruckten Text in die [ANSI Farbe](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` setzt unterstrichenen Text auf einen orangenen Hintergrund
- `"bold italic fg:purple"` setzt lilane fett-kursive Schrift
- `""` deaktiviert explizit jeden Stil

Wie genau sich diese Konfiguration auswirkt liegt an deinem Terminal-Emulator. Einige Emulatoren zum Beispiel werden die Farben erhellen statt Text dick zu machen, und ein paar Farbthemen benutzen dieselben Werte für normale und helle Farben. Für kursiven Text muss dein Terminal Kursivschrift unterstützen.

#### Bedingte Formatierung

Ein Formatierungszeichenkette (string) in `(` and `)` eingeklammert wird nicht ausgewertet wenn alle darin benutzten Variablen leer oder undefiniert sind.

Hier sind ein paar Beispiele:

- `(@$region)` ist die Variable `region` undefiniert (`None`) oder eine leere Zeichenkette (`""`) dann wird nichts angezeigt, ansonsten <0>@</0> gefolgt von dem Inhalt der Variablen.
- `(some text)` zeigt nichts an, weil der Text in Klammern keine Variablen enthält.
- Wenn `$all` eine Abkürzung für `\[$a$b\]` ist, dann zeigt `($all)` nur nichts an wenn `$a` and `$b` beide undefiniert (`None`) sind. Dasselbe passiert mit `(\[$a$b\] )`.

#### Spezielle Zeichen

Die folgenden Zeichen habe eine spezielle Bedeutung in Formatstrings und müssen durch einen vorangestellten Backslash '\' besonders markiert werden (escaped): `$ \ [ ] ( )`.

Wichtig: TOML hat sowohl Basis-Zeichenketten (basic strings, eingeschlossen in " ") und literale Zeichenketten (literal strings, eingeschlossenin ' '). Empfehlung ist nur literale Zeichenketten (in ' ') in der Konfigurationsdatei zu verwenden. Wenn man Basis-Zeichenketten (in " ") verwenden will, dann muss man den Backslash mit einem Backslash markieren (d.h. '\\' verwenden).

Die folgenden Bespiele für eine Formatierungszeichenkette sind gleich, wenn man ein `$` Symbol in einer neuen Zeile ausgeben will:

```toml
# with basic string
format = "\n\\$"

# with multiline basic string
format = """

\\$"""

# with literal string
format = '''

\$'''
```

### Negative matching

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. These take lists of strings to match or not match. "Negative" options, those which should not be matched, are indicated with a leading "!" character. The presence of _any_ negative indicator in the directory will result in the module not being matched.

Extensions are matched against both the characters after the last dot in a filename, and the characters after the first dot in a filename. For example, `foo.bar.tar.gz` will be matched against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Files whose name begins with a dot are not considered to have extensions at all.

To see how this works in practice, you could match TypeScript but not MPEG Transport Stream files thus:

```toml
detect_extensions = ["ts", "!video.ts", "!audio.ts"]
```

## Prompt

Dies ist eine Liste mit Prompt-weiten Konfigurationsoptionen.

### Optionen

| Option            | Standardwert                   | Beschreibung                                                      |
| ----------------- | ------------------------------ | ----------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | Das Aussehen des Prompts festlegen.                               |
| `right_format`    | `""`                           | Sieh [Enable Right Prompt](/advanced-config/#enable-right-prompt) |
| `scan_timeout`    | `30`                           | Timeout für das Scannen von Dateien (in Millisekunden).           |
| `command_timeout` | `500`                          | Maximale Zeit für von Starship ausgeführte Kommandos.             |
| `add_newline`     | `true`                         | Fügt leere Zeilen zwischen Shell Prompts ein.                     |

### Beispiel

```toml
# ~/.config/starship.toml

# Verwende benutzerdefiniertes Format
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Warte 10 Millisekunden damit Starship die Dateien im aktuellen Ordner überprüfen kann.
scan_timeout = 10

# Deaktiviere die Leerzeile am Anfang der Eingabeaufforderung
add_newline = false
```

### Vordefiniertes Aussehen des Prompts

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. Die Standardwerte sind folgende:

```toml
format = "$all"

# Which is equivalent to
format = """
$username\
$hostname\
$localip\
$shlvl\
$singularity\
$kubernetes\
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
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
$golang\
$haskell\
$helm\
$java\
$julia\
$kotlin\
$lua\
$nim\
$nodejs\
$ocaml\
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
$container\
$shell\
$character"""
```

If you just want to extend the default format, you can use `$all`; modules you explicitly add to the format will not be duplicated. Eg.

```toml
# Move the directory to the second line
format = "$all$directory$character"
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials. The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process` or `sso_start_url` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice. If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var and the credentials expiration date is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var and the credentials expiration date is read from the `AWSUME_EXPIRATION` env var.

### Optionen

| Option              | Standardwert                                                          | Beschreibung                                                                                                |
| ------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | Das Format für das Modul.                                                                                   |
| `symbol`            | `"☁️ "`                                                               | Symbol das vor dem aktuellen AWS-Profil angezeigt wird.                                                     |
| `region_aliases`    |                                                                       | Tabelle der Regionaliasen, die zusätzlich zum AWS-Namen angezeigt werden sollen.                            |
| `profile_aliases`   |                                                                       | Table of profile aliases to display in addition to the AWS name.                                            |
| `style`             | `"bold yellow"`                                                       | Stil für dieses Modul.                                                                                      |
| `expiration_symbol` | `X`                                                                   | Das Symbol, das angezeigt wird, wenn die temporären Anmeldeinformationen abgelaufen sind.                   |
| `disabled`          | `false`                                                               | Deaktiviert das `aws`-Modul.                                                                                |
| `force_display`     | `false`                                                               | If `true` displays info even if `credentials`, `credential_process` or `sso_start_url` have not been setup. |

### Variables

| Variable  | Beispiel         | Beschreibung                                         |
| --------- | ---------------- | ---------------------------------------------------- |
| region    | `ap-northeast-1` | Die aktuelle AWS-Region                              |
| profile   | `astronauts`     | Das aktuelle AWS Profil                              |
| duration  | `2h27m20s`       | Gültigkeitsdauer der temporären Anmeldeinformationen |
| symbol    |                  | Spiegelt den Wert der Option `symbol`                |
| style\* |                  | Spiegelt den Wert der Option `style`                 |

*: This variable can only be used as a part of a style string

### Beispiele

#### Alles anzeigen

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
```

#### Region anzeigen

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$region]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Profil anzeigen

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

The `azure` module shows the current Azure Subscription. This is based on showing the name of the default subscription, as defined in the `~/.azure/azureProfile.json` file.

### Optionen

| Variable   | Standardwert                             | Beschreibung                               |
| ---------- | ---------------------------------------- | ------------------------------------------ |
| `format`   | `"on [$symbol($subscription)]($style) "` | The format for the Azure module to render. |
| `symbol`   | `"ﴃ "`                                   | The symbol used in the format.             |
| `style`    | `"blue bold"`                            | The style used in the format.              |
| `disabled` | `true`                                   | Disables the `azure` module.               |

### Beispiel

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($subscription)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## Akkustand

Das `battery` Modul zeigt, wie hoch der Akku des Geräts geladen ist und den aktuellen Ladestatus. Das Modul ist nur sichtbar, wenn der Akku des Geräts unter 10% geladen ist.

### Optionen

| Option               | Standardwert                      | Beschreibung                                                                        |
| -------------------- | --------------------------------- | ----------------------------------------------------------------------------------- |
| `full_symbol`        | `" "`                            | Das Symbol das angezeigt wird wenn der Akku voll geladen ist.                       |
| `charging_symbol`    | `" "`                            | Das Symbol das angezeigt wird wenn der Akku aufgeladen wird.                        |
| `discharging_symbol` | `" "`                            | Das Symbol, das angezeigt wird, wenn die Batterie entladen wird.                    |
| `unknown_symbol`     | `" "`                            | Das Symbol, das angezeigt wird, wenn der Batteriezustand unbekannt ist.             |
| `empty_symbol`       | `" "`                            | Das Symbol, das angezeigt wird, wenn die Batterie leer ist.                         |
| `format`             | `"[$symbol$percentage]($style) "` | Das Format für das Modul.                                                           |
| `display`            | [link](#battery-display)          | Stellt den Grenzwert ein ab dem der Ladezustand (das battery-Modul) angezeigt wird. |
| `disabled`           | `false`                           | Wenn der Wert auf `true` steht, wird das Akkustand-Modul deaktiviert.               |

### Beispiel

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### Anzeige des Akkustandes

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style). Wenn `display` nicht angegeben ist. Die Standardwerte sind folgende:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Optionen

Die `display`-Option beinhaltet ein Array mit den folgenden Werten.

| Option               | Standardwert | Beschreibung                                                                                              |
| -------------------- | ------------ | --------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | Der Schwellenwert zur Anzeige dieser Option.                                                              |
| `style`              | `bold red`   | Der Stil, der zur Anzeige dieser Option verwendet wird.                                                   |
| `charging_symbol`    | `-`          | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` | `-`          | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### Beispiel

```toml
[[battery.display]] # "bold red" style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]] # "bold yellow" style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"
discharging_symbol = "💦"

# when capacity is over 30%, the battery indicator will not be displayed
```

## Buf

The `buf` module shows the currently installed version of [Buf](https://buf.build). By default, the module is shown if all of the following conditions are met:

- The [`buf`](https://github.com/bufbuild/buf) CLI is installed.
- The current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### Optionen

| Option              | Standardwert                                                 | Beschreibung                                          |
| ------------------- | ------------------------------------------------------------ | ----------------------------------------------------- |
| `format`            | `'with [$symbol($version \(Buf $buf_version\) )]($style)'` | The format for the `buf` module.                      |
| `version_format`    | `"v${raw}"`                                                  | The version format.                                   |
| `symbol`            | `"🦬 "`                                                       | The symbol used before displaying the version of Buf. |
| `detect_extensions` | `[]`                                                         | Which extensions should trigger this module.          |
| `detect_files`      | `["buf.yaml", "buf.gen.yaml", "buf.work.yaml"]`              | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                                         | Which folders should trigger this modules.            |
| `style`             | `"bold blue"`                                                | Stil für dieses Modul.                                |
| `disabled`          | `false`                                                      | Disables the `elixir` module.                         |

### Variables

| Variable      | Beispiel | Beschreibung                          |
| ------------- | -------- | ------------------------------------- |
| `buf_version` | `v1.0.0` | The version of `buf`                  |
| `symbol`      |          | Spiegelt den Wert der Option `symbol` |
| `style`*      |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[buf]
symbol = "🦬 "
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime. By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `bun.lockb`-Datei
- Das aktuelle Verzeichnis enthält eine `bunfig.toml`-Datei

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🍞 "`                               | A format string representing the symbol of Node.js.                       |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["bun.lockb", "bunfig.toml"]`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold red"`                         | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Disables the `bun` module.                                                |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v0.1.4` | The version of `bun`                  |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[bun]
format = "via [🍔 $version](bold green) "
```

## C

The `c` module shows some information about your C compiler. By default the module will be shown if the current directory contains a `.c` or `.h` file.

### Optionen

| Option              | Standardwert                                                                | Beschreibung                                                              |
| ------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version(-$name) )]($style)"`                                | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                                                                 | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"C "`                                                                      | The symbol used before displaying the compiler details                    |
| `detect_extensions` | `["c", "h"]`                                                                | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                                                        | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                        | Which folders should trigger this module.                                 |
| `commands`          | [ [ "cc", "--version" ], [ "gcc", "--version" ], [ "clang", "--version" ] ] | How to detect what the compiler is                                        |
| `style`             | `"bold 149"`                                                                | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                                                     | Disables the `c` module.                                                  |

### Variables

| Variable | Beispiel | Beschreibung                          |
| -------- | -------- | ------------------------------------- |
| name     | clang    | The name of the compiler              |
| version  | 13.0.0   | The version of the compiler           |
| symbol   |          | Spiegelt den Wert der Option `symbol` |
| style    |          | Spiegelt den Wert der Option `style`  |

NB that `version` is not in the default format.

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `["mycc", "--version"]`. Starship will try executing each command until it gets a result on STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).

### Beispiel

```toml
# ~/.config/starship.toml

[c]
format = "via [$name $version]($style)"
```

## Zeichen

Das `character` Modul zeigt ein Zeichen ( meistens einen Pfeil "❯") vor der Texteingabe an.

Das Zeichen zeigt an ob der letzte Befehl erfolgreich war, oder einen Fehler erzeugt hat. It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change its shape take a look at [this example](#with-custom-error-shape).

::: warning

`vimcmd_symbol` is only supported in cmd, fish and zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

:::

### Optionen

| Option                      | Standardwert         | Beschreibung                                                                            |
| --------------------------- | -------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `"$symbol "`         | The format string used before the text input.                                           |
| `success_symbol`            | `"[❯](bold green)"`  | The format string used before the text input if the previous command succeeded.         |
| `error_symbol`              | `"[❯](bold red)"`    | The format string used before the text input if the previous command failed.            |
| `vimcmd_symbol`             | `"[❮](bold green)"`  | The format string used before the text input if the shell is in vim normal mode.        |
| `vimcmd_replace_one_symbol` | `"[❮](bold purple)"` | The format string used before the text input if the shell is in vim `replace_one` mode. |
| `vimcmd_replace_symbol`     | `"[❮](bold purple)"` | The format string used before the text input if the shell is in vim replace mode.       |
| `vimcmd_visual_symbol`      | `"[❮](bold yellow)"` | The format string used before the text input if the shell is in vim replace mode.       |
| `disabled`                  | `false`              | Deaktiviert das `character`-Modul.                                                      |

### Variables

| Variable | Beispiel | Beschreibung                                                          |
| -------- | -------- | --------------------------------------------------------------------- |
| symbol   |          | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

### Beispiele

#### With custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Without custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### With custom vim shape

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). By default the module will be activated if any of the following conditions are met:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### Optionen

| Option              | Standardwert                           | Beschreibung                                                              |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`   | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                            | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"△ "`                                 | The symbol used before the version of cmake.                              |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this module                               |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Which filenames should trigger this module                                |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module                                  |
| `style`             | `"bold blue"`                          | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                | Disables the `cmake` module.                                              |

### Variables

| Variable  | Beispiel  | Beschreibung                          |
| --------- | --------- | ------------------------------------- |
| version   | `v3.17.3` | The version of cmake                  |
| symbol    |           | Spiegelt den Wert der Option `symbol` |
| style\* |           | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL. By default, the module will be shown if any of the following conditions are met:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `"⚙️ "`                              | The symbol used before displaying the version of COBOL.                   |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold blue"`                        | Stil für dieses Modul.                                                    |
| `detect_extensions` | `["cbl", "cob", "CBL", "COB"]`       | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `cobol` module.                                              |

### Variables

| Variable  | Beispiel   | Beschreibung                          |
| --------- | ---------- | ------------------------------------- |
| version   | `v3.1.2.0` | The version of `cobol`                |
| symbol    |            | Spiegelt den Wert der Option `symbol` |
| style\* |            | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

## Befehlsdauer

Das `cmd_duration` Modul zeigt an wie lange der letzte Befehl ausgeführt wurde. Das Modul wird nur angezeigt wenn der letzte Befehl länger als zwei Sekunden ausgeführt wurde. Mit der `min_time` Option kann die Zeit eingestellt werden ab der <0>cmd_duration</0> angezeigt wird.

::: warning Nicht die DEBUG-trap in der Bash hooken

Ist `bash` die Konsole der Wahl, dann nicht die `DEBUG`-trap nach der Ausführung von `eval $(starship init $0)` hooken, andernfalls **wird** dieses Modul unweigerlich untergehen.

:::

Bash Nutzer, die eine "preexec" ähnliche Funktion benötigen, können [rcaloras bash_preexec Framework](https://github.com/rcaloras/bash-preexec) verwenden. Definieren Sie einfach die Arrays `preexec_functions` und `precmd_functions` bevor sie `eval $(starship init $0)` ausführen, und fahren Sie dann wie gewohnt fort.

### Optionen

| Option                 | Standardwert                  | Beschreibung                                                                                                                                                      |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Schwellwert für kleinste anzuzeigende Laufzeit (in Millisekunden).                                                                                                |
| `show_milliseconds`    | `false`                       | Zeige Millisekunden zusätzlich zu Sekunden.                                                                                                                       |
| `format`               | `"took [$duration]($style) "` | Das Format für das Modul.                                                                                                                                         |
| `style`                | `"bold yellow"`               | Stil für dieses Modul.                                                                                                                                            |
| `disabled`             | `false`                       | Deaktiviert das `cmd_duration`-Modul.                                                                                                                             |
| `show_notifications`   | `false`                       | Show desktop notifications when command completes.                                                                                                                |
| `min_time_to_notify`   | `45_000`                      | Shortest duration for notification (in milliseconds).                                                                                                             |
| `notification_timeout` |                               | Duration to show notification for (in milliseconds). If unset, notification timeout will be determined by daemon. Not all notification daemons honor this option. |

### Variables

| Variable  | Beispiel | Beschreibung                            |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | Spiegelt den Wert der Option `style`    |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

Hinweis: Dies unterdrückt nicht conda's eigenen Prompt-Modifikator, sie können jedoch conda mit `conda config --set changeps1 False` konfigurieren, um die Ausgabe von conda selbst auszuschalten.

:::

### Optionen

| Option              | Standardwert                           | Beschreibung                                                                                                                                                                                                                                      |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | Die Anzahl der Verzeichnisse, auf die der Verzeichnisspfad abgeschnitten werden soll, wenn die Umgebung über `conda erstellt wurde -p [path]`. `0` bedeutet keine Kürzung. Beachte auch die Beschreibung für das [`directory`](#directory) Modul. |
| `symbol`            | `"🅒 "`                                 | Symbol das vor dem Umgebungsnamen angezeigt wird.                                                                                                                                                                                                 |
| `style`             | `"bold green"`                         | Stil für dieses Modul.                                                                                                                                                                                                                            |
| `format`            | `"via [$symbol$environment]($style) "` | Das Format für das Modul.                                                                                                                                                                                                                         |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                                                        |
| `disabled`          | `false`                                | Deaktiviert das `conda`-Modul.                                                                                                                                                                                                                    |

### Variables

| Variable    | Beispiel     | Beschreibung                          |
| ----------- | ------------ | ------------------------------------- |
| environment | `astronauts` | The current conda environment         |
| symbol      |              | Spiegelt den Wert der Option `symbol` |
| style\*   |              | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Container

The `container` module displays a symbol and container name, if inside a container.

### Optionen

| Option     | Standardwert                           | Beschreibung                              |
| ---------- | -------------------------------------- | ----------------------------------------- |
| `symbol`   | `"⬢"`                                  | The symbol shown, when inside a container |
| `style`    | `"bold red dimmed"`                    | Stil für dieses Modul.                    |
| `format`   | `"[$symbol \\[$name\\]]($style) "` | Das Format für das Modul.                 |
| `disabled` | `false`                                | Disables the `container` module.          |

### Variables

| Variable  | Beispiel            | Beschreibung                          |
| --------- | ------------------- | ------------------------------------- |
| name      | `fedora-toolbox:35` | The name of the container             |
| symbol    |                     | Spiegelt den Wert der Option `symbol` |
| style\* |                     | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[container]
format = "[$symbol \\[$name\\]]($style) "
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/). By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `shard.yml`-Datei
- The current directory contains a `.cr` file

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | The symbol used before displaying the version of crystal.                 |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | Stil für dieses Modul.                                                    |
| `detect_extensions` | `["cr"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["shard.yml"]`                      | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `crystal` module.                                            |

### Variables

| Variable  | Beispiel  | Beschreibung                          |
| --------- | --------- | ------------------------------------- |
| version   | `v0.32.1` | The version of `crystal`              |
| symbol    |           | Spiegelt den Wert der Option `symbol` |
| style\* |           | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers) SDK version when you are in the root directory of your Daml project. The `sdk-version` in the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION` environment variable. By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `daml.yaml`-Datei

### Optionen

| Option              | Standardwert                       | Beschreibung                                                              |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `via [$symbol($version )]($style)` | Das Format für das Modul.                                                 |
| `version_format`    | `v${raw}`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"Λ "`                             | A format string representing the symbol of Daml                           |
| `style`             | `"bold cyan"`                      | Stil für dieses Modul.                                                    |
| `detect_extensions` | `[]`                               | Which extensions should trigger this module.                              |
| `detect_files`      | `["daml.yaml"]`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                               | Which folders should trigger this module.                                 |
| `disabled`          | `false`                            | Disables the `daml` module.                                               |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v2.2.0` | The version of `daml`                 |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[daml]
format = "via [D $version](bold bright-green) "
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- Das aktuelle Verzeichnis enthält `pubspec.yaml`, `pubspec.yml` oder `pubspec.lock`

### Optionen

| Option              | Standardwert                                      | Beschreibung                                                              |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | A format string representing the symbol of Dart                           |
| `detect_extensions` | `["dart"]`                                        | Which extensions should trigger this module.                              |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".dart_tool"]`                                  | Which folders should trigger this module.                                 |
| `style`             | `"bold blue"`                                     | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                           | Disables the `dart` module.                                               |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v2.8.4` | The version of `dart`                 |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Optionen

| Option              | Standardwert                                                            | Beschreibung                                                              |
| ------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                    | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                                                  | A format string representing the symbol of Deno                           |
| `detect_extensions` | `[]`                                                                    | Which extensions should trigger this module.                              |
| `detect_files`      | `["deno.json", "deno.jsonc", "mod.ts", "mod.js", "deps.ts", "deps.js"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                    | Which folders should trigger this module.                                 |
| `style`             | `"green bold"`                                                          | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                                                 | Disables the `deno` module.                                               |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v1.8.3` | The version of `deno`                 |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

### Beispiel

```toml
# ~/.config/starship.toml

[deno]
format = "via [🦕 $version](green bold) "
```

## Verzeichnis

Das `directory` -Modul zeigt den Pfad zu Ihrem aktuellen Verzeichnis an, abgeschnitten auf drei übergeordnete Ordner. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Optionen

| Option              | Standardwert                                                                                                | Beschreibung                                                                            |
| ------------------- | ----------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                                                                         | Die Anzahl der übergeordneten Ordner, die angezeigt werden.                             |
| `truncate_to_repo`  | `true`                                                                                                      | Whether or not to truncate to the root of the git repo that you're currently in.        |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "`                                                          | Das Format für das Modul.                                                               |
| `style`             | `"bold cyan"`                                                                                               | Stil für dieses Modul.                                                                  |
| `disabled`          | `false`                                                                                                     | Deaktiviert das `directory`-Modul.                                                      |
| `read_only`         | `"🔒"`                                                                                                       | The symbol indicating current directory is read only.                                   |
| `read_only_style`   | `"red"`                                                                                                     | The style for the read only symbol.                                                     |
| `truncation_symbol` | `""`                                                                                                        | The symbol to prefix to truncated paths. eg: "…/"                                       |
| `repo_root_style`   | `None`                                                                                                      | The style for the root of the git repo. The default value is equivalent to `style`.     |
| `repo_root_format`  | `"[$before_root_path]($style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) "` | The format of a git repo when `repo_root_style` is defined.                             |
| `home_symbol`       | `"~"`                                                                                                       | The symbol indicating home directory.                                                   |
| `use_os_path_sep`   | `true`                                                                                                      | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows) |

<details>
<summary>Dieses Modul hat einige erweiterte Konfigurationsoptionen, welche die Darstellung von Verzeichnissen steuern.</summary>

| Advanced Option             | Standardwert | Beschreibung                                                                                                                                                           |
| --------------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |              | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`          | The number of characters to use when applying fish shell pwd path logic.                                                                                               |
| `use_logical_path`          | `true`       | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable  | Beispiel              | Beschreibung                         |
| --------- | --------------------- | ------------------------------------ |
| path      | `"D:/Projects"`       | The current directory path           |
| style\* | `"black bold dimmed"` | Spiegelt den Wert der Option `style` |

*: This variable can only be used as a part of a style string

<details>
<summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| Variable           | Beispiel              | Beschreibung                            |
| ------------------ | --------------------- | --------------------------------------- |
| before_root_path | `"/path/to/home/"`    | The path before git root directory path |
| repo_root          | `"git_repo"`          | The git root directory name             |
| path               | `"/src/lib"`          | The remaining path                      |
| style              | `"black bold dimmed"` | Spiegelt den Wert der Option `style`    |
| repo_root_style  | `"underline white"`   | Style for git root directory name       |

</details>

### Beispiel

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### Optionen

| Option              | Standardwert                                                  | Beschreibung                                                                      |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | Das Format für das Modul.                                                         |
| `symbol`            | `"🐳 "`                                                        | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                        | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                          | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                          | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `"blue bold"`                                                 | Stil für dieses Modul.                                                            |
| `disabled`          | `false`                                                       | Disables the `docker_context` module.                                             |

### Variables

| Variable  | Beispiel       | Beschreibung                          |
| --------- | -------------- | ------------------------------------- |
| context   | `test_context` | The current docker context            |
| symbol    |                | Spiegelt den Wert der Option `symbol` |
| style\* |                | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
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

### Optionen

| Option              | Standardwert                                                                                            | Beschreibung                                                              |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                                                                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | Symbol das vor der dotnet-Version angezeigt wird.                         |
| `heuristic`         | `true`                                                                                                  | Schnelle Versionserkennung nutzen um Starship bedienbar zu halten.        |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Which extensions should trigger this module.                              |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                    | Which folders should trigger this modules.                                |
| `style`             | `"bold blue"`                                                                                           | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                                                                                 | Deaktiviert das `dotnet`-Modul.                                           |

### Variables

| Variable  | Beispiel         | Beschreibung                                                       |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Spiegelt den Wert der Option `symbol`                              |
| style\* |                  | Spiegelt den Wert der Option `style`                               |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/). By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `mix.exs`-Datei.

### Optionen

| Option              | Standardwert                                                | Beschreibung                                                              |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                                         |
| `version_format`    | `"v${raw}"`                                                 | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | The symbol used before displaying the version of Elixir/Erlang.           |
| `detect_extensions` | `[]`                                                        | Which extensions should trigger this module.                              |
| `detect_files`      | `["mix.exs"]`                                               | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                        | Which folders should trigger this modules.                                |
| `style`             | `"bold purple"`                                             | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                             |

### Variables

| Variable    | Beispiel | Beschreibung                          |
| ----------- | -------- | ------------------------------------- |
| version     | `v1.10`  | The version of `elixir`               |
| otp_version |          | The otp version of `elixir`           |
| symbol      |          | Spiegelt den Wert der Option `symbol` |
| style\*   |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/). By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `elm.json`-Datei
- Das aktuelle Verzeichnis enthält eine `elm-package.json`-Datei
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains `*.elm` files

### Optionen

| Option              | Standardwert                                       | Beschreibung                                                              |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                                        | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | A format string representing the symbol of Elm.                           |
| `detect_extensions` | `["elm"]`                                          | Which extensions should trigger this module.                              |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `["elm-stuff"]`                                    | Which folders should trigger this modules.                                |
| `style`             | `"cyan bold"`                                      | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                |

### Variables

| Variable  | Beispiel  | Beschreibung                          |
| --------- | --------- | ------------------------------------- |
| version   | `v0.19.1` | The version of `elm`                  |
| symbol    |           | Spiegelt den Wert der Option `symbol` |
| style\* |           | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Umgebungsvariablen

The `env_var` module displays the current value of a selected environment variables. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

::: tip

Multiple environmental variables can be displayed by using a `.`. (see example) If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.

Example: following configuration will display value of USER environment variable

```toml
# ~/.config/starship.toml

[env_var.USER]
default = "unknown user"
```

:::

### Optionen

| Option     | Standardwert                   | Beschreibung                                                                             |
| ---------- | ------------------------------ | ---------------------------------------------------------------------------------------- |
| `symbol`   | `""`                           | Das Symbol, das vor der Anzeige der Variable verwendet wird.                             |
| `variable` |                                | Die anzuzeigende Umgebungsvariable.                                                      |
| `default`  |                                | Der Standardwert, der angezeigt wird, wenn die ausgewählte Variable nicht definiert ist. |
| `format`   | `"with [$env_value]($style) "` | Das Format für das Modul.                                                                |
| `disabled` | `false`                        | Deaktiviert das `env_var`-Modul.                                                         |

### Variables

| Variable  | Beispiel                                    | Beschreibung                               |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Spiegelt den Wert der Option `symbol`      |
| style\* | `black bold dimmed`                         | Spiegelt den Wert der Option `style`       |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

Displaying multiple environmental variables:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = "SHELL"
default = "unknown shell"
[env_var.USER]
default = "unknown user"
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `rebar.config`-Datei.
- Das aktuelle Verzeichnis enthält eine `erlang.mk`-Datei.

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | The symbol used before displaying the version of erlang.                  |
| `style`             | `"bold red"`                         | Stil für dieses Modul.                                                    |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `disabled`          | `false`                              | Disables the `erlang` module.                                             |

### Variables

| Variable  | Beispiel  | Beschreibung                          |
| --------- | --------- | ------------------------------------- |
| version   | `v22.1.3` | The version of `erlang`               |
| symbol    |           | Spiegelt den Wert der Option `symbol` |
| style\* |           | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are present in a line they will split the space evenly between them. This is useful for aligning other modules.

### Optionen

| Option     | Standardwert   | Beschreibung                      |
| ---------- | -------------- | --------------------------------- |
| `symbol`   | `"."`          | The symbol used to fill the line. |
| `style`    | `"bold black"` | Stil für dieses Modul.            |
| `disabled` | `false`        | Disables the `fill` module        |

### Beispiel

```toml
# ~/.config/starship.toml
format = "AA $fill BB $fill CC"

[fill]
symbol = "-"
style = "bold green"
```

Produces a prompt that looks like:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Optionen

| Option            | Standardwert                                               | Beschreibung                                                     |
| ----------------- | ---------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Das Format für das Modul.                                        |
| `symbol`          | `"☁️  "`                                                   | The symbol used before displaying the current GCP profile.       |
| `region_aliases`  |                                                            | Table of region aliases to display in addition to the GCP name.  |
| `project_aliases` |                                                            | Table of project aliases to display in addition to the GCP name. |
| `style`           | `"bold blue"`                                              | Stil für dieses Modul.                                           |
| `disabled`        | `false`                                                    | Disables the `gcloud` module.                                    |

### Variables

| Variable  | Beispiel      | Beschreibung                                                       |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | The current GCP region                                             |
| account   | `foo`         | The current GCP profile                                            |
| domain    | `example.com` | The current GCP profile domain                                     |
| project   |               | The current GCP project                                            |
| active    | `default`     | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |               | Spiegelt den Wert der Option `symbol`                              |
| style\* |               | Spiegelt den Wert der Option `style`                               |

*: This variable can only be used as a part of a style string

### Beispiele

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
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Display account and aliased region

```toml
# ~/.config/starship.toml

[gcloud]
symbol = "️🇬️ "
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"
```

#### Display account and aliased project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = "vlpn"
```

## Git-Branch

Das `git_branch`-Modul zeigt den aktiven Git-Branch des Repositories im aktuellen Verzeichnis an.

### Optionen

| Option               | Standardwert                                      | Beschreibung                                                                             |
| -------------------- | ------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Shows the remote tracking branch name, even if it is equal to the local branch name.     |
| `format`             | `"on [$symbol$branch(:$remote_branch)]($style) "` | Das Format für das Modul. Use `"$branch"` to refer to the current branch name.           |
| `symbol`             | `" "`                                            | A format string representing the symbol of git branch.                                   |
| `style`              | `"bold purple"`                                   | Stil für dieses Modul.                                                                   |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                                                 |
| `truncation_symbol`  | `"…"`                                             | The symbol used to indicate a branch name was truncated. You can use `""` for no symbol. |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state.                           |
| `ignore_branches`    | `[]`                                              | A list of names to avoid displaying. Useful for "master" or "main".                      |
| `disabled`           | `false`                                           | Deaktiviert das `git_branch`-Modul.                                                      |

### Variables

| Variable      | Beispiel | Beschreibung                                                                                           |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | Spiegelt den Wert der Option `symbol`                                                                  |
| style\*     |          | Spiegelt den Wert der Option `style`                                                                   |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
ignore_branches = ["master", "main"]
```

## Git Commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Optionen

| Option               | Standardwert                       | Beschreibung                                                                         |
| -------------------- | ---------------------------------- | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                                | The length of the displayed git commit hash.                                         |
| `format`             | `"[\\($hash$tag\\)]($style) "` | Das Format für das Modul.                                                            |
| `style`              | `"bold green"`                     | Stil für dieses Modul.                                                               |
| `only_detached`      | `true`                             | Only show git commit hash when in detached `HEAD` state                              |
| `tag_disabled`       | `true`                             | Disables showing tag info in `git_commit` module.                                    |
| `tag_max_candidates` | `0`                                | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `" 🏷 "`                            | Tag symbol prefixing the info shown                                                  |
| `disabled`           | `false`                            | Disables the `git_commit` module.                                                    |

### Variables

| Variable  | Beispiel  | Beschreibung                         |
| --------- | --------- | ------------------------------------ |
| hash      | `b703eb3` | The current git commit hash          |
| style\* |           | Spiegelt den Wert der Option `style` |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git-Zustand

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Optionen

| Option         | Standardwert                                                    | Beschreibung                                                                            |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | Stil für dieses Modul.                                                                  |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Das Format für das Modul.                                                               |
| `disabled`     | `false`                                                         | Deaktiviert das `git_state`-Modul.                                                      |

### Variables

| Variable         | Beispiel   | Beschreibung                         |
| ---------------- | ---------- | ------------------------------------ |
| state            | `REBASING` | The current state of the repo        |
| progress_current | `1`        | The current operation progress       |
| progress_total   | `2`        | The total operation progress         |
| style\*        |            | Spiegelt den Wert der Option `style` |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in the current git repository.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Option               | Standardwert                                                 | Beschreibung                          |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `"bold green"`                                               | The style for the added count.        |
| `deleted_style`      | `"bold red"`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Das Format für das Modul.             |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |

### Variables

| Variable          | Beispiel | Beschreibung                                |
| ----------------- | -------- | ------------------------------------------- |
| added             | `1`      | The current number of added lines           |
| deleted           | `2`      | The current number of deleted lines         |
| added_style\*   |          | Mirrors the value of option `added_style`   |
| deleted_style\* |          | Mirrors the value of option `deleted_style` |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = "bold blue"
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git-Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

::: tip

The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment. You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

:::

### Optionen

| Option              | Standardwert                                    | Beschreibung                                                                                                |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status`                                                                         |
| `conflicted`        | `"="`                                           | This branch has merge conflicts.                                                                            |
| `ahead`             | `"⇡"`                                           | The format of `ahead`                                                                                       |
| `behind`            | `"⇣"`                                           | The format of `behind`                                                                                      |
| `diverged`          | `"⇕"`                                           | The format of `diverged`                                                                                    |
| `up_to_date`        | `""`                                            | The format of `up_to_date`                                                                                  |
| `untracked`         | `"?"`                                           | The format of `untracked`                                                                                   |
| `stashed`           | `"$"`                                           | The format of `stashed`                                                                                     |
| `modified`          | `"!"`                                           | The format of `modified`                                                                                    |
| `staged`            | `"+"`                                           | The format of `staged`                                                                                      |
| `renamed`           | `"»"`                                           | The format of `renamed`                                                                                     |
| `deleted`           | `"✘"`                                           | The format of `deleted`                                                                                     |
| `style`             | `"bold red"`                                    | Stil für dieses Modul.                                                                                      |
| `ignore_submodules` | `false`                                         | Ignore changes to submodules.                                                                               |
| `disabled`          | `false`                                         | Deaktiviert das `git_status`-Modul.                                                                         |
| `windows_starship`  |                                                 | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |

### Variables

The following variables can be used in `format`:

| Variable       | Beschreibung                                                                                                  |
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
| style\*      | Spiegelt den Wert der Option `style`                                                                          |

*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Variable       | Beschreibung                                   |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Variable | Beschreibung             |
| -------- | ------------------------ |
| `count`  | Show the number of files |

### Beispiel

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
up_to_date = "✓"
untracked = "🤷"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

Use Windows Starship executable on Windows paths in WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/). By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `go.mod`-Datei
- Das aktuelle Verzeichnis enthält eine `go.sum`-Datei
- Das aktuelle Verzeichnis enthält eine `go.work`-Datei
- Das aktuelle Verzeichnis enthält eine `glide.yaml`-Datei
- Das aktuelle Verzeichnis enthält eine `Gopkg.yml`-Datei
- Das aktuelle Verzeichnis enthält eine `Gopkg.lock`-Datei
- The current directory contains a `.go-version` file
- Das aktuelle Verzeichnis enthält ein `Godeps`-Verzeichnis
- Das aktuelle Verzeichnis enthält eine Datei mit der `.go`-Erweiterung

### Optionen

| Option              | Standardwert                                                                              | Beschreibung                                                              |
| ------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                      | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                                                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                                    | A format string representing the symbol of Go.                            |
| `detect_extensions` | `["go"]`                                                                                  | Which extensions should trigger this module.                              |
| `detect_files`      | `["go.mod", "go.sum", "go.work", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `["Godeps"]`                                                                              | Which folders should trigger this module.                                 |
| `style`             | `"bold cyan"`                                                                             | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                                                                   | Deaktiviert das `golang`-Modul.                                           |

### Variables

| Variable  | Beispiel  | Beschreibung                          |
| --------- | --------- | ------------------------------------- |
| version   | `v1.12.1` | The version of `go`                   |
| symbol    |           | Spiegelt den Wert der Option `symbol` |
| style\* |           | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `stack.yaml`-Datei
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### Optionen

| Option              | Standardwert                         | Beschreibung                                       |
| ------------------- | ------------------------------------ | -------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                          |
| `symbol`            | `"λ "`                               | A format string representing the symbol of Haskell |
| `detect_extensions` | `["hs", "cabal", "hs-boot"]`         | Which extensions should trigger this module.       |
| `detect_files`      | `["stack.yaml", "cabal.project"]`    | Which filenames should trigger this module.        |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.          |
| `style`             | `"bold purple"`                      | Stil für dieses Modul.                             |
| `disabled`          | `false`                              | Disables the `haskell` module.                     |

### Variables

| Variable       | Beispiel    | Beschreibung                                                                            |
| -------------- | ----------- | --------------------------------------------------------------------------------------- |
| version        |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot       | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbol         |             | Spiegelt den Wert der Option `symbol`                                                   |
| style\*      |             | Spiegelt den Wert der Option `style`                                                    |

*: This variable can only be used as a part of a style string

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `helmfile.yaml`-Datei
- The current directory contains a `Chart.yaml` file

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm.                          |
| `style`             | `"bold white"`                       | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Disables the `helm` module.                                               |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v3.1.1` | The version of `helm`                 |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

Das `hostname`-Modul zeigt den Hostnamen des Systems an.

### Optionen

| Option       | Standardwert                           | Beschreibung                                                                                                                         |
| ------------ | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only`   | `true`                                 | Zeigt den Hostnamen nur, wenn via SSH-Sitzung verbunden.                                                                             |
| `ssh_symbol` | `"🌐 "`                                 | A format string representing the symbol when connected to SSH session.                                                               |
| `trim_at`    | `"."`                                  | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `format`     | `"[$ssh_symbol$hostname]($style) in "` | Das Format für das Modul.                                                                                                            |
| `style`      | `"bold dimmed green"`                  | Stil für dieses Modul.                                                                                                               |
| `disabled`   | `false`                                | Deaktiviert das `hostname`-Modul.                                                                                                    |

### Variables

| Variable   | Beispiel   | Beschreibung                                          |
| ---------- | ---------- | ----------------------------------------------------- |
| hostname   | `computer` | The hostname of the computer                          |
| style\*  |            | Spiegelt den Wert der Option `style`                  |
| ssh_symbol | `"🌏 "`     | The symbol to represent when connected to SSH session |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = "[$ssh_symbol](bold blue) on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Optionen

| Option              | Standardwert                                                                                              | Beschreibung                                                              |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                                                                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                      | Which folders should trigger this modules.                                |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java                           |
| `style`             | `"red dimmed"`                                                                                            | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                                                                                   | Deaktiviert das `Java`-Modul.                                             |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v14`    | The version of `java`                 |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
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

### Optionen

| Option             | Standardwert                  | Beschreibung                                                                     |
| ------------------ | ----------------------------- | -------------------------------------------------------------------------------- |
| `threshold`*       | `1`                           | Zeigt die Anzahl der Jobs wenn der angegebene Schwellenwert überschritten wurde. |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.                   |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`.         |
| `format`           | `"[$symbol$number]($style) "` | Das Format für das Modul.                                                        |
| `symbol`           | `"✦"`                         | The string used to represent the `symbol` variable.                              |
| `style`            | `"bold blue"`                 | Stil für dieses Modul.                                                           |
| `disabled`         | `false`                       | Deaktiviert das `jobs`-Modul.                                                    |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| number    | `1`      | The number of jobs                    |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
number_threshold = 4
symbol_threshold = 0
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.                         |
| `style`             | `"bold purple"`                      | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Disables the `julia` module.                                              |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v1.4.0` | The version of `julia`                |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.kt` or a `.kts` file

### Optionen

| Option              | Standardwert                         | Beschreibung                                                                  |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`     |
| `detect_extensions` | `["kt", "kts"]`                      | Which extensions should trigger this module.                                  |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                                   |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                    |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `"bold blue"`                        | Stil für dieses Modul.                                                        |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Variables

| Variable  | Beispiel  | Beschreibung                          |
| --------- | --------- | ------------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`               |
| symbol    |           | Spiegelt den Wert der Option `symbol` |
| style\* |           | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = "kotlinc"
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-context --namespace astronaut`. Similarly the user and cluster can be set with `kubectl config set-context starship-context --user starship-user` and `kubectl config set-context starship-context --cluster starship-cluster`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

When the module is enabled it will always be active, unless any of `detect_extensions`, `detect_files` or `detect_folders` have been st in which case the module will only be active in directories that match those conditions.

:::

### Optionen

| Option              | Standardwert                                         | Beschreibung                                                          |
| ------------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`            | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | Das Format für das Modul.                                             |
| `style`             | `"cyan bold"`                                        | Stil für dieses Modul.                                                |
| `context_aliases`   |                                                      | Table of context aliases to display.                                  |
| `user_aliases`      |                                                      | Table of user aliases to display.                                     |
| `detect_extensions` | `[]`                                                 | Which extensions should trigger this module.                          |
| `detect_files`      | `[]`                                                 | Which filenames should trigger this module.                           |
| `detect_folders`    | `[]`                                                 | Which folders should trigger this modules.                            |
| `disabled`          | `true`                                               | Deaktiviert das `kubernetes`-Modul.                                   |

### Variables

| Variable  | Beispiel             | Beschreibung                             |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | Spiegelt den Wert der Option `symbol`    |
| style\* |                      | Spiegelt den Wert der Option `style`     |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
".*/openshift-cluster/.*" = "openshift"
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
[kubernetes.user_aliases]
"dev.local.cluster.k8s" = "dev"
"root/.*" = "root"
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
".*/openshift-cluster/.*" = "openshift"
# Or better, to rename every OpenShift cluster at once:
".*/(?P<var_cluster>[\\w-]+)/.*" = "$var_cluster"

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
```

## Zeilenumbruch

Das `line_break`-Modul unterteilt den Prompt in zwei Zeilen.

### Optionen

| Option     | Standardwert | Beschreibung                                                           |
| ---------- | ------------ | ---------------------------------------------------------------------- |
| `disabled` | `false`      | Deaktiviert das `line_break`-Modul, wodurch der Prompt einzeilig wird. |

### Beispiel

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

The `localip` module shows the IPv4 address of the primary network interface.

### Optionen

| Option     | Standardwert              | Beschreibung                                           |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `"[$localipv4]($style) "` | Das Format für das Modul.                              |
| `style`    | `"bold yellow"`           | Stil für dieses Modul.                                 |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### Variables

| Variable  | Beispiel     | Beschreibung                         |
| --------- | ------------ | ------------------------------------ |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address    |
| style\* |              | Spiegelt den Wert der Option `style` |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = "@[$localipv4](bold red) "
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Optionen

| Option              | Standardwert                         | Beschreibung                                                               |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                  |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`  |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | Which extensions should trigger this module.                               |
| `detect_files`      | `[".lua-version"]`                   | Which filenames should trigger this module.                                |
| `detect_folders`    | `["lua"]`                            | Which folders should trigger this module.                                  |
| `style`             | `"bold blue"`                        | Stil für dieses Modul.                                                     |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v5.4.0` | The version of `lua`                  |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Speicherauslastung

Das `memory_usage` Modul zeigt den aktuellen Systemspeicher und die swap-Nutzung an.

Standardmäßig wird die swap-Nutzung angezeigt, wenn der gesamte System-swap nicht Null ist.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Option      | Standardwert                                    | Beschreibung                                                          |
| ----------- | ----------------------------------------------- | --------------------------------------------------------------------- |
| `threshold` | `75`                                            | Speicherauslastung ausblenden, wenn sie unter diesem Prozentsatz ist. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | Das Format für das Modul.                                             |
| `symbol`    | `"🐏"`                                           | Symbol das vor der Speicherauslastung angezeigt wird.                 |
| `style`     | `"bold dimmed white"`                           | Stil für dieses Modul.                                                |
| `disabled`  | `true`                                          | Deaktiviert das `memory_usage`-Modul.                                 |

### Variables

| Variable         | Beispiel      | Beschreibung                                                       |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Spiegelt den Wert der Option `symbol`                              |
| style\*        |               | Spiegelt den Wert der Option `style`                               |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

### Beispiel

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Optionen

| Option              | Standardwert                     | Beschreibung                                                                                 |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | Stil für dieses Modul.                                                                       |
| `format`            | `"on [$symbol$branch]($style) "` | Das Format für das Modul.                                                                    |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| branch    | `master` | The active mercurial branch           |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `nim.cfg`-Datei
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul                                                  |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.                     |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Which extensions should trigger this module.                              |
| `detect_files`      | `["nim.cfg"]`                        | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold yellow"`                      | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Disables the `nim` module.                                                |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v1.2.0` | The version of `nimc`                 |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-Shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. Das Modul wird angezeigt, wenn es sich in einer nix-Shell-Umgebung befindet.

### Optionen

| Option       | Standardwert                                   | Beschreibung                                          |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | Das Format für das Modul.                             |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | Stil für dieses Modul.                                |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Deaktiviert das `nix_shell`-Modul.                    |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| state     | `pure`   | The state of the nix-shell            |
| name      | `lorri`  | The name of the nix-shell             |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `package.json`-Datei
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- Das aktuelle Verzeichnis enthält ein `node_modules`-Verzeichnis
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

### Optionen

| Option              | Standardwert                               | Beschreibung                                                                                          |
| ------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`       | Das Format für das Modul.                                                                             |
| `version_format`    | `"v${raw}"`                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                             |
| `symbol`            | `" "`                                     | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts", "mts", "cts"]` | Which extensions should trigger this module.                                                          |
| `detect_files`      | `["package.json", ".node-version"]`        | Which filenames should trigger this module.                                                           |
| `detect_folders`    | `["node_modules"]`                         | Which folders should trigger this module.                                                             |
| `style`             | `"bold green"`                             | Stil für dieses Modul.                                                                                |
| `disabled`          | `false`                                    | Deaktiviert das `nodejs`-Modul.                                                                       |
| `not_capable_style` | `bold red`                                 | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variables

| Variable  | Beispiel   | Beschreibung                          |
| --------- | ---------- | ------------------------------------- |
| version   | `v13.12.0` | The version of `node`                 |
| symbol    |            | Spiegelt den Wert der Option `symbol` |
| style\* |            | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Optionen

| Option                    | Standardwert                                                               | Beschreibung                                                              |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | The format string for the module.                                         |
| `version_format`          | `"v${raw}"`                                                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml.                   |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch.                   |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.                    |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Which extensions should trigger this module.                              |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Which filenames should trigger this module.                               |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Which folders should trigger this module.                                 |
| `style`                   | `"bold yellow"`                                                            | Stil für dieses Modul.                                                    |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                              |

### Variables

| Variable         | Beispiel     | Beschreibung                                                      |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Spiegelt den Wert der Option `symbol`                             |
| style\*        |              | Spiegelt den Wert der Option `style`                              |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Optionen

| Option     | Standardwert                                        | Beschreibung                                                   |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | Das Format für das Modul.                                      |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | Stil für dieses Modul.                                         |
| `disabled` | `false`                                             | Disables the `openstack` module.                               |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| cloud     | `corp`   | The current OpenStack cloud           |
| project   | `dev`    | The current OpenStack project         |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Paketversion

Das `Package` Modul wird angezeigt, wenn das aktuelle Verzeichnis das Repository für ein Paket ist, und zeigt dessen aktuelle Version an. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `daml` and `dart` packages.

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

> ⚠️ Die angezeigte Version ist die des Pakets, dessen Quellcode im Verzeichnis liegt, nicht die des Paketmanagers.

### Optionen

| Option            | Standardwert                      | Beschreibung                                                              |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | Das Format für das Modul.                                                 |
| `symbol`          | `"📦 "`                            | Symbol das vor der Paketversion angezeigt wird.                           |
| `version_format`  | `"v${raw}"`                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`           | `"bold 208"`                      | Stil für dieses Modul.                                                    |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.                 |
| `disabled`        | `false`                           | Deaktiviert das `package`-Modul.                                          |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v1.0.0` | The version of your package           |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Optionen

| Option              | Standardwert                                                                                             | Beschreibung                                                              |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                                                                                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl                     |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Which extensions should trigger this module.                              |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                     | Which folders should trigger this module.                                 |
| `style`             | `"bold 149"`                                                                                             | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                               |

### Variables

| Variable  | Beispiel  | Beschreibung                          |
| --------- | --------- | ------------------------------------- |
| version   | `v5.26.1` | The version of `perl`                 |
| symbol    |           | Spiegelt den Wert der Option `symbol` |
| style\* |           | Spiegelt den Wert der Option `style`  |

### Beispiel

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `composer.json`-Datei
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | Symbol das vor der PHP-Version angezeigt wird.                            |
| `detect_extensions` | `["php"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `["composer.json", ".php-version"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"147 bold"`                         | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Deaktiviert das `php`-Modul.                                              |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v7.3.8` | The version of `php`                  |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

::: tip

By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). If you still want to enable it, [follow the example shown below](#with-pulumi-version).

:::

By default the module will be shown if any of the following conditions are met:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Optionen

| Option           | Standardwert                                 | Beschreibung                                                              |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`         | `"via [$symbol($username@)$stack]($style) "` | The format string for the module.                                         |
| `version_format` | `"v${raw}"`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `" "`                                       | A format string shown before the Pulumi stack.                            |
| `style`          | `"bold 5"`                                   | Stil für dieses Modul.                                                    |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.            |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                             |

### Variables

| Variable     | Beispiel   | Beschreibung                          |
| ------------ | ---------- | ------------------------------------- |
| version      | `v0.12.24` | The version of `pulumi`               |
| stack        | `dev`      | The current Pulumi stack              |
| benutzername | `alice`    | The current Pulumi username           |
| symbol       |            | Spiegelt den Wert der Option `symbol` |
| style\*    |            | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

#### With Pulumi Version

```toml
# ~/.config/starship.toml

[pulumi]
format = "[🛥 ($version )$stack]($style) "
```

#### Without Pulumi version

```toml
# ~/.config/starship.toml
[pulumi]
symbol = "🛥 "
format = "[$symbol$stack]($style) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `spago.dhall`-Datei
- The current directory contains a file with the `.purs` extension

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript.              |
| `detect_extensions` | `["purs"]`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `["spago.dhall"]`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold white"`                       | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Disables the `purescript` module.                                         |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `0.13.5` | The version of `purescript`           |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `.python-version`-Datei
- Das aktuelle Verzeichnis enthält eine `Pipfile`-Datei
- The current directory contains a `__init__.py` file
- Das aktuelle Verzeichnis enthält eine `pyproject.toml`-Datei
- Das aktuelle Verzeichnis enthält eine `requirements.txt`-Datei
- Das aktuelle Verzeichnis enthält eine `setup.py`-Datei
- Das aktuelle Verzeichnis enthält eine `tox.ini`-Datei
- Das aktuelle Verzeichnis enthält eine Datei mit der `.py`-Erweiterung.
- Ein virtualenv ist momentan aktiv

### Optionen

| Option               | Standardwert                                                                                                 | Beschreibung                                                                           |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Das Format für das Modul.                                                              |
| `version_format`     | `"v${raw}"`                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch`              |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | Stil für dieses Modul.                                                                 |
| `pyenv_version_name` | `false`                                                                                                      | Verwende `pyenv` um die Python-Versionzu beziehen.                                     |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `["py"]`                                                                                                     | Which extensions should trigger this module                                            |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Which filenames should trigger this module                                             |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                               |
| `disabled`           | `false`                                                                                                      | Deaktiviert das `python`-Modul.                                                        |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Variables

| Variable     | Beispiel        | Beschreibung                               |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Spiegelt den Wert der Option `symbol`      |
| style        | `"yellow bold"` | Spiegelt den Wert der Option `style`       |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |

### Beispiel

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = "python3"
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
python_binary = ["./venv/bin/python", "python", "python3", "python2"]
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

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"📐"`                                | A format string representing the symbol of R.                             |
| `style`             | `"blue bold"`                        | Stil für dieses Modul.                                                    |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | Which extensions should trigger this module                               |
| `detect_files`      | `[".Rprofile"]`                      | Which filenames should trigger this module                                |
| `detect_folders`    | `[".Rproj.user"]`                    | Which folders should trigger this module                                  |
| `disabled`          | `false`                              | Disables the `r` module.                                                  |

### Variables

| Variable | Beispiel      | Beschreibung                          |
| -------- | ------------- | ------------------------------------- |
| version  | `v4.0.5`      | The version of `R`                    |
| symbol   |               | Spiegelt den Wert der Option `symbol` |
| style    | `"blue bold"` | Spiegelt den Wert der Option `style`  |

### Beispiel

```toml
# ~/.config/starship.toml

[rlang]
format = "with [📐 $version](blue bold) "
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Optionen

| Option              | Standardwert                                     | Beschreibung                                                              |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version-$vm_version )]($style)"` | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                                      | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦋 "`                                           | The symbol used before displaying the version of Raku                     |
| `detect_extensions` | `["p6", "pm6", "pod6", "raku", "rakumod"]`       | Which extensions should trigger this module.                              |
| `detect_files`      | `["META6.json"]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                             | Which folders should trigger this module.                                 |
| `style`             | `"bold 149"`                                     | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                          | Disables the `raku` module.                                               |

### Variables

| Variable   | Beispiel | Beschreibung                          |
| ---------- | -------- | ------------------------------------- |
| version    | `v6.d`   | The version of `raku`                 |
| vm_version | `moar`   | The version of VM `raku` is built on  |
| symbol     |          | Spiegelt den Wert der Option `symbol` |
| style\*  |          | Spiegelt den Wert der Option `style`  |

### Beispiel

```toml
# ~/.config/starship.toml

[raku]
format = "via [🦪 $version]($style) "
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current directory contains a file with `.red` or `.reds` extension

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                           |
| `detect_extensions` | `["red"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"red bold"`                         | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Disables the `red` module.                                                |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v2.5.1` | The version of `red`                  |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Das aktuelle Verzeichnis enthält eine `Gemfile`-Datei
- The current directory contains a `.ruby-version` file
- Das aktuelle Verzeichnis enthält eine `.rb`-Datei
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                          |
| `detect_extensions` | `["rb"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `detect_variables`  | `["RUBY_VERSION", "RBENV_VERSION"]`  | Which environment variables should trigger this module.                   |
| `style`             | `"bold red"`                         | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Deaktiviert das `ruby`-Modul.                                             |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v2.5.1` | The version of `ruby`                 |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Das aktuelle Verzeichnis enthält eine `Cargo.toml`-Datei
- Das aktuelle Verzeichnis enthält eine Datei mit der `.rs`-Erweiterung

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                           |
| `detect_extensions` | `["rs"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Cargo.toml"]`                     | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold red"`                         | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Deaktiviert das `rust`-Modul.                                             |

### Variables

| Variable  | Beispiel          | Beschreibung                                 |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | Spiegelt den Wert der Option `symbol`        |
| style\* |                   | Spiegelt den Wert der Option `style`         |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Optionen

| Option              | Standardwert                             | Beschreibung                                                              |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Which extensions should trigger this module.                              |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".metals"]`                            | Which folders should trigger this modules.                                |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.                         |
| `style`             | `"red dimmed"`                           | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                  | Disables the `scala` module.                                              |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `2.13.5` | The version of `scala`                |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Option                 | Standardwert              | Beschreibung                                                 |
| ---------------------- | ------------------------- | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`                     | A format string used to represent bash.                      |
| `fish_indicator`       | `fsh`                     | A format string used to represent fish.                      |
| `zsh_indicator`        | `zsh`                     | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`                     | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`                     | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`                     | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`                     | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `xsh`                     | A format string used to represent xonsh.                     |
| `cmd_indicator`        | `⌘ cmd`                   | A format string used to represent cmd.                       |
| `nu_indicator`         | `nu`                      | A format string used to represent nu.                        |
| `unknown_indicator`    |                           | The default value to be displayed when the shell is unknown. |
| `format`               | `"[$indicator]($style) "` | Das Format für das Modul.                                    |
| `style`                | `"white bold"`            | Stil für dieses Modul.                                       |
| `disabled`             | `true`                    | Disables the `shell` module.                                 |

### Variables

| Variable  | Standardwert | Beschreibung                                               |
| --------- | ------------ | ---------------------------------------------------------- |
| indicator |              | Mirrors the value of `indicator` for currently used shell. |
| style\* |              | Spiegelt den Wert der Option `style`.                      |

*: This variable can only be used as a part of a style string

### Beispiele

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
style = "cyan bold"
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Optionen

| Option      | Standardwert                 | Beschreibung                                                  |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | Das Format für das Modul.                                     |
| `symbol`    | `"↕️  "`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | Stil für dieses Modul.                                        |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| shlvl     | `3`      | The current value of `SHLVL`          |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### Optionen

| Option     | Standardwert                     | Beschreibung                                     |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Das Format für das Modul.                        |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | Stil für dieses Modul.                           |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

| Variable  | Beispiel     | Beschreibung                          |
| --------- | ------------ | ------------------------------------- |
| env       | `centos.img` | The current Singularity image         |
| symbol    |              | Spiegelt den Wert der Option `symbol` |
| style\* |              | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Optionen

| Option              | Standardwert                           | Beschreibung                                                                                                                                                              |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to. `0` bedeutet keine Kürzung. Beachte auch die Beschreibung für das [`directory`](#directory) Modul. |
| `symbol`            | `"🅢  "`                                | Symbol das vor dem Umgebungsnamen angezeigt wird.                                                                                                                         |
| `style`             | `"bold blue"`                          | Stil für dieses Modul.                                                                                                                                                    |
| `format`            | `"via [$symbol$environment]($style) "` | Das Format für das Modul.                                                                                                                                                 |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                                              |

### Variables

| Variable    | Beispiel     | Beschreibung                          |
| ----------- | ------------ | ------------------------------------- |
| environment | `astronauts` | The current spack environment         |
| symbol      |              | Spiegelt den Wert der Option `symbol` |
| style\*   |              | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[spack]
format = "[$symbol$environment](dimmed blue) "
```

## Status

The `status` module displays the exit code of the previous command. If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`. The status code will cast to a signed 32-bit integer.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Option                      | Standardwert                                                                         | Beschreibung                                                          |
| --------------------------- | ------------------------------------------------------------------------------------ | --------------------------------------------------------------------- |
| `format`                    | `"[$symbol$status]($style) "`                                                        | The format of the module                                              |
| `symbol`                    | `"✖"`                                                                                | The symbol displayed on program error                                 |
| `success_symbol`            | `""`                                                                                 | The symbol displayed on program success                               |
| `not_executable_symbol`     | `"🚫"`                                                                                | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `"🔍"`                                                                                | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `"🧱"`                                                                                | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `"⚡"`                                                                                | The symbol displayed on any signal                                    |
| `style`                     | `"bold red"`                                                                         | Stil für dieses Modul.                                                |
| `recognize_signal_code`     | `true`                                                                               | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                              | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                              | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                            | The symbol used to separate pipestatus segments                       |
| `pipestatus_format`         | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                      | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                               | Disables the `status` module.                                         |

### Variables

| Variable       | Beispiel | Beschreibung                                                                                |
| -------------- | -------- | ------------------------------------------------------------------------------------------- |
| status         | `127`    | The exit code of the last command                                                           |
| hex_status     | `0x7F`   | The exit code of the last command in hex                                                    |
| int            | `127`    | The exit code of the last command                                                           |
| common_meaning | `ERROR`  | Meaning of the code if not a signal                                                         |
| signal_number  | `9`      | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`   | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`      | Contains the exit code number when no meaning has been found                                |
| pipestatus     |          | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbol         |          | Spiegelt den Wert der Option `symbol`                                                       |
| style\*      |          | Spiegelt den Wert der Option `style`                                                        |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴 "
success_symbol = "🟢 SUCCESS"
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

The `sudo` module displays if sudo credentials are currently cached. The module will only be shown if credentials are cached.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Option          | Standardwert            | Beschreibung                                            |
| --------------- | ----------------------- | ------------------------------------------------------- |
| `format`        | `[as $symbol]($style)"` | The format of the module                                |
| `symbol`        | `"🧙 "`                  | The symbol displayed when credentials are cached        |
| `style`         | `"bold blue"`           | Stil für dieses Modul.                                  |
| `allow_windows` | `false`                 | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                  | Disables the `sudo` module.                             |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[sudo]
style = "bold green"
symbol = "👩‍💻 "
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

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift                          |
| `detect_extensions` | `["swift"]`                          | Which extensions should trigger this module.                              |
| `detect_files`      | `["Package.swift"]`                  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold 202"`                         | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Disables the `swift` module.                                              |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v5.2.4` | The version of `swift`                |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.

::: tip

By default the Terraform version is not shown, since this is slow for current versions of Terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

:::

By default the module will be shown if any of the following conditions are met:

- Das aktuelle Verzeichnis enthält eine `.terraform`-Datei
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                     |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]`        | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".terraform"]`                     | Which folders should trigger this module.                                 |
| `style`             | `"bold 105"`                         | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Deaktiviert das `terraform` Modul.                                        |

### Variables

| Variable  | Beispiel   | Beschreibung                          |
| --------- | ---------- | ------------------------------------- |
| version   | `v0.12.24` | The version of `terraform`            |
| workspace | `default`  | The current Terraform workspace       |
| symbol    |            | Spiegelt den Wert der Option `symbol` |
| style\* |            | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

#### With Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without Terraform version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Uhrzeit

Das `time` Modul zeigt die aktuelle **lokale** Zeit an. Der `format` Wert wird von der crate [`chrono`](https://crates.io/crates/chrono) benutzt um die Zeit zu formatieren. Schau dir [die chrono strftime Dokumentation](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) an, um die möglichen Optionen zu sehen.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Option            | Standardwert            | Beschreibung                                                                                                                                                |
| ----------------- | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                                           |
| `use_12hr`        | `false`                 | Aktiviert die Formatierung der Uhrzeit im 12-Stunden-Format.                                                                                                |
| `time_format`     | Siehe unten             | Das Format zum Anzeigen der Uhrzeit in [chrono-Formatierung](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html).                               |
| `style`           | `"bold yellow"`         | Stil für dieses Modul.                                                                                                                                      |
| `utc_time_offset` | `"local"`               | Legt das UTC-Offset fest, das verwendet werden soll. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Deaktiviert das `time`-Modul.                                                                                                                               |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                                                       |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Ansonsten ist der Standardwert hierfür `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable  | Beispiel   | Beschreibung                         |
| --------- | ---------- | ------------------------------------ |
| uhrzeit   | `13:08:10` | The current time.                    |
| style\* |            | Spiegelt den Wert der Option `style` |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Benutzername

Das Modul `username` zeigt den Benutzernamen des aktiven Benutzers. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current user is root/admin
- Der aktuelle Benutzer ist nicht derjenige, der derzeit angemeldet ist
- Der Benutzer ist über eine SSH-Sitzung verbunden
- Die Variale `show_always` ist auf `true` gesetzt

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Optionen

| Option        | Standardwert            | Beschreibung                                |
| ------------- | ----------------------- | ------------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root/admin. |
| `style_user`  | `"bold yellow"`         | Stil bei allen anderen Benutzern.           |
| `format`      | `"[$user]($style) in "` | Das Format für das Modul.                   |
| `show_always` | `false`                 | `username`-Modul immer anzeigen.            |
| `disabled`    | `false`                 | Deaktiviert das `username`-Modul.           |

### Variables

| Variable | Beispiel     | Beschreibung                                                                                |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `"matchai"`  | The currently logged-in user ID.                                                            |

### Beispiel

```toml
# ~/.config/starship.toml

[username]
style_user = "white bold"
style_root = "black bold"
format = "user: [$user]($style) "
disabled = false
show_always = true
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Vagrantfile` file

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.                       |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["Vagrantfile"]`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"cyan bold"`                        | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                            |

### Variables

| Variable  | Beispiel         | Beschreibung                          |
| --------- | ---------------- | ------------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`              |
| symbol    |                  | Spiegelt den Wert der Option `symbol` |
| style\* |                  | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Optionen

| Option              | Standardwert                                 | Beschreibung                                                              |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"V "`                                       | A format string representing the symbol of V                              |
| `detect_extensions` | `["v"]`                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                         | Which folders should trigger this module.                                 |
| `style`             | `"blue bold"`                                | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                              |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v0.2`   | The version of `v`                    |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

### Beispiel

```toml
# ~/.config/starship.toml
[vlang]
format = "via [V $version](blue bold) "
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### Optionen

| Option     | Standardwert                     | Beschreibung                                           |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   |                                  | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | Stil für dieses Modul.                                 |
| `format`   | `"vcsh [$symbol$repo]($style) "` | Das Format für das Modul.                              |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variables

| Variable  | Beispiel                                    | Beschreibung                          |
| --------- | ------------------------------------------- | ------------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name            |
| symbol    |                                             | Spiegelt den Wert der Option `symbol` |
| style\* | `black bold dimmed`                         | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current directory contains a `.zig` file

### Optionen

| Option              | Standardwert                         | Beschreibung                                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Das Format für das Modul.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.                     |
| `style`             | `"bold yellow"`                      | Stil für dieses Modul.                                                    |
| `disabled`          | `false`                              | Disables the `zig` module.                                                |
| `detect_extensions` | `["zig"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |

### Variables

| Variable  | Beispiel | Beschreibung                          |
| --------- | -------- | ------------------------------------- |
| version   | `v0.6.0` | The version of `zig`                  |
| symbol    |          | Spiegelt den Wert der Option `symbol` |
| style\* |          | Spiegelt den Wert der Option `style`  |

*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
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

### Optionen

| Option              | Standardwert                    | Beschreibung                                                                                                                                                                                                                                                                                  |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `""`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                                                                                                                                 |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the command returns a `0` status code.                                                                          |
| `shell`             |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                            |
| `beschreibung`      | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                  |
| `detect_files`      | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                         |
| `detect_folders`    | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                   |
| `detect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                    |
| `symbol`            | `""`                            | The symbol used before displaying the command output.                                                                                                                                                                                                                                         |
| `style`             | `"bold green"`                  | Stil für dieses Modul.                                                                                                                                                                                                                                                                        |
| `format`            | `"[$symbol($output )]($style)"` | Das Format für das Modul.                                                                                                                                                                                                                                                                     |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                |
| `os`                |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                 |
| `use_stdin`         |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                     |

### Variables

| Variable  | Beschreibung                           |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Spiegelt den Wert der Option `symbol`  |
| style\* | Spiegelt den Wert der Option `style`   |

*: This variable can only be used as a part of a style string

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. If `shell` is not given or only contains one element and Starship detects Cmd will be used, the following argument will automatically be added: `/C` and `stdin` will be set to `false`. If `shell` is not given or only contains one element and Starship detects Nushell will be used, the following arguments will automatically be added: `-c` and `stdin` will be set to `false`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ["pwsh", "-Command", "-"]
```

::: warning Make sure your custom shell configuration exits gracefully

If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).

For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.

Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.

Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

:::

### Beispiel

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo" # shows output of command
detect_files = ["foo"] # can specify filters but wildcards are not supported
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
detect_extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]

[custom.time-as-arg]
command = "time /T"
detect_extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command"]
use_stdin = false
```
