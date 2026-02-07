# Directory

The `directory` module shows the path to your current directory, truncated to
three parent folders. Your directory will also be truncated to the root of the
git repo that you're currently in.

When using the `fish_style_pwd_dir_length` option, instead of hiding the path that is
truncated, you will see a shortened name of each directory based on the number
you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root,
and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before
it would have been `nixpkgs/pkgs`.

### Options

| Option                   | Default                                                                                                                      | Description                                                                                                |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | The number of parent folders that the current directory should be truncated to.                            |
| `truncate_to_repo`       | `true`                                                                                                                       | Whether or not to truncate to the root of the git repo that you're currently in.                           |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | The format for the module.                                                                                 |
| `style`                  | `'bold cyan'`                                                                                                                | The style for the module.                                                                                  |
| `disabled`               | `false`                                                                                                                      | Disables the `directory` module.                                                                           |
| `read_only`              | `'🔒'`                                                                                                                       | The symbol indicating current directory is read only.                                                      |
| `read_only_style`        | `'red'`                                                                                                                      | The style for the read only symbol.                                                                        |
| `truncation_symbol`      | `''`                                                                                                                         | The symbol to prefix to truncated paths. eg: '…/'                                                          |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. The default value is equivalent to `style`. |
| `repo_root_style`        |                                                                                                                              | The style for the root of the git repo. The default value is equivalent to `style`.                        |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                   |
| `home_symbol`            | `'~'`                                                                                                                        | The symbol indicating home directory.                                                                      |
| `use_os_path_sep`        | `true`                                                                                                                       | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows)                       |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | Default | Description                                                                                                                                                            |
| --------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |         | An Array or table of substitutions to be made to the path.                                                                                                             |
| `fish_style_pwd_dir_length` | `0`     | The number of characters to use when applying fish shell pwd path logic.                                                                                               |
| `use_logical_path`          | `true`  | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network
prefixes or development directories of Java. Note that this will disable the fish style PWD. It takes an array of the following
key/value pairs:

| Value   | Type    | Description                              |
| ------- | ------- | ---------------------------------------- |
| `from`  | String  | The value to substitute                  |
| `to`    | String  | The replacement for that value, if found |
| `regex` | Boolean | (Optional) Whether `from` is a regex     |

By using `regex = true`, you can use [Rust's regular expressions](https://docs.rs/regex/latest/regex/#syntax) in `from`.
For instance you can replace every slash except the first with the following:

```toml
substitutions = [
  { from = "^/", to = "<root>/", regex = true },
  { from = "/", to = " | " },
  { from = "^<root>", to = "/", regex = true },
]
```

This will replace `/var/log` to `/ | var | log`.

The old syntax still works, although it doesn't support regular expressions:

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero,
the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path
`/built/this/city/on/rock/and/roll`, which would normally be displayed as `rock/and/roll`, would be displayed as
`/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with
a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable | Example               | Description                         |
| -------- | --------------------- | ----------------------------------- |
| path     | `'D:/Projects'`       | The current directory path          |
| style\*  | `'black bold dimmed'` | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

<details>
<summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| Variable         | Example               | Description                             |
| ---------------- | --------------------- | --------------------------------------- |
| before_root_path | `'/path/to/home/'`    | The path before git root directory path |
| repo_root        | `'git_repo'`          | The git root directory name             |
| path             | `'/src/lib'`          | The remaining path                      |
| style            | `'black bold dimmed'` | Mirrors the value of option `style`     |
| repo_root_style  | `'underline white'`   | Style for git root directory name       |

</details>

### Example

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```
