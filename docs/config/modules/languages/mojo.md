# Mojo

The `mojo` module shows the current version of [Mojo programming language](https://www.modular.com/mojo) installed

### Options

| Option              | Default                               | Description                                            |
| ------------------- | ------------------------------------- | ------------------------------------------------------ |
| `format`            | `'with [$symbol($version )]($style)'` | The format for the module.                             |
| `symbol`            | `'🔥 '`                               | The symbol used before displaying the version of Mojo. |
| `style`             | `'bold 208'`                          | The style for the module.                              |
| `disabled`          | `false`                               | Disables the `mojo` module.                            |
| `detect_extensions` | `['mojo', '🔥']`                      | Which extensions should trigger this module.           |
| `detect_files`      | `[]`                                  | Which filenames should trigger this module.            |
| `detect_folders`    | `[]`                                  | Which folders should trigger this module.              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `24.4.0` | The version of `mojo`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )($hash )]($style)'
```
