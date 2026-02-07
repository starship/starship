# Buf

The `buf` module shows the currently installed version of [Buf](https://buf.build). By default, the module is shown if the current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### Options

| Option              | Default                                         | Description                                           |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | The format for the `buf` module.                      |
| `version_format`    | `'v${raw}'`                                     | The version format.                                   |
| `symbol`            | `'🐃 '`                                         | The symbol used before displaying the version of Buf. |
| `detect_extensions` | `[]`                                            | Which extensions should trigger this module.          |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                            | Which folders should trigger this modules.            |
| `style`             | `'bold blue'`                                   | The style for the module.                             |
| `disabled`          | `false`                                         | Disables the `elixir` module.                         |

### Variables

| Variable  | Example  | Description                          |
| --------- | -------- | ------------------------------------ |
| `version` | `v1.0.0` | The version of `buf`                 |
| `symbol`  |          | Mirrors the value of option `symbol` |
| `style`*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```
