# Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

> [!TIP]
> By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms).
> If you still want to enable it, [follow the example shown below](#with-pulumi-version).

By default the module will be shown if any of the following conditions are met:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Options

| Option           | Default                                      | Description                                                               |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | The format string for the module.                                         |
| `version_format` | `'v${raw}'`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `' '`                                       | A format string shown before the Pulumi stack.                            |
| `style`          | `'bold 5'`                                   | The style for the module.                                                 |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.            |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                             |

### Variables

| Variable | Example    | Description                          |
| -------- | ---------- | ------------------------------------ |
| version  | `v0.12.24` | The version of `pulumi`              |
| stack    | `dev`      | The current Pulumi stack             |
| username | `alice`    | The current Pulumi username          |
| symbol   |            | Mirrors the value of option `symbol` |
| style\*  |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

#### With Pulumi Version

```toml
# ~/.config/starship.toml

[pulumi]
format = '[🛥 ($version )$stack]($style) '
```

#### Without Pulumi version

```toml
# ~/.config/starship.toml
[pulumi]
symbol = '🛥 '
format = '[$symbol$stack]($style) '
```
