# Configuration

To get started configuring starship, create the following file: `~/.config/starship.toml`.

```shell
touch ~/.config/starship.toml
```

All configuration for starship is done in a [TOML](https://github.com/toml-lang/toml) file.
Here we will demonstrate a couple simple configuration examples.

```toml
# Replace the "➜" symbol in the prompt with "❯"
[char]           # The name of the module we are confguring is "char"
symbol = "❯"     # The "symbol" segment is being set to "❯"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

## Terminology

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of NodeJS that is currently installed on your computer, if your current directory is a NodeJS project.

**Segment**: Smaller sub-components that compose a module. For example, the "symbol" segment in the "nodejs" module contains the character that is shown before the version number (⬢ by default).

Here is the representation of the node module. In the following example, "symbol" and "version"
are segments within it. Every module also has a prefix and suffix that are the default terminal color.

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

## Planned Configuration

- [x] Overriding segments within a module
- [x] Disabling a module entirely
- [ ] Overriding the prefix and suffix of a module
- [ ] Prompt order
- [ ] Module and segment colors and text styling

## Modules

### Battery

The battery module shows how charged the device's battery is and its current charging status.
The module is only visible when the device's battery is below 10%.

| Variable             | Default | Description                                       |
| -------------------- | ------- | ------------------------------------------------- |
| `full_symbol`        | `"•"`   | The symbol shown when the battery is full.        |
| `charging_symbol`    | `"⇡"`   | The symbol shown when the battery is charging.    |
| `discharging_symbol` | `"⇣"`   | The symbol shown when the battery is discharging. |
| `disabled`           | `false` | Disables the `battery` module.                    |

#### Example

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```
