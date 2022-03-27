# Configuration

Pour commencer à configurer starship, créez le fichier suivant : `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toute la configuration de starship est effectuée dans ce fichier [TOML](https://github.com/toml-lang/toml) :

```toml
# Insère un saut de ligne entre les invites de commande
add_newline = true

# Remplace le symbole «❯» dans le prompt par «➜»
[character] # Le nom du module qu’on configure est «character»
success_symbol = "[➜](bold green)" # Le segment «success_symbol» est changé à «➜» avec la couleur «bold green»

# Désactive le module «package», le cachant complètement de l’invite
[package]
disabled = true
```

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

**Module**: Un composant dans l'invite donnant des informations basées sur des informations contextuelles à propos de votre système d'exploitation. Par exemple, le module "nodejs" montre la version de Node.js qui installée sur votre ordinateur, si votre dossier actuel est un projet Node.js.

**Variable**: Petits sous-composants qui contiennent des informations fournies par le module. Par exemple, la variable "version" dans le module "nodejs" contient la version actuelle de Node.js.

Par convention, la plupart des modules ont un préfixe de la couleur par défaut du terminal (par exemple `via` dans "nodejs") et un espace vide comme suffixe.

### Chaîne de formatage

Les chaînes de formatage sont le format avec lequel un module affiche toutes ses variables. La plupart des modules ont une entrée appelée `format` qui configure le format d'affichage du module. Vous pouvez utiliser des textes, des variables et des groupes de texte dans une chaîne de format.

#### Variable

Une variable contient un symbole `$` suivi du nom de la variable. Le nom d’une variable peut seulement container des lettres, des nombres et `_`.

Par exemple :

- `$version` est une chaîne de formatage avec une variable nommée `version`.
- `$git_branch$git_commit` est une chaîne de formatage avec deux variables appelées `git_branch` et `git_commit`.
- `$git_branch $git_commit` a les deux variables séparées par un espace.

#### Groupe de texte

Un groupe de texte se compose de deux parties différentes.

La première partie, qui est entourée dans un `[]`, est une [chaîne de formatage](#format-strings). Vous pouvez y ajouter des textes, des variables, ou même des groupes de texte imbriqués.

La deuxième partie, qui est entourée par `()`, est une [chaîne de style](#style-strings). Elle peut être utilisée pour styliser la première partie.

Par exemple :

- `[on](red bold)` affichera une chaîne de caractères `on` avec un texte gras de couleur rouge.
- `[⌘ $version](bold green)` affichera le symbole `⌘` suivi par le contenu de la variable `version`, en texte gras de couleur verte.
- `[a [b](red) c](green)` affichera `a b c` avec `b` rouge, et `a` et `c` vert.

#### Chaînes de style

La plupart des modules de Starship vous permettent de configurer leurs styles d'affichage. Cela se fait avec une entrée (généralement appelée `style`) qui est une chaîne de caractères spécifiant la configuration. Voici quelques exemples de chaînes de style avec ce qu'elles font. Pour plus de détails sur la syntaxe complète, consultez le [guide de configuration avancé](/advanced-config/).

- `"fg:green bg:blue"` définit un texte vert sur un fond bleu
- `"bg:blue fg:bright-green"` définit un texte vert clair sur un fond bleu
- `"bold fg:27"` définit le texte en gras avec la [couleur ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` définit le texte en souligné sur un fond orange foncé
- `"bold italic fg:purple"` définit le texte en italique et gras sur un fond violet
- `""` désactive explicitement tous les styles

Notez que ce style sera contrôlé par votre émulateur de terminal. Par exemple, certains émulateurs de terminal éclairciront les couleurs au lieu de mettre le texte en gras, et certains thèmes de couleurs utilisent les mêmes valeurs pour les couleurs normales et claires. De plus, pour obtenir du texte italique, votre terminal doit prendre en charge l'italique.

#### Chaînes de formatage conditionnel

Une chaîne de formatage conditionnel enveloppée dans `(` et `)` ne sera pas rendue si toutes les variables à l'intérieur sont vides.

Par exemple :

- `(@$region)` n’affichera rien si la variable `region` est `None` or une chaine vide, sinon `@` suivi par la valeur de region.
- `(some text)` ne montrera toujours rien puisqu'il n'y a pas de variables enveloppées dans les accolades.
- Lorsque `$all` est un raccourci pour `\[$a$b\]`, `($all)` ne montrera rien que si `$a` et `$b` sont tous les deux `None`. Cela fonctionne comme `(\[$a$b\] )`.

#### Caractères spéciaux

Les caractères suivants ont une utilisation spéciale dans les chaines de formatage et doivent être échappées : `$ \ [ ] ( )`.

