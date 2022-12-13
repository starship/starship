# Migrer vers la v0.45.0

La version 0.45.0 de Starship contient des changements non rétro-compatibles, en préparation de la v1.0.0. Nous avons apporté des changements majeurs à la manière de configurer l'invite, afin de permettre une personnalisation bien plus poussée.

Ce guide a pour but de vous guider à travers les changements majeurs qui ont eu lieu.

## `prompt_order` a été remplacé par `format`, situé à la racine

Avant la v0.45.0, `prompt_order` acceptait un tableau de noms de modules dans l'ordre dans lequel ils devaient être affichés par Starship.

Starship v0.45.0 accepte à la place une valeur de `format` permettant la personnalisation de l'invite en dehors des modules eux-mêmes.

**Exemple de configuration pré-v0.45.0**

```toml
prompt_order = [
  "username",
  "hostname",
  "directory",
  "git_branch",
  "git_commit",
  "git_state",
  "git_status",
  "cmd_duration",
  "custom",
  "line_break",
  "jobs",
  "battery",
  "time",
  "character",
]
```

**Exemple de configuration post-v0.45.0**

```toml
format = """\
  $username\
  $hostname\
  $directory\
  $git_branch\
  $git_commit\
  $git_state\
  $git_status\
  $cmd_duration\
  $custom\
  $line_break\
  $jobs\
  $battery\
  $time\
  $character\
  """
```

## Les modules `prefix` et `suffix` ont été remplacés par `format`

Antérieurement à la v0.45.0, certains modules acceptaient un `prefix` et/ou un `suffix` afin de personnaliser la façon dont les modules sont affichés.

Starship v0.45.0 accepte à la place une valeur de `format` permettant une personnalisation supplémentaire de la façon dont les modules sont rendus. Au lieu de définir un préfixe et un suffixe pour les variables contextuelles, les variables peuvent maintenant être remplacées dans une chaîne de format, qui représente la sortie du module.

**Exemple de configuration pré-v0.45.0**

```toml
[cmd_duration]
prefix = "took "
```

**Exemple de configuration post-v0.45.0**

```toml
[cmd_duration]
# $duration – La durée de la commande (p. ex. "15s")
# $style    – Le style par défaut du module (p. ex. "bold yellow")
format = "took [$duration]($style) "
```

### Modules affectés

#### Caractère

| Propriété supprimée     | Remplacement     |
| ----------------------- | ---------------- |
| `symbole`               | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Modifications de la configuration par défaut**

```diff
[character]
-- symbol = "❯"
-- error_symbol = "✖"
-- use_symbol_for_status = true
-- vicmd_symbol = "❮"
++ success_symbol = "[❯](bold green)"
++ error_symbol = "[❯](bold red)"
++ vicmd_symbol = "[❮](bold green)"
```

Précédemment, la propriété `use_symbol_for_status` était utilisée pour configurer l'invite afin d'afficher le `error_symbol` lorsque la dernière commande aboutissait à un résultat non-nul.

Depuis la version v0.45.0, nous utilisons systématiquement `error_symbol` après un code de statut non-nul, unifiant ainsi les propriétés `use_symbol_for_status` et `error_symbol`.

Pour que l'invite utilise l'ancienne configuration `use_symbol_for_status = true`, ajoutez la ligne suivante à votre configuration:

```toml
[character]
error_symbol = "[✖](bold red)"
```

_Note:_ L'élément `character` est automatiquement suivi d'un espace, donc à la différence des autres valeurs `format`, nous n'en ajoutons pas dans les exemples précédents.

#### Temps d'exécution

| Propriété supprimée | Remplacement |
| ------------------- | ------------ |
| `prefix`            | `format`     |

**Modifications de la configuration par défaut**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Dossier

| Propriété supprimée | Remplacement |
| ------------------- | ------------ |
| `prefix`            | `format`     |

**Modifications de la configuration par défaut**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Variable d'environnement

| Propriété supprimée | Remplacement |
| ------------------- | ------------ |
| `prefix`            | `format`     |
| `suffix`            | `format`     |

**Modifications de la configuration par défaut**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Commit Git

| Propriété supprimée | Remplacement |
| ------------------- | ------------ |
| `prefix`            | `format`     |
| `suffix`            | `format`     |

**Modifications de la configuration par défaut**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Statut Git

| Propriété supprimée | Remplacement |
| ------------------- | ------------ |
| `prefix`            | `format`     |
| `suffix`            | `format`     |
| `show_sync_count`   | `format`     |

**Modifications de la configuration par défaut**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Précédemment, la propriété `show_sync_count` était utilisée pour configurer l'invite de manière à afficher le nombre de commits en avance ou en retard de la branche locale par rapport à la branche distante.

Avec l'arrivée de la v0.45.0, cela a été séparé en trois propriétés distinctes, `ahead`, `behind`, et `diverged`.

Pour que l'invite utilise l'ancienne configuration `show_sync_count = true`, ajoutez la ligne suivante à votre configuration:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Nom d'hôte

| Propriété supprimée | Remplacement |
| ------------------- | ------------ |
| `prefix`            | `format`     |
| `suffix`            | `format`     |

**Modifications de la configuration par défaut**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| Propriété supprimée | Remplacement |
| ------------------- | ------------ |
| `label`             | `format`     |
| `prefix`            | `format`     |
| `suffix`            | `format`     |

**Modifications de la configuration par défaut**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Date et Heure

| Propriété supprimée | Remplacement  |
| ------------------- | ------------- |
| `format`            | `time_format` |

**Modifications de la configuration par défaut**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Commandes personnalisées

| Propriété supprimée | Remplacement |
| ------------------- | ------------ |
| `prefix`            | `format`     |
| `suffix`            | `format`     |

**Modifications de la configuration par défaut**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
