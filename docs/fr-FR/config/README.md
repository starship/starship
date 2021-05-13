# Configuration

Pour commencer à configurer starship, créez le fichier suivant : `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toute la configuration de starship est faite dans ce fichier [TOML](https://github.com/toml-lang/toml):

```toml
# Insérer une ligne vide entre deux invites
add_newline = true

# Remplacer le "❯" dans l'invite par "➜"
[character]                            # On configure le module appelé "character"
success_symbol = "[➜](bold green)"     # Le segment "success_symbol" est configuré sur la valeur "➜" avec la couleur "bold green" (vert, gras)

# Désactiver le module "package", pour le supprimer totalement de l'invite
[package]
disabled = true
```

Vous pouvez changer l'emplacement par défaut du fichier  avec la variable d'environnement `STARSHIP_CONFIG` :

```sh
export STARSHIP_CONFIG=~/.starship/config.toml
```

De manière équivalente, pour Powershell (Windows), ajoutez la ligne suivante à votre `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
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

### Terminologie

**Module**: Un composant dans l'invite donnant des informations basées sur des informations contextuelles à propos de votre Système d'Exploitation. Par exemple, le module "nodejs" montre la version de Node.js qui est actuellement installée sur votre ordinateur, si votre répertoire actuel est un projet Node.js.

**Variable**: Petits sous-composants qui contiennent des informations fournies par le module. Par exemple, la variable "version" dans le module "nodejs" contient la version actuelle de Node.js.

Par convention, la plupart des modules ont un préfixe de la couleur par défaut du terminal (par exemple `via` dans "nodejs") et un espace vide comme suffixe.

### Chaîne de formatage

Les chaînes de formatage sont le format avec lequel un module affiche toutes ses variables. La plupart des modules ont une entrée appelée `format` qui configure le format d'affichage du module. Vous pouvez utiliser des textes, des variables et des groupes de texte dans une chaîne de format.

#### Variable

Une variable contient un symbole `$` suivi du nom de la variable. Le nom d'une variable ne contient que des lettres, des chiffres et `_`.

Par exemple :

- `$version` est une chaîne de formatage avec une variable nommée `version`.
- `$git_branch$git_commit` est une chaîne de formatage avec deux variables appelées `git_branch` et `git_commit`.
- `$git_branch $git_commit` a les deux variables séparées par un espace.

#### Groupe de texte

Un groupe de texte se compose de deux parties différentes.

