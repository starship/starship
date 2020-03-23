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
    "package",
    "dotnet",
    "elixir",
    "elm",
    "golang",
    "haskell",
    "java",
    "nodejs",
    "php",
    "python",
    "ruby",
    "rust",
    "terraform",
    "nix_shell",
    "conda",
    "memory_usage",
    "aws",
    "env_var",
    "crystal",
    "cmd_duration",
    "line_break",
    "jobs",
    "battery",
    "time",
    "character",
]
```

## AWS

Das `aws`-Modul zeigt das aktuelle AWS-Profil an. Dies basiert auf den Umgebungsvariablen: `AWS_REGION`, `AWS_DEFAULT_REGION`, `AWS_PROFILE` und der `~/.aws/config` Datei.

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

::: tip

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

The `crystal` module shows the currently installed version of Crystal. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current directory contains a `shard.yml` file
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

## Directory

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Optionen

| Variable            | Standardwert  | Beschreibung                                                                     |
| ------------------- | ------------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`           | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`  | `true`        | Whether or not to truncate to the root of the git repo that you're currently in. |
| `prefix`            | `"in "`       | Prefix to display immediately before the directory.                              |
| `style`             | `"bold cyan"` | Stil für dieses Modul.                                                           |
| `disabled`          | `false`       | Disables the `directory` module.                                                 |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Variable                    | Standardwert | Beschreibung                                                                             |
| --------------------------- | ------------ | ---------------------------------------------------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`          | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`       | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Beispiel

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one of the following files are present in the current directory: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj`. You'll also need the .NET Core command-line tools installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

### Optionen

| Variable    | Standardwert  | Beschreibung                                             |
| ----------- | ------------- | -------------------------------------------------------- |
| `symbol`    | `"•NET "`     | The symbol used before displaying the version of dotnet. |
| `heuristic` | `true`        | Use faster version detection to keep starship snappy.    |
| `style`     | `"bold blue"` | Stil für dieses Modul.                                   |
| `disabled`  | `false`       | Disables the `dotnet` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current directory contains a `mix.exs` file.

### Optionen

| Variable   | Standardwert | Beschreibung                                                    |
| ---------- | ------------ | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`       | The symbol used before displaying the version of Elixir/Erlang. |
| `disabled` | `false`      | Disables the `elixir` module.                                   |

### Beispiel

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
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

## Environment Variable

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### Optionen

| Variable   | Standardwert     | Beschreibung                                                                 |
| ---------- | ---------------- | ---------------------------------------------------------------------------- |
| `symbol`   |                  | The symbol used before displaying the variable value.                        |
| `variable` |                  | The environment variable to be displayed.                                    |
| `default`  |                  | The default value to be displayed when the selected variable is not defined. |
| `prefix`   | `""`             | Prefix to display immediately before the variable value.                     |
| `suffix`   | `""`             | Suffix to display immediately after the variable value.                      |
| `style`    | `"dimmed black"` | Stil für dieses Modul.                                                       |
| `disabled` | `false`          | Disables the `env_var` module.                                               |

### Beispiel

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Optionen

| Variable            | Standardwert    | Beschreibung                                                                          |
| ------------------- | --------------- | ------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the branch name of the repo in your current directory.         |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                 |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol |
| `style`             | `"bold purple"` | Stil für dieses Modul.                                                                |
| `disabled`          | `false`         | Disables the `git_branch` module.                                                     |

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

## Git State

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
| `disabled`         | `false`            | Disables the `git_state` module.                                                                                 |

### Beispiel

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git Status

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
| `disabled`         | `false`                    | Disables the `git_status` module.                       |

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

The `golang` module shows the currently installed version of Golang. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Optionen

| Variable   | Standardwert  | Beschreibung                                             |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | Stil für dieses Modul.                                   |
| `disabled` | `false`       | Disables the `golang` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current directory contains a `stack.yaml` file

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

Das `hostname`-Modul zeigt den Hostnamen des Systems an.

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
prefix = "10218;"
suffix = " 10219;"
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- The current directory contains a `pom.xml`, `build.gradle.kts` or `build.sbt` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### Optionen

| Variable   | Standardwert   | Beschreibung                                           |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"☕ "`         | The symbol used before displaying the version of Java. |
| `style`    | `"dimmed red"` | Stil für dieses Modul.                                 |
| `disabled` | `false`        | Disables the `java` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Optionen

| Variable    | Standardwert  | Beschreibung                                          |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | Stil für dieses Modul.                                |
| `disabled`  | `false`       | Disables the `jobs` module.                           |

### Beispiel

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Variable   | Standardwert  | Beschreibung                                        |
| ---------- | ------------- | --------------------------------------------------- |
| `symbol`   | `"☸ "`        | The symbol used before displaying the Cluster info. |
| `style`    | `"bold blue"` | Stil für dieses Modul.                              |
| `disabled` | `true`        | Disables the `kubernetes` module                    |

### Beispiel

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dimmed green"
disabled = false
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Optionen

