# Configuration

Pour commencer à configurer starship, créez le fichier suivant : `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toute la configuration de starship est effectuée dans ce fichier [TOML](https://github.com/toml-lang/toml) :

```toml
# Pour avoir des complétions dans votre éditeur basées sur le schéma de la config
"$schema" = 'https://starship.rs/config-schema.json'

# Insère un ligne vide entre les invites de shells
add_newline = true

# Remplace le symbole «❯» dans l’invite par «➜»
[character] # Le nom du module que nous configurons est «character»
success_symbol = "[➜](bold green)" # Le segment «success_symbol» est mis défini à «➜» avec la couleur «bold green»

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

### Negative matching

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. These take lists of strings to match or not match. "Negative" options, those which should not be matched, are indicated with a leading "!" character. The presence of _any_ negative indicator in the directory will result in the module not being matched.

Extensions are matched against both the characters after the last dot in a filename, and the characters after the first dot in a filename. For example, `foo.bar.tar.gz` will be matched against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Files whose name begins with a dot are not considered to have extensions at all.

To see how this works in practice, you could match TypeScript but not MPEG Transport Stream files thus:

```toml
detect_extensions = ["ts", "!video.ts", "!audio.ts"]
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

# Attendez 10 millisecondes pour que starship vérifie les fichiers dans le répertoire de travail.
scan_timeout = 10

# Désactive la nouvelle ligne au début de l'invite
add_newline = false
```

### Format par Défaut

Le `format` par défaut est utilisé pour définir le format de l'invite, si il est vide ou mal `formaté`. La valeur par défaut est la suivante :

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

Si vous voulez étendre le format par défaut, pour pouvoir utiliser `$all` ; les modules que vous ajouter explicitement au format ne seront pas dupliqués. Par ex.

```toml
# Déplace le dossier à la ligne suivante
format = "$all$directory$character"
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials. The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process` or `sso_start_url` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice. If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

