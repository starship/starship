# Azure

The `azure` module shows the current Azure Subscription. This is based on showing the name of the default subscription or the username, as defined in the `~/.azure/azureProfile.json` file.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Variable               | Default                                  | Description                                                                           |
| ---------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | The format for the Azure module to render.                                            |
| `symbol`               | `'󰠅 '`                                   | The symbol used in the format.                                                        |
| `style`                | `'blue bold'`                            | The style used in the format.                                                         |
| `disabled`             | `true`                                   | Disables the `azure` module.                                                          |
| `subscription_aliases` | `{}`                                     | Table of subscription name aliases to display in addition to Azure subscription name. |

### Examples

#### Display Subscription Name

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = '󰠅 '
style = 'blue bold'
```

#### Display Username

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "󰠅 "
style = "blue bold"
```

#### Display Subscription Name Alias

```toml
# ~/.config/starship.toml

[azure.subscription_aliases]
very-long-subscription-name = 'vlsn'
```
