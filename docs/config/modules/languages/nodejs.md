# Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

Additionally, the module will be hidden by default if the directory contains a `bunfig.toml`, `bun.lock`, or `bun.lockb` file, overriding the above conditions.

### Options

| Option              | Default                                       | Description                                                                                           |
| ------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | The format for the module.                                                                            |
| `version_format`    | `'v${raw}'`                                   | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                             |
| `symbol`            | `' '`                                        | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | Which extensions should trigger this module.                                                          |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | Which filenames should trigger this module.                                                           |
| `detect_folders`    | `['node_modules']`                            | Which folders should trigger this module.                                                             |
| `style`             | `'bold green'`                                | The style for the module.                                                                             |
| `disabled`          | `false`                                       | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `'bold red'`                                  | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variables

| Variable        | Example    | Description                                                                                                                                               |
| --------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version         | `v13.12.0` | The version of `node`                                                                                                                                     |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbol          |            | Mirrors the value of option `symbol`                                                                                                                      |
| style\*         |            | Mirrors the value of option `style`                                                                                                                       |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```
