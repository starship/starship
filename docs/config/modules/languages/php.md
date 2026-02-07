# PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐘 '`                              | The symbol used before displaying the version of PHP.                     |
| `detect_extensions` | `['php']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `['composer.json', '.php-version']`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'147 bold'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `php` module.                                                |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v7.3.8` | The version of `php`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```
