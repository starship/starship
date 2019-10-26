# Konfiguration

::: tip

🔥 Die Konfiguration befindet sich derzeit noch in der Entwicklung. Viele neue Konfigurationsoptionen werden in den kommenden Versionen verfügbar sein.

:::

Um mit der Konfiguration von Starship zu beginnen, musst du die folgende Datei erstellen: `~/.config/starship.toml`.

```shell
$ touch ~/.config/starship.toml
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

### Terminologie

**Module**: Eine Komponente in der Konsole, die auf kontextualisierte Informationen des OS basiert. Zum Beispiel das "nodejs" Modul zeigt die version von NodeJS, die derzeit auf deinem Rechner installiert ist, wenn dein Arbeitsordner ein NodeJS Projekt umfasst.

**Segment**: Kleinere Sub-Komponente die ein Modul ergeben. Zum Beispiel, das "symbol" Segment im "nodejs" Modul beinhaltet das Symbol das vor der Versionsnummer gezeigt wird (Standard: ⬢).

Hier ist die Darstellung des Node Moduls. Im folgenden Beispiel betrachten wir die Segmente "symbol" und "version". Jedes Modul hat auch einen Prefix und einen Suffix, das auf die Standardfarbe des Terminals gesetzt ist.

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### Style-Strings

Die meisten Module in starship lassen dich den Darstellungsstil verändern. Dies passiert meistens an einem bestimmten Eintrag (gewöhnlich `style` genannt), der einen String mit den Einstellungen darstellt. Es folgen ein paar Beispiele für solche Strings zusammen mit Beschreibungen was sie bewirken. Details zur vollen Syntax findest du im [Erweiterten Einstellungshandbuch](/advanced-config/).

- `"fg:green bg:blue"` setzt grünen Text auf blauen Hintergrund
- `"bg:blue fg:bright-green"` setzt hell-grünen Text auf blauen Hintergrund
- `"bold fg:27"` setzt dicke Schrift auf [ANSI Farbe](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` setzt unterstrichenen Text auf einen orangenen Hintergrund
- `"bold italic fg:purple"` setzt dicke Kursivschrift lila
- `""` deativiert explizit jeden Stil

Wie genau sich diese Konfiguration auswirkt liegt an deinem Shell Emulator. Einige Emulatoren zum Beispiel werden die Farben erhellen statt Text dick zu machen, und ein paar Farbthemata benutzen dieselben werte für normale und helle Farben. Für kursiven Text muss dein Terminal Kursivschrift unterstützen.

## Prompt

Dies ist eine Liste mit Prompt-weiten Konfigurationsoptionen.

### Optionen

