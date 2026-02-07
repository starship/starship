# Mercurial Branch

The `hg_branch` module shows the active branch and topic of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                                   | Description                                                                                  |
| ------------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `' '`                                    | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `'bold purple'`                           | The style for the module.                                                                    |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | The format for the module.                                                                   |
| `truncation_length` | `2^63 - 1`                                | Truncates the hg branch / topic name to `N` graphemes                                        |
| `truncation_symbol` | `'…'`                                     | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                                    | Disables the `hg_branch` module.                                                             |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| branch   | `master`  | The active mercurial branch          |
| topic    | `feature` | The active mercurial topic           |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```
