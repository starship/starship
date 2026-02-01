# Git State

The `git_state` module will show in directories which are part of a git
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc. If there is progress information (e.g., REBASING 3/10),
that information will be shown too.

### Options

| Option         | Default                                                       | Description                                                                             |
| -------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                  | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `'MERGING'`                                                   | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `'REVERTING'`                                                 | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `'CHERRY-PICKING'`                                            | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `'BISECTING'`                                                 | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `'AM'`                                                        | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `'AM/REBASE'`                                                 | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `'bold yellow'`                                               | The style for the module.                                                               |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | The format for the module.                                                              |
| `disabled`     | `false`                                                       | Disables the `git_state` module.                                                        |

### Variables

| Variable         | Example    | Description                         |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*          |            | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```
