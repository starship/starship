# Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime.
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `bun.lock` file
- The current directory contains a `bun.lockb` file
- The current directory contains a `bunfig.toml` file

### Options

| Option              | Default                                    | Description                                                               |
| ------------------- | ------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🥟 '`                                    | A format string representing the symbol of Bun.                           |
| `detect_extensions` | `[]`                                       | Which extensions should trigger this module.                              |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                       | Which folders should trigger this module.                                 |
| `style`             | `'bold red'`                               | The style for the module.                                                 |
| `disabled`          | `false`                                    | Disables the `bun` module.                                                |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v0.1.4` | The version of `bun`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

#### Customize the format

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```
