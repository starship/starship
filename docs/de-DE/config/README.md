# Konfiguration

::: tip

🔥 Die Konfiguration befindet sich derzeit noch in der Entwicklung. Viele neue Konfigurationsoptionen werden in den kommenden Versionen verfügbar sein.

:::

Um mit der Konfiguration von Starship zu beginnen, musst du die folgende Datei erstellen: `~/.config/starship.toml`.

```sh
$ mkdir -p ~/.config && touch ~/.config/starship.toml
```

Die gesamte Konfiguration von Starship wird über diese [TOML](https://github.com/toml-lang/toml)-Datei durchgeführt:

```toml
# Kein Zeilenumbruch am Anfang der Eingabe
add_newline = false

# Ersetze das "❯" Symbol in der Konsole mit "➜"
[character]      # Wir nehmen Änderungen im Module "character" vor
symbol = "➜"     # Das "symbol" Feld wird "➜" gesetzt

# Deaktiviere das "package" Modul um es ganz aus der Anzeige zu entfernen
[package]
disabled = true
```

Sie können den Pfad zur `starship.toml` mit der `STARSHIP_CONFIG` Umgebungsvariable ändern:
```sh
export STARSHIP_CONFIG=~/.starship
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:
```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### Terminologie

**Module**: Eine Komponente in der Konsole, die auf kontextualisierte Informationen des OS basiert. Zum Beispiel zeigt das Modul "nodejs" die Version von NodeJS, die derzeit auf Ihrem Computer installiert ist, wenn Ihr aktuelles Verzeichnis ein NodeJS-Projekt ist.

**Segment**: Kleinere Unterkomponenten, die ein Modul erstellen. Zum Beispiel, das "symbol" Segment im "nodejs" Modul beinhaltet das Symbol das vor der Versionsnummer gezeigt wird (Standard: ⬢).

Hier eine Darstellung des Node-Moduls. Im folgenden Beispiel betrachten wir die Segmente "symbol" und "version". Jedes Modul hat einen Präfix und einen Suffix, welche auf die Standardfarbe des Terminals gesetzt sind.

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### Style-Strings

Die meisten Module in Starship lassen dich den Darstellungsstil verändern. Dies passiert meistens an einem bestimmten Eintrag (gewöhnlich `style` genannt), der einen String mit den Einstellungen darstellt. Es folgen ein paar Beispiele für solche Strings zusammen mit Beschreibungen was sie bewirken. Details zur vollen Syntax findest du im [Erweiterten Konfigurationshandbuch](/advanced-config/).

- `"fg:green bg:blue"` setzt grünen Text auf blauen Hintergrund
- `"bg:blue fg:bright-green"` setzt hell-grünen Text auf blauen Hintergrund
- `"bold fg:27"` setzt dicke Schrift auf [ANSI Farbe](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` setzt unterstrichenen Text auf einen orangenen Hintergrund
- `"bold italic fg:purple"` setzt dicke lila Kursivschrift
- `""` deaktiviert explizit jeden Stil

Wie genau sich diese Konfiguration auswirkt liegt an deinem Terminal-Emulator. Einige Emulatoren zum Beispiel werden die Farben erhellen statt Text dick zu machen, und ein paar Farbthemen benutzen dieselben Werte für normale und helle Farben. Für kursiven Text muss dein Terminal Kursivschrift unterstützen.

## Prompt

Dies ist eine Liste mit Prompt-weiten Konfigurationsoptionen.

### Optionen

| Variable       | Standardwert                  | Beschreibung                                                       |
| -------------- | ----------------------------- | ------------------------------------------------------------------ |
| `add_newline`  | `true`                        | Neuer Zeilenumbruch bei Start des Prompts.                         |
| `prompt_order` | [link](#default-prompt-order) | Stelle die Reihenfolge ein, in der die Module den prompt aufbauen. |
| `scan_timeout` | `30`                          | Timeout für das Scannen von Dateien (in Millisekunden).            |

### Beispiel

```toml
# ~/.config/starship.toml

# Kein Zeilenumbrunch am Anfang der Eingabe
add_newline = false
# Ersetze die vorgegebene Reihenfolge der Statuszeile mit einer eigenen
prompt_order=["rust","line_break","package","line_break","character"]
# Gib Starship zehn Millisekunden um die Dateien im akutellen Pfad zu prüfen.
scan_timeout = 10
```

### Standard-Promptreihenfolge

Die Standard `prompt_order` wird benutzt um die Reihenfolge der Module im Prompt zu definieren, falls `prompt_order` leer oder nicht gesetzt ist. Die Standardwerte sind folgende:

```toml
prompt_order = [
    "username",
    "hostname",
    "kubernetes",
    "directory",
    "git_branch",
    "git_commit",
    "git_state",
    "git_status",
    "hg_branch",
    "docker_context",
    "package",
    "dotnet",
    "elixir",
    "elm",
    "erlang",
    "golang",
    "haskell",
    "java",
    "julia",
    "nim",
    "nodejs",
    "ocaml",
    "php",
    "purescript",
    "python",
    "ruby",
    "rust",
    "terraform",
    "zig",
    "nix_shell",
    "conda",
    "memory_usage",
    "aws",
    "env_var",
    "crystal",
    "cmd_duration",
    "custom",
    "line_break",
    "jobs",
    "battery",
    "time",
    "character",
]
```

## AWS

Das `aws`-Modul zeigt das aktuelle AWS-Profil an. Dies basiert auf den Umgebungsvariablen: `AWS_REGION`, `AWS_DEFAULT_REGION`, `AWS_PROFILE` und der `~/.aws/config` Datei.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var.

### Optionen

| Variable          | Standardwert    | Beschreibung                                                                         |
| ----------------- | --------------- | ------------------------------------------------------------------------------------ |
| `symbol`          | `"☁️ "`         | Symbol das vor dem aktuellen AWS-Profil angezeigt wird.                              |
| `displayed_items` | `all`           | Welche Objekte angezeigt werden sollen. Mögliche Werte: [`all`, `profile`, `region`] |
| `region_aliases`  |                 | Table of region aliases to display in addition to the AWS name.                      |
| `style`           | `"bold yellow"` | Stil für dieses Modul.                                                               |
| `disabled`        | `false`         | Deaktiviert das `aws`-Modul.                                                         |

### Beispiel

```toml
# ~/.config/starship.toml

[aws]
style = "bold blue"
symbol = "🅰 "
displayed_items = "region"
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

## Akkustand

Das `battery` Modul zeigt, wie hoch der Akku des Geräts geladen ist und den aktuellen Ladestatus. Das Modul ist nur sichtbar, wenn der Akku des Geräts unter 10% geladen ist.

### Optionen

| Variable             | Standardwert             | Beschreibung                                                                        |
| -------------------- | ------------------------ | ----------------------------------------------------------------------------------- |
| `full_symbol`        | `"•"`                    | Das Symbol das angezeigt wird wenn der Akku voll geladen ist.                       |
| `charging_symbol`    | `"⇡"`                    | Das Symbol das angezeigt wird wenn der Akku aufgeladen wird.                        |
| `discharging_symbol` | `"⇣"`                    | Das Symbol, das angezeigt wird, wenn die Batterie entladen wird.                    |
| `display`            | [link](#battery-display) | Stellt den Grenzwert ein ab dem der Ladezustand (das battery-Modul) angezeigt wird. |
| `disabled`           | `false`                  | Wenn der Wert auf `true` steht, wird das Akkustand-Modul deaktiviert.               |

<details>
<summary>Das Batterie-Modul unterstützt auch einige untypische Zustände.</summary>

| Variable         | Beschreibung                                        |
| ---------------- | --------------------------------------------------- |
| `unknown_symbol` | The symbol shown when the battery state is unknown. |
| `empty_symbol`   | The symbol shown when the battery state is empty.   |

Note: Battery indicator will be hidden if the status is `unknown` or `empty` unless you specify the option in the config.

</details>

### Beispiel

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Anzeige des Akkustandes

Die `display` Konfiguration "threshold" stellt ein ab wann die Akkuanzeige eingeblendet wird. Mit "style" wird das Erscheinungsbild festgelegt. Wenn `display` nicht angegeben ist. Die Standardwerte sind folgende:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Optionen

Die `display`-Option beinhaltet ein Array mit den folgenden Werten.

| Variable    | Beschreibung                                            |
| ----------- | ------------------------------------------------------- |
| `threshold` | Der Schwellenwert zur Anzeige dieser Option.            |
| `style`     | Der Stil, der zur Anzeige dieser Option verwendet wird. |

#### Beispiel

```toml
[[battery.display]]  # "bold red" bei Akkustand zwischen 0–10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" bei Akkustand zwischen 10–30%
threshold = 30
style = "bold yellow"

# Bei Akkustand über 30% wird der Akkustand nicht angezeigt

```

## Zeichen

Das `character` Modul zeigt ein Zeichen ( meistens einen Pfeil "❯") vor der Texteingabe an.

Das Zeichen zeigt an ob der letzte Befehl erfolgreich war, oder einen Fehler erzeugt hat. Das Modul ändert entweder die Farbe, ("style_success","style_failure") standardmäßig Grün/Rot. Oder das Symbol, wenn `use_symbol_for_status` auf `true` steht.

### Optionen

| Variable                | Standardwert   | Beschreibung                                                                                                      |
| ----------------------- | -------------- | ----------------------------------------------------------------------------------------------------------------- |
| `symbol`                | `"❯"`          | Das Symbol, das vor der Texteingabe im Prompt verwendet wird.                                                     |
| `error_symbol`          | `"✖"`          | Das Symbol, das vor der Texteingabe angezeigt wird, wenn der letzte Befehl fehlgeschlagen ist.                    |
| `use_symbol_for_status` | `false`        | Fehlerstatus durch Ändern des Symbols anzeigen.                                                                   |
| `vicmd_symbol`          | `"❮"`          | Das Symbol, das vor Texteingaben im Prompt verwendet wird, wenn die Shell sich im "normal mode" von vim befindet. |
| `style_success`         | `"bold green"` | Der Stil, der verwendet wird, wenn der letzte Befehl erfolgreich war.                                             |
| `style_failure`         | `"bold red"`   | Der Stil, der verwendet wird, wenn der letzte Befehl fehlgeschlagen ist.                                          |
| `disabled`              | `false`        | Deaktiviert das `character`-Modul.                                                                                |

### Beispiel

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## Befehlsdauer

Das `cmd_duration` Modul zeigt an wie lange der letzte Befehl ausgeführt wurde. Das Modul wird nur angezeigt wenn der letzte Befehl länger als zwei Sekunden ausgeführt wurde. Mit der `min_time` Option kann die Zeit eingestellt werden ab der `cmd_duration` angezeigt wird.

::: warning Nicht die DEBUG-trap in der Bash hooken

Ist `bash` die Konsole der Wahl, dann nicht die `DEBUG`-trap nach der Ausführung von `eval $(starship init $0)` hooken, andernfalls **wird** dieses Modul unweigerlich untergehen.

:::

Bash Nutzer, die eine "preexec" ähnliche Funktion benötigen, können [rcaloras bash_preexec Framework](https://github.com/rcaloras/bash-preexec) verwenden. Definieren Sie einfach die Arrays `preexec_functions` und `precmd_functions` bevor sie `eval $(starship init $0)` ausführen, und fahren Sie dann wie gewohnt fort.

### Optionen

| Variable            | Standardwert    | Beschreibung                                                       |
| ------------------- | --------------- | ------------------------------------------------------------------ |
| `min_time`          | `2_000`         | Schwellwert für kleinste anzuzeigende Laufzeit (in Millisekunden). |
| `show_milliseconds` | `false`         | Zeige Millisekunden zusätzlich zu Sekunden.                        |
| `prefix`            | `took`          | Prefix der unmittelbar vor der Ausführungszeit angezeigt wird.     |
| `style`             | `"bold yellow"` | Stil für dieses Modul.                                             |
| `disabled`          | `false`         | Deaktiviert das `cmd_duration`-Modul.                              |

### Beispiel

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
prefix = "underwent "
```

## Conda

Das `conda`-Modul zeigt dessen aktuelle Umgebung an, sofern `$CONDA_DEFAULT_ENV` gesetzt ist.

::: Tipp

Hinweis: Dies unterdrückt nicht conda's eigenen Prompt-Modifikator, sie können jedoch conda mit `conda config --set changeps1 False` konfigurieren, um die Ausgabe von conda selbst auszuschalten.

:::

### Optionen

| Variable            | Standardwert   | Beschreibung                                                                                                                                                                                                                                      |
| ------------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`            | Die Anzahl der Verzeichnisse, auf die der Verzeichnisspfad abgeschnitten werden soll, wenn die Umgebung über `conda erstellt wurde -p [path]`. `0` bedeutet keine Kürzung. Beachte auch die Beschreibung für das [`directory`](#directory) Modul. |
| `symbol`            | `"C "`         | Symbol das vor dem Umgebungsnamen angezeigt wird.                                                                                                                                                                                                 |
| `style`             | `"bold green"` | Stil für dieses Modul.                                                                                                                                                                                                                            |
| `disabled`          | `false`        | Deaktiviert das `conda`-Modul.                                                                                                                                                                                                                    |

### Beispiel

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- Das aktuelle Verzeichnis enthält eine `shard.yml`-Datei
- The current directory contains a `.cr` file

### Optionen

| Variable   | Standardwert | Beschreibung                                              |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`       | The symbol used before displaying the version of crystal. |
| `style`    | `"bold red"` | Stil für dieses Modul.                                    |
| `disabled` | `false`      | Disables the `crystal` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[crystal]
symbol = "✨ "
style = "bold blue"
```

## Verzeichnis

Das `directory` -Modul zeigt den Pfad zu Ihrem aktuellen Verzeichnis an, abgeschnitten auf drei übergeordnete Ordner. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Optionen

| Variable            | Standardwert  | Beschreibung                                                                     |
| ------------------- | ------------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`           | Die Anzahl der übergeordneten Ordner, die angezeigt werden.                      |
| `truncate_to_repo`  | `true`        | Whether or not to truncate to the root of the git repo that you're currently in. |
| `prefix`            | `"in "`       | Präfix der vor dem Verzeichnis angezeigt wird.                                   |
| `style`             | `"bold cyan"` | Stil für dieses Modul.                                                           |
| `disabled`          | `false`       | Deaktiviert das `directory`-Modul.                                               |

<details>
<summary>Dieses Modul hat einige erweiterte Konfigurationsoptionen, welche die Darstellung von Verzeichnissen steuern.</summary>

| Variable                    | Standardwert | Beschreibung                                                                             |
| --------------------------- | ------------ | ---------------------------------------------------------------------------------------- |
| `substitutions`             |              | A table of substitutions to be made to the path.                                         |
| `fish_style_pwd_dir_length` | `0`          | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`       | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Beispiel

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### Optionen

| Variable          | Standardwert  | Beschreibung                                                                            |
| ----------------- | ------------- | --------------------------------------------------------------------------------------- |
| `symbol`          | `"🐳 "`        | The symbol used before displaying the Docker context .                                  |
| `only_with_files` | `false`       | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `style`           | `"bold blue"` | Stil für dieses Modul.                                                                  |
| `disabled`        | `true`        | Disables the `docker_context` module.                                                   |

### Beispiel

```toml
# ~/.config/starship.toml

[docker_context]
symbol = "🐋 "
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one of the following files are present in the current directory: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj`. You'll also need the .NET Core command-line tools installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

### Optionen

| Variable    | Standardwert  | Beschreibung                                                       |
| ----------- | ------------- | ------------------------------------------------------------------ |
| `symbol`    | `"•NET "`     | Symbol das vor der dotnet-Version angezeigt wird.                  |
| `heuristic` | `true`        | Schnelle Versionserkennung nutzen um Starship bedienbar zu halten. |
| `style`     | `"bold blue"` | Stil für dieses Modul.                                             |
| `disabled`  | `false`       | Deaktiviert das `dotnet`-Modul.                                    |

### Beispiel

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- Das aktuelle Verzeichnis enthält eine `mix.exs`-Datei.

### Optionen

| Variable   | Standardwert    | Beschreibung                                                    |
| ---------- | --------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`          | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"` | Stil für dieses Modul.                                          |
| `disabled` | `false`         | Disables the `elixir` module.                                   |

### Beispiel

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- Das aktuelle Verzeichnis enthält eine `elm.json`-Datei
- Das aktuelle Verzeichnis enthält eine `elm-package.json`-Datei
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### Optionen

| Variable   | Standardwert  | Beschreibung                                          |
| ---------- | ------------- | ----------------------------------------------------- |
| `symbol`   | `"🌳 "`        | The symbol used before displaying the version of Elm. |
| `style`    | `"bold cyan"` | Stil für dieses Modul.                                |
| `disabled` | `false`       | Disables the `elm` module.                            |


### Beispiel

```toml
# ~/.config/starship.toml

[elm]
symbol = " "
```

## Umgebungsvariablen

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### Optionen

| Variable   | Standardwert          | Beschreibung                                                                             |
| ---------- | --------------------- | ---------------------------------------------------------------------------------------- |
| `symbol`   |                       | Das Symbol, das vor der Anzeige der Variable verwendet wird.                             |
| `variable` |                       | Die anzuzeigende Umgebungsvariable.                                                      |
| `default`  |                       | Der Standardwert, der angezeigt wird, wenn die ausgewählte Variable nicht definiert ist. |
| `prefix`   | `""`                  | Präfix der vor der Variable angezeigt wird.                                              |
| `suffix`   | `""`                  | Suffix der nach der Variable angezeigt wird.                                             |
| `style`    | `"dimmed bold black"` | Stil für dieses Modul.                                                                   |
| `disabled` | `false`               | Deaktiviert das `env_var`-Modul.                                                         |

### Beispiel

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- Das aktuelle Verzeichnis enthält eine `rebar.config`-Datei.
- Das aktuelle Verzeichnis enthält eine `erlang.mk`-Datei.

### Optionen

| Variable   | Standardwert | Beschreibung                                             |
| ---------- | ------------ | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`       | The symbol used before displaying the version of Erlang. |
| `style`    | `bold red`   | The style for this module.                               |
| `disabled` | `false`      | Disables the `erlang` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[erlang]
symbol = "e "
```

## Git-Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Optionen

| Variable            | Standardwert    | Beschreibung                                                                                           |
| ------------------- | --------------- | ------------------------------------------------------------------------------------------------------ |
| `symbol`            | `" "`          | Das Symbol, das vor dem Branchnamen des Git-Repositorys in Ihrem aktuellen Verzeichnis angezeigt wird. |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol                  |
| `style`             | `"bold purple"` | Stil für dieses Modul.                                                                                 |
| `disabled`          | `false`         | Deaktiviert das `git_branch`-Modul.                                                                    |

### Beispiel

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash of the repo in your current directory.

### Optionen

| Variable             | Standardwert   | Beschreibung                                          |
| -------------------- | -------------- | ----------------------------------------------------- |
| `commit_hash_length` | `7`            | The length of the displayed git commit hash.          |
| `prefix`             | `"("`          | Prefix to display immediately before git commit.      |
| `suffix`             | `")"`          | Suffix to display immediately after git commit.       |
| `style`              | `"bold green"` | Stil für dieses Modul.                                |
| `only_detached`      | `true`         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`        | Disables the `git_commit` module.                     |

### Beispiel

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git-Zustand

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Optionen

| Variable           | Standardwert       | Beschreibung                                                                                                     |
| ------------------ | ------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `rebase`           | `"REBASING"`       | The text displayed when a `rebase` is in progress.                                                               |
| `merge`            | `"MERGING"`        | The text displayed when a `merge` is in progress.                                                                |
| `revert`           | `"REVERTING"`      | The text displayed when a `revert` is in progress.                                                               |
| `cherry_pick`      | `"CHERRY-PICKING"` | The text displayed when a `cherry-pick` is in progress.                                                          |
| `bisect`           | `"BISECTING"`      | The text displayed when a `bisect` is in progress.                                                               |
| `am`               | `"AM"`             | The text displayed when an `apply-mailbox` (`git am`) is in progress.                                            |
| `am_or_rebase`     | `"AM/REBASE"`      | The text displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.                                 |
| `progress_divider` | `"/"`              | The symbol or text which will separate the current and total progress amounts. (e.g., `" of "`, for `"3 of 10"`) |
| `style`            | `"bold yellow"`    | Stil für dieses Modul.                                                                                           |
| `disabled`         | `false`            | Deaktiviert das `git_state`-Modul.                                                                               |

### Beispiel

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git-Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Optionen

| Variable           | Standardwert               | Beschreibung                                            |
| ------------------ | -------------------------- | ------------------------------------------------------- |
| `conflicted`       | `"="`                      | This branch has merge conflicts.                        |
| `conflicted_count` | [link](#git-status-counts) | Show and style the number of conflicts.                 |
| `ahead`            | `"⇡"`                      | This branch is ahead of the branch being tracked.       |
| `behind`           | `"⇣"`                      | This branch is behind of the branch being tracked.      |
| `diverged`         | `"⇕"`                      | This branch has diverged from the branch being tracked. |
| `untracked`        | `"?"`                      | There are untracked files in the working directory.     |
| `untracked_count`  | [link](#git-status-counts) | Show and style the number of untracked files.           |
| `stashed`          | `"$"`                      | A stash exists for the local repository.                |
| `stashed_count`    | [link](#git-status-counts) | Show and style the number of stashes.                   |
| `modified`         | `"!"`                      | There are file modifications in the working directory.  |
| `modified_count`   | [link](#git-status-counts) | Show and style the number of modified files.            |
| `staged`           | `"+"`                      | A new file has been added to the staging area.          |
| `staged_count`     | [link](#git-status-counts) | Show and style the number of files staged files.        |
| `renamed`          | `"»"`                      | A renamed file has been added to the staging area.      |
| `renamed_count`    | [link](#git-status-counts) | Show and style the number of renamed files.             |
| `deleted`          | `"✘"`                      | A file's deletion has been added to the staging area.   |
| `deleted_count`    | [link](#git-status-counts) | Show and style the number of deleted files.             |
| `show_sync_count`  | `false`                    | Show ahead/behind count of the branch being tracked.    |
| `prefix`           | `[`                        | Prefix to display immediately before git status.        |
| `suffix`           | `]`                        | Suffix to display immediately after git status.         |
| `style`            | `"bold red"`               | Stil für dieses Modul.                                  |
| `disabled`         | `false`                    | Deaktiviert das `git_status`-Modul.                     |

#### Git Status Counts

| Variable  | Standardwert | Beschreibung                                           |
| --------- | ------------ | ------------------------------------------------------ |
| `enabled` | `false`      | Show the number of files                               |
| `style`   |              | Optionally style the count differently than the module |

### Beispiel

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
untracked = "🤷‍"
stashed = "📦"
modified = "📝"
staged.value = "++"
staged.style = "green"
staged_count.enabled = true
staged_count.style = "green"
renamed = "👅"
deleted = "🗑"
```

## Golang

The `golang` module shows the currently installed version of Golang. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- Das aktuelle Verzeichnis enthält eine `go.mod`-Datei
- Das aktuelle Verzeichnis enthält eine `go.sum`-Datei
- Das aktuelle Verzeichnis enthält eine `glide.yaml`-Datei
- Das aktuelle Verzeichnis enthält eine `Gopkg.yml`-Datei
- Das aktuelle Verzeichnis enthält eine `Gopkg.lock`-Datei
- The current directory contains a `.go-version` file
- Das aktuelle Verzeichnis enthält ein `Godeps`-Verzeichnis
- Das aktuelle Verzeichnis enthält eine Datei mit der `.go`-Erweiterung

### Optionen

| Variable   | Standardwert  | Beschreibung                                      |
| ---------- | ------------- | ------------------------------------------------- |
| `symbol`   | `"🐹 "`        | Symbol das vor der Golang-Version angezeigt wird. |
| `style`    | `"bold cyan"` | Stil für dieses Modul.                            |
| `disabled` | `false`       | Deaktiviert das `golang`-Modul.                   |

### Beispiel

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- Das aktuelle Verzeichnis enthält eine `stack.yaml`-Datei

### Optionen

| Variable   | Standardwert | Beschreibung                                              |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"λ "`       | The symbol used before displaying the version of Haskell. |
| `style`    | `"bold red"` | Stil für dieses Modul.                                    |
| `disabled` | `false`      | Disables the `haskell` module.                            |


### Beispiel

```toml
# ~/.config/starship.toml

[haskell]
symbol = " "
```

## Hostname

The `hostname` module shows the system hostname.

### Optionen

| Variable   | Standardwert          | Beschreibung                                                                                                                         |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                | Zeigt den Hostnamen nur, wenn via SSH-Sitzung verbunden.                                                                             |
| `prefix`   | `""`                  | Prefix der unmittelbar vor dem Hostnamen angezeigt wird.                                                                             |
| `suffix`   | `""`                  | Suffix der unmittelbar nach dem Hostnamen angezeigt wird.                                                                            |
| `trim_at`  | `"."`                 | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `style`    | `"bold dimmed green"` | Stil für dieses Modul.                                                                                                               |
| `disabled` | `false`               | Deaktiviert das `hostname`-Modul.                                                                                                    |

### Beispiel

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
prefix = "⟪"
suffix = "⟫"
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### Optionen

| Variable   | Standardwert   | Beschreibung                                    |
| ---------- | -------------- | ----------------------------------------------- |
| `symbol`   | `"☕ "`         | Symbol das vor der Java-Version angezeigt wird. |
| `style`    | `"dimmed red"` | Stil für dieses Modul.                          |
| `disabled` | `false`        | Deaktiviert das `Java`-Modul.                   |

### Beispiel

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Optionen

| Variable    | Standardwert  | Beschreibung                                                                     |
| ----------- | ------------- | -------------------------------------------------------------------------------- |
| `symbol`    | `"✦"`         | Symbol das vor der Anzahl der Jobs angezeigt wird.                               |
| `threshold` | `1`           | Zeigt die Anzahl der Jobs wenn der angegebene Schwellenwert überschritten wurde. |
| `style`     | `"bold blue"` | Stil für dieses Modul.                                                           |
| `disabled`  | `false`       | Deaktiviert das `jobs`-Modul.                                                    |

### Beispiel

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Optionen

| Variable   | Standardwert    | Beschreibung                                            |
| ---------- | --------------- | ------------------------------------------------------- |
| `symbol`   | `"ஃ "`          | The symbol used before displaying the version of Julia. |
| `style`    | `"bold purple"` | Stil für dieses Modul.                                  |
| `disabled` | `false`         | Disables the `julia` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```
## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Optionen

| Variable          | Standardwert  | Beschreibung                                           |
| ----------------- | ------------- | ------------------------------------------------------ |
| `symbol`          | `"☸ "`        | Symbol das vor der Cluster-Information angezeigt wird. |
| `context_aliases` |               | Table of context aliases to display                    |
| `style`           | `"bold blue"` | Stil für dieses Modul.                                 |
| `disabled`        | `true`        | Deaktiviert das `kubernetes`-Modul                     |

### Beispiel

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dimmed green"
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Zeilenumbruch

The `line_break` module separates the prompt into two lines.

### Optionen

| Variable   | Standardwert | Beschreibung                                                           |
| ---------- | ------------ | ---------------------------------------------------------------------- |
| `disabled` | `false`      | Deaktiviert das `line_break`-Modul, wodurch der Prompt einzeilig wird. |

### Beispiel

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Speicherauslastung

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Optionen

| Variable          | Standardwert          | Beschreibung                                                               |
| ----------------- | --------------------- | -------------------------------------------------------------------------- |
| `show_percentage` | `false`               | Zeigt die Speicherauslastung als Prozentsatz des verfügbaren Speichers an. |
| `show_swap`       | `true`                | Swap-Nutzung anzeigen, wenn die Gesamtsumme des swaps nicht Null ist.      |
| `threshold`       | `75`                  | Speicherauslastung ausblenden, wenn sie unter diesem Prozentsatz ist.      |
| `symbol`          | `"🐏 "`                | Symbol das vor der Speicherauslastung angezeigt wird.                      |
| `separator`       | `" | "`               | Symbol oder Text, der ram und swap-Nutzung trennt.                         |
| `style`           | `"bold dimmed white"` | Stil für dieses Modul.                                                     |
| `disabled`        | `true`                | Deaktiviert das `memory_usage`-Modul.                                      |

### Beispiel

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
show_percentage = true
show_swap = true
threshold = -1
symbol = " "
separator = "/"
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Optionen

| Variable            | Standardwert    | Beschreibung                                                                                 |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated.                                     |
| `style`             | `"bold purple"` | Stil für dieses Modul.                                                                       |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### Beispiel

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:
- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Optionen

| Variable   | Standardwert    | Beschreibung                                          |
| ---------- | --------------- | ----------------------------------------------------- |
| `symbol`   | `"👑 "`          | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"` | Stil für dieses Modul.                                |
| `disabled` | `false`         | Disables the `nim` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### Optionen

| Variable     | Standardwert  | Beschreibung                                      |
| ------------ | ------------- | ------------------------------------------------- |
| `use_name`   | `false`       | Display the name of the nix-shell.                |
| `impure_msg` | `"impure"`    | Customize the "impure" msg.                       |
| `pure_msg`   | `"pure"`      | Customize the "pure" msg.                         |
| `symbol`     | `"❄️  "`      | The symbol used before displaying the shell name. |
| `style`      | `"bold blue"` | Stil für dieses Modul.                            |
| `disabled`   | `false`       | Disables the `nix_shell` module.                  |

### Beispiel

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
symbol = "☃️  "
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### Optionen

| Variable   | Standartwert   | Beschreibung                                             |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | Stil für dieses Modul.                                   |
| `disabled` | `false`        | Disables the `nodejs` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠️ Die angezeigte Version ist die des Pakets, dessen Quellcode im Verzeichnis liegt, nicht die des Paketmanagers.

### Optionen

| Variable          | Standardwert | Beschreibung                                               |
| ----------------- | ------------ | ---------------------------------------------------------- |
| `symbol`          | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"` | Stil für dieses Modul.                                     |
| `display_private` | `false`      | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`      | Disables the `package` module.                             |

### Beispiel

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Optionen

| Variable   | Standardwert    | Beschreibung                                            |
| ---------- | --------------- | ------------------------------------------------------- |
| `symbol`   | `"🐫 "`          | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"` | Stil für dieses Modul.                                  |
| `disabled` | `false`         | Disables the `ocaml` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[ocaml]
symbol = "🐪 "
```

## PHP

The `php` module shows the currently installed version of PHP. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### Optionen

| Variable   | Standardwert | Beschreibung                                          |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold 147"` | Stil für dieses Modul.                                |
| `disabled` | `false`      | Disables the `php` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- The current directory contains a `setup.py` file
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### Optionen

| Variable             | Standardwert    | Beschreibung                                                                |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `scan_for_pyfiles`   | `true`          | If false, Python files in the current directory will not show this module.  |
| `style`              | `"bold yellow"` | Stil für dieses Modul.                                                      |
| `disabled`           | `false`         | Disables the `python` module.                                               |

<details>
<summary>This module has some advanced configuration options.</summary>

| Variable        | Standardwert | Beschreibung                                                                  |
| --------------- | ------------ | ----------------------------------------------------------------------------- |
| `python_binary` | `python`     | Configures the python binary that Starship executes when getting the version. |

The `python_binary` variable changes the binary that Starship executes to get the version of Python, it doesn't change the arguments that are used.

```toml
# ~/.config/starship.toml

[python]
python_binary = "python3"
```

</details>

### Beispiel

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

The `ruby` module shows the currently installed version of Ruby. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Optionen

| Variable   | Standardwert | Beschreibung                                           |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`       | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | Stil für dieses Modul.                                 |
| `disabled` | `false`      | Disables the `ruby` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Optionen

| Variable   | Standardwert | Beschreibung                                           |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`       | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | Stil für dieses Modul.                                 |
| `disabled` | `false`      | Disables the `rust` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### Optionen

| Variable   | Standartwert         | Beschreibung                                     |
| ---------- | -------------------- | ------------------------------------------------ |
| `label`    | `""`                 | Prefix before the image name display.            |
| `prefix`   | `"["`                | Prefix to display immediately before image name. |
| `suffix`   | `"]"`                | Suffix to display immediately after image name.  |
| `symbol`   | `""`                 | The symbol used before the image name.           |
| `style`    | `"bold dimmed blue"` | Stil für dieses Modul.                           |
| `disabled` | `false`              | Disables the `singularity` module.               |

### Beispiel

```toml
# ~/.config/starship.toml

[singularity]
symbol = "📦 "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### Optionen

| Variable       | Standartwert | Beschreibung                                                |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`       | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | Stil für dieses Modul.                                      |
| `disabled`     | `false`      | Disables the `terraform` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Optionen

| Variable          | Standartwert    | Beschreibung                                                                                                        |
| ----------------- | --------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | Enables 12 hour formatting.                                                                                         |
| `format`          | see below       | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `"bold yellow"` | The style for the module time.                                                                                      |
| `utc_time_offset` | `"local"`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |
| `disabled`        | `true`          | Disables the `time` module.                                                                                         |
| `time_range`      | `"-"`           | Sets the time range during which the module will be shown. Times must be specified in 24-hours format               |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### Beispiel

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### Optionen

| Variable      | Standartwert    | Beschreibung                          |
| ------------- | --------------- | ------------------------------------- |
| `style_root`  | `"bold red"`    | The style used when the user is root. |
| `style_user`  | `"bold yellow"` | The style used for non-root users.    |
| `show_always` | `false`         | Always shows the `username` module.   |
| `disabled`    | `false`         | Disables the `username` module.       |

### Beispiel

```toml
# ~/.config/starship.toml

[username]
disabled = true
```


## Zig

The `zig` module shows the currently installed version of Zig. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `.zig` file

### Optionen

| Variable   | Standartwert    | Beschreibung                                          |
| ---------- | --------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`          | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"` | Stil für dieses Modul.                                |
| `disabled` | `false`         | Disables the `zig` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:
- The current directory contains a file whose name is in `files`
- The current directory contains a directory whose name is in `directories`
- The current directory contains a file whose extension is in `extensions`
- The `when` command returns 0

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by setting `custom.foo` in `prompt_order`. By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

### Optionen

| Variable      | Standardwert              | Beschreibung                                                                                                               |
| ------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                           | The command whose output should be printed.                                                                                |
| `when`        |                           | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                           | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"` | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                      | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                      | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                      | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                      | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`            | Stil für dieses Modul.                                                                                                     |
| `prefix`      | `""`                      | Prefix to display immediately before the command output.                                                                   |
| `suffix`      | `""`                      | Suffix to display immediately after the command output.                                                                    |
| `disabled`    | `false`                   | Disables this `custom` module.                                                                                             |

#### Custom command shell

`shell` accepts a non-empty list of strings, where:
- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

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
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
prefix = " transcending "

[custom.time]
command = "time /T"
files = ["*.pst"]
prefix = "transcending "
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `spago.dhall` file
- The current directory contains a \*.purs files

### Optionen

| Variable   | Standardwert   | Beschreibung                                                 |
| ---------- | -------------- | ------------------------------------------------------------ |
| `symbol`   | `"<=> "` | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"` | Stil für dieses Modul.                                       |
| `disabled` | `false`        | Disables the `purescript` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[purescript]
symbol = "<=> "
```
