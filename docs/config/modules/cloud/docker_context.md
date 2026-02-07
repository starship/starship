# Docker Context

The `docker_context` module shows the currently active
[Docker context](https://docs.docker.com/engine/context/working-with-contexts/)
if it's not set to `default` or `desktop-linux`, or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or
`DOCKER_CONTEXT` environment variables are set (as they are meant to override
the context in use).

### Options

| Option              | Default                                                                                      | Description                                                                       |
| ------------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                                                           | The format for the module.                                                        |
| `symbol`            | `'🐳 '`                                                                                      | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                                                       | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                                                         | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `['compose.yml', 'compose.yaml', 'docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                                                         | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `'blue bold'`                                                                                | The style for the module.                                                         |
| `disabled`          | `false`                                                                                      | Disables the `docker_context` module.                                             |

### Variables

| Variable | Example        | Description                          |
| -------- | -------------- | ------------------------------------ |
| context  | `test_context` | The current docker context           |
| symbol   |                | Mirrors the value of option `symbol` |
| style\*  |                | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```