Lorsque vous utilisez [aws-vault](https://github.com/99designs/aws-vault) le profil est lu à partir de la variable d'environnement `AWS_VAULT` et la date d'expiration des identifiants est lue à partir de la variable d'environnement `AWS_SESSION_EXPIRATION`.

Lorsque vous utilisez [awsu](https://github.com/kreuzwerker/awsu) le profil est lu depuis la variable d'environnement `AWSU_PROFILE`.

Lorsque vous utilisez [AWSume](https://awsu.me) le profil est lu à partir de la variable d'environnement `AWSUME_PROFILE` et la date d'expiration des identifiants est lue à partir de la variable d'environnement `AWSUME_EXPIRATION`.

### Options

| Option              | Défaut                                                                | Description                                                                                                                   |
| ------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | Format du module.                                                                                                             |
| `symbol`            | `"☁️ "`                                                               | Le symbole est affiché avant le profil AWS actuel.                                                                            |
| `region_aliases`    |                                                                       | Tableau des alias de région à afficher en plus du nom AWS.                                                                    |
| `profile_aliases`   |                                                                       | Tableau des alias de profil à afficher en plus du nom AWS.                                                                    |
| `style`             | `"bold yellow"`                                                       | Le style du module.                                                                                                           |
| `expiration_symbol` | `X`                                                                   | Le symbole est affiché lorsque les identifiants temporaires ont expiré.                                                       |
| `disabled`          | `false`                                                               | Désactive le module `AWS`.                                                                                                    |
| `force_display`     | `false`                                                               | Si `true`, affiche les informations même si `credentials`, `credential_process` ou `sso_start_url` n'ont pas été configurées. |

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

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime. Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `bun.lockb`
- Le répertoire courant contient un fichier `bunfig.toml`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🍞 "`                               | Une chaîne de caractères représentant le symbole de Node.js.                               |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["bun.lockb", "bunfig.toml"]`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold green"`                       | Le style du module.                                                                        |
| `disabled`          | `false`                              | Disables the `bun` module.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.1.4` | The version of `bun`                   |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[bun]
format = "via [🍔 $version](bold green) "
```

## C

Le module `c` affiche des informations à propos de votre compilateur C. Par défaut, ce module sera affiché si le dossier courant contient un fichier `.c` ou `.h`.

### Options

| Option              | Défaut                                                                      | Description                                                                                |
| ------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version(-$name) )]($style)"`                                | La chaîne de format pour le module.                                                        |
| `version_format`    | `"v${raw}"`                                                                 | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"C "`                                                                      | Le symbole utilisé avant d’afficher les détails du compilateur                             |
| `detect_extensions` | `["c", "h"]`                                                                | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                                                        | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                        | Les dossiers qui activent ce module.                                                       |
| `commands`          | [ [ "cc", "--version" ], [ "gcc", "--version" ], [ "clang", "--version" ] ] | Comment détecter quel est le compilateur                                                   |
| `style`             | `"bold 149"`                                                                | Le style du module.                                                                        |
| `disabled`          | `false`                                                                     | Désactive le module `c`.                                                                   |

### Variables

| Variable | Exemple | Description                            |
| -------- | ------- | -------------------------------------- |
| name     | clang   | Le nom du compilateur                  |
| version  | 13.0.0  | La version du compilateur              |
| symbol   |         | Reflète la valeur de l'option `symbol` |
| style    |         | Reflète la valeur de l'option `style`  |

Notez que `version` n’est pas dans le format par défaut.

### Commandes

L’option `commands` accepte une liste de commandes pour déterminer la version du compilateur et son nom.

Chaque commande est représentée par une liste du nom de l'exécutable suivie de ses arguments, généralement quelque chose comme `["mycc", "--version"]`. Starship essayera d'exécuter chaque commande jusqu'à obtenir un résultat sur STDOUT.

Si un compilateur C n’est pas supporté par ce module, vous pouvez demander son ajout en [créant un ticket sur GitHub](https://github.com/starship/starship/).

### Exemple

```toml
# ~/.config/starship.toml

[c]
format = "via [$name $version]($style)"
```

## Caractères

Le module `character` affiche un caractère (habituellement une flèche) à côté de l'endroit où le texte est entré dans votre terminal.

Le caractère vous dira si la dernière commande a été réussie ou pas. Cela peut être fait de deux manières:

- en changeant de couleur(`red`/`green`)
- en changeant de forme (`❯`/`✖`)

Par défaut, il ne change que la couleur. Si vous désirez également changer sa forme, jetez un œil à [cet exemple](#with-custom-error-shape).

::: warning

`vimcmd_symbol` is only supported in cmd, fish and zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

:::

### Options

| Option                      | Défaut               | Description                                                                             |
| --------------------------- | -------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `"$symbol "`         | Le format utilisée avant l'entrée de texte.                                             |
| `success_symbol`            | `"[❯](bold green)"`  | Le format utilisé avant l'entrée de texte si la commande précédente a réussi.           |
| `error_symbol`              | `"[❯](bold red)"`    | Le format utilisé avant l'entrée de texte si la commande précédente a échoué.           |
| `vimcmd_symbol`             | `"[❮](bold green)"`  | Le format utilisé avant l'entrée de texte si le shell est en mode vim normal.           |
| `vimcmd_replace_one_symbol` | `"[❮](bold purple)"` | The format string used before the text input if the shell is in vim `replace_one` mode. |
| `vimcmd_replace_symbol`     | `"[❮](bold purple)"` | The format string used before the text input if the shell is in vim replace mode.       |
| `vimcmd_visual_symbol`      | `"[❮](bold yellow)"` | The format string used before the text input if the shell is in vim replace mode.       |
| `disabled`                  | `false`              | Désactive le module `character`.                                                        |

### Variables

| Variable | Exemple | Description                                                     |
| -------- | ------- | --------------------------------------------------------------- |
| symbol   |         | Reflète sois `success_symbol`, `error_symbol` ou `vicmd_symbol` |

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

Le module `cmake` affiche la version actuellement installée de [CMake](https://cmake.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le dossier courant contient un fichier `CMakeLists.txt`
- Le dossier courant contient un fichier `CMakeCache.txt`

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

*: Cette variable peut uniquement être utilisée dans une chaine de style

## COBOL / GNUCOBOL

Le module `cobol` affiche la version de COBOL installée. Par défaut, le module sera affiché si l’une de ces conditions est remplie :

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

Le module `cmd_duration` montre le temps qu'a pris la dernière commande a pris pour s'exécuter. Le module ne sera affiché que si la commande a pris plus de deux secondes, ou si la valeur de configuration `min_time` existe.

::: warning N'interceptez pas le signal DEBUG avec Bash

Si vous utilisez starship en `bash`, n'accrochez pas `DEBUG` après avoir exécuté `eval $(starship init $0)`, ou ce module **cassera**.

:::

Les utilisateurs de Bash qui ont besoin de fonctionnalité pré-exec peuvent utiliser [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Définissez simplement les array `preexec_functions` et `precmd_functions` avant d'éxécuter `eval $(starship init $0)`, puis procédez comme d'habitude.

### Options

| Option                 | Défaut                        | Description                                                                                                                                                                  |
| ---------------------- | ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Durée la plus courte quand afficher le temps (en millisecondes).                                                                                                             |
| `show_milliseconds`    | `false`                       | Afficher les millisecondes en plus des secondes pendant la durée.                                                                                                            |
| `format`               | `"took [$duration]($style) "` | Format du module.                                                                                                                                                            |
| `style`                | `"bold yellow"`               | Le style du module.                                                                                                                                                          |
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
format = "underwent [$duration](bold yellow)"
```

## Conda

Le module `conda` affiche l’environnement [Conda](https://docs.conda.io/en/latest/) courant, si `$CONDA_DEFAULT_ENV` est définie.

::: tip

Cela ne supprime pas le modificateur d'invite de conda, vous pouvez exécuter `conda config --set changeps1 False`.

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

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Conteneur

Le module `container` affiche un symbole et le nom du conteneur, si vous êtes dans un conteneur.

### Options

| Option     | Défaut                                 | Description                                          |
| ---------- | -------------------------------------- | ---------------------------------------------------- |
| `symbol`   | `"⬢"`                                  | Le symbole affiché quand vous êtes dans un conteneur |
| `style`    | `"bold red dimmed"`                    | Le style du module.                                  |
| `format`   | `"[$symbol \\[$name\\]]($style) "` | Format du module.                                    |
| `disabled` | `false`                                | Désactive le module `container`.                     |

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
| `detect_files`      | `["shard.yml"]`                      | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `disabled`          | `false`                              | Désactive le module `crystal`.                                                             |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | La version de `cristal`                |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers) SDK version when you are in the root directory of your Daml project. The `sdk-version` in the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION` environment variable. Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `daml.yaml`

### Options

| Option              | Défaut                             | Description                                                                                |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `via [$symbol($version )]($style)` | Format du module.                                                                          |
| `version_format`    | `v${raw}`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"Λ "`                             | A format string representing the symbol of Daml                                            |
| `style`             | `"bold cyan"`                      | Le style du module.                                                                        |
| `detect_extensions` | `[]`                               | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["daml.yaml"]`                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                               | Les dossiers qui activent ce module.                                                       |
| `disabled`          | `false`                            | Disables the `daml` module.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.2.0` | The version of `daml`                  |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[daml]
format = "via [D $version](bold bright-green) "
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

Le module `deno` affiche la version actuellement installée de [Deno](https://deno.land/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire actuel contient un fichier `deno.json`, `deno.jsonc`, `mod.ts`, `mod.ts`, `mod.js`, `deps.ts`, ou `deps.js`

### Options

| Option              | Défaut                                                                  | Description                                                                                |
| ------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                    | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                                             | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                                                  | Une chaîne de caractères représentant le symbole de Deno                                   |
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

Le mode `directory` montre le chemin de votre dossier actuel, tronqué aux 3 dossiers parents. Votre répertoire sera également tronqué à la racine du repo git dans lequel vous vous trouvez actuellement.

Quand vous utilisez le style pwd de fish, au lieu de cacher le chemin qui est tronqué, vous verrez un nom raccourci de chaque dossier basé sur le nombre établi pour l'option.

Par exemple, donné `~/Dev/Nix/nixpkgs/pkgs` où `nixpkgs` est la racine du repo, et l'option définie à `1`. Vous verrez maintenant `~/D/N/nixpkgs/pkgs`, alors que vous auriez vu `nixpkgs/pkgs` avant.

### Options

| Option              | Défaut                                                                                                      | Description                                                                                                               |
| ------------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                                                                         | Le nombre de dossiers parents selon lesquels le répertoire courant doit être tronqué.                                     |
| `truncate_to_repo`  | `true`                                                                                                      | Si oui ou non tronquer à la racine du repo git dans lequel vous vous trouvez.                                             |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "`                                                          | Format du module.                                                                                                         |
| `style`             | `"bold cyan"`                                                                                               | Le style du module.                                                                                                       |
| `disabled`          | `false`                                                                                                     | Désactive le module `directory`.                                                                                          |
| `read_only`         | `"🔒"`                                                                                                       | Le symbole indiquant que le répertoire courant est en lecture seule.                                                      |
| `read_only_style`   | `"red"`                                                                                                     | Le style du symbole en lecture seule.                                                                                     |
| `truncation_symbol` | `""`                                                                                                        | Le symbole en préfixe aux chemins tronqués. eg: "…/"                                                                      |
| `repo_root_style`   | `None`                                                                                                      | Le style pour la racine du dépôt Git. La valeur par défaut est équivalent à `style`.                                      |
| `repo_root_format`  | `"[$before_root_path]($style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) "` | Le format d’un dépôt Git quand `repo_root_style` est défini.                                                              |
| `home_symbol`       | `"~"`                                                                                                       | Le symbole indiquant le répertoire personnel.                                                                             |
| `use_os_path_sep`   | `true`                                                                                                      | Utiliser le séparateur de chemin du système d’exploitation au lieu de toujours utiliser `/` (par ex. `\` sous Windows) |

<details>
<summary>Ce module possède quelques options de configuration avancées qui contrôlent l'affichage du répertoire.</summary>

| Options avancées            | Défaut | Description                                                                                                                                                                              |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | Table de substitutions à faire au chemin.                                                                                                                                                |
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

*: Cette variable peut uniquement être utilisée dans une chaine de style

<details>
<summary>Les dépôts Git peuvent avoir des variables additionnelles.</summary>

Considérons le chemin `/path/to/home/git_repo/src/lib`

| Variable           | Exemple               | Description                             |
| ------------------ | --------------------- | --------------------------------------- |
| before_root_path | `"/path/to/home/"`    | Le chemin avant le dossier racine Git   |
| repo_root          | `"git_repo"`          | Le nom du dossier racine Git            |
| path               | `"/src/lib"`          | Le reste du chemin                      |
| style              | `"black bold dimmed"` | Reflète la valeur de l'option `style`   |
| repo_root_style  | `"underline white"`   | Style pour le nom du dossier racine Git |

</details>

### Exemple

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Contexte Docker

Le module `docker_context` affiche le [context Docker](https://docs.docker.com/engine/context/working-with-contexts/) actif, si sa valeur est différente de `default` ou si les variables d’environnement `DOCKER_MACHINE_NAME`, `DOCKER_HOST` ou `DOCKER_CONTEXT` sont définies (puisqu’elles sont utilisées pour changer le contexte utilisé).

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

*: Cette variable peut uniquement être utilisée dans une chaine de style

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
- `*.csproj`
- `*.fsproj`
- `*.xproj`

Vous aurez également besoin du SDK .NET Core pour pouvoir l'utiliser correctement.

En interne, ce module utilise son propre mécanisme de détection de version. Généralement, il est deux fois plus rapide que d'exécuter `dotnet --version`, mais il peut afficher une version incorrecte si votre projet .NET a une arborescence inhabituelle. Si la précision est plus importante que la vitesse, vous pouvez désactiver le mécanisme en définissant `heuristic = false` dans les options du module.

Le module affichera aussi le Moniker de Framework Cible (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>) quand il y a un fichier `.csproj` dans le dossier courant.

### Options

| Option              | Défault                                                                                                 | Description                                                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                                                                             | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | Le symbole utilisé avant d'afficher la version de dotnet.                                  |
| `heuristic`         | `true`                                                                                                  | Utilisez la détection de versions plus rapide pour garder starship instantané.             |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                    | Quels dossiers devraient activer ce module.                                                |
| `style`             | `"bold blue"`                                                                                           | Le style du module.                                                                        |
| `disabled`          | `false`                                                                                                 | Désactive le module `dotnet`.                                                              |

### Variables

| Variable  | Exemple          | Description                                    |
| --------- | ---------------- | ---------------------------------------------- |
| version   | `v3.1.201`       | La version du sdk `dotnet`                     |
| tfm       | `netstandard2.0` | Le Moniker de Framework Cible du projet actuel |
| symbol    |                  | Reflète la valeur de l'option `symbol`         |
| style\* |                  | Reflète la valeur de l'option `style`          |

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

Le module `elixir` montre la version actuellement installée de [Elixir](https://elixir-lang.org/) et [Erlang/OTP](https://erlang.org/doc/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `mix.exs`.

### Options

| Option              | Défaut                                                      | Description                                                                                |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | Format du module elixir.                                                                   |
| `version_format`    | `"v${raw}"`                                                 | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | Le symbole utilisé avant d'afficher la version d'Elixir/Erlang.                            |
| `detect_extensions` | `[]`                                                        | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["mix.exs"]`                                               | Les fichiers qui activent ce module.                                                       |
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

*: Cette variable peut uniquement être utilisée dans une chaine de style

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
- Le dossier courant contient des fichiers `*.elm`

### Options

| Option              | Défaut                                             | Description                                                                                |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`               | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                        | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | Une chaîne de format représentant le symbole d'Elm.                                        |
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

Le module `env_var` affiche la valeur actuelle de la variable d’environnement choisie. Le module est affiché si l'une de ces conditions est remplie :

- L'option `variable` correspond à une variable d'environnement existante
- L'option `variable` n'est pas définie, mais l'option `default` l'est

::: tip

Plusieurs variables d’environnement peuvent être affichées en utilisant un `.`. (voir exemple). Si l’option de configuration `variable` n’est pas définie, le module affichera la valeur de la variable dont le nom est le texte après le caractère `.`.

Exemple : la configuration suivante va afficher la valeur de la variable d’environnement UTILISATEUR

```toml
# ~/.config/starship.toml

[env_var.UTILISATEUR]
default = "utilisateur inconnu"
```

:::

### Options

| Option     | Défaut                         | Description                                                                         |
| ---------- | ------------------------------ | ----------------------------------------------------------------------------------- |
| `symbol`   | `""`                           | Le symbole utilisé avant d'afficher la valeur de la variable.                       |
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

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

Afficher plusieurs variables d’environnement :

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = "SHELL"
default = "shell inconnu"
[env_var.UTILISATEUR]
default = "utilisateur inconnu"
```

## Erlang

Le module `erlang` montre la version actuellement installée de [Erlang/OTP](https://erlang.org/doc/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le dossier courant contient un fichier `rebar.config`.
- Le dossier courant contient un fichier `erlang.mk`.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | Le symbole utilisé avant d'afficher la version d'erlang.                                   |
| `style`             | `"bold green"`                       | Le style du module.                                                                        |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `disabled`          | `false`                              | Désactive le module `erlang`.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | La version d'`erlang`                  |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Remplissage

Le module `fill` remplit l’espace restant sur la ligne avec un symbole. Si plusieurs modules `fill` sont présents sur une ligne, ils divisent de manière égale l’espace entre eux. C’est utile pour aligner d’autres modules.

### Options

| Option     | Défaut         | Description                               |
| ---------- | -------------- | ----------------------------------------- |
| `symbol`   | `"."`          | Le symbole utilisé pour remplir la ligne. |
| `style`    | `"bold black"` | Le style du module.                       |
| `disabled` | `false`        | Désactive le module `fill`                |

### Exemple

```toml
# ~/.config/starship.toml
format = "AA $fill BB $fill CC"

[fill]
symbol = "-"
style = "bold green"
```

Produit une invite qui ressemble à :

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Google Cloud (`gcloud`)

Le module `gcloud` affiche la configuration actuelle pour [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. Ceci est basé sur les fichiers `~/.config/gcloud/active_config` et `~/.config/gcloud/configurations/config_{CONFIG NAME}` et la variable d'environnement `CLOUDSDK_CONFIG`.

### Options

| Option            | Défaut                                                     | Description                                                 |
| ----------------- | ---------------------------------------------------------- | ----------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Format du module.                                           |
| `symbol`          | `"☁️  "`                                                   | Le symbole affiché avant le profil GCP actuel.              |
| `region_aliases`  |                                                            | Table des alias de région à afficher en plus du nom du GCP. |
| `project_aliases` |                                                            | Table des alias de projet à afficher en plus du nom du GCP. |
| `style`           | `"bold blue"`                                              | Le style du module.                                         |
| `disabled`        | `false`                                                    | Désactive le module `gcloud`.                               |

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

#### Afficher le compte et le projet aliasée

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
nom-de-projet-long = "ndpl"
```

## Branche Git

Le module `git_branch` affiche la branche active du dépôt dans votre répertoire courant.

### Options

| Option               | Défaut                                            | Description                                                                                                                     |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Affiche le nom de la branche suivie distante, même si elle est égale au nom de la branche locale.                               |
| `format`             | `"on [$symbol$branch(:$remote_branch)]($style) "` | Format du module. Utilisez `"$branch"` pour vous référer au nom de la branche courante.                                         |
| `symbol`             | `" "`                                            | Une chaîne de format représentant le symbole de la branche git.                                                                 |
| `style`              | `"bold purple"`                                   | Le style du module.                                                                                                             |
| `truncation_length`  | `2^63 - 1`                                        | Tronque une branche git à `N` graphèmes.                                                                                        |
| `truncation_symbol`  | `"…"`                                             | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué. Vous pouvez utiliser `""` pour ne pas afficher de symbole. |
| `only_attached`      | `false`                                           | Ne montrer le nom de la branche que si elle n'est pas dans un état `HEAD` détachée.                                             |
| `ignore_branches`    | `[]`                                              | Une liste de noms à ne pas afficher. Utile pour "master" ou "main".                                                             |
| `disabled`           | `false`                                           | Désactive le module `git_branch`.                                                                                               |

### Variables

| Variable      | Exemple  | Description                                                                                                          |
| ------------- | -------- | -------------------------------------------------------------------------------------------------------------------- |
| branch        | `master` | Le nom de la branche actuelle, par défaut à `HEAD` s'il n'y a pas de branche actuelle (par exemple `HEAD` détachée). |
| remote_name   | `origin` | Le nom du dépôt distant.                                                                                             |
| remote_branch | `master` | Le nom de la branche suivie sur `remote_name`.                                                                       |
| symbol        |          | Reflète la valeur de l'option `symbol`                                                                               |
| style\*     |          | Reflète la valeur de l'option `style`                                                                                |

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

Le module `git_commit` affiche le hash du commit actuel ainsi que le tag (le cas échéant) du dépôt dans votre répertoire courant.

### Options

| Option               | Défaut                             | Description                                                                          |
| -------------------- | ---------------------------------- | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                                | La longueur du hash affiché du commit git.                                           |
| `format`             | `"[\\($hash$tag\\)]($style) "` | Format du module.                                                                    |
| `style`              | `"bold green"`                     | Le style du module.                                                                  |
| `only_detached`      | `true`                             | Ne montrer le hash du commit qu'en mode `HEAD` détachée.                             |
| `tag_disabled`       | `true`                             | Désactive l'affichage des informations du tag dans le module `git_commit`.           |
| `tag_max_candidates` | `0`                                | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `" 🏷 "`                            | Symbole préfixant les informations affichées concernant le tag                       |
| `disabled`           | `false`                            | Désactive le module `git_commit`.                                                    |

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

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Métriques Git

Le module `git_metrics` affiche le nombre de lignes ajoutées et supprimées dans le dépôt Git courant.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option               | Défaut                                                       | Description                                           |
| -------------------- | ------------------------------------------------------------ | ----------------------------------------------------- |
| `added_style`        | `"bold green"`                                               | Le style pour le compte des ajouts.                   |
| `deleted_style`      | `"bold green"`                                               | Le style pour le compte des suppressions.             |
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
added_style = "bold blue"
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Statut Git

Le module `git_status` affiche des symboles représentant l'état du dépôt dans votre répertoire courant.

::: tip

Le module Statut Git est très lent dans les dossiers Windows (par exemple sous `/mnt/c/`) dans un environnement WSL. Vous pouvez désactiver le module ou utiliser l’option `windows_starship` pour utiliser un exécutable Starship natif pour calculer le `git_status` pour ces chemins.

:::

### Options

| Option              | Défaut                                          | Description                                                                                                                     |
| ------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | Le format par défaut du module `git_status`                                                                                     |
| `conflicted`        | `"="`                                           | Cette branche a des conflits de fusion.                                                                                         |
| `ahead`             | `"⇡"`                                           | Le format de `ahead`                                                                                                            |
| `behind`            | `"⇣"`                                           | Le format de `behind`                                                                                                           |
| `diverged`          | `"⇕"`                                           | Le format de `diverged`                                                                                                         |
| `up_to_date`        | `""`                                            | The format de `up_to_date`                                                                                                      |
| `untracked`         | `"?"`                                           | Le format de `untracked`                                                                                                        |
| `stashed`           | `"$"`                                           | Le format de `stashed`                                                                                                          |
| `modified`          | `"!"`                                           | Le format de `modified`                                                                                                         |
| `staged`            | `"+"`                                           | Le format de `staged`                                                                                                           |
| `renamed`           | `"»"`                                           | Le format de `renamed`                                                                                                          |
| `deleted`           | `"✘"`                                           | Le format de `deleted`                                                                                                          |
| `style`             | `"bold green"`                                  | Le style du module.                                                                                                             |
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

Afficher le nombre de commits en avance/en retard par rapport à la branche suivie

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

Utiliser un exécutable Starship Windows pour les chemins Windows dans WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```

## Go

Par défaut, le module `go` affiche la version de [Go](https://golang.org/) installée. Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le dossier courant contient un fichier `go.mod`
- Le dossier courant contient un fichier `go.sum`
- Le répertoire courant contient un fichier `go.work`
- Le dossier courant contient un fichier `glide.yaml`
- Le répertoire courant contient un fichier `Gopkg.yml`
- Le répertoire courant contient un fichier `Gopkg.lock`
- Le répertoire courant contient un fichier `.go-version`
- Le répertoire courant contient un répertoire `Godeps`
- Le répertoire actuel contient un fichier avec l'extension `.go`

### Options

| Option              | Défaut                                                                                    | Description                                                                                |
| ------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                                      | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                                                               | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                                    | Une chaîne de caractères représentant le symbole de Go.                                    |
| `detect_extensions` | `["go"]`                                                                                  | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["go.mod", "go.sum", "go.work", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `["Godeps"]`                                                                              | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold cyan"`                                                                             | Le style du module.                                                                        |
| `disabled`          | `false`                                                                                   | Désactive le module `golang`.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | La version de `go`                     |
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

Le module `haskell` affiche la version de GHC et/ou l’instantané Stack sélectionnée.

Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le dossier courant contient un fichier `stack.yaml`
- Le dossier courant contient un fichier `.hs`, `.cabal` ou `.hs-boot`

### Options

| Option              | Défaut                               | Description                                             |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                       |
| `symbol`            | `"λ "`                               | Une chaîne de format représentant le symbole de Haskell |
| `detect_extensions` | `["hs", "cabal", "hs-boot"]`         | Quelles extensions devraient activer ce module.         |
| `detect_files`      | `["stack.yaml", "cabal.project"]`    | Les fichiers qui activent ce module.                    |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                    |
| `style`             | `"bold purple"`                      | Le style du module.                                     |
| `disabled`          | `false`                              | Désactive le module `haskell`.                          |

### Variables

| Variable       | Exemple     | Description                                                                                  |
| -------------- | ----------- | -------------------------------------------------------------------------------------------- |
| version        |             | `ghc_version` ou `snapshot` en fonction de si le dossier courant est un project Stack ou pas |
| snapshot       | `lts-18.12` | L’instantané de Stack sélectionné                                                            |
| ghc\_version | `9.2.1`     | Version de GHC installée                                                                     |
| symbol         |             | Reflète la valeur de l'option `symbol`                                                       |
| style\*      |             | Reflète la valeur de l'option `style`                                                        |

*: Cette variable peut uniquement être utilisée dans une chaine de style

## Helm

Le module `helm` montre la version actuellement installée de [Helm](https://helm.sh/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le dossier courant contient un fichier `helmfile.yaml`
- Le répertoire courant contient un fichier `Chart.yml`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"⎈ "`                               | Une chaîne de format représentant le symbole de Helm.                                      |
| `style`             | `"bold white"`                       | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `helm`.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | La version de `helm`                   |
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

Le module `hostname` affiche le nom d'hôte du système.

### Options

| Option       | Défaut                                 | Description                                                                                                                                             |
| ------------ | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`   | `true`                                 | Afficher uniquement le nom d'hôte lorsque vous êtes connecté à une session SSH.                                                                         |
| `ssh_symbol` | `"🌐 "`                                 | A format string representing the symbol when connected to SSH session.                                                                                  |
| `trim_at`    | `"."`                                  | Chaîne à laquelle le nom d'hôte est coupé, après la première correspondance. `"."` s'arrêtera après le premier point. `""` désactivera toute troncature |
| `format`     | `"[$ssh_symbol$hostname]($style) in "` | Format du module.                                                                                                                                       |
| `style`      | `"bold dimmed green"`                  | Le style du module.                                                                                                                                     |
| `disabled`   | `false`                                | Désactive le module `hostname`.                                                                                                                         |

### Variables

| Variable   | Exemple    | Description                                           |
| ---------- | ---------- | ----------------------------------------------------- |
| nom d'hôte | `computer` | Le nom d’hôte de l’ordinateur                         |
| style\*  |            | Reflète la valeur de l'option `style`                 |
| ssh_symbol | `"🌏 "`     | The symbol to represent when connected to SSH session |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = "[$ssh_symbol](bold blue) on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

Le module `java` affiche la version actuellement installée de [Java](https://www.oracle.com/java/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire actuel contient un fichier `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, ou `build.boot`
- Le répertoire actuel contient un fichier avec l'extension `.java`, `.class`, `. gradle`, `.jar`, `.clj`, ou `.cljc`

### Options

| Option              | Défaut                                                                                                    | Description                                                                                |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                                                                               | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                      | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"☕ "`                                                                                                    | Une chaîne de caractères représentant le symbole de Java                                   |
| `style`             | `"red dimmed"`                                                                                            | Le style du module.                                                                        |
| `disabled`          | `false`                                                                                                   | Désactive le module `java`.                                                                |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| version   | `v14`   | La version de `java`                   |
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

Le module `jobs` affiche le nombre de tâches en cours d'exécution. Le module ne sera affiché que s'il y a des tâches de fond. Le module affichera le nombre de tâches en cours d’exécution s’il y en a au moins 2, ou plus que la valeur `threshold`, si elle existe. Le monde affichera un symbole s’il y a au moins 1 tâche, ou plus que la valeur de `symbol_threshold`, si elle existe. Vous pouvez définir les deux valeurs à 0 pour _toujours_ montrer le symbole et le nombre de tâches, même s’il n’y a aucune tâche en cours d’exécution.

Le fonctionnement par défaut est:

- 0 tâche -> Rien n’est affiché.
- 1 tâche -> `symbol` est affiché.
- 2 taĉhes ou plus -> `symbol` + `number` sont affichés.

::: warning

Ce module n'est pas pris en charge sur tcsh et nu.

:::

::: warning

L’option `threshold` est dépréciée, mais si vous voulez l’utiliser, le module affichera le nombre de tâches en cours s’il y a plus d'une tâche, ou plus que la valeur de `threshold`, si elle existe. Si `threshold` est définie à 0, alors le module s'affichera également lorsqu'il n'y a pas de tâches de fond en cours.

:::

### Options

| Option             | Défaut                        | Description                                                               |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------- |
| `threshold`*       | `1`                           | Afficher le nombre de jobs si dépassé.                                    |
| `symbol_threshold` | `1`                           | Affiche `symbol` si le nombre de tâches vaut au moins `symbol_threshold`. |
| `number_threshold` | `2`                           | Affiche le nombre de tâches s’il y en a au moins `number_threshold`.      |
| `format`           | `"[$symbol$number]($style) "` | Format du module.                                                         |
| `symbol`           | `"✦"`                         | La chaine utilisée pour représenter la variable `symbole`.                |
| `style`            | `"bold blue"`                 | Le style du module.                                                       |
| `disabled`         | `false`                       | Désactive le module `jobs`.                                               |

*: Cette option is dépréciée, utilisez les options `number_threshold` et `symbol_threshold` à la place.

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | Le nombre de tâches                    |
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

Le module `julia` affiche la version actuellement installée de [Julia](https://julialang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `Project.toml`
- Le répertoire courant contient un fichier `Manifest.toml`
- Le répertoire actuel contient un fichier avec l'extension `.jl`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"ஃ "`                               | Une chaîne de caractères représentant le symbole de Julia.                                 |
| `style`             | `"bold purple"`                      | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `Julia`.                                                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.4.0` | La version de `julia`                  |
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

Le module `kotlin` affiche la version actuellement installée de [Kotlin](https://kotlinlang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `.kt` ou `.kts`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["kt", "kts"]`                      | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"🅺 "`                               | Une chaîne de caractères représentant le symbole de Kotlin.                                |
| `style`             | `"bold blue"`                        | Le style du module.                                                                        |
| `kotlin_binary`     | `"kotlin"`                           | Configure le binaire kotlin que Starship exécute lors de l'obtention de la version.        |
| `disabled`          | `false`                              | Désactive le module `kotlin`.                                                              |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.4.21` | La version de `kotlin`                 |
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

Afficher le nom du [contexte Kubernetes](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) courant, et, si défini, l’espace de nom, l’utilisateur, et le cluster depuis le fichier kubeconfig. L'espace de noms doit être défini dans le fichier kubeconfig, ce qui peut être fait via `kubectl config set-context starship-cluster --namespace astronaut`. De même, l'utilisateur et l'instance peuvent être définies avec `kubectl config set-context starship-context --user starship-user` et `kubectl config set-context starship-context --cluster starship-cluster`. Si la variable d'environnement `$KUBECONFIG` est définie, le module l'utilisera sinon il utilisera le fichier `~/.kube/config`.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

When the module is enabled it will always be active, unless any of `detect_extensions`, `detect_files` or `detect_folders` have been st in which case the module will only be active in directories that match those conditions.

:::

### Options

| Option              | Défaut                                               | Description                                                            |
| ------------------- | ---------------------------------------------------- | ---------------------------------------------------------------------- |
| `symbol`            | `"☸ "`                                               | Une chaîne de format représentant le symbole affiché avant le Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | Format du module.                                                      |
| `style`             | `"cyan bold"`                                        | Le style du module.                                                    |
| `context_aliases`   |                                                      | Tableau des alias de contexte à afficher.                              |
| `user_aliases`      |                                                      | Table of user aliases to display.                                      |
| `detect_extensions` | `[]`                                                 | Quelles extensions devraient activer ce module.                        |
| `detect_files`      | `[]`                                                 | Les fichiers qui activent ce module.                                   |
| `detect_folders`    | `[]`                                                 | Quels dossiers devraient activer ce module.                            |
| `disabled`          | `true`                                               | Désactiver le module `kubernetes`.                                     |

### Variables

| Variable  | Exemple              | Description                                      |
| --------- | -------------------- | ------------------------------------------------ |
| context   | `starship-context`   | Le nom du contexte de kubernetes actuel          |
| namespace | `starship-namespace` | Si défini, l'espace de noms actuel de kubernetes |
| user      | `starship-user`      | Si défini, l’utilisateur de kubernetes actuel    |
| cluster   | `starship-cluster`   | Si défini, le cluster de kubernetes actuel       |
| symbol    |                      | Reflète la valeur de l'option `symbol`           |
| style\* |                      | Reflète la valeur de l'option `style`            |

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

#### Filtrage par regex

Additional to simple aliasing, `context_aliases` and `user_aliases` also supports extended matching and renaming using regular expressions.

L’expression rationnelle doit correspondre au contexte kube entier, et des groupes de capture peuvent être référencés en utilisant `$name` et `$N` dans la valeur de remplacement. Ceci est expliqué plus en détails dans la documentation de [la crate regex](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace).

Les noms de cluster long ou générés automatiquement peuvent être identifiés et raccourcis en utilisant des expressions rationnelles:

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

## Saut de ligne

Le module `line_break` sépare l'invite en deux lignes.

### Options

| Option     | Défaut  | Description                                                             |
| ---------- | ------- | ----------------------------------------------------------------------- |
| `disabled` | `false` | Désactive le module `line_break` , faisant de l'invite une seule ligne. |

### Exemple

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## IP locale

Le module `localip` affiche l’adresse IPv4 de l’interface réseau principale.

### Options

| Option     | Défaut                    | Description                                                       |
| ---------- | ------------------------- | ----------------------------------------------------------------- |
| `ssh_only` | `true`                    | Afficher uniquenement l’adresse IP si connecté à une session SSH. |
| `format`   | `"[$localipv4]($style) "` | Format du module.                                                 |
| `style`    | `"bold yellow"`           | Le style du module.                                               |
| `disabled` | `true`                    | Désactive le module `localip`.                                    |

### Variables

| Variable  | Exemple      | Description                           |
| --------- | ------------ | ------------------------------------- |
| localipv4 | 192.168.1.13 | Contient l’adresse IPv4 principale    |
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

Le module `lua` affiche la version actuellement installée de [Lua](http://www.lua.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `.lua-version`
- Le répertoire courant contient un répertoire `lua`
- Le répertoire actuel contient un fichier avec l'extension `.lua`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌙 "`                               | Une chaîne de caractères représentant le symbole de Lua.                                   |
| `detect_extensions` | `["lua"]`                            | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[".lua-version"]`                   | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `["lua"]`                            | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold blue"`                        | Le style du module.                                                                        |
| `lua_binary`        | `"lua"`                              | Configure le binaire lua que Starship exécute lors de l'obtention de la version.           |
| `disabled`          | `false`                              | Désactive le module `lua`.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | La version de `lua`                    |
| symbol    |          | Reflète la valeur de l'option `symbol` |
| style\* |          | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Utilisation mémoire

Le module `memory_usage` affiche la mémoire système actuelle et l'utilisation de swap.

Par défaut, l'utilisation du swap est affichée si le swap total du système n'est pas nul.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option      | Défaut                                          | Description                                                                    |
| ----------- | ----------------------------------------------- | ------------------------------------------------------------------------------ |
| `threshold` | `75`                                            | Masquer l'utilisation de la mémoire à moins qu'elle ne dépasse ce pourcentage. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | Format du module.                                                              |
| `symbol`    | `"🐏"`                                           | Le symbole utilisé avant d'afficher l'utilisation de la mémoire.               |
| `style`     | `"bold dimmed white"`                           | Le style du module.                                                            |
| `disabled`  | `true`                                          | Désactiver le module `memory_usage`.                                           |

### Variables

| Variable         | Exemple       | Description                                                                     |
| ---------------- | ------------- | ------------------------------------------------------------------------------- |
| ram              | `31GiB/65GiB` | La mémoire système utilisée/totale .                                            |
| ram_pct          | `48%`         | Le pourcentage de la mémoire du système actuel.                                 |
| swap\*\*     | `1GiB/4GiB`   | La taille de la mémoire swap du fichier de mémoire swap du système courant.     |
| swap_pct\*\* | `77%`         | Le poucentage de la mémoire swap du fichier de mémoire swap du système courant. |
| symbol           | `🐏`           | Reflète la valeur de l'option `symbol`                                          |
| style\*        |               | Reflète la valeur de l'option `style`                                           |

*: Cette variable peut uniquement être utilisée dans une chaine de style *\*: Les informations sur le fichier SWAP est uniquement affichée si détectée sur le système courant

### Exemple

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Branche Mercurial

Le module `hg_branch` affiche la branche active du dépôt dans votre répertoire courant.

### Options

| Option              | Défaut                           | Description                                                                                                |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | Le symbole utilisé avant le marque-page hg ou le nom de la branche du dépôt dans votre répertoire courant. |
| `style`             | `"bold purple"`                  | Le style du module.                                                                                        |
| `format`            | `"on [$symbol$branch]($style) "` | Format du module.                                                                                          |
| `truncation_length` | `2^63 - 1`                       | Tronque le nom de la branche hg à `N` graphèmes                                                            |
| `truncation_symbol` | `"…"`                            | Le symbole utilisé pour indiquer qu'un nom de branche a été tronqué.                                       |
| `disabled`          | `true`                           | Désactive le module `hg_branch`.                                                                           |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| branch    | `master` | La branche mercuriale active           |
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

Le module `nim` affiche la version actuellement installée de [Nim](https://nim-lang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le dossier courant contient un fichier `nim.cfg`
- Le répertoire actuel contient un fichier avec l'extension `.nim`
- Le répertoire actuel contient un fichier avec l'extension `.nims`
- Le répertoire actuel contient un fichier avec l'extension `.nimble`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module                                                                           |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | Le symbole utilisé avant d'afficher la version de Nim.                                     |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["nim.cfg"]`                        | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold yellow"`                      | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `nim`.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.2.0` | La version de `nim`                    |
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

Le module `nix_shell` affiche la version de l’environnement [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html). Ce module s’affichera quand vous serez à l’intérieur d’un environnement nix-shell.

### Options

| Option       | Défaut                                         | Description                                                |
| ------------ | ---------------------------------------------- | ---------------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | Format du module.                                          |
| `symbol`     | `"❄️ "`                                        | Une chaîne de format représentant le symbole de nix-shell. |
| `style`      | `"bold blue"`                                  | Le style du module.                                        |
| `impure_msg` | `"impure"`                                     | Une chaîne de format affichée lorsque le shell est impur.  |
| `pure_msg`   | `"pure"`                                       | Une chaîne de format affichée lorsque le shell est pur.    |
| `disabled`   | `false`                                        | Désactive le module `nix_shell`.                           |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| state     | `pure`  | L’état du nix-shell                    |
| name      | `lorri` | Le nom du nix-shell                    |
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

Le module `nodejs` affiche la version actuellement installée de [Node.js](https://nodejs.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le dossier courant contient un fichier `package.json`
- Le répertoire courant contient un fichier `.node-version`
- Le répertoire courant contient un fichier `.nvmrc`
- Le répertoire courant contient un répertoire `node_modules`
- Le répertoire actuel contient un fichier avec l'extension `.js`, `.mjs` ou `.cjs`
- Le dossier courant contient un fichier avec l’extension `.ts`, `.mts` ou `.cts`

### Options

| Option              | Défaut                                     | Description                                                                                                 |
| ------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`       | Format du module.                                                                                           |
| `version_format`    | `"v${raw}"`                                | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch`                  |
| `symbol`            | `" "`                                     | Une chaîne de caractères représentant le symbole de Node.js.                                                |
| `detect_extensions` | `["js", "mjs", "cjs", "ts", "mts", "cts"]` | Quelles extensions devraient activer ce module.                                                             |
| `detect_files`      | `["package.json", ".node-version"]`        | Les fichiers qui activent ce module.                                                                        |
| `detect_folders`    | `["node_modules"]`                         | Les dossiers qui activent ce module.                                                                        |
| `style`             | `"bold green"`                             | Le style du module.                                                                                         |
| `disabled`          | `false`                                    | Désactive le module `nodejs`.                                                                               |
| `not_capable_style` | `bold red`                                 | Le style du module quand une propriété engines dans le package.json ne correspond pas à la version Node.js. |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | La version de `node`                   |
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

Le module `ocaml` affiche la version actuellement installée de [OCaml](https://ocaml.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier avec l'extension `.opam` ou le répertoire `_opam`
- Le répertoire courant contient un répertoire `esy.lock`
- Le répertoire courant contient un fichier `dune` ou `dune-project`
- Le répertoire courant contient un fichier `jbuild` ou `jbuild-ignore`
- Le répertoire courant contient un fichier `.merlin`
- Le répertoire actuel contient un fichier avec l'extension `.ml`, `.mli`, `.re` ou `.rei`

### Options

| Option                    | Défaut                                                                     | Description                                                                                |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | La chaîne de format pour le module.                                                        |
| `version_format`          | `"v${raw}"`                                                                | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | Le symbole utilisé avant d'afficher la version de OCaml.                                   |
| `global_switch_indicator` | `""`                                                                       | La chaîne de caractères utilisée pour représenter le commutateur OPAM global.              |
| `local_switch_indicator`  | `"*"`                                                                      | La chaîne de caractères utilisée pour représenter le commutateur OPAM local.               |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Quelles extensions devraient activer ce module.                                            |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Les fichiers qui activent ce module.                                                       |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Les dossiers qui activent ce module.                                                       |
| `style`                   | `"bold yellow"`                                                            | Le style du module.                                                                        |
| `disabled`                | `false`                                                                    | Désactive le module `ocaml`.                                                               |

### Variables

| Variable         | Exemple      | Description                                                       |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | La version de `ocaml`                                             |
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
| `symbol`   | `"☁️ "`                                             | Le symbole utilisé avant d'afficher le cloud OpenStack actuel. |
| `style`    | `"bold yellow"`                                     | Le style du module.                                            |
| `disabled` | `false`                                             | Désactive le module `openstack`.                               |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | Le cloud OpenStack courant             |
| project   | `dev`   | Le projet OpenStack courant            |
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

## Version du package

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – La version du paquet `npm` est extraite du `package.json` présent dans le répertoire courant
- [**Cargo**](https://doc.rust-lang.org/cargo/) – La version du paquet `cargo` est extraite du `Cargo.toml` présent dans le répertoire courant
- [**Nimble**](https://github.com/nim-lang/nimble) - La version du paquet `nimble` est extraite du fichier `*.nimble` dans le répertoire courant avec la commande `nimble dump`
- [**Poetry**](https://python-poetry.org/) – La version du paquet `poetry` est extraite du `pyproject.toml` présent dans le répertoire courant
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – La version du paquet `composer` est extraite du `composer.json` présent dans le répertoire courant
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - La version du paquet `sbt` est extraite du `build.sbt` présent dans le dossier courant
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) – La version du paquet `dart` est extrait du `pubspec.yaml` présent dans le répertoire courant

> ⚠️ La version montrée est celle du paquet dont le code source est dans votre dossier courant, pas votre gestionnaire de paquet.

### Options

| Option            | Défaut                            | Description                                                                                |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`          | `"is [$symbol$version]($style) "` | Format du module.                                                                          |
| `symbol`          | `"📦 "`                            | Le symbole utilisé avant d'afficher la version du paquet.                                  |
| `version_format`  | `"v${raw}"`                       | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `style`           | `"bold 208"`                      | Le style du module.                                                                        |
| `display_private` | `false`                           | Active l’affichage des versions des paquets marqués comme privés.                          |
| `disabled`        | `false`                           | Désactive le module `package`.                                                             |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.0.0` | La version de votre package            |
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

Le module `perl` affiche la version actuellement installée de [Perl](https://www.perl.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `Makefile.PL` ou `Build.PL`
- Le répertoire courant contient un fichier `cpanfile` ou `cpanfile.snapshot`
- Le répertoire courant contient un fichier `META.json` ou `META.yml`
- Le répertoire courant contient un fichier `.perl-version`
- Le répertoire courant contient un fichier `.pl`, `.pm` ou `.pod`

### Options

| Option              | Défaut                                                                                                   | Description                                                                                |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | La chaîne de format pour le module.                                                        |
| `version_format`    | `"v${raw}"`                                                                                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | Le symbole utilisé avant d'afficher la version de Perl                                     |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                                                                                     | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold 149"`                                                                                             | Le style du module.                                                                        |
| `disabled`          | `false`                                                                                                  | Désactive le module `perl`.                                                                |

### Variables

| Variable  | Exemple   | Description                            |
| --------- | --------- | -------------------------------------- |
| version   | `v5.26.1` | La version de `perl`                   |
| symbol    |           | Reflète la valeur de l'option `symbol` |
| style\* |           | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

Le module `php` affiche la version actuellement installée de [PHP](https://www.php.net/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le dossier courant contient un fichier `composer.json`
- Le répertoire courant contient un fichier `.php-version`
- Le répertoire courant contient un fichier avec l'extension `.php`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | Le symbole utilisé avant d'afficher la version de PHP.                                     |
| `detect_extensions` | `["php"]`                            | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["composer.json", ".php-version"]`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"147 bold"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `php`.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v7.3.8` | La version de `php`                    |
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

Par défaut, la version de Pulumi n'est pas affichée, car cela prendre un ordre de magnitude plus de temps à charger que la plupart des plugins (~70ms). Si vous voulez quand même l’activer, [suivez l’exemple montré plus bas](#with-pulumi-version).

:::

Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le dossier courant contient soit un `Pulumi.yaml`, soit un `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Options

| Option           | Défaut                                       | Description                                                                                |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`         | `"via [$symbol($username@)$stack]($style) "` | La chaîne de format pour le module.                                                        |
| `version_format` | `"v${raw}"`                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `" "`                                       | A format string shown before the Pulumi stack.                                             |
| `style`          | `"bold 5"`                                   | Le style du module.                                                                        |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.                             |
| `disabled`       | `false`                                      | Désactive le module `pulumi`.                                                              |

### Variables

| Variable          | Exemple    | Description                            |
| ----------------- | ---------- | -------------------------------------- |
| version           | `v0.12.24` | La version de `pulumi`                 |
| stack             | `dev`      | The current Pulumi stack               |
| nom d'utilisateur | `alice`    | Le nom d’utilisateur Pulumi actuel     |
| symbol            |            | Reflète la valeur de l'option `symbol` |
| style\*         |            | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

#### Avec la version de Pulumi

```toml
# ~/.config/starship.toml

[pulumi]
format = "[🛥 ($version )$stack]($style) "
```

#### Sans la version de Pulumi

```toml
# ~/.config/starship.toml
[pulumi]
symbol = "🛥 "
format = "[$symbol$stack]($style) "
```

## PureScript

Le module `purescript` affiche la version actuellement installée de [PureScript](https://www.purescript.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `spago.dhall`
- Le répertoire actuel contient un fichier avec l'extension `.purs`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | Le symbole utilisé avant d'afficher la version de PureScript.                              |
| `detect_extensions` | `["purs"]`                           | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["spago.dhall"]`                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold white"`                       | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `purescript`.                                                          |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | La version de `purescript`             |
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

Le module `python` affiche la version actuellement installée de [Python](https://www.python.org/) ainsi que la version d'[environnement virtuel Python](https://docs.python.org/tutorial/venv.html) si il y en a un d'activé.

Si `pyenv_version_name` est défini à `true`, il affichera le nom de la version de pyenv. Sinon, il affichera le numéro de version que donne `python --version`.

Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `.python-version`
- Le répertoire courant contient un fichier `Pipfile`
- Le répertoire courant contient un fichier `__init__.py`
- Le dossier courant contient un fichier `pyproject.toml`
- Le dossier courant contient un fichier `requirements.txt`
- Le dossier courant contient un fichier `setup.py`
- Le dossier courant contient un fichier `tox.ini`
- Le répertoire actuel contient un fichier avec l'extension `.py`.
- Un environnement virtuel est actuellement activé

### Options

| Option               | Défaut                                                                                                       | Description                                                                                |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Format du module.                                                                          |
| `version_format`     | `"v${raw}"`                                                                                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`             | `"🐍 "`                                                                                                       | Une chaîne de caractères représentant le symbole de Python                                 |
| `style`              | `"yellow bold"`                                                                                              | Le style du module.                                                                        |
| `pyenv_version_name` | `false`                                                                                                      | Utiliser pyenv pour obtenir la version de Python                                           |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                            |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configures the python binaries that Starship should executes when getting the version.     |
| `detect_extensions`  | `["py"]`                                                                                                     | Quelles extensions devraient activer ce module                                             |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`     | `[]`                                                                                                         | Quels dossiers devraient activer ce module                                                 |
| `disabled`           | `false`                                                                                                      | Désactive le module `python`.                                                              |

::: tip

La variable `python_binary` accepte soit une chaine, soit une liste de chaines de caractères. Starship essayera d'exécuter chaque binaire jusqu'à obtenir un résultat. Notez que vous ne pouvez modifier que le binaire que Starship exécute pour obtenir la version de Python, mais pas les arguments utilisés.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). Si vous ne travaillez plus avec Python 2 mais ne pouvez pas le supprimer du système, changer ceci par `"python3"` cachera tous les Python version 2, voir l’exemple ci-dessous.

:::

### Variables

| Variable     | Exemple         | Description                                  |
| ------------ | --------------- | -------------------------------------------- |
| version      | `"v3.8.1"`      | La version de `python`                       |
| symbol       | `"🐍 "`          | Reflète la valeur de l'option `symbol`       |
| style        | `"yellow bold"` | Reflète la valeur de l'option `style`        |
| pyenv_prefix | `"pyenv "`      | Reflète la valeur de l'option `pyenv_prefix` |
| virtualenv   | `"venv"`        | Le nom du `virtualenv` courant               |

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
# N'utilisez que le binaire `python3` pour obtenir la version.
python_binary = "python3"
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
python_binary = ["./venv/bin/python", "python", "python3", "python2"]
```

## R

Le module `rlang` affiche la version de [R](https://www.r-project.org/) actuellement installée. Le module s’affiche si l’une de ces conditions est remplie :

- Le répertoire actuel contient un fichier avec l'extension `.R`.
- Le répertoire actuel contient un fichier avec l'extension `.Rd`.
- Le répertoire actuel contient un fichier avec l'extension `.Rmd`.
- Le répertoire actuel contient un fichier avec l'extension `.Rproj`.
- Le répertoire actuel contient un fichier avec l'extension `.Rsx`.
- Le répertoire courant contient un fichier `.Rprofile`
- Le répertoire courant contient un dossier `.Rproj.user`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"📐"`                                | Une chaîne de caractères représentant le symbole de R.                                     |
| `style`             | `"blue bold"`                        | Le style du module.                                                                        |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | Quelles extensions devraient activer ce module                                             |
| `detect_files`      | `[".Rprofile"]`                      | Quels fichiers devraient activer ce module                                                 |
| `detect_folders`    | `[".Rproj.user"]`                    | Quels dossiers devraient activer ce module                                                 |
| `disabled`          | `false`                              | Désactive le module `r`.                                                                   |

### Variables

| Variable | Exemple       | Description                            |
| -------- | ------------- | -------------------------------------- |
| version  | `v4.0.5`      | La version de `R`                      |
| symbol   |               | Reflète la valeur de l'option `symbol` |
| style    | `"blue bold"` | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[rlang]
format = "with [📐 $version](blue bold) "
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Options

| Option              | Défaut                                           | Description                                                                                |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version-$vm_version )]($style)"` | La chaîne de format pour le module.                                                        |
| `version_format`    | `"v${raw}"`                                      | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦋 "`                                           | The symbol used before displaying the version of Raku                                      |
| `detect_extensions` | `["p6", "pm6", "pod6", "raku", "rakumod"]`       | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["META6.json"]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                             | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold 149"`                                     | Le style du module.                                                                        |
| `disabled`          | `false`                                          | Disables the `raku` module.                                                                |

### Variables

| Variable   | Exemple | Description                            |
| ---------- | ------- | -------------------------------------- |
| version    | `v6.d`  | The version of `raku`                  |
| vm_version | `moar`  | The version of VM `raku` is built on   |
| symbol     |         | Reflète la valeur de l'option `symbol` |
| style\*  |         | Reflète la valeur de l'option `style`  |

### Exemple

```toml
# ~/.config/starship.toml

[raku]
format = "via [🦪 $version]($style) "
```

## Red

Par défaut, le module `red` affiche la version actuellement installée de [Red](https://www.red-lang.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le répertoire actuel contient un fichier avec l'extension `.red` ou `.reds`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | Une chaîne de caractères représentant le symbole de Red.                                   |
| `detect_extensions` | `["red"]`                            | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"red bold"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `red`.                                                                 |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | La version de `red`                    |
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

Par défaut, le module `ruby` affiche la version actuellement installée de [Ruby](https://www.ruby-lang.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le répertoire courant contient un fichier `Gemfile`
- Le répertoire courant contient un fichier `.ruby-version`
- Le répertoire courant contient un fichier `.rb`
- La variable d’environnement `RUBY_VERSION` ou `RBENV_VERSION` est définie

Starship obtient la version actuelle de Ruby en exécutant `ruby -v`.

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | Une chaîne de caractères représentant le symbole de Ruby.                                  |
| `detect_extensions` | `["rb"]`                             | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `detect_variables`  | `["RUBY_VERSION", "RBENV_VERSION"]`  | Les variables d’environnement qui activent ce module.                                      |
| `style`             | `"bold green"`                       | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `ruby`.                                                                |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | La version de `ruby`                   |
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

Par défaut, le module `rust` affiche la version actuellement installée de [Rust](https://www.rust-lang.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le répertoire courant contient un fichier `Cargo.toml`
- Le répertoire actuel contient un fichier avec l'extension `.rs`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | Une chaîne de caractères représentant le symbole de Rust                                   |
| `detect_extensions` | `["rs"]`                             | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Cargo.toml"]`                     | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold green"`                       | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `rust`.                                                                |

### Variables

| Variable  | Exemple           | Description                                  |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | La version de `rustc`                        |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | Reflète la valeur de l'option `symbol`       |
| style\* |                   | Reflète la valeur de l'option `style`        |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

Le module `scala` affiche la version actuellement installée de [Scala](https://www.scala-lang.org/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `build.sbt`, `.scalaenv` ou `.sbtenv`
- Le répertoire actuel contient un fichier avec l'extension `.scala` ou `.sbt`
- Le répertoire courant contient un répertoire nommé `.metals`

### Options

| Option              | Défaut                                   | Description                                                                                |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                              | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[".metals"]`                            | Quels dossiers devraient activer ce module.                                                |
| `symbol`            | `"🆂 "`                                   | Une chaîne de caractères représentant le symbole de Scala.                                 |
| `style`             | `"red dimmed"`                           | Le style du module.                                                                        |
| `disabled`          | `false`                                  | Désactive le module `scala`.                                                               |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `2.13.5` | La version de `scala`                  |
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

Le module `shell` affiche un indicateur en fonction du shell actuellement utilisé.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option                 | Défaut                    | Description                                                 |
| ---------------------- | ------------------------- | ----------------------------------------------------------- |
| `bash_indicator`       | `bsh`                     | Une chaîne de format utilisée pour représenter bash.        |
| `fish_indicator`       | `fsh`                     | Une chaîne de format utilisée pour représenter fish.        |
| `zsh_indicator`        | `zsh`                     | Une chaîne de format utilisée pour représenter zsh.         |
| `powershell_indicator` | `psh`                     | Une chaîne de format utilisée pour représenter powershell.  |
| `ion_indicator`        | `ion`                     | Une chaîne de format utilisée pour représenter ion.         |
| `elvish_indicator`     | `esh`                     | Une chaîne de format utilisée pour représenter elvish.      |
| `tcsh_indicator`       | `tsh`                     | Une chaîne de format utilisée pour représenter tcsh.        |
| `xonsh_indicator`      | `xsh`                     | Chaine de formatage utilisée pour représenter xonsh.        |
| `cmd_indicator`        | `cmd`                     | Chaine de formatage utilisée pour représenter cmd.          |
| `nu_indicator`         | `nu`                      | Chaine de formatage utilisée pour représenter nu.           |
| `unknown_indicator`    |                           | La valeur par défaut à afficher quand le shell est inconnu. |
| `format`               | `"[$indicator]($style) "` | Format du module.                                           |
| `style`                | `"white bold"`            | Le style du module.                                         |
| `disabled`             | `true`                    | Désactive le module `shell`.                                |

### Variables

| Variable  | Défaut | Description                                       |
| --------- | ------ | ------------------------------------------------- |
| indicator |        | Reflète la valeur de `indicator` du shell actuel. |
| style\* |        | Reflète la valeur de l'option `style`.            |

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

Le module `shlvl` montre la variable d’environnement [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) («niveau de shell») actuelle, si elle a une valeur numérique qui est égale ou dépasse le seuil défini.

### Options

| Option      | Défaut                       | Description                                                             |
| ----------- | ---------------------------- | ----------------------------------------------------------------------- |
| `threshold` | `2`                          | Seuil d’affichage.                                                      |
| `format`    | `"[$symbol$shlvl]($style) "` | Format du module.                                                       |
| `symbol`    | `"↕️  "`                     | Le symbole utilisée pour représenter le `SHLVL`.                        |
| `repeat`    | `false`                      | Fait répéter `symbol` autant de fois que la valeur actuelle de `SHLVL`. |
| `style`     | `"bold yellow"`              | Le style du module.                                                     |
| `disabled`  | `true`                       | Désactive le module `shlvl`.                                            |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | La valeur actuelle de `SHLVL`          |
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

Le module `singularity` affiche l’image [Singularity](https://sylabs.io/singularity/) courante, quand vous êtes à l’intérieur d’un conteneur et que `$SINGULARITY_NAME` est définie.

### Options

| Option     | Défaut                           | Description                                            |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Format du module.                                      |
| `symbol`   | `""`                             | Une chaîne de format affichée avant le nom de l'image. |
| `style`    | `"bold dimmed blue"`             | Le style du module.                                    |
| `disabled` | `false`                          | Désactive le module `singularity`.                     |

### Variables

| Variable  | Exemple      | Description                            |
| --------- | ------------ | -------------------------------------- |
| env       | `centos.img` | L’image Singularity actuelle           |
| symbol    |              | Reflète la valeur de l'option `symbol` |
| style\* |              | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Spack

Le module `spack` affiche l’environnement [Spack](https://spack.readthedocs.io/en/latest/) courant, si `$SPACK_ENV` est définie.

### Options

| Option              | Défaut                                 | Description                                                                                                                                                        |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `1`                                    | Le nombre de dossiers auxquels le chemin d’environnement doit être tronqué. `0` ne signifie pas de troncature. Regardez aussi le module [`directory`](#directory). |
| `symbol`            | `"🅢  "`                                | Le symbole utilisé avant le nom d'environnement.                                                                                                                   |
| `style`             | `"bold blue"`                          | Le style du module.                                                                                                                                                |
| `format`            | `"via [$symbol$environment]($style) "` | Format du module.                                                                                                                                                  |
| `disabled`          | `false`                                | Désactive le module `spack`.                                                                                                                                       |

### Variables

| Variable      | Exemple      | Description                            |
| ------------- | ------------ | -------------------------------------- |
| environnement | `astronauts` | L’environnement de spack courant       |
| symbol        |              | Reflète la valeur de l'option `symbol` |
| style\*     |              | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[spack]
format = "[$symbol$environment](dimmed blue) "
```

## Status

Le module `status` affiche le code de sortie de la commande précédente. Si $success_symbol est vide (par défaut), ce module sera affiché uniquement quand le code de sortie n’est pas `0`. Le code de statut est converti en entier signé 32 bits.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option                      | Défaut                                                                               | Description                                                           |
| --------------------------- | ------------------------------------------------------------------------------------ | --------------------------------------------------------------------- |
| `format`                    | `"[$symbol$status]($style) "`                                                        | Le format du module                                                   |
| `symbol`                    | `"✖"`                                                                                | The symbol displayed on program error                                 |
| `success_symbol`            | `""`                                                                                 | The symbol displayed on program success                               |
| `not_executable_symbol`     | `"🚫"`                                                                                | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `"🔍"`                                                                                | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `"🧱"`                                                                                | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `"⚡"`                                                                                | The symbol displayed on any signal                                    |
| `style`                     | `"bold green"`                                                                       | Le style du module.                                                   |
| `recognize_signal_code`     | `true`                                                                               | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                              | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                              | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                            | The symbol used to separate pipestatus segments                       |
| `pipestatus_format`         | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                      | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                               | Désactiver le module `status`.                                        |

### Variables

| Variable       | Exemple | Description                                                                                 |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | Le code de sortie de la dernière commande                                                   |
| hex_status     | `0x7F`  | Le code de sortie de la dernière commande en hexa                                           |
| int            | `127`   | Le code de sortie de la dernière commande                                                   |
| common_meaning | `ERROR` | Signification du code si n’est pas un signal                                                |
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

Le moduel `sudo` affiche si les identifiants sudo sont actuellement en cache. Le module sera uniquement affiché si les identifiants sont en cache.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option          | Défaut                  | Description                                                       |
| --------------- | ----------------------- | ----------------------------------------------------------------- |
| `format`        | `[as $symbol]($style)"` | Le format du module                                               |
| `symbol`        | `"🧙 "`                  | Le symbole affiché quand les identifiants sont en cache           |
| `style`         | `"bold blue"`           | Le style du module.                                               |
| `allow_windows` | `false`                 | Puisque Windows n’a pas de sudo par défaut, désactivé par défaut. |
| `disabled`      | `true`                  | Désactive le module `sudo`.                                       |

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
# Sous Windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

Par défaut, le module `swift` affiche la version actuellement installée de [Swift](https://swift.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le répertoire courant contient un fichier `Package.swift`
- Le répertoire actuel contient un fichier avec l'extension `.swift`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | Une chaîne de caractères représentant le symbole de Swift                                  |
| `detect_extensions` | `["swift"]`                          | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Package.swift"]`                  | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold 202"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactiver le module `swift`.                                                              |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.2.4` | La version de `swift`                  |
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

Le module `terraform` affiche [l’espace de travail Terraform](https://www.terraform.io/docs/language/state/workspaces.html) sélectionné et sa version.

::: tip

Par défaut, la version de Terraform n’est pas affichée, car elle est lente pour les versions actuelles de Terraform quand beaucoup de plugins sont utilisés. Si vous voulez quand même l’activer, [suivez l’exemple montré plus bas](#with-terraform-version).

:::

Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un dossier `.terraform`
- Le répertoire courant contient un fichier avec l’extension `.tf`, `.tfplan` ou `.tfstate`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol$workspace]($style) "` | La chaîne de format pour le module.                                                        |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | Une chaîne de format montrée avant l'espace de travail terraform.                          |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]`        | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[".terraform"]`                     | Les dossiers qui activent ce module.                                                       |
| `style`             | `"bold 105"`                         | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `terraform`.                                                           |

### Variables

| Variable  | Exemple    | Description                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | La version de `terraform`              |
| workspace | `default`  | L’espace de travail Terraform courant  |
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

Le module `time` affiche l'heure actuelle **localement**. La valeur de `format` est utilisée par le package [`chrono`](https://crates.io/crates/chrono) pour contrôler la façon dont l'heure est affichée. Consultez la [doc de chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) pour découvrir les options disponibles.

::: tip

Ce module est désactivé par défaut. Pour l'activer, configurez `disabled` sur `false` dans votre fichier de configuration.

:::

### Options

| Option            | Défaut                  | Description                                                                                                                                                           |
| ----------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | La chaîne de format pour le module.                                                                                                                                   |
| `use_12hr`        | `false`                 | Activer le format 12h                                                                                                                                                 |
| `time_format`     | voir plus bas           | Le [format chrono](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) utilisé pour formater l'heure.                                                     |
| `style`           | `"bold yellow"`         | Le style utilisé par le module                                                                                                                                        |
| `utc_time_offset` | `"local"`               | Définir le décalage horaire UTC à utiliser. Intervalle de -24 &lt; x &lt; 24. Accepte des nombres décimaux pour s'adapter aux décalages de 30/45 minutes. |
| `disabled`        | `true`                  | Désactiver le module `time`.                                                                                                                                          |
| `time_range`      | `"-"`                   | Définit la plage de temps pendant laquelle le module sera affiché. Les heures doivent être spécifiées au format 24 heures                                             |

Si `use_12hr` est à `true`, alors `time_format` vaut `"%r"`. Sinon, il est défini comme `"%T"`. Définir manuellement `time_format` outrepasse le paramètre `use_12hr`.

### Variables

| Variable  | Exemple    | Description                           |
| --------- | ---------- | ------------------------------------- |
| time      | `13:08:10` | L’heure actuelle.                     |
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

## Nom d'utilisateur

Le module `username` affiche le nom d'utilisateur de l'utilisateur actif. Le module est affiché si l'une de ces conditions est remplie :

- L'utilisateur courant est root/admin
- L'utilisateur courant est différent de celui connecté
- L'utilisateur est actuellement connecté à une session SSH
- La variable `show_always` a comme valeur true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Options

| Option        | Défaut                  | Description                                          |
| ------------- | ----------------------- | ---------------------------------------------------- |
| `style_root`  | `"bold green"`          | Le style utilisé quand l'utilisateur est root/admin. |
| `style_user`  | `"bold yellow"`         | Le style utilisé pour les utilisateurs non-root.     |
| `format`      | `"[$user]($style) in "` | Format du module.                                    |
| `show_always` | `false`                 | Toujours afficher le module `username`.              |
| `disabled`    | `false`                 | Désactiver le module `username`.                     |

### Variables

| Variable | Exemple      | Description                                                                                 |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `"matchai"`  | L’identifiant de l’utilisateur courant.                                                     |

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

Le module `vagrant` affiche la version actuellement installée de [Vagrant](https://www.vagrantup.com/). Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier `Vagrantfile`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | Une chaîne de caractères représentant le symbole de Vagrant.                               |
| `detect_extensions` | `[]`                                 | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["Vagrantfile"]`                    | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |
| `style`             | `"cyan bold"`                        | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `vagrant`.                                                             |

### Variables

| Variable  | Exemple          | Description                            |
| --------- | ---------------- | -------------------------------------- |
| version   | `Vagrant 2.2.10` | La version de `Vagrant`                |
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

Le module `vlang` affiche la version de [V](https://vlang.io/) installée. Par défaut le module sera activé si au moins l'une des conditions suivantes est remplie:

- Le répertoire courant contient un fichier avec l'extension `.v`
- Le répertoire courant contient un fichier `v.mod`, `vpkg.json` ou `.vpkg-lock.json`

### Options

| Option              | Défaut                                       | Description                                                                                |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`         | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                                  | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"V "`                                       | Une chaîne de caractères représentant le symbole de V                                      |
| `detect_extensions` | `["v"]`                                      | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                         | Les dossiers qui activent ce module.                                                       |
| `style`             | `"blue bold"`                                | Le style du module.                                                                        |
| `disabled`          | `false`                                      | Désactive le module `vlang`.                                                               |

### Variables

| Variable  | Exemple | Description                            |
| --------- | ------- | -------------------------------------- |
| version   | `v0.2`  | La version de `v`                      |
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

| Option     | Défaut                           | Description                                          |
| ---------- | -------------------------------- | ---------------------------------------------------- |
| `symbol`   |                                  | Le symbole utilisé avant d'afficher le nom du dépôt. |
| `style`    | `"bold yellow"`                  | Le style du module.                                  |
| `format`   | `"vcsh [$symbol$repo]($style) "` | Format du module.                                    |
| `disabled` | `false`                          | Désactive le module `vcsh`.                          |

### Variables

| Variable  | Exemple                                         | Description                            |
| --------- | ----------------------------------------------- | -------------------------------------- |
| repo      | `dotfiles` si dans un dépôt VCSH nommé dotfiles | Le nom du dépôt actif                  |
| symbol    |                                                 | Reflète la valeur de l'option `symbol` |
| style\* | `black bold dimmed`                             | Reflète la valeur de l'option `style`  |

*: Cette variable peut uniquement être utilisée dans une chaine de style

### Exemple

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

Par défaut, le module `zig` affiche la version actuellement installée de [Zig](https://ziglang.org/). Le module est affiché si l'une de ces conditions est remplie :

- Le répertoire courant contient un fichier `.zig`

### Options

| Option              | Défaut                               | Description                                                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | Format du module.                                                                          |
| `version_format`    | `"v${raw}"`                          | Le format de la version. Les variables disponibles sont `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | Le symbole utilisé avant d'afficher la version de Zig.                                     |
| `style`             | `"bold yellow"`                      | Le style du module.                                                                        |
| `disabled`          | `false`                              | Désactive le module `zig`.                                                                 |
| `detect_extensions` | `["zig"]`                            | Quelles extensions devraient activer ce module.                                            |
| `detect_files`      | `[]`                                 | Les fichiers qui activent ce module.                                                       |
| `detect_folders`    | `[]`                                 | Les dossiers qui activent ce module.                                                       |

### Variables

| Variable  | Exemple  | Description                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.6.0` | La version de `zig`                    |
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

Les modules `custom` affichent la sortie d’une commande arbitaitre.

Ces modules seront affichés si l'une de ces conditions est remplie:

- Le dossier courant contient un fichier dont le nom est dans `detect_files`
- Le dossier courant contient un dossier dont le nom est dans `detect_folders`
- Le dossier courant contient un fichier dont l’extension est dans `detect_extensions`
- La commande `when` retourne 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

::: tip

Plusieurs modules personnalisés peuvent être définis en utilisant un `.`.

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

| Option              | Défaut                          | Description                                                                                                                                                                                                                                                                                   |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `""`                            | La commande dont la sortie doit être affichée. La commande sera transmise au shell sur l’entrée standard.                                                                                                                                                                                     |
| `when`              | `false`                         | Soit une valeur booléenne (`true` ou `false`, sans guillemets) ou une commande shell utilisée comme condition pour afficher le module. Dans le cas d’une chaine, le module sera affiché si la commande renvoie un code de statut `0`.                                                         |
| `shell`             |                                 | [Voir plus bas](#custom-command-shell)                                                                                                                                                                                                                                                        |
| `description`       | `"<custom module>"`       | La description du module qui est affichée lors de l’exécution de `starship explain`.                                                                                                                                                                                                          |
| `detect_files`      | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                         |
| `detect_folders`    | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                   |
| `detect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                    |
| `symbol`            | `""`                            | Le symbole utilisé avant d'afficher la sortie de la commande.                                                                                                                                                                                                                                 |
| `style`             | `"bold green"`                  | Le style du module.                                                                                                                                                                                                                                                                           |
| `format`            | `"[$symbol($output )]($style)"` | Format du module.                                                                                                                                                                                                                                                                             |
| `disabled`          | `false`                         | Désactive le module `custom`.                                                                                                                                                                                                                                                                 |
| `os`                |                                 | Nom du système d'exploitation sur lequel le module sera affiché (unix, linux, macos, windows, ... ) [Voir les valeurs possibles](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                  |
| `use_stdin`         |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`    | `false`                         | Ignore le paramètre global `command_timeout` et continuer à exécuter des commandes externes, peu importe le temps qu'elles prennent.                                                                                                                                                          |

### Variables

| Variable  | Description                                 |
| --------- | ------------------------------------------- |
| output    | La sortie de la commande shell dans `shell` |
| symbol    | Reflète la valeur de l'option `symbol`      |
| style\* | Reflète la valeur de l'option `style`       |

*: Cette variable peut uniquement être utilisée dans une chaine de style

#### Commandes shell personnalisées

`shell` accepte une liste de chaînes non vide, où:

- La première chaîne est le chemin vers le shell à utiliser pour exécuter la commande.
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

### Exemple

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
