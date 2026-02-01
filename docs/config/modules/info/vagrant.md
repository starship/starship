# Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Vagrantfile` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⍱ '`                               | A format string representing the symbol of Vagrant.                       |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['Vagrantfile']`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'cyan bold'`                        | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                            |

### Variables

| Variable | Example          | Description                          |
| -------- | ---------------- | ------------------------------------ |
| version  | `Vagrant 2.2.10` | The version of `Vagrant`             |
| symbol   |                  | Mirrors the value of option `symbol` |
| style\*  |                  | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```
