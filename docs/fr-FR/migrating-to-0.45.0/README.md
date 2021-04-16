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

#### Character

| Propriété supprimée     | Remplacement     |
| ----------------------- | ---------------- |
| `symbol`                | `success_symbol` |
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

Previously, the `use_symbol_for_status` property was used to configure the prompt to show the `error_symbol` when the last command resulted in a non-zero status code.

With the release of v0.45.0, we now always use `error_symbol` after non-zero status codes, unifying `use_symbol_for_status` and `error_symbol` properties.

To configure the prompt to use the older `use_symbol_for_status = true` configuration, add the following to your config file:

```toml
[character]
error_symbol = "[✖](bold red)"
```

*Note:* The `character` element automatically adds a space after, so unlike the other `format` strings, we specifically do not add one in the above examples.

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

#### Environment Variable

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

#### Git Commit

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

#### Git Status

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

Previously, the `show_sync_count` property was used to configure the prompt to show the number of commits the branch was ahead or behind the remote branch.

With the release of v0.45.0, this has been replaced with three separate properties, `ahead`, `behind`, and `diverged`.

To configure the prompt to use the older `show_sync_count = true` configuration, set the following to your config file:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

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

#### Temps

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

#### Custom Commands

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
