# Mercurial State

The `hg_state` module will show in directories which are part of a mercurial
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option       | Default                   | Description                                                   |
| ------------ | ------------------------- | ------------------------------------------------------------- |
| `merge`      | `'MERGING'`               | A format string displayed when a `merge` is in progress.      |
| `rebase`     | `'REBASING'`              | A format string displayed when a `rebase` is in progress.     |
| `update`     | `'UPDATING'`              | A format string displayed when a `update` is in progress.     |
| `bisect`     | `'BISECTING'`             | A format string displayed when a `bisect` is in progress.     |
| `shelve`     | `'SHELVING'`              | A format string displayed when a `shelve` is in progress.     |
| `graft`      | `'GRAFTING'`              | A format string displayed when a `graft` is in progress.      |
| `transplant` | `'TRANSPLANTING'`         | A format string displayed when a `transplant` is in progress. |
| `histedit`   | `'HISTEDITING'`           | A format string displayed when a `histedit` is in progress.   |
| `style`      | `'bold yellow'`           | The style for the module.                                     |
| `format`     | `'\([$state]($style)\) '` | The format for the module.                                    |
| `disabled`   | `true`                    | Disables the `hg_state` module.                               |

### Variables

| Variable         | Example    | Description                         |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*          |            | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string
