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

# Remplacer le symbole '❯' dans l'invite par '➜'
[character] # Le nom du module que nous configurons est 'character'
success_symbol = '[➜](bold green)' # Le segment 'success_symbol' est défini à '➜' avec la couleur 'bold green'

# Désactiver le module package, le masquant complètement de l'invite
[package]
disabled = true
```

### Emplacement du fichier de configuration

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

Par défaut, starship enregistre les avertissements et les erreurs dans un fichier nommé `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, où la clé de session correspond à une instance de votre terminal. Ceci peut cependant être modifié en utilisant la variable d'environnement `STARSHIP_CACHE` :

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

Les symboles de syntaxe Starship suivants ont une utilisation spéciale dans une chaine de formatage et doivent être échappés pour être affichés littéralement : `$ [ ] ( )`.

| Symbol | Type                         | Notes                                                                                 |
| ------ | ---------------------------- | ------------------------------------------------------------------------------------- |
| `'`    | chaîne littérale             | moins d'échappement                                                                   |
| `"`    | chaîne                       | plus d'échappement                                                                    |
| `'''`  | chaîne littérale multi-ligne | moins d'échappement                                                                   |
| `"""`  | chaîne multi-ligne           | plus d'échappement, les retours à la ligne dans les déclarations peuvent être ignorés |

Par exemple :

```toml
# literal string
format = '☺\☻ '

# regular string
format = "☺\\☻ "

# escaping Starship symbols
format = '\[\$\] '
```

Lors de l'utilisation de retours à la ligne, des déclarations multi-lignes peuvent être utilisées. Par exemple, si vous voulez afficher un symbole `$` sur une nouvelle ligne, les valeurs suivantes pour `format` sont équivalentes :

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

Dans les chaînes multi-lignes basiques, les retours à la ligne peuvent être utilisés pour le formatage sans être présents dans la valeur en les échappant.

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

La plupart des modules de Starship vous permettent de configurer leurs styles d'affichage. Cela se fait avec une entrée (généralement appelée `style`) qui est une chaîne de caractères spécifiant la configuration. Voici quelques exemples de chaînes de style avec ce qu'elles font. For details on the full syntax, consult the [advanced config guide](../advanced-config/).

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

- `'(@$region)'` n'affichera rien si la variable `region` est `None` ou une chaîne vide, sinon `@` suivi de la valeur de region.
- `'(some text)'` n'affichera toujours rien puisqu'il n'y a pas de variables entre les accolades.
- Quand `$combined` est un raccourci pour `\[$a$b\]`, `'($combined)'` n'affichera rien uniquement si `$a` et `$b` sont tous deux `None`. Cela fonctionne de la même manière que `'(\[$a$b\] )'`.

### Negative matching

De nombreux modules ont des variables `detect_extensions`, `detect_files` et `detect_folders`. Celles-ci prennent des listes de chaînes à correspondre ou non. Les options "négatives", celles qui ne doivent pas correspondre, sont indiquées par un caractère `!` en tête. La présence de _n'importe quel_ indicateur négatif dans le répertoire entraînera la non-correspondance du module.

Les extensions sont comparées à la fois aux caractères après le dernier point dans un nom de fichier, et aux caractères après le premier point dans un nom de fichier. Par exemple, `foo.bar.tar.gz` sera comparé à `bar.tar.gz` et `gz` dans la variable `detect_extensions`. Les fichiers dont le nom commence par un point ne sont pas considérés comme ayant des extensions.

Pour voir comment cela fonctionne en pratique, vous pourriez faire correspondre TypeScript mais pas les fichiers MPEG Transport Stream ainsi :

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Invite

Voici la liste des options de configuration globales de l'invite de commandes.

### Options

