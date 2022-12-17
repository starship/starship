# Configuration

Pour commencer à configurer starship, créez le fichier suivant : `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toute la configuration de starship est effectuée dans ce fichier [TOML](https://github.com/toml-lang/toml) :

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

Vous pouvez choisir l'emplacement du fichier avec la variable d'environnement `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

De manière équivalente, pour Powershell (Windows), ajoutez la ligne suivante à votre `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Ou pour Cmd (Windows) ajouter cette ligne à votre `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Journalisation

Par défaut, Starship enregistre les avertissements et les erreurs dans un fichier nommé `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, où la clé de session correspond à une instance de votre terminal. Ceci peut cependant être modifié en utilisant la variable d'environnement `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

De manière équivalente, pour Powershell (Windows), ajoutez la ligne suivante à votre `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Ou pour Cmd (Windows) ajouter cette ligne à votre `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminologie

**Module**: Un composant dans l'invite de commande donnant des informations basées sur des données contextuelles de votre OS. Par exemple, le module "nodejs" montre la version de Node.js qui installée sur votre ordinateur, si votre dossier actuel est un projet Node.js.

**Variable**: Petits sous-composants qui contiennent des informations fournies par le module. Par exemple, la variable "version" dans le module "nodejs" contient la version actuelle de Node.js.

Par convention, la plupart des modules ont un préfixe de la couleur par défaut du terminal (par exemple `via` dans "nodejs") et un espace vide comme suffixe.

### Strings

