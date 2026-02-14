# V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Options

| Option              | Default                                      | Description                                                               |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`         | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'V '`                                       | A format string representing the symbol of V                              |
| `detect_extensions` | `['v']`                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                         | Which folders should trigger this module.                                 |
| `style`             | `'blue bold'`                                | The style for the module.                                                 |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                              |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| version  | `v0.2`  | The version of `v`                   |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```