| Option            | Défaut                         | Description                                                                                                                                                                                                                                  |
| ----------------- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [lien](#default-prompt-format) | Configure le format de l'invite.                                                                                                                                                                                                             |
| `right_format`    | `''`                           | See [Enable Right Prompt](../advanced-config/#enable-right-prompt)                                                                                                                                                                           |
| `scan_timeout`    | `30`                           | Délai maximal pour le scan des fichiers par starship (en millisecondes).                                                                                                                                                                     |
| `command_timeout` | `500`                          | Délai maximal pour les commandes exécutées par starship (en millisecondes).                                                                                                                                                                  |
| `add_newline`     | `true`                         | Insère une ligne vide entre les invites du shell.                                                                                                                                                                                            |
| `palette`         | `''`                           | Définit quelle palette de couleurs de `palettes` utiliser.                                                                                                                                                                                   |
| `palettes`        | `{}`                           | Collection de palettes de couleurs qui assignent des [couleurs](../advanced-config/#style-strings) à des noms définis par l'utilisateur. Notez que les palettes de couleurs ne peuvent pas référencer leurs propres définitions de couleurs. |
| `follow_symlinks` | `true`                         | Suit les liens symboliques pour vérifier s'ils sont des répertoires ; utilisé dans des modules tels que git.                                                                                                                                 |

> [!TIP] Si vous avez des liens symboliques vers des systèmes de fichiers en réseau, envisagez de définir `follow_symlinks` à `false`.

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
$fortran\
$gleam\
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
$nats\
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
$os\
$container\
$netns\
$shell\
$character"""
```

Si vous voulez étendre le format par défaut, pour pouvoir utiliser `$all` ; les modules que vous ajouter explicitement au format ne seront pas dupliqués. Par ex.

```toml
# Move the directory to the second line
format = '$all$directory$character'
```

## AWS

Le module `aws` affiche la région et le profil AWS courants ainsi qu'un minuteur d'expiration lors de l'utilisation d'identifiants temporaires. La sortie du module utilise les variables d'environnement `AWS_REGION`, `AWS_DEFAULT_REGION` et `AWS_PROFILE` ainsi que les fichiers `~/.aws/config` et `~/.aws/credentials` selon les besoins.

Le module n'affichera un profil que si ses identifiants sont présents dans `~/.aws/credentials` ou si un `credential_process`, `sso_start_url` ou `sso_session` sont définis dans `~/.aws/config`. Alternativement, avoir l'une des variables d'environnement `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` ou `AWS_SESSION_TOKEN` définie suffira également. Si l'option `force_display` est définie à `true`, toutes les informations disponibles seront affichées même si aucun identifiant n'est détecté selon les conditions ci-dessus.

Lorsque vous utilisez [aws-vault](https://github.com/99designs/aws-vault) le profil est lu à partir de la variable d'environnement `AWS_VAULT` et la date d'expiration des identifiants est lue à partir de la variable d'environnement `AWS_SESSION_EXPIRATION`.

Lorsque vous utilisez [awsu](https://github.com/kreuzwerker/awsu) le profil est lu depuis la variable d'environnement `AWSU_PROFILE`.

Lorsque vous utilisez [AWSume](https://awsu.me) le profil est lu à partir de la variable d'environnement `AWSUME_PROFILE` et la date d'expiration des identifiants est lue à partir de la variable d'environnement `AWSUME_EXPIRATION`.

Lorsque vous utilisez [saml2aws](https://github.com/Versent/saml2aws), les informations d'expiration obtenues depuis `~/.aws/credentials` se rabattent sur la clé `x_security_token_expires`.

Lorsque vous utilisez [aws-sso-cli](https://github.com/synfinatic/aws-sso-cli), le profil est lu depuis la variable d'environnement `AWS_SSO_PROFILE`.

### Options

| Option              | Défaut                                                            | Description                                                                                                                   |
| ------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | Format du module.                                                                                                             |
| `symbole`           | `'☁️ '`                                                           | Le symbole est affiché avant le profil AWS actuel.                                                                            |
| `region_aliases`    | `{}`                                                              | Tableau des alias de région à afficher en plus du nom AWS.                                                                    |
| `profile_aliases`   | `{}`                                                              | Tableau des alias de profil à afficher en plus du nom AWS.                                                                    |
| `style`             | `'bold yellow'`                                                   | Le style pour le module.                                                                                                      |
| `expiration_symbol` | `'X'`                                                             | Le symbole est affiché lorsque les identifiants temporaires ont expiré.                                                       |
| `disabled`          | `false`                                                           | Désactive le module `AWS`.                                                                                                    |
| `force_display`     | `false`                                                           | Si `true`, affiche les informations même si `credentials`, `credential_process` ou `sso_start_url` n'ont pas été configurées. |

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

Le module `azure` affiche l'abonnement Azure actuel. Ceci est basé sur l'affichage du nom de l'abonnement par défaut ou du nom d'utilisateur, tel que défini dans le fichier `~/.azure/azureProfile.json`.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Variable               | Défaut                                   | Description                                                                      |
| ---------------------- | ---------------------------------------- | -------------------------------------------------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | Le format pour le rendu du module Azure.                                         |
| `symbole`              | `'󰠅 '`                                   | Le symbole utilisé dans le format.                                               |
| `style`                | `'blue bold'`                            | Le style utilisé dans le format.                                                 |
| `disabled`             | `true`                                   | Désactive le module `azure`.                                                     |
| `subscription_aliases` | `{}`                                     | Table d'alias de noms d'abonnement à afficher en plus du nom d'abonnement Azure. |

### Exemples

#### Afficher le nom de l'abonnement

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = '󰠅 '
style = 'blue bold'
```

#### Afficher le nom d'utilisateur

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "󰠅 "
style = "blue bold"
```

#### Afficher l'alias du nom de l'abonnement

```toml
# ~/.config/starship.toml

[azure.subscription_aliases]
very-long-subscription-name = 'vlsn'
```

## Battery

Le module `battery` montre à quel point la batterie de l'appareil est chargée et son état de charge actuel. Ce module n'est visible que lorsque la batterie de l'appareil est inférieure à 10%.

### Options

| Option               | Défaut                            | Description                                                   |
| -------------------- | --------------------------------- | ------------------------------------------------------------- |
| `full_symbol`        | `'󰁹 '`                            | Le symbole affiché lorsque la batterie est pleine.            |
| `charging_symbol`    | `'󰂄 '`                            | Le symbole affiché lorsque la batterie se charge.             |
| `discharging_symbol` | `'󰂃 '`                            | Le symbole affiché lorsque la batterie se décharge.           |
| `unknown_symbol`     | `'󰁽 '`                            | Le symbole affiché lorsque l'état de la batterie est inconnu. |
| `empty_symbol`       | `'󰂎 '`                            | Le symbole affiché lorsque la batterie est vide.              |
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
discharging_symbol = '💦 '

# when capacity is over 30%, the battery indicator will not be displayed
```

## Buf

Le module `buf` affiche la version de [Buf](https://buf.build) installée. Par défaut, le module est affiché si le répertoire courant contient un fichier de configuration [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml) ou [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml).

### Options

| Option              | Défaut                                          | Description                                            |
| ------------------- | ----------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'with [$symbol($version )]($style)'`           | Le format du module `buf`.                             |
| `version_format`    | `'v${raw}'`                                     | Le format de la version.                               |
| `symbole`           | `'🐃 '`                                         | Le symbole utilisé avant d’afficher la version de Buf. |
| `detect_extensions` | `[]`                                            | Les extensions qui déclenchent ce module.              |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Les fichiers qui activent ce module.                   |
| `detect_folders`    | `[]`                                            | Quels dossiers devraient activer ce module.            |
| `style`             | `'bold blue'`                                   | Le style pour le module.                               |
| `disabled`          | `false`                                         | Désactive le module `elixir`.                          |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| `version` | `v1.0.0` | La version de `buf`                    |
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

Le module `bun` affiche la version actuellement installée du runtime JavaScript [bun](https://bun.sh). Par défaut, le module sera affiché si l’une de ces conditions est remplie :

- Le dossier courant contient un fichier `bun.lock`
- Le dossier courant contient un fichier `bun.lockb`
- Le dossier courant contient un fichier `bunfig.toml`

### Options

| Option              | Défaut                                     | Description                                                                                |
| ------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`       | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                                | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🥟 '`                                    | Une chaîne de format représentant le symbole de Bun.                                       |
| `detect_extensions` | `[]`                                       | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                       | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold red'`                               | Le style pour le module.                                                                   |
| `disabled`          | `false`                                    | Désactive le module `bun`.                                                                 |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v0.1.4` | La version de `bun`                    |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

#### Personnaliser le format

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

Le module `c` affiche des informations à propos de votre compilateur C. Par défaut, ce module sera affiché si le dossier courant contient un fichier `.c` ou `.h`.

### Options

| Option              | Défaut                                                                        | Description                                                                                |
| ------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | La chaîne de format pour le module.                                                        |
| `version_format`    | `'v${raw}'`                                                                   | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'C '`                                                                        | Le symbole utilisé avant d’afficher les détails du compilateur                             |
| `detect_extensions` | `['c', 'h']`                                                                  | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                                                          | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                          | Les dossiers qui activent ce module.                                                       |
| `commands`          | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | Comment détecter quel est le compilateur                                                   |
| `style`             | `'bold 149'`                                                                  | Le style pour le module.                                                                   |
| `disabled`          | `false`                                                                       | Désactive le module `c`.                                                                   |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| name     | clang   | Le nom du compilateur                  |
| version  | 13.0.0  | La version du compilateur              |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style    |         | Reflète la valeur de l'option `style`  |

### Commandes

L’option `commands` accepte une liste de commandes pour déterminer la version du compilateur et son nom.

Chaque commande est représentée par une liste contenant le nom de l'exécutable, suivi de ses arguments, généralement quelque chose comme `['mycc', '--version']`. Starship essayera d'exécuter chaque commande jusqu'à obtenir un résultat sur STDOUT.

Si un compilateur C n'est pas supporté par ce module, vous pouvez le demander en [ouvrant une issue sur GitHub](https://github.com/starship/starship/issues/new/choose).

### Exemple

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## CPP

Le module `cpp` affiche des informations sur votre compilateur `C++`. Par défaut, le module sera affiché si le dossier courant contient un fichier `.cpp`, `.hpp` ou d'autres fichiers liés au `C++`.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option              | Défaut                                                                           | Description                                                                                |
| ------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | La chaîne de format pour le module.                                                        |
| `version_format`    | `'v${raw}'`                                                                      | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'C++ '`                                                                         | Le symbole utilisé avant d’afficher les détails du compilateur                             |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                                                             | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                             | Les dossiers qui activent ce module.                                                       |
| `commands`          | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | Comment détecter quel est le compilateur                                                   |
| `style`             | `'bold 149'`                                                                     | Le style pour le module.                                                                   |
| `disabled`          | `true`                                                                           | Désactive le module `cpp`.                                                                 |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| name     | clang++ | Le nom du compilateur                  |
| version  | 13.0.0  | La version du compilateur              |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style    |         | Reflète la valeur de l'option `style`  |

### Commandes

L’option `commands` accepte une liste de commandes pour déterminer la version du compilateur et son nom.

Chaque commande est représentée par une liste contenant le nom de l'exécutable, suivi de ses arguments, généralement quelque chose comme `['mycpp', '--version']`. Starship essayera d'exécuter chaque commande jusqu'à obtenir un résultat sur STDOUT.

Si un compilateur C++ n'est pas supporté par ce module, vous pouvez le demander en [ouvrant une issue sur GitHub](https://github.com/starship/starship/issues/new/choose).

### Exemple

```toml
# ~/.config/starship.toml

[cpp]
disabled = false
format = 'via [$name $version]($style)'
```

## Caractère

Le module `character` affiche un caractère (en général une flèche) à côté de là où vous entrez le texte dans votre terminal.

Le caractère vous dira si la dernière commande a été réussie ou pas. Il peut faire ça de deux façons:

- en changeant de couleur(`red`/`green`)
- en changeant de forme (`❯`/`✖`)

Par défaut, il ne change que de couleur. Si vous désirez également changer sa forme, jetez un à [cet exemple](#with-custom-error-shape).

> [!WARNING] `vimcmd_symbol` n'est supporté que dans cmd, fish et zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol` et `vimcmd_visual_symbol` ne sont supportés que dans fish en raison de [problèmes en amont avec la détection de mode dans zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

### Options

| Option                      | Défaut               | Description                                                                                     |
| --------------------------- | -------------------- | ----------------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | Le format utilisé avant l'entrée de texte.                                                      |
| `success_symbol`            | `'[❯](bold green)'`  | Le format utilisé avant l'entrée de texte si la commande précédente a réussi.                   |
| `error_symbol`              | `'[❯](bold red)'`    | Le format utilisé avant l'entrée de texte si la commande précédente a échoué.                   |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | Le format utilisé avant l'entrée de texte si le shell est en mode vim normal.                   |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | La chaîne de format utilisée avant l'entrée de texte si le shell est en mode vim `replace_one`. |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | La chaîne de format utilisée avant l'entrée de texte si le shell est en mode vim remplacement.  |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | La chaîne de format utilisée avant l'entrée de texte si le shell est en mode vim visuel.        |
| `disabled`                  | `false`              | Désactive le module `character`.                                                                |

### Variables

| Variable | Exemple | Description                                                                                        |
| -------- | ------- | -------------------------------------------------------------------------------------------------- |
| symbole  |         | Un reflet de `success_symbol`, `error_symbol`, `vimcmd_symbol` ou `vimcmd_replace_one_symbol` etc. |

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

Le module `cmake` affiche la version de [CMake](https://cmake.org/) installée. Par défaut, le module s’activera si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `CMakeLists.txt`
- Le dossier courant contient un fichier `CMakeCache.txt`

### Options

| Option              | Défaut                                 | Description                                                                                |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`   | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                            | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'△ '`                                 | Le symbole utilisé avant la version de cmake.                                              |
| `detect_extensions` | `[]`                                   | Les extensions qui déclenchent ce module                                                   |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`    | `[]`                                   | Quels dossiers devraient activer ce module                                                 |
| `style`             | `'bold blue'`                          | Le style pour le module.                                                                   |
| `disabled`          | `false`                                | Désactive le module `cmake`.                                                               |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v3.17.3` | La version de cmake                    |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## COBOL / GNUCOBOL

Le module `cobol` affiche la version de COBOL installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier finissant par `.cob` ou `.COB`
- Le dossier courant contiens un fichier finissant par `.cbl` ou `.CBL`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `symbole`           | `'⚙️ '`                              | Le symbole utilisé avant d’afficher la version de COBOL.                                   |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | Le style pour le module.                                                                   |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`          | `false`                              | Désactive le module `cobol`.                                                               |

### Variables

| Variable | Exemple    | Description                            |
| -------- | ---------- | -------------------------------------- |
| version  | `v3.1.2.0` | La version de `cobol`                  |
| symbole  |            | Reflète la valeur de l'option `symbol` |
| style\*  |            | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Temps d'exécution

Le module `cmd_duration` montre le temps qu'a pris la dernière commande pour s'exécuter. Le module ne sera affiché que si la commande a pris plus de deux secondes, ou plus que la valeur `min_time`, si elle existe.

> [!WARNING] Ne pas intercepter le piège DEBUG dans Bash
>
> Si vous exécutez Starship dans `bash`, n'interceptez pas le piège `DEBUG` après avoir exécuté `eval $(starship init $0)`, sinon ce module **cessera** de fonctionner.

Les utilisateurs de Bash qui ont besoin d'une fonctionnalité de type preexec peuvent utiliser le [framework bash_preexec de rcaloras](https://github.com/rcaloras/bash-preexec). Définissez simplement les tableaux `preexec_functions` et `precmd_functions` avant d'exécuter `eval $(starship init $0)`, puis procédez comme d'habitude.

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

Le module `conda` affiche l'environnement [Conda](https://docs.conda.io/en/latest/) courant, si `$CONDA_DEFAULT_ENV` est défini.

> [!TIP] Ceci ne supprime pas le modificateur d'invite de conda lui-même, vous pouvez exécuter `conda config --set changeps1 False`. Si vous utilisez [pixi](https://pixi.sh), vous pouvez désactiver le modificateur d'invite de pixi en exécutant `pixi config set shell.change-ps1 false`.

### Options

| Option              | Défaut                                 | Description                                                                                                                                                                                                                                   |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | Le nombre de répertoires dans lesquels le chemin d'environnement (Path) doit être tronqué, si l'environnement a été créé via `conda create -p [path]`. `0` ne signifie pas de troncature. Regardez aussi le module [`directory`](#directory). |
| `symbole`           | `'🅒 '`                                 | Le symbole utilisé avant le nom d'environnement.                                                                                                                                                                                              |
| `style`             | `'bold green'`                         | Le style pour le module.                                                                                                                                                                                                                      |
| `format`            | `'via [$symbol$environment]($style) '` | Format du module.                                                                                                                                                                                                                             |
| `ignore_base`       | `true`                                 | Ignore l'environnement `base` lorsqu'il est activé.                                                                                                                                                                                           |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | Quelles variables d'environnement devraient activer ce module. S'il s'agit d'un environnement pixi, ce module n'est pas activé par défaut.                                                                                                    |
| `disabled`          | `false`                                | Désactive le module `conda`.                                                                                                                                                                                                                  |

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

Le module `container` affiche un symbole et le nom du conteneur, si vous êtes dans un conteneur.

### Options

| Option     | Défaut                           | Description                                          |
| ---------- | -------------------------------- | ---------------------------------------------------- |
| `symbole`  | `'⬢'`                            | Le symbole affiché quand vous êtes dans un conteneur |
| `style`    | `'bold red dimmed'`              | Le style pour le module.                             |
| `format`   | `'[$symbol \[$name\]]($style) '` | Format du module.                                    |
| `disabled` | `false`                          | Désactive le module `container`.                     |

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

Le module `crystal` affiche la version actuellement installée de [Crystal](https://crystal-lang.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie :

- Le dossier courant contient un fichier `shard.yml`
- Le dossier courant contient un fichier `.cr`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `symbole`           | `'🔮 '`                              | Le symbole utilisé avant d'afficher la version de crystal.                                 |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | Le style pour le module.                                                                   |
| `detect_extensions` | `['cr']`                             | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['shard.yml']`                      | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`          | `false`                              | Désactive le module `crystal`.                                                             |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v0.32.1` | La version de `crystal`                |
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

Le module `daml` affiche la version du SDK [Daml](https://www.digitalasset.com/developers) actuellement utilisée lorsque vous êtes dans le répertoire racine de votre projet Daml. La `sdk-version` dans le fichier `daml.yaml` sera utilisée, sauf si elle est remplacée par la variable d’environnement `DAML_SDK_VERSION`. Par défaut, le module sera affiché si l’une de ces conditions est remplie :

- Le dossier courant contient un fichier `daml.yaml`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'Λ '`                               | Une chaîne de format représentant le symbole de Daml                                       |
| `style`             | `'bold cyan'`                        | Le style pour le module.                                                                   |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['daml.yaml']`                      | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`          | `false`                              | Désactive le module `daml`.                                                                |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.2.0` | La version de `daml`                   |
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

Le module `dart` affiche la version actuellement installée de [Dart](https://dart.dev/). Par défaut, le module sera affiché si l’une de ces conditions est remplie :

- Le dossier courant contient un fichier avec l’extension `.dart`
- Le dossier courant contient un fichier `.dart_tool`
- Le dossier courant contient un fichier `pubsepc.yaml`, `pubspec.yml` ou `pubspec.lock`

### Options

| Option              | Défaut                                            | Description                                                                                |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`              | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                                       | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🎯 '`                                           | Une chaîne de caractères représentant le symbole de Dart                                   |
| `detect_extensions` | `['dart']`                                        | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `['.dart_tool']`                                  | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold blue'`                                     | Le style pour le module.                                                                   |
| `disabled`          | `false`                                           | Désactive le module `dart`.                                                                |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.8.4` | La version de `dart`                   |
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

Le module `deno` affiche la version actuellement installée de [Deno](https://deno.land/). Par défaut, le module sera affiché si l’une de ces conditions est remplie :

- Le dossier courant contient un fichier `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` ou `deps.js`

### Options

| Option              | Défaut                                                                               | Description                                                                                |
| ------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                                                                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🦕 '`                                                                              | Une chaîne de caractères représentant le symbole de Deno                                   |
| `detect_extensions` | `[]`                                                                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `'green bold'`                                                                       | Le style pour le module.                                                                   |
| `disabled`          | `false`                                                                              | Désactive le module `deno`.                                                                |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.8.3` | La version de `deno`                   |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Dossier

Le module `directory` affiche le chemin du dossier courant, tronqué à 3 dossiers parents. Votre dossier sera également tronqué à la racine du repo git dans lequel vous vous trouvez actuellement.

Lorsque l'option `fish_style_pwd_dir_length` est utilisée, au lieu de masquer le chemin tronqué, vous verrez un nom raccourci de chaque dossier basé sur le nombre que vous activez pour l'option.

Par exemple, `~/Dev/Nix/nixpkgs/pkgs` où `nixpkgs` est la racine du repo, et l'option définie à `1`. Vous verrez maintenant `~/D/N/nixpkgs/pkgs`, alors que vous auriez vu `nixpkgs/pkgs` avant.

### Options

| Option                   | Défaut                                                                                                                       | Description                                                                                                            |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | Le nombre de dossiers parents auquel tronquer le chemin du répertoire courant.                                         |
| `truncate_to_repo`       | `true`                                                                                                                       | Si oui ou non tronquer à la racine du repo git dans lequel vous vous trouvez.                                          |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | Format du module.                                                                                                      |
| `style`                  | `'bold cyan'`                                                                                                                | Le style pour le module.                                                                                               |
| `disabled`               | `false`                                                                                                                      | Désactive le module `directory`.                                                                                       |
| `read_only`              | `'🔒'`                                                                                                                       | Le symbole indiquant que le répertoire courant est en lecture seule.                                                   |
| `read_only_style`        | `'red'`                                                                                                                      | Le style du symbole de lecture seule.                                                                                  |
| `truncation_symbol`      | `''`                                                                                                                         | Le symbole pour préfixer les chemins tronqués. eg: '…/'                                                                |
| `before_repo_root_style` |                                                                                                                              | Le style du segment de chemin au-dessus de la racine du dépôt git. La valeur par défaut est équivalent à `style`.      |
| `repo_root_style`        |                                                                                                                              | Le style pour la racine du dépôt Git. La valeur par défaut est équivalent à `style`.                                   |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | Le format d'un dépôt git lorsque `before_repo_root_style` et `repo_root_style` sont définis.                           |
| `home_symbol`            | `'~'`                                                                                                                        | Le symbole indiquant le répertoire personnel.                                                                          |
| `use_os_path_sep`        | `true`                                                                                                                       | Utiliser le séparateur de chemin du système d’exploitation au lieu de toujours utiliser `/` (par ex. `\` sous Windows) |

<details>
<summary>Ce module possède quelques options de configuration avancées qui contrôlent l'affichage du répertoire.</summary>

| Options avancées            | Défaut | Description                                                                                                                                                                              |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | Une table de substitutions à appliquer aux chemins.                                                                                                                                      |
| `fish_style_pwd_dir_length` | `0`    | Le nombre de caractères à utiliser lors de l'application de la logique de troncature du pwd de fish.                                                                                     |
| `use_logical_path`          | `true` | Si `true` affiche le chemin logique issu du shell via `PWD` ou `--logical-path`. Si `false` renvoie plutôt le chemin du système de fichiers physique avec les liens symboliques résolus. |

`substitutions` vous permet de définir des remplacements arbitraires pour les chaînes littérales dans le chemin, par exemple les longs préfixes réseau ou les répertoires de développement Java. Notez que cela désactivera la PWD de style fish. L'option accepte un tableau avec les paires clé/valeur suivantes :

| Valeur  | Type    | Description                                            |
| ------- | ------- | ------------------------------------------------------ |
| `from`  | String  | La valeur à substituer                                 |
| `to`    | String  | Le remplacement pour cette valeur, si elle est trouvée |
| `regex` | Boolean | (Optionnel) Si `from` est une expression régulière     |

En utilisant `regex = true`, vous pouvez utiliser les [expressions régulières Rust](https://docs.rs/regex/latest/regex/#syntax) dans `from`.
Par exemple, vous pouvez remplacer chaque slash sauf le premier avec ce qui suit :

```toml
substitutions = [
  { from = "^/", to = "<root>/", regex = true },
  { from = "/", to = " | " },
  { from = "^<root>", to = "/", regex = true },
]
```

Cela remplacera `/var/log` par `/ | var | log`.

L'ancienne syntaxe fonctionne toujours, bien qu'elle ne prenne pas en charge les expressions régulières :

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` interagit avec les options de troncature d'une manière qui peut être surprenante au début : si elle n'est pas nulle, les composantes du chemin qui seraient normalement tronquées sont affichées à la place avec autant de caractères. Par exemple, le chemin `/built/this/city/on/rock/and/roll`, qui serait normalement affiché comme `rock/and/roll`, serait affiché comme `/b/t/c/o/rock/and/roll` avec `fish_style_pwd_dir_length = 1` — les composantes du chemin qui seraient normalement supprimées sont affichées avec un seul caractère. Pour `fish_style_pwd_dir_length = 2`, ce serait `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable | Exemple               | Description                           |
| -------- | --------------------- | ------------------------------------- |
| path     | `'D:/Projects'`       | Le chemin du répertoire courant       |
| style\*  | `'black bold dimmed'` | Reflète la valeur de l'option `style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

<details>
<summary>Les dépôts Git peuvent avoir des variables additionnelles.</summary>

Considérons le chemin `/path/to/home/git_repo/src/lib`

| Variable         | Exemple               | Description                             |
| ---------------- | --------------------- | --------------------------------------- |
| before_root_path | `'/path/to/home/'`    | Le chemin avant le dossier racine Git   |
| repo_root        | `'git_repo'`          | Le nom du dossier racine Git            |
| path             | `'/src/lib'`          | Le reste du chemin                      |
| style            | `'black bold dimmed'` | Reflète la valeur de l'option `style`   |
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

Le module `direnv` affiche le statut du fichier rc courant s'il y en a un. Le statut inclut le chemin vers le fichier rc, s'il est chargé et s'il a été autorisé par `direnv`.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option              | Défaut                                 | Description                                                 |
| ------------------- | -------------------------------------- | ----------------------------------------------------------- |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | Format du module.                                           |
| `symbole`           | `'direnv '`                            | Le symbole utilisé avant d'afficher le contexte direnv.     |
| `style`             | `'bold orange'`                        | Le style pour le module.                                    |
| `disabled`          | `true`                                 | Désactive le module `direnv`.                               |
| `detect_extensions` | `[]`                                   | Les extensions qui déclenchent ce module.                   |
| `detect_files`      | `['.envrc']`                           | Les fichiers qui activent ce module.                        |
| `detect_folders`    | `[]`                                   | Les dossiers qui activent ce module.                        |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Les variables d’environnement qui activent ce module.       |
| `allowed_msg`       | `'allowed'`                            | Le message affiché lorsqu'un fichier rc est autorisé.       |
| `not_allowed_msg`   | `'not allowed'`                        | Le message affiché lorsqu'un fichier rc n'est pas autorisé. |
| `denied_msg`        | `'denied'`                             | Le message affiché lorsqu'un fichier rc est refusé.         |
| `loaded_msg`        | `'loaded'`                             | Le message affiché lorsqu'un fichier rc est chargé.         |
| `unloaded_msg`      | `'not loaded'`                         | Le message affiché lorsqu'un fichier rc n'est pas chargé.   |

### Variables

| Variable | Exemple             | Description                                   |
| -------- | ------------------- | --------------------------------------------- |
| loaded   | `loaded`            | Indique si le fichier rc actuel est chargé.   |
| allowed  | `denied`            | Indique si le fichier rc actuel est autorisé. |
| rc_path  | `/home/test/.envrc` | Le chemin du fichier rc actuel.               |
| symbole  |                     | Reflète la valeur de l'option `symbol`.       |
| style\*  | `red bold`          | Reflète la valeur de l'option `style`.        |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Contexte Docker

Le module `docker_context` affiche le [contexte Docker](https://docs.docker.com/engine/context/working-with-contexts/) actuellement actif s'il n'est pas défini à `default` ou `desktop-linux`, ou si les variables d'environnement `DOCKER_MACHINE_NAME`, `DOCKER_HOST` ou `DOCKER_CONTEXT` sont définies (car elles sont destinées à remplacer le contexte en cours d'utilisation).

### Options

| Option              | Défaut                                                                                       | Description                                                                                            |
| ------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol$context]($style) '`                                                           | Format du module.                                                                                      |
| `symbole`           | `'🐳 '`                                                                                      | Le symbole utilisé avant d'afficher le contexte Docker.                                                |
| `only_with_files`   | `true`                                                                                       | Afficher uniquement quand il y a une correspondance                                                    |
| `detect_extensions` | `[]`                                                                                         | Quelles extensions devraient activer ce module (il faut que `only_with_files` soit réglé sur true).    |
| `detect_files`      | `['compose.yml', 'compose.yaml', 'docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Quels noms de fichier devraient activer ce module (il faut que `only_with_files` soit réglé sur true). |
| `detect_folders`    | `[]`                                                                                         | Quels dossiers devraient activer ce module (il faut que `only_with_files` soit réglé sur true).        |
| `style`             | `'blue bold'`                                                                                | Le style pour le module.                                                                               |
| `disabled`          | `false`                                                                                      | Désactive le module `docker_context`.                                                                  |

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

| Option              | Défaut                                                                                                  | Description                                                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                          | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                                                                                             | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'.NET '`                                                                                               | Le symbole utilisé avant d'afficher la version de dotnet.                                  |
| `heuristic`         | `true`                                                                                                  | Utilisez la détection de versions plus rapide pour garder starship instantané.             |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                    | Quels dossiers devraient activer ce module.                                                |
| `style`             | `'bold blue'`                                                                                           | Le style pour le module.                                                                   |
| `disabled`          | `false`                                                                                                 | Désactive le module `dotnet`.                                                              |

### Variables

| Variable | Exemple          | Description                                    |
| -------- | ---------------- | ---------------------------------------------- |
| version  | `v3.1.201`       | La version du sdk `dotnet`                     |
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

Le module `elixir` affiche la version de [Elixir](https://elixir-lang.org/) et [Erlang/OTP](https://erlang.org/doc/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `mix.exs`.

### Options

| Option              | Défaut                                                    | Description                                                                                |
| ------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | Format du module elixir.                                                                   |
| `version_format`    | `'v${raw}'`                                               | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'💧 '`                                                   | Le symbole utilisé avant d'afficher la version d'Elixir/Erlang.                            |
| `detect_extensions` | `[]`                                                      | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['mix.exs']`                                             | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                      | Quels dossiers devraient activer ce module.                                                |
| `style`             | `'bold purple'`                                           | Le style pour le module.                                                                   |
| `disabled`          | `false`                                                   | Désactive le module `elixir`.                                                              |

### Variables

| Variable    | Exemple | Description                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | La version d' `elixir`                 |
| otp_version |         | La version otp d' `elixir`             |
| symbole     |         | Reflète la valeur de l'option `symbol` |
| style\*     |         | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

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

| Option              | Défaut                                             | Description                                                                                |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`               | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                                        | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🌳 '`                                            | Une chaîne de format représentant le symbole d'Elm.                                        |
| `detect_extensions` | `['elm']`                                          | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `['elm-stuff']`                                    | Quels dossiers devraient activer ce module.                                                |
| `style`             | `'cyan bold'`                                      | Le style pour le module.                                                                   |
| `disabled`          | `false`                                            | Désactive le module `elm`.                                                                 |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v0.19.1` | La version de `elm`                    |
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

Le module `env_var` affiche la valeur actuelle de la variable d’environnement choisie. Le module sera affiché si l'une de ces conditions est remplie:

- L'option `variable` correspond à une variable d'environnement existante
- L'option `variable` n'est pas définie, mais l'option `default` l'est

> [!TIP] L'ordre dans lequel les modules env_var sont affichés peut être défini individuellement en incluant `${env_var.foo}` dans le `format` de niveau supérieur (comme il contient un point, vous devez utiliser `${...}`). Par défaut, le module `env_var` affichera simplement tous les modules env_var dans l'ordre où ils ont été définis.

> [!TIP] Plusieurs variables d'environnement peuvent être affichées en utilisant un `.`. (voir l'exemple) Si l'option de configuration `variable` n'est pas définie, le module affichera la valeur de la variable dont le nom correspond au texte après le caractère `.`.
>
> Exemple : la configuration suivante va afficher la valeur de la variable d’environnement UTILISATEUR
>
> ````toml
>
> # ~/.config/starship.toml
>
> [env_var.USER] default = 'unknown user' ```
> ````

### Options

| Option        | Défaut                         | Description                                                                          |
| ------------- | ------------------------------ | ------------------------------------------------------------------------------------ |
| `symbole`     | `""`                           | Le symbole utilisé avant d'afficher la valeur de la variable.                        |
| `variable`    |                                | La variable d'environnement à afficher.                                              |
| `default`     |                                | La valeur par défaut à afficher lorsque la variable sélectionnée n'est pas définie.  |
| `format`      | `"with [$env_value]($style) "` | Format du module.                                                                    |
| `description` | `"<env_var module>"`           | La description du module qui est affichée lors de l’exécution de `starship explain`. |
| `disabled`    | `false`                        | Désactive le module `env_var`.                                                       |

### Variables

| Variable  | Exemple                                  | Description                                      |
| --------- | ---------------------------------------- | ------------------------------------------------ |
| env_value | `Windows NT` (si _variable_ était `$OS`) | La valeur d'environnement de l'option `variable` |
| symbole   |                                          | Reflète la valeur de l'option `symbol`           |
| style\*   | `black bold dimmed`                      | Reflète la valeur de l'option `style`            |

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

Le module `erlang` affiche la version de [Erlang/OTP](https://erlang.org/doc/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `rebar.config`.
- Le dossier courant contient un fichier `erlang.mk`.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `' '`                               | Le symbole utilisé avant d'afficher la version d'erlang.                                   |
| `style`             | `'bold red'`                         | Le style pour le module.                                                                   |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `disabled`          | `false`                              | Désactive le module `erlang`.                                                              |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v22.1.3` | La version d'`erlang`                  |
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

Le module `fennel` affiche la version actuellement installée de [Fennel](https://fennel-lang.org). Par défaut, le module sera affiché si l’une de ces conditions est remplie :

- Le dossier courant contient un fichier avec l’extension `.fnl`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🧅 '`                              | Le symbole utilisé avant d'afficher la version de fennel.                                  |
| `style`             | `'bold green'`                       | Le style pour le module.                                                                   |
| `detect_extensions` | `['fnl']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `disabled`          | `false`                              | Désactive le module `fennel`.                                                              |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.2.1` | La version de `fennel`                 |
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

## Fortran

Le module `fortran` affiche la version actuelle du compilateur Fortran.

### Options

| Option              | Défaut                                                                                                                      | Description                                                                                |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `symbole`           | `' '`                                                                                                                      | Le symbole utilisé avant d'afficher la version de Fortran.                                 |
| `format`            | `'via [$symbol($version )]($style)'`                                                                                        | Format du module.                                                                          |
| `version_format`    | `'${raw}'`                                                                                                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold purple'`                                                                                                             | Le style pour le module.                                                                   |
| `detect_extensions` | `['f', 'F', 'for', 'FOR', 'ftn', 'FTN', 'f77', 'F77', 'f90', 'F90', 'f95', 'F95','f03', 'F03', 'f08', 'F08', 'f18', 'F18']` | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['fpm.toml']`                                                                                                              | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                                        | Les dossiers qui activent ce module.                                                       |
| `commands`          | `[ [ 'gfortran', '--version' ], [ 'flang', '--version' ], [ 'flang-new', '--version' ] ]`                                   | Comment détecter quel est le compilateur                                                   |
| `disabled`          | `false`                                                                                                                     | Désactive le module `fortran`.                                                             |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| name     | gfortran | Le nom du compilateur                  |
| version  | `14.2.0` | La version du compilateur Fortran      |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Commandes

L’option `commands` accepte une liste de commandes pour déterminer la version du compilateur et son nom.

Chaque commande est représentée par une liste contenant le nom de l'exécutable, suivi de ses arguments, généralement quelque chose comme `['myfortran', '--version']`. Starship essayera d'exécuter chaque commande jusqu'à obtenir un résultat sur STDOUT.

Si un compilateur Fortran n'est pas supporté par ce module, vous pouvez en faire la demande en [créant un ticket sur GitHub](https://github.com/starship/starship/).

## Branche Fossil

Le module `fossil_branch` affiche le nom de la branche active du checkout dans votre répertoire courant.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option              | Défaut                           | Description                                                                                                         |
| ------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | Format du module. Utilisez `'$branch'` pour faire référence au nom de la branche courante.                          |
| `symbole`           | `' '`                           | Le symbole utilisé avant le nom de la branche du checkout dans votre répertoire courant.                            |
| `style`             | `'bold purple'`                  | Le style pour le module.                                                                                            |
| `truncation_length` | `2^63 - 1`                       | Tronque le nom de la branche Fossil à `N` graphèmes.                                                                |
| `truncation_symbol` | `'…'`                            | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué. Utilisez `''` pour ne pas afficher de symbole. |
| `disabled`          | `true`                           | Désactive le module `fossil_branch`.                                                                                |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| branch   | `trunk` | La branche Fossil active               |
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

## Métriques Fossil

Le module `fossil_metrics` affiche le nombre de lignes ajoutées et supprimées dans le checkout de votre répertoire courant. La version v2.14 (2021-01-20) de Fossil ou supérieure est requise.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option               | Défaut                                                       | Description                                           |
| -------------------- | ------------------------------------------------------------ | ----------------------------------------------------- |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Format du module.                                     |
| `added_style`        | `'bold green'`                                               | Le style pour le compte des ajouts.                   |
| `deleted_style`      | `'bold red'`                                                 | Le style pour le compte des suppressions.             |
| `only_nonzero_diffs` | `true`                                                       | Afficher le statut seulement pour les items modifiés. |
| `disabled`           | `true`                                                       | Désactive le module `fossil_metrics`.                 |

### Variables

| Variable        | Exemple | Description                                   |
| --------------- | ------- | --------------------------------------------- |
| added           | `1`     | Le nombre de lignes ajoutées                  |
| deleted         | `2`     | Le nombre de lignes supprimées                |
| added_style\*   |         | Possède la valeur de l’option `added_style`   |
| deleted_style\* |         | Possède la valeur de l’option `deleted_style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Google Cloud (`gcloud`)

Le module `gcloud` affiche la version de la commande [`gcloud`](https://cloud.google.com/sdk/gcloud) installée. Ceci est basé sur les fichiers `~/.config/gcloud/active_config` et `~/.config/gcloud/configurations/config_{CONFIG NAME}` et la variable d'environnement `CLOUDSDK_CONFIG`.

Quand le module est activé, il sera toujours visible, sauf si `detect_env_vars` a été défini, auquel cas le module ne sera actif que lorsqu'une des variables d'environnement aura été définie.

### Options

| Option            | Défaut                                                   | Description                                                   |
| ----------------- | -------------------------------------------------------- | ------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Format du module.                                             |
| `symbole`         | `'☁️  '`                                                 | Le symbole affiché avant le profil GCP actuel.                |
| `region_aliases`  | `{}`                                                     | Table des alias de région à afficher en plus du nom du GCP.   |
| `project_aliases` | `{}`                                                     | Table des alias de projet à afficher en plus du nom du GCP.   |
| `detect_env_vars` | `[]`                                                     | Quelles variables d'environnement devraient activer ce module |
| `style`           | `'bold blue'`                                            | Le style pour le module.                                      |
| `disabled`        | `false`                                                  | Désactive le module `gcloud`.                                 |

### Variables

| Variable | Exemple       | Description                                                                   |
| -------- | ------------- | ----------------------------------------------------------------------------- |
| region   | `us-central1` | La région GCP actuelle                                                        |
| account  | `foo`         | Le profil GCP actuel                                                          |
| domain   | `exemple.com` | Le domaine du profil GCP actuel                                               |
| project  |               | Le projet GCP actuel                                                          |
| active   | `default`     | Le nom de la configuration active écrit dans `~/.config/gcloud/active_config` |
| symbole  |               | Reflète la valeur de l'option `symbol`                                        |
| style\*  |               | Reflète la valeur de l'option `style`                                         |

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

Le module `git_branch` affiche la branche active du dépôt dans le dossier courant.

### Options

| Option               | Défaut                                            | Description                                                                                                         |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Affiche le nom de la branche suivie distante, même si elle est égale au nom de la branche locale.                   |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | Format du module. Utilisez `'$branch'` pour faire référence au nom de la branche courante.                          |
| `symbole`            | `' '`                                            | Une chaîne de format représentant le symbole de la branche git.                                                     |
| `style`              | `'bold purple'`                                   | Le style pour le module.                                                                                            |
| `truncation_length`  | `2^63 - 1`                                        | Tronque une branche git à `N` graphèmes.                                                                            |
| `truncation_symbol`  | `'…'`                                             | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué. Utilisez `''` pour ne pas afficher de symbole. |
| `only_attached`      | `false`                                           | Ne montrer le nom de la branche que si elle n'est pas dans un état `HEAD` détachée.                                 |
| `ignore_branches`    | `[]`                                              | Une liste de noms à ne pas afficher. Utile pour 'master' ou 'main'.                                                 |
| `ignore_bare_repo`   | `false`                                           | Ne pas afficher dans un dépôt nu (bare repo).                                                                       |
| `disabled`           | `false`                                           | Désactive le module `git_branch`.                                                                                   |

### Variables

| Variable      | Exemple  | Description                                                                                                          |
| ------------- | -------- | -------------------------------------------------------------------------------------------------------------------- |
| branch        | `master` | Le nom de la branche actuelle, par défaut à `HEAD` s'il n'y a pas de branche actuelle (par exemple `HEAD` détachée). |
| remote_name   | `origin` | Le nom du dépôt distant.                                                                                             |
| remote_branch | `master` | Le nom de la branche suivie sur `remote_name`.                                                                       |
| symbole       |          | Reflète la valeur de l'option `symbol`                                                                               |
| style\*       |          | Reflète la valeur de l'option `style`                                                                                |

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

Le module `git_commit` affiche le hash du commit courant et l’étiquette (s’il y en a) du dépôt dans le dossier courant.

### Options

| Option               | Défaut                       | Description                                                                                                              |
| -------------------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                          | La longueur du hash affiché du commit git.                                                                               |
| `format`             | `'[\($hash$tag\)]($style) '` | Format du module.                                                                                                        |
| `style`              | `'bold green'`               | Le style pour le module.                                                                                                 |
| `only_detached`      | `true`                       | Ne montrer le hash du commit qu'en mode `HEAD` détachée.                                                                 |
| `tag_disabled`       | `true`                       | Désactive l'affichage des informations du tag dans le module `git_commit`.                                               |
| `tag_max_candidates` | `0`                          | Combien de commits considérer pour l'affichage des tags. Par défaut, seules les correspondances exactes sont autorisées. |
| `tag_symbol`         | `' 🏷  '`                    | Symbole préfixant les informations affichées concernant le tag                                                           |
| `disabled`           | `false`                      | Désactive le module `git_commit`.                                                                                        |

### Variables

| Variable | Exemple   | Description                                                      |
| -------- | --------- | ---------------------------------------------------------------- |
| hash     | `b703eb3` | Le hash du commit git actuel                                     |
| tag      | `v1.0.0`  | Le nom du tag si l'affichage des informations de tag est activé. |
| style\*  |           | Reflète la valeur de l'option `style`                            |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

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

| Option         | Défaut                                                        | Description                                                                                                         |
| -------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                  | Une chaîne de format affichée lorsqu'un `rebase` est en cours.                                                      |
| `merge`        | `'MERGING'`                                                   | Une chaîne de format affichée quand un `merge` est en cours.                                                        |
| `revert`       | `'REVERTING'`                                                 | Une chaîne de format affichée quand un `revert` est en cours.                                                       |
| `cherry_pick`  | `'CHERRY-PICKING'`                                            | Une chaîne de format affichée quand un `cherry-pick` est en cours.                                                  |
| `bisect`       | `'BISECTING'`                                                 | Une chaîne de format affichée quand un `bisect` est en cours.                                                       |
| `am`           | `'AM'`                                                        | Une chaîne de format affichée lorsqu'un `apply-mailbox` (`git am`) est en cours.                                    |
| `am_or_rebase` | `'AM/REBASE'`                                                 | Une chaîne de format affichée lorsqu'une `apply-mailbox` ou un `rebase` est en cours sans pouvoir les différencier. |
| `style`        | `'bold yellow'`                                               | Le style pour le module.                                                                                            |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Format du module.                                                                                                   |
| `disabled`     | `false`                                                       | Désactive le module `git_state`.                                                                                    |

### Variables

| Variable         | Exemple    | Description                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | L'état actuel du dépôt                |
| progress_current | `1`        | Progression de l'opération en cours   |
| progress_total   | `2`        | Progression maximale de l'opération   |
| style\*          |            | Reflète la valeur de l'option `style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Métriques Git

Le module `git_metrics` affiche le nombre de lignes ajoutées et supprimées dans le dépôt Git courant.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option               | Défaut                                                       | Description                                           |
| -------------------- | ------------------------------------------------------------ | ----------------------------------------------------- |
| `added_style`        | `'bold green'`                                               | Le style pour le compte des ajouts.                   |
| `deleted_style`      | `'bold red'`                                                 | Le style pour le compte des suppressions.             |
| `only_nonzero_diffs` | `true`                                                       | Afficher le statut seulement pour les items modifiés. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Format du module.                                     |
| `disabled`           | `true`                                                       | Désactive le module `git_metrics`.                    |
| `ignore_submodules`  | `false`                                                      | Ignorer les changements des sous-modules              |

### Variables

| Variable        | Exemple | Description                                   |
| --------------- | ------- | --------------------------------------------- |
| added           | `1`     | Le nombre de lignes ajoutées                  |
| deleted         | `2`     | Le nombre de lignes supprimées                |
| added_style\*   |         | Possède la valeur de l’option `added_style`   |
| deleted_style\* |         | Possède la valeur de l’option `deleted_style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Statut Git

Le module `git_status` affiche des symboles représentant l’état du dépôt dans le dossier courant.

> [!TIP] The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment. Vous pouvez désactiver le module ou utiliser l’option `windows_starship` pour utiliser un exécutable Starship natif pour calculer le `git_status` pour ces chemins.

### Options

| Option                 | Défaut                                        | Description                                                                                                                     |
| ---------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `format`               | `'([\[$all_status$ahead_behind\]]($style) )'` | Le format par défaut du module `git_status`                                                                                     |
| `conflicted`           | `'='`                                         | Cette branche a des conflits de fusion.                                                                                         |
| `ahead`                | `'⇡'`                                         | Le format de `ahead`                                                                                                            |
| `behind`               | `'⇣'`                                         | Le format de `behind`                                                                                                           |
| `diverged`             | `'⇕'`                                         | Le format de `diverged`                                                                                                         |
| `up_to_date`           | `''`                                          | Le format de `up_to_date`                                                                                                       |
| `untracked`            | `'?'`                                         | Le format de `untracked`                                                                                                        |
| `stashed`              | `'\$'`                                        | Le format de `stashed`                                                                                                          |
| `modified`             | `'!'`                                         | Le format de `modified`                                                                                                         |
| `staged`               | `'+'`                                         | Le format de `staged`                                                                                                           |
| `renamed`              | `'»'`                                         | Le format de `renamed`                                                                                                          |
| `deleted`              | `'✘'`                                         | Le format de `deleted`                                                                                                          |
| `typechanged`          | `""`                                          | Le format affiché quand le type d'un fichier a été modifié dans la zone de validation.                                          |
| `style`                | `'bold red'`                                  | Le style pour le module.                                                                                                        |
| `ignore_submodules`    | `false`                                       | Ignorer les changements des sous-modules.                                                                                       |
| `worktree_added`       | `""`                                          | Le format affiché quand un nouveau fichier a été ajouté dans le répertoire de travail.                                          |
| `worktree_deleted`     | `""`                                          | Le format affiché quand un fichier a été supprimé dans le répertoire de travail.                                                |
| `worktree_modified`    | `""`                                          | Le format affiché quand un fichier a été modifié dans le répertoire de travail.                                                 |
| `worktree_typechanged` | `""`                                          | Le format affiché quand le type d'un fichier a été modifié dans le répertoire de travail.                                       |
| `index_added`          | `""`                                          | Le format affiché quand un nouveau fichier a été ajouté à la zone de validation.                                                |
| `index_deleted`        | `""`                                          | Le format affiché quand un fichier a été supprimé de la zone de validation.                                                     |
| `index_modified`       | `""`                                          | Le format affiché quand un fichier a été modifié dans la zone de validation.                                                    |
| `index_typechanged`    | `""`                                          | Le format affiché quand le type d'un fichier a été modifié dans la zone de validation.                                          |
| `disabled`             | `false`                                       | Désactive le module `git_status`.                                                                                               |
| `windows_starship`     |                                               | Utiliser ce chemin (Linux) vers un exécutable Starship Windows pour afficher le `git_status` pour les chemins Windows dans WSL. |
| `use_git_executable`   | `false`                                       | Ne pas utiliser `gitoxide` pour calculer le statut, mais utiliser l'exécutable `git` à la place.                                |

### Variables

Les variables suivantes peuvent être utilisées pour la valeur de `format`:

| Variable               | Description                                                                                                           |
| ---------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `all_status`           | Raccourci pour `$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`.                           |
| `ahead_behind`         | Affiche la chaîne de formatage `diverged`, `ahead`, `behind` ou `up_to_date` en se basant sur l’état actuel du dépôt. |
| `conflicted`           | Affiche `conflicted` lorsque la branche courante a des conflits de fusion.                                            |
| `untracked`            | Affiche `untracked` lorsqu’il y a des fichiers non suivis dans le répertoire de travail.                              |
| `stashed`              | Affiche `stashed` lorsqu’une remise existe pour le dépôt local.                                                       |
| `modified`             | Affiche `modified` lorsqu’il y a des fichiers modifiés dans le répertoire de travail.                                 |
| `staged`               | Affiche `staged` lorsqu’un nouveau fichier a été ajouté à la zone de validation.                                      |
| `renamed`              | Affiche `renamed` lorsqu’un fichier renommé a été ajouté à la zone de validation.                                     |
| `deleted`              | Affiche `deleted` lorsque la suppression d’un fichier a été ajoutée à la zone de validation.                          |
| `typechanged`          | Affiche `typechanged` lorsque le type d’un fichier a été modifié dans la zone de validation.                          |
| `worktree_added`       | Affiche `worktree_added` lorsqu’un nouveau fichier a été ajouté dans le répertoire de travail.                        |
| `worktree_deleted`     | Affiche `worktree_deleted` lorsqu’un fichier a été supprimé dans le répertoire de travail.                            |
| `worktree_modified`    | Affiche `worktree_modified` lorsqu’un fichier a été modifié dans le répertoire de travail.                            |
| `worktree_typechanged` | Affiche `worktree_typechanged` lorsque le type d’un fichier a été modifié dans le répertoire de travail.              |
| `index_added`          | Affiche `index_added` lorsqu’un nouveau fichier a été ajouté à la zone de validation.                                 |
| `index_deleted`        | Affiche `index_deleted` lorsqu’un fichier a été supprimé de la zone de validation.                                    |
| `index_modified`       | Affiche `index_modified` lorsqu’un fichier a été modifié dans la zone de validation.                                  |
| `index_typechanged`    | Affiche `index_typechanged` lorsque le type d’un fichier a été modifié dans la zone de validation.                    |
| style\*                | Reflète la valeur de l’option `style`                                                                                 |

\*: Cette variable peut uniquement être utilisée dans une chaîne de style

Les variables suivantes peuvent être utilisées pour la valeur de `diverged` :

| Variable       | Description                                       |
| -------------- | ------------------------------------------------- |
| `ahead_count`  | Nombre de commits en avance sur la branche suivie |
| `behind_count` | Nombre de commits en retard sur la branche suivie |

Les variables suivantes peuvent être utilisées dans `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed`, `deleted`, `typechanged`, `worktree_added`, `worktree_deleted`, `worktree_modified`, `worktree_typechanged`, `index_added`, `index_deleted`, `index_modified` et `index_typechanged` :

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

Le module `gleam` affiche la version actuellement installée de [Gleam](https://gleam.run/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `gleam.toml`
- Le répertoire courant contient un fichier avec l'extension `.gleam`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'⭐ '`                              | Une chaîne de format représentant le symbole de Gleam.                                     |
| `detect_extensions` | `['gleam']`                          | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['gleam.toml']`                     | Les fichiers qui activent ce module.                                                       |
| `style`             | `'bold #FFAFF3'`                     | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `gleam`.                                                               |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.0.0` | La version de `gleam`                  |
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

| Option              | Défaut                                                                                    | Description                                                                                                    |
| ------------------- | ----------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | Format du module.                                                                                              |
| `version_format`    | `'v${raw}'`                                                                               | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch`                     |
| `symbole`           | `'🐹 '`                                                                                   | Une chaîne de caractères représentant le symbole de Go.                                                        |
| `detect_extensions` | `['go']`                                                                                  | Les extensions qui déclenchent ce module.                                                                      |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Les fichiers qui activent ce module.                                                                           |
| `detect_folders`    | `['Godeps']`                                                                              | Les dossiers qui activent ce module.                                                                           |
| `style`             | `'bold cyan'`                                                                             | Le style pour le module.                                                                                       |
| `not_capable_style` | `'bold red'`                                                                              | Le style du module lorsque la directive go dans le fichier go.mod ne correspond pas à la version Go installée. |
| `disabled`          | `false`                                                                                   | Désactive le module `golang`.                                                                                  |

### Variables

| Variable    | Exemple   | Description                                                                                                                                 |
| ----------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| version     | `v1.12.1` | La version de `go`                                                                                                                          |
| mod_version | `1.16`    | `go` version requirement as set in the go directive of `go.mod`. Will only show if the version requirement does not match the `go` version. |
| symbole     |           | Reflète la valeur de l'option `symbol`                                                                                                      |
| style\*     |           | Reflète la valeur de l'option `style`                                                                                                       |

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

Le module `guix_shell` affiche l'environnement [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html). Le module sera affiché lorsque vous êtes dans un environnement guix-shell.

### Options

| Option     | Défaut                     | Description                                                 |
| ---------- | -------------------------- | ----------------------------------------------------------- |
| `format`   | `'via [$symbol]($style) '` | Format du module.                                           |
| `symbole`  | `'🐃 '`                    | Une chaîne de format représentant le symbole de guix-shell. |
| `style`    | `'yellow bold'`            | Le style pour le module.                                    |
| `disabled` | `false`                    | Désactive le module `guix_shell`.                           |

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

Le module `gradle` affiche la version du [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) actuellement utilisée dans le répertoire du projet.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le répertoire courant contient un répertoire `gradle/wrapper/gradle-wrapper.properties`.
- Le répertoire courant contient un fichier se terminant par `.gradle` ou `.gradle.kts`.

Le module `gradle` ne peut lire la version de votre Gradle Wrapper que depuis votre fichier de configuration ; il n'exécute pas votre wrapper pour des raisons de sécurité.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🅶 '`                               | Une chaîne de format représentant le symbole de Gradle.                                    |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `['gradle']`                         | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold bright-cyan'`                 | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `gradle`.                                                              |
| `recursive`         | `false`                              | Active la recherche récursive du répertoire `gradle`.                                      |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v7.5.1` | La version de `gradle`                 |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Haskell

Le module `haskell` affiche la version de GHC et/ou l’instantané Stack sélectionnée.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `stack.yaml`
- Le dossier courant contient un fichier `.hs`, `.cabal` ou `.hs-boot`

### Options

| Option              | Défaut                               | Description                                             |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                       |
| `symbole`           | `'λ '`                               | Une chaîne de format représentant le symbole de Haskell |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Les extensions qui déclenchent ce module.               |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Les fichiers qui activent ce module.                    |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                    |
| `style`             | `'bold purple'`                      | Le style pour le module.                                |
| `disabled`          | `false`                              | Désactive le module `haskell`.                          |

### Variables

| Variable    | Exemple     | Description                                                                                  |
| ----------- | ----------- | -------------------------------------------------------------------------------------------- |
| version     |             | `ghc_version` ou `snapshot` en fonction de si le dossier courant est un project Stack ou pas |
| snapshot    | `lts-18.12` | L’instantané de Stack sélectionné                                                            |
| ghc_version | `9.2.1`     | Version de GHC installée                                                                     |
| symbole     |             | Reflète la valeur de l'option `symbol`                                                       |
| style\*     |             | Reflète la valeur de l'option `style`                                                        |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Haxe

Le module `haxe` affiche la version actuellement installée de [Haxe](https://haxe.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le répertoire courant contient un fichier `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` ou `.haxerc`
- Le répertoire courant contient un répertoire `.haxelib` ou `haxe_libraries`
- Le répertoire courant contient un fichier avec l'extension `.hx` ou `.hxml`

### Options

| Option              | Défaut                                                                                          | Description                                                                                |
| ------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                                                                                     | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | Quels dossiers devraient activer ce module.                                                |
| `symbole`           | `'⌘ '`                                                                                          | Une chaîne de format représentant le symbole de Haxe.                                      |
| `style`             | `'bold fg:202'`                                                                                 | Le style pour le module.                                                                   |
| `disabled`          | `false`                                                                                         | Désactive le module `haxe`.                                                                |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v4.2.5` | La version de `haxe`                   |
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

Le module `helm` affiche la version de [Helm](https://helm.sh/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `helmfile.yaml`
- Le dossier courant contient un fichier `Chart.yaml`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbole`           | `'⎈ '`                               | Une chaîne de format représentant le symbole de Helm.                                      |
| `style`             | `'bold white'`                       | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `helm`.                                                                |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v3.1.1` | La version de `helm`                   |
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

Le module `hostname` affiche le nom d’hôte du système system.

### Options

| Option            | Défaut                                 | Description                                                                                                                                         |
| ----------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | Afficher uniquement le nom d'hôte lorsque vous êtes connecté à une session SSH.                                                                     |
| `ssh_symbol`      | `'🌐 '`                                | Une chaîne de format représentant le symbole lors d'une connexion SSH.                                                                              |
| `trim_at`         | `'.'`                                  | Chaîne à laquelle le nom d'hôte est coupé, après la première correspondance. `'.'` will stop after the first dot. `''` will disable any truncation. |
| `detect_env_vars` | `[]`                                   | Quelles variables d'environnement devraient activer ce module.                                                                                      |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | Format du module.                                                                                                                                   |
| `style`           | `'bold dimmed green'`                  | Le style pour le module.                                                                                                                            |
| `disabled`        | `false`                                | Désactive le module `hostname`.                                                                                                                     |
| `aliases`         | `{}`                                   | Traduit les noms d'hôte système en autre chose. Si `trim_at` est spécifié, seule la première partie sera comparée et remplacée.                     |

### Variables

| Variable   | Exemple    | Description                               |
| ---------- | ---------- | ----------------------------------------- |
| nom d'hôte | `computer` | Le nom d’hôte de l’ordinateur             |
| style\*    |            | Reflète la valeur de l'option `style`     |
| ssh_symbol | `'🌏 '`    | Le symbole représentant une connexion SSH |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemples

#### Toujours afficher le nom d'hôte

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

#### Masquer le nom d'hôte dans les sessions tmux distantes

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
detect_env_vars = ['!TMUX', 'SSH_CONNECTION']
disabled = false
```

#### Remplacer le nom d'hôte par un surnom

```toml
# ~/.config/starship.toml
[hostname]
aliases = { "Max's MacBook Pro" = "home" }
```

## Java

Le module `java` affiche la version de [Java](https://www.oracle.com/java/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le répertoire courant contient un fichier `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, ou `.sdkmanrc`
- Le dossier courant contient un fichier avec l’extension `.java`, `.class`, `.gradle`, `.jar`, `.clj` ou `.cljc`

### Options

| Option              | Défaut                                                                                                                | Description                                                                                |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                                                                                                           | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                                  | Quels dossiers devraient activer ce module.                                                |
| `symbole`           | `'☕ '`                                                                                                               | Une chaîne de caractères représentant le symbole de Java                                   |
| `style`             | `'red dimmed'`                                                                                                        | Le style pour le module.                                                                   |
| `disabled`          | `false`                                                                                                               | Désactive le module `java`.                                                                |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| version  | `v14`   | La version de `java`                   |
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

Le module `jobs` affiche le nombre de tâches en cours. Le module ne sera affiché que s'il y a des tâches de fond. Le module affichera le nombre de tâches en cours d’exécution s’il y en a au moins 2, ou plus que la valeur `threshold`, si elle existe. Le monde affichera un symbole s’il y a au moins 1 tâche, ou plus que la valeur de `symbol_threshold`, si elle existe. Vous pouvez définir les deux valeurs à 0 pour _toujours_ montrer le symbole et le nombre de tâches, même s’il n’y a aucune tâche en cours d’exécution.

Le fonctionnement par défaut est:

- 0 tâche -> Rien n’est affiché.
- 1 tâche -> `symbol` est affiché.
- 2 taĉhes ou plus -> `symbol` + `number` sont affichés.

> [!WARNING] Ce module n'est pas supporté sur tcsh.

> [!WARNING] L'option `threshold` est obsolète, mais si vous souhaitez l'utiliser, le module affichera le nombre de tâches en cours s'il y a plus d'une tâche, ou plus que la valeur de configuration `threshold`, si elle existe. Si `threshold` est défini à 0, alors le module s'affichera aussi quand il y a 0 tâche en cours.

### Options

| Option             | Défaut                        | Description                                                               |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------- |
| `threshold`\*      | `1`                           | Afficher le nombre de jobs si dépassé.                                    |
| `symbol_threshold` | `1`                           | Affiche `symbol` si le nombre de tâches vaut au moins `symbol_threshold`. |
| `number_threshold` | `2`                           | Affiche le nombre de tâches s’il y en a au moins `number_threshold`.      |
| `format`           | `'[$symbol$number]($style) '` | Format du module.                                                         |
| `symbole`          | `'✦'`                         | La chaine utilisée pour représenter la variable `symbole`.                |
| `style`            | `'bold blue'`                 | Le style pour le module.                                                  |
| `disabled`         | `false`                       | Désactive le module `jobs`.                                               |

\*: Cette option is dépréciée, utilisez les options `number_threshold` et `symbol_threshold` à la place.

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

#### Modifier le comportement de regroupement des processus dans fish

Lorsque vous utilisez le shell Fish, Starship compte les **groupes de tâches** au lieu des identifiants de processus individuels par défaut. Cela évite le surcomptage lorsqu'un pipeline a plusieurs processus mais un seul groupe suspendu. Pour revenir au comptage basé sur les PID, ajoutez ce qui suit à votre configuration shell :

```fish
set -g __starship_fish_use_job_groups "false"
```

## Julia

Le module `julia` affiche la version de [Julia](https://julialang.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `Project.toml`
- Le dossier courant contient un fichier `Manifest.toml`
- Le dossier courant contient un fichier `.jl`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbole`           | `'ஃ '`                               | Une chaîne de caractères représentant le symbole de Julia.                                 |
| `style`             | `'bold purple'`                      | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `Julia`.                                                               |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.4.0` | La version de `julia`                  |
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

Le module `kotlin` affiche la version de [Kotlin](https://kotlinlang.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `.kt` ou `.kts`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['kt', 'kts']`                      | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbole`           | `'🅺 '`                               | Une chaîne de caractères représentant le symbole de Kotlin.                                |
| `style`             | `'bold blue'`                        | Le style pour le module.                                                                   |
| `kotlin_binary`     | `'kotlin'`                           | Configure le binaire kotlin que Starship exécute lors de l'obtention de la version.        |
| `disabled`          | `false`                              | Désactive le module `kotlin`.                                                              |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v1.4.21` | La version de `kotlin`                 |
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
# Utilise le binaire du compilateur Kotlin pour obtenir la version installée
kotlin_binary = 'kotlinc'
```

## Kubernetes

Afficher le nom du [contexte Kubernetes](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) courant, et, si défini, l’espace de nom, l’utilisateur, et le cluster depuis le fichier kubeconfig. L’espace de noms doit être défini dans le fichier kubeconfig, ce qui peut être fait via `kubectl config set-context starship-context --namespace astronaut`. De la même façon, l’utilisateur et le cluster peuvent être définis avec `kubectl config set-context starship-context --user starship-user` et `kubectl config set-context starship-context --cluster starship-cluster`. Si la variable d’environnement `$KUBECONFIG` est définie, le module l’utilisera, sinon il utilisera le fichier `~/.kube/config`.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.
>
> Quand le module est activé, il sera toujours actif, à moins que l'une des options `detect_env_vars`, `detect_extensions`, `detect_files` ou `detect_folders` n'ait été définie, auquel cas le module ne sera actif que dans les répertoires correspondant à ces conditions ou lorsqu'une des variables d'environnement a été définie.

### Options

> [!WARNING] Les options `context_aliases` et `user_aliases` sont obsolètes. Utilisez `contexts` et les options `context_alias` et `user_alias` correspondantes à la place.

| Option              | Défaut                                             | Description                                                            |
| ------------------- | -------------------------------------------------- | ---------------------------------------------------------------------- |
| `symbole`           | `'☸ '`                                             | Une chaîne de format représentant le symbole affiché avant le Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | Format du module.                                                      |
| `style`             | `'cyan bold'`                                      | Le style pour le module.                                               |
| `context_aliases`\* | `{}`                                               | Tableau des alias de contexte à afficher.                              |
| `user_aliases`\*    | `{}`                                               | Tableau des alias d'utilisateur à afficher.                            |
| `detect_extensions` | `[]`                                               | Les extensions qui déclenchent ce module.                              |
| `detect_files`      | `[]`                                               | Les fichiers qui activent ce module.                                   |
| `detect_folders`    | `[]`                                               | Quels dossiers devraient activer ce module.                            |
| `detect_env_vars`   | `[]`                                               | Les variables d'environnement qui déclenchent ce module.               |
| `contexts`          | `[]`                                               | Styles et symboles personnalisés pour des contextes spécifiques.       |
| `disabled`          | `true`                                             | Désactiver le module `kubernetes`.                                     |

\*: Cette option est obsolète, veuillez ajouter `contexts` avec les options `context_alias` et `user_alias` correspondantes à la place.

Pour personnaliser le style du module pour des environnements spécifiques, utilisez la configuration suivante dans la liste `contexts` :

| Variable          | Description                                                                                                  |
| ----------------- | ------------------------------------------------------------------------------------------------------------ |
| `context_pattern` | **Requis** Expression régulière pour correspondre au nom du contexte Kubernetes actuel.                      |
| `user_pattern`    | Expression régulière pour correspondre au nom d'utilisateur Kubernetes actuel.                               |
| `context_alias`   | Alias de contexte à afficher à la place du nom complet du contexte.                                          |
| `user_alias`      | Alias d'utilisateur à afficher à la place du nom complet de l'utilisateur.                                   |
| `style`           | Le style du module lors de l'utilisation de ce contexte. Si non défini, le style du module sera utilisé.     |
| `symbole`         | Le symbole du module lors de l'utilisation de ce contexte. Si non défini, le symbole du module sera utilisé. |

Notez que toutes les expressions régulières sont ancrées avec `^<pattern>$` et doivent donc correspondre à la chaîne entière. Les expressions régulières `*_pattern` peuvent contenir des groupes de capture, qui peuvent être référencés dans l'alias correspondant via `$name` et `$N` (voir l'exemple ci-dessous et la [documentation de Regex::replace() de Rust](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

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

N'afficher le module que dans les répertoires contenant un fichier `k8s`.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Configuration spécifique au contexte Kubernetes

L'option de configuration `contexts` est utilisée pour personnaliser l'apparence du nom du contexte Kubernetes actuel (style et symbole) si le nom correspond à l'expression régulière définie.

```toml
# ~/.config/starship.toml

[[kubernetes.contexts]]
# Style "bold red" + symbole par défaut quand le nom du contexte Kubernetes actuel est "production" *et* que l'utilisateur
# actuel est "admin_user"
context_pattern = "production"
user_pattern = "admin_user"
style = "bold red"
context_alias = "prod"
user_alias = "admin"

[[kubernetes.contexts]]
# Style "green" + un symbole différent quand le nom du contexte Kubernetes actuel contient openshift
context_pattern = ".*openshift.*"
style = "green"
symbol = "💔 "
context_alias = "openshift"

[[kubernetes.contexts]]
# Utilisation de groupes de capture
# Les contextes de GKE, AWS et d'autres fournisseurs cloud contiennent généralement des informations supplémentaires, comme la région/zone.
# L'entrée suivante correspond au format GKE (`gke_projectname_zone_cluster-name`)
# et renomme chaque contexte kube correspondant dans un format plus lisible (`gke-cluster-name`) :
context_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
context_alias = "gke-$cluster"
```

## Saut de ligne

Le module `line_break` sépare l'invite en deux lignes.

### Options

| Option     | Défaut  | Description                                                             |
| ---------- | ------- | ----------------------------------------------------------------------- |
| `disabled` | `false` | Désactive le module `line_break`, mettant l'invite sur une seule ligne. |

### Exemple

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## IP locale

Le module `localip` affiche l’adresse IPv4 de l’interface réseau principale.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option     | Défaut                    | Description                                                       |
| ---------- | ------------------------- | ----------------------------------------------------------------- |
| `ssh_only` | `true`                    | Afficher uniquenement l’adresse IP si connecté à une session SSH. |
| `format`   | `'[$localipv4]($style) '` | Format du module.                                                 |
| `style`    | `'bold yellow'`           | Le style pour le module.                                          |
| `disabled` | `true`                    | Désactive le module `localip`.                                    |

### Variables

| Variable  | Exemple      | Description                           |
| --------- | ------------ | ------------------------------------- |
| localipv4 | 192.168.1.13 | Contient l’adresse IPv4 principale    |
| style\*   |              | Reflète la valeur de l'option `style` |

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

Le module `lua` affiche la version de [Lua](http://www.lua.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `.lua-version`
- Le dossier courant contient un dossier `lua`
- Le dossier courant contient un fichier avec l’extension `.lua`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🌙 '`                              | Une chaîne de caractères représentant le symbole de Lua.                                   |
| `detect_extensions` | `['lua']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['.lua-version']`                   | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `['lua']`                            | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold blue'`                        | Le style pour le module.                                                                   |
| `lua_binary`        | `'lua'`                              | Configure le binaire lua que Starship exécute lors de l'obtention de la version.           |
| `disabled`          | `false`                              | Désactive le module `lua`.                                                                 |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v5.4.0` | La version de `lua`                    |
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

Le module `maven` indique la présence d'un projet Maven dans le répertoire courant. Si le [Maven Wrapper](https://maven.apache.org/wrapper/) est activé, la version de Maven sera analysée depuis `.mvn/wrapper/maven-wrapper.properties` et affichée.

Par défaut, le module sera affiché si l'une des conditions suivantes est remplie :

- Le répertoire courant contient un fichier `pom.xml`.
- Le répertoire courant contient un fichier `.mvn/wrapper/maven-wrapper.properties`.

Si vous utilisez une syntaxe POM alternative (par exemple `pom.hocon`), ajoutez son nom de fichier à `detect_files`.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Le format du module.                                                                       |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor` et `patch` |
| `symbol`            | `'🅼 '`                               | Une chaîne de formatage représentant le symbole de Maven.                                  |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['pom.xml']`                        | Les fichiers qui déclenchent ce module.                                                    |
| `detect_folders`    | `['.mvn']`                           | Les dossiers qui déclenchent ce module.                                                    |
| `style`             | `'bold bright-cyan'`                 | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `maven`.                                                               |
| `recursive`         | `false`                              | Active la recherche récursive du répertoire `.mvn`.                                        |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v3.2.0` | La version de `maven`                  |
| symbol   |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable ne peut être utilisée que dans une chaîne de style

## Utilisation mémoire

Le module `memory_usage` affiche la mémoire système actuelle et l'utilisation de swap.

Par défaut, l'utilisation du swap est affichée si le swap total du système n'est pas nul.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option      | Défaut                                         | Description                                                                    |
| ----------- | ---------------------------------------------- | ------------------------------------------------------------------------------ |
| `threshold` | `75`                                           | Masquer l'utilisation de la mémoire à moins qu'elle ne dépasse ce pourcentage. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | Format du module.                                                              |
| `symbole`   | `'🐏'`                                         | Le symbole utilisé avant d'afficher l'utilisation de la mémoire.               |
| `style`     | `'bold dimmed white'`                          | Le style pour le module.                                                       |
| `disabled`  | `true`                                         | Désactiver le module `memory_usage`.                                           |

### Variables

| Variable     | Exemple       | Description                                                                     |
| ------------ | ------------- | ------------------------------------------------------------------------------- |
| ram          | `31GiB/65GiB` | La mémoire système utilisée/totale .                                            |
| ram_pct      | `48%`         | Le pourcentage de la mémoire du système actuel.                                 |
| swap\*\*     | `1GiB/4GiB`   | La taille de la mémoire swap du fichier de mémoire swap du système courant.     |
| swap_pct\*\* | `77%`         | Le poucentage de la mémoire swap du fichier de mémoire swap du système courant. |
| symbole      | `🐏`          | Reflète la valeur de l'option `symbol`                                          |
| style\*      |               | Reflète la valeur de l'option `style`                                           |

_: Cette variable peut uniquement être utilisée dans une chaine de style _\*: Les informations sur le fichier SWAP est uniquement affichée si détectée sur le système courant

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

Le module `meson` affiche l'état actuel de l'environnement de développement Meson.

Par défaut, le nom du projet Meson est affiché si `$MESON_DEVENV` est défini.

### Options

| Option              | Défaut                             | Description                                                                                                        |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `2^32 - 1`                         | Tronque le nom du projet à `N` graphèmes.                                                                          |
| `truncation_symbol` | `'…'`                              | Le symbole utilisé pour indiquer qu'un nom de projet a été tronqué. Utilisez `''` pour ne pas afficher de symbole. |
| `format`            | `'via [$symbol$project]($style) '` | Format du module.                                                                                                  |
| `symbole`           | `'⬢ '`                             | Le symbole utilisé avant d'afficher le nom du projet.                                                              |
| `style`             | `'blue bold'`                      | Le style pour le module.                                                                                           |
| `disabled`          | `false`                            | Désactive le module `meson`.                                                                                       |

### Variables

| Variable | Exemple    | Description                            |
| -------- | ---------- | -------------------------------------- |
| project  | `starship` | Le nom du projet Meson actuel          |
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

Le module `hg_branch` affiche la branche et le sujet actifs du dépôt dans votre répertoire courant.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option              | Défaut                                    | Description                                                                                                |
| ------------------- | ----------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `symbole`           | `' '`                                    | Le symbole utilisé avant le marque-page hg ou le nom de la branche du dépôt dans votre répertoire courant. |
| `style`             | `'bold purple'`                           | Le style pour le module.                                                                                   |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | Format du module.                                                                                          |
| `truncation_length` | `2^63 - 1`                                | Tronque le nom de la branche / du sujet hg à `N` graphèmes                                                 |
| `truncation_symbol` | `'…'`                                     | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué.                                       |
| `disabled`          | `true`                                    | Désactive le module `hg_branch`.                                                                           |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| branch   | `master`  | La branche mercuriale active           |
| topic    | `feature` | Le sujet Mercurial actif               |
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

## État Mercurial

Le module `hg_state` s'affiche dans les répertoires faisant partie d'un dépôt Mercurial où une opération est en cours, telle que : _REBASING_, _BISECTING_, etc.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option       | Défaut                    | Description                                                        |
| ------------ | ------------------------- | ------------------------------------------------------------------ |
| `merge`      | `'MERGING'`               | Une chaîne de format affichée quand un `merge` est en cours.       |
| `rebase`     | `'REBASING'`              | Une chaîne de format affichée lorsqu'un `rebase` est en cours.     |
| `update`     | `'UPDATING'`              | Une chaîne de format affichée lorsqu'un `update` est en cours.     |
| `bisect`     | `'BISECTING'`             | Une chaîne de format affichée quand un `bisect` est en cours.      |
| `shelve`     | `'SHELVING'`              | Une chaîne de format affichée lorsqu'un `shelve` est en cours.     |
| `graft`      | `'GRAFTING'`              | Une chaîne de format affichée lorsqu'un `graft` est en cours.      |
| `transplant` | `'TRANSPLANTING'`         | Une chaîne de format affichée lorsqu'un `transplant` est en cours. |
| `histedit`   | `'HISTEDITING'`           | Une chaîne de format affichée lorsqu'un `histedit` est en cours.   |
| `style`      | `'bold yellow'`           | Le style pour le module.                                           |
| `format`     | `'\([$state]($style)\) '` | Format du module.                                                  |
| `disabled`   | `true`                    | Désactive le module `hg_state`.                                    |

### Variables

| Variable         | Exemple    | Description                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | L'état actuel du dépôt                |
| progress_current | `1`        | Progression de l'opération en cours   |
| progress_total   | `2`        | Progression maximale de l'opération   |
| style\*          |            | Reflète la valeur de l'option `style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Mise

Le module `mise` affiche l'état de santé actuel de mise tel que rapporté par `mise doctor`.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option              | Défaut                                                               | Description                                                 |
| ------------------- | -------------------------------------------------------------------- | ----------------------------------------------------------- |
| `symbole`           | `'mise '`                                                            | Le symbole utilisé avant d'afficher l'état de _mise_.       |
| `style`             | `'bold purple'`                                                      | Le style pour le module.                                    |
| `format`            | `'on [$symbol$health]($style) '`                                     | Format du module.                                           |
| `detect_extensions` | `[]`                                                                 | Les extensions qui déclenchent ce module.                   |
| `detect_files`      | `['mise.toml', 'mise.local.toml', '.mise.toml', '.mise.local.toml']` | Les fichiers qui activent ce module.                        |
| `detect_folders`    | `['.mise']`                                                          | Les dossiers qui activent ce module.                        |
| `healthy_symbol`    | `healthy`                                                            | Le message affiché lorsque _mise_ est en bonne santé.       |
| `unhealthy_symbol`  | `unhealthy`                                                          | Le message affiché lorsque _mise_ n'est pas en bonne santé. |
| `disabled`          | `true`                                                               | Désactive le module `mise`.                                 |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| health   | `healthy` | L'état de santé de _mise_              |
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

Le module `mojo` affiche la version actuellement installée du [langage de programmation Mojo](https://www.modular.com/mojo)

### Options

| Option              | Défaut                                | Description                                             |
| ------------------- | ------------------------------------- | ------------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'` | Format du module.                                       |
| `symbole`           | `'🔥 '`                               | Le symbole utilisé avant d'afficher la version de Mojo. |
| `style`             | `'bold 208'`                          | Le style pour le module.                                |
| `disabled`          | `false`                               | Désactive le module `mojo`.                             |
| `detect_extensions` | `['mojo', '🔥']`                      | Les extensions qui déclenchent ce module.               |
| `detect_files`      | `[]`                                  | Les fichiers qui activent ce module.                    |
| `detect_folders`    | `[]`                                  | Les dossiers qui activent ce module.                    |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `24.4.0` | La version de `mojo`                   |
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

Le module `nats` affiche le nom du contexte [NATS](https://nats.io) actuel.

### Options

| Option     | Défaut                     | Description                                                  |
| ---------- | -------------------------- | ------------------------------------------------------------ |
| `symbole`  | `'✉️ '`                    | Le symbole utilisé avant le contexte NATS (vide par défaut). |
| `style`    | `'bold purple'`            | Le style pour le module.                                     |
| `format`   | `'[$symbol$name]($style)'` | Format du module.                                            |
| `disabled` | `false`                    | Désactive le module `nats`.                                  |

### Variables

| Variable | Exemple     | Description                            |
| -------- | ----------- | -------------------------------------- |
| name     | `localhost` | Le nom du contexte NATS                |
| symbole  |             | Reflète la valeur de l'option `symbol` |
| style\*  |             | Reflète la valeur de l'option `style`  |

### Exemple

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## Espace de noms réseau

Le module `netns` affiche l'espace de noms réseau actuel. Il utilise `ip netns identify` pour obtenir l'espace de noms réseau ; seuls les espaces de noms montés dans `/var/run/netns` seront détectés.

### Options

| Option     | Défaut                          | Description                                                         |
| ---------- | ------------------------------- | ------------------------------------------------------------------- |
| `format`   | `'[$symbol \[$name\]]($style)'` | Format du module.                                                   |
| `symbole`  | `'🛜 '`                         | Le symbole utilisé avant l'espace de noms réseau (vide par défaut). |
| `style`    | `'blue bold dimmed'`            | Le style pour le module.                                            |
| `disabled` | `false`                         | Désactive le module `netns`.                                        |

### Variables

| Variable | Exemple    | Description                              |
| -------- | ---------- | ---------------------------------------- |
| name     | `my-netns` | Le nom de l'espace de noms réseau actuel |
| symbole  |            | Reflète la valeur de l'option `symbol`   |
| style\*  |            | Reflète la valeur de l'option `style`    |

### Exemple

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
```

## Nim

Le module `nim` affiche la version de [Nim](https://nim-lang.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `nim.cfg`
- Le dossier courant contient un fichier avec l’extension `.nim`
- Le dossier courant contient un fichier avec l’extension `.nims`
- Le dossier courant contient un fichier avec l’extension `.nimble`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module                                                                           |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'👑 '`                              | Le symbole utilisé avant d'afficher la version de Nim.                                     |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['nim.cfg']`                        | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold yellow'`                      | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `nim`.                                                                 |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.2.0` | La version de `nim`                    |
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

Le module `nix_shell` affiche la version de l’environnement [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html). Ce module s’affichera quand vous serez à l’intérieur d’un environnement nix-shell.

### Options

| Option        | Défaut                                       | Description                                                                      |
| ------------- | -------------------------------------------- | -------------------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | Format du module.                                                                |
| `symbole`     | `'❄️ '`                                      | Une chaîne de format représentant le symbole de nix-shell.                       |
| `style`       | `'bold blue'`                                | Le style pour le module.                                                         |
| `impure_msg`  | `'impure'`                                   | Une chaîne de format affichée lorsque le shell est impur.                        |
| `pure_msg`    | `'pure'`                                     | Une chaîne de format affichée lorsque le shell est pur.                          |
| `unknown_msg` | `''`                                         | Une chaîne de format affichée lorsqu'il est inconnu si le shell est pur/impur.   |
| `disabled`    | `false`                                      | Désactive le module `nix_shell`.                                                 |
| `heuristic`   | `false`                                      | Tente de détecter les nouveaux shells de style `nix shell` avec une heuristique. |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| state    | `pure`  | L’état du nix-shell                    |
| name     | `lorri` | Le nom du nix-shell                    |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

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

Le module `nodejs` affiche la version de [Node.js](https://nodejs.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `package.json`
- Le dossier courant contient un fichier `.node-version`
- Le dossier courant contient un fichier `.nvmrc`
- Le répertoire courant contient un répertoire `node_modules`
- Le dossier courant contient un fichier avec l’extension `.js`, `.mjs` ou `.cjs`
- Le dossier courant contient un fichier avec l’extension `.ts`, `.mts` ou `.cts`

De plus, le module sera masqué par défaut si le répertoire contient un fichier `bunfig.toml`, `bun.lock` ou `bun.lockb`, ce qui outrepasse les conditions ci-dessus.

### Options

| Option              | Défaut                                        | Description                                                                                                 |
| ------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | Format du module.                                                                                           |
| `version_format`    | `'v${raw}'`                                   | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch`                  |
| `symbole`           | `' '`                                        | Une chaîne de caractères représentant le symbole de Node.js.                                                |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | Les extensions qui déclenchent ce module.                                                                   |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | Les fichiers qui activent ce module.                                                                        |
| `detect_folders`    | `['node_modules']`                            | Les dossiers qui activent ce module.                                                                        |
| `style`             | `'bold green'`                                | Le style pour le module.                                                                                    |
| `disabled`          | `false`                                       | Désactive le module `nodejs`.                                                                               |
| `not_capable_style` | `'bold red'`                                  | Le style du module quand une propriété engines dans le package.json ne correspond pas à la version Node.js. |

### Variables

| Variable        | Exemple    | Description                                                                                                                                               |
| --------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version         | `v13.12.0` | La version de `node`                                                                                                                                      |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbole         |            | Reflète la valeur de l'option `symbol`                                                                                                                    |
| style\*         |            | Reflète la valeur de l'option `style`                                                                                                                     |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

Le module `ocaml` affiche la version de [OCaml](https://ocaml.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier avec l’extension `.opam` ou un dossier `_opam`
- Le répertoire courant contient un répertoire `esy.lock`
- Le dossier courant contient un fichier `dune` ou `dune-project`
- Le dossier courant contient un fichier `jbuild` ou `jbuild-ignore`
- Le dossier courant contient un fichier `.merlin`
- Le dossier courant contient un fichier avec l’extension `.ml`, `.mli`, `.re` ou `.rei`

### Options

| Option                    | Défaut                                                                   | Description                                                                                |
| ------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | La chaîne de format pour le module.                                                        |
| `version_format`          | `'v${raw}'`                                                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`                 | `'🐫 '`                                                                  | Le symbole utilisé avant d'afficher la version de OCaml.                                   |
| `global_switch_indicator` | `''`                                                                     | La chaîne de caractères utilisée pour représenter le commutateur OPAM global.              |
| `local_switch_indicator`  | `'*'`                                                                    | La chaîne de caractères utilisée pour représenter le commutateur OPAM local.               |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                     | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`         | Les fichiers qui activent ce module.                                                       |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                  | Les dossiers qui activent ce module.                                                       |
| `style`                   | `'bold yellow'`                                                          | Le style pour le module.                                                                   |
| `disabled`                | `false`                                                                  | Désactive le module `ocaml`.                                                               |

### Variables

| Variable         | Exemple      | Description                                                             |
| ---------------- | ------------ | ----------------------------------------------------------------------- |
| version          | `v4.10.0`    | La version de `ocaml`                                                   |
| switch_name      | `my-project` | Le switch OPAM actif                                                    |
| switch_indicator |              | Reflète la valeur de `indicator` pour le switch OPAM actuellement actif |
| symbole          |              | Reflète la valeur de l'option `symbol`                                  |
| style\*          |              | Reflète la valeur de l'option `style`                                   |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Odin

Le module `odin` affiche la version actuellement installée de [Odin](https://odin-lang.org/). Par défaut, le module sera affiché si le répertoire courant contient un fichier `.odin`.

### Options

| Option              | Défaut                               | Description                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                      |
| `show_commit`       | `false`                              | Affiche le commit comme partie de la version.          |
| `symbole`           | `'Ø '`                               | Le symbole utilisé avant d'afficher la version d'Odin. |
| `style`             | `'bold bright-blue'`                 | Le style pour le module.                               |
| `disabled`          | `false`                              | Désactive le module `odin`.                            |
| `detect_extensions` | `['odin']`                           | Les extensions qui déclenchent ce module.              |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                   |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                   |

### Variables

| Variable | Exemple       | Description                            |
| -------- | ------------- | -------------------------------------- |
| version  | `dev-2024-03` | La version d'`odin`                    |
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

Le module `opa` affiche la version actuellement installée de l'outil OPA. Par défaut, le module sera affiché si le répertoire courant contient un fichier `.rego`.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🪖  '`                             | Une chaîne de format représentant le symbole d'OPA.                                        |
| `detect_extensions` | `['rego']`                           | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold blue'`                        | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `opa`.                                                                 |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v0.44.0` | La version d'`opa`                     |
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

Le module `openstack` affiche le cloud et le projet OpenStack actuels. Le module n'est actif que lorsque la variable d'environnement `OS_CLOUD` est définie, auquel cas il lira le fichier `clouds.yaml` depuis l'un des [emplacements par défaut](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files) pour récupérer le projet en cours d'utilisation.

### Options

| Option     | Défaut                                        | Description                                                    |
| ---------- | --------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | Format du module.                                              |
| `symbole`  | `'☁️ '`                                       | Le symbole utilisé avant d'afficher le cloud OpenStack actuel. |
| `style`    | `'bold yellow'`                               | Le style pour le module.                                       |
| `disabled` | `false`                                       | Désactive le module `openstack`.                               |

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

Le module `os` affiche le système d'exploitation actuel. Les informations sur l'OS sont détectées via le crate [os_info](https://lib.rs/crates/os_info).

> [!WARNING] Le crate [os_info](https://lib.rs/crates/os_info) utilisé par ce module est connu pour être imprécis sur certains systèmes.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option     | Défaut                | Description                                                        |
| ---------- | --------------------- | ------------------------------------------------------------------ |
| `format`   | `'[$symbol]($style)'` | Format du module.                                                  |
| `style`    | `'bold white'`        | Le style pour le module.                                           |
| `disabled` | `true`                | Désactive le module `os`.                                          |
| `symbols`  |                       | Une table qui associe chaque système d'exploitation à son symbole. |

`symbols` vous permet de définir des symboles arbitraires à afficher pour chaque type de système d'exploitation. Les types de systèmes d'exploitation non définis par votre configuration utilisent la table de symboles par défaut ci-dessous. Tous les systèmes d'exploitation actuellement pris en charge par le module sont listés ci-dessous. Si vous souhaitez qu'un système d'exploitation soit ajouté, n'hésitez pas à ouvrir une [demande de fonctionnalité](https://github.com/starship/starship/issues/new/choose).

```toml
# Ceci est la table de symboles par défaut.
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

| Variable | Exemple      | Description                                                                   |
| -------- | ------------ | ----------------------------------------------------------------------------- |
| symbole  | `🎗️`         | Le symbole du système d'exploitation actuel depuis l'option avancée `symbols` |
| name     | `Arch Linux` | Le nom du système d'exploitation actuel                                       |
| type     | `Arch`       | Le type du système d'exploitation actuel                                      |
| codename |              | Le nom de code du système d'exploitation actuel, si applicable                |
| edition  |              | L'édition du système d'exploitation actuel, si applicable                     |
| version  |              | La version du système d'exploitation actuel, si applicable                    |
| style\*  |              | Reflète la valeur de l'option `style`                                         |

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

Le module `package` est affiché lorsque le répertoire courant est le dépôt d'un paquet et affiche sa version actuelle. Le module supporte actuellement les paquets `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `galaxy`, `daml` et `dart`.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – La version du paquet `npm` est extraite du `package.json` présent dans le répertoire courant
- [**JSR**](https://jsr.io/) – La version du paquet `jsr` est extraite du `jsr.json`/`jsr.jsonc` ou `deno.json`/`deno.jsonc` présent dans le répertoire courant
- [**Cargo**](https://doc.rust-lang.org/cargo/) – La version du paquet `cargo` est extraite du `Cargo.toml` présent dans le répertoire courant
- [**Nimble**](https://github.com/nim-lang/nimble) - La version du paquet `nimble` est extraite du fichier `*.nimble` dans le répertoire courant avec la commande `nimble dump`
- [**Poetry**](https://python-poetry.org/) – La version du paquet `poetry` est extraite du `pyproject.toml` présent dans le répertoire courant
- [**Python**](https://www.python.org) - La version du paquet `python` est extraite d'un `pyproject.toml` conforme à [PEP 621](https://peps.python.org/pep-0621/) ou d'un `setup.cfg` présent dans le répertoire courant
- [**Composer**](https://getcomposer.org/) – La version du paquet `composer` est extraite du `composer.json` présent dans le répertoire courant
- [**Gradle**](https://gradle.org/) – La version du paquet `gradle` est extraite du `build.gradle` présent dans le répertoire courant
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - La version du paquet est extraite du `Project.toml` présent dans le répertoire courant
- [**Mix**](https://hexdocs.pm/mix/) - La version du paquet `mix` est extraite du `mix.exs` présent dans le répertoire courant
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - La version du chart `helm` est extraite du `Chart.yaml` présent dans le répertoire courant
- [**Maven**](https://maven.apache.org/) - La version du paquet `maven` est extraite du `pom.xml` présent dans le répertoire courant
- [**Meson**](https://mesonbuild.com/) - La version du paquet `meson` est extraite du `meson.build` présent dans le répertoire courant
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - La version du paquet `shards` est extraite du `shard.yml` présent dans le répertoire courant
- [**Galaxy**](https://galaxy.ansible.com/) - La version du paquet `galaxy` est extraite du `galaxy.yml` présent dans le répertoire courant
- [**V**](https://vlang.io) - La version du paquet `vlang` est extraite du `v.mod` présent dans le répertoire courant
- [**SBT**](https://scala-sbt.org) - La version du paquet `sbt` est extraite du `build.sbt` présent dans le répertoire courant
- [**Daml**](https://www.digitalasset.com/developers) - La version du paquet `daml` est extraite du `daml.yaml` présent dans le répertoire courant
- [**Dart**](https://pub.dev/) – La version du paquet `dart` est extrait du `pubspec.yaml` présent dans le répertoire courant

> ⚠️ La version montrée est celle du paquet dont le code source est dans votre dossier courant, pas votre gestionnaire de paquet.

### Options

| Option            | Défaut                            | Description                                                                                |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`          | `'is [$symbol$version]($style) '` | Format du module.                                                                          |
| `symbole`         | `'📦 '`                           | Le symbole utilisé avant d'afficher la version du paquet.                                  |
| `version_format`  | `'v${raw}'`                       | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | Le style pour le module.                                                                   |
| `display_private` | `false`                           | Active l’affichage des versions des paquets marqués comme privés.                          |
| `disabled`        | `false`                           | Désactive le module `package`.                                                             |

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

Le module `perl` affiche la version de [Perl](https://www.perl.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `Makefile.PL` ou `Build.PL`
- Le dossier courant contient un fichier `cpanfile` ou `cpanfile.snapshot`
- Le dossier courant contient un fichier `META.json` ou `META.yml`
- Le dossier courant contient un fichier `.perl-version`
- Le répertoire courant contient un fichier `.pl`, `.pm` ou `.pod`

### Options

| Option              | Défaut                                                                                                   | Description                                                                                |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | La chaîne de format pour le module.                                                        |
| `version_format`    | `'v${raw}'`                                                                                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🐪 '`                                                                                                  | Le symbole utilisé avant d'afficher la version de Perl                                     |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                     | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold 149'`                                                                                             | Le style pour le module.                                                                   |
| `disabled`          | `false`                                                                                                  | Désactive le module `perl`.                                                                |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `v5.26.1` | La version de `perl`                   |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

Le module `php` affiche la version de [PHP](https://www.php.net/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `composer.json`
- Le dossier courant contient un fichier `.php-version`
- Le répertoire courant contient un fichier avec l'extension `.php`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🐘 '`                              | Le symbole utilisé avant d'afficher la version de PHP.                                     |
| `detect_extensions` | `['php']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['composer.json', '.php-version']`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `'147 bold'`                         | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `php`.                                                                 |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v7.3.8` | La version de `php`                    |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Canal Pijul

Le module `pijul_channel` affiche le canal actif du dépôt dans votre répertoire courant.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option              | Défaut                            | Description                                                                            |
| ------------------- | --------------------------------- | -------------------------------------------------------------------------------------- |
| `symbole`           | `' '`                            | Le symbole utilisé avant le nom du canal pijul du dépôt dans votre répertoire courant. |
| `style`             | `'bold purple'`                   | Le style pour le module.                                                               |
| `format`            | `'on [$symbol$channel]($style) '` | Format du module.                                                                      |
| `truncation_length` | `2^63 - 1`                        | Tronque le nom du canal pijul à `N` graphèmes                                          |
| `truncation_symbol` | `'…'`                             | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué.                   |
| `disabled`          | `true`                            | Désactive le module `pijul`.                                                           |

## Pixi

Le module `pixi` affiche la version installée de [pixi](https://pixi.sh) ainsi que l'environnement activé, si `$PIXI_ENVIRONMENT_NAME` est défini.

> [!TIP] Ceci ne supprime pas le modificateur de prompt propre à pixi, vous pouvez exécuter `pixi config set shell.change-ps1 false`.

### Options

| Option                     | Défaut                                                  | Description                                                                     |
| -------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------- |
| `format`                   | `'via [$symbol($version )(\($environment\) )]($style)'` | Format du module.                                                               |
| `version_format`           | `'v${raw}'`                                             | Le format de la version. Available vars are `raw`, `major`, `minor`, & `patch`. |
| `symbole`                  | `'🧚 '`                                                 | Le symbole utilisé avant le nom d'environnement.                                |
| `style`                    | `'yellow bold'`                                         | Le style pour le module.                                                        |
| `show_default_environment` | `true`                                                  | Indique si l'environnement `default` de votre projet est activé.                |
| `pixi_binary`              | `['pixi']`                                              | Configure le binaire pixi que Starship doit exécuter pour obtenir la version.   |
| `detect_extensions`        | `[]`                                                    | Les extensions qui déclenchent ce module.                                       |
| `detect_files`             | `['pixi.toml']`                                         | Les fichiers qui activent ce module.                                            |
| `detect_folders`           | `[]`                                                    | Les dossiers qui activent ce module.                                            |
| `disabled`                 | `false`                                                 | Désactive le module `pixi`.                                                     |

### Variables

| Variable    | Exemple   | Description                            |
| ----------- | --------- | -------------------------------------- |
| version     | `v0.33.0` | La version de `pixi`                   |
| environment | `py311`   | L'environnement pixi actuel            |
| symbole     |           | Reflète la valeur de l'option `symbol` |
| style       |           | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[pixi]
format = '[$symbol$environment](yellow) '
```

## Pulumi

Le module `pulumi` affiche le nom d'utilisateur actuel, la [Stack Pulumi](https://www.pulumi.com/docs/intro/concepts/stack/) sélectionnée, et la version.

> [!TIP] Par défaut, la version de Pulumi n’est pas affichée, car le chargement prend un ordre de grandeur plus long que la plupart des plugins (~70ms). Si vous voulez quand même l’activer, [suivez l’exemple montré plus bas](#with-pulumi-version).

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient soit un `Pulumi.yaml`, soit un `Pulumi.yml`
- Un répertoire parent contient soit un `Pulumi.yaml`, soit un `Pulumi.yml`, sauf si `search_upwards` est défini à `false`

### Options

| Option           | Défaut                                       | Description                                                                                |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | La chaîne de format pour le module.                                                        |
| `version_format` | `'v${raw}'`                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`        | `' '`                                       | Une chaîne de format affichée avant la stack Pulumi.                                       |
| `style`          | `'bold 5'`                                   | Le style pour le module.                                                                   |
| `search_upwards` | `true`                                       | Active la découverte des fichiers de configuration Pulumi dans les répertoires parents.    |
| `disabled`       | `false`                                      | Désactive le module `pulumi`.                                                              |

### Variables

| Variable          | Exemple    | Description                            |
| ----------------- | ---------- | -------------------------------------- |
| version           | `v0.12.24` | La version de `pulumi`                 |
| stack             | `dev`      | La stack Pulumi actuelle               |
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

Le module `purescript` affiche la version de [PureScript](https://www.purescript.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `spago.dhall`
- Le dossier courant contient un fichier `spago.yaml`
- Le dossier courant contient un fichier `spago.lock`
- Le dossier courant contient un fichier avec l’extension `.purs`

### Options

| Option              | Défaut                                        | Description                                                                                |
| ------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`          | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                                   | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'<=> '`                                      | Le symbole utilisé avant d'afficher la version de PureScript.                              |
| `detect_extensions` | `['purs']`                                    | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                          | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold white'`                                | Le style pour le module.                                                                   |
| `disabled`          | `false`                                       | Désactive le module `purescript`.                                                          |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `0.13.5` | La version de `purescript`             |
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

Le module `python` affiche la version de [Python](https://www.python.org/) installée et l’[environnement virtual Python](https://docs.python.org/tutorial/venv.html) actif s’il y en a un.

Si `pyenv_version_name` est défini à `true`, il affichera le nom de la version de pyenv. Sinon, il affichera le numéro de version que donne `python --version`.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `.python-version`
- Le dossier courant contient un fichier `Pipfile`
- Le dossier courant contient un fichier `__init__.py`
- Le dossier courant contient un fichier `pyproject.toml`
- Le dossier courant contient un fichier `requirements.txt`
- Le dossier courant contient un fichier `setup.py`
- Le dossier courant contient un fichier `tox.ini`
- Le dossier courant contient un fichier avec l’extension `.py`.
- Le dossier courant contient un fichier avec l'extension `.ipynb`.
- Un environnement virtuel est actuellement activé

### Options

| Option               | Défaut                                                                                                       | Description                                                                                |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                    | Format du module.                                                                          |
| `version_format`     | `'v${raw}'`                                                                                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`            | `'🐍 '`                                                                                                      | Une chaîne de caractères représentant le symbole de Python                                 |
| `style`              | `'yellow bold'`                                                                                              | Le style pour le module.                                                                   |
| `pyenv_version_name` | `false`                                                                                                      | Utiliser pyenv pour obtenir la version de Python                                           |
| `pyenv_prefix`       | `'pyenv'`                                                                                                    | Préfixe avant l'affichage de la version pyenv, utilisé uniquement si pyenv est utilisé     |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configure les binaires Python que Starship doit exécuter pour obtenir la version.          |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                            | Les extensions qui déclenchent ce module                                                   |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`     | `[]`                                                                                                         | Quels dossiers devraient activer ce module                                                 |
| `generic_venv_names` | `[]`                                                                                                         | Quels noms de venv doivent être remplacés par le nom du répertoire parent.                 |
| `disabled`           | `false`                                                                                                      | Désactive le module `python`.                                                              |

> [!TIP] La variable `python_binary` accepte soit une chaîne de caractères, soit une liste de chaînes. Starship essayera d'exécuter chaque binaire jusqu'à obtenir un résultat. Notez que vous ne pouvez modifier que le binaire que Starship exécute pour obtenir la version de Python, pas les arguments utilisés.
>
> Les valeurs et l'ordre par défaut de `python_binary` ont été choisis pour d'abord identifier la version de Python dans les environnements virtualenv/conda (qui ajoutent toujours un `python`, qu'il pointe vers `python3` ou `python2`). Cela a pour effet secondaire que si vous avez encore un Python 2 système installé, il peut être détecté avant tout Python 3 (du moins sur les distributions Linux qui font toujours un lien symbolique de `/usr/bin/python` vers Python 2). Si vous n'utilisez plus Python 2 mais ne pouvez pas supprimer le Python 2 système, changer cette option en `'python3'` masquera toute version de Python 2, voir l'exemple ci-dessous.

### Variables

| Variable     | Exemple         | Description                                                                      |
| ------------ | --------------- | -------------------------------------------------------------------------------- |
| version      | `'v3.8.1'`      | La version de `python`                                                           |
| symbole      | `'🐍 '`         | Reflète la valeur de l'option `symbol`                                           |
| style        | `'yellow bold'` | Reflète la valeur de l'option `style`                                            |
| pyenv_prefix | `'pyenv '`      | Reflète la valeur de l'option `pyenv_prefix`                                     |
| virtualenv   | `'venv'`        | Le nom du `virtualenv` courant ou le parent si correspond à `generic_venv_names` |

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

## Quarto

Le module `quarto` affiche la version actuellement installée de Quarto utilisée dans un projet.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le répertoire courant contient un fichier `_quarto.yml`
- Le répertoire courant contient un fichier `*.qmd`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'⨁ '`                               | Une chaîne de format représentant le symbole de Quarto                                     |
| `style`             | `'bold #75AADB'`                     | Le style pour le module.                                                                   |
| `detect_extensions` | `['.qmd']`                           | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['_quarto.yml']`                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`          | `false`                              | Désactive le module `quarto`.                                                              |

### Variables

| Variable | Exemple   | Description                            |
| -------- | --------- | -------------------------------------- |
| version  | `1.4.549` | La version de `quarto`                 |
| symbole  |           | Reflète la valeur de l'option `symbol` |
| style\*  |           | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## R

Le module `rlang` affiche la version de [R](https://www.r-project.org/) installée. Le module sera affiché si l'une de ces conditions est remplie:

- Le dossier courant contient un fichier avec l’extension `.R`.
- Le dossier courant contient un fichier avec l’extension `.Rd`.
- Le dossier courant contient un fichier avec l’extension `.Rmd`.
- Le dossier courant contient un fichier avec l’extension `.Rproj`.
- Le dossier courant contient un fichier avec l’extension `.Rsx`.
- Le dossier courant contient un fichier `.Rprofile`
- Le répertoire courant contient un dossier `.Rproj.user`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'📐'`                               | Une chaîne de caractères représentant le symbole de R.                                     |
| `style`             | `'blue bold'`                        | Le style pour le module.                                                                   |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Les extensions qui déclenchent ce module                                                   |
| `detect_files`      | `['.Rprofile']`                      | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`    | `['.Rproj.user']`                    | Quels dossiers devraient activer ce module                                                 |
| `disabled`          | `false`                              | Désactive le module `r`.                                                                   |

### Variables

| Variable | Exemple       | Description                            |
| -------- | ------------- | -------------------------------------- |
| version  | `v4.0.5`      | La version de `R`                      |
| symbole  |               | Reflète la valeur de l'option `symbol` |
| style    | `'blue bold'` | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

Le module `raku` affiche la version actuellement installée de [Raku](https://www.raku.org/). Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le répertoire courant contient un fichier `META6.json`
- Le répertoire courant contient un fichier `.p6`, `.pm6`, `.raku`, `.rakumod` ou `.pod6`

### Options

| Option              | Défaut                                           | Description                                                                                |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | La chaîne de format pour le module.                                                        |
| `version_format`    | `'v${raw}'`                                      | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🦋 '`                                          | Le symbole utilisé avant d'afficher la version de Raku                                     |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['META6.json']`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                             | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold 149'`                                     | Le style pour le module.                                                                   |
| `disabled`          | `false`                                          | Désactive le module `raku`.                                                                |

### Variables

| Variable   | Exemple | Description                                           |
| ---------- | ------- | ----------------------------------------------------- |
| version    | `v6.d`  | La version de `raku`                                  |
| vm_version | `moar`  | La version de la VM sur laquelle `raku` est construit |
| symbole    |         | Reflète la valeur de l'option `symbol`                |
| style\*    |         | Reflète la valeur de l'option `style`                 |

### Exemple

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

Par défaut, le module `red` affiche la version de [Red](https://www.red-lang.org/) installée. Le module sera affiché si l'une de ces conditions est remplie:

- Le dossier courant contient un fichier avec l’extension `.red` ou `.reds`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🔺 '`                              | Une chaîne de caractères représentant le symbole de Red.                                   |
| `detect_extensions` | `['red']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `'red bold'`                         | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `red`.                                                                 |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.5.1` | La version de `red`                    |
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

Le module `ruby` affiche la version de [Ruby](https://www.ruby-lang.org/) installée. Le module sera affiché si l'une de ces conditions est remplie:

- Le dossier courant contient un fichier `Gemfile`
- Le dossier courant contient un fichier `.ruby-version`
- Le dossier courant contient un fichier `.rb`
- La variable d’environnement `RUBY_VERSION` ou `RBENV_VERSION` est définie

Starship obtient la version actuelle de Ruby en exécutant `ruby -v`.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'💎 '`                              | Une chaîne de caractères représentant le symbole de Ruby.                                  |
| `detect_extensions` | `['rb']`                             | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Les variables d’environnement qui activent ce module.                                      |
| `style`             | `'bold red'`                         | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `ruby`.                                                                |

### Variables

| Variable | Exemple  | Description                                      |
| -------- | -------- | ------------------------------------------------ |
| version  | `v2.5.1` | La version de `ruby`                             |
| symbole  |          | Reflète la valeur de l'option `symbol`           |
| style\*  |          | Reflète la valeur de l'option `style`            |
| gemset   | `test`   | Optionnel, récupère le nom du gemset RVM actuel. |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

Le module `rust` affiche la version de [Rust](https://www.rust-lang.org/) installée. Le module sera affiché si l'une de ces conditions est remplie:

- Le dossier courant contient un fichier `Cargo.toml`
- Le dossier courant contient un fichier avec l’extension `.rs`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🦀 '`                              | Une chaîne de caractères représentant le symbole de Rust                                   |
| `detect_extensions` | `['rs']`                             | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['Cargo.toml']`                     | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold red'`                         | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `rust`.                                                                |

### Variables

| Variable  | Exemple           | Description                                      |
| --------- | ----------------- | ------------------------------------------------ |
| version   | `v1.43.0-nightly` | La version de `rustc`                            |
| numver    | `1.51.0`          | La composante numérique de la version de `rustc` |
| toolchain | `beta`            | La version de la toolchain                       |
| symbole   |                   | Reflète la valeur de l'option `symbol`           |
| style\*   |                   | Reflète la valeur de l'option `style`            |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

Le module `scale` affiche la version de [Scala](https://www.scala-lang.org/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `build.sbt`, `.scalaenv` ou `.sbtenv`
- Le dossier courant contient un fichier avec l’extension `.scala` ou `.sbt`
- Le répertoire courant contient un répertoire nommé `.metals`

### Options

| Option              | Défaut                                   | Description                                                                                |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [${symbol}(${version} )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `['.metals']`                            | Quels dossiers devraient activer ce module.                                                |
| `symbole`           | `'🆂 '`                                   | Une chaîne de caractères représentant le symbole de Scala.                                 |
| `style`             | `'red dimmed'`                           | Le style pour le module.                                                                   |
| `disabled`          | `false`                                  | Désactive le module `scala`.                                                               |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `2.13.5` | La version de `scala`                  |
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

Le module `shell` affiche un indicateur en fonction du shell actuellement utilisé.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option                 | Défaut                    | Description                                                                                                            |
| ---------------------- | ------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `bash_indicator`       | `'bsh'`                   | Chaine de formatage utilisée pour représenter bash.                                                                    |
| `fish_indicator`       | `'fsh'`                   | Chaine de formatage utilisée pour représenter fish.                                                                    |
| `zsh_indicator`        | `'zsh'`                   | Chaine de formatage utilisée pour représenter zsh.                                                                     |
| `powershell_indicator` | `'psh'`                   | Chaine de formatage utilisée pour représenter powershell.                                                              |
| `pwsh_indicator`       |                           | Une chaîne de format utilisée pour représenter pwsh. La valeur par défaut reflète la valeur de `powershell_indicator`. |
| `ion_indicator`        | `'ion'`                   | Chaine de formatage utilisée pour représenter ion.                                                                     |
| `elvish_indicator`     | `'esh'`                   | Chaine de formatage utilisée pour représenter elvish.                                                                  |
| `tcsh_indicator`       | `'tsh'`                   | Chaine de formatage utilisée pour représenter tcsh.                                                                    |
| `xonsh_indicator`      | `'xsh'`                   | Chaine de formatage utilisée pour représenter xonsh.                                                                   |
| `cmd_indicator`        | `'cmd'`                   | Chaine de formatage utilisée pour représenter cmd.                                                                     |
| `nu_indicator`         | `'nu'`                    | Chaine de formatage utilisée pour représenter nu.                                                                      |
| `unknown_indicator`    | `''`                      | La valeur par défaut à afficher quand le shell est inconnu.                                                            |
| `format`               | `'[$indicator]($style) '` | Format du module.                                                                                                      |
| `style`                | `'white bold'`            | Le style pour le module.                                                                                               |
| `disabled`             | `true`                    | Désactive le module `shell`.                                                                                           |

### Variables

| Variable  | Défaut | Description                                       |
| --------- | ------ | ------------------------------------------------- |
| indicator |        | Reflète la valeur de `indicator` du shell actuel. |
| style\*   |        | Reflète la valeur de l'option `style`.            |

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

Le module `shlvl` affiche la variable d'environnement [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('niveau de shell') actuelle, si elle est définie comme un nombre et atteint ou dépasse le seuil spécifié.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option          | Défaut                       | Description                                                             |
| --------------- | ---------------------------- | ----------------------------------------------------------------------- |
| `threshold`     | `2`                          | Seuil d’affichage.                                                      |
| `format`        | `'[$symbol$shlvl]($style) '` | Format du module.                                                       |
| `symbole`       | `'↕️  '`                     | Le symbole utilisée pour représenter le `SHLVL`.                        |
| `repeat`        | `false`                      | Fait répéter `symbol` autant de fois que la valeur actuelle de `SHLVL`. |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value     |
| `style`         | `'bold yellow'`              | Le style pour le module.                                                |
| `disabled`      | `true`                       | Désactive le module `shlvl`.                                            |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| shlvl    | `3`     | La valeur actuelle de `SHLVL`          |
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

En utilisant `repeat` et `repeat_offset` avec le module `character`, on peut obtenir un prompt comme `❯❯❯` où le dernier caractère est coloré selon le code de retour et les caractères précédents sont fournis par `shlvl`.

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

Le module `singularity` affiche l’image [Singularity](https://sylabs.io/singularity/) courante, quand vous êtes à l’intérieur d’un conteneur et que `$SINGULARITY_NAME` est définie.

### Options

| Option     | Défaut                         | Description                                            |
| ---------- | ------------------------------ | ------------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Format du module.                                      |
| `symbole`  | `''`                           | Une chaîne de format affichée avant le nom de l'image. |
| `style`    | `'bold dimmed blue'`           | Le style pour le module.                               |
| `disabled` | `false`                        | Désactive le module `singularity`.                     |

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

Le module `solidity` affiche la version actuellement installée de [Solidity](https://soliditylang.org/). Le module sera affiché si l'une des conditions suivantes est remplie :

- Le répertoire courant contient un fichier avec l'extension `.sol`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${major}.${minor}.${patch}'`      | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'S '`                               | Une chaîne de format représentant le symbole de Solidity                                   |
| `compiler           | ['solc']                             | Le compilateur par défaut pour Solidity.                                                   |
| `detect_extensions` | `['sol']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold blue'`                        | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive ce module.                                                                       |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v0.8.1` | La version de `solidity`               |
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

Le module `spack` affiche l’environnement [Spack](https://spack.readthedocs.io/en/latest/) courant, si `$SPACK_ENV` est définie.

### Options

| Option              | Défaut                                 | Description                                                                                                                                                        |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `1`                                    | Le nombre de dossiers auxquels le chemin d’environnement doit être tronqué. `0` ne signifie pas de troncature. Regardez aussi le module [`directory`](#directory). |
| `symbole`           | `'🅢  '`                                | Le symbole utilisé avant le nom d'environnement.                                                                                                                   |
| `style`             | `'bold blue'`                          | Le style pour le module.                                                                                                                                           |
| `format`            | `'via [$symbol$environment]($style) '` | Format du module.                                                                                                                                                  |
| `disabled`          | `false`                                | Désactive le module `spack`.                                                                                                                                       |

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

Le module `status` affiche le code de sortie de la commande précédente. Si $success_symbol est vide (par défaut), ce module sera affiché uniquement quand le code de sortie n’est pas `0`. Le code de statut est converti en entier signé 32 bits.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option                      | Défaut                                                                         | Description                                                                        |
| --------------------------- | ------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                  | Le format du module                                                                |
| `symbole`                   | `'❌'`                                                                         | Le symbole affiché lors d'une erreur de programme                                  |
| `success_symbol`            | `''`                                                                           | Le symbole affiché lors d'un succès de programme                                   |
| `not_executable_symbol`     | `'🚫'`                                                                         | Le symbole affiché lorsque le fichier n'est pas exécutable                         |
| `not_found_symbol`          | `'🔍'`                                                                         | Le symbole affiché lorsque la commande est introuvable                             |
| `sigint_symbol`             | `'🧱'`                                                                         | Le symbole affiché lors d'un SIGINT (Ctrl + c)                                     |
| `signal_symbol`             | `'⚡'`                                                                         | Le symbole affiché lors de n'importe quel signal                                   |
| `style`                     | `'bold red'`                                                                   | Le style pour le module.                                                           |
| `success_style`             |                                                                                | Le style utilisé en cas de succès du programme (par défaut `style` si non défini). |
| `failure_style`             |                                                                                | Le style utilisé en cas d'échec du programme (par défaut `style` si non défini).   |
| `recognize_signal_code`     | `true`                                                                         | Active la correspondance des signaux à partir du code de sortie                    |
| `map_symbol`                | `false`                                                                        | Active la correspondance des symboles à partir du code de sortie                   |
| `pipestatus`                | `false`                                                                        | Active le rapport pipestatus                                                       |
| `pipestatus_separator`      | <code>&vert;</code>                                                            | Le symbole utilisé pour séparer les segments pipestatus (supporte le formatage)    |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | Le format du module lorsque la commande est un pipeline                            |
| `pipestatus_segment_format` |                                                                                | Lorsque spécifié, remplace `format` lors du formatage des segments pipestatus      |
| `disabled`                  | `true`                                                                         | Désactiver le module `status`.                                                     |

### Variables

| Variable       | Exemple | Description                                                                                             |
| -------------- | ------- | ------------------------------------------------------------------------------------------------------- |
| statut         | `127`   | Le code de sortie de la dernière commande                                                               |
| hex_status     | `0x7F`  | Le code de sortie de la dernière commande en hexa                                                       |
| int            | `127`   | Le code de sortie de la dernière commande                                                               |
| common_meaning | `ERROR` | Signification du code si n’est pas un signal                                                            |
| signal_number  | `9`     | Numéro du signal correspondant au code de sortie, uniquement si un signal a été reçu                    |
| signal_name    | `KILL`  | Nom du signal correspondant au code de sortie, uniquement si un signal a été reçu                       |
| maybe_int      | `7`     | Contient le numéro du code de sortie quand aucune signification n'a été trouvée                         |
| pipestatus     |         | Rendu des codes de sortie des programmes dans un pipeline, disponible uniquement dans pipestatus_format |
| symbole        |         | Reflète la valeur de l'option `symbol`                                                                  |
| style\*        |         | Reflète la valeur de l'option `success_style` en cas de succès du programme et `failure_style` sinon    |

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

Le moduel `sudo` affiche si les identifiants sudo sont actuellement en cache. Le module sera uniquement affiché si les identifiants sont en cache.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option          | Défaut                   | Description                                                       |
| --------------- | ------------------------ | ----------------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | Le format du module                                               |
| `symbole`       | `'🧙 '`                  | Le symbole affiché quand les identifiants sont en cache           |
| `style`         | `'bold blue'`            | Le style pour le module.                                          |
| `allow_windows` | `false`                  | Puisque Windows n’a pas de sudo par défaut, désactivé par défaut. |
| `disabled`      | `true`                   | Désactive le module `sudo`.                                       |

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

Par défaut, le module `swift` affiche la version de [Swift](https://swift.org/) installée. Le module sera affiché si l'une de ces conditions est remplie:

- Le dossier courant contient un fichier `Package.swift`
- Le dossier courant contient un fichier avec l’extension `.swift`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'🐦 '`                              | Une chaîne de caractères représentant le symbole de Swift                                  |
| `detect_extensions` | `['swift']`                          | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['Package.swift']`                  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold 202'`                         | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactiver le module `swift`.                                                              |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v5.2.4` | La version de `swift`                  |
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

Le module `terraform` affiche [l’espace de travail Terraform](https://www.terraform.io/docs/language/state/workspaces.html) sélectionné et sa version. Il supporte à la fois Hashicorp Terraform et OpenTofu pour la détection de version.

> [!TIP] Par défaut, la version de Terraform/OpenTofu n’est pas affichée, car c’est lent avec les versions actuelles quand beaucoup de plugins sont utilisés. Si vous voulez quand même l’activer, [suivez l’exemple montré plus bas](#with-terraform-version).

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un dossier `.terraform`
- Le dossier courant contient un fichier avec l’extension `.tf`, `.tfplan` ou `.tfstate`

### Options

| Option              | Défaut                                                  | Description                                                                                |
| ------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol$workspace]($style) '`                    | La chaîne de format pour le module.                                                        |
| `version_format`    | `'v${raw}'`                                             | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'💠'`                                                  | Une chaîne de format montrée avant l'espace de travail terraform.                          |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`                           | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `['.terraform']`                                        | Les dossiers qui activent ce module.                                                       |
| `style`             | `'bold 105'`                                            | Le style pour le module.                                                                   |
| `disabled`          | `false`                                                 | Désactive le module `terraform`.                                                           |
| `commands`          | `[ [ 'terraform', 'version' ], [ 'tofu', 'version' ] ]` | Comment détecter la version de Terraform.                                                  |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | La version de `terraform`              |
| workspace | `default`  | L’espace de travail Terraform courant  |
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

Le module `time` affiche la date et heure **locale**. La valeur de `format` est utilisée par le package [`chrono`](https://crates.io/crates/chrono) pour contrôler la façon dont l'heure est affichée. Consultez la [doc de chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) pour découvrir les options disponibles.

> [!TIP] Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

### Options

| Option            | Défaut                  | Description                                                                                                                                               |
| ----------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | La chaîne de format pour le module.                                                                                                                       |
| `use_12hr`        | `false`                 | Activer le format 12h                                                                                                                                     |
| `time_format`     | voir plus bas           | Le [format chrono](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) utilisé pour formater l'heure.                                         |
| `style`           | `'bold yellow'`         | Le style utilisé par le module                                                                                                                            |
| `utc_time_offset` | `'local'`               | Définir le décalage horaire UTC à utiliser. Intervalle de -24 &lt; x &lt; 24. Accepte des nombres décimaux pour s'adapter aux décalages de 30/45 minutes. |
| `disabled`        | `true`                  | Désactive le module `time`.                                                                                                                               |
| `time_range`      | `'-'`                   | Définit la plage de temps pendant laquelle le module sera affiché. Les heures doivent être spécifiées au format 24 heures                                 |

Si `use_12hr` est `true`, alors `time_format` prend par défaut la valeur `'%r'`. Sinon, il prend par défaut la valeur `'%T'`. Définir manuellement `time_format` outrepasse le paramètre `use_12hr`.

### Variables

| Variable | Exemple    | Description                           |
| -------- | ---------- | ------------------------------------- |
| time     | `13:08:10` | L’heure actuelle.                     |
| style\*  |            | Reflète la valeur de l'option `style` |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

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

## Typst

Le module `typst` affiche la version actuellement installée de Typst utilisée dans un projet.

Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `template.typ`
- Le répertoire courant contient un fichier `*.typ`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'t '`                               | Une chaîne de format représentant le symbole de Typst                                      |
| `style`             | `'bold #0093A7'`                     | Le style pour le module.                                                                   |
| `detect_extensions` | `['.typ']`                           | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['template.typ']`                   | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`          | `false`                              | Désactive le module `typst`.                                                               |

### Variables

| Variable      | Exemple   | Description                                     |
| ------------- | --------- | ----------------------------------------------- |
| version       | `v0.9.0`  | La version de `typst`, alias pour typst_version |
| typst_version | `default` | La version Typst actuelle                       |
| symbole       |           | Reflète la valeur de l'option `symbol`          |
| style\*       |           | Reflète la valeur de l'option `style`           |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Nom d'utilisateur

Le module `username` affiche le nom de l’utilisateur actif. Le module sera affiché si l'une de ces conditions est remplie:

- L'utilisateur courant est root/admin
- L'utilisateur courant est différent de celui connecté
- L'utilisateur est actuellement connecté en tant que session SSH
- La variable `show_always` est définie à true
- Le tableau `detect_env_vars` contient au moins le nom d'une variable d'environnement qui est définie

> [!TIP] La connexion SSH est détectée en vérifiant les variables d'environnement `SSH_CONNECTION`, `SSH_CLIENT` et `SSH_TTY`. Si votre hôte SSH ne définit pas ces variables, une solution de contournement consiste à en définir une avec une valeur fictive.

### Options

| Option            | Défaut                  | Description                                                    |
| ----------------- | ----------------------- | -------------------------------------------------------------- |
| `style_root`      | `'bold red'`            | Le style utilisé quand l'utilisateur est root/admin.           |
| `style_user`      | `'bold yellow'`         | Le style utilisé pour les utilisateurs non-root.               |
| `detect_env_vars` | `[]`                    | Quelles variables d'environnement devraient activer ce module. |
| `format`          | `'[$user]($style) in '` | Format du module.                                              |
| `show_always`     | `false`                 | Toujours afficher le module `username`.                        |
| `disabled`        | `false`                 | Désactive le module `username`.                                |
| `aliases`         | `{}`                    | Translate system usernames to something else.                  |

### Variables

| Variable | Exemple      | Description                                                                               |
| -------- | ------------ | ----------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | Reflète la valeur de l'option `style_root` quand root est connecté et `style_user` sinon. |
| `user`   | `'matchai'`  | L’identifiant de l’utilisateur courant.                                                   |

### Exemple

#### Toujours afficher le nom d'utilisateur

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

Le module `vagrant` affiche la version de [Vagrant](https://www.vagrantup.com/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `Vagrantfile`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'⍱ '`                               | Une chaîne de caractères représentant le symbole de Vagrant.                               |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['Vagrantfile']`                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `'cyan bold'`                        | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `vagrant`.                                                             |

### Variables

| Variable | Exemple          | Description                            |
| -------- | ---------------- | -------------------------------------- |
| version  | `Vagrant 2.2.10` | La version de `Vagrant`                |
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

Le module `vlang` affiche la version de [V](https://vlang.io/) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier avec l’extension `.v`
- Le dossier courant contient un fichier `v.mod`, `vpkg.json` ou `.vpkg-lock.json`

### Options

| Option              | Défaut                                       | Description                                                                                |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`         | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'V '`                                       | Une chaîne de caractères représentant le symbole de V                                      |
| `detect_extensions` | `['v']`                                      | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                         | Les dossiers qui activent ce module.                                                       |
| `style`             | `'blue bold'`                                | Le style pour le module.                                                                   |
| `disabled`          | `false`                                      | Désactive le module `vlang`.                                                               |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| version  | `v0.2`  | La version de `v`                      |
| symbole  |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCS

> Notez que le module est activé par défaut mais **n'est pas** inclus dans la liste par défaut car cela constituerait un changement cassant.
> De plus, le format exact du module pourrait changer à l'avenir, par exemple pour gérer l'invite alignée à droite.

Le module `vcs` affiche le système de contrôle de version (VCS) actif actuel.
Le module ne sera affiché que si un VCS configuré est actuellement utilisé.

### Options

| Option           | Défaut                                                      | Description                                                 |
| ---------------- | ----------------------------------------------------------- | ----------------------------------------------------------- |
| `order`          | `["git", "hg", "pijul", "fossil"]`                          | L'ordre dans lequel rechercher les VCS.                     |
| `fossil_modules` | `"$fossil_branch$fossil_metrics"`                           | Les modules à afficher quand un dépôt Fossil est trouvé.    |
| `git_modules`    | `"$git_branch$git_commit$git_state$git_metrics$git_status"` | Les modules à afficher quand un dépôt Git est trouvé.       |
| `hg_modules`     | `"$hg_branch$hg_state"`                                     | Les modules à afficher quand un dépôt Mercurial est trouvé. |
| `pijul_modules`  | `"$pijul_channel"`                                          | Les modules à afficher quand un dépôt Pijul est trouvé.     |
| `disabled`       | `false`                                                     | Désactive le module `vcs`.                                  |

### Exemple

```toml
# ~/.config/starship.toml

[vcs]
# Cherchera Git puis Pijul si non trouvé mais pas les autres VCS
order = [
  "git",
  "pijul",
]
# N'importe quel module (sauf `$vcs` lui-même pour éviter les boucles infinies) peut être inclus ici
git_modules = "$git_branch${custom.foo}"

# Voir la documentation pour les modules personnalisés
[custom.foo]
command = 'echo foo'
detect_files = ['foo']
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'
```

## VCSH

Le module `vcsh` affiche le dépôt [VCSH](https://github.com/RichiH/vcsh) actif actuel. Le module ne sera affiché que si un dépôt est actuellement utilisé.

### Options

| Option     | Défaut                           | Description                                          |
| ---------- | -------------------------------- | ---------------------------------------------------- |
| `symbole`  | `''`                             | Le symbole utilisé avant d'afficher le nom du dépôt. |
| `style`    | `'bold yellow'`                  | Le style pour le module.                             |
| `format`   | `'vcsh [$symbol$repo]($style) '` | Format du module.                                    |
| `disabled` | `false`                          | Désactive le module `vcsh`.                          |

### Variables

| Variable | Exemple                                         | Description                            |
| -------- | ----------------------------------------------- | -------------------------------------- |
| repo     | `dotfiles` si dans un dépôt VCSH nommé dotfiles | Le nom du dépôt actif                  |
| symbole  |                                                 | Reflète la valeur de l'option `symbol` |
| style\*  | `black bold dimmed`                             | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## XMake

Le module `xmake` affiche la version actuellement installée de [XMake](https://xmake.io/). Par défaut, le module s’activera si l’une de ces conditions est remplie:

- Le dossier courant contient un fichier `xmake.lua`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'△ '`                               | Le symbole utilisé avant la version de cmake.                                              |
| `detect_extensions` | `[]`                                 | Les extensions qui déclenchent ce module                                                   |
| `detect_files`      | `['xmake.lua']`                      | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module                                                 |
| `style`             | `'bold green'`                       | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `xmake`.                                                               |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.9.5` | La version de xmake                    |
| symbole  |          | Reflète la valeur de l'option `symbol` |
| style\*  |          | Reflète la valeur de l'option `style`  |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

## Zig

Par défaut, le module `zig` affiche la version actuellement installée de [Zig](https://ziglang.org/). Le module sera affiché si l'une de ces conditions est remplie:

- Le dossier courant contient un fichier `.zip`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | Format du module.                                                                          |
| `version_format`    | `'v${raw}'`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbole`           | `'↯ '`                               | Le symbole utilisé avant d'afficher la version de Zig.                                     |
| `style`             | `'bold yellow'`                      | Le style pour le module.                                                                   |
| `disabled`          | `false`                              | Désactive le module `zig`.                                                                 |
| `detect_extensions` | `['zig']`                            | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |

### Variables

| Variable | Exemple  | Description                            |
| -------- | -------- | -------------------------------------- |
| version  | `v0.6.0` | La version de `zip`                    |
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

Les modules `custom` affichent la sortie d’une commande arbitaitre.

Ces modules seront affichés si l'une de ces conditions est remplie:

- Le dossier courant contient un fichier dont le nom est dans `detect_files`
- Le dossier courant contient un dossier dont le nom est dans `detect_folders`
- Le dossier courant contient un fichier dont l’extension est dans `detect_extensions`
- La commande `when` retourne 0
- Le système d'exploitation actuel (std::env::consts::OS) correspond au champ `os` si défini.

> [!TIP] Plusieurs modules personnalisés peuvent être définis en utilisant un `.`.

> [!TIP] L'ordre dans lequel les modules personnalisés sont affichés peut être défini individuellement en incluant `${custom.foo}` dans le `format` de niveau supérieur (comme il contient un point, vous devez utiliser `${...}`). Par défaut, le module `custom` affichera simplement tous les modules personnalisés dans l'ordre dans lequel ils ont été définis.

> [!TIP] [L'Issue #1252](https://github.com/starship/starship/discussions/1252) contient des exemples de modules personnalisés. Si vous avez un exemple intéressant qui n'y est pas couvert, n'hésitez pas à le partager !

> [!WARNING] Si `unsafe_no_escape` est activé ou avant starship v1.20, la sortie de la commande est affichée sans échappement dans l'invite.
>
> Toute sortie générée par la commande est affichée telle quelle dans l'invite. Cela signifie que si la sortie contient des séquences interprétables spécifiques au shell, elles pourraient être interprétées à l'affichage. Selon le shell, cela peut signifier par exemple que les chaînes entre backticks sont exécutées par le shell. Ces séquences sont généralement spécifiques au shell, par ex. vous pouvez écrire un module de commande qui produit des séquences bash, par ex. `\h`, mais ce module ne fonctionnera pas dans un shell fish ou zsh.
>
> Les chaînes de format peuvent aussi contenir des séquences d'invite spécifiques au shell, par ex. [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

### Options

| Option              | Défaut                          | Description                                                                                                                                                                                                                                                                                                                          |
| ------------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `command`           | `''`                            | La commande dont la sortie doit être affichée. La commande sera transmise au shell sur l’entrée standard.                                                                                                                                                                                                                            |
| `when`              | `false`                         | Soit une valeur booléenne (`true` ou `false`, sans guillemets) ou une commande shell utilisée comme condition pour afficher le module. Dans le cas d'une chaîne, le module sera affiché si le `shell` retourne un code de statut `0` en l'exécutant.                                                                                 |
| `require_repo`      | `false`                         | Si `true`, le module ne sera affiché que dans les chemins contenant un dépôt (git). Cette option seule n'est pas une condition d'affichage suffisante en l'absence d'autres options.                                                                                                                                                 |
| `shell`             |                                 | [Voir plus bas](#custom-command-shell)                                                                                                                                                                                                                                                                                               |
| `description`       | `'<custom module>'`             | La description du module qui est affichée lors de l’exécution de `starship explain`.                                                                                                                                                                                                                                                 |
| `unsafe_no_escape`  | `false`                         | Quand activé, la sortie de la commande n'est pas échappée des caractères qui pourraient être interprétés par le shell.                                                                                                                                                                                                               |
| `detect_files`      | `[]`                            | Les fichiers qui seront recherchés dans le répertoire de travail pour une correspondance.                                                                                                                                                                                                                                            |
| `detect_folders`    | `[]`                            | Les répertoires qui seront recherchés dans le répertoire de travail pour une correspondance.                                                                                                                                                                                                                                         |
| `detect_extensions` | `[]`                            | Les extensions qui seront recherchées dans le répertoire de travail pour une correspondance.                                                                                                                                                                                                                                         |
| `symbole`           | `''`                            | Le symbole utilisé avant d'afficher la sortie de la commande.                                                                                                                                                                                                                                                                        |
| `style`             | `'bold green'`                  | Le style pour le module.                                                                                                                                                                                                                                                                                                             |
| `format`            | `'[$symbol($output )]($style)'` | Format du module.                                                                                                                                                                                                                                                                                                                    |
| `disabled`          | `false`                         | Désactive le module `custom`.                                                                                                                                                                                                                                                                                                        |
| `os`                |                                 | Nom du système d'exploitation sur lequel le module sera affiché (unix, linux, macos, windows, ... ) [Voir les valeurs possibles](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                                         |
| `use_stdin`         |                                 | Une valeur booléenne optionnelle qui détermine si les commandes doivent être transmises au shell via l'entrée standard ou comme argument. Si non défini, l'entrée standard est utilisée par défaut, sauf si le shell ne la supporte pas (cmd, nushell). Définir cette option désactive la gestion des arguments spécifique au shell. |
| `ignore_timeout`    | `false`                         | Ignore le paramètre global `command_timeout` et continuer à exécuter des commandes externes, peu importe le temps qu'elles prennent.                                                                                                                                                                                                 |

### Variables

| Variable | Description                                  |
| -------- | -------------------------------------------- |
| output   | La sortie de `command` exécutée dans `shell` |
| symbole  | Reflète la valeur de l'option `symbol`       |
| style\*  | Reflète la valeur de l'option `style`        |

\*: Cette variable peut uniquement être utilisée dans une chaine de style

#### Commandes shell personnalisées

`shell` accepte une liste de chaînes non vide, où:

- La première chaîne est le chemin vers le shell à utiliser pour exécuter la commande.
- Les arguments suivants sont passés au shell.

Si non défini, il utilisera STARSHIP_SHELL puis 'sh' sur Linux, et 'cmd /C' sur Windows.

La `command` (et `when`, le cas échéant) sera transmise sur l'entrée standard (stdin).

Si `shell` n'est pas défini ou ne contient qu'un seul élément et que Starship détecte que PowerShell sera utilisé, les arguments suivants seront automatiquement ajoutés : `-NoProfile -Command -`. Si `shell` n'est pas défini ou ne contient qu'un seul élément et que Starship détecte que Cmd sera utilisé, l'argument suivant sera automatiquement ajouté : `/C` et `stdin` sera défini à `false`. Si `shell` n'est pas défini ou ne contient qu'un seul élément et que Starship détecte que Nushell sera utilisé, les arguments suivants seront automatiquement ajoutés : `-c` et `stdin` sera défini à `false`. Ce comportement peut être évité en passant explicitement des arguments au shell, par ex.

```toml
shell = ['pwsh', '-Command', '-']
```

> [!WARNING] Assurez-vous que votre configuration de shell personnalisé se termine correctement
>
> Si vous définissez une commande personnalisée, assurez-vous que le shell par défaut utilisé par starship exécutera correctement la commande avec une sortie propre (via l'option `shell`).
>
> Par exemple, PowerShell nécessite le paramètre `-Command` pour exécuter une commande en une ligne. Omettre ce paramètre pourrait entraîner starship dans une boucle récursive où le shell pourrait essayer de charger un environnement de profil complet avec starship lui-même et donc ré-exécuter la commande personnalisée, entrant dans une boucle infinie.
>
> Des paramètres similaires à `-NoProfile` dans PowerShell sont recommandés pour les autres shells afin d'éviter un temps de chargement supplémentaire d'un profil personnalisé à chaque invocation de starship.
>
> La détection automatique des shells et l'ajout des paramètres appropriés sont actuellement implémentés, mais il est possible que tous les shells ne soient pas couverts. [Veuillez ouvrir une issue](https://github.com/starship/starship/issues/new/choose) avec les détails du shell et la configuration starship si vous rencontrez un tel scénario.

### Exemple

```toml
# ~/.config/starship.toml

[custom.foo]
command = 'echo foo' # affiche la sortie de la commande
detect_files = ['foo'] # peut spécifier des filtres mais les wildcards ne sont pas supportés
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'

[custom.time]
command = 'time /T'
detect_extensions = ['pst'] # filtre les fichiers *.pst
shell = ['pwsh.exe', '-NoProfile', '-Command', '-']

[custom.time-as-arg]
command = 'time /T'
detect_extensions = ['pst'] # filtre les fichiers *.pst
shell = ['pwsh.exe', '-NoProfile', '-Command']
use_stdin = false
```
