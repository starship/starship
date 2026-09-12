# Migration auf v0.45.0

Starship v0.45.0 ist ein Release mit bahnbrechenden Änderungen in Vorbereitung auf das große v1.0.0. Wir haben einige wichtige Änderungen vorgenommen, wie die Konfiguration auf der Eingabeaufforderung durchgeführt wird, um einen höheren Grad an Individualisierung zu ermöglichen.

This guide is intended to walk you through the breaking changes.

## `prompt_order` has been replaced by a root-level `format`

Previously to v0.45.0, `prompt_order` would accept an array of module names in the order which they should be rendered by Starship.

Starship v0.45.0 instead accepts a `format` value, allowing for customization of the prompt outside of the modules themselves.

**Beispiel für Pre-v0.45.0 Konfiguration**

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

**Beispiel für v0.45.0 Konfiguration**

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

## Module `prefix` and `suffix` have been replaced by `format`

Previously to v0.45.0, some modules would accept `prefix` and/or `suffix` in order to stylize the way that modules are rendered.

Starship v0.45.0 instead accepts a `format` value, allowing for further customization of how modules are rendered. Instead of defining a prefix and suffix for the context-based variables, the variables can now be substituted from within a format string, which represents the module's output.

**Beispiel für Pre-v0.45.0 Konfiguration**

```toml
[cmd_duration]
prefix = "took "
```

**Beispiel für v0.45.0 Konfiguration**

```toml
[cmd_duration]
# $duration – The command duration (e.g. "15s")
# $style    – The default style of the module (e.g. "bold yellow")
format = "took [$duration]($style) "
```

### Betroffene Module

#### Zeichen

| Entfernte Eigenschaft   | Ersetzung        |
| ----------------------- | ---------------- |
| `Symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Änderungen an der Standardkonfiguration**

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

_Note:_ The `character` element automatically adds a space after, so unlike the other `format` strings, we specifically do not add one in the above examples.

#### Befehlsdauer

| Entfernte Eigenschaft | Ersetzung |
| --------------------- | --------- |
| `prefix`              | `format`  |

**Änderungen an der Standardkonfiguration**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Verzeichnis

| Entfernte Eigenschaft | Ersetzung |
| --------------------- | --------- |
| `prefix`              | `format`  |

**Änderungen an der Standardkonfiguration**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Umgebungsvariablen

| Entfernte Eigenschaft | Ersetzung |
| --------------------- | --------- |
| `prefix`              | `format`  |
| `suffix`              | `format`  |

**Änderungen an der Standardkonfiguration**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git-Commit

| Entfernte Eigenschaft | Ersetzung |
| --------------------- | --------- |
| `prefix`              | `format`  |
| `suffix`              | `format`  |

**Änderungen an der Standardkonfiguration**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git-Status

| Entfernte Eigenschaft | Ersetzung |
| --------------------- | --------- |
| `prefix`              | `format`  |
| `suffix`              | `format`  |
| `show_sync_count`     | `format`  |

**Änderungen an der Standardkonfiguration**

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

| Entfernte Eigenschaft | Ersetzung |
| --------------------- | --------- |
| `prefix`              | `format`  |
| `suffix`              | `format`  |

**Änderungen an der Standardkonfiguration**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularität

| Entfernte Eigenschaft | Ersetzung |
| --------------------- | --------- |
| `Label`               | `format`  |
| `prefix`              | `format`  |
| `suffix`              | `format`  |

**Änderungen an der Standardkonfiguration**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Uhrzeit

| Entfernte Eigenschaft | Ersetzung     |
| --------------------- | ------------- |
| `format`              | `time_format` |

**Änderungen an der Standardkonfiguration**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Benutzerdefinierte Befehle

| Entfernte Eigenschaft | Ersetzung |
| --------------------- | --------- |
| `prefix`              | `format`  |
| `suffix`              | `format`  |

**Änderungen an der Standardkonfiguration**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
