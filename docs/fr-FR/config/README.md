# Configuration

To get started configuring starship, create the following file: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

All configuration for starship is done in this [TOML](https://github.com/toml-lang/toml) file:

```toml
# Get editor completions based on the config schema
"$schema" = 'https://starship.rs/config-schema.json'

# Inserts a blank line between shell prompts
add_newline = true

# Replace the '❯' symbol in the prompt with '➜'
[character] # The name of the module we are configuring is 'character'
success_symbol = '[➜](bold green)' # The 'success_symbol' segment is being set to '➜' with the color 'bold green'

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

### Config File Location

You can change default configuration file location with `STARSHIP_CONFIG` environment variable:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Journalisation

By default starship logs warnings and errors into a file named `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, where the session key is corresponding to an instance of your terminal.
This, however can be changed using the `STARSHIP_CACHE` environment variable:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminologie

**Module**: A component in the prompt giving information based on contextual information from your OS. Par exemple, le module "nodejs" montre la version de Node.js qui installée sur votre ordinateur, si votre dossier actuel est un projet Node.js.

**Variable**: Smaller sub-components that contain information provided by the module. Par exemple, la variable "version" dans le module "nodejs" contient la version actuelle de Node.js.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Strings

In TOML syntax, [text values](https://toml.io/en/v1.0.0#string) are declared with `'`, `"`, `'''`, or `"""`.

The following Starship syntax symbols have special usage in a format string and must be escaped to display as that character: `$ [ ] ( )`.

| Symbole | Type                      | Notes                                                  |
| ------- | ------------------------- | ------------------------------------------------------ |
| `'`     | literal string            | less escaping                                          |
| `"`     | string                    | more escaping                                          |
| `'''`   | multi-line literal string | less escaping                                          |
| `"""`   | multi-line string         | more escaping, newlines in declarations can be ignored |

Par exemple :

```toml
# literal string
format = '☺\☻ '

# regular string
format = "☺\\☻ "

# escaping Starship symbols
format = '\[\$\] '
```

When using line breaks, multi-line declarations can be used.
For example, if you want to print a `$` symbol on a new line, the following values for `format` are equivalent:

```toml
# with literal string
format = '''

\$'''

# with multiline basic string
format = """

\\$"""

# with basic string
format = "\n\\$"
```

In multiline basic strings, newlines can be used for formatting without being present in the value by escaping them.

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

### Chaîne de formatage

Les chaînes de formatage sont le format avec lequel un module affiche toutes ses variables.
Most modules have an entry called `format` that configures the display format of the module.
Vous pouvez utiliser des textes, des variables et des groupes de texte dans une chaîne de format.

#### Variable

A variable contains a `$` symbol followed by the name of the variable.
Le nom d’une variable peut seulement container des lettres, des nombres et `_`.

Par exemple :

- `'$version'` is a format string with a variable named `version`.
- `'$git_branch$git_commit'` is a format string with two variables named `git_branch` and `git_commit`.
- `'$git_branch $git_commit'` has the two variables separated with a space.

#### Groupe de texte

Un groupe de texte se compose de deux parties différentes.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings).
Vous pouvez y ajouter des textes, des variables, ou même des groupes de texte imbriqués.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). Elle peut être utilisée pour styliser la première partie.

Par exemple :

- `'[on](red bold)'` will print a string `on` with bold text colored red.
- `'[⌘ $version](bold green)'` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `'[a [b](red) c](green)'` will print `a b c` with `b` red, and `a` and `c` green.

#### Chaînes de style

La plupart des modules de Starship vous permettent de configurer leurs styles d'affichage. This is done with an entry (usually called `style`) which is a string specifying the configuration. Voici quelques exemples de chaînes de style avec ce qu'elles font. For details on the full syntax, consult the [advanced config guide](../advanced-config/).

- `'fg:green bg:blue'` sets green text on a blue background
- `'bg:blue fg:bright-green'` sets bright green text on a blue background
- `'bold fg:27'` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `'underline bg:#bf5700'` sets underlined text on a burnt orange background
- `'bold italic fg:purple'` sets bold italic purple text
- `''` explicitly disables all styling

Notez que ce style sera contrôlé par votre émulateur de terminal. Par exemple, certains émulateurs de terminal éclairciront les couleurs au lieu de mettre le texte en gras, et certains thèmes de couleurs utilisent les mêmes valeurs pour les couleurs normales et claires. De plus, pour obtenir du texte italique, votre terminal doit prendre en charge l'italique.

#### Chaînes de formatage conditionnel

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

Par exemple :

- `'(@$region)'` will show nothing if the variable `region` is `None` or empty string, otherwise `@` followed by the value of region.
- `'(some text)'` will always show nothing since there are no variables wrapped in the braces.
- When `$combined` is a shortcut for `\[$a$b\]`, `'($combined)'` will show nothing only if `$a` and `$b` are both `None`.
  This works the same as `'(\[$a$b\] )'`.

### Negative matching

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. These take
lists of strings to match or not match. "Negative" options, those which should not be matched, are
indicated with a leading '!' character. The presence of _any_ negative indicator in the directory
will result in the module not being matched.

Extensions are matched against both the characters after the last dot in a filename, and the
characters after the first dot in a filename. For example, `foo.bar.tar.gz` will be matched
against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Files whose name begins with a
dot are not considered to have extensions at all.

To see how this works in practice, you could match TypeScript but not MPEG Transport Stream files thus:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Invite

Voici la liste des options de configuration globales de l'invite de commandes.

### Options

