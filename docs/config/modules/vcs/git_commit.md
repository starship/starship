# Git Commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Options

| Option               | Default                      | Description                                                                          |
| -------------------- | ---------------------------- | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                          | The length of the displayed git commit hash.                                         |
| `format`             | `'[\($hash$tag\)]($style) '` | The format for the module.                                                           |
| `style`              | `'bold green'`               | The style for the module.                                                            |
| `only_detached`      | `true`                       | Only show git commit hash when in detached `HEAD` state                              |
| `tag_disabled`       | `true`                       | Disables showing tag info in `git_commit` module.                                    |
| `tag_max_candidates` | `0`                          | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `' 🏷  '`                     | Tag symbol prefixing the info shown                                                  |
| `disabled`           | `false`                      | Disables the `git_commit` module.                                                    |

### Variables

| Variable | Example   | Description                                  |
| -------- | --------- | -------------------------------------------- |
| hash     | `b703eb3` | The current git commit hash                  |
| tag      | `v1.0.0`  | The tag name if showing tag info is enabled. |
| style\*  |           | Mirrors the value of option `style`          |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```