| Variable   | Standardwert | Beschreibung                                                       |
| ---------- | ------------ | ------------------------------------------------------------------ |
| `disabled` | `false`      | Disables the `line_break` module, making the prompt a single line. |

### Beispiel

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Variable          | Standardwert          | Beschreibung                                                  |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | Display memory usage as a percentage of the available memory. |
| `show_swap`       | `true`                | Display swap usage if total swap is non-zero.                 |
| `threshold`       | `75`                  | Hide the memory usage unless it exceeds this percentage.      |
| `symbol`          | `"🐏 "`                | The symbol used before displaying the memory usage.           |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | Stil für dieses Modul.                                        |
| `disabled`        | `true`                | Disables the `memory_usage` module.                           |

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

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### Optionen

| Variable     | Standardwert | Beschreibung                       |
| ------------ | ------------ | ---------------------------------- |
| `use_name`   | `false`      | Display the name of the nix-shell. |
| `impure_msg` | `"impure"`   | Customize the "impure" msg.        |
| `pure_msg`   | `"pure"`     | Customize the "pure" msg.          |
| `style`      | `"bold red"` | Stil für dieses Modul.             |
| `disabled`   | `false`      | Disables the `nix_shell` module.   |

### Beispiel

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
```

## NodeJS

Das `nodejs` Modul zeigt die derzeit installierte Version von NodeJS. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Das aktuelle Verzeichnis enthält eine `package.json`-Datei
- Das aktuelle Verzeichnis enthält ein `node_modules`-Verzeichnis
- Das aktuelle Verzeichnis enthält eine Datei mit der `.js`-Erweiterung

### Optionen

| Variable   | Standardwert   | Beschreibung                                      |
| ---------- | -------------- | ------------------------------------------------- |
| `symbol`   | `"⬢ "`         | Symbol das vor der NodeJS-Version angezeigt wird. |
| `style`    | `"bold green"` | Stil für dieses Modul.                            |
| `disabled` | `false`        | Deaktiviert das `nodejs`-Modul.                   |

### Beispiel

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## Paketversion

Das `Package` Modul wird angezeigt, wenn das aktuelle Verzeichnis das Repository für ein Paket ist, und zeigt dessen aktuelle Version an. The module currently supports `npm`, `cargo`, `poetry`, `composer`, and `gradle` packages.

- **npm** – Die `npm` Paketversion wird aus dem `package.json` gelesen, das sich im aktuellen Verzeichnis befindet
- **Cargo** – Die `Cargo` Paketversion wird aus dem `Cargo.toml` gelesen, das sich im aktuellen Verzeichnis befindet
- **poetry** – Die `poetry` Paketversion wird aus der `pyproject.toml` gelesen, das sich im aktuellen Verzeichnis befindet
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present

> ⚠️ Die angezeigte Version ist die des Pakets, dessen Quellcode im Verzeichnis liegt, nicht die des Paketmanagers.

### Optionen

| Variable   | Standardwert | Beschreibung                                    |
| ---------- | ------------ | ----------------------------------------------- |
| `symbol`   | `"📦 "`       | Symbol das vor der Paketversion angezeigt wird. |
| `style`    | `"bold red"` | Stil für dieses Modul.                          |
| `disabled` | `false`      | Deaktiviert das `package`-Modul.                |

### Beispiel

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

Das `php`-Modul zeigt die aktuell installierte Version von PHP. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Das aktuelle Verzeichnis enthält eine `composer.json`-Datei
- Das aktuelle Verzeichnis enthält eine `.php`-Datei

### Optionen

| Variable   | Standardwert | Beschreibung                                   |
| ---------- | ------------ | ---------------------------------------------- |
| `symbol`   | `"🐘 "`       | Symbol das vor der PHP-Version angezeigt wird. |
| `style`    | `"bold red"` | Stil für dieses Modul.                         |
| `disabled` | `false`      | Deaktiviert das `php`-Modul.                   |

### Beispiel

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

Das `python`-Modul zeigt die aktuell installierte Version von Python.

Wenn `pyenv_version_name` auf `true` gesetzt ist, wird die version der pyenv angezeigt.

Andernfalls wird die gleiche Versionsnummer angezeigt wie `python --version`, sowie der Name des aktuellen virtualenvs, wenn eines aktiv ist.

Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Das aktuelle Verzeichnis enthält eine `.python-version`-Datei
- Das aktuelle Verzeichnis enthält eine `requirements.txt`-Datei
- Das aktuelle Verzeichnis enthält eine `pyproject.toml`-Datei
- Das aktuelle Verzeichnis enthält eine Datei mit der `.py`-Erweiterung
- Das aktuelle Verzeichnis enthält eine `Pipfile`-Datei
- Das aktuelle Verzeichnis enthält eine `tox.ini`-Datei
- Ein virtualenv ist momentan aktiv

### Optionen

| Variable             | Standardwert    | Beschreibung                                                        |
| -------------------- | --------------- | ------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | Symbol das vor der Python-Version angezeigt wird.                   |
| `pyenv_version_name` | `false`         | Verwende `pyenv` um die Python-Versionzu beziehen.                  |
| `pyenv_prefix`       | `"pyenv "`      | Prefix zur Anzeige der pyenv-Version (Standard: `pyenv MY_VERSION`) |
| `style`              | `"bold yellow"` | Stil für dieses Modul.                                              |
| `disabled`           | `false`         | Deaktiviert das `python`-Modul.                                     |

### Beispiel

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

Das `ruby` Modul zeigt die derzeit installierte Version von Ruby an. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Das aktuelle Verzeichnis enthält eine `Gemfile`-Datei
- Das aktuelle Verzeichnis enthält eine `.rb`-Datei

### Optionen

| Variable   | Standartwert | Beschreibung                                    |
| ---------- | ------------ | ----------------------------------------------- |
| `symbol`   | `"💎 "`       | Symbol das vor der Ruby-Version angezeigt wird. |
| `style`    | `"bold red"` | Stil für dieses Modul.                          |
| `disabled` | `false`      | Deaktiviert das `ruby`-Modul.                   |

### Beispiel

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

Das `rust` Modul zeigt die derzeit installierte Version von Rust an. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Das aktuelle Verzeichnis enthält eine `Cargo.toml`-Datei
- Das aktuelle Verzeichnis enthält eine Datei mit der `.rs`-Erweiterung

### Optionen

| Variable   | Standardwert | Beschreibung                                    |
| ---------- | ------------ | ----------------------------------------------- |
| `symbol`   | `"🦀 "`       | Symbol das vor der Rust-Version angezeigt wird. |
| `style`    | `"bold red"` | Stil für dieses Modul.                          |
| `disabled` | `false`      | Deaktiviert das `rust`-Modul.                   |

### Beispiel

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

:::

### Optionen

| Variable   | Standardwert         | Beschreibung                                     |
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

Das `Terraform` Modul zeigt den aktuell ausgewählten terraform Arbeitsbereich und die Version an. Standardmäßig wird die Terraform-Version nicht angezeigt, da dies bei aktuellen Versionen von Terraform langsam ist, wenn viele Plugins verwendet werden. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Das aktuelle Verzeichnis enthält eine `.terraform`-Datei
- Das aktuelle Verzeichnis enthält eine Datei mit der `.tf`-Erweiterung

### Optionen

| Variable       | Standardwert | Beschreibung                                                                           |
| -------------- | ------------ | -------------------------------------------------------------------------------------- |
| `symbol`       | `"💠 "`       | Das Symbol das vor dem Terraform-Workspacenamen angezeigt wird.                        |
| `show_version` | `false`      | Blendet die Terraform Versionsnummer ein. Kann In großen Workspaces sehr langsam sein. |
| `style`        | `"bold 105"` | Stil für dieses Modul.                                                                 |
| `disabled`     | `false`      | Deaktiviert das `terraform` Modul.                                                     |

### Beispiel

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## Uhrzeit

Das `time` Modul zeigt die aktuelle **lokale** Zeit an. Der `format` Wert wird von der crate [`chrono`](https://crates.io/crates/chrono) benutzt um die Zeit zu formatieren. Schau dir [die chrono strftime Dokumentation](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) an, um die möglichen Optionen zu sehen.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Variable          | Standardwert    | Beschreibung                                                                                                                              |
| ----------------- | --------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | Aktiviert die Formatierung der Uhrzeit im 12-Stunden-Format.                                                                              |
| `format`          | Siehe unten     | Das Format zum Anzeigen der Uhrzeit in [chrono-Formatierung](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html).             |
| `style`           | `"bold yellow"` | Stil für dieses Modul.                                                                                                                    |
| `utc_time_offset` | `"local"`       | Legt das UTC-Offset fest, das verwendet werden soll. Reicht von -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`          | Deaktiviert das `time`-Modul.                                                                                                             |

Wird `use_12hr` auf `true` gestellt, so wird `format` automatisch auf `"%r"` gesetzt. Ansonsten ist der Standardwert hierfür `"%T"`. Wird hingegen `format` gesetzt, so überschreibt dies die Einstellung `use_12hr`.

### Beispiel

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
```

## Benutzername

Das Modul `username` zeigt den Benutzernamen des aktiven Benutzers. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Der aktuelle Benutzer ist root
- Der aktuelle Benutzer ist nicht derjenige, der derzeit angemeldet ist
- Der Benutzer ist über eine SSH-Sitzung verbunden
- Die Variale `show_always` ist auf `true` gesetzt

### Optionen

| Variable      | Standardwert    | Beschreibung                      |
| ------------- | --------------- | --------------------------------- |
| `style_root`  | `"bold red"`    | Stil beim root-Benutzer.          |
| `style_user`  | `"bold yellow"` | Stil bei allen anderen Benutzern. |
| `show_always` | `false`         | `username`-Modul immer anzeigen.  |
| `disabled`    | `false`         | Deaktiviert das `username`-Modul. |

### Beispiel

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
