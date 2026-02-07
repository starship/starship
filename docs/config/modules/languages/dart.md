# Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml`, `pubspec.yml` or `pubspec.lock` file

### Options

| Option              | Default                                           | Description                                                               |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`              | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🎯 '`                                           | A format string representing the symbol of Dart                           |
| `detect_extensions` | `['dart']`                                        | Which extensions should trigger this module.                              |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.dart_tool']`                                  | Which folders should trigger this module.                                 |
| `style`             | `'bold blue'`                                     | The style for the module.                                                 |
| `disabled`          | `false`                                           | Disables the `dart` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v2.8.4` | The version of `dart`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```
