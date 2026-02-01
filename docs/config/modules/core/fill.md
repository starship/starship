# Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are
present in a line they will split the space evenly between them. This is useful for aligning
other modules.

### Options

| Option     | Default        | Description                       |
| ---------- | -------------- | --------------------------------- |
| `symbol`   | `'.'`          | The symbol used to fill the line. |
| `style`    | `'bold black'` | The style for the module.         |
| `disabled` | `false`        | Disables the `fill` module        |

### Example

```toml
# ~/.config/starship.toml
format = 'AA $fill BB $fill CC'

[fill]
symbol = '-'
style = 'bold green'
```

Produces a prompt that looks like:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```
