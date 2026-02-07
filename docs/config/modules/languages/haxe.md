# Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### Options

| Option              | Default                                                                                         | Description                                                               |
| ------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                                                                     | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | Which extensions should trigger this module.                              |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | Which folders should trigger this modules.                                |
| `symbol`            | `'⌘ '`                                                                                          | A format string representing the symbol of Haxe.                          |
| `style`             | `'bold fg:202'`                                                                                 | The style for the module.                                                 |
| `disabled`          | `false`                                                                                         | Disables the `haxe` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v4.2.5` | The version of `haxe`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```