Notez que TOML a [à la fois des chaines basiques et des chaines littérales](https://toml.io/en/v1.0.0#string). Il est recommandé d'utiliser une chaine littérale (entourée de guillemets simples) dans votre configuration. Si vous voulez utiliser une chaîne basique (entourée de guillemets doubles), vous devez échapper l'antislash lui-même (c-à-d. utiliser `\\`).

Par exemple, lorsque vous voulez imprimer un symbole `$` sur une nouvelle ligne, les configurations suivantes pour le `formatage` sont équivalentes :

```toml
# avec la chaîne de base
format = "\n\\$"

# avec la chaîne de caractères de base multiligne
format = """

\\$"""

# avec la chaîne littérale
format = '''

\$'''
```

## Invite

Voici la liste des options de configuration de l'invite en lui-même.

### Options

| Option            | Défaut                         | Description                                                                 |
| ----------------- | ------------------------------ | --------------------------------------------------------------------------- |
| `format`          | [lien](#default-prompt-format) | Configure le format de l'invite.                                            |
| `right_format`    | `""`                           | Voir [Activer le prompt à droite](/advanced-config/#enable-right-prompt)    |
| `scan_timeout`    | `30`                           | Délai d'attente pour que starship scanne les fichiers (en millisecondes).   |
| `command_timeout` | `500`                          | Délai maximal pour les commandes exécutées par starship (en millisecondes). |
| `add_newline`     | `true`                         | Insère une ligne vide entre les invites du shell.                           |

### Exemple

```toml
# ~/.config/starship. oml

# Utilisez un format personnalisé
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Attendez 10 millisecondes pour que starship vérifie les fichiers dans le dossier courant.
scan_timeout = 10

# Désactive la nouvelle ligne au début de l'invite
add_newline = false
```

### Format par Défaut

Le `format` par défaut est utilisé pour définir le format de l'invite, si il est vide ou mal `formaté`. La valeur par défaut est la suivante :

```toml
format = "$all"

# Ce qui est équivalent à
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
$buf\
$cmake\
$cobol\
$container\
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
$nix_shell\
$conda\
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
$shell\
$character"""
```

Si vous voulez étendre le format par défaut, pour pouvoir utiliser `$all` ; les modules que vous ajouter explicitement au format ne seront pas dupliqués. Par ex.

```toml
# Déplace le dossier à la ligne suivante
format = "$all$directory$character"
```

## AWS

The `aws` module shows the current AWS region and profile when credentials, a `credential_process` or a `sso_start_url` have been setup. Ces informations sont basées sur les variables d'environnement `AWS_REGION`, `AWS_DEFAULT_REGION`, et `AWS_PROFILE` ainsi que le fichier `~/.aws/config`. Ce module affiche également un minuteur d'expiration lorsque vous utilisez des identifiants temporaires.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or a `credential_process` is defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice.

Lorsque vous utilisez [aws-vault](https://github.com/99designs/aws-vault) le profil est lu à partir de la variable d'environnement `AWS_VAULT` et la date d'expiration des identifiants est lue à partir de la variable d'environnement `AWS_SESSION_EXPIRATION`.

Lorsque vous utilisez [awsu](https://github.com/kreuzwerker/awsu) le profil est lu depuis la variable d'environnement `AWSU_PROFILE`.

Lorsque vous utilisez [AWSume](https://awsu.me) le profil est lu à partir de la variable d'environnement `AWSUME_PROFILE` et la date d'expiration des identifiants est lue à partir de la variable d'environnement `AWSUME_EXPIRATION`.

### Options

| Option              | Défaut                                                               | Description                                                             |
| ------------------- | -------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\])]($style)'` | Format du module.                                                       |
| `symbol`            | `"☁️ "`                                                              | Le symbole est affiché avant le profil AWS actuel.                      |
| `region_aliases`    |                                                                      | Table des alias de région à afficher en plus du nom AWS.                |
| `profile_aliases`   |                                                                      | Table of profile aliases to display in addition to the AWS name.        |
| `style`             | `"bold yellow"`                                                      | Le style du module.                                                     |
| `expiration_symbol` | `X`                                                                  | Le symbole est affiché lorsque les identifiants temporaires ont expiré. |
| `disabled`          | `false`                                                              | Désactive le module `AWS`.                                              |

### Variables

| Variable  | Exemple          | Description                                    |
| --------- | ---------------- | ---------------------------------------------- |
| region    | `ap-northeast-1` | La région AWS actuelle                         |
| profile   | `astronauts`     | Le profil AWS actuel                           |
| duration  | `2h27m20s`       | Durée de validité des identifiants temporaires |
| symbol    |                  | Reflète la valeur de l'option `symbol`         |
| style\* |                  | Reflète la valeur de l'option `style`          |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemples

#### Tout afficher

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

#### Afficher la région

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

#### Afficher le profil

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

Le module `azure` affiche l'abonnement Azure actuel. Ceci est basé sur l'affichage du nom de l'abonnement par défaut, tel que défini dans le fichier `~/.azure/azureProfile.json`.

### Options

| Variable   | Défaut                                   | Description                              |
| ---------- | ---------------------------------------- | ---------------------------------------- |
| `format`   | `"on [$symbol($subscription)]($style) "` | Le format pour le rendu du module Azure. |
| `symbol`   | `"ﴃ "`                                   | Le symbole utilisé dans le format.       |
| `style`    | `"blue bold"`                            | Le style utilisé dans le format.         |
| `disabled` | `true`                                   | Désactive le module `azure`.             |

### Exemple

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($subscription)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## Battery

Le module `battery` montre à quel point la batterie de l'appareil est chargée et son état de charge actuel. Ce module n'est visible que lorsque la batterie de l'appareil est inférieure à 10%.

### Options

| Option               | Défaut                            | Description                                                   |
| -------------------- | --------------------------------- | ------------------------------------------------------------- |
| `full_symbol`        | `" "`                            | Le symbole affiché lorsque la batterie est pleine.            |
| `charging_symbol`    | `" "`                            | Le symbole affiché lorsque la batterie se charge.             |
| `discharging_symbol` | `" "`                            | Le symbole affiché lorsque la batterie se décharge.           |
| `unknown_symbol`     | `" "`                            | Le symbole affiché lorsque l'état de la batterie est inconnu. |
| `empty_symbol`       | `" "`                            | Le symbole affiché lorsque la batterie est vide.              |
| `format`             | `"[$symbol$percentage]($style) "` | Format du module.                                             |
| `display`            | [lien](#battery-display)          | Affiche le seuil et le style du module.                       |
| `disabled`           | `false`                           | Désactive le module `battery`.                                |

### Exemple

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### Indicateur de batterie

L'option de configuration `display` est utilisée pour définir quand l'indicateur de batterie doit être affiché (threshold), quel symbole doit être utilisé (symbol) et à quoi il ressemble (style). Si aucun `display` n'est fourni. La valeur par défaut est la suivante :

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

La valeur par défaut pour les options `charging_symbol` et `discharging_symbol` est respectivement la valeur des options `charging_symbol` et `discharging_symbol` du module `battery`.

#### Options

L'option `display` est une array de la table suivante.

| Option               | Défaut     | Description                                                                                                                             |
| -------------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`       | La limite supérieure pour l'option d'affichage.                                                                                         |
| `style`              | `bold red` | Le style de l'option display si elle est utilisée.                                                                                      |
| `charging_symbol`    | `-`        | Symbole optionnel affiché si l'option display est utilisée, la valeur par défaut est l'option `charging_symbol` du module "battery".    |
| `discharging_symbol` | `-`        | Symbole optionnel affiché si l'option display est utilisée, la valeur par défaut est l'option `discharging_symbol` du module "battery". |

#### Exemple

```toml
[[battery.display]] # style "bold red" et discharging_symbol quand la batterie est entre 0% et 10%
threshold = 10
style = "bold red"

[[battery.display]] # style "bold yellow" et symbole 💦 quand la batterie est entre 10% et 30%
threshold = 30
style = "bold yellow"
discharging_symbol = "💦"

# quand la batterie est au-dessus de 30%, l’indicateur de batterie ne sera pas affiché
```

## Buf

Le module `buf` affiche la version de [Buf](https://buf.build) installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie:

- La commande [`buf`](https://github.com/bufbuild/buf) est installée.
- Le dossier actuel contient un fichier de configuration [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml) ou [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml).

### Options

| Option              | Défaut                                                       | Description                                            |
| ------------------- | ------------------------------------------------------------ | ------------------------------------------------------ |
| `format`            | `'with [$symbol($version \(Buf $buf_version\) )]($style)'` | Le format du module `buf`.                             |
| `version_format`    | `"v${raw}"`                                                  | Le format de la version.                               |
| `symbol`            | `"🦬 "`                                                       | Le symbole utilisé avant d’afficher la version de Buf. |
| `detect_extensions` | `[]`                                                         | Quelles extensions devraient activer ce module.        |
| `detect_files`      | `["buf.yaml", "buf.gen.yaml", "buf.work.yaml"]`              | Les fichiers qui activent ce module.                   |
| `detect_folders`    | `[]`                                                         | Quels dossiers devraient activer ce module.            |
| `style`             | `"bold blue"`                                                | Le style du module.                                    |
| `disabled`          | `false`                                                      | Désactive le module `elixir`.                          |

### Variables

| Variable      | Exemple  | Description                            |
| ------------- | -------- | -------------------------------------- |
| `buf_version` | `v1.0.0` | La version de `buf`                    |
| `symbol`      |          | Reflète la valeur de l'option `symbol` |
| `style`*      |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[buf]
symbol = "🦬 "
```

## Caractère

Le module `character` affiche un caractère (en général une flèche) à côté de là où vous entrez le texte dans votre terminal.

Le caractère vous dira si la dernière commande a été réussie ou pas. Il peut faire ça de deux façons:

- en changeant de couleur(`red`/`green`)
- en changeant de forme (`❯`/`✖`)

Par défaut, il ne change que de couleur. Si vous désirez également changer sa forme, jetez un à [cet exemple](#with-custom-error-shape).

::: warning

`vicmd_symbol` n’est supporté que dans cmd, fish et zsh.

:::

### Options

| Option           | Défaut              | Description                                                                      |
| ---------------- | ------------------- | -------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | The format string used before the text input.                                    |
| `success_symbol` | `"[❯](bold green)"` | The format string used before the text input if the previous command succeeded.  |
| `error_symbol`   | `"[❯](bold red)"`   | The format string used before the text input if the previous command failed.     |
| `vicmd_symbol`   | `"[❮](bold green)"` | The format string used before the text input if the shell is in vim normal mode. |
| `disabled`       | `false`             | Désactive le module `character`.                                                 |

### Variables

| Variable | Exemple | Description                                                           |
| -------- | ------- | --------------------------------------------------------------------- |
| symbol   |         | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

### Exemples

#### Avec un caractère d'erreur personnalisé

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Sans caractère d'erreur personnalisé

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### Avec une forme vim personnalisée

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

Le module `cmake` affiche la version de [CMake](https://cmake.org/) installée. By default the module will be activated if any of the following conditions are met:

- Le dossier courant contient un fichier `CMakeLists.txt`
- Le dossier courant contient un fichier `CMakeCache.txt`

### Options

| Option              | Défaut                                 | Description                                                                                |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`   | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                            | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"△ "`                                 | The symbol used before the version of cmake.                                               |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this module                                                |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Which filenames should trigger this module                                                 |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module                                                   |
| `style`             | `"bold blue"`                          | Le style du module.                                                                        |
| `disabled`          | `false`                                | Désactive le module `cmake`.                                                               |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | La version de cmake                    |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

## COBOL / GNUCOBOL

Le module `cobol` affiche la version de COBOL installée. By default, the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier finissant par `.cob` ou `.COB`
- Le dossier courant contiens un fichier finissant par `.cbl` ou `.CBL`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `symbol`            | `"⚙️ "`                              | Le symbole utilisé avant d’afficher la version de COBOL.                                   |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold blue"`                        | Le style du module.                                                                        |
| `detect_extensions` | `["cbl", "cob", "CBL", "COB"]`       | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`          | `false`                              | Désactive le module `cobol`.                                                               |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v3.1.2.0` | La version de `cobol`                  |
| symbol    |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

## Temps d'exécution

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Options

| Option                 | Défaut                        | Description                                                                                                                                                       |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Shortest duration to show time for (in milliseconds).                                                                                                             |
| `show_milliseconds`    | `false`                       | Show milliseconds in addition to seconds for the duration.                                                                                                        |
| `format`               | `"took [$duration]($style) "` | Format du module.                                                                                                                                                 |
| `style`                | `"bold yellow"`               | Le style du module.                                                                                                                                               |
| `disabled`             | `false`                       | Désactive le module `cmd_duration`.                                                                                                                               |
| `show_notifications`   | `false`                       | Show desktop notifications when command completes.                                                                                                                |
| `min_time_to_notify`   | `45_000`                      | Shortest duration for notification (in milliseconds).                                                                                                             |
| `notification_timeout` |                               | Duration to show notification for (in milliseconds). If unset, notification timeout will be determined by daemon. Not all notification daemons honor this option. |

### Variables

| Variable  | Exemple  | Description                             |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | Reflète la valeur de l'option `style`   |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### Options

| Option              | Défaut                                 | Description                                                                                                                                                                                                 |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `"🅒 "`                                 | The symbol used before the environment name.                                                                                                                                                                |
| `style`             | `"bold green"`                         | Le style du module.                                                                                                                                                                                         |
| `format`            | `"via [$symbol$environment]($style) "` | Format du module.                                                                                                                                                                                           |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                  |
| `disabled`          | `false`                                | Désactive le module `conda`.                                                                                                                                                                                |

### Variables

| Variable    | Exemple      | Description                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | The current conda environment          |
| symbol      |              | Reflète la valeur de l'option `symbol` |
| style\*   |              | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Container

The `container` module displays a symbol and container name, if inside a container.

### Options

| Option     | Défaut                                 | Description                               |
| ---------- | -------------------------------------- | ----------------------------------------- |
| `symbol`   | `"⬢"`                                  | The symbol shown, when inside a container |
| `style`    | `"bold red dimmed"`                    | Le style du module.                       |
| `format`   | `"[$symbol \\[$name\\]]($style) "` | Format du module.                         |
| `disabled` | `false`                                | Désactive le module `container`.          |

### Variables

| Variable  | Exemple             | Description                            |
| --------- | ------------------- | -------------------------------------- |
| name      | `fedora-toolbox:35` | Le nom du conteneur                    |
| symbol    |                     | Reflète la valeur de l'option `symbol` |
| style\* |                     | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[container]
format = "[$symbol \\[$name\\]]($style) "
```

## Crystal

Le module `crystal` affiche la version de [Crystal](https://crystal-lang.org/) installée. By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `shard.yml`
- Le dossier courant contient un fichier `.cr`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `symbol`            | `"🔮 "`                               | Le symbole utilisé avant d'afficher la version de crystal.                                 |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | Le style du module.                                                                        |
| `detect_extensions` | `["cr"]`                             | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["shard.yml"]`                      | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`          | `false`                              | Désactive le module `crystal`.                                                             |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | La version de `crystal`                |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

Le module `dart` affiche la version de [Dart](https://dart.dev/) installée. By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier avec l’extension `.dart`
- Le dossier courant contient un fichier `.dart_tool`
- Le dossier courant contient un fichier `pubsepc.yaml`, `pubspec.yml` ou `pubspec.lock`

### Options

| Option              | Défaut                                            | Description                                                                                |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`              | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                       | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | A format string representing the symbol of Dart                                            |
| `detect_extensions` | `["dart"]`                                        | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[".dart_tool"]`                                  | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold blue"`                                     | Le style du module.                                                                        |
| `disabled`          | `false`                                           | Désactive le module `dart`.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.8.4` | La version de `dart`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

Le module `deno` affiche la version de [Deno](https://deno.land/) installée. By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` ou `deps.js`

### Options

| Option              | Défaut                                                                  | Description                                                                                |
| ------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                    | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                                             | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                                                  | A format string representing the symbol of Deno                                            |
| `detect_extensions` | `[]`                                                                    | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["deno.json", "deno.jsonc", "mod.ts", "mod.js", "deps.ts", "deps.js"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                    | Les dossiers qui activent ce module.                                                       |
| `style`             | `"green bold"`                                                          | Le style du module.                                                                        |
| `disabled`          | `false`                                                                 | Désactive le module `deno`.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.8.3` | La version de `deno`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[deno]
format = "via [🦕 $version](green bold) "
```

## Dossier

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Options

| Option              | Défaut                                                                                                      | Description                                                                             |
| ------------------- | ----------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                                                                         | The number of parent folders that the current directory should be truncated to.         |
| `truncate_to_repo`  | `true`                                                                                                      | Whether or not to truncate to the root of the git repo that you're currently in.        |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "`                                                          | Format du module.                                                                       |
| `style`             | `"bold cyan"`                                                                                               | Le style du module.                                                                     |
| `disabled`          | `false`                                                                                                     | Désactive le module `directory`.                                                        |
| `read_only`         | `"🔒"`                                                                                                       | The symbol indicating current directory is read only.                                   |
| `read_only_style`   | `"red"`                                                                                                     | The style for the read only symbol.                                                     |
| `truncation_symbol` | `""`                                                                                                        | The symbol to prefix to truncated paths. eg: "…/"                                       |
| `repo_root_style`   | `None`                                                                                                      | The style for the root of the git repo. The default value is equivalent to `style`.     |
| `repo_root_format`  | `"[$before_root_path]($style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) "` | The format of a git repo when `repo_root_style` is defined.                             |
| `home_symbol`       | `"~"`                                                                                                       | The symbol indicating home directory.                                                   |
| `use_os_path_sep`   | `true`                                                                                                      | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows) |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | Défaut | Description                                                                                                                                                            |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`    | The number of characters to use when applying fish shell pwd path logic.                                                                                               |
| `use_logical_path`          | `true` | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable  | Exemple               | Description                           |
| --------- | --------------------- | ------------------------------------- |
| path      | `"D:/Projects"`       | The current directory path            |
| style\* | `"black bold dimmed"` | Reflète la valeur de l'option `style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

<details>
<summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| Variable           | Exemple               | Description                             |
| ------------------ | --------------------- | --------------------------------------- |
| before_root_path | `"/path/to/home/"`    | The path before git root directory path |
| repo_root          | `"git_repo"`          | The git root directory name             |
| path               | `"/src/lib"`          | The remaining path                      |
| style              | `"black bold dimmed"` | Reflète la valeur de l'option `style`   |
| repo_root_style  | `"underline white"`   | Style for git root directory name       |

</details>

### Exemple

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### Options

| Option              | Défaut                                                        | Description                                                                       |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | Format du module.                                                                 |
| `symbol`            | `"🐳 "`                                                        | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                        | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                          | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                          | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `"blue bold"`                                                 | Le style du module.                                                               |
| `disabled`          | `false`                                                       | Désactive le module `docker_context`.                                             |

### Variables

| Variable  | Exemple        | Description                            |
| --------- | -------------- | -------------------------------------- |
| context   | `test_context` | Le contexte docker courant             |
| symbol    |                | Reflète la valeur de l'option `symbol` |
| style\* |                | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Options

| Option              | Défaut                                                                                                  | Description                                                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                                                                             | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | The symbol used before displaying the version of dotnet.                                   |
| `heuristic`         | `true`                                                                                                  | Use faster version detection to keep starship snappy.                                      |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Les extensions qui déclenchent ce module.                                                  |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                    | Quels dossiers devraient activer ce module.                                                |
| `style`             | `"bold blue"`                                                                                           | Le style du module.                                                                        |
| `disabled`          | `false`                                                                                                 | Désactive le module `dotnet`.                                                              |

### Variables

| Variable  | Exemple          | Description                                                        |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Reflète la valeur de l'option `symbol`                             |
| style\* |                  | Reflète la valeur de l'option `style`                              |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

Le module `elixir` affiche la version de [Elixir](https://elixir-lang.org/) et [Erlang/OTP](https://erlang.org/doc/) installée. By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `mix.exs`.

### Options

| Option              | Défaut                                                      | Description                                                                                |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                                                          |
| `version_format`    | `"v${raw}"`                                                 | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | The symbol used before displaying the version of Elixir/Erlang.                            |
| `detect_extensions` | `[]`                                                        | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["mix.exs"]`                                               | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                        | Quels dossiers devraient activer ce module.                                                |
| `style`             | `"bold purple"`                                             | Le style du module.                                                                        |
| `disabled`          | `false`                                                     | Désactive le module `elixir`.                                                              |

### Variables

| Variable    | Exemple | Description                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | The version of `elixir`                |
| otp_version |         | The otp version of `elixir`            |
| symbol      |         | Reflète la valeur de l'option `symbol` |
| style\*   |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

Le module `elm` affiche la version de [Elm](https://elm-lang.org/) installée. By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `elm.json`
- Le dossier courant contient un fichier `elm-package.json`
- Le dossier courant contient un fichier `elm-version`
- Le dossier courant contient un dossier `elm-stuff`
- Le dossier courant contient des fichiers `*.elm`

### Options

| Option              | Défaut                                             | Description                                                                                |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`               | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                        | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | A format string representing the symbol of Elm.                                            |
| `detect_extensions` | `["elm"]`                                          | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `["elm-stuff"]`                                    | Quels dossiers devraient activer ce module.                                                |
| `style`             | `"cyan bold"`                                      | Le style du module.                                                                        |
| `disabled`          | `false`                                            | Désactive le module `elm`.                                                                 |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.19.1` | La version de `elm`                    |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Variable d'environnement

The `env_var` module displays the current value of a selected environment variables. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

::: tip Multiple environmental variables can be displayed by using a `.`. (see example) If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.

Example: following configuration will display value of USER environment variable

```toml
# ~/.config/starship.toml

[env_var.USER]
default = "utilisateur inconnu"
```

:::

### Options

| Option     | Défault                        | Description                                                                  |
| ---------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`   | `""`                           | The symbol used before displaying the variable value.                        |
| `variable` |                                | The environment variable to be displayed.                                    |
| `default`  |                                | The default value to be displayed when the selected variable is not defined. |
| `format`   | `"with [$env_value]($style) "` | Format du module.                                                            |
| `disabled` | `false`                        | Désactive le module `env_var`.                                               |

### Variables

| Variable  | Exemple                                  | Description                                |
| --------- | ---------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (si _variable_ était `$OS`) | The environment value of option `variable` |
| symbol    |                                          | Reflète la valeur de l'option `symbol`     |
| style\* | `black bold dimmed`                      | Reflète la valeur de l'option `style`      |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "shell inconnu"
```

Displaying multiple environmental variables:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = "SHELL"
default = "shell inconnu"
[env_var.USER]
default = "utilisateur inconnu"
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `rebar.config`.
- Le dossier courant contient un fichier `erlang.mk`.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | The symbol used before displaying the version of erlang.                                   |
| `style`             | `"bold red"`                         | Le style du module.                                                                        |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `disabled`          | `false`                              | Désactive le module `erlang`.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | The version of `erlang`                |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are present in a line they will split the space evenly between them. This is useful for aligning other modules.

### Options

| Option     | Défaut         | Description                       |
| ---------- | -------------- | --------------------------------- |
| `symbol`   | `"."`          | The symbol used to fill the line. |
| `style`    | `"bold black"` | Le style du module.               |
| `disabled` | `false`        | Désactive le module `fill`        |

### Exemple

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

### Options

| Option            | Défaut                                                     | Description                                                      |
| ----------------- | ---------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Format du module.                                                |
| `symbol`          | `"☁️  "`                                                   | The symbol used before displaying the current GCP profile.       |
| `region_aliases`  |                                                            | Table of region aliases to display in addition to the GCP name.  |
| `project_aliases` |                                                            | Table of project aliases to display in addition to the GCP name. |
| `style`           | `"bold blue"`                                              | Le style du module.                                              |
| `disabled`        | `false`                                                    | Désactive le module `gcloud`.                                    |

### Variables

| Variable  | Exemple       | Description                                                        |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | The current GCP region                                             |
| account   | `foo`         | The current GCP profile                                            |
| domain    | `example.com` | The current GCP profile domain                                     |
| project   |               | The current GCP project                                            |
| active    | `default`     | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |               | Reflète la valeur de l'option `symbol`                             |
| style\* |               | Reflète la valeur de l'option `style`                              |

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
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Afficher le compte et la région aliasée

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

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Options

| Option               | Défaut                           | Description                                                                              |
| -------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Shows the remote tracking branch name, even if it is equal to the local branch name.     |
| `format`             | `"on [$symbol$branch]($style) "` | Format du module. Use `"$branch"` to refer to the current branch name.                   |
| `symbol`             | `" "`                           | A format string representing the symbol of git branch.                                   |
| `style`              | `"bold purple"`                  | Le style du module.                                                                      |
| `truncation_length`  | `2^63 - 1`                       | Truncates a git branch to `N` graphemes.                                                 |
| `truncation_symbol`  | `"…"`                            | The symbol used to indicate a branch name was truncated. You can use `""` for no symbol. |
| `only_attached`      | `false`                          | Only show the branch name when not in a detached `HEAD` state.                           |
| `ignore_branches`    | `[]`                             | A list of names to avoid displaying. Utile pour "master" ou "main".                      |
| `disabled`           | `false`                          | Désactive le module `git_branch`.                                                        |

### Variables

| Variable      | Exemple  | Description                                                                                            |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | Reflète la valeur de l'option `symbol`                                                                 |
| style\*     |          | Reflète la valeur de l'option `style`                                                                  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
ignore_branches = ["master", "main"]
```

## Commit Git

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Options

| Option               | Défaut                             | Description                                             |
| -------------------- | ---------------------------------- | ------------------------------------------------------- |
| `commit_hash_length` | `7`                                | The length of the displayed git commit hash.            |
| `format`             | `"[\\($hash$tag\\)]($style) "` | Format du module.                                       |
| `style`              | `"bold green"`                     | Le style du module.                                     |
| `only_detached`      | `true`                             | Only show git commit hash when in detached `HEAD` state |
| `tag_disabled`       | `true`                             | Disables showing tag info in `git_commit` module.       |
| `tag_symbol`         | `" 🏷 "`                            | Tag symbol prefixing the info shown                     |
| `disabled`           | `false`                            | Désactive le module `git_commit`.                       |

### Variables

| Variable  | Exemple   | Description                           |
| --------- | --------- | ------------------------------------- |
| hash      | `b703eb3` | The current git commit hash           |
| style\* |           | Reflète la valeur de l'option `style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Options

| Option         | Défaut                                                          | Description                                                                             |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | Le style du module.                                                                     |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Format du module.                                                                       |
| `disabled`     | `false`                                                         | Désactive le module `git_state`.                                                        |

### Variables

| Variable         | Exemple    | Description                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | The current state of the repo         |
| progress_current | `1`        | The current operation progress        |
| progress_total   | `2`        | The total operation progress          |
| style\*        |            | Reflète la valeur de l'option `style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in the current git repository.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Option               | Défaut                                                       | Description                           |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `"bold green"`                                               | The style for the added count.        |
| `deleted_style`      | `"bold red"`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Format du module.                     |
| `disabled`           | `true`                                                       | Désactive le module `git_metrics`.    |

### Variables

| Variable          | Exemple | Description                                 |
| ----------------- | ------- | ------------------------------------------- |
| added             | `1`     | The current number of added lines           |
| deleted           | `2`     | The current number of deleted lines         |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = "bold blue"
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Statut Git

The `git_status` module shows symbols representing the state of the repo in your current directory.

::: tip

The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment. You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

:::

### Options

| Option              | Défaut                                          | Description                                                                                                 |
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
| `style`             | `"bold red"`                                    | Le style du module.                                                                                         |
| `ignore_submodules` | `false`                                         | Ignore changes to submodules.                                                                               |
| `disabled`          | `false`                                         | Désactive le module `git_status`.                                                                           |
| `windows_starship`  |                                                 | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |

### Variables

The following variables can be used in `format`:

| Variable       | Description                                                                                                   |
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
| style\*      | Reflète la valeur de l'option `style`                                                                         |

*: Cette variable peut uniquement être utilisée dans une chaine de style

The following variables can be used in `diverged`:

| Variable       | Description                                    |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Variable | Description              |
| -------- | ------------------------ |
| `count`  | Show the number of files |

### Exemple

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

- Le dossier courant contient un fichier `go.mod`
- Le dossier courant contient un fichier `go.sum`
- Le dossier courant contient un fichier `glide.yaml`
- Le dossier courant contient un fichier `Gopkg.yml`
- Le dossier courant contient un fichier `Gopkg.lock`
- Le dossier courant contient un fichier `.go-version`
- Le dossier courant contient un dossier `Godeps`
- Le dossier courant contient un fichier avec l’extension `.go`

### Options

| Option              | Défaut                                                                         | Description                                                                                |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                           | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                                                    | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                         | A format string representing the symbol of Go.                                             |
| `detect_extensions` | `["go"]`                                                                       | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `["Godeps"]`                                                                   | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold cyan"`                                                                  | Le style du module.                                                                        |
| `disabled`          | `false`                                                                        | Désactive le module `golang`.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | The version of `go`                    |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `stack.yaml`
- Le dossier courant contient un fichier `.hs`, `.cabal` ou `.hs-boot`

### Options

| Option              | Défaut                               | Description                                        |
| ------------------- | ------------------------------------ | -------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                  |
| `symbol`            | `"λ "`                               | A format string representing the symbol of Haskell |
| `detect_extensions` | `["hs", "cabal", "hs-boot"]`         | Quelles extensions devraient activer ce module.    |
| `detect_files`      | `["stack.yaml", "cabal.project"]`    | Les fichiers qui activent ce module.               |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.               |
| `style`             | `"bold purple"`                      | Le style du module.                                |
| `disabled`          | `false`                              | Désactive le module `haskell`.                     |

### Variables

| Variable       | Exemple     | Description                                                                             |
| -------------- | ----------- | --------------------------------------------------------------------------------------- |
| version        |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot       | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbol         |             | Reflète la valeur de l'option `symbol`                                                  |
| style\*      |             | Reflète la valeur de l'option `style`                                                   |

*: Cette variable peut uniquement être utilisée dans une chaine de style

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `helmfile.yaml`
- Le dossier courant contient un fichier `Chart.yaml`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm.                                           |
| `style`             | `"bold white"`                       | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `helm`.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | The version of `helm`                  |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Nom d'hôte

The `hostname` module shows the system hostname.

### Options

| Option     | Défaut                      | Description                                                                                                                          |
| ---------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                      | Only show hostname when connected to an SSH session.                                                                                 |
| `trim_at`  | `"."`                       | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `format`   | `"[$hostname]($style) in "` | Format du module.                                                                                                                    |
| `style`    | `"bold dimmed green"`       | Le style du module.                                                                                                                  |
| `disabled` | `false`                     | Désactive le module `hostname`.                                                                                                      |

### Variables

| Variable  | Exemple    | Description                           |
| --------- | ---------- | ------------------------------------- |
| hostname  | `computer` | The hostname of the computer          |
| style\* |            | Reflète la valeur de l'option `style` |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`ou `build.boot`
- Le dossier courant contient un fichier avec l’extension `.java`, `.class`, `.gradle`, `.jar`, `.clj` ou `.cljc`

### Options

| Option              | Défaut                                                                                                    | Description                                                                                |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                                                                               | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                      | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java                                            |
| `style`             | `"red dimmed"`                                                                                            | Le style du module.                                                                        |
| `disabled`          | `false`                                                                                                   | Désactive le module `java`.                                                                |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| version   | `v14`   | The version of `java`                  |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

### Options

| Option             | Défaut                        | Description                                                              |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | Format du module.                                                        |
| `symbol`           | `"✦"`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `"bold blue"`                 | Le style du module.                                                      |
| `disabled`         | `false`                       | Désactive le module `jobs`.                                              |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | The number of jobs                     |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
number_threshold = 4
symbol_threshold = 0
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `Project.toml`
- Le dossier courant contient un fichier `Manifest.toml`
- Le dossier courant contient un fichier `.jl`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.                                          |
| `style`             | `"bold purple"`                      | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `Julia`.                                                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.4.0` | The version of `julia`                 |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `.kt` ou `.kts`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["kt", "kts"]`                      | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                                         |
| `style`             | `"bold blue"`                        | Le style du module.                                                                        |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version.              |
| `disabled`          | `false`                              | Désactive le module `kotlin`.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`                |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Option            | Défaut                                               | Description                                                           |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | Format du module.                                                     |
| `style`           | `"cyan bold"`                                        | Le style du module.                                                   |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Désactiver le module `kubernetes`.                                    |

### Variables

| Variable  | Exemple              | Description                              |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | Reflète la valeur de l'option `symbol`   |
| style\* |                      | Reflète la valeur de l'option `style`    |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
".*/openshift-cluster/.*" = "openshift"
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
```

#### Regex Matching

Additional to simple aliasing, `context_aliases` also supports extended matching and renaming using regular expressions.

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

## Line Break

The `line_break` module separates the prompt into two lines.

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

## Local IP

The `localip` module shows the IPv4 address of the primary network interface.

### Options

| Option     | Défaut                    | Description                                            |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `"[$localipv4]($style) "` | Format du module.                                      |
| `style`    | `"bold yellow"`           | Le style du module.                                    |
| `disabled` | `true`                    | Désactive le module `localip`.                         |

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
format = "@[$localipv4](bold red) "
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `.lua-version`
- Le dossier courant contient un dossier `lua`
- Le dossier courant contient un fichier avec l’extension `.lua`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                                            |
| `detect_extensions` | `["lua"]`                            | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[".lua-version"]`                   | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `["lua"]`                            | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold blue"`                        | Le style du module.                                                                        |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version.                 |
| `disabled`          | `false`                              | Désactive le module `lua`.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | The version of `lua`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Option      | Défaut                                          | Description                                              |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | Format du module.                                        |
| `symbol`    | `"🐏"`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                           | Le style du module.                                      |
| `disabled`  | `true`                                          | Désactiver le module `memory_usage`.                     |

### Variables

| Variable         | Exemple       | Description                                                        |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Reflète la valeur de l'option `symbol`                             |
| style\*        |               | Reflète la valeur de l'option `style`                              |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

### Exemple

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

### Options

| Option              | Défaut                           | Description                                                                                  |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | Le style du module.                                                                          |
| `format`            | `"on [$symbol$branch]($style) "` | Format du module.                                                                            |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                           | Désactive le module `hg_branch`.                                                             |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| branch    | `master` | The active mercurial branch            |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `nim.cfg`
- Le dossier courant contient un fichier avec l’extension `.nim`
- Le dossier courant contient un fichier avec l’extension `.nims`
- Le dossier courant contient un fichier avec l’extension `.nimble`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module                                                                  |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.                                      |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["nim.cfg"]`                        | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold yellow"`                      | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `nim`.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.2.0` | The version of `nimc`                  |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. The module will be shown when inside a nix-shell environment.

### Options

| Option       | Défaut                                         | Description                                           |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | Format du module.                                     |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | Le style du module.                                   |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Désactive le module `nix_shell`.                      |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| state     | `pure`  | The state of the nix-shell             |
| name      | `lorri` | The name of the nix-shell              |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

- Le dossier courant contient un fichier `package.json`
- Le dossier courant contient un fichier `.node-version`
- Le dossier courant contient un fichier `.nvmrc`
- The current directory contains a `node_modules` directory
- Le dossier courant contient un fichier avec l’extension `.js`, `.mjs` ou `.cjs`
- Le dossier courant contient un fichier avec l’extension `.ts`, `.mts` ou `.cts`

### Options

| Option              | Défaut                                     | Description                                                                                           |
| ------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`       | Format du module.                                                                                     |
| `version_format`    | `"v${raw}"`                                | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch`            |
| `symbol`            | `" "`                                     | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts", "mts", "cts"]` | Quelles extensions devraient activer ce module.                                                       |
| `detect_files`      | `["package.json", ".node-version"]`        | Les fichiers qui activent ce module.                                                                  |
| `detect_folders`    | `["node_modules"]`                         | Les dossiers qui activent ce module.                                                                  |
| `style`             | `"bold green"`                             | Le style du module.                                                                                   |
| `disabled`          | `false`                                    | Désactive le module `nodejs`.                                                                         |
| `not_capable_style` | `bold red`                                 | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | The version of `node`                  |
| symbol    |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier avec l’extension `.opam` ou un dossier `_opam`
- The current directory contains a `esy.lock` directory
- Le dossier courant contient un fichier `dune` ou `dune-project`
- Le dossier courant contient un fichier `jbuild` ou `jbuild-ignore`
- Le dossier courant contient un fichier `.merlin`
- Le dossier courant contient un fichier avec l’extension `.ml`, `.mli`, `.re` ou `.rei`

### Options

| Option                    | Défaut                                                                     | Description                                                                                |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | La chaîne de format pour le module.                                                        |
| `version_format`          | `"v${raw}"`                                                                | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml.                                    |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch.                                    |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.                                     |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Quelles extensions devraient activer ce module.                                            |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Les fichiers qui activent ce module.                                                       |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Les dossiers qui activent ce module.                                                       |
| `style`                   | `"bold yellow"`                                                            | Le style du module.                                                                        |
| `disabled`                | `false`                                                                    | Désactive le module `ocaml`.                                                               |

### Variables

| Variable         | Exemple      | Description                                                       |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Reflète la valeur de l'option `symbol`                            |
| style\*        |              | Reflète la valeur de l'option `style`                             |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Options

| Option     | Défaut                                              | Description                                                    |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | Format du module.                                              |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | Le style du module.                                            |
| `disabled` | `false`                                             | Désactive le module `openstack`.                               |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | The current OpenStack cloud            |
| project   | `dev`   | The current OpenStack project          |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from the `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ La version montrée est celle du paquet dont le code source est dans votre dossier courant, pas votre gestionnaire de paquet.

### Options

| Option            | Défaut                            | Description                                                                                |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`          | `"is [$symbol$version]($style) "` | Format du module.                                                                          |
| `symbol`          | `"📦 "`                            | The symbol used before displaying the version the package.                                 |
| `version_format`  | `"v${raw}"`                       | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`           | `"bold 208"`                      | Le style du module.                                                                        |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.                                  |
| `disabled`        | `false`                           | Désactive le module `package`.                                                             |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.0.0` | The version of your package            |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `Makefile.PL` ou `Build.PL`
- Le dossier courant contient un fichier `cpanfile` ou `cpanfile.snapshot`
- Le dossier courant contient un fichier `META.json` ou `META.yml`
- Le dossier courant contient un fichier `.perl-version`
- The current directory contains a `.pl`, `.pm` or `.pod`

### Options

| Option              | Défaut                                                                                                   | Description                                                                                |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | La chaîne de format pour le module.                                                        |
| `version_format`    | `"v${raw}"`                                                                                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl                                      |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                     | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold 149"`                                                                                             | Le style du module.                                                                        |
| `disabled`          | `false`                                                                                                  | Désactive le module `perl`.                                                                |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v5.26.1` | The version of `perl`                  |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `composer.json`
- Le dossier courant contient un fichier `.php-version`
- The current directory contains a `.php` extension

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP.                                      |
| `detect_extensions` | `["php"]`                            | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["composer.json", ".php-version"]`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"147 bold"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `php`.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v7.3.8` | The version of `php`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml`

### Options

| Option           | Défaut                                       | Description                                                                                |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`         | `"via [$symbol($username@)$stack]($style) "` | La chaîne de format pour le module.                                                        |
| `version_format` | `"v${raw}"`                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `" "`                                       | A format string shown before the Pulumi stack.                                             |
| `style`          | `"bold 5"`                                   | Le style du module.                                                                        |
| `disabled`       | `false`                                      | Désactive le module `pulumi`.                                                              |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `pulumi`                |
| stack     | `dev`      | The current Pulumi stack               |
| username  | `alice`    | The current Pulumi username            |
| symbol    |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

- Le dossier courant contient un fichier `spago.dhall`
- Le dossier courant contient un fichier avec l’extension `.purs`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript.                               |
| `detect_extensions` | `["purs"]`                           | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["spago.dhall"]`                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold white"`                       | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `purescript`.                                                          |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | The version of `purescript`            |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `.python-version`
- Le dossier courant contient un fichier `Pipfile`
- Le dossier courant contient un fichier `__init__.py`
- Le dossier courant contient un fichier `pyproject.toml`
- Le dossier courant contient un fichier `requirements.txt`
- Le dossier courant contient un fichier `setup.py`
- Le dossier courant contient un fichier `tox.ini`
- Le dossier courant contient un fichier avec l’extension `.py`.
- A virtual environment is currently activated

### Options

| Option               | Défaut                                                                                                       | Description                                                                                |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Format du module.                                                                          |
| `version_format`     | `"v${raw}"`                                                                                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                          |
| `style`              | `"yellow bold"`                                                                                              | Le style du module.                                                                        |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                            |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                            |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configures the python binaries that Starship should executes when getting the version.     |
| `detect_extensions`  | `["py"]`                                                                                                     | Which extensions should trigger this module                                                |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Which filenames should trigger this module                                                 |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                                   |
| `disabled`           | `false`                                                                                                      | Désactive le module `python`.                                                              |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Variables

| Variable     | Exemple         | Description                                |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Reflète la valeur de l'option `symbol`     |
| style        | `"yellow bold"` | Reflète la valeur de l'option `style`      |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |

### Exemple

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

- Le dossier courant contient un fichier avec l’extension `.R`.
- Le dossier courant contient un fichier avec l’extension `.Rd`.
- Le dossier courant contient un fichier avec l’extension `.Rmd`.
- Le dossier courant contient un fichier avec l’extension `.Rproj`.
- Le dossier courant contient un fichier avec l’extension `.Rsx`.
- Le dossier courant contient un fichier `.Rprofile`
- The current directory contains a `.Rproj.user` folder

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"📐"`                                | A format string representing the symbol of R.                                              |
| `style`             | `"blue bold"`                        | Le style du module.                                                                        |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | Which extensions should trigger this module                                                |
| `detect_files`      | `[".Rprofile"]`                      | Which filenames should trigger this module                                                 |
| `detect_folders`    | `[".Rproj.user"]`                    | Which folders should trigger this module                                                   |
| `disabled`          | `false`                              | Désactive le module `r`.                                                                   |

### Variables

| Variable | Exemple       | Description                            |
| -------- | ------------- | -------------------------------------- |
| version  | `v4.0.5`      | The version of `R`                     |
| symbol   |               | Reflète la valeur de l'option `symbol` |
| style    | `"blue bold"` | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[rlang]
format = "with [📐 $version](blue bold) "
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le dossier courant contient un fichier avec l’extension `.red` ou `.reds`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                                            |
| `detect_extensions` | `["red"]`                            | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"red bold"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `red`.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `red`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le dossier courant contient un fichier `Gemfile`
- Le dossier courant contient un fichier `.ruby-version`
- Le dossier courant contient un fichier `.rb`
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                                           |
| `detect_extensions` | `["rb"]`                             | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `detect_variables`  | `["RUBY_VERSION", "RBENV_VERSION"]`  | Which environment variables should trigger this module.                                    |
| `style`             | `"bold red"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `ruby`.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `ruby`                  |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le dossier courant contient un fichier `Cargo.toml`
- Le dossier courant contient un fichier avec l’extension `.rs`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                                            |
| `detect_extensions` | `["rs"]`                             | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Cargo.toml"]`                     | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold red"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `rust`.                                                                |

### Variables

| Variable  | Exemple           | Description                            |
| --------- | ----------------- | -------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                 |
| symbol    |                   | Reflète la valeur de l'option `symbol` |
| style\* |                   | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier `build.sbt`, `.scalaenv` ou `.sbtenv`
- Le dossier courant contient un fichier avec l’extension `.scala` ou `.sbt`
- The current directory contains a directory named `.metals`

### Options

| Option              | Défaut                                   | Description                                                                                |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[".metals"]`                            | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.                                          |
| `style`             | `"red dimmed"`                           | Le style du module.                                                                        |
| `disabled`          | `false`                                  | Désactive le module `scala`.                                                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `2.13.5` | The version of `scala`                 |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Option                 | Défaut                    | Description                                                  |
| ---------------------- | ------------------------- | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`                     | A format string used to represent bash.                      |
| `fish_indicator`       | `fsh`                     | A format string used to represent fish.                      |
| `zsh_indicator`        | `zsh`                     | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`                     | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`                     | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`                     | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`                     | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `xsh`                     | A format string used to represent xonsh.                     |
| `cmd_indicator`        | `cmd`                     | A format string used to represent cmd.                       |
| `nu_indicator`         | `nu`                      | A format string used to represent nu.                        |
| `unknown_indicator`    |                           | The default value to be displayed when the shell is unknown. |
| `format`               | `"[$indicator]($style) "` | Format du module.                                            |
| `style`                | `"white bold"`            | Le style du module.                                          |
| `disabled`             | `true`                    | Désactive le module `shell`.                                 |

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
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "shell mystère"
style = "cyan bold"
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Options

| Option      | Défaut                       | Description                                                   |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | Format du module.                                             |
| `symbol`    | `"↕️  "`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | Le style du module.                                           |
| `disabled`  | `true`                       | Désactive le module `shlvl`.                                  |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | The current value of `SHLVL`           |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### Options

| Option     | Défaut                           | Description                                      |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Format du module.                                |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | Le style du module.                              |
| `disabled` | `false`                          | Désactive le module `singularity`.               |

### Variables

| Variable  | Exemple      | Description                            |
| --------- | ------------ | -------------------------------------- |
| env       | `centos.img` | The current Singularity image          |
| symbol    |              | Reflète la valeur de l'option `symbol` |
| style\* |              | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`. The status code will cast to a signed 32-bit integer.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Option                  | Défaut                                                                               | Description                                             |
| ----------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "`                                                        | The format of the module                                |
| `symbol`                | `"✖"`                                                                                | The symbol displayed on program error                   |
| `success_symbol`        | `""`                                                                                 | The symbol displayed on program success                 |
| `not_executable_symbol` | `"🚫"`                                                                                | The symbol displayed when file isn't executable         |
| `not_found_symbol`      | `"🔍"`                                                                                | The symbol displayed when the command can't be found    |
| `sigint_symbol`         | `"🧱"`                                                                                | The symbol displayed on SIGINT (Ctrl + c)               |
| `signal_symbol`         | `"⚡"`                                                                                | The symbol displayed on any signal                      |
| `style`                 | `"bold red"`                                                                         | Le style du module.                                     |
| `recognize_signal_code` | `true`                                                                               | Enable signal mapping from exit code                    |
| `map_symbol`            | `false`                                                                              | Enable symbols mapping from exit code                   |
| `pipestatus`            | `false`                                                                              | Enable pipestatus reporting                             |
| `pipestatus_separator`  | `|`                                                                                  |                                                         |
| `pipestatus_format`     | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | The format of the module when the command is a pipeline |
| `disabled`              | `true`                                                                               | Désactiver le module `status`.                          |

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
| symbol         |         | Reflète la valeur de l'option `symbol`                                                      |
| style\*      |         | Reflète la valeur de l'option `style`                                                       |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Option          | Défaut                  | Description                                             |
| --------------- | ----------------------- | ------------------------------------------------------- |
| `format`        | `[as $symbol]($style)"` | The format of the module                                |
| `symbol`        | `"🧙 "`                  | The symbol displayed when credentials are cached        |
| `style`         | `"bold blue"`           | Le style du module.                                     |
| `allow_windows` | `false`                 | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                  | Désactive le module `sudo`.                             |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le dossier courant contient un fichier `Package.swift`
- Le dossier courant contient un fichier avec l’extension `.swift`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift                                           |
| `detect_extensions` | `["swift"]`                          | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Package.swift"]`                  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold 202"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactiver le module `swift`.                                                              |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.2.4` | The version of `swift`                 |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

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

- The current directory contains a `.terraform` folder
- Le dossier courant contient un fichier avec l’extension `.tf`, `.tfplan` ou `.tfstate`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol$workspace]($style) "` | La chaîne de format pour le module.                                                        |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                                      |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]`        | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[".terraform"]`                     | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold 105"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `terraform`.                                                           |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `terraform`             |
| workspace | `default`  | The current Terraform workspace        |
| symbol    |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

#### Avec la version de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Sans la version de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Date et Heure

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Option            | Défaut                  | Description                                                                                                                        |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | La chaîne de format pour le module.                                                                                                |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Désactiver le module `time`.                                                                                                       |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

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
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. Le module est affiché si l'une de ces conditions est remplie :

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Options

| Option        | Défaut                  | Description                             |
| ------------- | ----------------------- | --------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root.   |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.      |
| `format`      | `"[$user]($style) in "` | Format du module.                       |
| `show_always` | `false`                 | Toujours afficher le module `username`. |
| `disabled`    | `false`                 | Désactiver le module `username`.        |

### Variables

| Variable | Exemple      | Description                                                                                 |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `"matchai"`  | The currently logged-in user ID.                                                            |

### Exemple

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

- Le dossier courant contient un fichier `Vagrantfile`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.                                        |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Vagrantfile"]`                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"cyan bold"`                        | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `vagrant`.                                                             |

### Variables

| Variable  | Exemple          | Description                            |
| --------- | ---------------- | -------------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`               |
| symbol    |                  | Reflète la valeur de l'option `symbol` |
| style\* |                  | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). By default the module will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier avec l’extension `.v`
- Le dossier courant contient un fichier `v.mod`, `vpkg.json` ou `.vpkg-lock.json`

### Options

| Option              | Défaut                                       | Description                                                                                |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`         | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"V "`                                       | A format string representing the symbol of V                                               |
| `detect_extensions` | `["v"]`                                      | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                         | Les dossiers qui activent ce module.                                                       |
| `style`             | `"blue bold"`                                | Le style du module.                                                                        |
| `disabled`          | `false`                                      | Désactive le module `vlang`.                                                               |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| version   | `v0.2`  | The version of `v`                     |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml
[vlang]
format = "via [V $version](blue bold) "
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### Options

| Option     | Défaut                           | Description                                            |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   |                                  | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | Le style du module.                                    |
| `format`   | `"vcsh [$symbol$repo]($style) "` | Format du module.                                      |
| `disabled` | `false`                          | Désactive le module `vcsh`.                            |

### Variables

| Variable  | Exemple                                     | Description                            |
| --------- | ------------------------------------------- | -------------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name             |
| symbol    |                                             | Reflète la valeur de l'option `symbol` |
| style\* | `black bold dimmed`                         | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le dossier courant contient un fichier `.zip`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.                                      |
| `style`             | `"bold yellow"`                      | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `zig`.                                                                 |
| `detect_extensions` | `["zig"]`                            | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.6.0` | La version de `zip`                    |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Commandes personnalisées

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- Le dossier courant contient un fichier dont le nom est dans `files`
- The current directory contains a directory whose name is in `directories`
- Le dossier courant contient un fichier dont l’extension est dans `extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matchs with `os` field if defined.

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

| Option        | Défaut                          | Description                                                                                                                                                                   |
| ------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | `""`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                 |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code.                                                    |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                                                                            |
| `description` | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                                                                                  |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                         |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                   |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                    |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                                                                         |
| `style`       | `"bold green"`                  | Le style du module.                                                                                                                                                           |
| `format`      | `"[$symbol($output )]($style)"` | Format du module.                                                                                                                                                             |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                                                                                |
| `os`          |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html). |

### Variables

| Variable  | Description                            |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Reflète la valeur de l'option `symbol` |
| style\* | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

#### Commandes shell personnalisées

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

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
command = "echo foo" # affiche la sortie de la commande
files = ["foo"] # possible de spécifier des filtres, mais les jokers ne sont pas supportés
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
extensions = ["pst"] # filtre les fichiers *.pst
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