In TOML syntax, [text values](https://toml.io/en/v1.0.0#string) are declared with `'`, `"`, `'''`, or `"""`.

The following Starship syntax symbols have special usage in a format string and must be escaped to display as that character: `$ [ ] ( )`.

| Symbol | Type                      | Notes                                                  |
| ------ | ------------------------- | ------------------------------------------------------ |
| `'`    | literal string            | less escaping                                          |
| `"`    | string                    | more escaping                                          |
| `'''`  | multi-line literal string | less escaping                                          |
| `"""`  | multi-line string         | more escaping, newlines in declarations can be ignored |

Par exemple :

```toml
# literal string
format = '☺\☻ '

# regular string
format = "☺\\☻ "

# escaping Starship symbols
format = '\[\$\] '
```

When using line breaks, multi-line declarations can be used. For example, if you want to print a `$` symbol on a new line, the following values for `format` are equivalent:

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

Les chaînes de formatage sont le format avec lequel un module affiche toutes ses variables. La plupart des modules ont une entrée appelée `format` qui configure le format d'affichage du module. Vous pouvez utiliser des textes, des variables et des groupes de texte dans une chaîne de format.

#### Variable

Une variable contient un symbole `$` suivi du nom de la variable. Le nom d’une variable peut seulement container des lettres, des nombres et `_`.

Par exemple :

- `'$version'` is a format string with a variable named `version`.
- `'$git_branch$git_commit'` is a format string with two variables named `git_branch` and `git_commit`.
- `'$git_branch $git_commit'` has the two variables separated with a space.

#### Groupe de texte

Un groupe de texte se compose de deux parties différentes.

La première partie, qui est entourée dans un `[]`, est une [chaîne de formatage](#format-strings). Vous pouvez y ajouter des textes, des variables, ou même des groupes de texte imbriqués.

La deuxième partie, qui est entourée par `()`, est une [chaîne de style](#style-strings). Elle peut être utilisée pour styliser la première partie.

Par exemple :

- `'[on](red bold)'` will print a string `on` with bold text colored red.
- `'[⌘ $version](bold green)'` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `'[a [b](red) c](green)'` will print `a b c` with `b` red, and `a` and `c` green.

#### Chaînes de style

La plupart des modules de Starship vous permettent de configurer leurs styles d'affichage. Cela se fait avec une entrée (généralement appelée `style`) qui est une chaîne de caractères spécifiant la configuration. Voici quelques exemples de chaînes de style avec ce qu'elles font. Pour plus de détails sur la syntaxe complète, consultez le [guide de configuration avancé](/advanced-config/).

- `'fg:green bg:blue'` sets green text on a blue background
- `'bg:blue fg:bright-green'` sets bright green text on a blue background
- `'bold fg:27'` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `'underline bg:#bf5700'` sets underlined text on a burnt orange background
- `'bold italic fg:purple'` sets bold italic purple text
- `''` explicitly disables all styling

Notez que ce style sera contrôlé par votre émulateur de terminal. Par exemple, certains émulateurs de terminal éclairciront les couleurs au lieu de mettre le texte en gras, et certains thèmes de couleurs utilisent les mêmes valeurs pour les couleurs normales et claires. De plus, pour obtenir du texte italique, votre terminal doit prendre en charge l'italique.

#### Chaînes de formatage conditionnel

Une chaîne de formatage conditionnel enveloppée dans `(` et `)` ne sera pas rendue si toutes les variables à l'intérieur sont vides.

Par exemple :

- `'(@$region)'` will show nothing if the variable `region` is `None` or empty string, otherwise `@` followed by the value of region.
- `'(some text)'` will always show nothing since there are no variables wrapped in the braces.
- When `$combined` is a shortcut for `\[$a$b\]`, `'($combined)'` will show nothing only if `$a` and `$b` are both `None`. This works the same as `'(\[$a$b\] )'`.

### Negative matching

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. These take lists of strings to match or not match. "Negative" options, those which should not be matched, are indicated with a leading '!' character. The presence of _any_ negative indicator in the directory will result in the module not being matched.

Extensions are matched against both the characters after the last dot in a filename, and the characters after the first dot in a filename. For example, `foo.bar.tar.gz` will be matched against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Files whose name begins with a dot are not considered to have extensions at all.

To see how this works in practice, you could match TypeScript but not MPEG Transport Stream files thus:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Invite

Voici la liste des options de configuration globales de l'invite de commandes.

### Options

| Option            | Défaut                         | Description                                                                                                                                                                      |
| ----------------- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [lien](#default-prompt-format) | Configure le format de l'invite.                                                                                                                                                 |
| `right_format`    | `''`                           | Voir [Activer le prompt à droite](/advanced-config/#enable-right-prompt)                                                                                                         |
| `scan_timeout`    | `30`                           | Délai maximal pour le scan des fichiers par starship (en millisecondes).                                                                                                         |
| `command_timeout` | `500`                          | Délai maximal pour les commandes exécutées par starship (en millisecondes).                                                                                                      |
| `add_newline`     | `true`                         | Insère une ligne vide entre les invites du shell.                                                                                                                                |
| `palette`         | `''`                           | Sets which color palette from `palettes` to use.                                                                                                                                 |
| `palettes`        | `{}`                           | Collection of color palettes that assign [colors](/advanced-config/#style-strings) to user-defined names. Note that color palettes cannot reference their own color definitions. |

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

Le `format` par défaut est utilisé pour définir le format de l'invite, si il est vide ou mal `formaté`. La valeur par défaut est la suivante :

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

Si vous voulez étendre le format par défaut, pour pouvoir utiliser `$all` ; les modules que vous ajouter explicitement au format ne seront pas dupliqués. Par ex.

```toml
# Move the directory to the second line
format = '$all$directory$character'
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials. The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process` or `sso_start_url` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice. If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

Lorsque vous utilisez [aws-vault](https://github.com/99designs/aws-vault) le profil est lu à partir de la variable d'environnement `AWS_VAULT` et la date d'expiration des identifiants est lue à partir de la variable d'environnement `AWS_SESSION_EXPIRATION`.

Lorsque vous utilisez [awsu](https://github.com/kreuzwerker/awsu) le profil est lu depuis la variable d'environnement `AWSU_PROFILE`.

Lorsque vous utilisez [AWSume](https://awsu.me) le profil est lu à partir de la variable d'environnement `AWSUME_PROFILE` et la date d'expiration des identifiants est lue à partir de la variable d'environnement `AWSUME_EXPIRATION`.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials` falls back to the `x_security_token_expires` key.

### Options

| Option              | Défaut                                                                | Description                                                                                                                   |
| ------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | Format du module.                                                                                                             |
| `symbole`           | `'☁️ '`                                                               | Le symbole est affiché avant le profil AWS actuel.                                                                            |
| `region_aliases`    | `{}`                                                                  | Tableau des alias de région à afficher en plus du nom AWS.                                                                    |
| `profile_aliases`   | `{}`                                                                  | Tableau des alias de profil à afficher en plus du nom AWS.                                                                    |
| `style`             | `'bold yellow'`                                                       | Le style pour le module.                                                                                                      |
| `expiration_symbol` | `X`                                                                   | Le symbole est affiché lorsque les identifiants temporaires ont expiré.                                                       |
| `disabled`          | `false`                                                               | Désactive le module `AWS`.                                                                                                    |
| `force_display`     | `false`                                                               | Si `true`, affiche les informations même si `credentials`, `credential_process` ou `sso_start_url` n'ont pas été configurées. |

### Variables

| Variable  | Exemple          | Description                                    |
| --------- | ---------------- | ---------------------------------------------- |
| region    | `ap-northeast-1` | La région AWS actuelle                         |
| profile   | `astronauts`     | Le profil AWS actuel                           |
| duration  | `2h27m20s`       | Durée de validité des identifiants temporaires |
| symbole   |                  | Reflète la valeur de l'option `symbol`         |
| style\* |                  | Reflète la valeur de l'option `style`          |

*: Cette variable peut uniquement être utilisée dans une chaine de style

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

Le module `azure` affiche l'abonnement Azure actuel. This is based on showing the name of the default subscription or the username, as defined in the `~/.azure/azureProfile.json` file.

### Options

| Variable   | Défaut                                   | Description                              |
| ---------- | ---------------------------------------- | ---------------------------------------- |
| `format`   | `'on [$symbol($subscription)]($style) '` | Le format pour le rendu du module Azure. |
| `symbole`  | `'ﴃ '`                                   | Le symbole utilisé dans le format.       |
| `style`    | `'blue bold'`                            | Le style utilisé dans le format.         |
| `disabled` | `true`                                   | Désactive le module `azure`.             |

### Exemples

#### Display Subscription Name

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = 'ﴃ '
style = 'blue bold'
```

#### Display Username

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## Battery

Le module `battery` montre à quel point la batterie de l'appareil est chargée et son état de charge actuel. Ce module n'est visible que lorsque la batterie de l'appareil est inférieure à 10%.

### Options

| Option               | Défaut                            | Description                                                   |
| -------------------- | --------------------------------- | ------------------------------------------------------------- |
| `full_symbol`        | `' '`                            | Le symbole affiché lorsque la batterie est pleine.            |
| `charging_symbol`    | `' '`                            | Le symbole affiché lorsque la batterie se charge.             |
| `discharging_symbol` | `' '`                            | Le symbole affiché lorsque la batterie se décharge.           |
| `unknown_symbol`     | `' '`                            | Le symbole affiché lorsque l'état de la batterie est inconnu. |
| `empty_symbol`       | `' '`                            | Le symbole affiché lorsque la batterie est vide.              |
| `format`             | `'[$symbol$percentage]($style) '` | Format du module.                                             |
| `display`            | [lien](#battery-display)          | Affiche le seuil et le style du module.                       |
| `disabled`           | `false`                           | Désactive le module `battery`.                                |

### Exemple

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Indicateur de batterie

L'option de configuration `display` est utilisée pour définir quand l'indicateur de batterie doit être affiché (threshold), quel symbole doit être utilisé (symbol) et à quoi il ressemble (style). Si aucun `display` n'est fourni. La valeur par défaut est la suivante :

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

La valeur par défaut pour les options `charging_symbol` et `discharging_symbol` est respectivement la valeur des options `charging_symbol` et `discharging_symbol` du module `battery`.

#### Options

L'option `display` est un tableau des propriétés suivantes.

| Option               | Défaut       | Description                                                                                                                             |
| -------------------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | La limite supérieure pour l'option display.                                                                                             |
| `style`              | `'red bold'` | Le style de l'option display si elle est utilisée.                                                                                      |
| `charging_symbol`    |              | Symbole optionnel affiché si l'option display est utilisée, la valeur par défaut est l'option `charging_symbol` du module "battery".    |
| `discharging_symbol` |              | Symbole optionnel affiché si l'option display est utilisée, la valeur par défaut est l'option `discharging_symbol` du module "battery". |

#### Exemple

```toml
[[battery.display]] # 'bold red' style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = 'bold red'

[[battery.display]] # 'bold yellow' style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦'

# when capacity is over 30%, the battery indicator will not be displayed
```

## Buf

Le module `buf` affiche la version de [Buf](https://buf.build) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- La commande [`buf`](https://github.com/bufbuild/buf) est installée.
- Le dossier actuel contient un fichier de configuration [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml) ou [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml).

### Options

| Option                               | Défaut                                          | Description                                            |
| ------------------------------------ | ----------------------------------------------- | ------------------------------------------------------ |
| `format`                             | `'with [$symbol($version )]($style)'`           | Le format du module `buf`.                             |
| `version_format`                     | `'v${raw}'`                                     | Le format de la version.                               |
| `symbole`                            | `'🐃 '`                                          | Le symbole utilisé avant d’afficher la version de Buf. |
| `detect_extensionsdetect_extensions` | `[]`                                            | Les extensions qui déclenchent ce module.              |
| `detect_files`                       | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Les fichiers qui activent ce module.                   |
| `detect_folders`                     | `[]`                                            | Quels dossiers devraient activer ce module.            |
| `style`                              | `'bold blue'`                                   | Le style pour le module.                               |
| `disabled`                           | `false`                                         | Désactive le module `elixir`.                          |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| `version` | `v1.0.0` | La version de `buf`                    |
| `symbole` |          | Reflète la valeur de l'option `symbol` |
| `style`*  |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `bun.lockb`
- Le dossier courant contient un fichier `bunfig.toml`

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🍞 '`                               | A format string representing the symbol of Bun.                                            |
| `detect_extensionsdetect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['bun.lockb', 'bunfig.toml']`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold red'`                         | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `bun` module.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.1.4` | The version of `bun`                   |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

Le module `c` affiche des informations à propos de votre compilateur C. Par défaut, ce module sera affiché si le dossier courant contient un fichier `.c` ou `.h`.

### Options

| Option                               | Défaut                                                                      | Description                                                                                |
| ------------------------------------ | --------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version(-$name) )]($style)'`                                | La chaîne de format pour le module.                                                        |
| `version_format`                     | `'v${raw}'`                                                                 | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'C '`                                                                      | Le symbole utilisé avant d’afficher les détails du compilateur                             |
| `detect_extensionsdetect_extensions` | `['c', 'h']`                                                                | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `[]`                                                                        | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                                                        | Les dossiers qui activent ce module.                                                       |
| `commands`                           | [ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ] | Comment détecter quel est le compilateur                                                   |
| `style`                              | `'bold 149'`                                                                | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                                                     | Désactive le module `c`.                                                                   |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| name     | clang   | Le nom du compilateur                  |
| version  | 13.0.0  | La version du compilateur              |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style    |         | Reflète la valeur de l'option `style`  |

Notez que `version` n’est pas dans le format par défaut.

### Commandes

L’option `commands` accepte une liste de commandes pour déterminer la version du compilateur et son nom.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship essayera d'exécuter chaque commande jusqu'à obtenir un résultat sur STDOUT.

Si un compilateur C n’est pas supporté par ce module, vous pouvez demander son ajout en [créant un ticket sur GitHub](https://github.com/starship/starship/).

### Exemple

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## Caractère

Le module `character` affiche un caractère (en général une flèche) à côté de là où vous entrez le texte dans votre terminal.

Le caractère vous dira si la dernière commande a été réussie ou pas. Il peut faire ça de deux façons:

- en changeant de couleur(`red`/`green`)
- en changeant de forme (`❯`/`✖`)

Par défaut, il ne change que de couleur. Si vous désirez également changer sa forme, jetez un à [cet exemple](#with-custom-error-shape).

::: warning

`vimcmd_symbol` is only supported in cmd, fish and zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

:::

### Options

| Option                      | Défaut               | Description                                                                             |
| --------------------------- | -------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | Le format utilisé avant l'entrée de texte.                                              |
| `success_symbol`            | `'[❯](bold green)'`  | Le format utilisé avant l'entrée de texte si la commande précédente a réussi.           |
| `error_symbol`              | `'[❯](bold red)'`    | Le format utilisé avant l'entrée de texte si la commande précédente a échoué.           |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | Le format utilisé avant l'entrée de texte si le shell est en mode vim normal.           |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode. |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.       |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.        |
| `disabled`                  | `false`              | Désactive le module `character`.                                                        |

### Variables

| Variable | Exemple | Description                                                                                              |
| -------- | ------- | -------------------------------------------------------------------------------------------------------- |
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
vicmd_symbol = '[V](bold green) '
```

## CMake

Le module `cmake` affiche la version de [CMake](https://cmake.org/) installée. Par défaut, le module s’activera si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `CMakeLists.txt`
- Le dossier courant contient un fichier `CMakeCache.txt`

### Options

| Option                               | Défaut                                 | Description                                                                                |
| ------------------------------------ | -------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'`   | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                            | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'△ '`                                 | Le symbole utilisé avant la version de cmake.                                              |
| `detect_extensionsdetect_extensions` | `[]`                                   | Les extensions qui déclenchent ce module                                                   |
| `detect_files`                       | `['CMakeLists.txt', 'CMakeCache.txt']` | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`                     | `[]`                                   | Quels dossiers devraient activer ce module                                                 |
| `style`                              | `'bold blue'`                          | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                | Désactive le module `cmake`.                                                               |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | La version de cmake                    |
| symbole   |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

## COBOL / GNUCOBOL

Le module `cobol` affiche la version de COBOL installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier finissant par `.cob` ou `.COB`
- Le dossier courant contiens un fichier finissant par `.cbl` ou `.CBL`

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `symbole`                            | `'⚙️ '`                              | Le symbole utilisé avant d’afficher la version de COBOL.                                   |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`                              | `'bold blue'`                        | Le style pour le module.                                                                   |
| `detect_extensionsdetect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`                           | `false`                              | Désactive le module `cobol`.                                                               |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v3.1.2.0` | La version de `cobol`                  |
| symbole   |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

## Temps d'exécution

Le module `cmd_duration` montre le temps qu'a pris la dernière commande pour s'exécuter. Le module ne sera affiché que si la commande a pris plus de deux secondes, ou plus que la valeur `min_time`, si elle existe.

::: warning N'accrochez pas la trappe DEBUG en Bash

Si vous utilisez starship avec `bash`, n'interceptez pas `DEBUG` après avoir exécuté `eval $(starship init $0)`, ou ce module **ne fonctionnera pas**.

:::

Les utilisateurs de Bash qui ont besoin de fonctionnalité pré-exec peuvent utiliser [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Définissez simplement les tableaux `preexec_functions` et `precmd_functions` avant d'exécuter `eval $(starship init $0)`, puis procédez comme d'habitude.

### Options

| Option                 | Défaut                        | Description                                                                                                                                                                  |
| ---------------------- | ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Durée la plus courte quand afficher le temps (en millisecondes).                                                                                                             |
| `show_milliseconds`    | `false`                       | Afficher les millisecondes en plus des secondes pendant la durée.                                                                                                            |
| `format`               | `'took [$duration]($style) '` | Format du module.                                                                                                                                                            |
| `style`                | `'bold yellow'`               | Le style pour le module.                                                                                                                                                     |
| `disabled`             | `false`                       | Désactive le module `cmd_duration`.                                                                                                                                          |
| `show_notifications`   | `false`                       | Afficher les notifications du bureau lorsque la commande est terminée.                                                                                                       |
| `min_time_to_notify`   | `45_000`                      | Durée minimale après laquelle activer la notification (en millisecondes).                                                                                                    |
| `notification_timeout` |                               | Durée d’affichage des notifications (en milisecondes). Si non défini, la durée sera déterminée par le démon. Tous les démons de notification ne respectent pas cette option. |

### Variables

| Variable  | Exemple  | Description                                   |
| --------- | -------- | --------------------------------------------- |
| duration  | `16m40s` | Le temps nécessaire pour exécuter la commande |
| style\* |          | Reflète la valeur de l'option `style`         |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

Le module `conda` affiche l’environnement [Conda](https://docs.conda.io/en/latest/) courant, si `$CONDA_DEFAULT_ENV` est définie.

::: tip

Cela ne supprime pas le modificateur d'invite de conda, vous pourriez vouloir exécuter `conda config --set changeps1 False` pour le désactiver.

:::

### Options

| Option              | Défaut                                 | Description                                                                                                                                                                                                                                   |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | Le nombre de répertoires dans lesquels le chemin d'environnement (Path) doit être tronqué, si l'environnement a été créé via `conda create -p [path]`. `0` ne signifie pas de troncature. Regardez aussi le module [`directory`](#directory). |
| `symbole`           | `'🅒 '`                                 | Le symbole utilisé avant le nom d'environnement.                                                                                                                                                                                              |
| `style`             | `'bold green'`                         | Le style pour le module.                                                                                                                                                                                                                      |
| `format`            | `'via [$symbol$environment]($style) '` | Format du module.                                                                                                                                                                                                                             |
| `ignore_base`       | `true`                                 | Ignore l'environnement `base` lorsqu'il est activé.                                                                                                                                                                                           |
| `disabled`          | `false`                                | Désactive le module `conda`.                                                                                                                                                                                                                  |

### Variables

| Variable    | Exemple      | Description                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | La version courante de conda           |
| symbole     |              | Reflète la valeur de l'option `symbol` |
| style\*   |              | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Conteneur

Le module `container` affiche un symbole et le nom du conteneur, si vous êtes dans un conteneur.

### Options

| Option     | Défaut                             | Description                                          |
| ---------- | ---------------------------------- | ---------------------------------------------------- |
| `symbole`  | `'⬢'`                              | Le symbole affiché quand vous êtes dans un conteneur |
| `style`    | `'bold red dimmed'`                | Le style pour le module.                             |
| `format`   | `'[$symbol \[$name\]]($style) '` | Format du module.                                    |
| `disabled` | `false`                            | Désactive le module `container`.                     |

### Variables

| Variable  | Exemple             | Description                            |
| --------- | ------------------- | -------------------------------------- |
| name      | `fedora-toolbox:35` | Le nom du conteneur                    |
| symbole   |                     | Reflète la valeur de l'option `symbol` |
| style\* |                     | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

Le module `crystal` affiche la version de [Crystal](https://crystal-lang.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `shard.yml`
- Le dossier courant contient un fichier `.cr`

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `symbole`                            | `'🔮 '`                               | Le symbole utilisé avant d'afficher la version de crystal.                                 |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`                              | `'bold red'`                         | Le style pour le module.                                                                   |
| `detect_extensionsdetect_extensions` | `['cr']`                             | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['shard.yml']`                      | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`                           | `false`                              | Désactive le module `crystal`.                                                             |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | La version de `crystal`                |
| symbole   |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers) SDK version when you are in the root directory of your Daml project. The `sdk-version` in the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION` environment variable. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `daml.yaml`

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'Λ '`                               | A format string representing the symbol of Daml                                            |
| `style`                              | `'bold cyan'`                        | Le style pour le module.                                                                   |
| `detect_extensionsdetect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['daml.yaml']`                      | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`                           | `false`                              | Disables the `daml` module.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.2.0` | The version of `daml`                  |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

Le module `dart` affiche la version de [Dart](https://dart.dev/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier avec l’extension `.dart`
- Le dossier courant contient un fichier `.dart_tool`
- Le dossier courant contient un fichier `pubsepc.yaml`, `pubspec.yml` ou `pubspec.lock`

### Options

| Option                               | Défaut                                            | Description                                                                                |
| ------------------------------------ | ------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'`              | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                                       | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🎯 '`                                            | Une chaîne de caractères représentant le symbole de Dart                                   |
| `detect_extensionsdetect_extensions` | `['dart']`                                        | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `['.dart_tool']`                                  | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold blue'`                                     | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                           | Désactive le module `dart`.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.8.4` | La version de `dart`                   |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

Le module `deno` affiche la version de [Deno](https://deno.land/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` ou `deps.js`

### Options

| Option                               | Défaut                                                                  | Description                                                                                |
| ------------------------------------ | ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'`                                    | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                                                             | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🦕 '`                                                                  | Une chaîne de caractères représentant le symbole de Deno                                   |
| `detect_extensionsdetect_extensions` | `[]`                                                                    | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['deno.json', 'deno.jsonc', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                                                    | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'green bold'`                                                          | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                                                 | Désactive le module `deno`.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.8.3` | La version de `deno`                   |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Dossier

Le module `directory` affiche le chemin du dossier courant, tronqué à 3 dossiers parents. Votre dossier sera également tronqué à la racine du repo git dans lequel vous vous trouvez actuellement.

Lorsque vous utilisez le style de pwd de fish, au lieu de cacher le chemin tronqué, vous verrez une abréviation du nom de chaque dossier, en fonction du nombre que vous avez utilisé comme valeur.

Par exemple, `~/Dev/Nix/nixpkgs/pkgs` où `nixpkgs` est la racine du repo, et l'option définie à `1`. Vous verrez maintenant `~/D/N/nixpkgs/pkgs`, alors que vous auriez vu `nixpkgs/pkgs` avant.

### Options

| Option                   | Défaut                                                                                                                       | Description                                                                                                               |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | Le nombre de dossiers parents auquel tronquer le chemin du répertoire courant.                                            |
| `truncate_to_repo`       | `true`                                                                                                                       | Si oui ou non tronquer à la racine du repo git dans lequel vous vous trouvez.                                             |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | Format du module.                                                                                                         |
| `style`                  | `'bold cyan'`                                                                                                                | Le style pour le module.                                                                                                  |
| `disabled`               | `false`                                                                                                                      | Désactive le module `directory`.                                                                                          |
| `read_only`              | `'🔒'`                                                                                                                        | Le symbole indiquant que le répertoire courant est en lecture seule.                                                      |
| `read_only_style`        | `'red'`                                                                                                                      | Le style du symbole de lecture seule.                                                                                     |
| `truncation_symbol`      | `''`                                                                                                                         | Le symbole pour préfixer les chemins tronqués. eg: '…/'                                                                   |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. La valeur par défaut est équivalent à `style`.             |
| `repo_root_style`        |                                                                                                                              | Le style pour la racine du dépôt Git. La valeur par défaut est équivalent à `style`.                                      |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                                  |
| `home_symbol`            | `'~'`                                                                                                                        | Le symbole indiquant le répertoire personnel.                                                                             |
| `use_os_path_sep`        | `true`                                                                                                                       | Utiliser le séparateur de chemin du système d’exploitation au lieu de toujours utiliser `/` (par ex. `\` sous Windows) |

<details>
<summary>Ce module possède quelques options de configuration avancées qui contrôlent l'affichage du répertoire.</summary>

| Options avancées            | Défaut | Description                                                                                                                                                                              |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | Une table de substitutions à appliquer aux chemins.                                                                                                                                      |
| `fish_style_pwd_dir_length` | `0`    | Le nombre de caractères à utiliser lors de l'application de la logique de troncature du pwd de fish.                                                                                     |
| `use_logical_path`          | `true` | Si `true` affiche le chemin logique issu du shell via `PWD` ou `--logical-path`. Si `false` renvoie plutôt le chemin du système de fichiers physique avec les liens symboliques résolus. |

`substitutions` vous permet de définir des remplacements arbitraires pour les chaînes littérales qui apparaissent dans le chemin, par exemple pour de longs préfixes de réseau ou des répertoires de développement (ex. Java). Notez que cela désactivera la PWD de style fish.

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_leng` interagit avec les options de troncature d'une manière qui peut être surprenante au début : si elle n'est pas nulle, les composantes du chemin qui seraient normalement tronquées sont affichées à la place avec autant de caractères. Par exemple, le chemin `/built/this/city/on/rock/and/roll`, qui devrait normalement être affiché comme `rock/and/roll`, sera affiché sous la forme de `/b/t/c/o/rock/and/roll` avec `fish_style_pwd_dir_length = 1`--les composants de chemin qui seraient normalement supprimés sont affichés avec un caractère unique. Pour `fish_style_pwd_dir_length = 2`, ce serait `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable  | Exemple               | Description                           |
| --------- | --------------------- | ------------------------------------- |
| path      | `'D:/Projects'`       | Le chemin du répertoire courant       |
| style\* | `'black bold dimmed'` | Reflète la valeur de l'option `style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

<details>
<summary>Les dépôts Git peuvent avoir des variables additionnelles.</summary>

Considérons le chemin `/path/to/home/git_repo/src/lib`

| Variable           | Exemple               | Description                             |
| ------------------ | --------------------- | --------------------------------------- |
| before_root_path | `'/path/to/home/'`    | Le chemin avant le dossier racine Git   |
| repo_root          | `'git_repo'`          | Le nom du dossier racine Git            |
| path               | `'/src/lib'`          | Le reste du chemin                      |
| style              | `'black bold dimmed'` | Reflète la valeur de l'option `style`   |
| repo_root_style  | `'underline white'`   | Style pour le nom du dossier racine Git |

</details>

### Exemple

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Contexte Docker

Le module `docker_context` affiche le [context Docker](https://docs.docker.com/engine/context/working-with-contexts/) actif, si sa valeur est différente de `default` ou si les variables d’environnement `DOCKER_MACHINE_NAME`, `DOCKER_HOST` ou `DOCKER_CONTEXT` sont définies (puisqu’elles sont utilisées pour changer le contexte utilisé).

### Options

| Option                               | Défaut                                                        | Description                                                                                            |
| ------------------------------------ | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol$context]($style) '`                            | Format du module.                                                                                      |
| `symbole`                            | `'🐳 '`                                                        | Le symbole utilisé avant d'afficher le contexte Docker.                                                |
| `only_with_files`                    | `true`                                                        | Afficher uniquement quand il y a une correspondance                                                    |
| `detect_extensionsdetect_extensions` | `[]`                                                          | Quelles extensions devraient activer ce module (il faut que `only_with_files` soit réglé sur true).    |
| `detect_files`                       | `['docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Quels noms de fichier devraient activer ce module (il faut que `only_with_files` soit réglé sur true). |
| `detect_folders`                     | `[]`                                                          | Quels dossiers devraient activer ce module (il faut que `only_with_files` soit réglé sur true).        |
| `style`                              | `'blue bold'`                                                 | Le style pour le module.                                                                               |
| `disabled`                           | `false`                                                       | Désactive le module `docker_context`.                                                                  |

### Variables

| Variable  | Exemple        | Description                            |
| --------- | -------------- | -------------------------------------- |
| context   | `test_context` | Le contexte docker courant             |
| symbole   |                | Reflète la valeur de l'option `symbol` |
| style\* |                | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

Le module `dotnet` montre la version pertinente du [SDK .NET Core](https://dotnet.microsoft.com/) pour le répertoire courant. Si le SDK a été épinglé dans le répertoire courant, la version épinglée est affichée. Sinon, le module affiche la dernière version installée du SDK.

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

En interne, ce module utilise son propre mécanisme de détection de version. Généralement, il est deux fois plus rapide que d'exécuter `dotnet --version`, mais il peut afficher une version incorrecte si votre projet .NET a une arborescence inhabituelle. Si la précision est plus importante que la vitesse, vous pouvez désactiver le mécanisme en définissant `heuristic = false` dans les options du module.

Le module affichera aussi le Moniker de Framework Cible (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>) quand il y a un fichier `.csproj` dans le dossier courant.

### Options

| Option                               | Défaut                                                                                                  | Description                                                                                |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                           | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                                                                                             | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'.NET '`                                                                                               | Le symbole utilisé avant d'afficher la version de dotnet.                                  |
| `heuristic`                          | `true`                                                                                                  | Utilisez la détection de versions plus rapide pour garder starship instantané.             |
| `detect_extensionsdetect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                                                                                    | Quels dossiers devraient activer ce module.                                                |
| `style`                              | `'bold blue'`                                                                                           | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                                                                                 | Désactive le module `dotnet`.                                                              |

### Variables

| Variable  | Exemple          | Description                                    |
| --------- | ---------------- | ---------------------------------------------- |
| version   | `v3.1.201`       | La version du sdk `dotnet`                     |
| tfm       | `netstandard2.0` | Le Moniker de Framework Cible du projet actuel |
| symbole   |                  | Reflète la valeur de l'option `symbol`         |
| style\* |                  | Reflète la valeur de l'option `style`          |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

Le module `elixir` affiche la version de [Elixir](https://elixir-lang.org/) et [Erlang/OTP](https://erlang.org/doc/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `mix.exs`.

### Options

| Option                               | Défaut                                                      | Description                                                                                |
| ------------------------------------ | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | Format du module elixir.                                                                   |
| `version_format`                     | `'v${raw}'`                                                 | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'💧 '`                                                      | Le symbole utilisé avant d'afficher la version d'Elixir/Erlang.                            |
| `detect_extensionsdetect_extensions` | `[]`                                                        | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['mix.exs']`                                               | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                                        | Quels dossiers devraient activer ce module.                                                |
| `style`                              | `'bold purple'`                                             | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                                     | Désactive le module `elixir`.                                                              |

### Variables

| Variable    | Exemple | Description                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | La version d' `elixir`                 |
| otp_version |         | La version otp d' `elixir`             |
| symbole     |         | Reflète la valeur de l'option `symbol` |
| style\*   |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

Le module `elm` affiche la version de [Elm](https://elm-lang.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `elm.json`
- Le dossier courant contient un fichier `elm-package.json`
- Le dossier courant contient un fichier `elm-version`
- Le dossier courant contient un dossier `elm-stuff`
- Le dossier courant contient des fichiers `*.elm`

### Options

| Option                               | Défaut                                             | Description                                                                                |
| ------------------------------------ | -------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'`               | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                                        | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🌳 '`                                             | Une chaîne de format représentant le symbole d'Elm.                                        |
| `detect_extensionsdetect_extensions` | `['elm']`                                          | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['elm.json', 'elm-package.json', '.elm-version']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `['elm-stuff']`                                    | Quels dossiers devraient activer ce module.                                                |
| `style`                              | `'cyan bold'`                                      | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                            | Désactive le module `elm`.                                                                 |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.19.1` | La version de `elm`                    |
| symbole   |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Variable d'environnement

Le module `env_var` affiche la valeur actuelle de la variable d’environnement choisie. Le module sera affiché si l'une de ces conditions est remplie:

- L'option `variable` correspond à une variable d'environnement existante
- L'option `variable` n'est pas définie, mais l'option `default` l'est

::: tip

Plusieurs variables d’environnement peuvent être affichées en utilisant un `.`. (voir exemple). Si l’option de configuration `variable` n’est pas définie, le module affichera la valeur de la variable dont le nom est le texte après le caractère `.`.

Exemple : la configuration suivante va afficher la valeur de la variable d’environnement UTILISATEUR

```toml
# ~/.config/starship.toml

[env_var.USER]
default = 'unknown user'
```

:::

### Options

| Option     | Défaut                         | Description                                                                         |
| ---------- | ------------------------------ | ----------------------------------------------------------------------------------- |
| `symbole`  | `''`                           | Le symbole utilisé avant d'afficher la valeur de la variable.                       |
| `variable` |                                | La variable d'environnement à afficher.                                             |
| `default`  |                                | La valeur par défaut à afficher lorsque la variable sélectionnée n'est pas définie. |
| `format`   | `'with [$env_value]($style) '` | Format du module.                                                                   |
| `disabled` | `false`                        | Désactive le module `env_var`.                                                      |

### Variables

| Variable  | Exemple                                  | Description                                      |
| --------- | ---------------------------------------- | ------------------------------------------------ |
| env_value | `Windows NT` (si _variable_ était `$OS`) | La valeur d'environnement de l'option `variable` |
| symbole   |                                          | Reflète la valeur de l'option `symbol`           |
| style\* | `black bold dimmed`                      | Reflète la valeur de l'option `style`            |

*: Cette variable peut uniquement être utilisée dans une chaine de style

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

Le module `erlang` affiche la version de [Erlang/OTP](https://erlang.org/doc/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `rebar.config`.
- Le dossier courant contient un fichier `erlang.mk`.

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `' '`                               | Le symbole utilisé avant d'afficher la version d'erlang.                                   |
| `style`                              | `'bold red'`                         | Le style pour le module.                                                                   |
| `detect_extensionsdetect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['rebar.config', 'elang.mk']`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `disabled`                           | `false`                              | Désactive le module `erlang`.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | La version d'`erlang`                  |
| symbole   |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Remplissage

Le module `fill` remplit l’espace restant sur la ligne avec un symbole. Si plusieurs modules `fill` sont présents sur une ligne, ils divisent de manière égale l’espace entre eux. C’est utile pour aligner d’autres modules.

### Options

| Option     | Défaut         | Description                               |
| ---------- | -------------- | ----------------------------------------- |
| `symbole`  | `'.'`          | Le symbole utilisé pour remplir la ligne. |
| `style`    | `'bold black'` | Le style pour le module.                  |
| `disabled` | `false`        | Désactive le module `fill`                |

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

## Google Cloud (`gcloud`)

Le module `gcloud` affiche la version de la commande [`gcloud`](https://cloud.google.com/sdk/gcloud) installée. Ceci est basé sur les fichiers `~/.config/gcloud/active_config` et `~/.config/gcloud/configurations/config_{CONFIG NAME}` et la variable d'environnement `CLOUDSDK_CONFIG`.

### Options

| Option            | Défaut                                                     | Description                                                 |
| ----------------- | ---------------------------------------------------------- | ----------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Format du module.                                           |
| `symbole`         | `'☁️  '`                                                   | Le symbole affiché avant le profil GCP actuel.              |
| `region_aliases`  | `{}`                                                       | Table des alias de région à afficher en plus du nom du GCP. |
| `project_aliases` | `{}`                                                       | Table des alias de projet à afficher en plus du nom du GCP. |
| `style`           | `'bold blue'`                                              | Le style pour le module.                                    |
| `disabled`        | `false`                                                    | Désactive le module `gcloud`.                               |

### Variables

| Variable  | Exemple       | Description                                                                   |
| --------- | ------------- | ----------------------------------------------------------------------------- |
| region    | `us-central1` | La région GCP actuelle                                                        |
| account   | `foo`         | Le profil GCP actuel                                                          |
| domain    | `exemple.com` | Le domaine du profil GCP actuel                                               |
| project   |               | Le projet GCP actuel                                                          |
| active    | `default`     | Le nom de la configuration active écrit dans `~/.config/gcloud/active_config` |
| symbole   |               | Reflète la valeur de l'option `symbol`                                        |
| style\* |               | Reflète la valeur de l'option `style`                                         |

*: Cette variable peut uniquement être utilisée dans une chaine de style

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

Le module `git_branch` affiche la branche active du dépôt dans le dossier courant.

### Options

| Option               | Défaut                                            | Description                                                                                          |
| -------------------- | ------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Affiche le nom de la branche suivie distante, même si elle est égale au nom de la branche locale.    |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | Format du module. Use `'$branch'` to refer to the current branch name.                               |
| `symbole`            | `' '`                                            | Une chaîne de format représentant le symbole de la branche git.                                      |
| `style`              | `'bold purple'`                                   | Le style pour le module.                                                                             |
| `truncation_length`  | `2^63 - 1`                                        | Tronque une branche git à `N` graphèmes.                                                             |
| `truncation_symbol`  | `'…'`                                             | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué. You can use `''` for no symbol. |
| `only_attached`      | `false`                                           | Ne montrer le nom de la branche que si elle n'est pas dans un état `HEAD` détachée.                  |
| `ignore_branches`    | `[]`                                              | Une liste de noms à ne pas afficher. Useful for 'master' or 'main'.                                  |
| `disabled`           | `false`                                           | Désactive le module `git_branch`.                                                                    |

### Variables

| Variable      | Exemple  | Description                                                                                                          |
| ------------- | -------- | -------------------------------------------------------------------------------------------------------------------- |
| branch        | `master` | Le nom de la branche actuelle, par défaut à `HEAD` s'il n'y a pas de branche actuelle (par exemple `HEAD` détachée). |
| remote_name   | `origin` | Le nom du dépôt distant.                                                                                             |
| remote_branch | `master` | Le nom de la branche suivie sur `remote_name`.                                                                       |
| symbole       |          | Reflète la valeur de l'option `symbol`                                                                               |
| style\*     |          | Reflète la valeur de l'option `style`                                                                                |

*: Cette variable peut uniquement être utilisée dans une chaine de style

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

Le module `git_commit` affiche le hash du commit courant et l’étiquette (s’il y en a) du dépôt dans le dossier courant.

### Options

| Option               | Défaut                         | Description                                                                          |
| -------------------- | ------------------------------ | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                            | La longueur du hash affiché du commit git.                                           |
| `format`             | `'[\($hash$tag\)]($style) '` | Format du module.                                                                    |
| `style`              | `'bold green'`                 | Le style pour le module.                                                             |
| `only_detached`      | `true`                         | Ne montrer le hash du commit qu'en mode `HEAD` détachée.                             |
| `tag_disabled`       | `true`                         | Désactive l'affichage des informations du tag dans le module `git_commit`.           |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `' 🏷 '`                        | Symbole préfixant les informations affichées concernant le tag                       |
| `disabled`           | `false`                        | Désactive le module `git_commit`.                                                    |

### Variables

| Variable  | Exemple   | Description                           |
| --------- | --------- | ------------------------------------- |
| hash      | `b703eb3` | Le hash du commit git actuel          |
| style\* |           | Reflète la valeur de l'option `style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## État Git

Le module `git_state` s'affichera dans les répertoires qui font partie d'un dépôt git, dans lesquels une opération est en cours, comme : _REBASING_, _BISECTING_, etc. S'il y a des informations de progression (par exemple, REBASING 3/10), ces informations seront également affichées.

### Options

| Option         | Défaut                                                          | Description                                                                                                         |
| -------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | Une chaîne de format affichée lorsqu'un `rebase` est en cours.                                                      |
| `merge`        | `'MERGING'`                                                     | Une chaîne de format affichée quand un `merge` est en cours.                                                        |
| `revert`       | `'REVERTING'`                                                   | Une chaîne de format affichée quand un `revert` est en cours.                                                       |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | Une chaîne de format affichée quand un `cherry-pick` est en cours.                                                  |
| `bisect`       | `'BISECTING'`                                                   | Une chaîne de format affichée quand un `bisect` est en cours.                                                       |
| `am`           | `'AM'`                                                          | Une chaîne de format affichée lorsqu'un `apply-mailbox` (`git am`) est en cours.                                    |
| `am_or_rebase` | `'AM/REBASE'`                                                   | Une chaîne de format affichée lorsqu'une `apply-mailbox` ou un `rebase` est en cours sans pouvoir les différencier. |
| `style`        | `'bold yellow'`                                                 | Le style pour le module.                                                                                            |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Format du module.                                                                                                   |
| `disabled`     | `false`                                                         | Désactive le module `git_state`.                                                                                    |

### Variables

| Variable         | Exemple    | Description                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | L'état actuel du dépôt                |
| progress_current | `1`        | Progression de l'opération en cours   |
| progress_total   | `2`        | Progression maximale de l'opération   |
| style\*        |            | Reflète la valeur de l'option `style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Métriques Git

Le module `git_metrics` affiche le nombre de lignes ajoutées et supprimées dans le dépôt Git courant.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option               | Défaut                                                       | Description                                           |
| -------------------- | ------------------------------------------------------------ | ----------------------------------------------------- |
| `added_style`        | `'bold green'`                                               | Le style pour le compte des ajouts.                   |
| `deleted_style`      | `'bold red'`                                                 | Le style pour le compte des suppressions.             |
| `only_nonzero_diffs` | `true`                                                       | Afficher le statut seulement pour les items modifiés. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Format du module.                                     |
| `disabled`           | `true`                                                       | Désactive le module `git_metrics`.                    |

### Variables

| Variable          | Exemple | Description                                   |
| ----------------- | ------- | --------------------------------------------- |
| added             | `1`     | Le nombre de lignes ajoutées                  |
| deleted           | `2`     | Le nombre de lignes supprimées                |
| added_style\*   |         | Possède la valeur de l’option `added_style`   |
| deleted_style\* |         | Possède la valeur de l’option `deleted_style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Statut Git

Le module `git_status` affiche des symboles représentant l’état du dépôt dans le dossier courant.

::: tip

Le module Statut Git est très lent dans les dossiers Windows (par exemple sous `/mnt/c/`) dans un environnement WSL. Vous pouvez désactiver le module ou utiliser l’option `windows_starship` pour utiliser un exécutable Starship natif pour calculer le `git_status` pour ces chemins.

:::

### Options

| Option              | Défaut                                          | Description                                                                                                                     |
| ------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | Le format par défaut du module `git_status`                                                                                     |
| `conflicted`        | `'='`                                           | Cette branche a des conflits de fusion.                                                                                         |
| `ahead`             | `'⇡'`                                           | Le format de `ahead`                                                                                                            |
| `behind`            | `'⇣'`                                           | Le format de `behind`                                                                                                           |
| `diverged`          | `'⇕'`                                           | Le format de `diverged`                                                                                                         |
| `up_to_date`        | `''`                                            | The format de `up_to_date`                                                                                                      |
| `untracked`         | `'?'`                                           | Le format de `untracked`                                                                                                        |
| `stashed`           | `'$'`                                           | Le format de `stashed`                                                                                                          |
| `modified`          | `'!'`                                           | Le format de `modified`                                                                                                         |
| `staged`            | `'+'`                                           | Le format de `staged`                                                                                                           |
| `renamed`           | `'»'`                                           | Le format de `renamed`                                                                                                          |
| `deleted`           | `'✘'`                                           | Le format de `deleted`                                                                                                          |
| `style`             | `'bold red'`                                    | Le style pour le module.                                                                                                        |
| `ignore_submodules` | `false`                                         | Ignorer les changements des sous-modules.                                                                                       |
| `disabled`          | `false`                                         | Désactive le module `git_status`.                                                                                               |
| `windows_starship`  |                                                 | Utiliser ce chemin (Linux) vers un exécutable Starship Windows pour afficher le `git_status` pour les chemins Windows dans WSL. |

### Variables

Les variables suivantes peuvent être utilisées pour la valeur de `format`:

| Variable       | Description                                                                                                           |
| -------------- | --------------------------------------------------------------------------------------------------------------------- |
| `all_status`   | Raccourci pour `$conflicted$stashed$deleted$renamed$modified$staged$untracked`                                        |
| `ahead_behind` | Affiche la chaine de formatage `diverged`, `ahead`, `behind` ou `up_to_date` en se basant sur l’état actuel du dépôt. |
| `conflicted`   | Affiche `conflicted` lorsque la branche courante a des conflits de fusion.                                            |
| `untracked`    | Affiche `untracked` lorsqu'il y a des fichiers non suivis dans le répertoire de travail.                              |
| `stashed`      | Affiche `stashed` lorsqu'une remise existe pour le dépôt local.                                                       |
| `modified`     | Affiche `modified` lorsqu'il y a des fichiers modifiés dans le répertoire de travail.                                 |
| `staged`       | Affiche `staged` lorsqu'un nouveau fichier a été ajouté à la zone de validation.                                      |
| `renamed`      | Affiche `renamed` lorsqu'un fichier renommé a été ajouté à la zone de validation.                                     |
| `deleted`      | Affiche `deleted` lorsque la suppression d'un fichier a été ajoutée à la zone de validation.                          |
| style\*      | Reflète la valeur de l'option `style`                                                                                 |

*: Cette variable peut uniquement être utilisée dans une chaine de style

Les variables suivantes peuvent être utilisées pour la valeur de `diverged`:

| Variable       | Description                                       |
| -------------- | ------------------------------------------------- |
| `ahead_count`  | Nombre de commits en avance sur la branche suivie |
| `behind_count` | Nombre de commits en retard sur la branche suivie |

Les variables suivantes peuvent êtres utilisées dans `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` et `deleted`:

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

## Go

Le module `golang` affiche la version de [Go](https://golang.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `go.mod`
- Le dossier courant contient un fichier `go.sum`
- Le dossier courant contient un fichier `go.work`
- Le dossier courant contient un fichier `glide.yaml`
- Le dossier courant contient un fichier `Gopkg.yml`
- Le dossier courant contient un fichier `Gopkg.lock`
- Le dossier courant contient un fichier `.go-version`
- Le dossier courant contient un dossier `Godeps`
- Le dossier courant contient un fichier avec l’extension `.go`

### Options

| Option                               | Défaut                                                                                    | Description                                                                                |
| ------------------------------------ | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'`                                                      | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                                                                               | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🐹 '`                                                                                    | Une chaîne de caractères représentant le symbole de Go.                                    |
| `detect_extensionsdetect_extensions` | `['go']`                                                                                  | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `['Godeps']`                                                                              | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold cyan'`                                                                             | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                                                                   | Désactive le module `golang`.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | La version de `go`                     |
| symbole   |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment. The module will be shown when inside a guix-shell environment.

### Options

| Option     | Défaut                     | Description                                            |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'via [$symbol]($style) '` | Format du module.                                      |
| `symbole`  | `"🐃 "`                     | A format string representing the symbol of guix-shell. |
| `style`    | `"yellow bold"`            | Le style pour le module.                               |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| symbole   |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) currently used in the project directory.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`                     | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `"🅶 "`                               | A format string representing the symbol of Gradle.                                         |
| `detect_extensionsdetect_extensions` | `["gradle", "gradle.kts"]`           | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `["gradle"]`                         | Les dossiers qui activent ce module.                                                       |
| `style`                              | `"bold bright-cyan"`                 | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `gradle` module.                                                              |
| `recursive`                          | `false`                              | Enables recursive finding for the `gradle` directory.                                      |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v7.5.1` | The version of `gradle`                |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style*   |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `stack.yaml` file
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### Options

| Option                               | Défaut                               | Description                                        |
| ------------------------------------ | ------------------------------------ | -------------------------------------------------- |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                  |
| `symbole`                            | `'λ '`                               | A format string representing the symbol of Haskell |
| `detect_extensionsdetect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Les extensions qui déclenchent ce module.          |
| `detect_files`                       | `['stack.yaml', 'cabal.project']`    | Les fichiers qui activent ce module.               |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.               |
| `style`                              | `'bold purple'`                      | Le style pour le module.                           |
| `disabled`                           | `false`                              | Disables the `haskell` module.                     |

### Variables

| Variable       | Exemple     | Description                                                                             |
| -------------- | ----------- | --------------------------------------------------------------------------------------- |
| version        |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot       | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbole        |             | Reflète la valeur de l'option `symbol`                                                  |
| style\*      |             | Reflète la valeur de l'option `style`                                                   |

*: Cette variable peut uniquement être utilisée dans une chaine de style

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### Options

| Option                               | Défaut                                                                                          | Description                                                                                |
| ------------------------------------ | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `"via [$symbol($version )]($style)"`                                                            | Format du module.                                                                          |
| `version_format`                     | `"v${raw}"`                                                                                     | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensionsdetect_extensions` | `["hx", "hxml"]`                                                                                | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `["project.xml", "Project.xml", "application.xml", "haxelib.json", "hxformat.json", ".haxerc"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[".haxelib", "haxe_libraries"]`                                                                | Quels dossiers devraient activer ce module.                                                |
| `symbole`                            | `"⌘ "`                                                                                          | Une chaîne de format représentant le symbole de Helm.                                      |
| `style`                              | `"bold fg:202"`                                                                                 | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                                                                         | Disables the `haxe` module.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v4.2.5` | The version of `haxe`                  |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensionsdetect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['helmfile.yaml', 'Chart.yaml']`    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbole`                            | `'⎈ '`                               | A format string representing the symbol of Helm.                                           |
| `style`                              | `'bold white'`                       | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `helm` module.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | The version of `helm`                  |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Nom d'hôte

The `hostname` module shows the system hostname.

### Options

| Option       | Défaut                                 | Description                                                                                                                          |
| ------------ | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only`   | `true`                                 | Only show hostname when connected to an SSH session.                                                                                 |
| `ssh_symbol` | `'🌐 '`                                 | A format string representing the symbol when connected to SSH session.                                                               |
| `trim_at`    | `'.'`                                  | String that the hostname is cut off at, after the first match. `'.'` will stop after the first dot. `''` will disable any truncation |
| `format`     | `'[$ssh_symbol$hostname]($style) in '` | Format du module.                                                                                                                    |
| `style`      | `'bold dimmed green'`                  | Le style pour le module.                                                                                                             |
| `disabled`   | `false`                                | Disables the `hostname` module.                                                                                                      |

### Variables

| Variable   | Exemple    | Description                                           |
| ---------- | ---------- | ----------------------------------------------------- |
| hostname   | `computer` | The hostname of the computer                          |
| style\*  |            | Reflète la valeur de l'option `style`                 |
| ssh_symbol | `'🌏 '`     | The symbol to represent when connected to SSH session |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Options

| Option                               | Défaut                                                                                                   | Description                                                                                |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [${symbol}(${version} )]($style)'`                                                                 | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                                                                                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensionsdetect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                     | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                                                                                     | Quels dossiers devraient activer ce module.                                                |
| `symbole`                            | `'☕ '`                                                                                                   | A format string representing the symbol of Java                                            |
| `style`                              | `'red dimmed'`                                                                                           | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                                                                                  | Disables the `java` module.                                                                |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| version   | `v14`   | The version of `java`                  |
| symbole   |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

### Options

| Option             | Défaut                        | Description                                                              |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | Format du module.                                                        |
| `symbole`          | `'✦'`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `'bold blue'`                 | Le style pour le module.                                                 |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | The number of jobs                     |
| symbole   |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensionsdetect_extensions` | `['jl']`                             | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['Project.toml', 'Manifest.toml']`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbole`                            | `'ஃ '`                               | A format string representing the symbol of Julia.                                          |
| `style`                              | `'bold purple'`                      | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `julia` module.                                                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.4.0` | The version of `julia`                 |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `.kt` or a `.kts` file

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensionsdetect_extensions` | `['kt', 'kts']`                      | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbole`                            | `'🅺 '`                               | A format string representing the symbol of Kotlin.                                         |
| `style`                              | `'bold blue'`                        | Le style pour le module.                                                                   |
| `kotlin_binary`                      | `'kotlin'`                           | Configures the kotlin binary that Starship executes when getting the version.              |
| `disabled`                           | `false`                              | Disables the `kotlin` module.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`                |
| symbole   |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

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

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-context --namespace astronaut`. Similarly the user and cluster can be set with `kubectl config set-context starship-context --user starship-user` and `kubectl config set-context starship-context --cluster starship-cluster`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

When the module is enabled it will always be active, unless any of `detect_extensions`, `detect_files` or `detect_folders` have been set in which case the module will only be active in directories that match those conditions.

:::

### Options

| Option                               | Défaut                                               | Description                                                           |
| ------------------------------------ | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbole`                            | `'☸ '`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`                             | `'[$symbol$context( \($namespace\))]($style) in '` | Format du module.                                                     |
| `style`                              | `'cyan bold'`                                        | Le style pour le module.                                              |
| `context_aliases`                    | `{}`                                                 | Table of context aliases to display.                                  |
| `user_aliases`                       | `{}`                                                 | Table of user aliases to display.                                     |
| `detect_extensionsdetect_extensions` | `[]`                                                 | Les extensions qui déclenchent ce module.                             |
| `detect_files`                       | `[]`                                                 | Les fichiers qui activent ce module.                                  |
| `detect_folders`                     | `[]`                                                 | Quels dossiers devraient activer ce module.                           |
| `disabled`                           | `true`                                               | Disables the `kubernetes` module.                                     |

### Variables

| Variable  | Exemple              | Description                              |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbole   |                      | Reflète la valeur de l'option `symbol`   |
| style\* |                      | Reflète la valeur de l'option `style`    |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
'dev.local.cluster.k8s' = 'dev'
'.*/openshift-cluster/.*' = 'openshift'
'gke_.*_(?P<var_cluster>[\\w-]+)' = 'gke-$var_cluster'
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

#### Filtrage par regex

Additional to simple aliasing, `context_aliases` and `user_aliases` also supports extended matching and renaming using regular expressions.

The regular expression must match on the entire kube context, capture groups can be referenced using `$name` and `$N` in the replacement. This is more explained in the [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace) documentation.

Long and automatically generated cluster names can be identified and shortened using regular expressions:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
'.*/openshift-cluster/.*' = 'openshift'
# Or better, to rename every OpenShift cluster at once:
'.*/(?P<var_cluster>[\\w-]+)/.*' = '$var_cluster'

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
'gke_.*_(?P<var_cluster>[\\w-]+)' = 'gke-$var_cluster'
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Options

| Option     | Défaut  | Description                                                        |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### Exemple

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

The `localip` module shows the IPv4 address of the primary network interface.

### Options

| Option     | Défaut                    | Description                                            |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `'[$localipv4]($style) '` | Format du module.                                      |
| `style`    | `'bold yellow'`           | Le style pour le module.                               |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### Variables

| Variable  | Exemple      | Description                           |
| --------- | ------------ | ------------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address     |
| style\* |              | Reflète la valeur de l'option `style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🌙 '`                               | A format string representing the symbol of Lua.                                            |
| `detect_extensionsdetect_extensions` | `['lua']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['.lua-version']`                   | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `['lua']`                            | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold blue'`                        | Le style pour le module.                                                                   |
| `lua_binary`                         | `'lua'`                              | Configures the lua binary that Starship executes when getting the version.                 |
| `disabled`                           | `false`                              | Disables the `lua` module.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | The version of `lua`                   |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option      | Défaut                                          | Description                                              |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | Format du module.                                        |
| `symbole`   | `'🐏'`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `'bold dimmed white'`                           | Le style pour le module.                                 |
| `disabled`  | `true`                                          | Disables the `memory_usage` module.                      |

### Variables

| Variable         | Exemple       | Description                                                        |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbole          | `🐏`           | Reflète la valeur de l'option `symbol`                             |
| style\*        |               | Reflète la valeur de l'option `style`                              |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

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

| Option              | Défaut                             | Description                                                                               |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | Format du module.                                                                         |
| `symbole`           | `'⬢ '`                             | The symbol used before displaying the project name.                                       |
| `style`             | `'blue bold'`                      | Le style pour le module.                                                                  |
| `disabled`          | `false`                            | Disables the `meson` module.                                                              |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| project   | `starship` | The current Meson project name         |
| symbole   | `🐏`        | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Options

| Option              | Défaut                           | Description                                                                                  |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbole`           | `' '`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `'bold purple'`                  | Le style pour le module.                                                                     |
| `format`            | `'on [$symbol$branch]($style) '` | Format du module.                                                                            |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `'…'`                            | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué.                         |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| branch    | `master` | The active mercurial branch            |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | The format for the module                                                                  |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'👑 '`                               | The symbol used before displaying the version of Nim.                                      |
| `detect_extensionsdetect_extensions` | `['nim', 'nims', 'nimble']`          | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['nim.cfg']`                        | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold yellow'`                      | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `nim` module.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.2.0` | The version of `nimc`                  |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. The module will be shown when inside a nix-shell environment.

### Options

| Option       | Défaut                                         | Description                                           |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | Format du module.                                     |
| `symbole`    | `'❄️ '`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `'bold blue'`                                  | Le style pour le module.                              |
| `impure_msg` | `'impure'`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `'pure'`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| state     | `pure`  | The state of the nix-shell             |
| name      | `lorri` | The name of the nix-shell              |
| symbole   |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = '[impure shell](bold red)'
pure_msg = '[pure shell](bold green)'
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

### Options

| Option                               | Défaut                                     | Description                                                                                           |
| ------------------------------------ | ------------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`                             | `'via [$symbol($version )]($style)'`       | Format du module.                                                                                     |
| `version_format`                     | `'v${raw}'`                                | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch`            |
| `symbole`                            | `' '`                                     | A format string representing the symbol of Node.js.                                                   |
| `detect_extensionsdetect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']` | Les extensions qui déclenchent ce module.                                                             |
| `detect_files`                       | `['package.json', '.node-version']`        | Les fichiers qui activent ce module.                                                                  |
| `detect_folders`                     | `['node_modules']`                         | Les dossiers qui activent ce module.                                                                  |
| `style`                              | `'bold green'`                             | Le style pour le module.                                                                              |
| `disabled`                           | `false`                                    | Disables the `nodejs` module.                                                                         |
| `not_capable_style`                  | `bold red`                                 | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | The version of `node`                  |
| symbole   |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Options

| Option                               | Défaut                                                                     | Description                                                                                |
| ------------------------------------ | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | La chaîne de format pour le module.                                                        |
| `version_format`                     | `'v${raw}'`                                                                | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🐫 '`                                                                     | The symbol used before displaying the version of OCaml.                                    |
| `global_switch_indicator`            | `''`                                                                       | The format string used to represent global OPAM switch.                                    |
| `local_switch_indicator`             | `'*'`                                                                      | The format string used to represent local OPAM switch.                                     |
| `detect_extensionsdetect_extensions` | `['opam', 'ml', 'mli', 're', 'rei']`                                       | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `['_opam', 'esy.lock']`                                                    | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold yellow'`                                                            | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                                                    | Disables the `ocaml` module.                                                               |

### Variables

| Variable         | Exemple      | Description                                                       |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbole          |              | Reflète la valeur de l'option `symbol`                            |
| style\*        |              | Reflète la valeur de l'option `style`                             |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool. By default the module will be shown if the current directory contains a `.rego` file.

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🪖  '`                              | A format string representing the symbol of OPA.                                            |
| `detect_extensionsdetect_extensions` | `['rego']`                           | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold blue'`                        | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `opa` module.                                                                 |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.44.0` | The version of `opa`                   |
| symbole   |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Options

| Option     | Défaut                                          | Description                                                    |
| ---------- | ----------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | Format du module.                                              |
| `symbole`  | `'☁️ '`                                         | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `'bold yellow'`                                 | Le style pour le module.                                       |
| `disabled` | `false`                                         | Disables the `openstack` module.                               |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | The current OpenStack cloud            |
| project   | `dev`   | The current OpenStack project          |
| symbole   |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option     | Défaut                | Description                                            |
| ---------- | --------------------- | ------------------------------------------------------ |
| `format`   | `"[$symbol]($style)"` | Format du module.                                      |
| `style`    | `"bold white"`        | Le style pour le module.                               |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type. Operating system types not defined by your configuration use the default symbols table below. All operating systems currently supported by the module are listed below. If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
[os.symbols]
Alpine = "🏔️ "
Amazon = "🙂 "
Android = "🤖 "
Arch = "🎗️ "
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
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
OpenBSD = "🐡 "
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

| Variable  | Exemple      | Description                                                        |
| --------- | ------------ | ------------------------------------------------------------------ |
| symbole   | `🎗️`         | The current operating system symbol from advanced option `symbols` |
| name      | `Arch Linux` | The current operating system name                                  |
| type      | `Arch`       | The current operating system type                                  |
| codename  |              | The current operating system codename, if applicable               |
| edition   |              | The current operating system edition, if applicable                |
| version   |              | The current operating system version, if applicable                |
| style\* |              | Reflète la valeur de l'option `style`                              |

*: Cette variable peut uniquement être utilisée dans une chaine de style

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

> ⚠️ La version montrée est celle du paquet dont le code source est dans votre dossier courant, pas votre gestionnaire de paquet.

### Options

| Option            | Défaut                            | Description                                                                                |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`          | `'is [$symbol$version]($style) '` | Format du module.                                                                          |
| `symbole`         | `'📦 '`                            | The symbol used before displaying the version the package.                                 |
| `version_format`  | `'v${raw}'`                       | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | Le style pour le module.                                                                   |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.                                  |
| `disabled`        | `false`                           | Disables the `package` module.                                                             |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.0.0` | The version of your package            |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Options

| Option                               | Défaut                                                                                                   | Description                                                                                |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'`                                                                     | La chaîne de format pour le module.                                                        |
| `version_format`                     | `'v${raw}'`                                                                                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🐪 '`                                                                                                   | The symbol used before displaying the version of Perl                                      |
| `detect_extensionsdetect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                                                                                     | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold 149'`                                                                                             | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                                                                                  | Disables the `perl` module.                                                                |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v5.26.1` | The version of `perl`                  |
| symbole   |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🐘 '`                               | The symbol used before displaying the version of PHP.                                      |
| `detect_extensionsdetect_extensions` | `['php']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['composer.json', '.php-version']`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'147 bold'`                         | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `php` module.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v7.3.8` | The version of `php`                   |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

::: tip

By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). If you still want to enable it, [follow the example shown below](#with-pulumi-version).

:::

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Options

| Option           | Défaut                                       | Description                                                                                |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | La chaîne de format pour le module.                                                        |
| `version_format` | `'v${raw}'`                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`        | `' '`                                       | A format string shown before the Pulumi stack.                                             |
| `style`          | `'bold 5'`                                   | Le style pour le module.                                                                   |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.                             |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                                              |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `pulumi`                |
| stack     | `dev`      | The current Pulumi stack               |
| username  | `alice`    | The current Pulumi username            |
| symbole   |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

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

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `spago.dhall` file
- The current directory contains a file with the `.purs` extension

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'<=> '`                       | The symbol used before displaying the version of PureScript.                               |
| `detect_extensionsdetect_extensions` | `['purs']`                           | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['spago.dhall']`                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold white'`                       | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `purescript` module.                                                          |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | The version of `purescript`            |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- A virtual environment is currently activated

### Options

| Option                               | Défaut                                                                                                       | Description                                                                                |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                                                                                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🐍 '`                                                                                                       | A format string representing the symbol of Python                                          |
| `style`                              | `'yellow bold'`                                                                                              | Le style pour le module.                                                                   |
| `pyenv_version_name`                 | `false`                                                                                                      | Use pyenv to get Python version                                                            |
| `pyenv_prefix`                       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                            |
| `python_binary`                      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should executes when getting the version.     |
| `detect_extensionsdetect_extensions` | `['py']`                                                                                                     | Les extensions qui déclenchent ce module                                                   |
| `detect_files`                       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`                     | `[]`                                                                                                         | Quels dossiers devraient activer ce module                                                 |
| `disabled`                           | `false`                                                                                                      | Disables the `python` module.                                                              |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `'python3'` will hide any Python version 2, see example below.

:::

### Variables

| Variable     | Exemple         | Description                                |
| ------------ | --------------- | ------------------------------------------ |
| version      | `'v3.8.1'`      | The version of `python`                    |
| symbole      | `'🐍 '`          | Reflète la valeur de l'option `symbol`     |
| style        | `'yellow bold'` | Reflète la valeur de l'option `style`      |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `'venv'`        | The current `virtualenv` name              |

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
# N'utilisez que le binaire `python3` pour obtenir la version.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# Ne pas déclencher pour les fichiers avec l'extension py
detect_extensions = []
```

```toml
# ~/.config/starship.toml

[python]
# Affiche la version de python depuis l'intérieur d'un venv local.
#
# Notez que cela ne fonctionnera que lorsque le venv est à l'intérieur du projet,
# et uniquement lorsque vous vous situez dans le répertoire contenant le dossier du venv
# mais peut-être que c'est suffisant?
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

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'📐'`                                | A format string representing the symbol of R.                                              |
| `style`                              | `'blue bold'`                        | Le style pour le module.                                                                   |
| `detect_extensionsdetect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Les extensions qui déclenchent ce module                                                   |
| `detect_files`                       | `['.Rprofile']`                      | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`                     | `['.Rproj.user']`                    | Quels dossiers devraient activer ce module                                                 |
| `disabled`                           | `false`                              | Disables the `r` module.                                                                   |

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

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Options

| Option                               | Défaut                                           | Description                                                                                |
| ------------------------------------ | ------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version-$vm_version )]($style)'` | La chaîne de format pour le module.                                                        |
| `version_format`                     | `'v${raw}'`                                      | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🦋 '`                                           | The symbol used before displaying the version of Raku                                      |
| `detect_extensionsdetect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['META6.json']`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                             | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold 149'`                                     | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                          | Disables the `raku` module.                                                                |

### Variables

| Variable   | Exemple | Description                            |
| ---------- | ------- | -------------------------------------- |
| version    | `v6.d`  | The version of `raku`                  |
| vm_version | `moar`  | The version of VM `raku` is built on   |
| symbole    |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.red` or `.reds` extension

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🔺 '`                               | A format string representing the symbol of Red.                                            |
| `detect_extensionsdetect_extensions` | `['red']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'red bold'`                         | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `red` module.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `red`                   |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'💎 '`                               | A format string representing the symbol of Ruby.                                           |
| `detect_extensionsdetect_extensions` | `['rb']`                             | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['Gemfile', '.ruby-version']`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `detect_variables`                   | `['RUBY_VERSION', 'RBENV_VERSION']`  | Which environment variables should trigger this module.                                    |
| `style`                              | `'bold red'`                         | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `ruby` module.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `ruby`                  |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🦀 '`                               | A format string representing the symbol of Rust                                            |
| `detect_extensionsdetect_extensions` | `['rs']`                             | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['Cargo.toml']`                     | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold red'`                         | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `rust` module.                                                                |

### Variables

| Variable  | Exemple           | Description                                  |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbole   |                   | Reflète la valeur de l'option `symbol`       |
| style\* |                   | Reflète la valeur de l'option `style`        |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Options

| Option                               | Défaut                                   | Description                                                                                |
| ------------------------------------ | ---------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [${symbol}(${version} )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensionsdetect_extensions` | `['sbt', 'scala']`                       | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `['.metals']`                            | Quels dossiers devraient activer ce module.                                                |
| `symbole`                            | `'🆂 '`                                   | A format string representing the symbol of Scala.                                          |
| `style`                              | `'red dimmed'`                           | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                  | Disables the `scala` module.                                                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `2.13.5` | The version of `scala`                 |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option                 | Défaut                    | Description                                                  |
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
| `format`               | `'[$indicator]($style) '` | Format du module.                                            |
| `style`                | `'white bold'`            | Le style pour le module.                                     |
| `disabled`             | `true`                    | Disables the `shell` module.                                 |

### Variables

| Variable  | Défaut | Description                                                |
| --------- | ------ | ---------------------------------------------------------- |
| indicator |        | Mirrors the value of `indicator` for currently used shell. |
| style\* |        | Mirrors the value of option `style`.                       |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemples

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

### Options

| Option      | Défaut                       | Description                                                   |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `'[$symbol$shlvl]($style) '` | Format du module.                                             |
| `symbole`   | `'↕️  '`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `'bold yellow'`              | Le style pour le module.                                      |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | The current value of `SHLVL`           |
| symbole   |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### Options

| Option     | Défaut                           | Description                                      |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Format du module.                                |
| `symbole`  | `''`                             | A format string displayed before the image name. |
| `style`    | `'bold dimmed blue'`             | Le style pour le module.                         |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

| Variable  | Exemple      | Description                            |
| --------- | ------------ | -------------------------------------- |
| env       | `centos.img` | The current Singularity image          |
| symbole   |              | Reflète la valeur de l'option `symbol` |
| style\* |              | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Options

| Option              | Défaut                                 | Description                                                                                                                                                   |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to. `0` ne signifie pas de troncature. Regardez aussi le module [`directory`](#directory). |
| `symbole`           | `'🅢  '`                                | Le symbole utilisé avant le nom d'environnement.                                                                                                              |
| `style`             | `'bold blue'`                          | Le style pour le module.                                                                                                                                      |
| `format`            | `'via [$symbol$environment]($style) '` | Format du module.                                                                                                                                             |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                                  |

### Variables

| Variable    | Exemple      | Description                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | The current spack environment          |
| symbole     |              | Reflète la valeur de l'option `symbol` |
| style\*   |              | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Status

The `status` module displays the exit code of the previous command. If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`. The status code will cast to a signed 32-bit integer.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option                      | Défaut                                                                             | Description                                                           |
| --------------------------- | ---------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                      | Le format du module                                                   |
| `symbole`                   | `'❌'`                                                                              | The symbol displayed on program error                                 |
| `success_symbol`            | `''`                                                                               | The symbol displayed on program success                               |
| `not_executable_symbol`     | `'🚫'`                                                                              | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `'🔍'`                                                                              | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `'🧱'`                                                                              | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `'⚡'`                                                                              | The symbol displayed on any signal                                    |
| `style`                     | `'bold red'`                                                                       | Le style pour le module.                                              |
| `recognize_signal_code`     | `true`                                                                             | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                            | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                            | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                          | The symbol used to separate pipestatus segments (supports formatting) |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style)'` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                    | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                             | Disables the `status` module.                                         |

### Variables

| Variable       | Exemple | Description                                                                                 |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                           |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                    |
| int            | `127`   | The exit code of the last command                                                           |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                         |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                |
| pipestatus     |         | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbole        |         | Reflète la valeur de l'option `symbol`                                                      |
| style\*      |         | Reflète la valeur de l'option `style`                                                       |

*: Cette variable peut uniquement être utilisée dans une chaine de style

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

The `sudo` module displays if sudo credentials are currently cached. The module will only be shown if credentials are cached.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option          | Défaut                   | Description                                             |
| --------------- | ------------------------ | ------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | Le format du module                                     |
| `symbole`       | `'🧙 '`                   | The symbol displayed when credentials are cached        |
| `style`         | `'bold blue'`            | Le style pour le module.                                |
| `allow_windows` | `false`                  | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                   | Disables the `sudo` module.                             |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| symbole   |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

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

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'🐦 '`                               | A format string representing the symbol of Swift                                           |
| `detect_extensionsdetect_extensions` | `['swift']`                          | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['Package.swift']`                  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold 202'`                         | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `swift` module.                                                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.2.4` | The version of `swift`                 |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol$workspace]($style) '` | La chaîne de format pour le module.                                                        |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'💠'`                                | A format string shown before the terraform workspace.                                      |
| `detect_extensionsdetect_extensions` | `['tf', 'tfplan', 'tfstate']`        | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `['.terraform']`                     | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'bold 105'`                         | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `terraform` module.                                                           |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `terraform`             |
| workspace | `default`  | The current Terraform workspace        |
| symbole   |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

#### Avec la version de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $version$workspace]($style) '
```

#### Sans la version de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $workspace]($style) '
```

## Date et Heure

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option            | Défaut                  | Description                                                                                                                        |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | La chaîne de format pour le module.                                                                                                |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `'bold yellow'`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `'local'`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `'-'`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable  | Exemple    | Description                           |
| --------- | ---------- | ------------------------------------- |
| time      | `13:08:10` | The current time.                     |
| style\* |            | Reflète la valeur de l'option `style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

### Options

| Option        | Défaut                  | Description                                 |
| ------------- | ----------------------- | ------------------------------------------- |
| `style_root`  | `'bold red'`            | The style used when the user is root/admin. |
| `style_user`  | `'bold yellow'`         | The style used for non-root users.          |
| `format`      | `'[$user]($style) in '` | Format du module.                           |
| `show_always` | `false`                 | Always shows the `username` module.         |
| `disabled`    | `false`                 | Disables the `username` module.             |

### Variables

| Variable | Exemple      | Description                                                                                 |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `'matchai'`  | The currently logged-in user ID.                                                            |

### Exemple

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

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a `Vagrantfile` file

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'⍱ '`                               | A format string representing the symbol of Vagrant.                                        |
| `detect_extensionsdetect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['Vagrantfile']`                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'cyan bold'`                        | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `vagrant` module.                                                             |

### Variables

| Variable  | Exemple          | Description                            |
| --------- | ---------------- | -------------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`               |
| symbole   |                  | Reflète la valeur de l'option `symbol` |
| style\* |                  | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Options

| Option                               | Défaut                                       | Description                                                                                |
| ------------------------------------ | -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'`         | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'V '`                                       | A format string representing the symbol of V                                               |
| `detect_extensionsdetect_extensions` | `['v']`                                      | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                         | Les dossiers qui activent ce module.                                                       |
| `style`                              | `'blue bold'`                                | Le style pour le module.                                                                   |
| `disabled`                           | `false`                                      | Disables the `vlang` module.                                                               |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| version   | `v0.2`  | The version of `v`                     |
| symbole   |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### Options

| Option     | Défaut                           | Description                                            |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbole`  | `''`                             | The symbol used before displaying the repository name. |
| `style`    | `'bold yellow'`                  | Le style pour le module.                               |
| `format`   | `'vcsh [$symbol$repo]($style) '` | Format du module.                                      |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variables

| Variable  | Exemple                                     | Description                            |
| --------- | ------------------------------------------- | -------------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name             |
| symbole   |                                             | Reflète la valeur de l'option `symbol` |
| style\* | `black bold dimmed`                         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### Options

| Option                               | Défaut                               | Description                                                                                |
| ------------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                             | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`                     | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                            | `'↯ '`                               | The symbol used before displaying the version of Zig.                                      |
| `style`                              | `'bold yellow'`                      | Le style pour le module.                                                                   |
| `disabled`                           | `false`                              | Disables the `zig` module.                                                                 |
| `detect_extensionsdetect_extensions` | `['zig']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`                       | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`                     | `[]`                                 | Les dossiers qui activent ce module.                                                       |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.6.0` | The version of `zig`                   |
| symbole   |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

### Options

| Option                               | Défaut                          | Description                                                                                                                                                                                                                                                                                   |
| ------------------------------------ | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`                            | `''`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                                                                                                                                 |
| `when`                               | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the command returns a `0` status code.                                                                          |
| `shell`                              |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                            |
| `description`                        | `'<custom module>'`       | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                  |
| `detect_files`                       | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                         |
| `detect_folders`                     | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                   |
| `detect_extensionsdetect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                    |
| `symbole`                            | `''`                            | The symbol used before displaying the command output.                                                                                                                                                                                                                                         |
| `style`                              | `'bold green'`                  | Le style pour le module.                                                                                                                                                                                                                                                                      |
| `format`                             | `'[$symbol($output )]($style)'` | Format du module.                                                                                                                                                                                                                                                                             |
| `disabled`                           | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                |
| `os`                                 |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                 |
| `use_stdin`                          |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`                     | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                     |

### Variables

| Variable  | Description                            |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbole   | Reflète la valeur de l'option `symbol` |
| style\* | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

#### Commandes shell personnalisées

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
