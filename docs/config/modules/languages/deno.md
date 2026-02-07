# Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Options

| Option              | Default                                                                              | Description                                                               |
| ------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                                                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦕 '`                                                                              | A format string representing the symbol of Deno                           |
| `detect_extensions` | `[]`                                                                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                 | Which folders should trigger this module.                                 |
| `style`             | `'green bold'`                                                                       | The style for the module.                                                 |
| `disabled`          | `false`                                                                              | Disables the `deno` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.8.3` | The version of `deno`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```
