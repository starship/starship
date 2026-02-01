# Git Status

The `git_status` module shows symbols representing the state of the repo in your
current directory.

> [!TIP]
> The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment.
> You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

### Options

| Option                 | Default                                       | Description                                                                                                 |
| ---------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`               | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status`                                                                         |
| `conflicted`           | `'='`                                         | This branch has merge conflicts.                                                                            |
| `ahead`                | `'⇡'`                                         | The format of `ahead`                                                                                       |
| `behind`               | `'⇣'`                                         | The format of `behind`                                                                                      |
| `diverged`             | `'⇕'`                                         | The format of `diverged`                                                                                    |
| `up_to_date`           | `''`                                          | The format of `up_to_date`                                                                                  |
| `untracked`            | `'?'`                                         | The format of `untracked`                                                                                   |
| `stashed`              | `'\$'`                                        | The format of `stashed`                                                                                     |
| `modified`             | `'!'`                                         | The format of `modified`                                                                                    |
| `staged`               | `'+'`                                         | The format of `staged`                                                                                      |
| `renamed`              | `'»'`                                         | The format of `renamed`                                                                                     |
| `deleted`              | `'✘'`                                         | The format of `deleted`                                                                                     |
| `typechanged`          | `""`                                          | The format of `typechanged`                                                                                 |
| `style`                | `'bold red'`                                  | The style for the module.                                                                                   |
| `ignore_submodules`    | `false`                                       | Ignore changes to submodules.                                                                               |
| `worktree_added`       | `""`                                          | The format of `worktree_added`                                                                              |
| `worktree_deleted`     | `""`                                          | The format of `worktree_deleted`                                                                            |
| `worktree_modified`    | `""`                                          | The format of `worktree_modified`                                                                           |
| `worktree_typechanged` | `""`                                          | The format of `worktree_typechanged`                                                                        |
| `index_added`          | `""`                                          | The format of `index_added`                                                                                 |
| `index_deleted`        | `""`                                          | The format of `index_deleted`                                                                               |
| `index_modified`       | `""`                                          | The format of `index_modified`                                                                              |
| `index_typechanged`    | `""`                                          | The format of `index_typechanged`                                                                           |
| `disabled`             | `false`                                       | Disables the `git_status` module.                                                                           |
| `windows_starship`     |                                               | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |
| `use_git_executable`   | `false`                                       | Do not use `gitoxide` for computing the status, but use the `git` executable instead.                       |

### Variables

The following variables can be used in `format`:

| Variable               | Description                                                                                                   |
| ---------------------- | ------------------------------------------------------------------------------------------------------------- |
| `all_status`           | Shortcut for`$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`                       |
| `ahead_behind`         | Displays `diverged`, `ahead`, `behind` or `up_to_date` format string based on the current status of the repo. |
| `conflicted`           | Displays `conflicted` when this branch has merge conflicts.                                                   |
| `untracked`            | Displays `untracked` when there are untracked files in the working directory.                                 |
| `stashed`              | Displays `stashed` when a stash exists for the local repository.                                              |
| `modified`             | Displays `modified` when there are file modifications in the working directory.                               |
| `staged`               | Displays `staged` when a new file has been added to the staging area.                                         |
| `renamed`              | Displays `renamed` when a renamed file has been added to the staging area.                                    |
| `deleted`              | Displays `deleted` when a file's deletion has been added to the staging area.                                 |
| `typechanged`          | Displays `typechanged` when a file's type has been changed in the staging area.                               |
| `worktree_added`       | Displays `worktree_added` when a new file has been added in the working directory.                            |
| `worktree_deleted`     | Displays `worktree_deleted` when a file's been deleted in the working directory.                              |
| `worktree_modified`    | Displays `worktree_modified` when a file's been modified in the working directory.                            |
| `worktree_typechanged` | Displays `worktree_typechanged` when a file's type has been changed in the working directory.                 |
| `index_added`          | Displays `index_added` when a new file has been added to the staging area.                                    |
| `index_deleted`        | Displays `index_deleted` when a file's been deleted to the staging area.                                      |
| `index_modified`       | Displays `index_modified` when a file's been modified to the staging area.                                    |
| `index_typechanged`    | Displays `index_typechanged` when a file's type has been changed to the staging area.                         |
| style\*                | Mirrors the value of option `style`                                                                           |

*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Variable       | Description                                    |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed`, `deleted`, `typechanged`, `worktree_added`, `worktree_deleted`, `worktree_modified`, `worktree_typechanged`, `index_added`, `index_deleted`, `index_modified`, and `index_typechanged`:

| Variable | Description              |
| -------- | ------------------------ |
| `count`  | Show the number of files |

### Example

```toml
# ~/.config/starship.toml

[git_status]
conflicted = '🏳'
ahead = '🏎💨'
behind = '😰'
diverged = '😵'
up_to_date = '✓'
untracked = '🤷'
stashed = '📦'
modified = '📝'
staged = '[++\($count\)](green)'
renamed = '👅'
deleted = '🗑'
```

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = '⇡${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
behind = '⇣${count}'
```

Use Windows Starship executable on Windows paths in WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```
