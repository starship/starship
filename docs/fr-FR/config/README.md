# Configuration

::: tip

🔥 La configuration est en train d'être travaillée. Beaucoup de nouvelles options de configuration seront disponibles dans les prochaines versions.

:::

Pour commencer à configurer starship, créez le fichier suivant : `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toute la configuration de starship est effectuée dans ce fichier [TOML](https://github.com/toml-lang/toml) :

```toml
# N'écrivez pas une nouvelle ligne au début de la console
add_newline = false

# Remplacez le symbole "❯" dans la console avec "➜"
[character]      # Le nom du module que nous configurons est "character"
symbol = "➜"     # Le segment "symbol" est mis comme "➜"

# Désactivez le module package, le cachant complètement de la console
[package]
disabled = true
```

Vous pouvez changer l'emplacement du fichier de configuration `starship.toml` grâce à la variable d'environnement `STARSHIP_CONFIG`:
```sh
export STARSHIP_CONFIG=~/.starship
```

De manière équivalente, pour Powershell (Windows), ajoutez la ligne suivante à votre `$PROFILE`:
```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### Terminologie

**Module**: Un composant dans l'invite donnant des informations basées sur des informations contextuelles à propos de votre Système d'Exploitation. Par exemple, le module "nodejs" montre la version de NodeJS qui est actuellement installée sur votre ordinateur, si votre répertoire actuel est un projet NodeJS.

**Segment**: Sous-composants plus petits qui composent un module. Par exemple, le segment "symbol" du module "nodejs" contient le caractère qui est affiché avant le numéro de version (⬢ par défaut).

Voici la représentation du module node. Dans l'exemple suivant, "symbol" et "version" sont des segments dans celui-ci. Chaque module a également un préfixe et un suffixe qui sont la couleur par défaut du terminal.

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### Chaînes de style

La plupart des modules de Starship vous permettent de configurer leurs styles d'affichage. Cela se fait avec une entrée (généralement appelée `style`) qui est une chaîne de caractères spécifiant la configuration. Voici quelques exemples de chaînes de style avec ce qu'elles font. Pour plus de détails sur la syntaxe complète, consultez le [guide de configuration avancé](/advanced-config/).

