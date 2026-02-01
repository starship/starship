# Odin

The `odin` module shows the currently installed version of [Odin](https://odin-lang.org/). By default the module will be shown if the current directory contains a `.odin` file.

### Options

| Option              | Default                              | Description                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                             |
| `show_commit`       | `false`                              | Shows the commit as part of the version.               |
| `symbol`            | `'Ø '`                               | The symbol used before displaying the version of Odin. |
| `style`             | `'bold bright-blue'`                 | The style for the module.                              |
| `disabled`          | `false`                              | Disables the `odin` module.                            |
| `detect_extensions` | `['odin']`                           | Which extensions should trigger this module.           |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.            |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.              |

### Variables

| Variable | Example       | Description                          |
| -------- | ------------- | ------------------------------------ |
| version  | `dev-2024-03` | The version of `odin`                |
| symbol   |               | Mirrors the value of option `symbol` |
| style\*  |               | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
```
