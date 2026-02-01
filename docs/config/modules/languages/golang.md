# Go

The `golang` module shows the currently installed version of [Go](https://golang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `go.work` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Options

| Option              | Default                                                                                   | Description                                                                                                |
| ------------------- | ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | The format for the module.                                                                                 |
| `version_format`    | `'v${raw}'`                                                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                                  |
| `symbol`            | `'🐹 '`                                                                                   | A format string representing the symbol of Go.                                                             |
| `detect_extensions` | `['go']`                                                                                  | Which extensions should trigger this module.                                                               |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Which filenames should trigger this module.                                                                |
| `detect_folders`    | `['Godeps']`                                                                              | Which folders should trigger this module.                                                                  |
| `style`             | `'bold cyan'`                                                                             | The style for the module.                                                                                  |
| `not_capable_style` | `'bold red'`                                                                              | The style for the module when the go directive in the go.mod file does not match the installed Go version. |
| `disabled`          | `false`                                                                                   | Disables the `golang` module.                                                                              |

### Variables

| Variable    | Example   | Description                                                                                                                                 |
| ----------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| version     | `v1.12.1` | The version of `go`                                                                                                                         |
| mod_version | `1.16`    | `go` version requirement as set in the go directive of `go.mod`. Will only show if the version requirement does not match the `go` version. |
| symbol      |           | Mirrors the value of option `symbol`                                                                                                        |
| style\*     |           | Mirrors the value of option `style`                                                                                                         |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

### Using `mod_version`

```toml
# ~/.config/starship.toml

[golang]
format = 'via [$symbol($version )($mod_version )]($style)'
```