- `"fg:green bg:blue"` définit un texte vert sur un fond bleu
- `"bg:blue fg:bright-green"` définit un texte vert clair sur un fond bleu
- `"bold fg:27"` définit le texte en gras avec la [couleur ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` définit le texte en souligné sur un fond orange foncé
- `"bold italic fg:violet"` définit le texte en italique et gras sur un fond violet
- `""` désactive explicitement tous les styles

Notez que ce style sera contrôlé par votre émulateur de terminal. Par exemple, certains émulateurs de terminal éclairciront les couleurs au lieu de mettre le texte en gras, et certains thèmes de couleurs utilisent les mêmes valeurs pour les couleurs normales et claires. De plus, pour obtenir du texte italique, votre terminal doit prendre en charge l'italique.

## Invite

Voici la liste des options de configuration de l'invite en lui-même.

### Options

| Variable       | Default                       | Description                                                               |
| -------------- | ----------------------------- | ------------------------------------------------------------------------- |
| `add_newline`  | `true`                        | Ajouter une nouvelle ligne avant le début de l'invite de commande.        |
| `prompt_order` | [lien](#default-prompt-order) | Configurer l'ordre dans lequel les modules s'affichent.                   |
| `scan_timeout` | `30`                          | Délai d'attente pour que starship scanne les fichiers (en millisecondes). |

### Exemple

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
add_newline = false
# Overwrite a default_prompt_order and  use custom prompt_order
prompt_order=["rust","line_break","package","line_break","character"]
# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10
```

### Ordre par défaut des modules

La valeur par défaut `prompt_order` est utilisée pour définir l'ordre selon lequel les modules sont affichés dans l'invite si aucun `prompt_order` n'est fourni ou s'il est vide. La valeur par défaut est la suivante :

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

Le module `aws` affiche la région et le profil AWS courant. Ces informations sont basées sur les variables d'environnement `AWS_REGION`, `AWS_DEFAULT_REGION`, et `AWS_PROFILE` ainsi que le fichier `~/.aws/config`.

Quand [aws-vault](https://github.com/99designs/aws-vault) est utilisé, la valeur du profil est lu dans la variable d'environnement: `AWS_VAULT`.

### Options

| Variable          | Default         | Description                                                                 |
| ----------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`          | `"☁️ "`         | The symbol used before displaying the current AWS profile.                  |
| `displayed_items` | `all`           | Choose which item to display. Possible values: [`all`, `profile`, `region`] |
| `region_aliases`  |                 | Table of region aliases to display in addition to the AWS name.             |
| `style`           | `"bold yellow"` | Le style du module.                                                         |
| `disabled`        | `false`         | Désactive le module `AWS`.                                                  |

### Exemple

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

## Battery

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### Options

| Variable             | Default                  | Description                                         |
| -------------------- | ------------------------ | --------------------------------------------------- |
| `full_symbol`        | `"•"`                    | Le symbole affiché lorsque la batterie est pleine.  |
| `charging_symbol`    | `"⇡"`                    | Le symbole affiché lorsque la batterie se charge.   |
| `discharging_symbol` | `"⇣"`                    | Le symbole affiché lorsque la batterie se décharge. |
| `display`            | [lien](#battery-display) | Affiche le seuil et le style du module.             |
| `disabled`           | `false`                  | Désactive le module `battery`.                      |

<details>
<summary>There are also options for some uncommon battery states.</summary>

| Variable         | Description                                         |
| ---------------- | --------------------------------------------------- |
| `unknown_symbol` | The symbol shown when the battery state is unknown. |
| `empty_symbol`   | The symbol shown when the battery state is empty.   |

Note: Battery indicator will be hidden if the status is `unknown` or `empty` unless you specify the option in the config.

</details>

### Exemple

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Battery Display

The `display` configuration option is used to define when the battery indicator should be shown (threshold) and what it looks like (style). If no `display` is provided. La valeur par défaut est la suivante :

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Options

The `display` option is an array of the following table.

| Variable    | Description                                     |
| ----------- | ----------------------------------------------- |
| `threshold` | The upper bound for the display option.         |
| `style`     | The style used if the display option is in use. |

#### Exemple

```toml
[[battery.display]]  # "bold red" style when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" style when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"

# when capacity is over 30%, the battery indicator will not be displayed

```

## Character

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways: by changing color (red/green) or by changing its shape (❯/✖). The latter will only be done if `use_symbol_for_status` is set to `true`.

### Options

| Variable                | Default        | Description                                                                         |
| ----------------------- | -------------- | ----------------------------------------------------------------------------------- |
| `symbol`                | `"❯"`          | The symbol used before the text input in the prompt.                                |
| `error_symbol`          | `"✖"`          | The symbol used before text input if the previous command failed.                   |
| `use_symbol_for_status` | `false`        | Indicate error status by changing the symbol.                                       |
| `vicmd_symbol`          | `"❮"`          | The symbol used before the text input in the prompt if shell is in vim normal mode. |
| `style_success`         | `"bold green"` | The style used if the last command was successful.                                  |
| `style_failure`         | `"bold green"` | The style used if the last command failed.                                          |
| `disabled`              | `false`        | Disables the `character` module.                                                    |

### Exemple

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## Command Duration

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Options

| Variable            | Default         | Description                                                |
| ------------------- | --------------- | ---------------------------------------------------------- |
| `min_time`          | `2_000`         | Shortest duration to show time for (in milliseconds).      |
| `show_milliseconds` | `false`         | Show milliseconds in addition to seconds for the duration. |
| `prefix`            | `took`          | Prefix to display immediately before the command duration. |
| `style`             | `"bold yellow"` | Le style du module.                                        |
| `disabled`          | `false`         | Disables the `cmd_duration` module.                        |

### Exemple

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
prefix = "underwent "
```

## Conda

The `conda` module shows the current conda environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### Options

| Variable            | Default        | Description                                                                                                                                                                                                 |
| ------------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`            | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `"C "`         | The symbol used before the environment name.                                                                                                                                                                |
| `style`             | `"bold green"` | Le style du module.                                                                                                                                                                                         |
| `disabled`          | `false`        | Disables the `conda` module.                                                                                                                                                                                |

### Exemple

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### Options

| Variable   | Default        | Description                                               |
| ---------- | -------------- | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`         | The symbol used before displaying the version of crystal. |
| `style`    | `"bold green"` | Le style du module.                                       |
| `disabled` | `false`        | Disables the `crystal` module.                            |

### Exemple

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

### Options

| Variable            | Default       | Description                                                                      |
| ------------------- | ------------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`           | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`  | `true`        | Whether or not to truncate to the root of the git repo that you're currently in. |
| `prefix`            | `"in "`       | Prefix to display immediately before the directory.                              |
| `style`             | `"bold cyan"` | Le style du module.                                                              |
| `disabled`          | `false`       | Disables the `directory` module.                                                 |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Variable                    | Default | Description                                                                              |
| --------------------------- | ------- | ---------------------------------------------------------------------------------------- |
| `substitutions`             |         | A table of substitutions to be made to the path.                                         |
| `fish_style_pwd_dir_length` | `0`     | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`  | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Exemple

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### Options

| Variable          | Default       | Description                                                                             |
| ----------------- | ------------- | --------------------------------------------------------------------------------------- |
| `symbol`          | `"🐳 "`        | The symbol used before displaying the Docker context .                                  |
| `only_with_files` | `false`       | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `style`           | `"bold blue"` | Le style du module.                                                                     |
| `disabled`        | `true`        | Disables the `docker_context` module.                                                   |

### Exemple

```toml
# ~/.config/starship.toml

[docker_context]
symbol = "🐋 "
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one of the following files are present in the current directory: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj`. You'll also need the .NET Core command-line tools installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

### Options

| Variable    | Default       | Description                                              |
| ----------- | ------------- | -------------------------------------------------------- |
| `symbol`    | `"•NET "`     | The symbol used before displaying the version of dotnet. |
| `heuristic` | `true`        | Use faster version detection to keep starship snappy.    |
| `style`     | `"bold blue"` | Le style du module.                                      |
| `disabled`  | `false`       | Disables the `dotnet` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `mix.exs` file.

### Options

| Variable   | Default         | Description                                                     |
| ---------- | --------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`          | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"` | Le style du module.                                             |
| `disabled` | `false`         | Disables the `elixir` module.                                   |

### Exemple

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### Options

| Variable   | Default       | Description                                           |
| ---------- | ------------- | ----------------------------------------------------- |
| `symbol`   | `"🌳 "`        | The symbol used before displaying the version of Elm. |
| `style`    | `"bold cyan"` | Le style du module.                                   |
| `disabled` | `false`       | Disables the `elm` module.                            |


### Exemple

```toml
# ~/.config/starship.toml

[elm]
symbol = " "
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### Options

| Variable   | Default               | Description                                                                  |
| ---------- | --------------------- | ---------------------------------------------------------------------------- |
| `symbol`   |                       | The symbol used before displaying the variable value.                        |
| `variable` |                       | The environment variable to be displayed.                                    |
| `default`  |                       | The default value to be displayed when the selected variable is not defined. |
| `prefix`   | `""`                  | Prefix to display immediately before the variable value.                     |
| `suffix`   | `""`                  | Suffix to display immediately after the variable value.                      |
| `style`    | `"dimmed bold black"` | Le style du module.                                                          |
| `disabled` | `false`               | Disables the `env_var` module.                                               |

### Exemple

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

### Options

| Variable   | Default      | Description                                              |
| ---------- | ------------ | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`       | The symbol used before displaying the version of Erlang. |
| `style`    | `bold green` | The style for this module.                               |
| `disabled` | `false`      | Disables the `erlang` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[erlang]
symbol = "e "
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Options

| Variable            | Default         | Description                                                                           |
| ------------------- | --------------- | ------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the branch name of the repo in your current directory.         |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                 |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol |
| `style`             | `"bold purple"` | Le style du module.                                                                   |
| `disabled`          | `false`         | Disables the `git_branch` module.                                                     |

### Exemple

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash of the repo in your current directory.

### Options

| Variable             | Default        | Description                                           |
| -------------------- | -------------- | ----------------------------------------------------- |
| `commit_hash_length` | `7`            | The length of the displayed git commit hash.          |
| `prefix`             | `"("`          | Prefix to display immediately before git commit.      |
| `suffix`             | `")"`          | Suffix to display immediately after git commit.       |
| `style`              | `"bold green"` | Le style du module.                                   |
| `only_detached`      | `true`         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`        | Disables the `git_commit` module.                     |

### Exemple

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Options

| Variable           | Default            | Description                                                                                                      |
| ------------------ | ------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `rebase`           | `"REBASING"`       | The text displayed when a `rebase` is in progress.                                                               |
| `merge`            | `"MERGING"`        | The text displayed when a `merge` is in progress.                                                                |
| `revert`           | `"REVERTING"`      | The text displayed when a `revert` is in progress.                                                               |
| `cherry_pick`      | `"CHERRY-PICKING"` | The text displayed when a `cherry-pick` is in progress.                                                          |
| `bisect`           | `"BISECTING"`      | The text displayed when a `bisect` is in progress.                                                               |
| `am`               | `"AM"`             | The text displayed when an `apply-mailbox` (`git am`) is in progress.                                            |
| `am_or_rebase`     | `"AM/REBASE"`      | The text displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.                                 |
| `progress_divider` | `"/"`              | The symbol or text which will separate the current and total progress amounts. (e.g., `" of "`, for `"3 of 10"`) |
| `style`            | `"bold yellow"`    | Le style du module.                                                                                              |
| `disabled`         | `false`            | Disables the `git_state` module.                                                                                 |

### Exemple

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Options

| Variable           | Default                    | Description                                             |
| ------------------ | -------------------------- | ------------------------------------------------------- |
| `conflicted`       | `"="`                      | This branch has merge conflicts.                        |
| `conflicted_count` | [lien](#git-status-counts) | Show and style the number of conflicts.                 |
| `ahead`            | `"⇡"`                      | This branch is ahead of the branch being tracked.       |
| `behind`           | `"⇣"`                      | This branch is behind of the branch being tracked.      |
| `diverged`         | `"⇕"`                      | This branch has diverged from the branch being tracked. |
| `untracked`        | `"?"`                      | There are untracked files in the working directory.     |
| `untracked_count`  | [lien](#git-status-counts) | Show and style the number of untracked files.           |
| `stashed`          | `"$"`                      | A stash exists for the local repository.                |
| `stashed_count`    | [lien](#git-status-counts) | Show and style the number of stashes.                   |
| `modified`         | `"!"`                      | There are file modifications in the working directory.  |
| `modified_count`   | [lien](#git-status-counts) | Show and style the number of modified files.            |
| `staged`           | `"+"`                      | A new file has been added to the staging area.          |
| `staged_count`     | [lien](#git-status-counts) | Show and style the number of files staged files.        |
| `renamed`          | `"»"`                      | A renamed file has been added to the staging area.      |
| `renamed_count`    | [lien](#git-status-counts) | Show and style the number of renamed files.             |
| `deleted`          | `"✘"`                      | A file's deletion has been added to the staging area.   |
| `deleted_count`    | [lien](#git-status-counts) | Show and style the number of deleted files.             |
| `show_sync_count`  | `false`                    | Show ahead/behind count of the branch being tracked.    |
| `prefix`           | `[`                        | Prefix to display immediately before git status.        |
| `suffix`           | `]`                        | Suffix to display immediately after git status.         |
| `style`            | `"bold green"`             | Le style du module.                                     |
| `disabled`         | `false`                    | Disables the `git_status` module.                       |

#### Git Status Counts

| Variable  | Default | Description                                            |
| --------- | ------- | ------------------------------------------------------ |
| `enabled` | `false` | Show the number of files                               |
| `style`   |         | Optionally style the count differently than the module |

### Exemple

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

The `golang` module shows the currently installed version of Golang. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Options

| Variable   | Default       | Description                                              |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | Le style du module.                                      |
| `disabled` | `false`       | Disables the `golang` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `stack.yaml` file

### Options

| Variable   | Default        | Description                                               |
| ---------- | -------------- | --------------------------------------------------------- |
| `symbol`   | `"λ "`         | The symbol used before displaying the version of Haskell. |
| `style`    | `"bold green"` | Le style du module.                                       |
| `disabled` | `false`        | Disables the `haskell` module.                            |


### Exemple

```toml
# ~/.config/starship.toml

[haskell]
symbol = " "
```

## Hostname

The `hostname` module shows the system hostname.

### Options

| Variable   | Default               | Description                                                                                                                          |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session.                                                                                 |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.                                                                                   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.                                                                                    |
| `trim_at`  | `"."`                 | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `style`    | `"bold dimmed green"` | Le style du module.                                                                                                                  |
| `disabled` | `false`               | Disables the `hostname` module.                                                                                                      |

### Exemple

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

The `java` module shows the currently installed version of Java. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### Options

| Variable   | Default        | Description                                            |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"☕ "`         | The symbol used before displaying the version of Java. |
| `style`    | `"dimmed red"` | Le style du module.                                    |
| `disabled` | `false`        | Disables the `java` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Options

| Variable    | Default       | Description                                           |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | Le style du module.                                   |
| `disabled`  | `false`       | Disables the `jobs` module.                           |

### Exemple

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Options

| Variable   | Default         | Description                                             |
| ---------- | --------------- | ------------------------------------------------------- |
| `symbol`   | `"ஃ "`          | The symbol used before displaying the version of Julia. |
| `style`    | `"bold purple"` | Le style du module.                                     |
| `disabled` | `false`         | Disables the `julia` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```
## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Variable          | Default       | Description                                         |
| ----------------- | ------------- | --------------------------------------------------- |
| `symbol`          | `"☸ "`        | The symbol used before displaying the Cluster info. |
| `context_aliases` |               | Table of context aliases to display                 |
| `style`           | `"bold blue"` | Le style du module.                                 |
| `disabled`        | `true`        | Disables the `kubernetes` module                    |

### Exemple

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dimmed green"
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Options

| Variable   | Default | Description                                                        |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### Exemple

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Variable          | Default               | Description                                                   |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | Display memory usage as a percentage of the available memory. |
| `show_swap`       | `true`                | Display swap usage if total swap is non-zero.                 |
| `threshold`       | `75`                  | Hide the memory usage unless it exceeds this percentage.      |
| `symbol`          | `"🐏 "`                | The symbol used before displaying the memory usage.           |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | Le style du module.                                           |
| `disabled`        | `true`                | Disables the `memory_usage` module.                           |

### Exemple

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

### Options

| Variable            | Default         | Description                                                                                  |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated.                                     |
| `style`             | `"bold purple"` | Le style du module.                                                                          |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### Exemple

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. Le module est affiché si l'une des ces conditions est remplie :
- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Options

| Variable   | Default         | Description                                           |
| ---------- | --------------- | ----------------------------------------------------- |
| `symbol`   | `"👑 "`          | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"` | Le style du module.                                   |
| `disabled` | `false`         | Disables the `nim` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### Options

| Variable     | Default       | Description                                       |
| ------------ | ------------- | ------------------------------------------------- |
| `use_name`   | `false`       | Display the name of the nix-shell.                |
| `impure_msg` | `"impure"`    | Customize the "impure" msg.                       |
| `pure_msg`   | `"pure"`      | Customize the "pure" msg.                         |
| `symbol`     | `"❄️  "`      | The symbol used before displaying the shell name. |
| `style`      | `"bold blue"` | Le style du module.                               |
| `disabled`   | `false`       | Disables the `nix_shell` module.                  |

### Exemple

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

The `nodejs` module shows the currently installed version of NodeJS. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### Options

| Variable   | Default        | Description                                              |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | Le style du module.                                      |
| `disabled` | `false`        | Disables the `nodejs` module.                            |

### Exemple

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

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Options

| Variable          | Default      | Description                                                |
| ----------------- | ------------ | ---------------------------------------------------------- |
| `symbol`          | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"` | Le style du module.                                        |
| `display_private` | `false`      | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`      | Disables the `package` module.                             |

### Exemple

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Options

| Variable   | Default         | Description                                             |
| ---------- | --------------- | ------------------------------------------------------- |
| `symbol`   | `"🐫 "`          | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"` | Le style du module.                                     |
| `disabled` | `false`         | Disables the `ocaml` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[ocaml]
symbol = "🐪 "
```

## PHP

The `php` module shows the currently installed version of PHP. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### Options

| Variable   | Default      | Description                                           |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold 147"` | Le style du module.                                   |
| `disabled` | `false`      | Disables the `php` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- The current directory contains a `setup.py` file
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### Options

| Variable             | Défault         | Description                                                                 |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `scan_for_pyfiles`   | `true`          | If false, Python files in the current directory will not show this module.  |
| `style`              | `"bold yellow"` | Le style du module.                                                         |
| `disabled`           | `false`         | Disables the `python` module.                                               |

<details>
<summary>This module has some advanced configuration options.</summary>

| Variable        | Défaut   | Description                                                                   |
| --------------- | -------- | ----------------------------------------------------------------------------- |
| `python_binary` | `python` | Configures the python binary that Starship executes when getting the version. |

The `python_binary` variable changes the binary that Starship executes to get the version of Python, it doesn't change the arguments that are used.

```toml
# ~/.config/starship.toml

[python]
python_binary = "python3"
```

</details>

### Exemple

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

The `ruby` module shows the currently installed version of Ruby. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Options

| Variable   | Default        | Description                                            |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"💎 "`         | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold green"` | Le style du module.                                    |
| `disabled` | `false`        | Disables the `ruby` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Options

| Variable   | Default        | Description                                            |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`         | The symbol used before displaying the version of Rust. |
| `style`    | `"bold green"` | Le style du module.                                    |
| `disabled` | `false`        | Disables the `rust` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### Options

| Variable   | Default              | Description                                      |
| ---------- | -------------------- | ------------------------------------------------ |
| `label`    | `""`                 | Prefix before the image name display.            |
| `prefix`   | `"["`                | Prefix to display immediately before image name. |
| `suffix`   | `"]"`                | Suffix to display immediately after image name.  |
| `symbol`   | `""`                 | The symbol used before the image name.           |
| `style`    | `"bold dimmed blue"` | Le style du module.                              |
| `disabled` | `false`              | Disables the `singularity` module.               |

### Exemple

```toml
# ~/.config/starship.toml

[singularity]
symbol = "📦 "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### Options

| Variable       | Default      | Description                                                 |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`       | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | Le style du module.                                         |
| `disabled`     | `false`      | Disables the `terraform` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## Temps

Le module `time` affiche l'heure actuelle **localement**. La valeur de `format` est utilisée par le package [`chrono`](https://crates.io/crates/chrono) pour contrôler la façon dont l'heure est affichée. Consultez la [doc de chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) pour découvrir les options disponibles.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Variable          | Default         | Description                                                                                                                                         |
| ----------------- | --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | Activer le format 12h.                                                                                                                              |
| `format`          | voir plus bas   | Le [format chrono](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) utilisé pour formater l'heure.                                   |
| `style`           | `"bold yellow"` | Le style utilisé par le module.                                                                                                                     |
| `utc_time_offset` | `"local"`       | Définir le décalage horaire UTC à utiliser. Intervalle de -24 < x < 24. Accepte des nombres décimaux pour s'adapter aux décalages de 30/45 minutes. |
| `disabled`        | `true`          | Désactiver le module `time`.                                                                                                                        |
| `time_range`      | `"-"`           | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                                               |

Si `use_12hr` a pour valeur `true`, le `format` par défaut est `"%r"`. Sinon, il est défini comme `"%T"`. Définir manuellement le `format` passera outre la valeur de `user_12hr`.

### Exemple

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Nom d'utilisateur

Le module `username` affiche le nom d'utilisateur de l'utilisateur actif. Le module est affiché si l'une des ces conditions est remplie :

- L'utilisateur courant est root
- L'utilisateur courant est différent de celui connecté
- L'utilisateur est actuellement connecté à une session SSH
- La variable `show_always` a comme valeur true

### Options

| Variable      | Default         | Description                                      |
| ------------- | --------------- | ------------------------------------------------ |
| `style_root`  | `"bold green"`  | Le style utilisé quand l'utilisateur est root.   |
| `style_user`  | `"bold yellow"` | Le style utilisé pour les utilisateurs non-root. |
| `show_always` | `false`         | Toujours afficher le module `username`.          |
| `disabled`    | `false`         | Désactiver le module `username`.                 |

### Exemple

```toml
# ~/.config/starship.toml

[username]
disabled = true
```


## Zig

The `zig` module shows the currently installed version of Zig. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `.zig` file

### Options

| Variable   | Default         | Description                                           |
| ---------- | --------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`          | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"` | Le style du module.                                   |
| `disabled` | `false`         | Disables the `zig` module.                            |

### Exemple

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

### Options

| Variable      | Default                   | Description                                                                                                                |
| ------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                           | The command whose output should be printed.                                                                                |
| `when`        |                           | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                           | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"` | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                      | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                      | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                      | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                      | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`            | Le style du module.                                                                                                        |
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

### Exemple

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

Le module `purescript` affiche la version courante de Purescript installée. Le module est affiché si l'une des ces conditions est remplie :

- The current directory contains a `spago.dhall` file
- The current directory contains a \*.purs files

### Options

| Variable   | Default        | Description                                                   |
| ---------- | -------------- | ------------------------------------------------------------- |
| `symbol`   | `"<=> "` | Le symbole utilisé avant d'afficher la version de PureScript. |
| `style`    | `"bold white"` | Le style du module.                                           |
| `disabled` | `false`        | Désactive le module `purescript`.                             |

### Exemple

```toml
# ~/.config/starship.toml

[purescript]
symbol = "<=> "
```
