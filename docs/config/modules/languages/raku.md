# Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Options

| Option              | Default                                          | Description                                                               |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | The format string for the module.                                         |
| `version_format`    | `'v${raw}'`                                      | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦋 '`                                          | The symbol used before displaying the version of Raku                     |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Which extensions should trigger this module.                              |
| `detect_files`      | `['META6.json']`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                             | Which folders should trigger this module.                                 |
| `style`             | `'bold 149'`                                     | The style for the module.                                                 |
| `disabled`          | `false`                                          | Disables the `raku` module.                                               |

### Variables

| Variable   | Example | Description                          |
| ---------- | ------- | ------------------------------------ |
| version    | `v6.d`  | The version of `raku`                |
| vm_version | `moar`  | The version of VM `raku` is built on |
| symbol     |         | Mirrors the value of option `symbol` |
| style\*    |         | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```
