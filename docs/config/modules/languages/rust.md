# Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦀 '`                              | A format string representing the symbol of Rust                           |
| `detect_extensions` | `['rs']`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `['Cargo.toml']`                     | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'bold red'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `rust` module.                                               |

### Variables

| Variable  | Example           | Description                                  |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | Mirrors the value of option `symbol`         |
| style\*   |                   | Mirrors the value of option `style`          |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```
