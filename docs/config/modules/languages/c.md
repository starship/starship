# C

The `c` module shows some information about your C compiler. By default
the module will be shown if the current directory contains a `.c` or `.h`
file.

### Options

| Option              | Default                                                                       | Description                                                               |
| ------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | The format string for the module.                                         |
| `version_format`    | `'v${raw}'`                                                                   | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'C '`                                                                        | The symbol used before displaying the compiler details                    |
| `detect_extensions` | `['c', 'h']`                                                                  | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                                                          | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                          | Which folders should trigger this module.                                 |
| `commands`          | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | How to detect what the compiler is                                        |
| `style`             | `'bold 149'`                                                                  | The style for the module.                                                 |
| `disabled`          | `false`                                                                       | Disables the `c` module.                                                  |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| name     | clang   | The name of the compiler             |
| version  | 13.0.0  | The version of the compiler          |
| symbol   |         | Mirrors the value of option `symbol` |
| style    |         | Mirrors the value of option `style`  |

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Example

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```
