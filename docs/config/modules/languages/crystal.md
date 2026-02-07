# Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `'🔮 '`                              | The symbol used before displaying the version of crystal.                 |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | The style for the module.                                                 |
| `detect_extensions` | `['cr']`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `['shard.yml']`                      | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `crystal` module.                                            |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v0.32.1` | The version of `crystal`             |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```