| Option            | Défaut                         | Description                                                                                                                                                                                                        |
| ----------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `format`          | [link](#default-prompt-format) | Configure le format de l'invite.                                                                                                                                                                   |
| `right_format`    | `''`                           | See [Enable Right Prompt](../advanced-config/#enable-right-prompt)                                                                                                                                                 |
| `scan_timeout`    | `30`                           | Délai maximal pour le scan des fichiers par starship (en millisecondes).                                                                                                        |
| `command_timeout` | `500`                          | Délai maximal pour les commandes exécutées par starship (en millisecondes).                                                                                                     |
| `add_newline`     | `true`                         | Insère une ligne vide entre les invites du shell.                                                                                                                                                  |
| `palette`         | `''`                           | Sets which color palette from `palettes` to use.                                                                                                                                                   |
| `palettes`        | `{}`                           | Collection of color palettes that assign [colors](../advanced-config/#style-strings) to user-defined names. Note that color palettes cannot reference their own color definitions. |
| `follow_symlinks` | `true`                         | Follows symlinks to check if they're directories; used in modules such as git.                                                                                                                     |

> [!TIP]
> If you have symlinks to networked filesystems, consider setting
> `follow_symlinks` to `false`.

### Exemple

```toml
# ~/.config/starship.toml

# Use custom format
format = '''
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) '''

# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10

# Disable the blank line at the start of the prompt
add_newline = false

# Set 'foo' as custom color palette
palette = 'foo'

# Define custom colors
[palettes.foo]
# Overwrite existing color
blue = '21'
# Define new color
mustard = '#af8700'
```

### Format par Défaut

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. La valeur par défaut est la suivante :

```toml
format = '$all'

# Which is equivalent to
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
modules you explicitly add to the format will not be duplicated. Par ex.

```toml
# Move the directory to the second line
format = '$all$directory$character'
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials.
The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process`, `sso_start_url`, or `sso_session` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice.
If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile
is read from the `AWS_VAULT` env var and the credentials expiration date
is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile
is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile
is read from the `AWSUME_PROFILE` env var and the credentials expiration
date is read from the `AWSUME_EXPIRATION` env var.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials`
falls back to the `x_security_token_expires` key.

When using [aws-sso-cli](https://github.com/synfinatic/aws-sso-cli) the profile
is read from the `AWS_SSO_PROFILE` env var.

### Options

| Option              | Défaut                                                                | Description                                                                                                                 |
| ------------------- | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | Format du module.                                                                                           |
| `symbole`           | `'☁️ '`                                                               | Le symbole est affiché avant le profil AWS actuel.                                                          |
| `region_aliases`    | `{}`                                                                  | Tableau des alias de région à afficher en plus du nom AWS.                                                  |
| `profile_aliases`   | `{}`                                                                  | Tableau des alias de profil à afficher en plus du nom AWS.                                                  |
| `style`             | `'bold yellow'`                                                       | Le style pour le module.                                                                                    |
| `expiration_symbol` | `'X'`                                                                 | Le symbole est affiché lorsque les identifiants temporaires ont expiré.                                     |
| `disabled`          | `false`                                                               | Disables the `AWS` module.                                                                                  |
| `force_display`     | `false`                                                               | If `true` displays info even if `credentials`, `credential_process` or `sso_start_url` have not been setup. |

### Variables

| Variable | Exemple          | Description                                    |
| -------- | ---------------- | ---------------------------------------------- |
| region   | `ap-northeast-1` | La région AWS actuelle                         |
| profile  | `astronauts`     | Le profil AWS actuel                           |
| duration | `2h27m20s`       | Durée de validité des identifiants temporaires |
| symbole  |                  | Reflète la valeur de l'option `symbol`         |
| style\*  |                  | Reflète la valeur de l'option `style`          |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemples

#### Tout afficher

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

#### Afficher la région

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

#### Afficher le profil

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

### Options

| Variable               | Défaut                                   | Description                                                                                           |
| ---------------------- | ---------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | Le format pour le rendu du module Azure.                                              |
| `symbole`              | `'󰠅 '`                                  | Le symbole utilisé dans le format.                                                    |
| `style`                | `'blue bold'`                            | Le style utilisé dans le format.                                                      |
| `disabled`             | `true`                                   | Disables the `azure` module.                                                          |
| `subscription_aliases` | `{}`                                     | Table of subscription name aliases to display in addition to Azure subscription name. |

### Exemples

#### Display Subscription Name

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = '󰠅 '
style = 'blue bold'
```

#### Display Username

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "󰠅 "
style = "blue bold"
```

#### Display Subscription Name Alias

```toml
# ~/.config/starship.toml

[azure.subscription_aliases]
very-long-subscription-name = 'vlsn'
```

## Battery

The `battery` module shows how charged the device's battery is and its current charging status.
Ce module n'est visible que lorsque la batterie de l'appareil est inférieure à 10%.

### Options

| Option               | Défaut                            | Description                                                                   |
| -------------------- | --------------------------------- | ----------------------------------------------------------------------------- |
| `full_symbol`        | `'󰁹 '`                           | Le symbole affiché lorsque la batterie est pleine.            |
| `charging_symbol`    | `'󰂄 '`                           | Le symbole affiché lorsque la batterie se charge.             |
| `discharging_symbol` | `'󰂃 '`                           | Le symbole affiché lorsque la batterie se décharge.           |
| `unknown_symbol`     | `'󰂑 '`                           | Le symbole affiché lorsque l'état de la batterie est inconnu. |
| `empty_symbol`       | `'󰂎 '`                           | Le symbole affiché lorsque la batterie est vide.              |
| `format`             | `'[$symbol$percentage]($style) '` | Format du module.                                             |
| `display`            | [link](#battery-display)          | Affiche le seuil et le style du module.                       |
| `disabled`           | `false`                           | Disables the `battery` module.                                |

### Exemple

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Indicateur de batterie

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style).
If no `display` is provided. La valeur par défaut est la suivante :

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Options

The `display` option is an array of the following table.

| Option               | Défaut       | Description                                                                                                               |
| -------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | La limite supérieure pour l'option display.                                                               |
| `style`              | `'red bold'` | Le style de l'option display si elle est utilisée.                                                        |
| `charging_symbol`    |              | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` |              | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### Exemple

```toml
[[battery.display]] # 'bold red' style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = 'bold red'

[[battery.display]] # 'bold yellow' style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦 '

# when capacity is over 30%, the battery indicator will not be displayed
```

## Buf

The `buf` module shows the currently installed version of [Buf](https://buf.build). By default, the module is shown if the current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### Options

| Option              | Défaut                                          | Description                                                            |
| ------------------- | ----------------------------------------------- | ---------------------------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | The format for the `buf` module.                       |
| `version_format`    | `'v${raw}'`                                     | Le format de la version.                               |
| `symbole`           | `'🐃 '`                                         | Le symbole utilisé avant d’afficher la version de Buf. |
| `detect_extensions` | `[]`                                            | Les extensions qui déclenchent ce module.              |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Les fichiers qui activent ce module.                   |
| `detect_folders`    | `[]`                                            | Les dossiers qui activent ce module.                   |
| `style`             | `'bold blue'`                                   | Le style pour le module.                               |
| `disabled`          | `false`                                         | Disables the `elixir` module.                          |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| `version` | `v1.0.0` | The version of `buf`                   |
| `symbole` |          | Reflète la valeur de l'option `symbol` |
| `style`\* |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime.
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `bun.lock` file
- The current directory contains a `bun.lockb` file
- The current directory contains a `bunfig.toml` file

### Options

| Option              | Défaut                                     | Description                                                                                                        |
| ------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`       | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                                | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🥟 '`                                    | A format string representing the symbol of Bun.                                                    |
| `detect_extensions` | `[]`                                       | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                       | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold red'`                               | Le style pour le module.                                                                           |
| `disabled`          | `false`                                    | Disables the `bun` module.                                                                         |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v0.1.4` | The version of `bun`                   |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

#### Customize the format

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

The `c` module shows some information about your C compiler. By default
the module will be shown if the current directory contains a `.c` or `.h`
file.

### Options

| Option              | Défaut                                                                        | Description                                                                                                        |
| ------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | La chaîne de format pour le module.                                                                |
| `version_format`    | `'v${raw}'`                                                                   | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'C '`                                                                        | Le symbole utilisé avant d’afficher les détails du compilateur                                                     |
| `detect_extensions` | `['c', 'h']`                                                                  | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                                                          | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                                                          | Les dossiers qui activent ce module.                                                               |
| `commandes`         | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | Comment détecter quel est le compilateur                                                                           |
| `style`             | `'bold 149'`                                                                  | Le style pour le module.                                                                           |
| `disabled`          | `false`                                                                       | Disables the `c` module.                                                                           |

### Variables

| Variable | Exemple                                | Description                            |
| -------- | -------------------------------------- | -------------------------------------- |
| name     | clang                                  | Le nom du compilateur                  |
| version  | 13.0.0 | La version du compilateur              |
| symbole  |                                        | Reflète la valeur de l'option `symbol` |
| style    |                                        | Reflète la valeur de l'option `style`  |

### Commandes

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship essayera d'exécuter chaque commande jusqu'à obtenir un résultat sur STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Exemple

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

### Options

| Option              | Défaut                                                                           | Description                                                                                                        |
| ------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | La chaîne de format pour le module.                                                                |
| `version_format`    | `'v${raw}'`                                                                      | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'C++ '`                                                                         | Le symbole utilisé avant d’afficher les détails du compilateur                                                     |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                                                             | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                                                             | Les dossiers qui activent ce module.                                                               |
| `commandes`         | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | Comment détecter quel est le compilateur                                                                           |
| `style`             | `'bold 149'`                                                                     | Le style pour le module.                                                                           |
| `disabled`          | `true`                                                                           | Disables the `cpp` module.                                                                         |

### Variables

| Variable | Exemple                                | Description                            |
| -------- | -------------------------------------- | -------------------------------------- |
| name     | clang++                                | Le nom du compilateur                  |
| version  | 13.0.0 | La version du compilateur              |
| symbole  |                                        | Reflète la valeur de l'option `symbol` |
| style    |                                        | Reflète la valeur de l'option `style`  |

### Commandes

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycpp', '--version']`. Starship essayera d'exécuter chaque commande jusqu'à obtenir un résultat sur STDOUT.

If a C++ compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Exemple

```toml
# ~/.config/starship.toml

[cpp]
disabled = false
format = 'via [$name $version]($style)'
```

## Caractère

The `character` module shows a character (usually an arrow) beside where the text
is entered in your terminal.

Le caractère vous dira si la dernière commande a été réussie ou pas. Il peut faire ça de deux façons:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

Par défaut, il ne change que de couleur. If you also want to change its shape take a
look at [this example](#with-custom-error-shape).

> [!WARNING]
> `vimcmd_symbol` is only supported in cmd, fish and zsh.
> `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol`
> are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

### Options

| Option                      | Défaut               | Description                                                                                             |
| --------------------------- | -------------------- | ------------------------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | Le format utilisé avant l'entrée de texte.                                              |
| `success_symbol`            | `'[❯](bold green)'`  | Le format utilisé avant l'entrée de texte si la commande précédente a réussi.           |
| `error_symbol`              | `'[❯](bold red)'`    | Le format utilisé avant l'entrée de texte si la commande précédente a échoué.           |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | Le format utilisé avant l'entrée de texte si le shell est en mode vim normal.           |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode. |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.       |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.        |
| `disabled`                  | `false`              | Disables the `character` module.                                                        |

### Variables

| Variable | Exemple | Description                                                                                                              |
| -------- | ------- | ------------------------------------------------------------------------------------------------------------------------ |
| symbole  |         | A mirror of either `success_symbol`, `error_symbol`, `vimcmd_symbol` or `vimcmd_replace_one_symbol` etc. |

### Exemples

#### Avec un caractère d'erreur personnalisé

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[✗](bold red) '
```

#### Sans caractère d'erreur personnalisé

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[➜](bold red) '
```

#### Avec une forme vim personnalisée

```toml
# ~/.config/starship.toml

[character]
vimcmd_symbol = '[V](bold green) '
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). Par défaut, le module s’activera si l’une de ces conditions est remplie:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### Options

| Option              | Défaut                                 | Description                                                                                                        |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`   | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                            | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'△ '`                                 | Le symbole utilisé avant la version de cmake.                                                      |
| `detect_extensions` | `[]`                                   | Les extensions qui déclenchent ce module                                                                           |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | Quels fichiers devraient activer ce module                                                                         |
| `detect_folders`    | `[]`                                   | Quels dossiers devraient activer ce module                                                                         |
| `style`             | `'bold blue'`                          | Le style pour le module.                                                                           |
| `disabled`          | `false`                                | Disables the `cmake` module.                                                                       |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v3.17.3` | La version de cmake                    |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL.
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `symbole`           | `'⚙️ '`                              | Le symbole utilisé avant d’afficher la version de COBOL.                                           |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | Le style pour le module.                                                                           |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `disabled`          | `false`                              | Disables the `cobol` module.                                                                       |

### Variables

| Variable | Exemple    | Description                            |
| -------- | ---------- | -------------------------------------- |
| version  | `v3.1.2.0` | The version of `cobol`                 |
| symbole  |            | Reflète la valeur de l'option `symbol` |
| style\*  |            | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Temps d'exécution

The `cmd_duration` module shows how long the last command took to execute.
The module will be shown only if the command took longer than two seconds, or
the `min_time` config value, if it exists.

> [!WARNING]
> Do not hook the DEBUG trap in Bash
>
> If you are running Starship in `bash`, do not hook the `DEBUG` trap after running
> `eval $(starship init $0)`, or this module **will** break.

Bash users who need preexec-like functionality can use
[rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec).
Simply define the arrays `preexec_functions` and `precmd_functions` before
running `eval $(starship init $0)`, and then proceed as normal.

### Options

| Option                 | Défaut                        | Description                                                                                                                                                                                                                                     |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Durée la plus courte quand afficher le temps (en millisecondes).                                                                                                                                             |
| `show_milliseconds`    | `false`                       | Afficher les millisecondes en plus des secondes pendant la durée.                                                                                                                                                               |
| `format`               | `'took [$duration]($style) '` | Format du module.                                                                                                                                                                                                               |
| `style`                | `'bold yellow'`               | Le style pour le module.                                                                                                                                                                                                        |
| `disabled`             | `false`                       | Disables the `cmd_duration` module.                                                                                                                                                                                             |
| `show_notifications`   | `false`                       | Afficher les notifications du bureau lorsque la commande est terminée.                                                                                                                                                          |
| `min_time_to_notify`   | `45_000`                      | Durée minimale après laquelle activer la notification (en millisecondes).                                                                                                                                    |
| `notification_timeout` |                               | Durée d’affichage des notifications (en milisecondes). Si non défini, la durée sera déterminée par le démon. Tous les démons de notification ne respectent pas cette option. |

### Variables

| Variable | Exemple  | Description                                   |
| -------- | -------- | --------------------------------------------- |
| duration | `16m40s` | Le temps nécessaire pour exécuter la commande |
| style\*  |          | Reflète la valeur de l'option `style`         |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

> [!TIP]
> This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.
> If you use [pixi](https://pixi.sh), you can disable pixi's prompt modifier by running `pixi config set shell.change-ps1 false`.

### Options

| Option              | Défaut                                 | Description                                                                                                                                                                                                                                                 |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbole`           | `'🅒 '`                                | Le symbole utilisé avant le nom d'environnement.                                                                                                                                                                                            |
| `style`             | `'bold green'`                         | Le style pour le module.                                                                                                                                                                                                                    |
| `format`            | `'via [$symbol$environment]($style) '` | Format du module.                                                                                                                                                                                                                           |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                                                  |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | Which environment variable(s) should trigger this module. If it's a pixi environment, this module is not being triggered by default.                                                                     |
| `disabled`          | `false`                                | Disables the `conda` module.                                                                                                                                                                                                                |

### Variables

| Variable    | Exemple      | Description                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | La version courante de conda           |
| symbole     |              | Reflète la valeur de l'option `symbol` |
| style\*     |              | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Conteneur

The `container` module displays a symbol and container name, if inside a container.

### Options

| Option     | Défaut                             | Description                                          |
| ---------- | ---------------------------------- | ---------------------------------------------------- |
| `symbole`  | `'⬢'`                              | Le symbole affiché quand vous êtes dans un conteneur |
| `style`    | `'bold red dimmed'`                | Le style pour le module.             |
| `format`   | `'[$symbol \[$name\]]($style) '` | Format du module.                    |
| `disabled` | `false`                            | Disables the `container` module.     |

### Variables

| Variable | Exemple             | Description                            |
| -------- | ------------------- | -------------------------------------- |
| name     | `fedora-toolbox:35` | Le nom du conteneur                    |
| symbole  |                     | Reflète la valeur de l'option `symbol` |
| style\*  |                     | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `symbole`           | `'🔮 '`                              | Le symbole utilisé avant d'afficher la version de crystal.                                         |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | Le style pour le module.                                                                           |
| `detect_extensions` | `['cr']`                             | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['shard.yml']`                      | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `disabled`          | `false`                              | Disables the `crystal` module.                                                                     |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v0.32.1` | The version of `crystal`               |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `daml.yaml` file

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'Λ '`                               | A format string representing the symbol of Daml                                                                    |
| `style`             | `'bold cyan'`                        | Le style pour le module.                                                                           |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['daml.yaml']`                      | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `disabled`          | `false`                              | Disables the `daml` module.                                                                        |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.2.0` | The version of `daml`                  |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml`, `pubspec.yml` or `pubspec.lock` file

### Options

| Option              | Défaut                                            | Description                                                                                                        |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`              | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                                       | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🎯 '`                                           | Une chaîne de caractères représentant le symbole de Dart                                                           |
| `detect_extensions` | `['dart']`                                        | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `['.dart_tool']`                                  | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold blue'`                                     | Le style pour le module.                                                                           |
| `disabled`          | `false`                                           | Disables the `dart` module.                                                                        |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.8.4` | The version of `dart`                  |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Options

| Option              | Défaut                                                                               | Description                                                                                                        |
| ------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                                                                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🦕 '`                                                                              | Une chaîne de caractères représentant le symbole de Deno                                                           |
| `detect_extensions` | `[]`                                                                                 | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                                                                 | Les dossiers qui activent ce module.                                                               |
| `style`             | `'green bold'`                                                                       | Le style pour le module.                                                                           |
| `disabled`          | `false`                                                                              | Disables the `deno` module.                                                                        |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.8.3` | The version of `deno`                  |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Dossier

The `directory` module shows the path to your current directory, truncated to
three parent folders. Votre dossier sera également tronqué à la racine du repo git dans lequel vous vous trouvez actuellement.

When using the `fish_style_pwd_dir_length` option, instead of hiding the path that is
truncated, you will see a shortened name of each directory based on the number
you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root,
and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before
it would have been `nixpkgs/pkgs`.

### Options

| Option                   | Défaut                                                                                                                       | Description                                                                                                                                |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length`      | `3`                                                                                                                          | Le nombre de dossiers parents auquel tronquer le chemin du répertoire courant.                                             |
| `truncate_to_repo`       | `true`                                                                                                                       | Si oui ou non tronquer à la racine du repo git dans lequel vous vous trouvez.                                              |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | Format du module.                                                                                                          |
| `style`                  | `'bold cyan'`                                                                                                                | Le style pour le module.                                                                                                   |
| `disabled`               | `false`                                                                                                                      | Disables the `directory` module.                                                                                           |
| `read_only`              | `'🔒'`                                                                                                                       | Le symbole indiquant que le répertoire courant est en lecture seule.                                                       |
| `read_only_style`        | `'red'`                                                                                                                      | Le style du symbole de lecture seule.                                                                                      |
| `truncation_symbol`      | `''`                                                                                                                         | Le symbole pour préfixer les chemins tronqués. eg: '…/'                                                    |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. The default value is equivalent to `style`. |
| `repo_root_style`        |                                                                                                                              | Le style pour la racine du dépôt Git. The default value is equivalent to `style`.                          |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                                   |
| `home_symbol`            | `'~'`                                                                                                                        | Le symbole indiquant le répertoire personnel.                                                                              |
| `use_os_path_sep`        | `true`                                                                                                                       | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows)   |

<details><summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Options avancées            | Défaut | Description                                                                                                                                                                                            |
| --------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `substitutions`             |        | An Array or table of substitutions to be made to the path.                                                                                                                             |
| `fish_style_pwd_dir_length` | `0`    | Le nombre de caractères à utiliser lors de l'application de la logique de troncature du pwd de fish.                                                                                   |
| `use_logical_path`          | `true` | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network
prefixes or development directories of Java. Notez que cela désactivera la PWD de style fish. It takes an array of the following
key/value pairs:

| Value   | Type    | Description                                             |
| ------- | ------- | ------------------------------------------------------- |
| `from`  | String  | The value to substitute                                 |
| `to`    | String  | The replacement for that value, if found                |
| `regex` | Boolean | (Optional) Whether `from` is a regex |

By using `regex = true`, you can use [Rust's regular expressions](https://docs.rs/regex/latest/regex/#syntax) in `from`.
For instance you can replace every slash except the first with the following:

```toml
substitutions = [
  { from = "^/", to = "<root>/", regex = true },
  { from = "/", to = " | " },
  { from = "^<root>", to = "/", regex = true },
]
```

This will replace `/var/log` to `/ | var | log`.

The old syntax still works, although it doesn't support regular expressions:

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

### Variables

| Variable | Exemple               | Description                           |
| -------- | --------------------- | ------------------------------------- |
| path     | `'D:/Projects'`       | Le chemin du répertoire courant       |
| style\*  | `'black bold dimmed'` | Reflète la valeur de l'option `style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

<details><summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| Variable                                                   | Exemple               | Description                             |
| ---------------------------------------------------------- | --------------------- | --------------------------------------- |
| before_root_path | `'/path/to/home/'`    | Le chemin avant le dossier racine Git   |
| repo_root                             | `'git_repo'`          | Le nom du dossier racine Git            |
| path                                                       | `'/src/lib'`          | Le reste du chemin                      |
| style                                                      | `'black bold dimmed'` | Reflète la valeur de l'option `style`   |
| repo_root_style  | `'underline white'`   | Style pour le nom du dossier racine Git |

</details>

### Exemple

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

### Options

| Option              | Défaut                                 | Description                                                                                |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | Format du module.                                                          |
| `symbole`           | `'direnv '`                            | The symbol used before displaying the direnv context.                      |
| `style`             | `'bold orange'`                        | Le style pour le module.                                                   |
| `disabled`          | `true`                                 | Disables the `direnv` module.                                              |
| `detect_extensions` | `[]`                                   | Les extensions qui déclenchent ce module.                                  |
| `detect_files`      | `['.envrc']`                           | Les fichiers qui activent ce module.                                       |
| `detect_folders`    | `[]`                                   | Les dossiers qui activent ce module.                                       |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Les variables d’environnement qui activent ce module.                      |
| `allowed_msg`       | `'allowed'`                            | The message displayed when an rc file is allowed.                          |
| `not_allowed_msg`   | `'not allowed'`                        | The message displayed when an rc file is not_allowed. |
| `denied_msg`        | `'denied'`                             | The message displayed when an rc file is denied.                           |
| `loaded_msg`        | `'loaded'`                             | The message displayed when an rc file is loaded.                           |
| `unloaded_msg`      | `'not loaded'`                         | The message displayed when an rc file is not loaded.                       |

### Variables

| Variable                     | Exemple             | Description                                             |
| ---------------------------- | ------------------- | ------------------------------------------------------- |
| loaded                       | `loaded`            | Whether the current rc file is loaded.  |
| allowed                      | `denied`            | Whether the current rc file is allowed. |
| rc_path | `/home/test/.envrc` | The current rc file path.               |
| symbole                      |                     | Reflète la valeur de l'option `symbol`. |
| style\*                      | `red bold`          | Reflète la valeur de l'option `style`.  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Contexte Docker

The `docker_context` module shows the currently active
[Docker context](https://docs.docker.com/engine/context/working-with-contexts/)
if it's not set to `default` or `desktop-linux`, or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or
`DOCKER_CONTEXT` environment variables are set (as they are meant to override
the context in use).

### Options

| Option              | Défaut                                                                                       | Description                                                                                                          |
| ------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                                                           | Format du module.                                                                                    |
| `symbole`           | `'🐳 '`                                                                                      | Le symbole utilisé avant d'afficher le contexte Docker.                                              |
| `only_with_files`   | `true`                                                                                       | Afficher uniquement quand il y a une correspondance                                                                  |
| `detect_extensions` | `[]`                                                                                         | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `['compose.yml', 'compose.yaml', 'docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                                                         | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `'blue bold'`                                                                                | Le style pour le module.                                                                             |
| `disabled`          | `false`                                                                                      | Disables the `docker_context` module.                                                                |

### Variables

| Variable | Exemple        | Description                            |
| -------- | -------------- | -------------------------------------- |
| context  | `test_context` | Le contexte docker courant             |
| symbole  |                | Reflète la valeur de l'option `symbol` |
| style\*  |                | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. Si le SDK a été épinglé dans le répertoire courant, la version épinglée est affichée. Sinon, le module affiche la dernière version installée du SDK.

Par défaut, ce module ne sera affiché dans votre invite que lorsqu'un ou plusieurs des fichiers suivants sont présents dans le répertoire courant :

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

Vous aurez également besoin du SDK .NET Core pour pouvoir l'utiliser correctement.

En interne, ce module utilise son propre mécanisme de détection de version. Typically it is twice as fast
as running `dotnet --version`, but it may show an incorrect version if your .NET project has an
unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by
setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker
(<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>)
when there is a `.csproj` file in the current directory.

### Options

| Option              | Défaut                                                                                                  | Description                                                                                                        |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                          | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                                                                                             | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'.NET '`                                                                                               | Le symbole utilisé avant d'afficher la version de dotnet.                                          |
| `heuristic`         | `true`                                                                                                  | Utilisez la détection de versions plus rapide pour garder starship instantané.                     |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                                                                                    | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold blue'`                                                                                           | Le style pour le module.                                                                           |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                                                                      |

### Variables

| Variable | Exemple          | Description                                    |
| -------- | ---------------- | ---------------------------------------------- |
| version  | `v3.1.201`       | The version of `dotnet` sdk                    |
| tfm      | `netstandard2.0` | Le Moniker de Framework Cible du projet actuel |
| symbole  |                  | Reflète la valeur de l'option `symbol`         |
| style\*  |                  | Reflète la valeur de l'option `style`          |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `mix.exs` file.

### Options

| Option              | Défaut                                                      | Description                                                                                                        |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | Format du module elixir.                                                                           |
| `version_format`    | `'v${raw}'`                                                 | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'💧 '`                                                     | Le symbole utilisé avant d'afficher la version d'Elixir/Erlang.                                    |
| `detect_extensions` | `[]`                                                        | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['mix.exs']`                                               | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                                        | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold purple'`                                             | Le style pour le module.                                                                           |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                                                      |

### Variables

| Variable                         | Exemple | Description                            |
| -------------------------------- | ------- | -------------------------------------- |
| version                          | `v1.10` | The version of `elixir`                |
| otp_version |         | The otp version of `elixir`            |
| symbole                          |         | Reflète la valeur de l'option `symbol` |
| style\*                          |         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains `*.elm` files

### Options

| Option              | Défaut                                             | Description                                                                                                        |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`               | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                                        | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🌳 '`                                            | Une chaîne de format représentant le symbole d'Elm.                                                |
| `detect_extensions` | `['elm']`                                          | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `['elm-stuff']`                                    | Les dossiers qui activent ce module.                                                               |
| `style`             | `'cyan bold'`                                      | Le style pour le module.                                                                           |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                                         |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v0.19.1` | The version of `elm`                   |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Variable d'environnement

The `env_var` module displays the current value of a selected environment variables.
Le module sera affiché si l'une de ces conditions est remplie:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

> [!TIP]
> The order in which env_var modules are shown can be individually set by including
> `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `env_var` module will simply show all env_var modules in the order they were defined.

> [!TIP]
> Multiple environmental variables can be displayed by using a `.`. (see example)
> If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.
>
> Exemple : la configuration suivante va afficher la valeur de la variable d’environnement UTILISATEUR
>
> ```toml
> # ~/.config/starship.toml
>
> [env_var.USER]
> default = 'unknown user'
> ```

### Options

| Option        | Défaut                                | Description                                                                                         |
| ------------- | ------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `symbole`     | `""`                                  | Le symbole utilisé avant d'afficher la valeur de la variable.                       |
| `variable`    |                                       | La variable d'environnement à afficher.                                             |
| `défaut`      |                                       | La valeur par défaut à afficher lorsque la variable sélectionnée n'est pas définie. |
| `format`      | `"with [$symbol$env_value]($style) "` | Format du module.                                                                   |
| `description` | `"<env_var module>"`                  | The description of the module that is shown when running `starship explain`.        |
| `disabled`    | `false`                               | Disables the `env_var` module.                                                      |
| `style`       | `"black bold dimmed"`                 | Le style pour le module.                                                            |

### Variables

| Variable                       | Exemple                                                        | Description                                |
| ------------------------------ | -------------------------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbole                        |                                                                | Reflète la valeur de l'option `symbol`     |
| style\*                        |                                                                | Reflète la valeur de l'option `style`      |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[env_var]
variable = 'SHELL'
default = 'unknown shell'
```

Afficher plusieurs variables d’environnement:

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
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `' '`                               | Le symbole utilisé avant d'afficher la version d'erlang.                                           |
| `style`             | `'bold red'`                         | Le style pour le module.                                                                           |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `disabled`          | `false`                              | Disables the `erlang` module.                                                                      |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v22.1.3` | The version of `erlang`                |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

The `fennel` module shows the currently installed version of [Fennel](https://fennel-lang.org).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a file with the `.fnl` extension

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🧅 '`                              | The symbol used before displaying the version of fennel.                                           |
| `style`             | `'bold green'`                       | Le style pour le module.                                                                           |
| `detect_extensions` | `['fnl']`                            | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `disabled`          | `false`                              | Disables the `fennel` module.                                                                      |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.2.1` | The version of `fennel`                |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## Remplissage

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are
present in a line they will split the space evenly between them. C’est utile pour aligner d’autres modules.

### Options

| Option     | Défaut         | Description                                               |
| ---------- | -------------- | --------------------------------------------------------- |
| `symbole`  | `'.'`          | Le symbole utilisé pour remplir la ligne. |
| `style`    | `'bold black'` | Le style pour le module.                  |
| `disabled` | `false`        | Disables the `fill` module                                |

### Exemple

```toml
# ~/.config/starship.toml
format = 'AA $fill BB $fill CC'

[fill]
symbol = '-'
style = 'bold green'
```

Produit une invite qui ressemble à :

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Fortran

The `fortran` module shows the current compiler version of Fortran.

### Options

| Option              | Défaut                                                                                                                      | Description                                                                                                        |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `symbole`           | `' '`                                                                                                                      | The symbol used before displaying the version of Fortran.                                          |
| `format`            | `'via [$symbol($version )]($style)'`                                                                                        | Format du module.                                                                                  |
| `version_format`    | `'${raw}'`                                                                                                                  | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold purple'`                                                                                                             | Le style pour le module.                                                                           |
| `detect_extensions` | `['f', 'F', 'for', 'FOR', 'ftn', 'FTN', 'f77', 'F77', 'f90', 'F90', 'f95', 'F95','f03', 'F03', 'f08', 'F08', 'f18', 'F18']` | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['fpm.toml']`                                                                                                              | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                                                                                                        | Les dossiers qui activent ce module.                                                               |
| `commandes`         | `[ [ 'gfortran', '--version' ], [ 'flang', '--version' ], [ 'flang-new', '--version' ] ]`                                   | Comment détecter quel est le compilateur                                                                           |
| `disabled`          | `false`                                                                                                                     | Disables the `fortran` module.                                                                     |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| name     | gfortran | Le nom du compilateur                  |
| version  | `14.2.0` | The version of the Fortran compiler    |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Commandes

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['myfortran', '--version']`. Starship essayera d'exécuter chaque commande jusqu'à obtenir un résultat sur STDOUT.

If a Fortran compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).

## Fossil Branch

The `fossil_branch` module shows the name of the active branch of the check-out in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Défaut                           | Description                                                                                                                          |
| ------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'on [$symbol$branch]($style) '` | Format du module. Use `'$branch'` to refer to the current branch name.                               |
| `symbole`           | `' '`                           | The symbol used before the branch name of the check-out in your current directory.                                   |
| `style`             | `'bold purple'`                  | Le style pour le module.                                                                                             |
| `truncation_length` | `2^63 - 1`                       | Truncates a Fossil branch name to `N` graphemes                                                                                      |
| `truncation_symbol` | `'…'`                            | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué. You can use `''` for no symbol. |
| `disabled`          | `true`                           | Disables the `fossil_branch` module.                                                                                 |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| branch   | `trunk` | The active Fossil branch               |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Fossil Metrics

The `fossil_metrics` module will show the number of added and deleted lines in the check-out in your current directory. At least v2.14 (2021-01-20) of Fossil is required.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option               | Défaut                                                       | Description                                                           |
| -------------------- | ------------------------------------------------------------ | --------------------------------------------------------------------- |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Format du module.                                     |
| `added_style`        | `'bold green'`                                               | Le style pour le compte des ajouts.                   |
| `deleted_style`      | `'bold red'`                                                 | Le style pour le compte des suppressions.             |
| `only_nonzero_diffs` | `true`                                                       | Afficher le statut seulement pour les items modifiés. |
| `disabled`           | `true`                                                       | Disables the `fossil_metrics` module.                 |

### Variables

| Variable                             | Exemple | Description                                 |
| ------------------------------------ | ------- | ------------------------------------------- |
| added                                | `1`     | Le nombre de lignes ajoutées                |
| deleted                              | `2`     | Le nombre de lignes supprimées              |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

When the module is enabled it will always be active, unless `detect_env_vars` has
been set in which case the module will only be active when one of the
environment variables has been set.

### Options

| Option            | Défaut                                                     | Description                                                                 |
| ----------------- | ---------------------------------------------------------- | --------------------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Format du module.                                           |
| `symbole`         | `'☁️  '`                                                   | Le symbole affiché avant le profil GCP actuel.              |
| `region_aliases`  | `{}`                                                       | Table des alias de région à afficher en plus du nom du GCP. |
| `project_aliases` | `{}`                                                       | Table des alias de projet à afficher en plus du nom du GCP. |
| `detect_env_vars` | `[]`                                                       | Which environmental variables should trigger this module                    |
| `style`           | `'bold blue'`                                              | Le style pour le module.                                    |
| `disabled`        | `false`                                                    | Disables the `gcloud` module.                               |

### Variables

| Variable | Exemple       | Description                                                        |
| -------- | ------------- | ------------------------------------------------------------------ |
| region   | `us-central1` | La région GCP actuelle                                             |
| account  | `foo`         | Le profil GCP actuel                                               |
| domain   | `exemple.com` | Le domaine du profil GCP actuel                                    |
| project  |               | Le projet GCP actuel                                               |
| active   | `défaut`      | The active config name written in `~/.config/gcloud/active_config` |
| symbole  |               | Reflète la valeur de l'option `symbol`                             |
| style\*  |               | Reflète la valeur de l'option `style`                              |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemples

#### Afficher le compte et le projet

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
```

#### Afficher uniquement le nom de la configuration active

```toml
# ~/.config/starship.toml

[gcloud]
format = '[$symbol$active]($style) '
style = 'bold yellow'
```

#### Afficher le compte et la région aliasée

```toml
# ~/.config/starship.toml

[gcloud]
symbol = '️🇬️ '
[gcloud.region_aliases]
us-central1 = 'uc1'
asia-northeast1 = 'an1'
```

#### Afficher le compte et le projet aliasée

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = 'vlpn'
```

## Branche Git

The `git_branch` module shows the active branch of the repo in your current directory.

### Options

| Option               | Défaut                                            | Description                                                                                                                          |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `always_show_remote` | `false`                                           | Affiche le nom de la branche suivie distante, même si elle est égale au nom de la branche locale.                    |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | Format du module. Use `'$branch'` to refer to the current branch name.                               |
| `symbole`            | `' '`                                            | Une chaîne de format représentant le symbole de la branche git.                                                      |
| `style`              | `'bold purple'`                                   | Le style pour le module.                                                                                             |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                                                                             |
| `truncation_symbol`  | `'…'`                                             | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué. You can use `''` for no symbol. |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state.                                                       |
| `ignore_branches`    | `[]`                                              | Une liste de noms à ne pas afficher. Useful for 'master' or 'main'.                                  |
| `ignore_bare_repo`   | `false`                                           | Do not show when in a bare repo.                                                                                     |
| `disabled`           | `false`                                           | Disables the `git_branch` module.                                                                                    |

### Variables

| Variable                           | Exemple  | Description                                                                                                                                                               |
| ---------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| branch                             | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | Le nom du dépôt distant.                                                                                                                                  |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                                                                          |
| symbole                            |          | Reflète la valeur de l'option `symbol`                                                                                                                                    |
| style\*                            |          | Reflète la valeur de l'option `style`                                                                                                                                     |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_branch]
symbol = '🌱 '
truncation_length = 4
truncation_symbol = ''
ignore_branches = ['master', 'main']
```

## Commit Git

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Options

| Option               | Défaut                         | Description                                                                                                          |
| -------------------- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| `commit_hash_length` | `7`                            | La longueur du hash affiché du commit git.                                                           |
| `format`             | `'[\($hash$tag\)]($style) '` | Format du module.                                                                                    |
| `style`              | `'bold green'`                 | Le style pour le module.                                                                             |
| `only_detached`      | `true`                         | Only show git commit hash when in detached `HEAD` state                                                              |
| `tag_disabled`       | `true`                         | Disables showing tag info in `git_commit` module.                                                    |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `' 🏷  '`                      | Symbole préfixant les informations affichées concernant le tag                                                       |
| `disabled`           | `false`                        | Disables the `git_commit` module.                                                                    |

### Variables

| Variable | Exemple   | Description                                                  |
| -------- | --------- | ------------------------------------------------------------ |
| hash     | `b703eb3` | Le hash du commit git actuel                                 |
| tag      | `v1.0.0`  | The tag name if showing tag info is enabled. |
| style\*  |           | Reflète la valeur de l'option `style`                        |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## État Git

The `git_state` module will show in directories which are part of a git
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc. S'il y a des informations de progression (par exemple, REBASING 3/10), ces informations seront également affichées.

### Options

| Option         | Défaut                                                          | Description                                                                                                     |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | A format string displayed when a `rebase` is in progress.                                       |
| `merge`        | `'MERGING'`                                                     | A format string displayed when a `merge` is in progress.                                        |
| `revert`       | `'REVERTING'`                                                   | A format string displayed when a `revert` is in progress.                                       |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | A format string displayed when a `cherry-pick` is in progress.                                  |
| `bisect`       | `'BISECTING'`                                                   | A format string displayed when a `bisect` is in progress.                                       |
| `am`           | `'AM'`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress. |
| `am_or_rebase` | `'AM/REBASE'`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.         |
| `style`        | `'bold yellow'`                                                 | Le style pour le module.                                                                        |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Format du module.                                                                               |
| `disabled`     | `false`                                                         | Disables the `git_state` module.                                                                |

### Variables

| Variable                              | Exemple    | Description                           |
| ------------------------------------- | ---------- | ------------------------------------- |
| state                                 | `REBASING` | L'état actuel du dépôt                |
| progress_current | `1`        | Progression de l'opération en cours   |
| progress_total   | `2`        | Progression maximale de l'opération   |
| style\*                               |            | Reflète la valeur de l'option `style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Métriques Git

The `git_metrics` module will show the number of added and deleted lines in
the current git repository.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option               | Défaut                                                       | Description                                                           |
| -------------------- | ------------------------------------------------------------ | --------------------------------------------------------------------- |
| `added_style`        | `'bold green'`                                               | Le style pour le compte des ajouts.                   |
| `deleted_style`      | `'bold red'`                                                 | Le style pour le compte des suppressions.             |
| `only_nonzero_diffs` | `true`                                                       | Afficher le statut seulement pour les items modifiés. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Format du module.                                     |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.                    |
| `ignore_submodules`  | `false`                                                      | Ignorer les changements des sous-modules                              |

### Variables

| Variable                             | Exemple | Description                                 |
| ------------------------------------ | ------- | ------------------------------------------- |
| added                                | `1`     | Le nombre de lignes ajoutées                |
| deleted                              | `2`     | Le nombre de lignes supprimées              |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Statut Git

The `git_status` module shows symbols representing the state of the repo in your
current directory.

> [!TIP]
> The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment.
> You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

### Options

| Option                 | Défaut                                          | Description                                                                                                                                    |
| ---------------------- | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`               | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status`                                                                                                            |
| `conflicted`           | `'='`                                           | The format shown when this branch has merge conflicts.                                                                         |
| `ahead`                | `'⇡'`                                           | The format shown when this branch is ahead of the branch being tracked.                                                        |
| `behind`               | `'⇣'`                                           | The format shown when this branch is behind the branch being tracked.                                                          |
| `diverged`             | `'⇕'`                                           | The format shown when this branch has diverged from the branch being tracked.                                                  |
| `up_to_date`           | `''`                                            | The format shown when this branch is up to date with the branch being tracked.                                                 |
| `untracked`            | `'?'`                                           | The format shown when there are untracked files in the working directory.                                                      |
| `stashed`              | `'\$'`                                         | The format shown when a stash exists for the local repository.                                                                 |
| `modified`             | `'!'`                                           | The format shown when there are file modifications in the working directory.                                                   |
| `staged`               | `'+'`                                           | The format shown when a new file has been added to the staging area.                                                           |
| `renamed`              | `'»'`                                           | The format shown when a renamed file has been added to the staging area.                                                       |
| `deleted`              | `'✘'`                                           | The format shown when a file's deletion has been added to the staging area.                                                    |
| `typechanged`          | `""`                                            | The format shown when a file's type has been changed in the staging area.                                                      |
| `style`                | `'bold red'`                                    | Le style pour le module.                                                                                                       |
| `ignore_submodules`    | `false`                                         | Ignorer les changements des sous-modules.                                                                                      |
| `worktree_added`       | `""`                                            | The format shown when a new file has been added in the working directory.                                                      |
| `worktree_deleted`     | `""`                                            | The format shown when a file has been deleted in the working directory.                                                        |
| `worktree_modified`    | `""`                                            | The format shown when a file has been modified in the working directory.                                                       |
| `worktree_typechanged` | `""`                                            | The format shown when a file's type has been changed in the working directory.                                                 |
| `index_added`          | `""`                                            | The format shown when a new file has been added to the staging area.                                                           |
| `index_deleted`        | `""`                                            | The format shown when a file has been deleted from the staging area.                                                           |
| `index_modified`       | `""`                                            | The format shown when a file has been modified in the staging area.                                                            |
| `index_typechanged`    | `""`                                            | The format shown when a file's type has been changed in the staging area.                                                      |
| `disabled`             | `false`                                         | Disables the `git_status` module.                                                                                              |
| `windows_starship`     |                                                 | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |
| `use_git_executable`   | `false`                                         | Do not use `gitoxide` for computing the status, but use the `git` executable instead.                                          |

### Variables

The following variables can be used in `format`:

| Variable               | Description                                                                                                                   |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `all_status`           | Shortcut for `$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`.                     |
| `ahead_behind`         | Displays `diverged`, `ahead`, `behind` or `up_to_date` format string based on the current status of the repo. |
| `conflicted`           | Displays `conflicted` when this branch has merge conflicts.                                                   |
| `untracked`            | Displays `untracked` when there are untracked files in the working directory.                                 |
| `stashed`              | Displays `stashed` when a stash exists for the local repository.                                              |
| `modified`             | Displays `modified` when there are file modifications in the working directory.                               |
| `staged`               | Displays `staged` when a new file has been added to the staging area.                                         |
| `renamed`              | Displays `renamed` when a renamed file has been added to the staging area.                                    |
| `deleted`              | Displays `deleted` when a file's deletion has been added to the staging area.                                 |
| `typechanged`          | Displays `typechanged` when a file's type has been changed in the staging area.                               |
| `worktree_added`       | Displays `worktree_added` when a new file has been added in the working directory.                            |
| `worktree_deleted`     | Displays `worktree_deleted` when a file's been deleted in the working directory.                              |
| `worktree_modified`    | Displays `worktree_modified` when a file's been modified in the working directory.                            |
| `worktree_typechanged` | Displays `worktree_typechanged` when a file's type has been changed in the working directory.                 |
| `index_added`          | Displays `index_added` when a new file has been added to the staging area.                                    |
| `index_deleted`        | Displays `index_deleted` when a file has been deleted from the staging area.                                  |
| `index_modified`       | Displays `index_modified` when a file has been modified in the staging area.                                  |
| `index_typechanged`    | Displays `index_typechanged` when a file's type has been changed in the staging area.                         |
| style\*                | Reflète la valeur de l'option `style`                                                                                         |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

The following variables can be used in `diverged`:

| Variable       | Description                                       |
| -------------- | ------------------------------------------------- |
| `ahead_count`  | Nombre de commits en avance sur la branche suivie |
| `behind_count` | Nombre de commits en retard sur la branche suivie |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed`, `deleted`, `typechanged`, `worktree_added`, `worktree_deleted`, `worktree_modified`, `worktree_typechanged`, `index_added`, `index_deleted`, `index_modified`, and `index_typechanged`:

| Variable | Description                             |
| -------- | --------------------------------------- |
| `count`  | Affiche le nombre de fichiers concernés |

### Exemple

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

Afficher le nombre de commits en avance/en retard par rapport à la branche suivie

```toml
# ~/.config/starship.toml

[git_status]
ahead = '⇡${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
behind = '⇣${count}'
```

Utiliser un exécutable Starship Windows pour les chemins Windows dans WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```

## Gleam

The `gleam` module shows the currently installed version of [Gleam](https://gleam.run/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `gleam.toml` file
- The current directory contains a file with the `.gleam` extension

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'⭐ '`                               | A format string representing the symbol of Gleam.                                                  |
| `detect_extensions` | `['gleam']`                          | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['gleam.toml']`                     | Les fichiers qui activent ce module.                                                               |
| `style`             | `'bold #FFAFF3'`                     | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `gleam` module.                                                                       |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.0.0` | The version of `gleam`                 |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `go.work` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Options

| Option              | Défaut                                                                                    | Description                                                                                                                                |
| ------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | Format du module.                                                                                                          |
| `version_format`    | `'v${raw}'`                                                                               | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch`                         |
| `symbole`           | `'🐹 '`                                                                                   | Une chaîne de caractères représentant le symbole de Go.                                                                    |
| `detect_extensions` | `['go']`                                                                                  | Les extensions qui déclenchent ce module.                                                                                  |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Les fichiers qui activent ce module.                                                                                       |
| `detect_folders`    | `['Godeps']`                                                                              | Les dossiers qui activent ce module.                                                                                       |
| `style`             | `'bold cyan'`                                                                             | Le style pour le module.                                                                                                   |
| `not_capable_style` | `'bold red'`                                                                              | The style for the module when the go directive in the go.mod file does not match the installed Go version. |
| `disabled`          | `false`                                                                                   | Disables the `golang` module.                                                                                              |

### Variables

| Variable                         | Exemple   | Description                                                                                                                                                                 |
| -------------------------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version                          | `v1.12.1` | The version of `go`                                                                                                                                                         |
| mod_version | `1.16`    | `go` version requirement as set in the go directive of `go.mod`. Will only show if the version requirement does not match the `go` version. |
| symbole                          |           | Reflète la valeur de l'option `symbol`                                                                                                                                      |
| style\*                          |           | Reflète la valeur de l'option `style`                                                                                                                                       |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

### Using `mod_version`

```toml
# ~/.config/starship.toml

[golang]
format = 'via [$symbol($version )($mod_version )]($style)'
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment.
The module will be shown when inside a guix-shell environment.

### Options

| Option     | Défaut                     | Description                                                            |
| ---------- | -------------------------- | ---------------------------------------------------------------------- |
| `format`   | `'via [$symbol]($style) '` | Format du module.                                      |
| `symbole`  | `'🐃 '`                    | A format string representing the symbol of guix-shell. |
| `style`    | `'yellow bold'`            | Le style pour le module.                               |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html)
currently used in the project directory.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🅶 '`                              | A format string representing the symbol of Gradle.                                                 |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `['gradle']`                         | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold bright-cyan'`                 | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `gradle` module.                                                                      |
| `recursive`         | `false`                              | Enables recursive finding for the `gradle` directory.                                              |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v7.5.1` | The version of `gradle`                |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `stack.yaml` file
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### Options

| Option              | Défaut                               | Description                                               |
| ------------------- | ------------------------------------ | --------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                         |
| `symbole`           | `'λ '`                               | Une chaîne de format représentant le symbole de Haskell   |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Les extensions qui déclenchent ce module. |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Les fichiers qui activent ce module.      |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.      |
| `style`             | `'bold purple'`                      | Le style pour le module.                  |
| `disabled`          | `false`                              | Disables the `haskell` module.            |

### Variables

| Variable                           | Exemple     | Description                                                                             |
| ---------------------------------- | ----------- | --------------------------------------------------------------------------------------- |
| version                            |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot                           | `lts-18.12` | L’instantané de Stack sélectionné                                                       |
| ghc\_version | `9.2.1`     | Version de GHC installée                                                                |
| symbole                            |             | Reflète la valeur de l'option `symbol`                                                  |
| style\*                            |             | Reflète la valeur de l'option `style`                                                   |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### Options

| Option              | Défaut                                                                                          | Description                                                                                                        |
| ------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                                                                                     | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | Les dossiers qui activent ce module.                                                               |
| `symbole`           | `'⌘ '`                                                                                          | A format string representing the symbol of Haxe.                                                   |
| `style`             | `'bold fg:202'`                                                                                 | Le style pour le module.                                                                           |
| `disabled`          | `false`                                                                                         | Disables the `haxe` module.                                                                        |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v4.2.5` | The version of `haxe`                  |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `symbole`           | `'⎈ '`                               | Une chaîne de format représentant le symbole de Helm.                                              |
| `style`             | `'bold white'`                       | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `helm` module.                                                                        |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v3.1.1` | The version of `helm`                  |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Nom d'hôte

The `hostname` module shows the system hostname.

### Options

| Option            | Défaut                                 | Description                                                                                                                                                                                         |
| ----------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | Afficher uniquement le nom d'hôte lorsque vous êtes connecté à une session SSH.                                                                                                     |
| `ssh_symbol`      | `'🌐 '`                                | A format string representing the symbol when connected to SSH session.                                                                                                              |
| `trim_at`         | `'.'`                                  | Chaîne à laquelle le nom d'hôte est coupé, après la première correspondance. `'.'` will stop after the first dot. `''` will disable any truncation. |
| `detect_env_vars` | `[]`                                   | Which environment variable(s) should trigger this module.                                                                                                        |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | Format du module.                                                                                                                                                                   |
| `style`           | `'bold dimmed green'`                  | Le style pour le module.                                                                                                                                                            |
| `disabled`        | `false`                                | Disables the `hostname` module.                                                                                                                                                     |
| `aliases`         | `{}`                                   | Translate system hostnames to something else. If `trim_at` is specified, only the first part will be matched and replaced.                                          |

### Variables

| Variable                        | Exemple    | Description                                           |
| ------------------------------- | ---------- | ----------------------------------------------------- |
| nom d'hôte                      | `computer` | Le nom d’hôte de l’ordinateur                         |
| style\*                         |            | Reflète la valeur de l'option `style`                 |
| ssh_symbol | `'🌏 '`    | The symbol to represent when connected to SSH session |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemples

#### Always show the hostname

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

#### Hide the hostname in remote tmux sessions

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
detect_env_vars = ['!TMUX', 'SSH_CONNECTION']
disabled = false
```

#### Replace the hostname with a nickname

```toml
# ~/.config/starship.toml
[hostname]
aliases = { "Max's MacBook Pro" = "home" }
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, or `.sdkmanrc` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Options

| Option              | Défaut                                                                                                                | Description                                                                                                        |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                                                                                                           | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                                                                                                  | Les dossiers qui activent ce module.                                                               |
| `symbole`           | `'☕ '`                                                                                                                | Une chaîne de caractères représentant le symbole de Java                                                           |
| `style`             | `'red dimmed'`                                                                                                        | Le style pour le module.                                                                           |
| `disabled`          | `false`                                                                                                               | Disables the `java` module.                                                                        |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| version  | `v14`   | The version of `java`                  |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## Jobs

The `jobs` module shows the current number of jobs running.
Le module ne sera affiché que s'il y a des tâches de fond.
The module will show the number of jobs running if there are at least
2 jobs, or more than the `number_threshold` config value, if it exists.
The module will show a symbol if there is at least 1 job, or more than the
`symbol_threshold` config value, if it exists. You can set both values
to 0 in order to _always_ show the symbol and number of jobs, even if there are
0 jobs running.

Le fonctionnement par défaut est:

- 0 tâche -> Rien n’est affiché.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

> [!WARNING]
> This module is not supported on tcsh.

> [!WARNING]
> The `threshold` option is deprecated, but if you want to use it,
> the module will show the number of jobs running if there is more than 1 job, or
> more than the `threshold` config value, if it exists. If `threshold` is set to 0,
> then the module will also show when there are 0 jobs running.

### Options

| Option             | Défaut                        | Description                                                                              |
| ------------------ | ----------------------------- | ---------------------------------------------------------------------------------------- |
| `threshold`\*      | `1`                           | Afficher le nombre de jobs si dépassé.                                   |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | Format du module.                                                        |
| `symbole`          | `'✦'`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `'bold blue'`                 | Le style pour le module.                                                 |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

\*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| number   | `1`     | Le nombre de tâches                    |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemples

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

#### Changing process grouping behavior in fish

When using the Fish shell, Starship counts **job groups** instead of individual process IDs by default. This prevents overcounting when a pipeline has multiple processes but only one suspended group. To revert to the legacy PID-based counting, please add the following to your shell config:

```fish
set -g __starship_fish_use_job_groups "false"
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `symbole`           | `'ஃ '`                               | Une chaîne de caractères représentant le symbole de Julia.                                         |
| `style`             | `'bold purple'`                      | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `julia` module.                                                                       |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.4.0` | The version of `julia`                 |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `.kt` or a `.kts` file

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['kt', 'kts']`                      | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `symbole`           | `'🅺 '`                              | Une chaîne de caractères représentant le symbole de Kotlin.                                        |
| `style`             | `'bold blue'`                        | Le style pour le module.                                                                           |
| `kotlin_binary`     | `'kotlin'`                           | Configure le binaire kotlin que Starship exécute lors de l'obtention de la version.                |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                                      |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v1.4.21` | The version of `kotlin`                |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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
> When the module is enabled it will always be active, unless any of
> `detect_env_vars`, `detect_extensions`, `detect_files` or `detect_folders` have
> been set in which case the module will only be active in directories that match
> those conditions or one of the environmental variables has been set.

### Options

> [!WARNING]
> The `context_aliases` and `user_aliases` options are deprecated. Use `contexts` and the corresponding `context_alias`
> and `user_alias` options instead.

| Option              | Défaut                                               | Description                                                                            |
| ------------------- | ---------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `symbole`           | `'☸ '`                                               | Une chaîne de format représentant le symbole affiché avant le Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | Format du module.                                                      |
| `style`             | `'cyan bold'`                                        | Le style pour le module.                                               |
| `context_aliases`\* | `{}`                                                 | Tableau des alias de contexte à afficher.                              |
| `user_aliases`\*    | `{}`                                                 | Table of user aliases to display.                                      |
| `detect_extensions` | `[]`                                                 | Les extensions qui déclenchent ce module.                              |
| `detect_files`      | `[]`                                                 | Les fichiers qui activent ce module.                                   |
| `detect_folders`    | `[]`                                                 | Les dossiers qui activent ce module.                                   |
| `detect_env_vars`   | `[]`                                                 | Which environmental variables should trigger this module                               |
| `contexts`          | `[]`                                                 | Customized styles and symbols for specific contexts.                   |
| `disabled`          | `true`                                               | Disables the `kubernetes` module.                                      |

\*: This option is deprecated, please add `contexts` with the corresponding `context_alias` and `user_alias` options instead.

To customize the style of the module for specific environments, use the following configuration as
part of the `contexts` list:

| Variable          | Description                                                                                                              |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `context_pattern` | **Required** Regular expression to match current Kubernetes context name.                                |
| `user_pattern`    | Regular expression to match current Kubernetes user name.                                                |
| `context_alias`   | Context alias to display instead of the full context name.                                               |
| `user_alias`      | User alias to display instead of the full user name.                                                     |
| `style`           | The style for the module when using this context. If not set, will use module's style.   |
| `symbole`         | The symbol for the module when using this context. If not set, will use module's symbol. |

Note that all regular expression are anchored with `^<pattern>$` and so must match the whole string. The `*_pattern`
regular expressions may contain capture groups, which can be referenced in the corresponding alias via `$name` and `$N`
(see example below and the
[rust Regex::replace() documentation](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

### Variables

| Variable  | Exemple              | Description                                      |
| --------- | -------------------- | ------------------------------------------------ |
| context   | `starship-context`   | Le nom du contexte de kubernetes actuel          |
| namespace | `starship-namespace` | Si défini, l'espace de noms de kubernetes actuel |
| user      | `starship-user`      | Si défini, l’utilisateur de kubernetes actuel    |
| cluster   | `starship-cluster`   | Si défini, le cluster de kubernetes actuel       |
| symbole   |                      | Reflète la valeur de l'option `symbol`           |
| style\*   |                      | Reflète la valeur de l'option `style`            |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
contexts = [
  { context_pattern = "dev.local.cluster.k8s", style = "green", symbol = "💔 " },
]
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Kubernetes Context specific config

The `contexts` configuration option is used to customise what the current Kubernetes context name looks
like (style and symbol) if the name matches the defined regular expression.

```toml
# ~/.config/starship.toml

[[kubernetes.contexts]]
# "bold red" style + default symbol when Kubernetes current context name equals "production" *and* the current user
# equals "admin_user"
context_pattern = "production"
user_pattern = "admin_user"
style = "bold red"
context_alias = "prod"
user_alias = "admin"

[[kubernetes.contexts]]
# "green" style + a different symbol when Kubernetes current context name contains openshift
context_pattern = ".*openshift.*"
style = "green"
symbol = "💔 "
context_alias = "openshift"

[[kubernetes.contexts]]
# Using capture groups
# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
context_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
context_alias = "gke-$cluster"
```

## Saut de ligne

The `line_break` module separates the prompt into two lines.

### Options

| Option     | Défaut  | Description                                                                        |
| ---------- | ------- | ---------------------------------------------------------------------------------- |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### Exemple

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## IP locale

The `localip` module shows the IPv4 address of the primary network interface.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option     | Défaut                    | Description                                                                       |
| ---------- | ------------------------- | --------------------------------------------------------------------------------- |
| `ssh_only` | `true`                    | Afficher uniquenement l’adresse IP si connecté à une session SSH. |
| `format`   | `'[$localipv4]($style) '` | Format du module.                                                 |
| `style`    | `'bold yellow'`           | Le style pour le module.                                          |
| `disabled` | `true`                    | Disables the `localip` module.                                    |

### Variables

| Variable  | Exemple                                                      | Description                           |
| --------- | ------------------------------------------------------------ | ------------------------------------- |
| localipv4 | 192.168.1.13 | Contient l’adresse IPv4 principale    |
| style\*   |                                                              | Reflète la valeur de l'option `style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🌙 '`                              | Une chaîne de caractères représentant le symbole de Lua.                                           |
| `detect_extensions` | `['lua']`                            | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['.lua-version']`                   | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `['lua']`                            | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold blue'`                        | Le style pour le module.                                                                           |
| `lua_binary`        | `'lua'`                              | Configure le binaire lua que Starship exécute lors de l'obtention de la version.                   |
| `disabled`          | `false`                              | Disables the `lua` module.                                                                         |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v5.4.0` | The version of `lua`                   |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Maven

The `maven` module indicates the presence of a Maven project in the current directory. If the [Maven Wrapper](https://maven.apache.org/wrapper/) is enabled, the Maven version will be parsed from `.mvn/wrapper/maven-wrapper.properties` and shown.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `pom.xml` file.
- The current directory contains a `.mvn/wrapper/maven-wrapper.properties` file.

If you use an alternate POM syntax (for example `pom.hocon`), add its filename to `detect_files`.

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🅼 '`                              | A format string representing the symbol of Maven.                                                  |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['pom.xml']`                        | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `['.mvn']`                           | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold bright-cyan'`                 | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `maven` module.                                                                       |
| `recursive`         | `false`                              | Enables recursive finding for the `.mvn` directory.                                                |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v3.2.0` | The version of `maven`                 |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Utilisation mémoire

The `memory_usage` module shows current system memory and swap usage.

Par défaut, l'utilisation du swap est affichée si le swap total du système n'est pas nul.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option      | Défaut                                         | Description                                                                                    |
| ----------- | ---------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `threshold` | `75`                                           | Masquer l'utilisation de la mémoire à moins qu'elle ne dépasse ce pourcentage. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | Format du module.                                                              |
| `symbole`   | `'🐏'`                                         | Le symbole utilisé avant d'afficher l'utilisation de la mémoire.               |
| `style`     | `'bold dimmed white'`                          | Le style pour le module.                                                       |
| `disabled`  | `true`                                         | Disables the `memory_usage` module.                                            |

### Variables

| Variable                          | Exemple       | Description                                                                                     |
| --------------------------------- | ------------- | ----------------------------------------------------------------------------------------------- |
| ram                               | `31GiB/65GiB` | La mémoire système utilisée/totale .                                            |
| ram_pct      | `48%`         | Le pourcentage de la mémoire du système actuel.                                 |
| swap\*\*                          | `1GiB/4GiB`   | La taille de la mémoire swap du fichier de mémoire swap du système courant.     |
| swap_pct\*\* | `77%`         | Le poucentage de la mémoire swap du fichier de mémoire swap du système courant. |
| symbole                           | `🐏`          | Reflète la valeur de l'option `symbol`                                                          |
| style\*                           |               | Reflète la valeur de l'option `style`                                                           |

\*: Cette variable peut uniquement être utilisée dans une chaine de style
\*\*: Les informations sur le fichier SWAP est uniquement affichée si détectée sur le système courant

### Exemple

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

### Options

| Option              | Défaut                             | Description                                                                                                               |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | Format du module.                                                                                         |
| `symbole`           | `'⬢ '`                             | The symbol used before displaying the project name.                                                       |
| `style`             | `'blue bold'`                      | Le style pour le module.                                                                                  |
| `disabled`          | `false`                            | Disables the `meson` module.                                                                              |

### Variables

| Variable | Exemple    | Description                            |
| -------- | ---------- | -------------------------------------- |
| project  | `starship` | The current Meson project name         |
| symbole  | `🐏`       | Reflète la valeur de l'option `symbol` |
| style\*  |            | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```

## Branche Mercurial

The `hg_branch` module shows the active branch and topic of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Défaut                                    | Description                                                                                                                |
| ------------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `symbole`           | `' '`                                    | Le symbole utilisé avant le marque-page hg ou le nom de la branche du dépôt dans votre répertoire courant. |
| `style`             | `'bold purple'`                           | Le style pour le module.                                                                                   |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | Format du module.                                                                                          |
| `truncation_length` | `2^63 - 1`                                | Truncates the hg branch / topic name to `N` graphemes                                                                      |
| `truncation_symbol` | `'…'`                                     | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué.                                       |
| `disabled`          | `true`                                    | Disables the `hg_branch` module.                                                                           |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| branch   | `master`  | La branche mercuriale active           |
| topic    | `feature` | The active mercurial topic             |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Mercurial State

The `hg_state` module will show in directories which are part of a mercurial
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option       | Défaut                      | Description                                                                   |
| ------------ | --------------------------- | ----------------------------------------------------------------------------- |
| `merge`      | `'MERGING'`                 | A format string displayed when a `merge` is in progress.      |
| `rebase`     | `'REBASING'`                | A format string displayed when a `rebase` is in progress.     |
| `update`     | `'UPDATING'`                | A format string displayed when a `update` is in progress.     |
| `bisect`     | `'BISECTING'`               | A format string displayed when a `bisect` is in progress.     |
| `shelve`     | `'SHELVING'`                | A format string displayed when a `shelve` is in progress.     |
| `graft`      | `'GRAFTING'`                | A format string displayed when a `graft` is in progress.      |
| `transplant` | `'TRANSPLANTING'`           | A format string displayed when a `transplant` is in progress. |
| `histedit`   | `'HISTEDITING'`             | A format string displayed when a `histedit` is in progress.   |
| `style`      | `'bold yellow'`             | Le style pour le module.                                      |
| `format`     | `'\([$state]($style)\) '` | Format du module.                                             |
| `disabled`   | `true`                      | Disables the `hg_state` module.                               |

### Variables

| Variable                              | Exemple    | Description                           |
| ------------------------------------- | ---------- | ------------------------------------- |
| state                                 | `REBASING` | L'état actuel du dépôt                |
| progress_current | `1`        | Progression de l'opération en cours   |
| progress_total   | `2`        | Progression maximale de l'opération   |
| style\*                               |            | Reflète la valeur de l'option `style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Mise

The `mise` module shows the current mise health as reported by running `mise doctor`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Défaut                                                               | Description                                                      |
| ------------------- | -------------------------------------------------------------------- | ---------------------------------------------------------------- |
| `symbole`           | `'mise '`                                                            | The symbol used before displaying _mise_ health. |
| `style`             | `'bold purple'`                                                      | Le style pour le module.                         |
| `format`            | `'on [$symbol$health]($style) '`                                     | Format du module.                                |
| `detect_extensions` | `[]`                                                                 | Les extensions qui déclenchent ce module.        |
| `detect_files`      | `['mise.toml', 'mise.local.toml', '.mise.toml', '.mise.local.toml']` | Les fichiers qui activent ce module.             |
| `detect_folders`    | `['.mise']`                                                          | Les dossiers qui activent ce module.             |
| `healthy_symbol`    | `healthy`                                                            | The message displayed when _mise_ is healthy.    |
| `unhealthy_symbol`  | `unhealthy`                                                          | The message displayed when _mise_ is unhealthy.  |
| `disabled`          | `true`                                                               | Disables the `mise` module.                      |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| health   | `healthy` | The health of _mise_                   |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[mise]
health = 'ready'
```

## Mojo

The `mojo` module shows the current version of [Mojo programming language](https://www.modular.com/mojo) installed

### Options

| Option              | Défaut                                | Description                                                            |
| ------------------- | ------------------------------------- | ---------------------------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'` | Format du module.                                      |
| `symbole`           | `'🔥 '`                               | The symbol used before displaying the version of Mojo. |
| `style`             | `'bold 208'`                          | Le style pour le module.                               |
| `disabled`          | `false`                               | Disables the `mojo` module.                            |
| `detect_extensions` | `['mojo', '🔥']`                      | Les extensions qui déclenchent ce module.              |
| `detect_files`      | `[]`                                  | Les fichiers qui activent ce module.                   |
| `detect_folders`    | `[]`                                  | Les dossiers qui activent ce module.                   |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `24.4.0` | The version of `mojo`                  |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )($hash )]($style)'
```

## NATS

The `nats` module shows the name of the current [NATS](https://nats.io) context.

### Options

| Option     | Défaut                     | Description                                                                                     |
| ---------- | -------------------------- | ----------------------------------------------------------------------------------------------- |
| `symbole`  | `'✉️ '`                    | The symbol used before the NATS context (defaults to empty). |
| `style`    | `'bold purple'`            | Le style pour le module.                                                        |
| `format`   | `'[$symbol$name]($style)'` | Format du module.                                                               |
| `disabled` | `false`                    | Disables the `nats` module.                                                     |

### Variables

| Variable | Exemple     | Description                            |
| -------- | ----------- | -------------------------------------- |
| name     | `localhost` | The name of the NATS context           |
| symbole  |             | Reflète la valeur de l'option `symbol` |
| style\*  |             | Reflète la valeur de l'option `style`  |

### Exemple

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## Network Namespace

The `netns` module shows the current network namespace.
This uses `ip netns identify` to get the network namespace, so only network namespaces mounted at `/var/run/netns` will be detected.

### Options

| Option     | Défaut                            | Description                                                                                          |
| ---------- | --------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `format`   | `'[$symbol \[$name\]]($style)'` | Format du module.                                                                    |
| `symbole`  | `'🛜 '`                           | The symbol used before the network namespace (defaults to empty). |
| `style`    | `'blue bold dimmed'`              | Le style pour le module.                                                             |
| `disabled` | `false`                           | Disables the `netns` module.                                                         |

### Variables

| Variable | Exemple    | Description                               |
| -------- | ---------- | ----------------------------------------- |
| name     | `my-netns` | The name of the current network namespace |
| symbole  |            | Reflète la valeur de l'option `symbol`    |
| style\*  |            | Reflète la valeur de l'option `style`     |

### Exemple

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module                                                                                                   |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'👑 '`                              | Le symbole utilisé avant d'afficher la version de Nim.                                             |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['nim.cfg']`                        | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold yellow'`                      | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `nim` module.                                                                         |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.2.0` | The version of `nimc`                  |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment.
Ce module s’affichera quand vous serez à l’intérieur d’un environnement nix-shell.

### Options

| Option        | Défaut                                         | Description                                                                           |
| ------------- | ---------------------------------------------- | ------------------------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | Format du module.                                                     |
| `symbole`     | `'❄️ '`                                        | Une chaîne de format représentant le symbole de nix-shell.            |
| `style`       | `'bold blue'`                                  | Le style pour le module.                                              |
| `impure_msg`  | `'impure'`                                     | Une chaîne de format affichée lorsque le shell est impur.             |
| `pure_msg`    | `'pure'`                                       | Une chaîne de format affichée lorsque le shell est pur.               |
| `unknown_msg` | `''`                                           | A format string shown when it is unknown if the shell is pure/impure. |
| `disabled`    | `false`                                        | Disables the `nix_shell` module.                                      |
| `heuristic`   | `false`                                        | Attempts to detect new `nix shell`-style shells with a heuristic.     |

### Variables

| Variable | Exemple | Description                                                                                      |
| -------- | ------- | ------------------------------------------------------------------------------------------------ |
| state    | `pure`  | L’état du nix-shell                                                                              |
| name     | `lorri` | Le nom du nix-shell                                                                              |
| level    | `1`     | The depth level of the nix-shell (Only when using [Lix](https://lix.systems)) |
| symbole  |         | Reflète la valeur de l'option `symbol`                                                           |
| style\*  |         | Reflète la valeur de l'option `style`                                                            |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

Additionally, the module will be hidden by default if the directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `bunfig.toml`, `bun.lock`, or `bun.lockb` file, overriding the above conditions.

### Options

| Option              | Défaut                                        | Description                                                                                                                                                 |
| ------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | Format du module.                                                                                                                           |
| `version_format`    | `'v${raw}'`                                   | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch`                                          |
| `symbole`           | `' '`                                        | Une chaîne de caractères représentant le symbole de Node.js.                                                                |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | Les extensions qui déclenchent ce module.                                                                                                   |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | Les fichiers qui activent ce module.                                                                                                        |
| `detect_folders`    | `['node_modules']`                            | Les dossiers qui activent ce module.                                                                                                        |
| `style`             | `'bold green'`                                | Le style pour le module.                                                                                                                    |
| `disabled`          | `false`                                       | Disables the `nodejs` module.                                                                                                               |
| `not_capable_style` | `'bold red'`                                  | Le style du module quand une propriété engines dans le package.json ne correspond pas à la version Node.js. |

### Variables

| Variable                             | Exemple    | Description                                                                                                                                                                               |
| ------------------------------------ | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version                              | `v13.12.0` | The version of `node`                                                                                                                                                                     |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbole                              |            | Reflète la valeur de l'option `symbol`                                                                                                                                                    |
| style\*                              |            | Reflète la valeur de l'option `style`                                                                                                                                                     |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Options

| Option                    | Défaut                                                                     | Description                                                                                                        |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | La chaîne de format pour le module.                                                                |
| `version_format`          | `'v${raw}'`                                                                | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`                 | `'🐫 '`                                                                    | Le symbole utilisé avant d'afficher la version de OCaml.                                           |
| `global_switch_indicator` | `''`                                                                       | La chaîne de caractères utilisée pour représenter le commutateur OPAM global.                      |
| `local_switch_indicator`  | `'*'`                                                                      | La chaîne de caractères utilisée pour représenter le commutateur OPAM local.                       |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | Les fichiers qui activent ce module.                                                               |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | Les dossiers qui activent ce module.                                                               |
| `style`                   | `'bold yellow'`                                                            | Le style pour le module.                                                                           |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                                                       |

### Variables

| Variable                              | Exemple      | Description                                                       |
| ------------------------------------- | ------------ | ----------------------------------------------------------------- |
| version                               | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbole                               |              | Reflète la valeur de l'option `symbol`                            |
| style\*                               |              | Reflète la valeur de l'option `style`                             |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Odin

The `odin` module shows the currently installed version of [Odin](https://odin-lang.org/). By default the module will be shown if the current directory contains a `.odin` file.

### Options

| Option              | Défaut                               | Description                                                            |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                      |
| `show_commit`       | `false`                              | Shows the commit as part of the version.               |
| `symbole`           | `'Ø '`                               | The symbol used before displaying the version of Odin. |
| `style`             | `'bold bright-blue'`                 | Le style pour le module.                               |
| `disabled`          | `false`                              | Disables the `odin` module.                            |
| `detect_extensions` | `['odin']`                           | Les extensions qui déclenchent ce module.              |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                   |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                   |

### Variables

| Variable | Exemple       | Description                            |
| -------- | ------------- | -------------------------------------- |
| version  | `dev-2024-03` | The version of `odin`                  |
| symbole  |               | Reflète la valeur de l'option `symbol` |
| style\*  |               | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool.
By default the module will be shown if the current directory contains a `.rego` file.

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🪖  '`                             | A format string representing the symbol of OPA.                                                    |
| `detect_extensions` | `['rego']`                           | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold blue'`                        | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `opa` module.                                                                         |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v0.44.0` | The version of `opa`                   |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module
only active when the `OS_CLOUD` env var is set, in which case it will read
`clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files).
to fetch the current project in use.

### Options

| Option     | Défaut                                          | Description                                                                    |
| ---------- | ----------------------------------------------- | ------------------------------------------------------------------------------ |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | Format du module.                                              |
| `symbole`  | `'☁️ '`                                         | Le symbole utilisé avant d'afficher le cloud OpenStack actuel. |
| `style`    | `'bold yellow'`                                 | Le style pour le module.                                       |
| `disabled` | `false`                                         | Disables the `openstack` module.                               |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| cloud    | `corp`  | Le cloud OpenStack courant             |
| project  | `dev`   | Le projet OpenStack courant            |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

> [!WARNING]
> The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option     | Défaut                | Description                                                            |
| ---------- | --------------------- | ---------------------------------------------------------------------- |
| `format`   | `'[$symbol]($style)'` | Format du module.                                      |
| `style`    | `'bold white'`        | Le style pour le module.                               |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type.
Operating system types not defined by your configuration use the default symbols table below.
All operating systems currently supported by the module are listed below.
If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
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

### Variables

| Variable | Exemple      | Description                                                        |
| -------- | ------------ | ------------------------------------------------------------------ |
| symbole  | `🎗️`        | The current operating system symbol from advanced option `symbols` |
| name     | `Arch Linux` | The current operating system name                                  |
| type     | `Arch`       | The current operating system type                                  |
| codename |              | The current operating system codename, if applicable               |
| edition  |              | The current operating system edition, if applicable                |
| version  |              | The current operating system version, if applicable                |
| style\*  |              | Reflète la valeur de l'option `style`                              |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

## Version du paquet

The `package` module is shown when the current directory is the repository for a
package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`,
`poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `galaxy`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present
  in the current directory
- [**JSR**](https://jsr.io/) – The `jsr` package version is extracted from the `jsr.json`/`jsr.jsonc` or `deno.json`/`deno.jsonc` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present
  in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present
  in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**Galaxy**](https://galaxy.ansible.com/) - The `galaxy` package version is extracted from the `galaxy.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ The version being shown is that of the package whose source code is in your
> current directory, not your package manager.

### Options

| Option            | Défaut                            | Description                                                                                                        |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`          | `'is [$symbol$version]($style) '` | Format du module.                                                                                  |
| `symbole`         | `'📦 '`                           | Le symbole utilisé avant d'afficher la version du paquet.                                          |
| `version_format`  | `'v${raw}'`                       | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | Le style pour le module.                                                                           |
| `display_private` | `false`                           | Active l’affichage des versions des paquets marqués comme privés.                                  |
| `disabled`        | `false`                           | Disables the `package` module.                                                                     |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.0.0` | La version de votre package            |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Options

| Option              | Défaut                                                                                                   | Description                                                                                                        |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | La chaîne de format pour le module.                                                                |
| `version_format`    | `'v${raw}'`                                                                                              | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🐪 '`                                                                                                  | Le symbole utilisé avant d'afficher la version de Perl                                                             |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                                                                                     | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold 149'`                                                                                             | Le style pour le module.                                                                           |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                                                        |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v5.26.1` | The version of `perl`                  |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🐘 '`                              | Le symbole utilisé avant d'afficher la version de PHP.                                             |
| `detect_extensions` | `['php']`                            | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['composer.json', '.php-version']`  | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `style`             | `'147 bold'`                         | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `php` module.                                                                         |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v7.3.8` | The version of `php`                   |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pijul Channel

The `pijul_channel` module shows the active channel of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Défaut                            | Description                                                                                          |
| ------------------- | --------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `symbole`           | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | Le style pour le module.                                                             |
| `format`            | `'on [$symbol$channel]($style) '` | Format du module.                                                                    |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                                    |
| `truncation_symbol` | `'…'`                             | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué.                 |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |

## Pixi

The `pixi` module shows the installed [pixi](https://pixi.sh) version as well as the activated
environment and project name, if `$PIXI_ENVIRONMENT_NAME` is set.

> [!TIP]
> This does not suppress pixi's own prompt modifier, you may want to run `pixi config set shell.change-ps1 false`.

### Options

| Option                     | Défaut                                                    | Description                                                                                                                         |
| -------------------------- | --------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `format`                   | `'via [$symbol($version )(\($environment\) )]($style)'` | Format du module.                                                                                                   |
| `version_format`           | `'v${raw}'`                                               | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch`. |
| `symbole`                  | `'🧚 '`                                                   | Le symbole utilisé avant le nom d'environnement.                                                                    |
| `style`                    | `'yellow bold'`                                           | Le style pour le module.                                                                                            |
| `show_default_environment` | `true`                                                    | Whether to indicate that the `default` environment of your project is activated.                                    |
| `pixi_binary`              | `['pixi']`                                                | Configures the pixi binary that Starship should execute when getting the version.                                   |
| `detect_extensions`        | `[]`                                                      | Les extensions qui déclenchent ce module.                                                                           |
| `detect_files`             | `['pixi.toml']`                                           | Les fichiers qui activent ce module.                                                                                |
| `detect_folders`           | `[]`                                                      | Les dossiers qui activent ce module.                                                                                |
| `disabled`                 | `false`                                                   | Disables the `pixi` module.                                                                                         |

### Variables

| Variable                          | Exemple      | Description                            |
| --------------------------------- | ------------ | -------------------------------------- |
| version                           | `v0.33.0`    | The version of `pixi`                  |
| environment                       | `py311`      | The current pixi environment           |
| project_name | `my-project` | The current pixi project name          |
| symbole                           |              | Reflète la valeur de l'option `symbol` |
| style                             |              | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[pixi]
format = '[$symbol$environment](yellow) '
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

> [!TIP]
> By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms).
> If you still want to enable it, [follow the example shown below](#with-pulumi-version).

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Options

| Option           | Défaut                                       | Description                                                                                                        |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | La chaîne de format pour le module.                                                                |
| `version_format` | `'v${raw}'`                                  | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`        | `' '`                                       | A format string shown before the Pulumi stack.                                                     |
| `style`          | `'bold 5'`                                   | Le style pour le module.                                                                           |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.                                     |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                                                      |

### Variables

| Variable          | Exemple    | Description                            |
| ----------------- | ---------- | -------------------------------------- |
| version           | `v0.12.24` | The version of `pulumi`                |
| stack             | `dev`      | The current Pulumi stack               |
| nom d'utilisateur | `alice`    | Le nom d’utilisateur Pulumi actuel     |
| symbole           |            | Reflète la valeur de l'option `symbol` |
| style\*           |            | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

#### Avec la version de Pulumi

```toml
# ~/.config/starship.toml

[pulumi]
format = '[🛥 ($version )$stack]($style) '
```

#### Sans la version de Pulumi

```toml
# ~/.config/starship.toml
[pulumi]
symbol = '🛥 '
format = '[$symbol$stack]($style) '
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version.
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `spago.dhall` file
- The current directory contains a `spago.yaml` file
- The current directory contains a `spago.lock` file
- The current directory contains a file with the `.purs` extension

### Options

| Option              | Défaut                                        | Description                                                                                                        |
| ------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`          | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                                   | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'<=> '`                                      | Le symbole utilisé avant d'afficher la version de PureScript.                                      |
| `detect_extensions` | `['purs']`                                    | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                          | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold white'`                                | Le style pour le module.                                                                           |
| `disabled`          | `false`                                       | Disables the `purescript` module.                                                                  |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `0.13.5` | The version of `purescript`            |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the
current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version
name. Otherwise, it will display the version number from `python --version`.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- The current directory contains a file with the `.ipynb` extension.
- Un environnement virtuel est actuellement activé

### Options

| Option               | Défaut                                                                                                       | Description                                                                                                        |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Format du module.                                                                                  |
| `version_format`     | `'v${raw}'`                                                                                                  | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`            | `'🐍 '`                                                                                                      | Une chaîne de caractères représentant le symbole de Python                                                         |
| `style`              | `'yellow bold'`                                                                                              | Le style pour le module.                                                                           |
| `pyenv_version_name` | `false`                                                                                                      | Utiliser pyenv pour obtenir la version de Python                                                                   |
| `pyenv_prefix`       | `'pyenv'`                                                                                                    | Prefix before pyenv version display, only used if pyenv is used                                                    |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should execute when getting the version.              |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                            | Les extensions qui déclenchent ce module                                                                           |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Quels fichiers devraient activer ce module                                                                         |
| `detect_folders`     | `[]`                                                                                                         | Quels dossiers devraient activer ce module                                                                         |
| `generic_venv_names` | `[]`                                                                                                         | Which venv names should be replaced with the parent directory name.                                |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                                      |

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

### Variables

| Variable                          | Exemple         | Description                                                                 |
| --------------------------------- | --------------- | --------------------------------------------------------------------------- |
| version                           | `'v3.8.1'`      | The version of `python`                                                     |
| symbole                           | `'🐍 '`         | Reflète la valeur de l'option `symbol`                                      |
| style                             | `'yellow bold'` | Reflète la valeur de l'option `style`                                       |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix`                                  |
| virtualenv                        | `'venv'`        | The current `virtualenv` name or the parent if matches `generic_venv_names` |

### Exemple

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
# Use `mise` to get the version.
python_binary = [['mise', 'exec', '--', 'python']]
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
# Ne pas déclencher pour les fichiers avec l'extension py
detect_extensions = []
```

## Quarto

The `quarto` module shows the current installed version of Quarto used in a project.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `_quarto.yml` file
- The current directory contains any `*.qmd` file

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'⨁ '`                               | A format string representing the symbol of Quarto                                                                  |
| `style`             | `'bold #75AADB'`                     | Le style pour le module.                                                                           |
| `detect_extensions` | `['.qmd']`                           | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['_quarto.yml']`                    | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `disabled`          | `false`                              | Disables the `quarto` module.                                                                      |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `1.4.549` | The version of `quarto`                |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). Le module sera affiché si l'une de ces conditions est remplie:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'📐'`                               | Une chaîne de caractères représentant le symbole de R.                                             |
| `style`             | `'blue bold'`                        | Le style pour le module.                                                                           |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Les extensions qui déclenchent ce module                                                                           |
| `detect_files`      | `['.Rprofile']`                      | Quels fichiers devraient activer ce module                                                                         |
| `detect_folders`    | `['.Rproj.user']`                    | Quels dossiers devraient activer ce module                                                                         |
| `disabled`          | `false`                              | Disables the `r` module.                                                                           |

### Variables

| Variable | Exemple       | Description                            |
| -------- | ------------- | -------------------------------------- |
| version  | `v4.0.5`      | The version of `R`                     |
| symbole  |               | Reflète la valeur de l'option `symbol` |
| style    | `'blue bold'` | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Options

| Option              | Défaut                                           | Description                                                                                                        |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | La chaîne de format pour le module.                                                                |
| `version_format`    | `'v${raw}'`                                      | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🦋 '`                                          | The symbol used before displaying the version of Raku                                                              |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['META6.json']`                                 | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                             | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold 149'`                                     | Le style pour le module.                                                                           |
| `disabled`          | `false`                                          | Disables the `raku` module.                                                                        |

### Variables

| Variable                        | Exemple | Description                            |
| ------------------------------- | ------- | -------------------------------------- |
| version                         | `v6.d`  | The version of `raku`                  |
| vm_version | `moar`  | The version of VM `raku` is built on   |
| symbole                         |         | Reflète la valeur de l'option `symbol` |
| style\*                         |         | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/).
Le module sera affiché si l'une de ces conditions est remplie:

- The current directory contains a file with `.red` or `.reds` extension

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🔺 '`                              | Une chaîne de caractères représentant le symbole de Red.                                           |
| `detect_extensions` | `['red']`                            | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `style`             | `'red bold'`                         | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `red` module.                                                                         |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.5.1` | The version of `red`                   |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/).
Le module sera affiché si l'une de ces conditions est remplie:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'💎 '`                              | Une chaîne de caractères représentant le symbole de Ruby.                                          |
| `detect_extensions` | `['rb']`                             | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Les variables d’environnement qui activent ce module.                                              |
| `style`             | `'bold red'`                         | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `ruby` module.                                                                        |

### Variables

| Variable | Exemple  | Description                                                 |
| -------- | -------- | ----------------------------------------------------------- |
| version  | `v2.5.1` | The version of `ruby`                                       |
| symbole  |          | Reflète la valeur de l'option `symbol`                      |
| style\*  |          | Reflète la valeur de l'option `style`                       |
| gemset   | `test`   | Optional, gets the current RVM gemset name. |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/).
Le module sera affiché si l'une de ces conditions est remplie:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🦀 '`                              | Une chaîne de caractères représentant le symbole de Rust                                                           |
| `detect_extensions` | `['rs']`                             | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['Cargo.toml']`                     | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold red'`                         | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `rust` module.                                                                        |

### Variables

| Variable  | Exemple           | Description                                  |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbole   |                   | Reflète la valeur de l'option `symbol`       |
| style\*   |                   | Reflète la valeur de l'option `style`        |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Options

| Option              | Défaut                                   | Description                                                                                                        |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [${symbol}(${version} )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                              | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `['.metals']`                            | Les dossiers qui activent ce module.                                                               |
| `symbole`           | `'🆂 '`                                  | Une chaîne de caractères représentant le symbole de Scala.                                         |
| `style`             | `'red dimmed'`                           | Le style pour le module.                                                                           |
| `disabled`          | `false`                                  | Disables the `scala` module.                                                                       |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `2.13.5` | The version of `scala`                 |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

### Options

| Option                 | Défaut                    | Description                                                                                                                            |
| ---------------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `bash_indicator`       | `'bsh'`                   | Chaine de formatage utilisée pour représenter bash.                                                                    |
| `fish_indicator`       | `'fsh'`                   | Chaine de formatage utilisée pour représenter fish.                                                                    |
| `zsh_indicator`        | `'zsh'`                   | Chaine de formatage utilisée pour représenter zsh.                                                                     |
| `powershell_indicator` | `'psh'`                   | Chaine de formatage utilisée pour représenter powershell.                                                              |
| `pwsh_indicator`       |                           | A format string used to represent pwsh. The default value mirrors the value of `powershell_indicator`. |
| `ion_indicator`        | `'ion'`                   | Chaine de formatage utilisée pour représenter ion.                                                                     |
| `elvish_indicator`     | `'esh'`                   | Chaine de formatage utilisée pour représenter elvish.                                                                  |
| `tcsh_indicator`       | `'tsh'`                   | Chaine de formatage utilisée pour représenter tcsh.                                                                    |
| `xonsh_indicator`      | `'xsh'`                   | Chaine de formatage utilisée pour représenter xonsh.                                                                   |
| `cmd_indicator`        | `'cmd'`                   | Chaine de formatage utilisée pour représenter cmd.                                                                     |
| `nu_indicator`         | `'nu'`                    | Chaine de formatage utilisée pour représenter nu.                                                                      |
| `unknown_indicator`    | `''`                      | La valeur par défaut à afficher quand le shell est inconnu.                                                            |
| `format`               | `'[$indicator]($style) '` | Format du module.                                                                                                      |
| `style`                | `'white bold'`            | Le style pour le module.                                                                                               |
| `disabled`             | `true`                    | Disables the `shell` module.                                                                                           |

### Variables

| Variable  | Défaut | Description                                                                |
| --------- | ------ | -------------------------------------------------------------------------- |
| indicator |        | Mirrors the value of `indicator` for currently used shell. |
| style\*   |        | Reflète la valeur de l'option `style`.                     |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemples

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

### Options

| Option          | Défaut                       | Description                                                                   |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------- |
| `threshold`     | `2`                          | Seuil d’affichage.                                            |
| `format`        | `'[$symbol$shlvl]($style) '` | Format du module.                                             |
| `symbole`       | `'↕️  '`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`        | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value           |
| `style`         | `'bold yellow'`              | Le style pour le module.                                      |
| `disabled`      | `true`                       | Disables the `shlvl` module.                                  |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| shlvl    | `3`     | The current value of `SHLVL`           |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

### Options

| Option     | Défaut                           | Description                                                            |
| ---------- | -------------------------------- | ---------------------------------------------------------------------- |
| `format`   | `'[$symbol\[$env\]]($style) '` | Format du module.                                      |
| `symbole`  | `''`                             | Une chaîne de format affichée avant le nom de l'image. |
| `style`    | `'bold dimmed blue'`             | Le style pour le module.                               |
| `disabled` | `false`                          | Disables the `singularity` module.                     |

### Variables

| Variable | Exemple      | Description                            |
| -------- | ------------ | -------------------------------------- |
| env      | `centos.img` | L’image Singularity actuelle           |
| symbole  |              | Reflète la valeur de l'option `symbol` |
| style\*  |              | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Solidity

The `solidity` module shows the currently installed version of [Solidity](https://soliditylang.org/)
The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.sol` extension

### Options

| Option              | Défaut                                                       | Description                                                                                                        |
| ------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                         | Format du module.                                                                                  |
| `version_format`    | `'v${major}.${minor}.${patch}'`                              | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'S '`                                                       | A format string representing the symbol of Solidity                                                                |
| \`compiler          | ['solc'] | The default compiler for Solidity.                                                                 |
| `detect_extensions` | `['sol']`                                                    | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                                         | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                                         | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold blue'`                                                | Le style pour le module.                                                                           |
| `disabled`          | `false`                                                      | Disables this module.                                                                              |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v0.8.1` | The version of `solidity`              |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml
[solidity]
format = "via [S $version](blue bold)"
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Options

| Option              | Défaut                                 | Description                                                                                                                                                                                         |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | Le nombre de dossiers auxquels le chemin d’environnement doit être tronqué. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbole`           | `'🅢  '`                               | Le symbole utilisé avant le nom d'environnement.                                                                                                                                    |
| `style`             | `'bold blue'`                          | Le style pour le module.                                                                                                                                                            |
| `format`            | `'via [$symbol$environment]($style) '` | Format du module.                                                                                                                                                                   |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                                                        |

### Variables

| Variable    | Exemple      | Description                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | L’environnement de spack courant       |
| symbole     |              | Reflète la valeur de l'option `symbol` |
| style\*     |              | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Statut

The `status` module displays the exit code of the previous command.
If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`.
Le code de statut est converti en entier signé 32 bits.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option                      | Défaut                                                                           | Description                                                                                          |
| --------------------------- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                    | Le format du module                                                                                  |
| `symbole`                   | `'❌'`                                                                            | The symbol displayed on program error                                                                |
| `success_symbol`            | `''`                                                                             | The symbol displayed on program success                                                              |
| `not_executable_symbol`     | `'🚫'`                                                                           | The symbol displayed when file isn't executable                                                      |
| `not_found_symbol`          | `'🔍'`                                                                           | The symbol displayed when the command can't be found                                                 |
| `sigint_symbol`             | `'🧱'`                                                                           | The symbol displayed on SIGINT (Ctrl + c)                                         |
| `signal_symbol`             | `'⚡'`                                                                            | The symbol displayed on any signal                                                                   |
| `style`                     | `'bold red'`                                                                     | Le style pour le module.                                                             |
| `success_style`             |                                                                                  | The style used on program success (defaults to `style` if unset). |
| `failure_style`             |                                                                                  | The style used on program failure (defaults to `style` if unset). |
| `recognize_signal_code`     | `true`                                                                           | Enable signal mapping from exit code                                                                 |
| `map_symbol`                | `false`                                                                          | Enable symbols mapping from exit code                                                                |
| `pipestatus`                | `false`                                                                          | Enable pipestatus reporting                                                                          |
| `pipestatus_separator`      | <code>&vert;</code>                                          | The symbol used to separate pipestatus segments (supports formatting)             |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | The format of the module when the command is a pipeline                                              |
| `pipestatus_segment_format` |                                                                                  | When specified, replaces `format` when formatting pipestatus segments                                |
| `disabled`                  | `true`                                                                           | Disables the `status` module.                                                        |

### Variables

| Variable                            | Exemple | Description                                                                                                     |
| ----------------------------------- | ------- | --------------------------------------------------------------------------------------------------------------- |
| statut                              | `127`   | Le code de sortie de la dernière commande                                                                       |
| hex_status     | `0x7F`  | Le code de sortie de la dernière commande en hexa                                                               |
| int                                 | `127`   | Le code de sortie de la dernière commande                                                                       |
| common_meaning | `ERROR` | Signification du code si n’est pas un signal                                                                    |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                                                 |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                                            |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                                    |
| pipestatus                          |         | Rendering of in pipeline programs' exit codes, this is only available in pipestatus_format |
| symbole                             |         | Reflète la valeur de l'option `symbol`                                                                          |
| style\*                             |         | Mirrors the value of option `success_style` on program success and `failure_style` otherwise                    |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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
Le module sera uniquement affiché si les identifiants sont en cache.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option          | Défaut                   | Description                                                                       |
| --------------- | ------------------------ | --------------------------------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | Le format du module                                                               |
| `symbole`       | `'🧙 '`                  | Le symbole affiché quand les identifiants sont en cache                           |
| `style`         | `'bold blue'`            | Le style pour le module.                                          |
| `allow_windows` | `false`                  | Puisque Windows n’a pas de sudo par défaut, désactivé par défaut. |
| `disabled`      | `true`                   | Disables the `sudo` module.                                       |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[sudo]
style = 'bold green'
symbol = '👩‍💻 '
disabled = false
```

```toml
# Sous windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/).
Le module sera affiché si l'une de ces conditions est remplie:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🐦 '`                              | Une chaîne de caractères représentant le symbole de Swift                                                          |
| `detect_extensions` | `['swift']`                          | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['Package.swift']`                  | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold 202'`                         | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `swift` module.                                                                       |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v5.2.4` | The version of `swift`                 |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.
It supports both Hashicorp Terraform and OpenTofu for version detection.

> [!TIP]
> By default the Terraform/OpenTofu version is not shown, since this is slow for current versions when a lot of plugins are in use.
> If you still want to enable it, [follow the example shown below](#with-terraform-version).

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Options

| Option              | Défaut                                                  | Description                                                                                                        |
| ------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol$workspace]($style) '`                    | La chaîne de format pour le module.                                                                |
| `version_format`    | `'v${raw}'`                                             | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'💠'`                                                  | Une chaîne de format montrée avant l'espace de travail terraform.                                  |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`                           | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                                    | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `['.terraform']`                                        | Les dossiers qui activent ce module.                                                               |
| `style`             | `'bold 105'`                                            | Le style pour le module.                                                                           |
| `disabled`          | `false`                                                 | Disables the `terraform` module.                                                                   |
| `commandes`         | `[ [ 'terraform', 'version' ], [ 'tofu', 'version' ] ]` | How to detect what the Terraform version is.                                                       |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `terraform`             |
| workspace | `défaut`   | L’espace de travail Terraform courant  |
| symbole   |            | Reflète la valeur de l'option `symbol` |
| style\*   |            | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

#### Avec la version de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$version $workspace]($style) '
```

#### Sans la version de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$workspace]($style) '
```

## Date et Heure

The `time` module shows the current **local** time.
The `format` configuration value is used by the [`jiff`](https://crates.io/crates/jiff) crate to control how the time is displayed. Take a look [at the jiff strftime docs](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) to see what options are available.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option            | Défaut                  | Description                                                                                                                                                                                                                                                                       |
| ----------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | La chaîne de format pour le module.                                                                                                                                                                                                                               |
| `use_12hr`        | `false`                 | Activer le format 12h                                                                                                                                                                                                                                                             |
| `time_format`     | voir plus bas           | The [jiff format string](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) used to format the time.                                                                                                                                                        |
| `style`           | `'bold yellow'`         | Le style utilisé par le module                                                                                                                                                                                                                                                    |
| `utc_time_offset` | `'local'`               | Définir le décalage horaire UTC à utiliser. Either an IANA time zone name or a range from -24 &lt; x &lt; 24. Accepte des nombres décimaux pour s'adapter aux décalages de 30/45 minutes. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                                                                                                                                                       |
| `time_range`      | `'-'`                   | Définit la plage de temps pendant laquelle le module sera affiché. Les heures doivent être spécifiées au format 24 heures                                                                                                                                         |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`.
Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable | Exemple    | Description                           |
| -------- | ---------- | ------------------------------------- |
| time     | `13:08:10` | L’heure actuelle.     |
| style\*  |            | Reflète la valeur de l'option `style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `template.typ` file
- The current directory contains any `*.typ` file

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'t '`                               | A format string representing the symbol of Typst                                                                   |
| `style`             | `'bold #0093A7'`                     | Le style pour le module.                                                                           |
| `detect_extensions` | `['.typ']`                           | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['template.typ']`                   | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `disabled`          | `false`                              | Disables the `typst` module.                                                                       |

### Variables

| Variable                           | Exemple  | Description                                                          |
| ---------------------------------- | -------- | -------------------------------------------------------------------- |
| version                            | `v0.9.0` | The version of `typst`, alias for typst_version |
| typst_version | `défaut` | The current Typst version                                            |
| symbole                            |          | Reflète la valeur de l'option `symbol`                               |
| style\*                            |          | Reflète la valeur de l'option `style`                                |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Nom d'utilisateur

The `username` module shows active user's username.
Le module sera affiché si l'une de ces conditions est remplie:

- L'utilisateur courant est root/admin
- L'utilisateur courant est différent de celui connecté
- L'utilisateur est actuellement connecté en tant que session SSH
- The variable `show_always` is set to true
- The array `detect_env_vars` contains at least the name of one environment variable, that is set

> [!TIP]
> SSH connection is detected by checking environment variables
> `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up
> these variables, one workaround is to set one of them with a dummy value.

### Options

| Option            | Défaut                  | Description                                                                                  |
| ----------------- | ----------------------- | -------------------------------------------------------------------------------------------- |
| `style_root`      | `'bold red'`            | Le style utilisé quand l'utilisateur est root/admin.                         |
| `style_user`      | `'bold yellow'`         | Le style utilisé pour les utilisateurs non-root.                             |
| `detect_env_vars` | `[]`                    | Which environment variable(s) should trigger this module. |
| `format`          | `'[$user]($style) in '` | Format du module.                                                            |
| `show_always`     | `false`                 | Always shows the `username` module.                                          |
| `disabled`        | `false`                 | Disables the `username` module.                                              |
| `aliases`         | `{}`                    | Translate system usernames to something else.                                |

### Variables

| Variable | Exemple      | Description                                                                                                 |
| -------- | ------------ | ----------------------------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `'matchai'`  | L’identifiant de l’utilisateur courant.                                                     |

### Exemple

#### Always show the username

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
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `Vagrantfile` file

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'⍱ '`                               | Une chaîne de caractères représentant le symbole de Vagrant.                                       |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['Vagrantfile']`                    | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |
| `style`             | `'cyan bold'`                        | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                                                     |

### Variables

| Variable | Exemple          | Description                            |
| -------- | ---------------- | -------------------------------------- |
| version  | `Vagrant 2.2.10` | The version of `Vagrant`               |
| symbole  |                  | Reflète la valeur de l'option `symbol` |
| style\*  |                  | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/).
Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Options

| Option              | Défaut                                       | Description                                                                                                        |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`         | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                                  | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'V '`                                       | Une chaîne de caractères représentant le symbole de V                                                              |
| `detect_extensions` | `['v']`                                      | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                         | Les dossiers qui activent ce module.                                                               |
| `style`             | `'blue bold'`                                | Le style pour le module.                                                                           |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                                                       |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| version  | `v0.2`  | The version of `v`                     |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCS

> Note the module is enabled by default but **not** included in the default list because that would be a breaking change.
> Additionally, the exact format of the module may change in the future, for example to handle right-aligned prompt.

The `vcs` module displays the current active Version Control System (VCS).
The module will be shown only if a configured VCS is currently in use.

### Options

| Option           | Défaut                                                      | Description                                                           |
| ---------------- | ----------------------------------------------------------- | --------------------------------------------------------------------- |
| `order`          | `["git", "hg", "pijul", "fossil"]`                          | The order in which to search VCSes.                   |
| `fossil_modules` | `"$fossil_branch$fossil_metrics"`                           | Modules to show when a Fossil repository is found.    |
| `git_modules`    | `"$git_branch$git_commit$git_state$git_metrics$git_status"` | Modules to show when a Git repository is found.       |
| `hg_modules`     | `"$hg_branch$hg_state"`                                     | Modules to show when a Mercurial repository is found. |
| `pijul_modules`  | `"$pijul_channel"`                                          | Modules to show when a Pijul repository is found.     |
| `disabled`       | `false`                                                     | Disables the `vcs` module.                            |

### Exemple

```toml
# ~/.config/starship.toml

[vcs]
# Will look for Git then Pijul if not found but not for other VCSes at all
order = [
  "git",
  "pijul",
]
# Any module (except `$vcs` itself to avoid infinite loops) can be included here
git_modules = "$git_branch${custom.foo}"

# See documentation for custom modules
[custom.foo]
command = 'echo foo'
detect_files = ['foo']
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository.
The module will be shown only if a repository is currently in use.

### Options

| Option     | Défaut                           | Description                                                          |
| ---------- | -------------------------------- | -------------------------------------------------------------------- |
| `symbole`  | `''`                             | Le symbole utilisé avant d'afficher le nom du dépôt. |
| `style`    | `'bold yellow'`                  | Le style pour le module.                             |
| `format`   | `'vcsh [$symbol$repo]($style) '` | Format du module.                                    |
| `disabled` | `false`                          | Disables the `vcsh` module.                          |

### Variables

| Variable | Exemple                                     | Description                            |
| -------- | ------------------------------------------- | -------------------------------------- |
| repo     | `dotfiles` if in a VCSH repo named dotfiles | Le nom du dépôt actif                  |
| symbole  |                                             | Reflète la valeur de l'option `symbol` |
| style\*  | `black bold dimmed`                         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## XMake

The `xmake` module shows the currently installed version of [XMake](https://xmake.io/). Par défaut, le module s’activera si l’une de ces conditions est remplie:

- The current directory contains a `xmake.lua` file

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'△ '`                               | Le symbole utilisé avant la version de cmake.                                                      |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module                                                                           |
| `detect_files`      | `['xmake.lua']`                      | Quels fichiers devraient activer ce module                                                                         |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module                                                                         |
| `style`             | `'bold green'`                       | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `xmake` module.                                                                       |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.9.5` | The version of xmake                   |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Zig

By default the `zig` module shows the currently installed version of [Zig](https://ziglang.org/).
Le module sera affiché si l'une de ces conditions est remplie:

- The current directory contains a `.zig` file

### Options

| Option              | Défaut                               | Description                                                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                                  |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'↯ '`                               | Le symbole utilisé avant d'afficher la version de Zig.                                             |
| `style`             | `'bold yellow'`                      | Le style pour le module.                                                                           |
| `disabled`          | `false`                              | Disables the `zig` module.                                                                         |
| `detect_extensions` | `['zig']`                            | Les extensions qui déclenchent ce module.                                                          |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                               |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v0.6.0` | The version of `zig`                   |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## Commandes personnalisées

The `custom` modules show the output of some arbitrary commands.

Ces modules seront affichés si l'une de ces conditions est remplie:

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
> If you have an interesting example not covered there, feel free to share it there!

> [!WARNING]
> If `unsafe_no_escape` is enabled or prior to starship v1.20 command output is printed unescaped to the prompt.
>
> Whatever output the command generates is printed unmodified in the prompt. This means if the output
> contains shell-specific interpretable sequences, they could be interpreted on display.
> Depending on the shell, this can mean that e.g. strings enclosed by backticks are executed by the shell.
> Such sequences are usually shell specific, e.g. you can write a command module that writes bash sequences,
> e.g. `\h`, but this module will not work in a fish or zsh shell.
>
> Format strings can also contain shell specific prompt sequences, e.g.
> [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html),
> [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

### Options

| Option              | Défaut                          | Description                                                                                                                                                                                                                                                                                                                                                      |
| ------------------- | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | La commande dont la sortie doit être affichée. La commande sera transmise au shell sur l’entrée standard.                                                                                                                                                                                                                        |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the `shell` returns a `0` status code from executing it.                                                                        |
| `require_repo`      | `false`                         | If `true`, the module will only be shown in paths containing a (git) repository. This option alone is not sufficient display condition in absence of other options.                                                                                                                                           |
| `shell`             |                                 | [Voir plus bas](#custom-command-shell)                                                                                                                                                                                                                                                                                                                           |
| `description`       | `'<custom module>'`             | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                                                                     |
| `unsafe_no_escape`  | `false`                         | When set, command output is not escaped of characters that could be interpreted by the shell.                                                                                                                                                                                                                                                    |
| `detect_files`      | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                                                                            |
| `detect_folders`    | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                                                                      |
| `detect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                                                                       |
| `symbole`           | `''`                            | Le symbole utilisé avant d'afficher la sortie de la commande.                                                                                                                                                                                                                                                                                    |
| `style`             | `'bold green'`                  | Le style pour le module.                                                                                                                                                                                                                                                                                                                         |
| `format`            | `'[$symbol($output )]($style)'` | Format du module.                                                                                                                                                                                                                                                                                                                                |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                                                                   |
| `os`                |                                 | Nom du système d'exploitation sur lequel le module sera affiché (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                         |
| `use_stdin`         |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                                                                        |

### Variables

| Variable | Description                            |
| -------- | -------------------------------------- |
| output   | The output of `command` run in `shell` |
| symbole  | Reflète la valeur de l'option `symbol` |
| style\*  | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

#### Commandes shell personnalisées

`shell` accepts a non-empty list of strings, where:

- La première chaîne est le chemin vers le shell à utiliser pour exécuter la commande.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to 'sh' on Linux, and 'cmd /C' on Windows.

The `command` (and `when`, if applicable) will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used,
the following arguments will automatically be added: `-NoProfile -Command -`.
If `shell` is not given or only contains one element and Starship detects Cmd will be used,
the following argument will automatically be added: `/C` and `stdin` will be set to `false`.
If `shell` is not given or only contains one element and Starship detects Nushell will be used,
the following arguments will automatically be added: `-c` and `stdin` will be set to `false`.
This behavior can be avoided by explicitly passing arguments to the shell, e.g.

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
> liner. Omitting this parameter might throw starship into a recursive loop
> where the shell might try to load a full profile environment with starship
> itself again and hence re-execute the custom command, getting into a never
> ending loop.
>
> Parameters similar to `-NoProfile` in PowerShell are recommended for other
> shells as well to avoid extra loading time of a custom profile on every
> starship invocation.
>
> Automatic detection of shells and proper parameters addition are currently
> implemented, but it's possible that not all shells are covered.
> [Please open an issue](https://github.com/starship/starship/issues/new/choose)
> with shell details and starship configuration if you hit such scenario.

### Exemple

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
