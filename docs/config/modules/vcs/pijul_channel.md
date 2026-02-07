# Pijul Channel

The `pijul_channel` module shows the active channel of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                           | Description                                                                          |
| ------------------- | --------------------------------- | ------------------------------------------------------------------------------------ |
| `symbol`            | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | The style for the module.                                                            |
| `format`            | `'on [$symbol$channel]($style) '` | The format for the module.                                                           |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                    |
| `truncation_symbol` | `'…'`                             | The symbol used to indicate a branch name was truncated.                             |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |
