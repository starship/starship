# Direnv

The `direnv` module shows the status of the current rc file if one is present. The status includes the path to the rc file, whether it is loaded, and whether it has been allowed by `direnv`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                                | Description                                             |
| ------------------- | -------------------------------------- | ------------------------------------------------------- |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | The format for the module.                              |
| `symbol`            | `'direnv '`                            | The symbol used before displaying the direnv context.   |
| `style`             | `'bold orange'`                        | The style for the module.                               |
| `disabled`          | `true`                                 | Disables the `direnv` module.                           |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this module.            |
| `detect_files`      | `['.envrc']`                           | Which filenames should trigger this module.             |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module.               |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Which environment variables should trigger this module. |
| `allowed_msg`       | `'allowed'`                            | The message displayed when an rc file is allowed.       |
| `not_allowed_msg`   | `'not allowed'`                        | The message displayed when an rc file is not_allowed.   |
| `denied_msg`        | `'denied'`                             | The message displayed when an rc file is denied.        |
| `loaded_msg`        | `'loaded'`                             | The message displayed when an rc file is loaded.        |
| `unloaded_msg`      | `'not loaded'`                         | The message displayed when an rc file is not loaded.    |

### Variables

| Variable | Example             | Description                             |
| -------- | ------------------- | --------------------------------------- |
| loaded   | `loaded`            | Whether the current rc file is loaded.  |
| allowed  | `denied`            | Whether the current rc file is allowed. |
| rc_path  | `/home/test/.envrc` | The current rc file path.               |
| symbol   |                     | Mirrors the value of option `symbol`.   |
| style\*  | `red bold`          | Mirrors the value of option `style`.    |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```