| Variable       | Standardwert                  | Beschreibung                                                       |
| -------------- | ----------------------------- | ------------------------------------------------------------------ |
| `add_newline`  | `true`                        | Neuer Zeilenumbruch bei Start des Prompts.                         |
| `prompt_order` | [link](#default-prompt-order) | Stelle die Reihenfolge ein, in der die Module den prompt aufbauen. |

### Beispiel

```toml
# ~/.config/starship.toml

# Kein Zeilenumbruch am Anfang der Eingabe
add_newline = false

# Überscheibt die Standard-Reihenfolge und nutzt die folgende
prompt_order=["rust","line_break","package","line_break","character"]
```

### Default Promp Order

Die Standard `prompt_order` wird benutzt um die Reihenfolge der Module im Prompt zu definieren, falls `prompt_order` leer oder nicht gesetzt ist. Der default sieht wie folgt aus:

```toml
prompt_order = [
    "username",
    "hostname",
    "kubernetes",
    "directory",
    "git_branch",
    "git_state",
    "git_status",
    "package",
    "dotnet",
    "golang",
    "java",
    "nodejs",
    "python",
    "ruby",
    "rust",
    "nix_shell",
    "conda",
    "memory_usage",
    "aws",
    "env_var",
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

| Variable   | Standardwert    | Beschreibung                                                        |
| ---------- | --------------- | ------------------------------------------------------------------- |
| `symbol`   | `"☁️ "`         | Symbol das vor der Anzahl des aktuellen AWS-Profils angezeigt wird. |
| `style`    | `"bold yellow"` | Stil für dieses Modul.                                              |
| `disabled` | `false`         | Deaktiviert das `aws`-Modul.                                        |

### Beispiel

```toml
# ~/.config/starship.toml

[aws]
style = "bold blue"
symbol = "🅰 "
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
<summary>Das Batterie-Modul unterstützt auch einige ungewöhnliche Zustände.</summary>

| Variable         | Description                                         |
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

Die `display` Konfiguration "threshold" stellt ein ab wann die Akkuanzeige eingeblendet wird. Mit "style" wird das Erscheinungsbild festgelegt. Wenn `display` nicht angegeben ist. Werden die folgenden Standardwerte verwendet:

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

Das `cmd_duration` Modul zeigt an wie lange der letzte Befehl ausgeführt wurde. Das Modul wird nur angezeigt wenn der letzte Befehl länger als zwei Sekunden ausgeführt wurde. Mit der `min_time` Option kann die Zeit eingestellt werden ab der <0>cmd_duration</0> angezeigt wird.

::: warning Nicht die DEBUG-trap in der Bash hooken

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Optionen

| Variable   | Standardwert    | Beschreibung                                                              |
| ---------- | --------------- | ------------------------------------------------------------------------- |
| `min_time` | `2`             | Kürzestes Wert für den die Ausführungsdauer angezeigt wird (in Sekunden). |
| `prefix`   | `took`          | Prefix der unmittelbar vor der Ausführungszeit angezeigt wird.            |
| `style`    | `"bold yellow"` | Stil für dieses Modul.                                                    |
| `disabled` | `false`         | Deaktiviert das `cmd_duration`-Modul.                                     |

### Beispiel

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 4
prefix = "underwent "
```

## Conda

Das `conda`-Modul zeigt dessen aktuelle Umgebung an, sofern `$CONDA_DEFAULT_ENV` gesetzt ist. Hinweis: Dies unterdrückt conda's eigenen Prompt-Modifikator nicht, sie können jedoch `conda config --set changeps1 False` setzen, um dies zu realisieren.

### Optionen

| Variable   | Standardwert   | Beschreibung                                      |
| ---------- | -------------- | ------------------------------------------------- |
| `symbol`   | `"C "`         | Symbol das vor dem Umgebungsnamen angezeigt wird. |
| `style`    | `"bold green"` | Stil für dieses Modul.                            |
| `disabled` | `false`        | Deaktiviert das `conda`-Modul.                    |

### Beispiel

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## Verzeichnis

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Optionen

| Variable            | Standardwert  | Beschreibung                                                                     |
| ------------------- | ------------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`           | Die Anzahl der übergeordneten Ordner, die angezeigt werden.                      |
| `truncate_to_repo`  | `true`        | Whether or not to truncate to the root of the git repo that you're currently in. |
| `style`             | `"bold cyan"` | Stil für dieses Modul.                                                           |
| `disabled`          | `false`       | Deaktiviert das `directory`-Modul.                                               |

<details>
<summary>Dieses Modul hat einige erweiterte Konfigurationsoptionen, welche die Darstellung von Verzeichnissen steuern.</summary>

| Variable                    | Standardwert | Beschreibung                                                                             |
| --------------------------- | ------------ | ---------------------------------------------------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`          | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`       | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

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

| Variable    | Standardwert  | Beschreibung                                                       |
| ----------- | ------------- | ------------------------------------------------------------------ |
| `symbol`    | `"•NET "`     | Symbol das vor der dotnet-Version angezeigt wird.                  |
| `style`     | `"bold blue"` | Stil für dieses Modul.                                             |
| `heuristic` | `true`        | Schnelle Versionserkennung nutzen um Starship bedienbar zu halten. |
| `disabled`  | `false`       | Deaktiviert das `dotnet`-Modul.                                    |

### Beispiel

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Umgebungsvariablen

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
| `disabled` | `false`          | Deaktiviert das `env_var`-Modul.                                             |

### Beispiel

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Git-Branch

Das `git_branch`-Modul zeigt den aktiven Git-Branch des Repositories im aktuellen Verzeichnis an.

### Optionen

| Variable            | Standardwert    | Beschreibung                                                                          |
| ------------------- | --------------- | ------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the branch name of the repo in your current directory.         |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                 |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol |
| `style`             | `"bold purple"` | Stil für dieses Modul.                                                                |
| `disabled`          | `false`         | Deaktiviert das `git_branch`-Modul.                                                   |

### Beispiel

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
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

### Options

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
| `style`            | `"bold red"`               | The style for the module.                               |
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

Das `golang`-Modul zeigt die aktuell installierte Version von Golang. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Das aktuelle Verzeichnis enthält eine `go.mod`-Datei
- Das aktuelle Verzeichnis enthält eine `go.sum`-Datei
- Das aktuelle Verzeichnis enthält eine `glide.yaml`-Datei
- Das aktuelle Verzeichnis enthält eine `Gopkg.yml`-Datei
- Das aktuelle Verzeichnis enthält eine `Gopkg.lock`-Datei
- Das aktuelle Verzeichnis enthält ein `Godeps`-Verzeichnis
- Das aktuelle Verzeichnis enthält eine Datei mit der `.go`-Erweiterung

### Optionen

| Variable   | Standardwert  | Beschreibung                                             |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | The style for the module.                                |
| `disabled` | `false`       | Disables the `golang` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```

## Hostname

Das `hostname`-Modul zeigt den Hostnamen des Systems an.

### Optionen

| Variable   | Standardwert          | Beschreibung                                                                                                                         |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session.                                                                                 |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.                                                                                   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.                                                                                    |
| `trim_at`  | `"."`                 | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `style`    | `"bold dimmed green"` | The style for the module.                                                                                                            |
| `disabled` | `false`               | Disables the `hostname` module.                                                                                                      |

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

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Optionen

| Variable    | Standardwert  | Beschreibung                                          |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | The style for the module.                             |
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
| `style`    | `"bold blue"` | The style for the module.                           |
| `disabled` | `true`        | Disables the `kubernetes` module                    |

### Beispiel

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dim green"
disabled = false
```

## Zeilenumbruch

Das `line_break`-Modul unterteilt den Prompt in zwei Zeilen.

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

## Nix-Shell

Das `nix_shell`-Modul zeigt die nix-shell Umgebung an. Das Modul wird angezeigt, wenn es sich in einer nix-Shell-Umgebung befindet.

### Optionen

| Variable     | Standardwert | Beschreibung                       |
| ------------ | ------------ | ---------------------------------- |
| `use_name`   | `false`      | Display the name of the nix-shell. |
| `impure_msg` | `impure`     | Customize the "impure" msg.        |
| `pure_msg`   | `pure`       | Customize the "pure" msg.          |
| `style`      | `"bold red"` | The style for the module.          |
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

## Speicherauslastung

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
| `style`           | `"bold dimmed white"` | The style for the module.                                     |
| `disabled`        | `true`                | Disables the `memory_usage` module.                           |

### Beispiel

```toml
# ~/.config/starship.toml

[memory_usage]
show_percentage = true
show_swap = true
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Java

The `java` module shows the currently installed version of Java. The module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle` or `build.sbt` file
- The current directory contains a file with the `.java`, `.class` or `.jar` extension

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

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. The module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### Optionen

| Variable   | Standardwert   | Beschreibung                                             |
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

## Paketversion

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, and `poetry` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Optionen

| Variable   | Standardwert | Beschreibung                                               |
| ---------- | ------------ | ---------------------------------------------------------- |
| `symbol`   | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`    | `"bold red"` | The style for the module.                                  |
| `disabled` | `false`      | Disables the `package` module.                             |

### Beispiel

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## Python

The `python` module shows the currently installed version of Python.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name.

Otherwise, it will display the version number from `python --version` and show the current Python virtual environment if one is activated.

The module will be shown if any of the following conditions are met:

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file

### Optionen

| Variable             | Standardwert    | Beschreibung                                                                |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `style`              | `"bold yellow"` | The style for the module.                                                   |
| `disabled`           | `false`         | Disables the `python` module.                                               |

### Beispiel

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

The `ruby` module shows the currently installed version of Ruby. The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
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

The `rust` module shows the currently installed version of Rust. The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Optionen

| Variable   | Standardwert | Beschreibung                                           |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`       | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | The style for the module.                              |
| `disabled` | `false`      | Disables the `rust` module.                            |

### Beispiel

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Uhrzeit

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

Dieses Modul ist standardmäßig deaktiviert. Setze in deiner Konfiguration `disabled` auf `false` um es zu aktivieren.

:::

### Optionen

| Variable          | Standardwert  | Beschreibung                                                                                                        |
| ----------------- | ------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`       | Enables 12 hour formatting                                                                                          |
| `format`          | see below     | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `bold yellow` | The style for the module time                                                                                       |
| `disabled`        | `true`        | Disables the `time` module.                                                                                         |
| `utc_time_offset` | `local`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |

Wird `use_12hr` auf `true` gestellt, so wird `format` automatisch auf `"%r"` gesetzt. Ansonsten ist der Standardwert hierfür `"%T"`. Wird hingegen `format` gesetzt, so überschreibt dies die Einstellung `use_12hr`.

### Beispiel

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = -5
```

## Benutzername

Das Modul `username` zeigt den Benutzernamen des aktiven Benutzers. Das Modul wird nur dann angezeigt, wenn eine der folgenden Bedingungen zutrifft:

- Der aktuelle Benutzer ist root
- Der aktuelle Benutzer ist nicht derjenige, der derzeit angemeldet ist
- Der Benutzer ist über eine SSH-Sitzung verbunden
- Die Variale `show_always` ist auf `true` gesetzt

### Optionen

| Variable      | Default         | Description                           |
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
