# Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Options

| Option              | Default                                | Description                                                                                                                                           |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to. `0` means no truncation. Also see the [`directory`](../core/directory) module. |
| `symbol`            | `'🅢  '`                                | The symbol used before the environment name.                                                                                                          |
| `style`             | `'bold blue'`                          | The style for the module.                                                                                                                             |
| `format`            | `'via [$symbol$environment]($style) '` | The format for the module.                                                                                                                            |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                          |

### Variables

| Variable    | Example      | Description                          |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current spack environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*     |              | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```
