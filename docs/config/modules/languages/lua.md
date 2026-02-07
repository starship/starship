# Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Options

| Option              | Default                              | Description                                                                |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                 |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`  |
| `symbol`            | `'🌙 '`                              | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `['lua']`                            | Which extensions should trigger this module.                               |
| `detect_files`      | `['.lua-version']`                   | Which filenames should trigger this module.                                |
| `detect_folders`    | `['lua']`                            | Which folders should trigger this module.                                  |
| `style`             | `'bold blue'`                        | The style for the module.                                                  |
| `lua_binary`        | `'lua'`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v5.4.0` | The version of `lua`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```
