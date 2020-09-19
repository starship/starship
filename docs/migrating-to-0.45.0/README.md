# Migrating to v0.45.0

Starship v0.45.0 is a release containing breaking changes, in preparation for the big v1.0.0. We have made some major changes around how configuration is done on the prompt, to allow for a greater degree of customization.

This guide is intended to walk you through the breaking changes.

## `prompt_order` has been replaced by a root-level `format`

Previously to v0.45.0, `prompt_order` would accept an array of module names
in the order which they should be rendered by Starship.

Starship v0.45.0 will instead be accepting a `format` value, allowing for customization of the prompt outside of the modules themselves.

**Example pre-v0.45.0 configuration**

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

**Example v0.45.0 configuration**

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

## Module `prefix` and `suffix` will be replaced by `format`

Previously to v0.45.0, some modules would accept `prefix` and/or `suffix`
in order to stylize the way that modules are rendered.

Starship v0.45.0 will instead be accepting a `format` value, allowing for further
customization of how modules are rendered. Instead of defining a prefix and suffix
for the context-based variables, the variables can now be substituted from within
a format string, which represents the module's output.

**Example pre-v0.45.0 configuration**

```toml
[cmd_duration]
prefix = "took "
```

**Example v0.45.0 configuration**

```toml
[cmd_duration]
# $duration – The command duration (e.g. "15s")
# $style    – The default style of the module (e.g. "bold yellow")
format = "took [$duration]($style)"
```

### Affected Modules

#### Command Duration

| Deprecated Property | Replacement |
| ------------------- | ----------- |
| `prefix`            | `format`    |

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style)"
```

#### Directory

| Deprecated Property | Replacement |
| ------------------- | ----------- |
| `prefix`            | `format`    |

```diff
[cmd_duration]
-- prefix = "in "
++ format = "[$path]($style)[$lock_symbol]($lock_style)"
```

#### Environment Variable

| Deprecated Property | Replacement |
| ------------------- | ----------- |
| `prefix`            | `format`    |
| `suffix`            | `format`    |

```diff
[cmd_duration]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git Commit

| Deprecated Property | Replacement |
| ------------------- | ----------- |
| `prefix`            | `format`    |
| `suffix`            | `format`    |

```diff
[cmd_duration]
-- prefix = "("
-- suffix = ")"
++ format = "[\\($hash\\)]($style) "
```

#### Git Status

| Deprecated Property | Replacement |
| ------------------- | ----------- |
| `prefix`            | `format`    |
| `suffix`            | `format`    |

```diff
[cmd_duration]
-- prefix = "["
-- suffix = "]"
++ format = "([$all_status$ahead_behind] )"
```

#### Hostname

| Deprecated Property | Replacement |
| ------------------- | ----------- |
| `prefix`            | `format`    |
| `suffix`            | `format`    |

```diff
[cmd_duration]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| Deprecated Property | Replacement |
| ------------------- | ----------- |
| `label`             | `format`    |
| `prefix`            | `format`    |
| `suffix`            | `format`    |

```diff
[cmd_duration]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol\\[$env\\]]($style) "
```

#### Custom Commands

| Deprecated Property | Replacement |
| ------------------- | ----------- |
| `prefix`            | `format`    |
| `suffix`            | `format`    |

```diff
[cmd_duration]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