La première partie, qui est entourée dans un `[]`, est une [chaîne de formatage](#format-strings). Vous pouvez y ajouter des textes, des variables, ou même des groupes de texte imbriqués.

La deuxième partie, qui est entourée par `()`, est une [chaîne de style](#style-strings). Ceci peut être utilisé pour styliser la première partie.

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

- `(@$region)` ne montrera rien si la variable `région` est `None`, sinon `@` suivi de la valeur de la région.
- `(some text)` ne montrera toujours rien puisqu'il n'y a pas de variables enveloppées dans les accolades.
- Lorsque `$all` est un raccourci pour `\[$a$b\]`, `($all)` ne montrera rien que si `$a` et `$b` sont tous les deux `None`. Cela fonctionne comme `(\[$a$b\] )`.

#### Caractère d’échappement

Les symboles suivants ont une utilisation spéciale dans une chaîne de formatage. Si vous voulez afficher les symboles suivants, vous devez les échapper avec un antislash (`\`).

- \$
- \\
- [
- ]
- (
- )

Notez que `toml` a [sa propre syntaxe d'échappement](https://github.com/toml-lang/toml#user-content-string). Il est recommandé d'utiliser une chaîne littérale (`''`) dans votre configuration. Si vous voulez utiliser une chaîne de base (`""`), faites attention à l'échappement de l'antislash `\`.

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
| `scan_timeout`    | `30`                           | Délai maximal pour le scan des fichiers par starship (en millisecondes).    |
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

# Attendez 10 millisecondes pour que starship vérifie les fichiers dans le répertoire de travail.
scan_timeout = 10

# Désactive la nouvelle ligne au début de l'invite
add_newline = false
```

### Format par Défaut

Le `format` par défaut est utilisé pour définir le format de l'invite, si il est vide ou mal `formaté`. La valeur par défaut est la suivante :

```toml
format = "$all"

# Est équivalent à
format = """
$username\
$hostname\
$shlvl\
$kubernetes\
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$kotlin\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
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
$env_var\
$crystal\
$custom\
$cmd_duration\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

## AWS

Le module `aws` affiche la région et le profil AWS courant. Ces informations sont basées sur les variables d'environnement `AWS_REGION`, `AWS_DEFAULT_REGION`, et `AWS_PROFILE` ainsi que le fichier `~/.aws/config`. This module also shows an expiration timer when using temporary credentials.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var and the credentials expiration date is read from the `AWS_SESSION_EXPIRATION` env var.

Lorsque vous utilisez [awsu](https://github.com/kreuzwerker/awsu) le profil est lu depuis la variable d'environnement `AWSU_PROFILE`.

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var and the credentials expiration date is read from the `AWSUME_EXPIRATION` env var.

### Options

| Option              | Default                                                              | Description                                                       |
| ------------------- | -------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\])]($style)'` | Format du module.                                                 |
| `symbol`            | `"☁️ "`                                                              | Le symbole affiché avant le profil AWS actuel.                    |
| `region_aliases`    |                                                                      | Table des alias de région à afficher en plus du nom AWS.          |
| `style`             | `"bold yellow"`                                                      | Le style du module.                                               |
| `expiration_symbol` | `X`                                                                  | The symbol displayed when the temporary credentials have expired. |
| `disabled`          | `false`                                                              | Disables the `AWS` module.                                        |

### Variables

| Variable  | Exemple          | Description                                 |
| --------- | ---------------- | ------------------------------------------- |
| region    | `ap-northeast-1` | La région AWS actuelle                      |
| profile   | `astronauts`     | Le profil AWS actuel                        |
| duration  | `2h27m20s`       | The temporary credentials validity duration |
| symbol    |                  | Reflète la valeur de l'option `symbol`      |
| style\* |                  | Reflète la valeur de l'option `style`       |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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
```

## Battery

Le module `battery` montre à quel point la batterie de l'appareil est chargée et son état de charge actuel. Ce module n'est visible que lorsque la batterie de l'appareil est inférieure à 10%.

### Options

| Option               | Default                           | Description                                                   |
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

L'option `display` est un tableau des propriétés suivantes.

| Option               | Défaut     | Description                                                                                                                             |
| -------------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`       | La limite supérieure pour l'option display.                                                                                             |
| `style`              | `bold red` | Le style de l'option display si elle est utilisée.                                                                                      |
| `charging_symbol`    | `-`        | Symbole optionnel affiché si l'option display est utilisée, la valeur par défaut est l'option `charging_symbol` du module "battery".    |
| `discharging_symbol` | `-`        | Symbole optionnel affiché si l'option display est utilisée, la valeur par défaut est l'option `discharging_symbol` du module "battery". |

#### Exemple

```toml
[[battery.display]]  # Style "rouge gras" et affichage de discharging_symbol lorsque la batterie est entre 0% et 10% de sa capacité
threshold = 10
style = "bold red"

[[battery.display]]  # Style "jaune gras" et affichage de💦 lorsque la batterie est entre 10% et 30% de sa capacité
threshold = 30
style = "bold yellow"
discharging_symbol = 💦

# Au-dessus de 30% de charge, l'indicateur de batterie ne sera pas affiché

```

## Caractères

Le module `character` affiche un caractère (habituellement une flèche) à côté de l'endroit où le texte est entré dans votre terminal.

Le caractère vous dira si la dernière commande a été réussie ou pas. Il y a deux façons de faire :

- changement de couleur (`red`/`green`)
- changement de forme (`❯`/`✖`)

Par défaut, seule la couleur change. Si vous désirez également changer sa forme, jetez un œil à [cet exemple](#with-custom-error-shape).

::: warning `error_symbol` n'est pas supporté avec elvish. :::

### Options

| Option           | Défaut              | Description                                                                   |
| ---------------- | ------------------- | ----------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | Le format utilisé avant l'entrée de texte.                                    |
| `success_symbol` | `"[❯](bold green)"` | Le format utilisé avant l'entrée de texte si la commande précédente a réussi. |
| `error_symbol`   | `"[❯](bold red)"`   | Le format utilisé avant l'entrée de texte si la commande précédente a échoué. |
| `vicmd_symbol`   | `"[❮](bold green)"` | Le format utilisé avant l'entrée de texte si le shell est en mode vim normal. |
| `disabled`       | `false`             | Désactive le module `character`.                                              |

### Variables

| Variable | Exemple | Description                                                     |
| -------- | ------- | --------------------------------------------------------------- |
| symbol   |         | Reflète soit `success_symbol`, `error_symbol` ou `vicmd_symbol` |

### Exemples

#### Avec une forme d'erreur personnalisée

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Sans forme d'erreur personnalisée

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

Le module `cmake` affiche la version actuellement installée de [CMake](https://cmake.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire actuel contient un fichier `CMakeLists.txt`
- Le répertoire actuel contient un fichier ` CMakeCache.txt`

### Options

| Option              | Défaut                                 | Description                                                                                |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`   | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                            | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"△ "`                                 | Le symbole utilisé avant la version de cmake.                                              |
| `detect_extensions` | `[]`                                   | Quelles extensions devraient activer ce module                                             |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`    | `[]`                                   | Quels dossiers devraient activer ce module                                                 |
| `style`             | `"bold blue"`                          | Le style du module.                                                                        |
| `disabled`          | `false`                                | Désactive le module `cmake`.                                                               |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | La version de cmake                    |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

## Temps d'exécution

Le module `cmd_duration` montre combien de temps la dernière commande a pris pour s'exécuter. Le module ne sera affiché que si la commande a pris plus de deux secondes, ou plus que la valeur `min_time`, si elle existe.

::: warning N'interceptez pas le signal DEBUG avec Bash

Si vous utilisez starship avec `bash`, n'interceptez pas `DEBUG` après avoir exécuté `eval $(starship init $0)`, ou ce module **ne fonctionnera pas**.

:::

Les utilisateurs de Bash qui ont besoin de fonctionnalité pré-exec peuvent utiliser [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Définissez simplement les array `preexec_functions` et `precmd_functions` avant d'exécuter `eval $(starship init $0)`, puis procédez comme d'habitude.

### Options

| Option               | Défaut                        | Description                                                               |
| -------------------- | ----------------------------- | ------------------------------------------------------------------------- |
| `min_time`           | `2_000`                       | Durée la plus courte quand afficher le temps (en millisecondes).          |
| `show_milliseconds`  | `false`                       | Afficher les millisecondes en plus des secondes pendant la durée.         |
| `format`             | `"took [$duration]($style) "` | Format du module.                                                         |
| `style`              | `"bold yellow"`               | Le style du module.                                                       |
| `disabled`           | `false`                       | Désactive le module `cmd_duration`.                                       |
| `show_notifications` | `false`                       | Afficher les notifications du bureau lorsque la commande est terminée.    |
| `min_time_to_notify` | `45_000`                      | Durée minimale après laquelle activer la notification (en millisecondes). |

::: tip

L'affichage des notifications de bureau nécessite que starship soit compilé avec la prise en charge de `rust-notify`. Vérifiez si starship supporte les notifications en exécutant `STARSHIP_LOG=debug starship module cmd_duration -d 60000` lorsque `show_notifications` est défini à `true`.

:::

### Variables

| Variable  | Exemple  | Description                                   |
| --------- | -------- | --------------------------------------------- |
| duration  | `16m40s` | Le temps nécessaire pour exécuter la commande |
| style\* |          | Reflète la valeur de l'option `style`         |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

Le module `conda` affiche l'environnement conda actuel, si `$CONDA_DEFAULT_ENV` est défini.

::: tip

Cela ne supprime pas le modificateur d'invite de conda, vous pourriez vouloir exécuter `conda config --set changeps1 False` pour le désactiver.

:::

### Options

| Option              | Défaut                                 | Description                                                                                                                                                                                                                                   |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | Le nombre de répertoires dans lesquels le chemin d'environnement (Path) doit être tronqué, si l'environnement a été créé via `conda create -p [path]`. `0` ne signifie pas de troncature. Regardez aussi le module [`directory`](#directory). |
| `symbol`            | `"🅒 "`                                 | Le symbole utilisé avant le nom d'environnement.                                                                                                                                                                                              |
| `style`             | `"bold green"`                         | Le style du module.                                                                                                                                                                                                                           |
| `format`            | `"via [$symbol$environment]($style) "` | Format du module.                                                                                                                                                                                                                             |
| `ignore_base`       | `true`                                 | Ignore l'environnement `base` lorsqu'il est activé.                                                                                                                                                                                           |
| `disabled`          | `false`                                | Désactive le module `conda`.                                                                                                                                                                                                                  |

### Variables

| Variable      | Exemple      | Description                            |
| ------------- | ------------ | -------------------------------------- |
| environnement | `astronauts` | La version courante de conda           |
| symbol        |              | Reflète la valeur de l'option `symbol` |
| style\*     |              | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

Le module `crystal` affiche la version actuellement installée de [Crystal](https://crystal-lang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `shard.yml`
- Le répertoire courant contient un fichier `.cr`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `symbol`            | `"🔮 "`                               | Le symbole utilisé avant d'afficher la version de crystal.                                 |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold green"`                       | Le style du module.                                                                        |
| `detect_extensions` | `["cr"]`                             | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["shard.yml"]`                      | Quels fichiers devraient activer ce module.                                                |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `disabled`          | `false`                              | Désactive le module `crystal`.                                                             |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | La version de `cristal`                |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

Le module `dart` affiche la version actuellement installée de [Dart](https://dart.dev/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `.dart`
- Le répertoire courant contient un répertoire `.dart_tool`
- Le répertoire courant contient un fichier `pubspec.yaml`, `pubspec.yml` ou `pubspec.lock`

### Options

| Option              | Défaut                                            | Description                                                                                |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`              | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                       | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | Une chaîne de caractères représentant le symbole de Dart                                   |
| `detect_extensions` | `["dart"]`                                        | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Quels fichiers devraient activer ce module.                                                |
| `detect_folders`    | `[".dart_tool"]`                                  | Quels dossiers devraient activer ce module.                                                |
| `style`             | `"bold blue"`                                     | Le style du module.                                                                        |
| `disabled`          | `false`                                           | Désactive le module `dart`.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.8.4` | La version de `dart`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

Le module `deno` affiche la version actuellement installée de [Deno](https://deno.land/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:
- Le répertoire courant contient un fichier `mod.ts`, `mod.js`, `deps.ts` ou `deps.ts`

### Options

| Option              | Défaut                                       | Description                                                                                |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`         | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                       | Une chaîne de caractères représentant le symbole de Deno                                   |
| `detect_extensions` | `[]`                                         | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["mod.ts", "mod.js", "deps.ts", "deps.js"]` | Quels fichiers devraient activer ce module.                                                |
| `detect_folders`    | `[]`                                         | Quels dossiers devraient activer ce module.                                                |
| `style`             | `"green bold"`                               | Le style du module.                                                                        |
| `disabled`          | `false`                                      | Désactive le module `deno`.                                                                |

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

Le module `Dossier` montre le chemin vers votre répertoire courant, tronqué à trois dossiers parents. Votre répertoire sera également tronqué à la racine du repo git dans lequel vous vous trouvez actuellement.

Lorsque vous utilisez le style de pwd de fish, au lieu de cacher le chemin tronqué, vous verrez une abréviation du nom de chaque dossier, en fonction du nombre que vous avez utilisé comme valeur.

Par exemple, `~/Dev/Nix/nixpkgs/pkgs` où `nixpkgs` est la racine du repo, et l'option définie à `1`. Vous verrez maintenant `~/D/N/nixpkgs/pkgs`, alors que vous auriez vu `nixpkgs/pkgs` avant.

### Options

| Option              | Défaut                                             | Description                                                                    |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------ |
| `truncation_length` | `3`                                                | Le nombre de dossiers parents auquel tronquer le chemin du répertoire courant. |
| `truncate_to_repo`  | `true`                                             | Si oui ou non tronquer à la racine du repo git dans lequel vous vous trouvez.  |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | Format du module.                                                              |
| `style`             | `"bold cyan"`                                      | Le style du module.                                                            |
| `disabled`          | `false`                                            | Désactive le module `directory`.                                               |
| `read_only`         | `"🔒"`                                              | Le symbole indiquant que le répertoire courant est en lecture seule.           |
| `read_only_style`   | `"red"`                                            | Le style du symbole de lecture seule.                                          |
| `truncation_symbol` | `""`                                               | Le symbole pour préfixer les chemins tronqués. ex: "…/"                        |
| `home_symbol`       | `"~"`                                              | Le symbole indiquant le répertoire personnel.                                  |

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
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_leng` interagit avec les options de troncature d'une manière qui peut être surprenante au début : si elle n'est pas nulle, les composantes du chemin qui seraient normalement tronquées sont affichées à la place avec autant de caractères. Par exemple, le chemin `/built/this/city/on/rock/and/roll`, qui devrait normalement être affiché comme `rock/and/roll`, sera affiché sous la forme de `/b/t/c/o/rock/and/roll` avec `fish_style_pwd_dir_length = 1`--les composants de chemin qui seraient normalement supprimés sont affichés avec un caractère unique. Pour `fish_style_pwd_dir_length = 2`, ce serait `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable  | Exemple               | Description                           |
| --------- | --------------------- | ------------------------------------- |
| path      | `"D:/Projects"`       | Le chemin du répertoire courant       |
| style\* | `"black bold dimmed"` | Reflète la valeur de l'option `style` |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Contexte Docker

Le module `docker_context` affiche le [contexte Docker](https://docs.docker.com/engine/context/working-with-contexts/) actuellement actif s'il n'est pas réglé à `default`.

### Options

| Option              | Défaut                                                        | Description                                                                                            |
| ------------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol$context]($style) "`                            | Format du module.                                                                                      |
| `symbol`            | `"🐳 "`                                                        | Le symbole utilisé avant d'afficher le contexte Docker.                                                |
| `only_with_files`   | `true`                                                        | Afficher uniquement quand il y a une correspondance                                                    |
| `detect_extensions` | `[]`                                                          | Quelles extensions devraient activer ce module (il faut que `only_with_files` soit réglé sur true).    |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Quels noms de fichier devraient activer ce module (il faut que `only_with_files` soit réglé sur true). |
| `detect_folders`    | `[]`                                                          | Quels dossiers devraient activer ce module (il faut que `only_with_files` soit réglé sur true).        |
| `style`             | `"blue bold"`                                                 | Le style du module.                                                                                    |
| `disabled`          | `false`                                                       | Désactive le module `docker_context`.                                                                  |

### Variables

| Variable  | Exemple        | Description                            |
| --------- | -------------- | -------------------------------------- |
| context   | `test_context` | Le contexte actuel de Docker           |
| symbol    |                | Reflète la valeur de l'option `symbol` |
| style\* |                | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

Le module `dotnet` montre la version pertinente du [SDK .NET Core](https://dotnet.microsoft.com/) pour le répertoire courant. Si le SDK a été épinglé dans le répertoire courant, la version épinglée est affichée. Sinon, le module affiche la dernière version installée du SDK.

Par défaut, ce module ne sera affiché dans votre invite que lorsqu'un ou plusieurs des fichiers suivants sont présents dans le répertoire courant :

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.sln`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

Vous aurez également besoin du SDK .NET Core pour pouvoir l'utiliser correctement.

En interne, ce module utilise son propre mécanisme de détection de version. Généralement, il est deux fois plus rapide que d'exécuter `dotnet --version`, mais il peut afficher une version incorrecte si votre projet .NET a une arborescence inhabituelle. Si la précision est plus importante que la vitesse, vous pouvez désactiver le mécanisme en définissant `heuristic = false` dans les options du module.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Options

| Option              | Défaut                                                                                                  | Description                                                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"[$symbol($version )(🎯 $tfm )]($style)"`                                                               | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                                                                             | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | Le symbole utilisé avant d'afficher la version de dotnet.                                  |
| `heuristic`         | `true`                                                                                                  | Utilisez la détection de versions plus rapide pour garder starship instantané.             |
| `detect_extensions` | `["sln", "csproj", "fsproj", "xproj"]`                                                                  | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Quels fichiers devraient activer ce module.                                                |
| `detect_folders`    | `[]`                                                                                                    | Quels dossiers devraient activer ce module.                                                |
| `style`             | `"bold blue"`                                                                                           | Le style du module.                                                                        |
| `disabled`          | `false`                                                                                                 | Désactive le module `dotnet`.                                                              |

### Variables

| Variable  | Exemple          | Description                                                        |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | La version du sdk `dotnet`                                         |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Reflète la valeur de l'option `symbol`                             |
| style\* |                  | Reflète la valeur de l'option `style`                              |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

Le module `elixir` montre la version actuellement installée de [Elixir](https://elixir-lang.org/) et [Erlang/OTP](https://erlang.org/doc/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `mix.exs`.

### Options

| Option              | Défaut                                                      | Description                                                                                |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | Format du module elixir.                                                                   |
| `version_format`    | `"v${raw}"`                                                 | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | Le symbole utilisé avant d'afficher la version d'Elixir/Erlang.                            |
| `detect_extensions` | `[]`                                                        | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["mix.exs"]`                                               | Quels fichiers devraient activer ce module.                                                |
| `detect_folders`    | `[]`                                                        | Quels dossiers devraient activer ce module.                                                |
| `style`             | `"bold purple"`                                             | Le style du module.                                                                        |
| `disabled`          | `false`                                                     | Désactive le module `elixir`.                                                              |

### Variables

| Variable    | Exemple | Description                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | La version d' `elixir`                 |
| otp_version |         | La version otp d' `elixir`             |
| symbol      |         | Reflète la valeur de l'option `symbol` |
| style\*   |         | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

Le module `elm` montre la version actuellement installée de [Elm](https://elm-lang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `elm.json`
- Le répertoire courant contient un fichier `elm-package.json`
- Le répertoire courant contient un fichier `elm-version`
- Le répertoire courant contient un dossier `elm-stuff`
- Le répertoire courant contient des fichiers `*.elm`

### Options

| Option              | Défaut                                             | Description                                                                                |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`               | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                        | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | Une chaîne de format représentant le symbole d'Elm.                                        |
| `detect_extensions` | `["elm"]`                                          | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Quels fichiers devraient activer ce module.                                                |
| `detect_folders`    | `["elm-stuff"]`                                    | Quels dossiers devraient activer ce module.                                                |
| `style`             | `"cyan bold"`                                      | Le style du module.                                                                        |
| `disabled`          | `false`                                            | Désactive le module `elm`.                                                                 |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.19.1` | La version de `elm`                    |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Variable d'environnement

Le module `env_var` affiche la valeur actuelle d'une variable d'environnement sélectionnée. Le module est affiché si l'une de ces conditions est remplie :

- L'option `variable` correspond à une variable d'environnement existante
- L'option `variable` n'est pas définie, mais l'option `default` l'est

### Options

| Option     | Défaut                         | Description                                                                         |
| ---------- | ------------------------------ | ----------------------------------------------------------------------------------- |
| `symbol`   |                                | Le symbole utilisé avant d'afficher la valeur de la variable.                       |
| `variable` |                                | La variable d'environnement à afficher.                                             |
| `default`  |                                | La valeur par défaut à afficher lorsque la variable sélectionnée n'est pas définie. |
| `format`   | `"with [$env_value]($style) "` | Format du module.                                                                   |
| `disabled` | `false`                        | Désactive le module `env_var`.                                                      |

### Variables

| Variable  | Exemple                                  | Description                                      |
| --------- | ---------------------------------------- | ------------------------------------------------ |
| env_value | `Windows NT` (si _variable_ était `$OS`) | La valeur d'environnement de l'option `variable` |
| symbol    |                                          | Reflète la valeur de l'option `symbol`           |
| style\* | `black bold dimmed`                      | Reflète la valeur de l'option `style`            |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

Le module `erlang` montre la version actuellement installée de [Erlang/OTP](https://erlang.org/doc/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `rebar.config`.
- Le répertoire courant contient un fichier `erlang.mk`.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | Le symbole utilisé avant d'afficher la version d'erlang.                                   |
| `style`             | `"bold green"`                       | Le style du module.                                                                        |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Quels fichiers devraient activer ce module.                                                |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `disabled`          | `false`                              | Désactive le module `erlang`.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | La version d'`erlang`                  |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Google Cloud (`gcloud`)

Le module `gcloud` affiche la configuration actuelle pour [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. Ceci est basé sur les fichiers `~/.config/gcloud/active_config` et `~/.config/gcloud/configurations/config_{CONFIG NAME}` et la variable d'environnement `CLOUDSDK_CONFIG`.

### Options

| Option           | Défaut                                                     | Description                                                 |
| ---------------- | ---------------------------------------------------------- | ----------------------------------------------------------- |
| `format`         | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Format du module.                                           |
| `symbol`         | `"☁️  "`                                                   | Le symbole affiché avant le profil GCP actuel.              |
| `region_aliases` |                                                            | Table des alias de région à afficher en plus du nom du GCP. |
| `style`          | `"bold blue"`                                              | Le style du module.                                         |
| `disabled`       | `false`                                                    | Désactive le module `gcloud`.                               |

### Variables

| Variable  | Exemple       | Description                                                                   |
| --------- | ------------- | ----------------------------------------------------------------------------- |
| region    | `us-central1` | La région GCP actuelle                                                        |
| account   | `foo`         | Le profil GCP actuel                                                          |
| domain    | `exemple.com` | Le domaine du profil GCP actuel                                               |
| project   |               | Le projet GCP actuel                                                          |
| active    | `default`     | Le nom de la configuration active écrit dans `~/.config/gcloud/active_config` |
| symbol    |               | Reflète la valeur de l'option `symbol`                                        |
| style\* |               | Reflète la valeur de l'option `style`                                         |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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

## Branche Git

Le module `git_branch` affiche la branche active du dépôt dans votre répertoire courant.

### Options

| Option               | Défaut                           | Description                                                                                                                     |
| -------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Affiche le nom de la branche suivie distante, même si elle est égale au nom de la branche locale.                               |
| `format`             | `"on [$symbol$branch]($style) "` | Format du module. Utilisez `"$branch"` pour vous référer au nom de la branche courante.                                         |
| `symbol`             | `" "`                           | Une chaîne de format représentant le symbole de la branche git.                                                                 |
| `style`              | `"bold purple"`                  | Le style du module.                                                                                                             |
| `truncation_length`  | `2^63 - 1`                       | Tronque une branche git à `N` graphèmes.                                                                                        |
| `truncation_symbol`  | `"…"`                            | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué. Vous pouvez utiliser `""` pour ne pas afficher de symbole. |
| `only_attached`      | `false`                          | Ne montrer le nom de la branche que si elle n'est pas dans un état `HEAD` détachée.                                             |
| `disabled`           | `false`                          | Désactive le module `git_branch`.                                                                                               |

### Variables

| Variable      | Exemple  | Description                                                                                                          |
| ------------- | -------- | -------------------------------------------------------------------------------------------------------------------- |
| branch        | `master` | Le nom de la branche actuelle, par défaut à `HEAD` s'il n'y a pas de branche actuelle (par exemple `HEAD` détachée). |
| remote_name   | `origin` | Le nom du dépôt distant.                                                                                             |
| remote_branch | `master` | Le nom de la branche suivie sur `remote_name`.                                                                       |
| symbol        |          | Reflète la valeur de l'option `symbol`                                                                               |
| style\*     |          | Reflète la valeur de l'option `style`                                                                                |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Commit Git

Le module `git_commit` affiche le hash du commit actuel ainsi que le tag (le cas échéant) du dépôt dans votre répertoire courant.

### Options

| Option               | Défaut                                                 | Description                                                                |
| -------------------- | ------------------------------------------------------ | -------------------------------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | La longueur du hash affiché du commit git.                                 |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | Format du module.                                                          |
| `style`              | `"bold green"`                                         | Le style du module.                                                        |
| `only_detached`      | `true`                                                 | Ne montrer le hash du commit qu'en mode `HEAD` détachée.                   |
| `tag_disabled`       | `true`                                                 | Désactive l'affichage des informations du tag dans le module `git_commit`. |
| `tag_symbol`         | `" 🏷 "`                                                | Symbole préfixant les informations affichées concernant le tag             |
| `disabled`           | `false`                                                | Désactive le module `git_commit`.                                          |

### Variables

| Variable  | Exemple   | Description                           |
| --------- | --------- | ------------------------------------- |
| hash      | `b703eb3` | Le hash du commit git actuel          |
| style\* |           | Reflète la valeur de l'option `style` |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## État Git

Le module `git_state` s'affichera dans les répertoires qui font partie d'un dépôt git, dans lesquels une opération est en cours, comme : _REBASING_, _BISECTING_, etc. S'il y a des informations de progression (par exemple, REBASING 3/10), ces informations seront également affichées.

### Options

| Option         | Défaut                                                          | Description                                                                                                         |
| -------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | Une chaîne de format affichée lorsqu'un `rebase` est en cours.                                                      |
| `merge`        | `"MERGING"`                                                     | Une chaîne de format affichée quand un `merge` est en cours.                                                        |
| `revert`       | `"REVERTING"`                                                   | Une chaîne de format affichée quand un `revert` est en cours.                                                       |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | Une chaîne de format affichée quand un `cherry-pick` est en cours.                                                  |
| `bisect`       | `"BISECTING"`                                                   | Une chaîne de format affichée quand un `bisect` est en cours.                                                       |
| `am`           | `"AM"`                                                          | Une chaîne de format affichée lorsqu'un `apply-mailbox` (`git am`) est en cours.                                    |
| `am_or_rebase` | `"AM/REBASE"`                                                   | Une chaîne de format affichée lorsqu'une `apply-mailbox` ou un `rebase` est en cours sans pouvoir les différencier. |
| `style`        | `"bold yellow"`                                                 | Le style du module.                                                                                                 |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Format du module.                                                                                                   |
| `disabled`     | `false`                                                         | Désactive le module `git_state`.                                                                                    |

### Variables

| Variable         | Exemple    | Description                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | L'état actuel du dépôt                |
| progress_current | `1`        | Progression de l'opération en cours   |
| progress_total   | `2`        | Progression maximale de l'opération   |
| style\*        |            | Reflète la valeur de l'option `style` |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Statut Git

Le module `git_status` affiche des symboles représentant l'état du dépôt dans votre répertoire courant.

### Options

| Option       | Défaut                                          | Description                         |
| ------------ | ----------------------------------------------- | ----------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status` |
| `conflicted` | `"="`                                           | This branch has merge conflicts.    |
| `ahead`      | `"⇡"`                                           | The format of `ahead`               |
| `behind`     | `"⇣"`                                           | The format of `behind`              |
| `diverged`   | `"⇕"`                                           | The format of `diverged`            |
| `untracked`  | `"?"`                                           | The format of `untracked`           |
| `stashed`    | `"$"`                                           | The format of `stashed`             |
| `modified`   | `"!"`                                           | The format of `modified`            |
| `staged`     | `"+"`                                           | The format of `staged`              |
| `renamed`    | `"»"`                                           | The format of `renamed`             |
| `deleted`    | `"✘"`                                           | The format of `deleted`             |
| `style`      | `"bold green"`                                  | Le style du module.                 |
| `disabled`   | `false`                                         | Disables the `git_status` module.   |

### Variables

The following variables can be used in `format`:

| Variable       | Description                                                                                   |
| -------------- | --------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                   |
| `ahead_behind` | Displays `diverged` `ahead` or `behind` format string based on the current status of the repo |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                   |
| `untracked`    | Displays `untracked` when there are untracked files in the working directory.                 |
| `stashed`      | Displays `stashed` when a stash exists for the local repository.                              |
| `modified`     | Displays `modified` when there are file modifications in the working directory.               |
| `staged`       | Displays `staged` when a new file has been added to the staging area.                         |
| `renamed`      | Displays `renamed` when a renamed file has been added to the staging area.                    |
| `deleted`      | Displays `deleted` when a file's deletion has been added to the staging area.                 |
| style\*      | Reflète la valeur de l'option `style`                                                         |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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
untracked = "🤷‍"
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

## Golang

The `golang` module shows the currently installed version of [Golang](https://golang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `go.mod`
- Le répertoire courant contient un fichier `go.sum`
- Le répertoire courant contient un fichier `glide.yaml`
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Options

| Option              | Défaut                                                                         | Description                                                               |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                           | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                                                                    | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                         | A format string representing the symbol of Go.                            |
| `detect_extensions` | `["go"]`                                                                       | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `["Godeps"]`                                                                   | Quels dossiers devraient activer ce module.                               |
| `style`             | `"bold cyan"`                                                                  | Le style du module.                                                       |
| `disabled`          | `false`                                                                        | Disables the `golang` module.                                             |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | The version of `go`                    |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `helmfile.yaml`
- The current directory contains a `Chart.yaml` file

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm.                          |
| `style`             | `"bold white"`                       | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `helm` module.                                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | The version of `helm`                  |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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
| `disabled` | `false`                     | Disables the `hostname` module.                                                                                                      |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Options

| Option              | Défaut                                                                                                    | Description                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                                                                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                                                                                      | Quels dossiers devraient activer ce module.                               |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java                           |
| `style`             | `"red dimmed"`                                                                                            | Le style du module.                                                       |
| `disabled`          | `false`                                                                                                   | Disables the `java` module.                                               |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| version   | `v14`   | La version de `java`                   |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

::: warning

This module is not supported on tcsh.

:::

### Options

| Option      | Défaut                        | Description                                      |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | Show number of jobs if exceeded.                 |
| `format`    | `"[$symbol$number]($style) "` | Format du module.                                |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | Le style du module.                              |
| `disabled`  | `false`                       | Disables the `jobs` module.                      |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | The number of jobs                     |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.                         |
| `style`             | `"bold purple"`                      | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `julia` module.                                              |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.4.0` | The version of `julia`                 |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `.kt` or a `.kts` file

### Options

| Option              | Défaut                               | Description                                                                   |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                             |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`     |
| `detect_extensions` | `["kt", "kts"]`                      | Quelles extensions devraient activer ce module.                               |
| `detect_files`      | `[]`                                 | Quels fichiers devraient activer ce module.                                   |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                   |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `"bold blue"`                        | Le style du module.                                                           |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`                |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option            | Défaut                                               | Description                                                           |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | Format du module.                                                     |
| `style`           | `"cyan bold"`                                        | Le style du module.                                                   |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Disables the `kubernetes` module.                                     |

### Variables

| Variable  | Exemple              | Description                              |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Reflète la valeur de l'option `symbol`   |
| style\* |                      | Reflète la valeur de l'option `style`    |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
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

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Options

| Option              | Défaut                               | Description                                                                |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                          |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`  |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | Quelles extensions devraient activer ce module.                            |
| `detect_files`      | `[".lua-version"]`                   | Quels fichiers devraient activer ce module.                                |
| `detect_folders`    | `["lua"]`                            | Quels dossiers devraient activer ce module.                                |
| `style`             | `"bold blue"`                        | Le style du module.                                                        |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | The version of `lua`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option      | Défaut                                          | Description                                              |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | Format du module.                                        |
| `symbol`    | `"🐏"`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                           | Le style du module.                                      |
| `disabled`  | `true`                                          | Disables the `memory_usage` module.                      |

### Variables

| Variable         | Exemple       | Description                                                        |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Reflète la valeur de l'option `symbol`                             |
| style\*        |               | Reflète la valeur de l'option `style`                              |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

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
| `truncation_symbol` | `"…"`                            | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué.                         |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| branch    | `master` | The active mercurial branch            |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `nim.cfg`
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module                                                          |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.                     |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["nim.cfg"]`                        | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `style`             | `"bold yellow"`                      | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `nim` module.                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.2.0` | The version of `nimc`                  |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| state     | `pure`  | The state of the nix-shell             |
| name      | `lorri` | The name of the nix-shell              |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `package.json`
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Options

| Option              | Défaut                               | Description                                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                             |
| `symbol`            | `" "`                               | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Quelles extensions devraient activer ce module.                                                       |
| `detect_files`      | `["package.json", ".node-version"]`  | Quels fichiers devraient activer ce module.                                                           |
| `detect_folders`    | `["node_modules"]`                   | Quels dossiers devraient activer ce module.                                                           |
| `style`             | `"bold green"`                       | Le style du module.                                                                                   |
| `disabled`          | `false`                              | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | The version of `node`                  |
| symbol    |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Options

| Option                    | Défaut                                                                     | Description                                                               |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | The format string for the module.                                         |
| `version_format`          | `"v${raw}"`                                                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml.                   |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch.                   |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.                    |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Quelles extensions devraient activer ce module.                           |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Quels fichiers devraient activer ce module.                               |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Quels dossiers devraient activer ce module.                               |
| `style`                   | `"bold yellow"`                                                            | Le style du module.                                                       |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                              |

### Variables

| Variable         | Exemple      | Description                                                       |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Reflète la valeur de l'option `symbol`                            |
| style\*        |              | Reflète la valeur de l'option `style`                             |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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
| `disabled` | `false`                                             | Disables the `openstack` module.                               |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | The current OpenStack cloud            |
| project   | `dev`   | The current OpenStack project          |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present
- [**julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present
- [**mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present
- [**helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present
- [**maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present
- [**meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present
- [**vlang**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Options

| Option            | Défaut                            | Description                                                |
| ----------------- | --------------------------------- | ---------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | Format du module.                                          |
| `symbol`          | `"📦 "`                            | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"`                      | Le style du module.                                        |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`                           | Disables the `package` module.                             |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.0.0` | The version of your package            |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Options

| Option              | Défaut                                                                                                   | Description                                                               |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                                                                                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl                     |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                                                                                     | Quels dossiers devraient activer ce module.                               |
| `style`             | `"bold 149"`                                                                                             | Le style du module.                                                       |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                               |

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

The `php` module shows the currently installed version of [PHP](https://www.php.net/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `composer.json`
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP.                     |
| `detect_extensions` | `["php"]`                            | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["composer.json", ".php-version"]`  | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `style`             | `"147 bold"`                         | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `php` module.                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v7.3.8` | The version of `php`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `spago.dhall`
- The current directory contains a file with the `.purs` extension

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | Le symbole utilisé avant d'afficher la version de PureScript.             |
| `detect_extensions` | `["purs"]`                           | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["spago.dhall"]`                    | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `style`             | `"bold white"`                       | Le style du module.                                                       |
| `disabled`          | `false`                              | Désactive le module `purescript`.                                         |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | The version of `purescript`            |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- Le répertoire courant contient un fichier `pyproject.toml`
- Le répertoire courant contient un fichier `requirements.txt`
- Le répertoire courant contient un fichier `setup.py`
- Le répertoire courant contient un fichier `tox.ini`
- The current directory contains a file with the `.py` extension.
- A virtual environment is currently activated

### Options

| Option               | Défaut                                                                                                       | Description                                                                            |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Format du module.                                                                      |
| `version_format`     | `"v${raw}"`                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch`              |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | Le style du module.                                                                    |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                        |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3, "python2"]`                                                                            | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `["py"]`                                                                                                     | Quelles extensions devraient activer ce module                                         |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Quels fichiers devraient activer ce module                                             |
| `detect_folders`     | `[]`                                                                                                         | Quels dossiers devraient activer ce module                                             |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                          |

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

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). Le module est affiché si l'une de ces conditions est remplie :

- The current directory contains a file with `.red` or `.reds` extension

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                           |
| `detect_extensions` | `["red"]`                            | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `[]`                                 | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `style`             | `"red bold"`                         | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `red` module.                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `red`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). Le module est affiché si l'une de ces conditions est remplie :

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                          |
| `detect_extensions` | `["rb"]`                             | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `style`             | `"bold green"`                       | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `ruby` module.                                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `ruby`                  |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). Le module est affiché si l'une de ces conditions est remplie :

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                           |
| `detect_extensions` | `["rs"]`                             | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["Cargo.toml"]`                     | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `style`             | `"bold green"`                       | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `rust` module.                                               |

### Variables

| Variable  | Exemple           | Description                            |
| --------- | ----------------- | -------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                 |
| symbol    |                   | Reflète la valeur de l'option `symbol` |
| style\* |                   | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Options

| Option              | Défaut                                   | Description                                                               |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[".metals"]`                            | Quels dossiers devraient activer ce module.                               |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.                         |
| `style`             | `"red dimmed"`                           | Le style du module.                                                       |
| `disabled`          | `false`                                  | Disables the `scala` module.                                              |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `2.13.5` | The version of `scala`                 |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option                 | Défaut       | Description                                                  |
| ---------------------- | ------------ | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`        | A format string used to represent bash.                      |
| `fish_indicator`       | `fsh`        | A format string used to represent fish.                      |
| `zsh_indicator`        | `zsh`        | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`        | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`        | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`        | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.                      |
| `unknown_indicator`    |              | The default value to be displayed when the shell is unknown. |
| `format`               | `$indicator` | Format du module.                                            |
| `disabled`             | `true`       | Disables the `shell` module.                                 |

### Variables

| Variable  | Défaut | Description                                                |
| --------- | ------ | ---------------------------------------------------------- |
| indicator |        | Mirrors the value of `indicator` for currently used shell. |

### Exemples

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
disabled = false
```

## SHLVL

The `shlvl` module shows the current `SHLVL` ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Options

| Option      | Défaut                       | Description                                                   |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | Format du module.                                             |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | Le style du module.                                           |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | The current value of `SHLVL`           |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

| Variable  | Exemple      | Description                            |
| --------- | ------------ | -------------------------------------- |
| env       | `centos.img` | The current Singularity image          |
| symbol    |              | Reflète la valeur de l'option `symbol` |
| style\* |              | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

::: warning This module is not supported on elvish shell. :::

### Options

| Option                  | Défaut                        | Description                                          |
| ----------------------- | ----------------------------- | ---------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "` | The format of the module                             |
| `symbol`                | `"✖"`                         | The symbol displayed on program error                |
| `not_executable_symbol` | `"🚫"`                         | The symbol displayed when file isn't executable      |
| `not_found_symbol`      | `"🔍"`                         | The symbol displayed when the command can't be found |
| `sigint_symbol`         | `"🧱"`                         | The symbol displayed on SIGINT (Ctrl + c)            |
| `signal_symbol`         | `"⚡"`                         | The symbol displayed on any signal                   |
| `style`                 | `"bold green"`                | Le style du module.                                  |
| `recognize_signal_code` | `true`                        | Enable signal mapping from exit code                 |
| `map_symbol`            | `false`                       | Enable symbols mapping from exit code                |
| `disabled`              | `true`                        | Disables the `status` module.                        |

### Variables

| Variable       | Exemple | Description                                                          |
| -------------- | ------- | -------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                    |
| int            | `127`   | The exit code of the last command                                    |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                  |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled      |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found         |
| symbol         |         | Reflète la valeur de l'option `symbol`                               |
| style\*      |         | Reflète la valeur de l'option `style`                                |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false

```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). Le module est affiché si l'une de ces conditions est remplie :

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift                          |
| `detect_extensions` | `["swift"]`                          | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["Package.swift"]`                  | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `style`             | `"bold 202"`                         | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `swift` module.                                              |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.2.4` | The version of `swift`                 |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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

Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                     |
| `detect_extensions` | `["tf", "hcl"]`                      | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `[]`                                 | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[".terraform"]`                     | Quels dossiers devraient activer ce module.                               |
| `style`             | `"bold 105"`                         | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `terraform` module.                                          |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `terraform`             |
| workspace | `default`  | The current Terraform workspace        |
| symbol    |            | Reflète la valeur de l'option `symbol` |
| style\* |            | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

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

## Date et Heure

Le module `time` affiche l'heure actuelle **localement**. La valeur de `format` est utilisée par le package [`chrono`](https://crates.io/crates/chrono) pour contrôler la façon dont l'heure est affichée. Consultez la [doc de chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) pour découvrir les options disponibles.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option            | Défaut                  | Description                                                                                                                                                        |
| ----------------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                                                  |
| `use_12hr`        | `false`                 | Activer le format 12h                                                                                                                                              |
| `time_format`     | voir plus bas           | Le [format chrono](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) utilisé pour formater l'heure.                                                  |
| `style`           | `"bold yellow"`         | Le style utilisé par le module                                                                                                                                     |
| `utc_time_offset` | `"local"`               | Définir le décalage horaire UTC à utiliser. Range from -24 &lt; x &lt; 24. Accepte des nombres décimaux pour s'adapter aux décalages de 30/45 minutes. |
| `disabled`        | `true`                  | Désactiver le module `time`.                                                                                                                                       |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                                                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Sinon, il est défini comme `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable  | Exemple    | Description                           |
| --------- | ---------- | ------------------------------------- |
| temps     | `13:08:10` | The current time.                     |
| style\* |            | Reflète la valeur de l'option `style` |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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

## Nom d'utilisateur

Le module `username` affiche le nom d'utilisateur de l'utilisateur actif. Le module est affiché si l'une de ces conditions est remplie :

- L'utilisateur courant est root
- L'utilisateur courant est différent de celui connecté
- L'utilisateur est actuellement connecté à une session SSH
- La variable `show_always` a comme valeur true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Options

| Option        | Défaut                  | Description                                      |
| ------------- | ----------------------- | ------------------------------------------------ |
| `style_root`  | `"bold green"`          | Le style utilisé quand l'utilisateur est root.   |
| `style_user`  | `"bold yellow"`         | Le style utilisé pour les utilisateurs non-root. |
| `format`      | `"[$user]($style) in "` | Format du module.                                |
| `show_always` | `false`                 | Toujours afficher le module `username`.          |
| `disabled`    | `false`                 | Désactiver le module `username`.                 |

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

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `Vagrantfile` file

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.                       |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `["Vagrantfile"]`                    | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |
| `style`             | `"cyan bold"`                        | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                            |

### Variables

| Variable  | Exemple          | Description                            |
| --------- | ---------------- | -------------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`               |
| symbol    |                  | Reflète la valeur de l'option `symbol` |
| style\* |                  | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## VLang

The `vlang` module shows you your currently installed version of V. By default the module will be shown if any of the following conditions are met:
- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Options

| Option              | Défaut                                       | Description                                     |
| ------------------- | -------------------------------------------- | ----------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | Format du module.                               |
| `symbol`            | `"V "`                                       | A format string representing the symbol of V    |
| `detect_extensions` | `["v"]`                                      | Quelles extensions devraient activer ce module. |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Quels fichiers devraient activer ce module.     |
| `detect_folders`    | `[]`                                         | Quels dossiers devraient activer ce module.     |
| `style`             | `"blue bold"`                                | Le style du module.                             |
| `disabled`          | `false`                                      | Disables the `vlang` module.                    |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| version   | `v0.2`  | The version of `v`                     |
| symbol    |         | Reflète la valeur de l'option `symbol` |
| style\* |         | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml
[v]
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
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variables

| Variable  | Exemple                                     | Description                            |
| --------- | ------------------------------------------- | -------------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name             |
| symbol    |                                             | Reflète la valeur de l'option `symbol` |
| style\* | `black bold dimmed`                         | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

### Exemple

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). Le module est affiché si l'une de ces conditions est remplie :

- The current directory contains a `.zig` file

### Options

| Option              | Défaut                               | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.                     |
| `style`             | `"bold yellow"`                      | Le style du module.                                                       |
| `disabled`          | `false`                              | Disables the `zig` module.                                                |
| `detect_extensions` | `["zig"]`                            | Quelles extensions devraient activer ce module.                           |
| `detect_files`      | `[]`                                 | Quels fichiers devraient activer ce module.                               |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.6.0` | The version of `zig`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

### Options

| Option        | Défaut                          | Description                                                                                                                |
| ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                  | Le style du module.                                                                                                        |
| `format`      | `"[$symbol($output )]($style)"` | Format du module.                                                                                                          |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                             |

### Variables

| Variable  | Description                            |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Reflète la valeur de l'option `symbol` |
| style\* | Reflète la valeur de l'option `style`  |

\* : Cette variable ne peut être utilisée que comme partie d'une chaîne de style

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
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
