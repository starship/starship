# Migrando a v0.45.0

Starship v0.45.0 es un lanzamiento que contiene cambios de ruptura, en preparación para la gran v1.0.0. Hemos hecho algunos cambios importantes en la forma en que se realiza la configuración en el prompt, para permitir un mayor grado de personalización.

Esta guía pretende guiarle a través de los cambios de ruptura.

## `prompt_order` ha sido reemplazado por un `format` en el nivel raiz

Previo a v0.45.0, `prompt_order` aceptaría un arreglo de nombres de módulos en el orden en que deberían ser renderizados por Starship.

En su lugar, Starship v0.45.0 acepta un valor de `format`, permitiendo la personalización del indicador fuera de los propios módulos.

**Configuración de ejemplo pre-v0.45.0**

```toml
prompt_order = [
  "username",
  "hostname",
  "directory",
  "git_branch",
  "git_commit",
  "git_state",
  "git_status",
  "cmd_duration",
  "custom",
  "line_break",
  "jobs",
  "battery",
  "time",
  "character",
]
```

**Configuración de ejemplo v0.45.0**

```toml
format = """\
  $username\
  $hostname\
  $directory\
  $git_branch\
  $git_commit\
  $git_state\
  $git_status\
  $cmd_duration\
  $custom\
  $line_break\
  $jobs\
  $battery\
  $time\
  $character\
  """
```

## El módulo `prefix` y `suffix` han sido reemplazados por `format`

Anteriormente a v0.45.0, algunos módulos aceptarían `prefix` y/o `suffix` para estilizar la forma en que se renderizan los módulos.

En su lugar, Starship v0.45.0 acepta un valor `format`, lo que permite una mayor personalización de cómo se renderizan los módulos. En lugar de definir un prefijo y un sufijo para las variables basadas en contextos, las variables ahora pueden ser sustituidas dentro de una cadena de formato, que representa la salida del módulo.

**Configuración de ejemplo pre-v0.45.0**

```toml
[cmd_duration]
prefix = "tomó "
```

**Configuración de ejemplo v0.45.0**

```toml
[cmd_duration]
# $duration – La duración del comando (p. ej. "15s")
# $style    – El estilo por defecto del módulo (p. ej. "bold yellow")
format = "tomó [$duration]($style) "
```

### Módulos afectados

#### Carácter

| Propiedad eliminada     | Reemplazo        |
| ----------------------- | ---------------- |
| `symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Cambios a la configuración por defecto**

```diff
[character]
-- symbol = "❯"
-- error_symbol = "✖"
-- use_symbol_for_status = true
-- vicmd_symbol = "❮"
++ success_symbol = "[❯](bold green)"
++ error_symbol = "[❯](bold red)"
++ vicmd_symbol = "[❮](bold green)"
```

Anteriormente, la propiedad `use_symbol_for_status` fue usada para configurar el prompt para mostrar el `error_symbol` cuando el último comando resultó en un código de estado distinto de cero.

Con la liberación de v0.45.0, ahora siempre usamos las propiedades `error_symbol` después de códigos de estado diferentes de cero, unificando `use_symbol_for_status` y `error_symbol`.

Para configurar el prompt para usar la configuración anterior `use_symbol_for_status = true`, agrega lo siguiente a tu archivo de configuración:

```toml
[character]
error_symbol = "[✖](bold red)"
```

_Nota:_ El elemento `character` añade automáticamente un espacio después, así que a diferencia de las otras cadenas de `format`, no añadimos uno en los ejemplos anteriores.

#### Tiempo de ejecución

| Propiedad eliminada | Reemplazo |
| ------------------- | --------- |
| `prefix`            | `format`  |

**Cambios a la configuración por defecto**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Directory

| Propiedad eliminada | Reemplazo |
| ------------------- | --------- |
| `prefix`            | `format`  |

**Cambios a la configuración por defecto**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Variable de entorno

| Propiedad eliminada | Reemplazo |
| ------------------- | --------- |
| `prefix`            | `format`  |
| `suffix`            | `format`  |

**Cambios a la configuración por defecto**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git commit

| Propiedad eliminada | Reemplazo |
| ------------------- | --------- |
| `prefix`            | `format`  |
| `suffix`            | `format`  |

**Cambios a la configuración por defecto**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git status

| Propiedad eliminada | Reemplazo |
| ------------------- | --------- |
| `prefix`            | `format`  |
| `suffix`            | `format`  |
| `show_sync_count`   | `format`  |

**Cambios a la configuración por defecto**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Anteriormente, la propiedad `show_sync_count` fue usada para configurar el prompt para mostrar el número de confirmaciones que la rama estaba por delante o detrás de la rama remota.

Con el lanzamiento de v0.45.0, esto ha sido reemplazado por tres propiedades separadas, `ahead`, `behind`, y `diverged`.

Para configurar el prompt para usar la configuración anterior `show_sync_count = true`, establece lo siguiente en tu archivo de configuración:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

| Propiedad eliminada | Reemplazo |
| ------------------- | --------- |
| `prefix`            | `format`  |
| `suffix`            | `format`  |

**Cambios a la configuración por defecto**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| Propiedad eliminada | Reemplazo |
| ------------------- | --------- |
| `label`             | `format`  |
| `prefix`            | `format`  |
| `suffix`            | `format`  |

**Cambios a la configuración por defecto**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Time

| Propiedad eliminada | Reemplazo     |
| ------------------- | ------------- |
| `format`            | `time_format` |

**Cambios a la configuración por defecto**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Comandos Personalizados

| Propiedad eliminada | Reemplazo |
| ------------------- | --------- |
| `prefix`            | `format`  |
| `suffix`            | `format`  |

**Cambios a la configuración por defecto**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
