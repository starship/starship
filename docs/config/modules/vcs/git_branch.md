# Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Options

| Option               | Default                                           | Description                                                                              |
| -------------------- | ------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Shows the remote tracking branch name, even if it is equal to the local branch name.     |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | The format for the module. Use `'$branch'` to refer to the current branch name.          |
| `symbol`             | `' '`                                            | A format string representing the symbol of git branch.                                   |
| `style`              | `'bold purple'`                                   | The style for the module.                                                                |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                                                 |
| `truncation_symbol`  | `'…'`                                             | The symbol used to indicate a branch name was truncated. You can use `''` for no symbol. |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state.                           |
| `ignore_branches`    | `[]`                                              | A list of names to avoid displaying. Useful for 'master' or 'main'.                      |
| `ignore_bare_repo`   | `false`                                           | Do not show when in a bare repo.                                                         |
| `disabled`           | `false`                                           | Disables the `git_branch` module.                                                        |

### Variables

| Variable      | Example  | Description                                                                                            |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | Mirrors the value of option `symbol`                                                                   |
| style\*       |          | Mirrors the value of option `style`                                                                    |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[git_branch]
symbol = '🌱 '
truncation_length = 4
truncation_symbol = ''
ignore_branches = ['master', 'main']
```
