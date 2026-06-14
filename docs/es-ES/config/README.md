# Configuración

To get started configuring starship, create the following file: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

All configuration for starship is done in this [TOML](https://github.com/toml-lang/toml) file:

```toml
# Obtener terminaciones del editor basado en el esquema de configuración
"$schema" = 'https://starship.rs/config-schema. son'

# Inserta una línea en blanco entre las instrucciones del intérprete de comandos
add_newline = true

# Reemplaza el símbolo '❯' en el prompt con ''➜'
[character] # El nombre del módulo que estamos configurando es 'character'
success_symbol = '[➜](bold green)' # El segmento 'success_symbol' se está configurando en '➜'  con el color 'bold green'

# Desactiva el módulo del paquete, ocultándolo del prompt completamente
[package]
disabled = true
```

### Configurar ubicación del archivo

You can change default configuration file location with `STARSHIP_CONFIG` environment variable:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Registros

By default starship logs warnings and errors into a file named `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, where the session key is corresponding to an instance of your terminal.
This, however can be changed using the `STARSHIP_CACHE` environment variable:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminología

**Module**: A component in the prompt giving information based on contextual information from your OS. Por ejemplo, el módulo "nodejs" muestra la versión de NodeJS que tienes actualmente instalada en tu ordenador, si el directorio actual es un proyecto NodeJS.

**Variable**: Smaller sub-components that contain information provided by the module. Por ejemplo, la variable "version" en el módulo "nodejs" contiene la versión actual de NodeJS.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Cadenas de Texto

In TOML syntax, [text values](https://toml.io/en/v1.0.0#string) are declared with `'`, `"`, `'''`, or `"""`.

Los siguientes símbolos de sintaxis de Starship tienen un uso especial en una cadena de formato y deben escaparse para mostrarse como ese carácter: `$ [ ] ( )`.

| Simbol | Tipo                       | Notas                                                                    |
| ------ | -------------------------- | ------------------------------------------------------------------------ |
| `'`    | cadena literal             | menos escapes                                                            |
| `"`    | cadena                     | mas escapes                                                              |
| `'''`  | cadena literal multilineas | menos escapes                                                            |
| `"""`  | Cadena multilínea          | Más escapes, las nuevas líneas en las declaraciones pueden ser ignoradas |

Por ejemplo:

```toml
# cadena literal
format = '☺\☻ '

# cadena estándar
format = "☺\\☻ "

# escapando símbolos de Starship
format = '\[\$\] '
```

Al usar saltos de línea, se pueden utilizar declaraciones multilínea.
For example, if you want to print a `$` symbol on a new line, the following values for `format` are equivalent:

```toml
# con cadena literal
format = '''

\$'''

# con cadena básica multilínea
format = """

\\$"""
```

En las cadenas básicas multilínea, se pueden usar saltos de línea para dar formato sin que estén presentes en el valor, escapándolos.

```toml
format = """
line1\
line1\
line1
line2\
line2\
line2
"""
```

### Cadenas de Formato

Es el formato con el que un módulo imprime todas sus variables.
Most modules have an entry called `format` that configures the display format of the module.
Se puede utilizar textos, variables y grupos de texto.

#### Variable

A variable contains a `$` symbol followed by the name of the variable.
El nombre de una variable solamente puede contener letras, números y `_`.

Por ejemplo:

- `'$version'` is a format string with a variable named `version`.
- `'$git_branch$git_commit'` is a format string with two variables named `git_branch` and `git_commit`.
- `'$git_branch $git_commit'` has the two variables separated with a space.

#### Grupo de Texto

Un grupo de texto se compone de dos partes diferentes.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings).
Se puede agregar textos, variables, o incluso grupos de texto anidados.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). Esto se puede utilizar para diseñar la primera parte.

Por ejemplo:

- `'[on](red bold)'` will print a string `on` with bold text colored red.
- `'[⌘ $version](bold green)'` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `'[a [b](red) c](green)'` will print `a b c` with `b` red, and `a` and `c` green.

#### Cadenas de Estilo

La mayoría de los módulos de starship permiten configurar sus estilos de visualización. This is done with an entry (usually called `style`) which is a string specifying the configuration. A continuación mostramos algunos ejemplos de cadenas de estilo junto con su funcionalidad. For details on the full syntax, consult the [advanced config guide](../advanced-config/).

- `'fg:green bg:blue'` sets green text on a blue background
- `'bg:blue fg:bright-green'` sets bright green text on a blue background
- `'bold fg:27'` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `'underline bg:#bf5700'` sets underlined text on a burnt orange background
- `'bold italic fg:purple'` sets bold italic purple text
- `''` explicitly disables all styling

Nótese que el estilo es similar a como se controlaría por el emulador de terminal. Por ejemplo, algunos emuladores de terminal harán los colores más brillantes en lugar de más gruesos, y algunos temas de colores usan los mismos valores para texto normal y colores brillantes. Además, para mostrar textos en cursiva tu terminal debe tener soporte para hacerlo.

#### Cadenas de Formato Condicional

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

Por ejemplo:

- `'(@$region)'` will show nothing if the variable `region` is `None` or empty string, otherwise `@` followed by the value of region.
- `'(some text)'` will always show nothing since there are no variables wrapped in the braces.
- When `$combined` is a shortcut for `\[$a$b\]`, `'($combined)'` will show nothing only if `$a` and `$b` are both `None`.
  This works the same as `'(\[$a$b\] )'`.

### Coincidencia negativa

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. Estas toman
listas de cadenas de texto para que coincidan o no coincidan. Las opciones "negativas", aquellas que no deben coincidir, se indican con un carácter '!' al principio. The presence of _any_ negative indicator in the directory
will result in the module not being matched.

Las extensiones coinciden tanto con los caracteres después del último punto en un nombre de archivo, como con los caracteres
después del primer punto en un nombre de archivo. For example, `foo.bar.tar.gz` will be matched
against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Los archivos cuyo nombre comienza con un punto
no se consideran extensiones en absoluto.

Para ver cómo funciona esto en la práctica, puede hacer coincidir con archivos TypeScript, pero no con archivos MPEG Transport Stream, así:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Prompt

Esta es la lista de opciones de configuración del prompt.

### Opciones

| Opción            | Predeterminado                 | Descripción                                                                                                                                                                                                                                            |
| ----------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `format`          | [link](#default-prompt-format) | Configura el formato del prompt.                                                                                                                                                                                                       |
| `right_format`    | `''`                           | See [Enable Right Prompt](../advanced-config/#enable-right-prompt)                                                                                                                                                                                     |
| `scan_timeout`    | `30`                           | Tiempo de espera tras el que Starship escanea archivos (en milisegundos).                                                                                                                                           |
| `command_timeout` | `500`                          | Tiempo de espera para los comandos ejecutados por Starship (en milisegundos).                                                                                                                                       |
| `add_newline`     | `true`                         | Inserta un línea en blanco entre las instrucciones del intérprete de comandos.                                                                                                                                                         |
| `palette`         | `''`                           | Sets which color palette from `palettes` to use.                                                                                                                                                                                       |
| `palettes`        | `{}`                           | Collection of color palettes that assign [colors](../advanced-config/#style-strings) to user-defined names. Tenga en cuenta que las paletas de colores no pueden hacer referencia a sus propias definiciones de color. |
| `follow_symlinks` | `true`                         | Sigue los enlaces simbólicos (symlinks) para comprobar si son directorios; se utiliza en módulos como git.                                                                                                          |

> [!TIP]
> If you have symlinks to networked filesystems, consider setting
> `follow_symlinks` to `false`.

### Ejemplo

```toml
# ~/.config/starship.toml

# Use custom format
format = '''
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) '''

# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10

# Disable the blank line at the start of the prompt
add_newline = false

# Set 'foo' as custom color palette
palette = 'foo'

# Define custom colors
[palettes.foo]
# Overwrite existing color
blue = '21'
# Define new color
mustard = '#af8700'
```

### Formato por Defecto del Prompt

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. El valor predeterminado es el siguiente:

```toml
format = '$all'

# Which is equivalent to
format = """
$username\
$hostname\
$localip\
$shlvl\
$singularity\
$kubernetes\
$nats\
$directory\
$vcsh\
$fossil_branch\
$fossil_metrics\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$hg_state\
$pijul_channel\
$docker_context\
$package\
$bun\
$c\
$cmake\
$cobol\
$cpp\
$daml\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$fennel\
$fortran\
$gleam\
$golang\
$gradle\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$lua\
$maven\
$mojo\
$nim\
$nodejs\
$ocaml\
$odin\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$quarto\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$solidity\
$swift\
$terraform\
$typst\
$vlang\
$vagrant\
$xmake\
$zig\
$buf\
$guix_shell\
$nix_shell\
$conda\
$pixi\
$meson\
$spack\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
$direnv\
$env_var\
$mise\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$container\
$netns\
$os\
$shell\
$character"""
```

If you just want to extend the default format, you can use `$all`;
modules you explicitly add to the format will not be duplicated. Ej.

```toml
# Mover el directorio a la segunda línea Código: format = '$all$directory$character'
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials.
The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process`, `sso_start_url`, or `sso_session` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice.
If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile
is read from the `AWS_VAULT` env var and the credentials expiration date
is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile
is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile
is read from the `AWSUME_PROFILE` env var and the credentials expiration
date is read from the `AWSUME_EXPIRATION` env var.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials`
falls back to the `x_security_token_expires` key.

When using [aws-sso-cli](https://github.com/synfinatic/aws-sso-cli) the profile
is read from the `AWS_SSO_PROFILE` env var.

### Opciones

| Opción              | Predeterminado                                                        | Descripción                                                                                                                 |
| ------------------- | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | El formato del módulo.                                                                                      |
| `simbol`            | `'☁️ '`                                                               | El símbolo que se muestra antes del perfil de AWS.                                                          |
| `region_aliases`    | `{}`                                                                  | Tabla de alias de región para mostrar además del nombre AWS.                                                |
| `profile_aliases`   | `{}`                                                                  | Tabla de alias de región para mostrar además del nombre AWS.                                                |
| `style`             | `'bold yellow'`                                                       | El estilo del módulo.                                                                                       |
| `expiration_symbol` | `'X'`                                                                 | El símbolo mostrado cuando las credenciales temporales han caducado.                                        |
| `disabled`          | `false`                                                               | Disables the `AWS` module.                                                                                  |
| `force_display`     | `false`                                                               | If `true` displays info even if `credentials`, `credential_process` or `sso_start_url` have not been setup. |

### Variables

| Variable | Ejemplo          | Descripción                                              |
| -------- | ---------------- | -------------------------------------------------------- |
| region   | `ap-northeast-1` | La región actual de AWS                                  |
| profile  | `astronauts`     | El perfil actual de AWS                                  |
| duration | `2h27m20s`       | La duración de la validez de las credenciales temporales |
| symbol   |                  | Refleja el valor de la opción `symbol`                   |
| style\*  |                  | Refleja el valor de la opción `style`                    |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplos

#### Mostrar todo

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
```

#### Mostrar región

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$region]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
```

#### Mostrar perfil

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$profile]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

The `azure` module shows the current Azure Subscription. This is based on showing the name of the default subscription or the username, as defined in the `~/.azure/azureProfile.json` file.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Variable               | Predeterminado                           | Descripción                                                                                                         |
| ---------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | El formato para renderizar el módulo Azure.                                                         |
| `simbol`               | `'󰠅 '`                                  | El símbolo utilizado en el formato.                                                                 |
| `style`                | `'blue bold'`                            | El estilo utilizado en el formato.                                                                  |
| `disabled`             | `true`                                   | Disables the `azure` module.                                                                        |
| `subscription_aliases` | `{}`                                     | Tabla de alias de nombres de suscripción para mostrar además del nombre de la suscripción de Azure. |

### Ejemplos

#### Mostrar el nombre de la suscripción

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = '󰠅 '
style = 'blue bold'
```

#### Mostrar el nombre de usuario

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "󰠅 "
style = "blue bold"
```

#### Mostrar alias del nombre de la suscripción

```toml
# ~/.config/starship.toml

[azure.subscription_aliases]
very-long-subscription-name = 'vlsn'
```

## Battery

The `battery` module shows how charged the device's battery is and its current charging status.
El módulo solamente es visible cuando la batería del dispositivo está por debajo del 10%.

### Opciones

| Opción               | Predeterminado                    | Descripción                                                                              |
| -------------------- | --------------------------------- | ---------------------------------------------------------------------------------------- |
| `full_symbol`        | `'󰁹 '`                           | Se muestra cuando la batería está cargada.                               |
| `charging_symbol`    | `'󰂄 '`                           | Se muestra cuando la batería se está cargando.                           |
| `discharging_symbol` | `'󰂃 '`                           | Se muestra cuando la batería se está descargando.                        |
| `unknown_symbol`     | `'󰂑 '`                           | El símbolo que se muestra cuando el estado de la batería es desconocido. |
| `empty_symbol`       | `'󰂎 '`                           | El símbolo que se muestra cuando el estado de la batería está vacío.     |
| `format`             | `'[$symbol$percentage]($style) '` | El formato del módulo.                                                   |
| `display`            | [link](#battery-display)          | Define cuándo mostrar el indicador y el estilo.                          |
| `disabled`           | `false`                           | Disables the `battery` module.                                           |

### Ejemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Indicador de batería

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style).
If no `display` is provided. El valor predeterminado es el siguiente:

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Opciones

The `display` option is an array of the following table.

| Opción               | Predeterminado | Descripción                                                                                                                       |
| -------------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`           | El umbral para la opción de visualización.                                                                        |
| `style`              | `'red bold'`   | El estilo usado cuando si la opción <0>display</0> está activa. |
| `charging_symbol`    |                | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.            |
| `discharging_symbol` |                | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option.         |

#### Ejemplo

```toml
[[battery.display]] # estilo 'bold red' y símbolo de descarga cuando la capacidad está entre 0% y 10%
threshold = 10
style = 'bold red'

[[battery.display]] # estilo 'bold yellow' y símbolo 💦 cuando la capacidad está entre 10% y 30%
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦 '

# cuando la capacidad es superior al 30%, el indicador de batería no se mostrará
```

## Buf

The `buf` module shows the currently installed version of [Buf](https://buf.build). By default, the module is shown if the current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### Opciones

| Opción              | Predeterminado                                  | Descripción                                                          |
| ------------------- | ----------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | The format for the `buf` module.                     |
| `version_format`    | `'v${raw}'`                                     | El formato de versión.                               |
| `simbol`            | `'🐃 '`                                         | El símbolo usado antes de mostrar la versión de Buf. |
| `detect_extensions` | `[]`                                            | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                            | Qué carpetas deberían activar este módulo.           |
| `style`             | `'bold blue'`                                   | El estilo del módulo.                                |
| `disabled`          | `false`                                         | Disables the `elixir` module.                        |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| `version` | `v1.0.0` | The version of `buf`                   |
| `simbol`  |          | Refleja el valor de la opción `symbol` |
| `style`\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime.
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `bun.lock` file
- The current directory contains a `bun.lockb` file
- The current directory contains a `bunfig.toml` file

### Opciones

| Opción              | Predeterminado                             | Descripción                                                                                                      |
| ------------------- | ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                                | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🥟 '`                                    | A format string representing the symbol of Bun.                                                  |
| `detect_extensions` | `[]`                                       | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                       | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold red'`                               | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                    | Disables the `bun` module.                                                                       |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v0.1.4` | The version of `bun`                   |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

#### Customize the format

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

The `c` module shows some information about your C compiler. By default
the module will be shown if the current directory contains a `.c` or `.h`
file.

### Opciones

| Opción              | Predeterminado                                                                | Descripción                                                                                                      |
| ------------------- | ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | La cadena de formato para el módulo.                                                             |
| `version_format`    | `'v${raw}'`                                                                   | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'C '`                                                                        | El símbolo usado antes de mostrar los detalles del compilador                                                    |
| `detect_extensions` | `['c', 'h']`                                                                  | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                                                          | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                                                          | Qué carpetas deberían activar este módulo.                                                       |
| `commands`          | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | Cómo detectar cuál compilador es                                                                                 |
| `style`             | `'bold 149'`                                                                  | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                                                       | Disables the `c` module.                                                                         |

### Variables

| Variable | Ejemplo                                | Descripción                            |
| -------- | -------------------------------------- | -------------------------------------- |
| name     | clang                                  | El nombre del compilador               |
| version  | 13.0.0 | La versión del compilador              |
| symbol   |                                        | Refleja el valor de la opción `symbol` |
| style    |                                        | Refleja el valor de la opción `style`  |

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship intentará ejecutar cada comando hasta que obtenga un resultado en STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Ejemplo

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## CPP

The `cpp` module shows some information about your `C++` compiler. By default,
the module will be shown if the current directory contains a `.cpp`, `.hpp`, or other `C++`-related files.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción              | Predeterminado                                                                   | Descripción                                                                                                      |
| ------------------- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | La cadena de formato para el módulo.                                                             |
| `version_format`    | `'v${raw}'`                                                                      | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'C++ '`                                                                         | El símbolo usado antes de mostrar los detalles del compilador                                                    |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                                                             | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                                                             | Qué carpetas deberían activar este módulo.                                                       |
| `commands`          | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | Cómo detectar cuál compilador es                                                                                 |
| `style`             | `'bold 149'`                                                                     | El estilo del módulo.                                                                            |
| `disabled`          | `true`                                                                           | Disables the `cpp` module.                                                                       |

### Variables

| Variable | Ejemplo                                | Descripción                            |
| -------- | -------------------------------------- | -------------------------------------- |
| name     | clang++                                | El nombre del compilador               |
| version  | 13.0.0 | La versión del compilador              |
| symbol   |                                        | Refleja el valor de la opción `symbol` |
| style    |                                        | Refleja el valor de la opción `style`  |

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycpp', '--version']`. Starship intentará ejecutar cada comando hasta que obtenga un resultado en STDOUT.

If a C++ compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Ejemplo

```toml
# ~/.config/starship.toml

[cpp]
disabled = false
format = 'via [$name $version]($style)'
```

## Carácter

The `character` module shows a character (usually an arrow) beside where the text
is entered in your terminal.

El caracter te dirá si el último comando fue exitoso o no. Se puede hacer de dos maneras:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

Por defecto sólo cambia el color. If you also want to change its shape take a
look at [this example](#with-custom-error-shape).

> [!WARNING]
> `vimcmd_symbol` is only supported in cmd, fish and zsh.
> `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol`
> are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

### Opciones

| Opción                      | Predeterminado       | Descripción                                                                                                             |
| --------------------------- | -------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | La cadena de formato usada antes de la entrada de texto.                                                |
| `success_symbol`            | `'[❯](bold green)'`  | La cadena de formato usada antes de la entrada de texto si el comando anterior tuvo éxito.              |
| `error_symbol`              | `'[❯](bold red)'`    | La cadena de formato usada antes de la entrada de texto si el comando anterior falló.                   |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | El cadena de formato antes de la entrada de texto si el intérprete de comandos está en modo vim normal. |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode.                 |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.                       |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.                        |
| `disabled`                  | `false`              | Disables the `character` module.                                                                        |

### Variables

| Variable | Ejemplo | Descripción                                                                                                              |
| -------- | ------- | ------------------------------------------------------------------------------------------------------------------------ |
| symbol   |         | A mirror of either `success_symbol`, `error_symbol`, `vimcmd_symbol` or `vimcmd_replace_one_symbol` etc. |

### Ejemplos

#### Con formato de error personalizado

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[✗](bold red) '
```

#### Sin formato de error personalizado

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[➜](bold red) '
```

#### Con formato de vim personalizado

```toml
# ~/.config/starship.toml

[character]
vimcmd_symbol = '[V](bold green) '
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). Por defecto el módulo se activará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### Opciones

| Opción              | Predeterminado                         | Descripción                                                                                                      |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`   | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                            | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'△ '`                                 | El símbolo usado antes de la versión de cmake.                                                   |
| `detect_extensions` | `[]`                                   | Qué extensiones deben activar este módulo                                                                        |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | Qué nombres de archivo deben activar este módulo                                                                 |
| `detect_folders`    | `[]`                                   | Qué carpetas deben activar este módulo                                                                           |
| `style`             | `'bold blue'`                          | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                | Disables the `cmake` module.                                                                     |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| version  | `v3.17.3` | La versión de cmake                    |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL.
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `simbol`            | `'⚙️ '`                              | El símbolo usado antes de mostrar la versión de COBOL.                                           |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | El estilo del módulo.                                                                            |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `disabled`          | `false`                              | Disables the `cobol` module.                                                                     |

### Variables

| Variable | Ejemplo    | Descripción                            |
| -------- | ---------- | -------------------------------------- |
| version  | `v3.1.2.0` | The version of `cobol`                 |
| symbol   |            | Refleja el valor de la opción `symbol` |
| style\*  |            | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## Tiempo de ejecución

The `cmd_duration` module shows how long the last command took to execute.
The module will be shown only if the command took longer than two seconds, or
the `min_time` config value, if it exists.

> [!WARNING]
> Do not hook the DEBUG trap in Bash
>
> If you are running Starship in `bash`, do not hook the `DEBUG` trap after running
> `eval $(starship init $0)`, or this module **will** break.

Bash users who need preexec-like functionality can use
[rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec).
Simply define the arrays `preexec_functions` and `precmd_functions` before
running `eval $(starship init $0)`, and then proceed as normal.

### Opciones

| Opción                 | Predeterminado                | Descripción                                                                                                                                                                                                                                                                     |
| ---------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Duración más corta para mostrar el tiempo (en milisegundos).                                                                                                                                                                                 |
| `show_milliseconds`    | `false`                       | Mostrar milisegundos además de segundos para la duración.                                                                                                                                                                                                       |
| `format`               | `'took [$duration]($style) '` | El formato del módulo.                                                                                                                                                                                                                                          |
| `style`                | `'bold yellow'`               | El estilo del módulo.                                                                                                                                                                                                                                           |
| `disabled`             | `false`                       | Disables the `cmd_duration` module.                                                                                                                                                                                                                             |
| `show_notifications`   | `false`                       | Muestra notificaciones de escritorio cuando se complete el comando.                                                                                                                                                                                             |
| `min_time_to_notify`   | `45_000`                      | Duración más corta para la notificación (en milisegundos).                                                                                                                                                                                   |
| `notification_timeout` |                               | Duración para mostrar la notificación (en milisegundos). Si no se establece, el tiempo de espera para notificar será determinado por el demonio. No todos los demonios de notificaciones honran esta opción. |

### Variables

| Variable | Ejemplo  | Descripción                                |
| -------- | -------- | ------------------------------------------ |
| duration | `16m40s` | El tiempo que tardó en ejecutar el comando |
| style\*  |          | Refleja el valor de la opción `style`      |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

> [!TIP]
> This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.
> If you use [pixi](https://pixi.sh), you can disable pixi's prompt modifier by running `pixi config set shell.change-ps1 false`.

### Opciones

| Opción              | Predeterminado                         | Descripción                                                                                                                                                                                                                                                 |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `simbol`            | `'🅒 '`                                | El símbolo usado antes del nombre del entorno.                                                                                                                                                                                              |
| `style`             | `'bold green'`                         | El estilo del módulo.                                                                                                                                                                                                                       |
| `format`            | `'via [$symbol$environment]($style) '` | El formato del módulo.                                                                                                                                                                                                                      |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                                                  |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | Qué variable(s) de entorno deben activar este módulo. If it's a pixi environment, this module is not being triggered by default.                                                                         |
| `disabled`          | `false`                                | Disables the `conda` module.                                                                                                                                                                                                                |

### Variables

| Variable    | Ejemplo      | Descripción                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | El entorno Conda actual                |
| symbol      |              | Refleja el valor de la opción `symbol` |
| style\*     |              | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Contenedor

The `container` module displays a symbol and container name, if inside a container.

### Opciones

| Opción     | Predeterminado                     | Descripción                                                      |
| ---------- | ---------------------------------- | ---------------------------------------------------------------- |
| `simbol`   | `'⬢'`                              | El símbolo mostrado, cuando se encuentra dentro de un contenedor |
| `style`    | `'bold red dimmed'`                | El estilo del módulo.                            |
| `format`   | `'[$symbol \[$name\]]($style) '` | El formato del módulo.                           |
| `disabled` | `false`                            | Disables the `container` module.                 |

### Variables

| Variable | Ejemplo             | Descripción                            |
| -------- | ------------------- | -------------------------------------- |
| name     | `fedora-toolbox:35` | El nombre del contenedor               |
| symbol   |                     | Refleja el valor de la opción `symbol` |
| style\*  |                     | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `simbol`            | `'🔮 '`                              | El símbolo usado antes de mostrar la versión del crystal.                                        |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | El estilo del módulo.                                                                            |
| `detect_extensions` | `['cr']`                             | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['shard.yml']`                      | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `disabled`          | `false`                              | Disables the `crystal` module.                                                                   |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| version  | `v0.32.1` | The version of `crystal`               |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers)
SDK version when you are in the root directory of your Daml project. The `sdk-version` in
the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION`
environment variable.
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `daml.yaml` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'Λ '`                               | A format string representing the symbol of Daml                                                                  |
| `style`             | `'bold cyan'`                        | El estilo del módulo.                                                                            |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['daml.yaml']`                      | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `disabled`          | `false`                              | Disables the `daml` module.                                                                      |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.2.0` | The version of `daml`                  |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml`, `pubspec.yml` or `pubspec.lock` file

### Opciones

| Opción              | Predeterminado                                    | Descripción                                                                                                      |
| ------------------- | ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`              | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                                       | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🎯 '`                                           | Una cadena de formato que representa el símbolo de Dart                                                          |
| `detect_extensions` | `['dart']`                                        | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `['.dart_tool']`                                  | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold blue'`                                     | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                           | Disables the `dart` module.                                                                      |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.8.4` | The version of `dart`                  |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Opciones

| Opción              | Predeterminado                                                                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                                                                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🦕 '`                                                                              | Una cadena de formato que representa el símbolo de Deno                                                          |
| `detect_extensions` | `[]`                                                                                 | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                 | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'green bold'`                                                                       | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                                                              | Disables the `deno` module.                                                                      |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.8.3` | The version of `deno`                  |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Directory

The `directory` module shows the path to your current directory, truncated to
three parent folders. Tu directorio se truncará a la raíz del repositorio git en el que te encuentres.

When using the `fish_style_pwd_dir_length` option, instead of hiding the path that is
truncated, you will see a shortened name of each directory based on the number
you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root,
and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before
it would have been `nixpkgs/pkgs`.

### Opciones

| Opción                   | Predeterminado                                                                                                               | Descripción                                                                                                                                |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length`      | `3`                                                                                                                          | El número de carpetas a las que se debe truncar el directorio actual.                                                      |
| `truncate_to_repo`       | `true`                                                                                                                       | Truncar o no hasta la raíz del repositorio git en el que se esté.                                                          |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | El formato del módulo.                                                                                                     |
| `style`                  | `'bold cyan'`                                                                                                                | El estilo del módulo.                                                                                                      |
| `disabled`               | `false`                                                                                                                      | Disables the `directory` module.                                                                                           |
| `read_only`              | `'🔒'`                                                                                                                       | El símbolo que indica el directorio actual es de sólo lectura.                                                             |
| `read_only_style`        | `'red'`                                                                                                                      | El estilo para el símbolo de sólo lectura.                                                                                 |
| `truncation_symbol`      | `''`                                                                                                                         | El símbolo a prefijar a las rutas truncadas. eg: '…/'                                                      |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. The default value is equivalent to `style`. |
| `repo_root_style`        |                                                                                                                              | El estilo para la raíz del repositorio de git. The default value is equivalent to `style`.                 |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                                   |
| `home_symbol`            | `'~'`                                                                                                                        | El símbolo que indica el directorio de inicio.                                                                             |
| `use_os_path_sep`        | `true`                                                                                                                       | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows)   |

<details><summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Opción avanzada             | Predeterminado | Descripción                                                                                                                                                                                            |
| --------------------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `substitutions`             |                | An Array or table of substitutions to be made to the path.                                                                                                                             |
| `fish_style_pwd_dir_length` | `0`            | El número de caracteres a usar al aplicar la lógica de ruta pwd del intérprete de comandos de Fish.                                                                                    |
| `use_logical_path`          | `true`         | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network
prefixes or development directories of Java. Ten en cuenta que esto desactivará el estilo PWD de fish. It takes an array of the following
key/value pairs:

| Value   | Tipo    | Descripción                                             |
| ------- | ------- | ------------------------------------------------------- |
| `from`  | String  | The value to substitute                                 |
| `to`    | String  | The replacement for that value, if found                |
| `regex` | Boolean | (Optional) Whether `from` is a regex |

By using `regex = true`, you can use [Rust's regular expressions](https://docs.rs/regex/latest/regex/#syntax) in `from`.
For instance you can replace every slash except the first with the following:

```toml
substitutions = [
  { from = "^/", to = "<root>/", regex = true },
  { from = "/", to = " | " },
  { from = "^<root>", to = "/", regex = true },
]
```

This will replace `/var/log` to `/ | var | log`.

The old syntax still works, although it doesn't support regular expressions:

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero,
the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path
`/built/this/city/on/rock/and/roll`, which would normally be displayed as `rock/and/roll`, would be displayed as
`/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with
a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable | Ejemplo               | Descripción                           |
| -------- | --------------------- | ------------------------------------- |
| ruta     | `'D:/Projects'`       | La ruta del directorio actual         |
| style\*  | `'black bold dimmed'` | Refleja el valor de la opción `style` |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

<details><summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| Variable                                                   | Ejemplo               | Descripción                                         |
| ---------------------------------------------------------- | --------------------- | --------------------------------------------------- |
| before_root_path | `'/path/to/home/'`    | La ruta antes de la ruta del directorio raíz de git |
| repo_root                             | `'git_repo'`          | El nombre del directorio raíz de git                |
| ruta                                                       | `'/src/lib'`          | La ruta restante                                    |
| style                                                      | `'black bold dimmed'` | Refleja el valor de la opción `style`               |
| repo_root_style  | `'underline white'`   | Estilo para el nombre del directorio raíz de git    |

</details>

### Ejemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Direnv

The `direnv` module shows the status of the current rc file if one is present. The status includes the path to the rc file, whether it is loaded, and whether it has been allowed by `direnv`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción              | Predeterminado                         | Descripción                                                                       |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | El formato del módulo.                                            |
| `simbol`            | `'direnv '`                            | The symbol used before displaying the direnv context.             |
| `style`             | `'bold orange'`                        | El estilo del módulo.                                             |
| `disabled`          | `true`                                 | Disables the `direnv` module.                                     |
| `detect_extensions` | `[]`                                   | Qué extensiones deberían activar este módulo.                     |
| `detect_files`      | `['.envrc']`                           | Qué nombres de archivo deberían activar este módulo.              |
| `detect_folders`    | `[]`                                   | Qué carpetas deberían activar este módulo.                        |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Qué variables de entorno deben activar este módulo.               |
| `allowed_msg`       | `'allowed'`                            | The message displayed when an rc file is allowed.                 |
| `not_allowed_msg`   | `'no permitido'`                       | El mensaje que se muestra cuando un archivo rc no está permitido. |
| `denied_msg`        | `'denied'`                             | The message displayed when an rc file is denied.                  |
| `loaded_msg`        | `'loaded'`                             | The message displayed when an rc file is loaded.                  |
| `unloaded_msg`      | `'not loaded'`                         | The message displayed when an rc file is not loaded.              |

### Variables

| Variable                     | Ejemplo             | Descripción                                             |
| ---------------------------- | ------------------- | ------------------------------------------------------- |
| cargado                      | `loaded`            | Whether the current rc file is loaded.  |
| allowed                      | `denied`            | Whether the current rc file is allowed. |
| rc_path | `/home/test/.envrc` | The current rc file path.               |
| symbol                       |                     | Refleja el valor de la opción `symbol`. |
| style\*                      | `red bold`          | Refleja el valor de la opción `style`.  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Contexto de Docker

The `docker_context` module shows the currently active
[Docker context](https://docs.docker.com/engine/context/working-with-contexts/)
if it's not set to `default` or `desktop-linux`, or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or
`DOCKER_CONTEXT` environment variables are set (as they are meant to override
the context in use).

### Opciones

| Opción              | Predeterminado                                                                               | Descripción                                                                                                          |
| ------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                                                           | El formato del módulo.                                                                               |
| `simbol`            | `'🐳 '`                                                                                      | El símbolo usado antes de mostrar el contexto de Docker.                                             |
| `only_with_files`   | `true`                                                                                       | Mostrar solo cuando haya una coincidencia                                                                            |
| `detect_extensions` | `[]`                                                                                         | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `['compose.yml', 'compose.yaml', 'docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                                                         | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `'blue bold'`                                                                                | El estilo del módulo.                                                                                |
| `disabled`          | `false`                                                                                      | Disables the `docker_context` module.                                                                |

### Variables

| Variable | Ejemplo        | Descripción                            |
| -------- | -------------- | -------------------------------------- |
| contexto | `test_context` | El contexto actual de docker           |
| symbol   |                | Refleja el valor de la opción `symbol` |
| style\*  |                | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. Si el SDK ha sido anclado en el directorio actual, se mostrará la versión fijada. De lo contrario, el módulo muestra la última versión instalada del SDK.

Por defecto, este módulo solo se mostrará en tu prompt cuando uno o más de los siguientes archivos estén presentes en el directorio actual:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

También necesitarás tener instalado el SDK de .NET Core para poder usarlo correctamente.

Internamente, este módulo utiliza su propio mecanismo para la detección de versiones. Typically it is twice as fast
as running `dotnet --version`, but it may show an incorrect version if your .NET project has an
unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by
setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker
(<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>)
when there is a `.csproj` file in the current directory.

### Opciones

| Opción              | Predeterminado                                                                                          | Descripción                                                                                                      |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                          | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                                                                                             | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'.NET '`                                                                                               | El símbolo usado antes de mostrar la version de dotnet.                                          |
| `heuristic`         | `true`                                                                                                  | Usa una detección de versiones más rápida para mantener la nave espacial veloz.                  |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                                    | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold blue'`                                                                                           | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                                                                    |

### Variables

| Variable | Ejemplo          | Descripción                                                     |
| -------- | ---------------- | --------------------------------------------------------------- |
| version  | `v3.1.201`       | The version of `dotnet` sdk                                     |
| tfm      | `netstandard2.0` | El Target Framework Moniker al que se dirige el proyecto actual |
| symbol   |                  | Refleja el valor de la opción `symbol`                          |
| style\*  |                  | Refleja el valor de la opción `style`                           |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `mix.exs` file.

### Opciones

| Opción              | Predeterminado                                              | Descripción                                                                                                      |
| ------------------- | ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | El formato para el módulo de elixir.                                                             |
| `version_format`    | `'v${raw}'`                                                 | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'💧 '`                                                     | El símbolo usado antes de mostrar la versión de Elixir/Erlang.                                   |
| `detect_extensions` | `[]`                                                        | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['mix.exs']`                                               | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                                        | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold purple'`                                             | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                                                    |

### Variables

| Variable                         | Ejemplo | Descripción                            |
| -------------------------------- | ------- | -------------------------------------- |
| version                          | `v1.10` | The version of `elixir`                |
| otp_version |         | The otp version of `elixir`            |
| symbol                           |         | Refleja el valor de la opción `symbol` |
| style\*                          |         | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains `*.elm` files

### Opciones

| Opción              | Predeterminado                                     | Descripción                                                                                                      |
| ------------------- | -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`               | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                                        | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🌳 '`                                            | Una cadena de formato que representa el símbolo de Elm.                                          |
| `detect_extensions` | `['elm']`                                          | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `['elm-stuff']`                                    | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'cyan bold'`                                      | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                                       |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| version  | `v0.19.1` | The version of `elm`                   |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Variable de entorno

The `env_var` module displays the current value of a selected environment variables.
El módulo se mostrará sólo si se cumplen cualquiera de las siguientes condiciones:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

> [!TIP]
> The order in which env_var modules are shown can be individually set by including
> `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `env_var` module will simply show all env_var modules in the order they were defined.

> [!TIP]
> Multiple environmental variables can be displayed by using a `.`. (see example)
> If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.
>
> Ejemplo: la siguiente configuración mostrará el valor de la variable de entorno USER
>
> ```toml
> # ~/.config/starship.toml
>
> [env_var.USER]
> default = 'unknown user'
> ```

### Opciones

| Opción           | Predeterminado                        | Descripción                                                                                            |
| ---------------- | ------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `simbol`         | `""`                                  | El símbolo usado antes de mostrar el valor de la variable.                             |
| `variable`       |                                       | La variable de entorno a mostrar.                                                      |
| `predeterminado` |                                       | El valor por defecto que se mostrará cuando la variable seleccionada no está definida. |
| `format`         | `"with [$symbol$env_value]($style) "` | El formato del módulo.                                                                 |
| `descripción`    | `"<env_var module>"`                  | The description of the module that is shown when running `starship explain`.           |
| `disabled`       | `false`                               | Disables the `env_var` module.                                                         |
| `style`          | `"black bold dimmed"`                 | El estilo del módulo.                                                                  |

### Variables

| Variable                       | Ejemplo                                                        | Descripción                                |
| ------------------------------ | -------------------------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol                         |                                                                | Refleja el valor de la opción `symbol`     |
| style\*                        |                                                                | Refleja el valor de la opción `style`      |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = 'SHELL'
default = 'unknown shell'
```

Mostrando múltiples variables de entorno:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = 'SHELL'
default = 'unknown shell'
[env_var.USER]
default = 'unknown user'
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `' '`                               | El símbolo usado antes de mostrar la versión de Erlang.                                          |
| `style`             | `'bold red'`                         | El estilo del módulo.                                                                            |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `disabled`          | `false`                              | Disables the `erlang` module.                                                                    |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| version  | `v22.1.3` | The version of `erlang`                |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

The `fennel` module shows the currently installed version of [Fennel](https://fennel-lang.org).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a file with the `.fnl` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🧅 '`                              | The symbol used before displaying the version of fennel.                                         |
| `style`             | `'bold green'`                       | El estilo del módulo.                                                                            |
| `detect_extensions` | `['fnl']`                            | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `disabled`          | `false`                              | Disables the `fennel` module.                                                                    |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.2.1` | The version of `fennel`                |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## Rellenar

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are
present in a line they will split the space evenly between them. Esto es útil para alinear
otros módulos.

### Opciones

| Opción     | Predeterminado | Descripción                                                |
| ---------- | -------------- | ---------------------------------------------------------- |
| `simbol`   | `'.'`          | El símbolo utilizado para llenar la línea. |
| `style`    | `'bold black'` | El estilo del módulo.                      |
| `disabled` | `false`        | Disables the `fill` module                                 |

### Ejemplo

```toml
# ~/.config/starship.toml
format = 'AA $fill BB $fill CC'

[fill]
symbol = '-'
style = 'bold green'
```

Produce un prompt que se ve como:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Fortran

The `fortran` module shows the current compiler version of Fortran.

### Opciones

| Opción              | Predeterminado                                                                                                              | Descripción                                                                                                      |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `simbol`            | `' '`                                                                                                                      | The symbol used before displaying the version of Fortran.                                        |
| `format`            | `'via [$symbol($version )]($style)'`                                                                                        | El formato del módulo.                                                                           |
| `version_format`    | `'${raw}'`                                                                                                                  | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold purple'`                                                                                                             | El estilo del módulo.                                                                            |
| `detect_extensions` | `['f', 'F', 'for', 'FOR', 'ftn', 'FTN', 'f77', 'F77', 'f90', 'F90', 'f95', 'F95','f03', 'F03', 'f08', 'F08', 'f18', 'F18']` | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['fpm.toml']`                                                                                                              | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                                                        | Qué carpetas deberían activar este módulo.                                                       |
| `commands`          | `[ [ 'gfortran', '--version' ], [ 'flang', '--version' ], [ 'flang-new', '--version' ] ]`                                   | Cómo detectar cuál compilador es                                                                                 |
| `disabled`          | `false`                                                                                                                     | Disables the `fortran` module.                                                                   |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| name     | gfortran | El nombre del compilador               |
| version  | `14.2.0` | The version of the Fortran compiler    |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['myfortran', '--version']`. Starship intentará ejecutar cada comando hasta que obtenga un resultado en STDOUT.

If a Fortran compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).

## Fossil Branch

The `fossil_branch` module shows the name of the active branch of the check-out in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción              | Predeterminado                   | Descripción                                                                                                                       |
| ------------------- | -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | El formato del módulo. Use `'$branch'` to refer to the current branch name.                       |
| `simbol`            | `' '`                           | The symbol used before the branch name of the check-out in your current directory.                                |
| `style`             | `'bold purple'`                  | El estilo del módulo.                                                                                             |
| `truncation_length` | `2^63 - 1`                       | Truncates a Fossil branch name to `N` graphemes                                                                                   |
| `truncation_symbol` | `'…'`                            | El símbolo usado para indicar que un nombre de rama fue truncado. You can use `''` for no symbol. |
| `disabled`          | `true`                           | Disables the `fossil_branch` module.                                                                              |

### Variables

| Variable | Ejemplo | Descripción                            |
| -------- | ------- | -------------------------------------- |
| rama     | `trunk` | The active Fossil branch               |
| symbol   |         | Refleja el valor de la opción `symbol` |
| style\*  |         | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Fossil Metrics

The `fossil_metrics` module will show the number of added and deleted lines in the check-out in your current directory. At least v2.14 (2021-01-20) of Fossil is required.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción               | Predeterminado                                               | Descripción                                                        |
| -------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------ |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | El formato del módulo.                             |
| `added_style`        | `'bold green'`                                               | El estilo para el recuento añadido.                |
| `deleted_style`      | `'bold red'`                                                 | El estilo para el recuento eliminado.              |
| `only_nonzero_diffs` | `true`                                                       | Mostrar sólo el estado de los elementos cambiados. |
| `disabled`           | `true`                                                       | Disables the `fossil_metrics` module.              |

### Variables

| Variable                             | Ejemplo | Descripción                                 |
| ------------------------------------ | ------- | ------------------------------------------- |
| añadido                              | `1`     | El número actual de líneas añadidas         |
| eliminado                            | `2`     | El número actual de líneas eliminadas       |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI.
This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.
The `CLOUDSDK_CORE_PROJECT` and `CLOUDSDK_COMPUTE_REGION` environment variables, when set, override the `project` and `region` values from the active configuration, mirroring the behavior of `gcloud` itself.

When the module is enabled it will always be active, unless `detect_env_vars` has
been set in which case the module will only be active when one of the
environment variables has been set.

### Opciones

| Opción            | Predeterminado                                             | Descripción                                                                  |
| ----------------- | ---------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | El formato del módulo.                                       |
| `simbol`          | `'☁️  '`                                                   | El símbolo usado antes de mostrar el perfil actual de GCP.   |
| `region_aliases`  | `{}`                                                       | Tabla de alias de región a mostrar además del nombre GCP.    |
| `project_aliases` | `{}`                                                       | Tabla de alias del proyecto a mostrar además del nombre GCP. |
| `detect_env_vars` | `[]`                                                       | Which environmental variables should trigger this module                     |
| `style`           | `'bold blue'`                                              | El estilo del módulo.                                        |
| `disabled`        | `false`                                                    | Disables the `gcloud` module.                                |

### Variables

| Variable | Ejemplo          | Descripción                                                        |
| -------- | ---------------- | ------------------------------------------------------------------ |
| region   | `us-central1`    | La actual región GCP                                               |
| cuenta   | `foo`            | El perfil actual de GCP                                            |
| dominio  | `example.com`    | El dominio actual del perfil GCP                                   |
| proyecto |                  | El proyecto GCP actual                                             |
| activo   | `predeterminado` | The active config name written in `~/.config/gcloud/active_config` |
| symbol   |                  | Refleja el valor de la opción `symbol`                             |
| style\*  |                  | Refleja el valor de la opción `style`                              |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplos

#### Mostrar cuenta y proyecto

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
```

#### Mostrar solo el nombre de la configuración activa

```toml
# ~/.config/starship.toml

[gcloud]
format = '[$symbol$active]($style) '
style = 'bold yellow'
```

#### Mostrar los alias de cuenta y región

```toml
# ~/.config/starship.toml

[gcloud]
symbol = '️🇬️ '
[gcloud.region_aliases]
us-central1 = 'uc1'
asia-northeast1 = 'an1'
```

#### Mostrar cuenta y proyecto con alias

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = 'vlpn'
```

## Rama Git

The `git_branch` module shows the active branch of the repo in your current directory.

### Opciones

| Opción               | Predeterminado                                    | Descripción                                                                                                                       |
| -------------------- | ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Muestra el nombre de la rama de seguimiento remoto, incluso si es igual al nombre de la rama local.               |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | El formato del módulo. Use `'$branch'` to refer to the current branch name.                       |
| `simbol`             | `' '`                                            | Una cadena de formato que representa el símbolo de la rama git.                                                   |
| `style`              | `'bold purple'`                                   | El estilo del módulo.                                                                                             |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                                                                          |
| `truncation_symbol`  | `'…'`                                             | El símbolo usado para indicar que un nombre de rama fue truncado. You can use `''` for no symbol. |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state.                                                    |
| `ignore_branches`    | `[]`                                              | Una lista de nombres a evitar ser visualizados. Useful for 'master' or 'main'.                    |
| `ignore_bare_repo`   | `false`                                           | Do not show when in a bare repo.                                                                                  |
| `disabled`           | `false`                                           | Disables the `git_branch` module.                                                                                 |

### Variables

| Variable                           | Ejemplo  | Descripción                                                                                                                                                               |
| ---------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| rama                               | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | El nombre remoto.                                                                                                                                         |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                                                                          |
| symbol                             |          | Refleja el valor de la opción `symbol`                                                                                                                                    |
| style\*                            |          | Refleja el valor de la opción `style`                                                                                                                                     |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = '🌱 '
truncation_length = 4
truncation_symbol = ''
ignore_branches = ['master', 'main']
```

## Git commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Opciones

| Opción               | Predeterminado                 | Descripción                                                                                                                            |
| -------------------- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------- |
| `commit_hash_length` | `7`                            | La longitud del hash de la confirmación de git mostrado.                                                               |
| `format`             | `'[\($hash$tag\)]($style) '` | El formato del módulo.                                                                                                 |
| `style`              | `'bold green'`                 | El estilo del módulo.                                                                                                  |
| `only_detached`      | `true`                         | Only show git commit hash when in detached `HEAD` state                                                                                |
| `tag_disabled`       | `true`                         | Disables showing tag info in `git_commit` module.                                                                      |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. El valor por defecto sólo permite coincidencias exactas. |
| `tag_symbol`         | `' 🏷  '`                      | Símbolo de etiqueta prefijando la información mostrada                                                                                 |
| `disabled`           | `false`                        | Disables the `git_commit` module.                                                                                      |

### Variables

| Variable | Ejemplo   | Descripción                                                  |
| -------- | --------- | ------------------------------------------------------------ |
| hash     | `b703eb3` | El hash actual de la confirmación de git                     |
| etiqueta | `v1.0.0`  | The tag name if showing tag info is enabled. |
| style\*  |           | Refleja el valor de la opción `style`                        |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git State

The `git_state` module will show in directories which are part of a git
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc. Si hay información de progreso (por ejemplo, REBASING 3/10),
esa información será mostrada también.

### Opciones

| Opción         | Predeterminado                                                  | Descripción                                                                                                     |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | A format string displayed when a `rebase` is in progress.                                       |
| `merge`        | `'FUSIONANDO'`                                                  | A format string displayed when a `merge` is in progress.                                        |
| `revert`       | `'REVERTING'`                                                   | A format string displayed when a `revert` is in progress.                                       |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | A format string displayed when a `cherry-pick` is in progress.                                  |
| `bisect`       | `'BISECTING'`                                                   | A format string displayed when a `bisect` is in progress.                                       |
| `am`           | `'AM'`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress. |
| `am_or_rebase` | `'AM/REBASE'`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.         |
| `style`        | `'bold yellow'`                                                 | El estilo del módulo.                                                                           |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | El formato del módulo.                                                                          |
| `disabled`     | `false`                                                         | Disables the `git_state` module.                                                                |

### Variables

| Variable                              | Ejemplo    | Descripción                           |
| ------------------------------------- | ---------- | ------------------------------------- |
| state                                 | `REBASING` | The current state of the repo         |
| progress_current | `1`        | El progreso de la operación actual    |
| progress_total   | `2`        | El progreso total de la operación     |
| style\*                               |            | Refleja el valor de la opción `style` |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Métricas de Git

The `git_metrics` module will show the number of added and deleted lines in
the current git repository.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción               | Predeterminado                                               | Descripción                                                        |
| -------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------ |
| `added_style`        | `'bold green'`                                               | El estilo para el recuento añadido.                |
| `deleted_style`      | `'bold red'`                                                 | El estilo para el recuento eliminado.              |
| `only_nonzero_diffs` | `true`                                                       | Mostrar sólo el estado de los elementos cambiados. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | El formato del módulo.                             |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.                 |
| `ignore_submodules`  | `false`                                                      | Ignorar cambios a los submódulos                                   |

### Variables

| Variable                             | Ejemplo | Descripción                                 |
| ------------------------------------ | ------- | ------------------------------------------- |
| añadido                              | `1`     | El número actual de líneas añadidas         |
| eliminado                            | `2`     | El número actual de líneas eliminadas       |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git status

The `git_status` module shows symbols representing the state of the repo in your
current directory.

> [!TIP]
> The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment.
> You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

### Opciones

| Opción                 | Predeterminado                                  | Descripción                                                                                                                                    |
| ---------------------- | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`               | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status`                                                                                                            |
| `conflicted`           | `'='`                                           | The format shown when this branch has merge conflicts.                                                                         |
| `ahead`                | `'⇡'`                                           | The format shown when this branch is ahead of the branch being tracked.                                                        |
| `behind`               | `'⇣'`                                           | The format shown when this branch is behind the branch being tracked.                                                          |
| `diverged`             | `'⇕'`                                           | The format shown when this branch has diverged from the branch being tracked.                                                  |
| `up_to_date`           | `''`                                            | The format shown when this branch is up to date with the branch being tracked.                                                 |
| `untracked`            | `'?'`                                           | The format shown when there are untracked files in the working directory.                                                      |
| `stashed`              | `'\$'`                                         | The format shown when a stash exists for the local repository.                                                                 |
| `modified`             | `'!'`                                           | The format shown when there are file modifications in the working directory.                                                   |
| `staged`               | `'+'`                                           | The format shown when a new file has been added to the staging area.                                                           |
| `renamed`              | `'»'`                                           | The format shown when a renamed file has been added to the staging area.                                                       |
| `deleted`              | `'✘'`                                           | The format shown when a file's deletion has been added to the staging area.                                                    |
| `typechanged`          | `""`                                            | The format shown when a file's type has been changed in the staging area.                                                      |
| `style`                | `'bold red'`                                    | El estilo del módulo.                                                                                                          |
| `ignore_submodules`    | `false`                                         | Ignorar cambios a los submódulos.                                                                                              |
| `worktree_added`       | `""`                                            | The format shown when a new file has been added in the working directory.                                                      |
| `worktree_deleted`     | `""`                                            | The format shown when a file has been deleted in the working directory.                                                        |
| `worktree_modified`    | `""`                                            | The format shown when a file has been modified in the working directory.                                                       |
| `worktree_typechanged` | `""`                                            | The format shown when a file's type has been changed in the working directory.                                                 |
| `index_added`          | `""`                                            | The format shown when a new file has been added to the staging area.                                                           |
| `index_deleted`        | `""`                                            | The format shown when a file has been deleted from the staging area.                                                           |
| `index_modified`       | `""`                                            | The format shown when a file has been modified in the staging area.                                                            |
| `index_typechanged`    | `""`                                            | The format shown when a file's type has been changed in the staging area.                                                      |
| `disabled`             | `false`                                         | Disables the `git_status` module.                                                                                              |
| `windows_starship`     |                                                 | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |
| `use_git_executable`   | `false`                                         | Do not use `gitoxide` for computing the status, but use the `git` executable instead.                                          |

### Variables

The following variables can be used in `format`:

| Variable               | Descripción                                                                                                                   |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `all_status`           | Shortcut for `$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`.                     |
| `ahead_behind`         | Displays `diverged`, `ahead`, `behind` or `up_to_date` format string based on the current status of the repo. |
| `conflicted`           | Displays `conflicted` when this branch has merge conflicts.                                                   |
| `untracked`            | Displays `untracked` when there are untracked files in the working directory.                                 |
| `stashed`              | Displays `stashed` when a stash exists for the local repository.                                              |
| `modified`             | Displays `modified` when there are file modifications in the working directory.                               |
| `staged`               | Displays `staged` when a new file has been added to the staging area.                                         |
| `renamed`              | Displays `renamed` when a renamed file has been added to the staging area.                                    |
| `deleted`              | Displays `deleted` when a file's deletion has been added to the staging area.                                 |
| `typechanged`          | Displays `typechanged` when a file's type has been changed in the staging area.                               |
| `worktree_added`       | Displays `worktree_added` when a new file has been added in the working directory.                            |
| `worktree_deleted`     | Displays `worktree_deleted` when a file's been deleted in the working directory.                              |
| `worktree_modified`    | Displays `worktree_modified` when a file's been modified in the working directory.                            |
| `worktree_typechanged` | Displays `worktree_typechanged` when a file's type has been changed in the working directory.                 |
| `index_added`          | Displays `index_added` when a new file has been added to the staging area.                                    |
| `index_deleted`        | Displays `index_deleted` when a file has been deleted from the staging area.                                  |
| `index_modified`       | Displays `index_modified` when a file has been modified in the staging area.                                  |
| `index_typechanged`    | Displays `index_typechanged` when a file's type has been changed in the staging area.                         |
| style\*                | Refleja el valor de la opción `style`                                                                                         |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

The following variables can be used in `diverged`:

| Variable       | Descripción                                                    |
| -------------- | -------------------------------------------------------------- |
| `ahead_count`  | Número de confirmaciones por delante de la rama de seguimiento |
| `behind_count` | Número de confirmaciones detrás de la rama de seguimiento      |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed`, `deleted`, `typechanged`, `worktree_added`, `worktree_deleted`, `worktree_modified`, `worktree_typechanged`, `index_added`, `index_deleted`, `index_modified`, and `index_typechanged`:

| Variable | Descripción                   |
| -------- | ----------------------------- |
| `count`  | Mostrar el número de archivos |

### Ejemplo

```toml
# ~/.config/starship.toml

[git_status]
conflicted = '🏳'
ahead = '🏎💨'
behind = '😰'
diverged = '😵'
up_to_date = '✓'
untracked = '🤷'
stashed = '📦'
modified = '📝'
staged = '[++\($count\)](green)'
renamed = '👅'
deleted = '🗑'
```

Muestra el conteo delante/detrás de la rama que está siendo rastreada

```toml
# ~/.config/starship.toml

[git_status]
ahead = '⇡${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
behind = '⇣${count}'
```

Usar el ejecutable de Starship de Windows en las rutas de Windows en WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```

## Gleam

The `gleam` module shows the currently installed version of [Gleam](https://gleam.run/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `gleam.toml` file
- The current directory contains a file with the `.gleam` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'⭐ '`                               | A format string representing the symbol of Gleam.                                                |
| `detect_extensions` | `['gleam']`                          | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['gleam.toml']`                     | Qué nombres de archivo deberían activar este módulo.                                             |
| `style`             | `'bold #FFAFF3'`                     | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `gleam` module.                                                                     |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.0.0` | The version of `gleam`                 |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `go.work` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Opciones

| Opción              | Predeterminado                                                                            | Descripción                                                                                                                                |
| ------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | El formato del módulo.                                                                                                     |
| `version_format`    | `'v${raw}'`                                                                               | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch`                           |
| `simbol`            | `'🐹 '`                                                                                   | Una cadena de formato que representa el símbolo de Go.                                                                     |
| `detect_extensions` | `['go']`                                                                                  | Qué extensiones deberían activar este módulo.                                                                              |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Qué nombres de archivo deberían activar este módulo.                                                                       |
| `detect_folders`    | `['Godeps']`                                                                              | Qué carpetas deberían activar este módulo.                                                                                 |
| `style`             | `'bold cyan'`                                                                             | El estilo del módulo.                                                                                                      |
| `not_capable_style` | `'bold red'`                                                                              | The style for the module when the go directive in the go.mod file does not match the installed Go version. |
| `disabled`          | `false`                                                                                   | Disables the `golang` module.                                                                                              |

### Variables

| Variable                         | Ejemplo   | Descripción                                                                                                                                                                 |
| -------------------------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version                          | `v1.12.1` | The version of `go`                                                                                                                                                         |
| mod_version | `1.16`    | `go` version requirement as set in the go directive of `go.mod`. Will only show if the version requirement does not match the `go` version. |
| symbol                           |           | Refleja el valor de la opción `symbol`                                                                                                                                      |
| style\*                          |           | Refleja el valor de la opción `style`                                                                                                                                       |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

### Using `mod_version`

```toml
# ~/.config/starship.toml

[golang]
format = 'via [$symbol($version )($mod_version )]($style)'
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment.
The module will be shown when inside a guix-shell environment.

### Opciones

| Opción     | Predeterminado             | Descripción                                                            |
| ---------- | -------------------------- | ---------------------------------------------------------------------- |
| `format`   | `'vía [$symbol]($style) '` | El formato del módulo.                                 |
| `simbol`   | `'🐃 '`                    | A format string representing the symbol of guix-shell. |
| `style`    | `'yellow bold'`            | El estilo del módulo.                                  |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### Variables

| Variable | Ejemplo | Descripción                            |
| -------- | ------- | -------------------------------------- |
| symbol   |         | Refleja el valor de la opción `symbol` |
| style\*  |         | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html)
currently used in the project directory.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🅶 '`                              | A format string representing the symbol of Gradle.                                               |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `['gradle']`                         | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold bright-cyan'`                 | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `gradle` module.                                                                    |
| `recursive`         | `false`                              | Enables recursive finding for the `gradle` directory.                                            |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v7.5.1` | The version of `gradle`                |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `stack.yaml` file
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                          |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                               |
| `simbol`            | `'λ '`                               | Una cadena de formato que representa el símbolo de Haskell           |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.           |
| `style`             | `'bold purple'`                      | El estilo del módulo.                                |
| `disabled`          | `false`                              | Disables the `haskell` module.                       |

### Variables

| Variable                           | Ejemplo     | Descripción                                                                             |
| ---------------------------------- | ----------- | --------------------------------------------------------------------------------------- |
| version                            |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot                           | `lts-18.12` | Instantánea de Stack seleccionada actualmente                                           |
| ghc\_version | `9.2.1`     | Versión GHC instalada actualmente                                                       |
| symbol                             |             | Refleja el valor de la opción `symbol`                                                  |
| style\*                            |             | Refleja el valor de la opción `style`                                                   |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### Opciones

| Opción              | Predeterminado                                                                                  | Descripción                                                                                                      |
| ------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                                                                                     | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | Qué carpetas deberían activar este módulo.                                                       |
| `simbol`            | `'⌘ '`                                                                                          | A format string representing the symbol of Haxe.                                                 |
| `style`             | `'bold fg:202'`                                                                                 | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                                                                         | Disables the `haxe` module.                                                                      |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v4.2.5` | The version of `haxe`                  |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `simbol`            | `'⎈ '`                               | Una cadena de formato que representa el símbolo de Helm.                                         |
| `style`             | `'bold white'`                       | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `helm` module.                                                                      |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v3.1.1` | The version of `helm`                  |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Hostname

The `hostname` module shows the system hostname.

### Opciones

| Opción            | Predeterminado                         | Descripción                                                                                                                                                                                              |
| ----------------- | -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | Mostrar sólo el nombre de host cuando esté conectado a una sesión SSH.                                                                                                                   |
| `ssh_symbol`      | `'🌐 '`                                | Una cadena de formato que representa el símbolo cuando se conecta a la sesión SSH.                                                                                                       |
| `trim_at`         | `'.'`                                  | Cadena en la que el nombre del host se corta, después de la primera coincidencia. `'.'` will stop after the first dot. `''` will disable any truncation. |
| `detect_env_vars` | `[]`                                   | Qué variable(s) de entorno deben activar este módulo.                                                                                                                 |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | El formato del módulo.                                                                                                                                                                   |
| `style`           | `'negrita oscurecida verde'`           | El estilo del módulo.                                                                                                                                                                    |
| `disabled`        | `false`                                | Disables the `hostname` module.                                                                                                                                                          |
| `aliases`         | `{}`                                   | Translate system hostnames to something else. If `trim_at` is specified, only the first part will be matched and replaced.                                               |

### Variables

| Variable                        | Ejemplo    | Descripción                                                    |
| ------------------------------- | ---------- | -------------------------------------------------------------- |
| nombre del host                 | `computer` | El nombre de host de la computadora                            |
| style\*                         |            | Refleja el valor de la opción `style`                          |
| ssh_symbol | `'🌏 '`    | El símbolo a representar cuando está conectado a la sesión SSH |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplos

#### Mostrar siempre el nombre del host

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

#### Ocultar el nombre de host en sesiones remotas de tmux

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
detect_env_vars = ['!TMUX', 'SSH_CONNECTION']
disabled = false
```

#### Replace the hostname with a nickname

```toml
# ~/.config/starship.toml
[hostname]
aliases = { "Max's MacBook Pro" = "home" }
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, or `.sdkmanrc` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Opciones

| Opción              | Predeterminado                                                                                                        | Descripción                                                                                                      |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                                                                                                           | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                                                  | Qué carpetas deberían activar este módulo.                                                       |
| `simbol`            | `'☕ '`                                                                                                                | Una cadena de formato que representa el símbolo de Java                                                          |
| `style`             | `'red dimmed'`                                                                                                        | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                                                                                               | Disables the `java` module.                                                                      |

### Variables

| Variable | Ejemplo | Descripción                            |
| -------- | ------- | -------------------------------------- |
| version  | `v14`   | The version of `java`                  |
| symbol   |         | Refleja el valor de la opción `symbol` |
| style\*  |         | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## Trabajos

The `jobs` module shows the current number of jobs running.
El módulo se mostrará sólo si hay tareas en segundo plano ejecutándose.
The module will show the number of jobs running if there are at least
2 jobs, or more than the `number_threshold` config value, if it exists.
The module will show a symbol if there is at least 1 job, or more than the
`symbol_threshold` config value, if it exists. You can set both values
to 0 in order to _always_ show the symbol and number of jobs, even if there are
0 jobs running.

La funcionalidad por defecto es:

- 0 tareas -> No se muestra nada.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

> [!WARNING]
> This module is not supported on tcsh.

> [!WARNING]
> The `threshold` option is deprecated, but if you want to use it,
> the module will show the number of jobs running if there is more than 1 job, or
> more than the `threshold` config value, if it exists. If `threshold` is set to 0,
> then the module will also show when there are 0 jobs running.

### Opciones

| Opción             | Predeterminado                | Descripción                                                                              |
| ------------------ | ----------------------------- | ---------------------------------------------------------------------------------------- |
| `threshold`\*      | `1`                           | Muestra el número de tareas si se exceden.                               |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | El formato del módulo.                                                   |
| `simbol`           | `'✦'`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `'bold blue'`                 | El estilo del módulo.                                                    |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

\*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| Variable | Ejemplo | Descripción                            |
| -------- | ------- | -------------------------------------- |
| número   | `1`     | El número de tareas                    |
| symbol   |         | Refleja el valor de la opción `symbol` |
| style\*  |         | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplos

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

#### Changing process grouping behavior in fish

When using the Fish shell, Starship counts **job groups** instead of individual process IDs by default. This prevents overcounting when a pipeline has multiple processes but only one suspended group. To revert to the legacy PID-based counting, please add the following to your shell config:

```fish
set -g __starship_fish_use_job_groups "false"
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `simbol`            | `'ஃ '`                               | Una cadena de formato que representa el símbolo de Julia.                                        |
| `style`             | `'bold purple'`                      | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `julia` module.                                                                     |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.4.0` | The version of `julia`                 |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.kt` or a `.kts` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['kt', 'kts']`                      | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `simbol`            | `'🅺 '`                              | Una cadena de formato que representa el símbolo de Kotlin.                                       |
| `style`             | `'bold blue'`                        | El estilo del módulo.                                                                            |
| `kotlin_binary`     | `'kotlin'`                           | Configura el binario kotlin que Starship ejecuta al obtener la versión.                          |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                                    |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| version  | `v1.4.21` | The version of `kotlin`                |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[kotlin]
symbol = '🅺 '
```

```toml
# ~/.config/starship.toml

[kotlin]
# Utiliza el compilador binario Kotlink para obtener la versión instalada
kotlin_binary = 'kotlinc'
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file.
The namespace needs to be set in the kubeconfig file, this can be done via
`kubectl config set-context starship-context --namespace astronaut`.
Similarly, the user and cluster can be set with `kubectl config set-context starship-context --user starship-user`
and `kubectl config set-context starship-context --cluster starship-cluster`.
If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.
>
> When the module is enabled it will always be active, unless any of
> `detect_env_vars`, `detect_extensions`, `detect_files` or `detect_folders` have
> been set in which case the module will only be active in directories that match
> those conditions or one of the environmental variables has been set.

### Opciones

> [!WARNING]
> The `context_aliases` and `user_aliases` options are deprecated. Use `contexts` and the corresponding `context_alias`
> and `user_alias` options instead.

| Opción              | Predeterminado                                       | Descripción                                                                                 |
| ------------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `simbol`            | `'☸ '`                                               | Una cadena de formato que representa el símbolo mostrado antes del Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | El formato del módulo.                                                      |
| `style`             | `'cyan bold'`                                        | El estilo del módulo.                                                       |
| `context_aliases`\* | `{}`                                                 | Tabla de alias de contexto a mostrar.                                       |
| `user_aliases`\*    | `{}`                                                 | Table of user aliases to display.                                           |
| `detect_extensions` | `[]`                                                 | Qué extensiones deberían activar este módulo.                               |
| `detect_files`      | `[]`                                                 | Qué nombres de archivo deberían activar este módulo.                        |
| `detect_folders`    | `[]`                                                 | Qué carpetas deberían activar este módulo.                                  |
| `detect_env_vars`   | `[]`                                                 | Which environmental variables should trigger this module                                    |
| `contexts`          | `[]`                                                 | Customized styles and symbols for specific contexts.                        |
| `disabled`          | `true`                                               | Disables the `kubernetes` module.                                           |

\*: This option is deprecated, please add `contexts` with the corresponding `context_alias` and `user_alias` options instead.

To customize the style of the module for specific environments, use the following configuration as
part of the `contexts` list:

| Variable          | Descripción                                                                                                              |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `context_pattern` | **Required** Regular expression to match current Kubernetes context name.                                |
| `user_pattern`    | Regular expression to match current Kubernetes user name.                                                |
| `context_alias`   | Context alias to display instead of the full context name.                                               |
| `user_alias`      | User alias to display instead of the full user name.                                                     |
| `style`           | The style for the module when using this context. If not set, will use module's style.   |
| `simbol`          | The symbol for the module when using this context. If not set, will use module's symbol. |

Note that all regular expression are anchored with `^<pattern>$` and so must match the whole string. The `*_pattern`
regular expressions may contain capture groups, which can be referenced in the corresponding alias via `$name` and `$N`
(see example below and the
[rust Regex::replace() documentation](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

### Variables

| Variable  | Ejemplo              | Descripción                                                 |
| --------- | -------------------- | ----------------------------------------------------------- |
| contexto  | `starship-context`   | El nombre del contexto actual de kubernetes                 |
| namespace | `starship-namespace` | Si se establece, el espacio de nombres actual de kubernetes |
| usuario   | `starship-user`      | Si se establece, el espacio de nombres actual de kubernetes |
| cluster   | `starship-cluster`   | Si se establece, el clúster actual de kubernetes            |
| symbol    |                      | Refleja el valor de la opción `symbol`                      |
| style\*   |                      | Refleja el valor de la opción `style`                       |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
contexts = [
  { context_pattern = "dev.local.cluster.k8s", style = "green", symbol = "💔 " },
]
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Configuración específica del Contexto de Kubernetes

The `contexts` configuration option is used to customise what the current Kubernetes context name looks
like (style and symbol) if the name matches the defined regular expression.

```toml
# ~/.config/starship.toml

[[kubernetes.contexts]]
# "bold red" style + default symbol when Kubernetes current context name equals "production" *and* the current user
# equals "admin_user"
context_pattern = "production"
user_pattern = "admin_user"
style = "bold red"
context_alias = "prod"
user_alias = "admin"

[[kubernetes.contexts]]
# "green" style + a different symbol when Kubernetes current context name contains openshift
context_pattern = ".*openshift.*"
style = "green"
symbol = "💔 "
context_alias = "openshift"

[[kubernetes.contexts]]
# Using capture groups
# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
context_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
context_alias = "gke-$cluster"
```

## Salto de línea

The `line_break` module separates the prompt into two lines.

### Opciones

| Opción     | Predeterminado | Descripción                                                                        |
| ---------- | -------------- | ---------------------------------------------------------------------------------- |
| `disabled` | `false`        | Disables the `line_break` module, making the prompt a single line. |

### Ejemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## IP local

The `localip` module shows the IPv4 address of the primary network interface.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción     | Predeterminado            | Descripción                                                                             |
| ---------- | ------------------------- | --------------------------------------------------------------------------------------- |
| `ssh_only` | `true`                    | Solo muestra la direccion IP cuando se está conectado a una sesión SSH. |
| `format`   | `'[$localipv4]($style) '` | El formato del módulo.                                                  |
| `style`    | `'bold yellow'`           | El estilo del módulo.                                                   |
| `disabled` | `true`                    | Disables the `localip` module.                                          |

### Variables

| Variable  | Ejemplo                                                      | Descripción                           |
| --------- | ------------------------------------------------------------ | ------------------------------------- |
| localipv4 | 192.168.1.13 | Contiene la dirección IPv4 primaria   |
| style\*   |                                                              | Refleja el valor de la opción `style` |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🌙 '`                              | Una cadena de formato que representa el símbolo de Lua.                                          |
| `detect_extensions` | `['lua']`                            | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['.lua-version']`                   | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `['lua']`                            | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold blue'`                        | El estilo del módulo.                                                                            |
| `lua_binary`        | `'lua'`                              | Configura el binario lua que Starship ejecuta al obtener la versión.                             |
| `disabled`          | `false`                              | Disables the `lua` module.                                                                       |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v5.4.0` | The version of `lua`                   |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Maven

The `maven` module indicates the presence of a Maven project in the current directory. If the [Maven Wrapper](https://maven.apache.org/wrapper/) is enabled, the Maven version will be parsed from `.mvn/wrapper/maven-wrapper.properties` and shown.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `pom.xml` file.
- The current directory contains a `.mvn/wrapper/maven-wrapper.properties` file.

If you use an alternate POM syntax (for example `pom.hocon`), add its filename to `detect_files`.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🅼 '`                              | A format string representing the symbol of Maven.                                                |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['pom.xml']`                        | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `['.mvn']`                           | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold bright-cyan'`                 | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `maven` module.                                                                     |
| `recursive`         | `false`                              | Enables recursive finding for the `.mvn` directory.                                              |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v3.2.0` | The version of `maven`                 |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## Uso de la memoria

The `memory_usage` module shows current system memory and swap usage.

Por defecto, el uso de la memoria de intercambio se muestra si no es cero.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción      | Predeterminado                                 | Descripción                                                                   |
| ----------- | ---------------------------------------------- | ----------------------------------------------------------------------------- |
| `threshold` | `75`                                           | Ocultar el uso de memoria a menos que supere este porcentaje. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | El formato del módulo.                                        |
| `simbol`    | `'🐏'`                                         | El símbolo usado antes de mostrar el uso de memoria.          |
| `style`     | `'bold dimmed white'`                          | El estilo del módulo.                                         |
| `disabled`  | `true`                                         | Disables the `memory_usage` module.                           |

### Variables

| Variable                          | Ejemplo       | Descripción                                                                                        |
| --------------------------------- | ------------- | -------------------------------------------------------------------------------------------------- |
| ram                               | `31GiB/65GiB` | La memoria RAM usada/total del sistema actual.                                     |
| ram_pct      | `48%`         | El porcentaje de la memoria actual del sistema.                                    |
| swap\*\*                          | `1GiB/4GiB`   | El tamaño de la memoria de intercambio del archivo de memoria del sistema actual.  |
| swap_pct\*\* | `77%`         | El porcentaje de memoria de intercambio del archivo de memoria del sistema actual. |
| symbol                            | `🐏`          | Refleja el valor de la opción `symbol`                                                             |
| style\*                           |               | Refleja el valor de la opción `style`                                                              |

\*: Está variable solo puede utilizarse como parte de una cadena de estilo
\*\*: La información del archivo SWAP solo se muestra si se detecta en el sistema actual

### Ejemplo

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = ' '
style = 'bold dimmed green'
```

## Meson

The `meson` module shows the current Meson developer environment status.

By default the Meson project name is displayed, if `$MESON_DEVENV` is set.

### Opciones

| Opción              | Predeterminado                     | Descripción                                                                                                               |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | El formato del módulo.                                                                                    |
| `simbol`            | `'⬢ '`                             | The symbol used before displaying the project name.                                                       |
| `style`             | `'blue bold'`                      | El estilo del módulo.                                                                                     |
| `disabled`          | `false`                            | Disables the `meson` module.                                                                              |

### Variables

| Variable | Ejemplo    | Descripción                            |
| -------- | ---------- | -------------------------------------- |
| proyecto | `starship` | El nombre actual del proyecto Meson    |
| symbol   | `🐏`       | Refleja el valor de la opción `symbol` |
| style\*  |            | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```

## Rama Mercurial

The `hg_branch` module shows the active branch and topic of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción              | Predeterminado                            | Descripción                                                                                                         |
| ------------------- | ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `simbol`            | `' '`                                    | El símbolo usado antes del marcador hg o nombre de la rama del repositorio en su directorio actual. |
| `style`             | `'bold purple'`                           | El estilo del módulo.                                                                               |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | El formato del módulo.                                                                              |
| `truncation_length` | `2^63 - 1`                                | Truncates the hg branch / topic name to `N` graphemes                                                               |
| `truncation_symbol` | `'…'`                                     | El símbolo usado para indicar que un nombre de rama fue truncado.                                   |
| `disabled`          | `true`                                    | Disables the `hg_branch` module.                                                                    |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| rama     | `master`  | La rama mercurial activa               |
| tema     | `feature` | The active mercurial topic             |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Estado Mercurial

The `hg_state` module will show in directories which are part of a mercurial
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción       | Predeterminado              | Descripción                                                                   |
| ------------ | --------------------------- | ----------------------------------------------------------------------------- |
| `merge`      | `'FUSIONANDO'`              | A format string displayed when a `merge` is in progress.      |
| `rebase`     | `'REBASING'`                | A format string displayed when a `rebase` is in progress.     |
| `update`     | `'ACTUALIZANDO'`            | A format string displayed when a `update` is in progress.     |
| `bisect`     | `'BISECTING'`               | A format string displayed when a `bisect` is in progress.     |
| `shelve`     | `'SHELVING'`                | A format string displayed when a `shelve` is in progress.     |
| `graft`      | `'GRAFTING'`                | A format string displayed when a `graft` is in progress.      |
| `transplant` | `'TRANSPLANTING'`           | A format string displayed when a `transplant` is in progress. |
| `histedit`   | `'HISTEDITING'`             | A format string displayed when a `histedit` is in progress.   |
| `style`      | `'bold yellow'`             | El estilo del módulo.                                         |
| `format`     | `'\([$state]($style)\) '` | El formato del módulo.                                        |
| `disabled`   | `true`                      | Disables the `hg_state` module.                               |

### Variables

| Variable                              | Ejemplo    | Descripción                           |
| ------------------------------------- | ---------- | ------------------------------------- |
| state                                 | `REBASING` | The current state of the repo         |
| progress_current | `1`        | El progreso de la operación actual    |
| progress_total   | `2`        | El progreso total de la operación     |
| style\*                               |            | Refleja el valor de la opción `style` |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## Mise

The `mise` module shows the current mise health as reported by running `mise doctor`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción              | Predeterminado                                                       | Descripción                                                          |
| ------------------- | -------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `simbol`            | `'mise '`                                                            | The symbol used before displaying _mise_ health.     |
| `style`             | `'bold purple'`                                                      | El estilo del módulo.                                |
| `format`            | `'on [$symbol$health]($style) '`                                     | El formato del módulo.                               |
| `detect_extensions` | `[]`                                                                 | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `['mise.toml', 'mise.local.toml', '.mise.toml', '.mise.local.toml']` | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `['.mise']`                                                          | Qué carpetas deberían activar este módulo.           |
| `healthy_symbol`    | `healthy`                                                            | The message displayed when _mise_ is healthy.        |
| `unhealthy_symbol`  | `unhealthy`                                                          | The message displayed when _mise_ is unhealthy.      |
| `disabled`          | `true`                                                               | Disables the `mise` module.                          |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| health   | `healthy` | The health of _mise_                   |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[mise]
health = 'ready'
```

## Mojo

The `mojo` module shows the current version of [Mojo programming language](https://www.modular.com/mojo) installed

### Opciones

| Opción              | Predeterminado                        | Descripción                                                            |
| ------------------- | ------------------------------------- | ---------------------------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'` | El formato del módulo.                                 |
| `simbol`            | `'🔥 '`                               | The symbol used before displaying the version of Mojo. |
| `style`             | `'bold 208'`                          | El estilo del módulo.                                  |
| `disabled`          | `false`                               | Disables the `mojo` module.                            |
| `detect_extensions` | `['mojo', '🔥']`                      | Qué extensiones deberían activar este módulo.          |
| `detect_files`      | `[]`                                  | Qué nombres de archivo deberían activar este módulo.   |
| `detect_folders`    | `[]`                                  | Qué carpetas deberían activar este módulo.             |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `24.4.0` | The version of `mojo`                  |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )($hash )]($style)'
```

## NATS

The `nats` module shows the name of the current [NATS](https://nats.io) context.

### Opciones

| Opción     | Predeterminado             | Descripción                                                                                     |
| ---------- | -------------------------- | ----------------------------------------------------------------------------------------------- |
| `simbol`   | `'✉️ '`                    | The symbol used before the NATS context (defaults to empty). |
| `style`    | `'bold purple'`            | El estilo del módulo.                                                           |
| `format`   | `'[$symbol$name]($style)'` | El formato del módulo.                                                          |
| `disabled` | `false`                    | Disables the `nats` module.                                                     |

### Variables

| Variable | Ejemplo     | Descripción                            |
| -------- | ----------- | -------------------------------------- |
| name     | `localhost` | The name of the NATS context           |
| symbol   |             | Refleja el valor de la opción `symbol` |
| style\*  |             | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## Network Namespace

The `netns` module shows the current network namespace.
This uses `ip netns identify` to get the network namespace, so only network namespaces mounted at `/var/run/netns` will be detected.

### Opciones

| Opción     | Predeterminado                    | Descripción                                                                                          |
| ---------- | --------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `format`   | `'[$symbol \[$name\]]($style)'` | El formato del módulo.                                                               |
| `simbol`   | `'🛜 '`                           | The symbol used before the network namespace (defaults to empty). |
| `style`    | `'blue bold dimmed'`              | El estilo del módulo.                                                                |
| `disabled` | `false`                           | Disables the `netns` module.                                                         |

### Variables

| Variable | Ejemplo    | Descripción                               |
| -------- | ---------- | ----------------------------------------- |
| name     | `my-netns` | The name of the current network namespace |
| symbol   |            | Refleja el valor de la opción `symbol`    |
| style\*  |            | Refleja el valor de la opción `style`     |

### Ejemplo

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo                                                                                            |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'👑 '`                              | El símbolo usado antes de mostrar la versión de Nim.                                             |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['nim.cfg']`                        | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold yellow'`                      | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `nim` module.                                                                       |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.2.0` | The version of `nimc`                  |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment.
El módulo se mostrará dentro de un entorno nix-shell.

### Opciones

| Opción        | Predeterminado                                 | Descripción                                                                                      |
| ------------- | ---------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | El formato del módulo.                                                           |
| `simbol`      | `'❄️ '`                                        | Una cadena de formato que representa el símbolo de nix-shell.                    |
| `style`       | `'bold blue'`                                  | El estilo del módulo.                                                            |
| `impure_msg`  | `'impure'`                                     | Una cadena de formato que se muestra cuando el intérprete de comandos es impuro. |
| `pure_msg`    | `'pure'`                                       | Una cadena de formato que se muestra cuando el intérprete de comandos es puro.   |
| `unknown_msg` | `''`                                           | A format string shown when it is unknown if the shell is pure/impure.            |
| `disabled`    | `false`                                        | Disables the `nix_shell` module.                                                 |
| `heuristic`   | `false`                                        | Attempts to detect new `nix shell`-style shells with a heuristic.                |

### Variables

| Variable | Ejemplo | Descripción                                                                                      |
| -------- | ------- | ------------------------------------------------------------------------------------------------ |
| state    | `pure`  | El estado de nix-shell                                                                           |
| name     | `lorri` | El nombre de nix-shell                                                                           |
| level    | `1`     | The depth level of the nix-shell (Only when using [Lix](https://lix.systems)) |
| symbol   |         | Refleja el valor de la opción `symbol`                                                           |
| style\*  |         | Refleja el valor de la opción `style`                                                            |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = '[impure shell](bold red)'
pure_msg = '[pure shell](bold green)'
unknown_msg = '[unknown shell](bold yellow)'
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

Additionally, the module will be hidden by default if the directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `bunfig.toml`, `bun.lock`, or `bun.lockb` file, overriding the above conditions.

### Opciones

| Opción              | Predeterminado                                | Descripción                                                                                                                                                     |
| ------------------- | --------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | El formato del módulo.                                                                                                                          |
| `version_format`    | `'v${raw}'`                                   | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch`                                                |
| `simbol`            | `' '`                                        | Una cadena de formato que representa el símbolo de Node.js.                                                                     |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | Qué extensiones deberían activar este módulo.                                                                                                   |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | Qué nombres de archivo deberían activar este módulo.                                                                                            |
| `detect_folders`    | `['node_modules']`                            | Qué carpetas deberían activar este módulo.                                                                                                      |
| `style`             | `'bold green'`                                | El estilo del módulo.                                                                                                                           |
| `disabled`          | `false`                                       | Disables the `nodejs` module.                                                                                                                   |
| `not_capable_style` | `'bold red'`                                  | El estilo para el módulo cuando una propiedad de motores en package.json no coincide con la versión de Node.js. |

### Variables

| Variable                             | Ejemplo    | Descripción                                                                                                                                                                               |
| ------------------------------------ | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version                              | `v13.12.0` | The version of `node`                                                                                                                                                                     |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbol                               |            | Refleja el valor de la opción `symbol`                                                                                                                                                    |
| style\*                              |            | Refleja el valor de la opción `style`                                                                                                                                                     |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCamlz

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Opciones

| Opción                    | Predeterminado                                                             | Descripción                                                                                                      |
| ------------------------- | -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | La cadena de formato para el módulo.                                                             |
| `version_format`          | `'v${raw}'`                                                                | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`                  | `'🐫 '`                                                                    | El símbolo usado antes de mostrar la versión de OCaml.                                           |
| `global_switch_indicator` | `''`                                                                       | La cadena de formato usada para representar el interruptor global de OPAM.                       |
| `local_switch_indicator`  | `'*'`                                                                      | La cadena de formato usada para representar el interruptor local de OPAM.                        |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | Qué carpetas deberían activar este módulo.                                                       |
| `style`                   | `'bold yellow'`                                                            | El estilo del módulo.                                                                            |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                                                     |

### Variables

| Variable                              | Ejemplo      | Descripción                                                       |
| ------------------------------------- | ------------ | ----------------------------------------------------------------- |
| version                               | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | El interruptor OPAM activo                                        |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol                                |              | Refleja el valor de la opción `symbol`                            |
| style\*                               |              | Refleja el valor de la opción `style`                             |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Odin

The `odin` module shows the currently installed version of [Odin](https://odin-lang.org/). By default the module will be shown if the current directory contains a `.odin` file.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                            |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                 |
| `show_commit`       | `false`                              | Shows the commit as part of the version.               |
| `simbol`            | `'Ø '`                               | The symbol used before displaying the version of Odin. |
| `style`             | `'bold bright-blue'`                 | El estilo del módulo.                                  |
| `disabled`          | `false`                              | Disables the `odin` module.                            |
| `detect_extensions` | `['odin']`                           | Qué extensiones deberían activar este módulo.          |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.   |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.             |

### Variables

| Variable | Ejemplo       | Descripción                            |
| -------- | ------------- | -------------------------------------- |
| version  | `dev-2024-03` | The version of `odin`                  |
| symbol   |               | Refleja el valor de la opción `symbol` |
| style\*  |               | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool.
By default the module will be shown if the current directory contains a `.rego` file.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🪖  '`                             | A format string representing the symbol of OPA.                                                  |
| `detect_extensions` | `['rego']`                           | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold blue'`                        | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `opa` module.                                                                       |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| version  | `v0.44.0` | The version of `opa`                   |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module
only active when the `OS_CLOUD` env var is set, in which case it will read
`clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files).
para obtener el proyecto actual en uso.

### Opciones

| Opción     | Predeterminado                                  | Descripción                                                                 |
| ---------- | ----------------------------------------------- | --------------------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | El formato del módulo.                                      |
| `simbol`   | `'☁️ '`                                         | El símbolo usado antes de mostrar la nube OpenStack actual. |
| `style`    | `'bold yellow'`                                 | El estilo del módulo.                                       |
| `disabled` | `false`                                         | Disables the `openstack` module.                            |

### Variables

| Variable | Ejemplo | Descripción                            |
| -------- | ------- | -------------------------------------- |
| nube     | `corp`  | La nube OpenStack actual               |
| proyecto | `dev`   | El actual proyecto OpenStack           |
| symbol   |         | Refleja el valor de la opción `symbol` |
| style\*  |         | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## SO

The `os` module shows the current operating system.
OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

> [!WARNING]
> The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción     | Predeterminado        | Descripción                                                            |
| ---------- | --------------------- | ---------------------------------------------------------------------- |
| `format`   | `'[$symbol]($style)'` | El formato del módulo.                                 |
| `style`    | `'bold white'`        | El estilo del módulo.                                  |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type.
Operating system types not defined by your configuration use the default symbols table below.
All operating systems currently supported by the module are listed below.
If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
[os.symbols]
AIX = "➿ "
Alpaquita = "🔔 "
AlmaLinux = "💠 "
Alpine = "🏔️ "
ALTLinux = "Ⓐ "
Amazon = "🙂 "
Android = "🤖 "
AOSC = "🐱 "
Arch = "🎗️ "
Artix = "🎗️ "
Bluefin = "🐟 "
CachyOS = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Elementary = "🍏 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Ios = "📱 "
InstantOS = "⏲️ "
Kali = "🐉 "
Linux = "🐧 "
Mabox = "📦 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
Nobara = "🎩 "
OpenBSD = "🐡 "
OpenCloudOS = "☁️ "
openEuler = "🦉 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
PikaOS = "🐤 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
RockyLinux = "💠 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Ultramarine = "🔷 "
Unknown = "❓ "
Uos = "🐲 "
Void = " "
Windows = "🪟 "
Zorin = "🔹 "
```

### Variables

| Variable | Ejemplo      | Descripción                                                        |
| -------- | ------------ | ------------------------------------------------------------------ |
| symbol   | `🎗️`        | The current operating system symbol from advanced option `symbols` |
| name     | `Arch Linux` | El nombre actual del sistema operativo                             |
| tipo     | `Arch`       | El tipo actual de sistema operativo                                |
| codename |              | The current operating system codename, if applicable               |
| edición  |              | The current operating system edition, if applicable                |
| version  |              | The current operating system version, if applicable                |
| style\*  |              | Refleja el valor de la opción `style`                              |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[os]
format = "on [($name )]($style)"
style = "bold blue"
disabled = false

[os.symbols]
Windows = " "
Arch = "Arch es lo mejor! "
```

## Package Version

The `package` module is shown when the current directory is the repository for a
package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`,
`poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `galaxy`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present
  in the current directory
- [**JSR**](https://jsr.io/) – The `jsr` package version is extracted from the `jsr.json`/`jsr.jsonc` or `deno.json`/`deno.jsonc` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present
  in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present
  in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**Galaxy**](https://galaxy.ansible.com/) - The `galaxy` package version is extracted from the `galaxy.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ The version being shown is that of the package whose source code is in your
> current directory, not your package manager.

### Opciones

| Opción            | Predeterminado                    | Descripción                                                                                                      |
| ----------------- | --------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`          | `'is [$symbol$version]($style) '` | El formato del módulo.                                                                           |
| `simbol`          | `'📦 '`                           | El símbolo usado antes de mostrar la versión del paquete.                                        |
| `version_format`  | `'v${raw}'`                       | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | El estilo del módulo.                                                                            |
| `display_private` | `false`                           | Activar la visualización de la versión para los paquetes marcados como privados.                 |
| `disabled`        | `false`                           | Disables the `package` module.                                                                   |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v1.0.0` | La versión de su paquete               |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[package]
format = 'vía [🎁 $version](208 bold) '
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Opciones

| Opción              | Predeterminado                                                                                           | Descripción                                                                                                      |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | La cadena de formato para el módulo.                                                             |
| `version_format`    | `'v${raw}'`                                                                                              | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🐪 '`                                                                                                  | El símbolo usado antes de mostrar la versión de Perl                                                             |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                                     | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold 149'`                                                                                             | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                                                      |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| version  | `v5.26.1` | The version of `perl`                  |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🐘 '`                              | El símbolo usado antes de mostrar la versión de PHP.                                             |
| `detect_extensions` | `['php']`                            | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['composer.json', '.php-version']`  | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'147 bold'`                         | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `php` module.                                                                       |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v7.3.8` | The version of `php`                   |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Canal Pijul

The `pijul_channel` module shows the active channel of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción              | Predeterminado                    | Descripción                                                                                          |
| ------------------- | --------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `simbol`            | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | El estilo del módulo.                                                                |
| `format`            | `'on [$symbol$channel]($style) '` | El formato del módulo.                                                               |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                                    |
| `truncation_symbol` | `'…'`                             | El símbolo usado para indicar que un nombre de rama fue truncado.                    |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |

## Pixi

The `pixi` module shows the installed [pixi](https://pixi.sh) version as well as the activated
environment and project name, if `$PIXI_ENVIRONMENT_NAME` is set.

> [!TIP]
> This does not suppress pixi's own prompt modifier, you may want to run `pixi config set shell.change-ps1 false`.

### Opciones

| Opción                     | Predeterminado                                            | Descripción                                                                                                                       |
| -------------------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `format`                   | `'via [$symbol($version )(\($environment\) )]($style)'` | El formato del módulo.                                                                                            |
| `version_format`           | `'v${raw}'`                                               | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch`. |
| `simbol`                   | `'🧚 '`                                                   | El símbolo usado antes del nombre del entorno.                                                                    |
| `style`                    | `'yellow bold'`                                           | El estilo del módulo.                                                                                             |
| `show_default_environment` | `true`                                                    | Whether to indicate that the `default` environment of your project is activated.                                  |
| `pixi_binary`              | `['pixi']`                                                | Configures the pixi binary that Starship should execute when getting the version.                                 |
| `detect_extensions`        | `[]`                                                      | Qué extensiones deberían activar este módulo.                                                                     |
| `detect_files`             | `['pixi.toml']`                                           | Qué nombres de archivo deberían activar este módulo.                                                              |
| `detect_folders`           | `[]`                                                      | Qué carpetas deberían activar este módulo.                                                                        |
| `disabled`                 | `false`                                                   | Disables the `pixi` module.                                                                                       |

### Variables

| Variable                          | Ejemplo      | Descripción                            |
| --------------------------------- | ------------ | -------------------------------------- |
| version                           | `v0.33.0`    | The version of `pixi`                  |
| environment                       | `py311`      | The current pixi environment           |
| project_name | `my-project` | The current pixi project name          |
| symbol                            |              | Refleja el valor de la opción `symbol` |
| style                             |              | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[pixi]
format = '[$symbol$environment](yellow) '
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

> [!TIP]
> By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms).
> If you still want to enable it, [follow the example shown below](#with-pulumi-version).

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Opciones

| Opción           | Predeterminado                               | Descripción                                                                                                      |
| ---------------- | -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`         | `'vía [$symbol($username@)$stack]($style) '` | La cadena de formato para el módulo.                                                             |
| `version_format` | `'v${raw}'`                                  | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`         | `' '`                                       | Una cadena de formato que se muestra antes de la pila de Pulumi.                                 |
| `style`          | `'bold 5'`                                   | El estilo del módulo.                                                                            |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.                                   |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                                                    |

### Variables

| Variable          | Ejemplo    | Descripción                            |
| ----------------- | ---------- | -------------------------------------- |
| version           | `v0.12.24` | The version of `pulumi`                |
| stack             | `dev`      | La pila actual de Pulumi               |
| nombre de usuario | `alice`    | El usuario actual de Pulumi            |
| symbol            |            | Refleja el valor de la opción `symbol` |
| style\*           |            | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

#### Con la versión de Pulumi

```toml
# ~/.config/starship.toml

[pulumi]
format = '[🛥 ($version )$stack]($style) '
```

#### Sin versión de Pulumi

```toml
# ~/.config/starship.toml
[pulumi]
symbol = '🛥 '
format = '[$symbol$stack]($style) '
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version.
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `spago.dhall` file
- The current directory contains a `spago.yaml` file
- The current directory contains a `spago.lock` file
- The current directory contains a file with the `.purs` extension

### Opciones

| Opción              | Predeterminado                                | Descripción                                                                                                      |
| ------------------- | --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                                   | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'<=> '`                                      | El símbolo usado antes de mostrar la versión de PureScript.                                      |
| `detect_extensions` | `['purs']`                                    | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                          | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold white'`                                | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                       | Disables the `purescript` module.                                                                |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `0.13.5` | The version of `purescript`            |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the
current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version
name. Otherwise, it will display the version number from `python --version`.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- The current directory contains a file with the `.ipynb` extension.
- Un entorno virtual está activado actualmente

### Opciones

| Opción               | Predeterminado                                                                                               | Descripción                                                                                                      |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | El formato del módulo.                                                                           |
| `version_format`     | `'v${raw}'`                                                                                                  | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`             | `'🐍 '`                                                                                                      | Una cadena de formato que representa el símbolo de Python                                                        |
| `style`              | `'yellow bold'`                                                                                              | El estilo del módulo.                                                                            |
| `pyenv_version_name` | `false`                                                                                                      | Usar pyenv para obtener la versión de Python                                                                     |
| `pyenv_prefix`       | `'pyenv'`                                                                                                    | Prefijo antes de mostrar la versión de pyenv sólo se utiliza si se utiliza pyenv                                 |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should execute when getting the version.            |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                            | Qué extensiones deben activar este módulo                                                                        |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Qué nombres de archivo deben activar este módulo                                                                 |
| `detect_folders`     | `[]`                                                                                                         | Qué carpetas deben activar este módulo                                                                           |
| `generic_venv_names` | `[]`                                                                                                         | Which venv names should be replaced with the parent directory name.                              |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                                    |

> [!TIP]
> The `python_binary` variable accepts either:
>
> - a string (e.g. `'python3'`),
> - a list of strings (e.g. `['python', 'python3']`)
> - a list of lists of strings, representing commands with optional arguments (e.g.
>   `[['mise', 'exec', '--', 'python'], ['python3']]`)
>
> Starship will try executing each configured command until it gets a result.
> Note you can only change the binary that Starship executes to get the version
> of Python not the arguments that are used.
>
> The default values and order for `python_binary` was chosen to first identify
> the Python version in a virtualenv/conda environments (which currently still
> add a `python`, no matter if it points to `python3` or `python2`). This has the
> side effect that if you still have a system Python 2 installed, it may be
> picked up before any Python 3 (at least on Linux Distros that always symlink
> `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but
> cannot remove the system Python 2, changing this to `'python3'` will hide any
> Python version 2, see example below.

### Variables

| Variable                          | Ejemplo         | Descripción                                                                 |
| --------------------------------- | --------------- | --------------------------------------------------------------------------- |
| version                           | `'v3.8.1'`      | The version of `python`                                                     |
| symbol                            | `'🐍 '`         | Refleja el valor de la opción `symbol`                                      |
| style                             | `'yellow bold'` | Refleja el valor de la opción `style`                                       |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix`                                  |
| virtualenv                        | `'venv'`        | The current `virtualenv` name or the parent if matches `generic_venv_names` |

### Ejemplo

```toml
# ~/.config/starship.toml

[python]
symbol = '👾 '
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# Use `mise` to get the version.
python_binary = [['mise', 'exec', '--', 'python']]
```

```toml
# ~/.config/starship.toml

[python]
# Potentially dangerous: `uv` can run any binary at `.venv/bin/python` without interaction
python_binary = [['uv', 'run', '--no-python-downloads', '--no-project', 'python']]
```

```toml
# ~/.config/starship.toml

[python]
# No se dispara con archivos con extensión py
detect_extensions = []
```

## Quarto

The `quarto` module shows the current installed version of Quarto used in a project.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `_quarto.yml` file
- The current directory contains any `*.qmd` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'⨁ '`                               | A format string representing the symbol of Quarto                                                                |
| `style`             | `'bold #75AADB'`                     | El estilo del módulo.                                                                            |
| `detect_extensions` | `['.qmd']`                           | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['_quarto.yml']`                    | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `disabled`          | `false`                              | Disables the `quarto` module.                                                                    |

### Variables

| Variable | Ejemplo   | Descripción                            |
| -------- | --------- | -------------------------------------- |
| version  | `1.4.549` | The version of `quarto`                |
| symbol   |           | Refleja el valor de la opción `symbol` |
| style\*  |           | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). El módulo se mostrará si
se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'📐'`                               | Una cadena de formato que representa el símbolo de R.                                            |
| `style`             | `'blue bold'`                        | El estilo del módulo.                                                                            |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Qué extensiones deben activar este módulo                                                                        |
| `detect_files`      | `['.Rprofile']`                      | Qué nombres de archivo deben activar este módulo                                                                 |
| `detect_folders`    | `['.Rproj.user']`                    | Qué carpetas deben activar este módulo                                                                           |
| `disabled`          | `false`                              | Disables the `r` module.                                                                         |

### Variables

| Variable | Ejemplo       | Descripción                            |
| -------- | ------------- | -------------------------------------- |
| version  | `v4.0.5`      | The version of `R`                     |
| symbol   |               | Refleja el valor de la opción `symbol` |
| style    | `'blue bold'` | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Opciones

| Opción              | Predeterminado                                   | Descripción                                                                                                      |
| ------------------- | ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | La cadena de formato para el módulo.                                                             |
| `version_format`    | `'v${raw}'`                                      | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🦋 '`                                          | El símbolo usado antes de mostrar la versión de Raku                                                             |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['META6.json']`                                 | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                             | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold 149'`                                     | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                          | Disables the `raku` module.                                                                      |

### Variables

| Variable                        | Ejemplo | Descripción                            |
| ------------------------------- | ------- | -------------------------------------- |
| version                         | `v6.d`  | The version of `raku`                  |
| vm_version | `moar`  | The version of VM `raku` is built on   |
| symbol                          |         | Refleja el valor de la opción `symbol` |
| style\*                         |         | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/).
El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- The current directory contains a file with `.red` or `.reds` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🔺 '`                              | Una cadena de formato que representa el símbolo de Red.                                          |
| `detect_extensions` | `['red']`                            | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'red bold'`                         | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `red` module.                                                                       |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.5.1` | The version of `red`                   |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/).
El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'💎 '`                              | Una cadena de formato que representa el símbolo de Ruby.                                         |
| `detect_extensions` | `['rb']`                             | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Qué variables de entorno deben activar este módulo.                                              |
| `style`             | `'bold red'`                         | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `ruby` module.                                                                      |

### Variables

| Variable | Ejemplo  | Descripción                                                 |
| -------- | -------- | ----------------------------------------------------------- |
| version  | `v2.5.1` | The version of `ruby`                                       |
| symbol   |          | Refleja el valor de la opción `symbol`                      |
| style\*  |          | Refleja el valor de la opción `style`                       |
| gemset   | `test`   | Optional, gets the current RVM gemset name. |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/).
El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🦀 '`                              | Una cadena de formato que representa el símbolo de Rust                                                          |
| `detect_extensions` | `['rs']`                             | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['Cargo.toml']`                     | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold red'`                         | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `rust` module.                                                                      |

### Variables

| Variable  | Ejemplo           | Descripción                                  |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | La versión de toolchain                      |
| symbol    |                   | Refleja el valor de la opción `symbol`       |
| style\*   |                   | Refleja el valor de la opción `style`        |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Opciones

| Opción              | Predeterminado                           | Descripción                                                                                                      |
| ------------------- | ---------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                              | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `['.metals']`                            | Qué carpetas deberían activar este módulo.                                                       |
| `simbol`            | `'🆂 '`                                  | Una cadena de formato que representa el símbolo de Scala.                                        |
| `style`             | `'red dimmed'`                           | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                  | Disables the `scala` module.                                                                     |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `2.13.5` | The version of `scala`                 |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

The `shell` module shows an indicator for currently used shell.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción                 | Predeterminado            | Descripción                                                                                                                            |
| ---------------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `bash_indicator`       | `'bsh'`                   | Una cadena de formato usada para representar bash.                                                                     |
| `fish_indicator`       | `'fsh'`                   | Una cadena de formato usada para representar fish.                                                                     |
| `zsh_indicator`        | `'zsh'`                   | Una cadena de formato usada para representar zsh.                                                                      |
| `powershell_indicator` | `'psh'`                   | Una cadena de formato usada para representar powershell.                                                               |
| `pwsh_indicator`       |                           | A format string used to represent pwsh. The default value mirrors the value of `powershell_indicator`. |
| `ion_indicator`        | `'ion'`                   | Una cadena de formato usada para representar ion.                                                                      |
| `elvish_indicator`     | `'esh'`                   | Una cadena de formato usada para representar elvish.                                                                   |
| `tcsh_indicator`       | `'tsh'`                   | A format string used to represent tcsh.                                                                                |
| `xonsh_indicator`      | `'xsh'`                   | Una cadena de formato usada para representar xonsh.                                                                    |
| `cmd_indicator`        | `'cmd'`                   | Una cadena de formato usada para representar cmd.                                                                      |
| `nu_indicator`         | `'nu'`                    | Una cadena de formato usada para representar nu.                                                                       |
| `unknown_indicator`    | `''`                      | El valor por defecto que se mostrará cuando se desconoce el intérprete.                                                |
| `format`               | `'[$indicator]($style) '` | El formato del módulo.                                                                                                 |
| `style`                | `'white bold'`            | El estilo del módulo.                                                                                                  |
| `disabled`             | `true`                    | Disables the `shell` module.                                                                                           |

### Variables

| Variable  | Predeterminado | Descripción                                                                |
| --------- | -------------- | -------------------------------------------------------------------------- |
| indicador |                | Mirrors the value of `indicator` for currently used shell. |
| style\*   |                | Refleja el valor de la opción `style`.                     |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplos

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = '󰈺 '
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is
set to a number and meets or exceeds the specified threshold.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción          | Predeterminado               | Descripción                                                                   |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------- |
| `threshold`     | `2`                          | Mostrar el umbral.                                            |
| `format`        | `'[$symbol$shlvl]($style) '` | El formato del módulo.                                        |
| `simbol`        | `'↕️  '`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`        | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value           |
| `style`         | `'bold yellow'`              | El estilo del módulo.                                         |
| `disabled`      | `true`                       | Disables the `shlvl` module.                                  |

### Variables

| Variable | Ejemplo | Descripción                            |
| -------- | ------- | -------------------------------------- |
| shlvl    | `3`     | The current value of `SHLVL`           |
| symbol   |         | Refleja el valor de la opción `symbol` |
| style\*  |         | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

Using `repeat` and `repeat_offset` along with `character` module, one can get
prompt like `❯❯❯` where last character is colored appropriately for return
status code and preceding characters are provided by `shlvl`.

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '[$symbol]($style)'
repeat = true
symbol = '❯'
repeat_offset = 1
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container
and `$SINGULARITY_NAME` is set.

### Opciones

| Opción     | Predeterminado                   | Descripción                                                                         |
| ---------- | -------------------------------- | ----------------------------------------------------------------------------------- |
| `format`   | `'[$symbol\[$env\]]($style) '` | El formato del módulo.                                              |
| `simbol`   | `''`                             | Una cadena de formato que se muestra antes del nombre de la imagen. |
| `style`    | `'bold dimmed blue'`             | El estilo del módulo.                                               |
| `disabled` | `false`                          | Disables the `singularity` module.                                  |

### Variables

| Variable | Ejemplo      | Descripción                            |
| -------- | ------------ | -------------------------------------- |
| env      | `centos.img` | La imagen de Singularity actual        |
| symbol   |              | Refleja el valor de la opción `symbol` |
| style\*  |              | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Solidity

The `solidity` module shows the currently installed version of [Solidity](https://soliditylang.org/)
The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.sol` extension

### Opciones

| Opción              | Predeterminado                                               | Descripción                                                                                                      |
| ------------------- | ------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                         | El formato del módulo.                                                                           |
| `version_format`    | `'v${major}.${minor}.${patch}'`                              | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'S '`                                                       | A format string representing the symbol of Solidity                                                              |
| \`compiler          | ['solc'] | The default compiler for Solidity.                                                               |
| `detect_extensions` | `['sol']`                                                    | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                                         | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                                         | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold blue'`                                                | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                                      | Disables this module.                                                                            |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v0.8.1` | The version of `solidity`              |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml
[solidity]
format = "via [S $version](blue bold)"
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Opciones

| Opción              | Predeterminado                         | Descripción                                                                                                                                                                                    |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | El número de directorios a los que se debe truncar la ruta de entorno. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `simbol`            | `'🅢  '`                               | El símbolo usado antes del nombre del entorno.                                                                                                                                 |
| `style`             | `'bold blue'`                          | El estilo del módulo.                                                                                                                                                          |
| `format`            | `'via [$symbol$environment]($style) '` | El formato del módulo.                                                                                                                                                         |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                                                   |

### Variables

| Variable    | Ejemplo      | Descripción                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | El entorno de spack actual             |
| symbol      |              | Refleja el valor de la opción `symbol` |
| style\*     |              | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Estado

The `status` module displays the exit code of the previous command.
If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`.
El código de estado se convertirá a un entero con signo de 32 bits.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción                      | Predeterminado                                                                   | Descripción                                                                                          |
| --------------------------- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                    | El formato del módulo                                                                                |
| `simbol`                    | `'❌'`                                                                            | El símbolo mostrado en el error del programa                                                         |
| `success_symbol`            | `''`                                                                             | El símbolo mostrado en el éxito del programa                                                         |
| `not_executable_symbol`     | `'🚫'`                                                                           | El símbolo mostrado cuando el archivo no es ejecutable                                               |
| `not_found_symbol`          | `'🔍'`                                                                           | El símbolo mostrado cuando no se encuentra el comando                                                |
| `sigint_symbol`             | `'🧱'`                                                                           | El símbolo mostrado en SIGINT (Ctrl + c)                                          |
| `signal_symbol`             | `'⚡'`                                                                            | El símbolo mostrado en cualquier señal                                                               |
| `style`                     | `'bold red'`                                                                     | El estilo del módulo.                                                                |
| `success_style`             |                                                                                  | The style used on program success (defaults to `style` if unset). |
| `failure_style`             |                                                                                  | The style used on program failure (defaults to `style` if unset). |
| `recognize_signal_code`     | `true`                                                                           | Habilita el mapeo de señales desde el código de salida                                               |
| `map_symbol`                | `false`                                                                          | Habilita el mapeo de símbolos desde el código de salida                                              |
| `pipestatus`                | `false`                                                                          | Habilita el reporte de pipstatus                                                                     |
| `pipestatus_separator`      | <code>&vert;</code>                                          | El símbolo usado para separar segmentos de pipestatus (soporta formato)           |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | El formato del módulo cuando el comando es un pipeline                                               |
| `pipestatus_segment_format` |                                                                                  | When specified, replaces `format` when formatting pipestatus segments                                |
| `disabled`                  | `true`                                                                           | Disables the `status` module.                                                        |

### Variables

| Variable                            | Ejemplo | Descripción                                                                                                     |
| ----------------------------------- | ------- | --------------------------------------------------------------------------------------------------------------- |
| estado                              | `127`   | El código de salida del último comando                                                                          |
| hex_status     | `0x7F`  | El código de salida del último comando en hexadecimal                                                           |
| int                                 | `127`   | El código de salida del último comando                                                                          |
| common_meaning | `ERROR` | Comprobación del código si no es una señal                                                                      |
| signal_number  | `9`     | Número de señal correspondiente al código de salida, sólo si está señalizado                                    |
| signal_name    | `KILL`  | Nombre de la señal correspondiente al código de salida, sólo si está señalizada                                 |
| maybe_int      | `7`     | Contiene el número de código de salida cuando no se ha encontrado ningún significado                            |
| pipestatus                          |         | Rendering of in pipeline programs' exit codes, this is only available in pipestatus_format |
| symbol                              |         | Refleja el valor de la opción `symbol`                                                                          |
| style\*                             |         | Mirrors the value of option `success_style` on program success and `failure_style` otherwise                    |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[status]
style = 'bg:blue'
symbol = '🔴 '
success_symbol = '🟢 SUCCESS'
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

The `sudo` module displays if sudo credentials are currently cached.
El módulo solo se mostrará si las credenciales están guardadas en caché.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción          | Predeterminado           | Descripción                                                                                      |
| --------------- | ------------------------ | ------------------------------------------------------------------------------------------------ |
| `format`        | `'[as $symbol]($style)'` | El formato del módulo                                                                            |
| `simbol`        | `'🧙 '`                  | El símbolo mostrado cuando las credenciales se almacenan en caché                                |
| `style`         | `'bold blue'`            | El estilo del módulo.                                                            |
| `allow_windows` | `false`                  | Como Windows no tiene sudo por defecto, el valor por defecto está deshabilitado. |
| `disabled`      | `true`                   | Disables the `sudo` module.                                                      |

### Variables

| Variable | Ejemplo | Descripción                            |
| -------- | ------- | -------------------------------------- |
| symbol   |         | Refleja el valor de la opción `symbol` |
| style\*  |         | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[sudo]
style = 'bold green'
symbol = '👩‍💻 '
disabled = false
```

```toml
# On windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/).
El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'🐦 '`                              | Una cadena de formato que representa el símbolo de Swift                                                         |
| `detect_extensions` | `['swift']`                          | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['Package.swift']`                  | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold 202'`                         | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `swift` module.                                                                     |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v5.2.4` | The version of `swift`                 |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[swift]
format = 'vía [🏎  $version](red bold)'
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.
It supports both Hashicorp Terraform and OpenTofu for version detection.

> [!TIP]
> By default the Terraform/OpenTofu version is not shown, since this is slow for current versions when a lot of plugins are in use.
> If you still want to enable it, [follow the example shown below](#with-terraform-version).

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Opciones

| Opción              | Predeterminado                                          | Descripción                                                                                                      |
| ------------------- | ------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '`                    | La cadena de formato para el módulo.                                                             |
| `version_format`    | `'v${raw}'`                                             | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'💠'`                                                  | Una cadena de formato que se muestra antes del espacio de trabajo terraform.                     |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`                           | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                                    | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `['.terraform']`                                        | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'bold 105'`                                            | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                                 | Disables the `terraform` module.                                                                 |
| `commands`          | `[ [ 'terraform', 'version' ], [ 'tofu', 'version' ] ]` | How to detect what the Terraform version is.                                                     |

### Variables

| Variable        | Ejemplo          | Descripción                               |
| --------------- | ---------------- | ----------------------------------------- |
| version         | `v0.12.24`       | The version of `terraform`                |
| área de trabajo | `predeterminado` | El espacio de trabajo actual de Terraform |
| symbol          |                  | Refleja el valor de la opción `symbol`    |
| style\*         |                  | Refleja el valor de la opción `style`     |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

#### Con Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$version $workspace]($style) '
```

#### Sin Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$workspace]($style) '
```

## Time

The `time` module shows the current **local** time.
The `format` configuration value is used by the [`jiff`](https://crates.io/crates/jiff) crate to control how the time is displayed. Take a look [at the jiff strftime docs](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) to see what options are available.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opciones

| Opción            | Predeterminado          | Descripción                                                                                                                                                                                                                                                                                  |
| ----------------- | ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | La cadena de formato para el módulo.                                                                                                                                                                                                                                         |
| `use_12hr`        | `false`                 | Habilita el formato de 12 horas                                                                                                                                                                                                                                                              |
| `time_format`     | see below               | The [jiff format string](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) used to format the time.                                                                                                                                                                   |
| `style`           | `'bold yellow'`         | El estilo para el módulo de time                                                                                                                                                                                                                                                             |
| `utc_time_offset` | `'local'`               | Establece el desplazamiento UTC a utilizar. Either an IANA time zone name or a range from -24 &lt; x &lt; 24. Permite a los flotantes acomodar los desplazamientos de zona horaria de 30/45 minutos. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                                                                                                                                                                  |
| `time_range`      | `'-'`                   | Establece el intervalo de tiempo durante el cual se mostrará el módulo. Las horas deben especificarse en formato de 24 horas                                                                                                                                                 |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`.
Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable | Ejemplo    | Descripción                           |
| -------- | ---------- | ------------------------------------- |
| tiempo   | `13:08:10` | La hora actual.       |
| style\*  |            | Refleja el valor de la opción `style` |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

#### With UTC offset

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = '%T'
utc_time_offset = '-5'
time_range = '10:00:00-14:00:00'
```

#### With Timezone name

```toml
# ~/.config/starship.toml

[time]
disabled = false
time_format = '%T'
utc_time_offset = 'Europe/Berlin'
```

## Typst

The `typst` module shows the current installed version of Typst used in a project.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `template.typ` file
- The current directory contains any `*.typ` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'t '`                               | A format string representing the symbol of Typst                                                                 |
| `style`             | `'bold #0093A7'`                     | El estilo del módulo.                                                                            |
| `detect_extensions` | `['.typ']`                           | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['template.typ']`                   | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `disabled`          | `false`                              | Disables the `typst` module.                                                                     |

### Variables

| Variable                           | Ejemplo          | Descripción                                                          |
| ---------------------------------- | ---------------- | -------------------------------------------------------------------- |
| version                            | `v0.9.0`         | The version of `typst`, alias for typst_version |
| typst_version | `predeterminado` | The current Typst version                                            |
| symbol                             |                  | Refleja el valor de la opción `symbol`                               |
| style\*                            |                  | Refleja el valor de la opción `style`                                |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## Username

The `username` module shows active user's username.
El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- El usuario actual es root/admin
- El usuario actual no es el mismo que el que está conectado
- El usuario está actualmente conectado como una sesión SSH
- The variable `show_always` is set to true
- The array `detect_env_vars` contains at least the name of one environment variable, that is set

> [!TIP]
> SSH connection is detected by checking environment variables
> `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. Si tu host SSH no configura estas variables, una solución es establecer una de ellas con un valor tonto.

### Opciones

| Opción            | Predeterminado          | Descripción                                                                              |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------- |
| `style_root`      | `'bold red'`            | El estilo usado cuando el usuario es root/admin.                         |
| `style_user`      | `'bold yellow'`         | El estilo usado para usuarios no root.                                   |
| `detect_env_vars` | `[]`                    | Qué variable(s) de entorno deben activar este módulo. |
| `format`          | `'[$user]($style) in '` | El formato del módulo.                                                   |
| `show_always`     | `false`                 | Always shows the `username` module.                                      |
| `disabled`        | `false`                 | Disables the `username` module.                                          |
| `aliases`         | `{}`                    | Translate system usernames to something else.                            |

### Variables

| Variable | Ejemplo      | Descripción                                                                                                 |
| -------- | ------------ | ----------------------------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `'matchai'`  | El ID de usuario conectado actualmente.                                                     |

### Ejemplo

#### Always show the username

```toml
# ~/.config/starship.toml

[username]
style_user = 'white bold'
style_root = 'black bold'
format = 'user: [$user]($style) '
disabled = false
show_always = true
aliases = { "corpuser034g" = "matchai" }
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `Vagrantfile` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'⍱ '`                               | Una cadena de formato que representa el símbolo de Vagrant.                                      |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['Vagrantfile']`                    | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'cyan bold'`                        | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                                                   |

### Variables

| Variable | Ejemplo          | Descripción                            |
| -------- | ---------------- | -------------------------------------- |
| version  | `Vagrant 2.2.10` | The version of `Vagrant`               |
| symbol   |                  | Refleja el valor de la opción `symbol` |
| style\*  |                  | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/).
Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Opciones

| Opción              | Predeterminado                               | Descripción                                                                                                      |
| ------------------- | -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`         | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                                  | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'V '`                                       | Una cadena de formato que representa el símbolo de V                                                             |
| `detect_extensions` | `['v']`                                      | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                         | Qué carpetas deberían activar este módulo.                                                       |
| `style`             | `'blue bold'`                                | El estilo del módulo.                                                                            |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                                                     |

### Variables

| Variable | Ejemplo | Descripción                            |
| -------- | ------- | -------------------------------------- |
| version  | `v0.2`  | The version of `v`                     |
| symbol   |         | Refleja el valor de la opción `symbol` |
| style\*  |         | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCS

> Note the module is enabled by default but **not** included in the default list because that would be a breaking change.
> Additionally, the exact format of the module may change in the future, for example to handle right-aligned prompt.

The `vcs` module displays the current active Version Control System (VCS).
The module will be shown only if a configured VCS is currently in use.

### Opciones

| Opción           | Predeterminado                                              | Descripción                                                           |
| ---------------- | ----------------------------------------------------------- | --------------------------------------------------------------------- |
| `order`          | `["git", "hg", "pijul", "fossil"]`                          | The order in which to search VCSes.                   |
| `fossil_modules` | `"$fossil_branch$fossil_metrics"`                           | Modules to show when a Fossil repository is found.    |
| `git_modules`    | `"$git_branch$git_commit$git_state$git_metrics$git_status"` | Modules to show when a Git repository is found.       |
| `hg_modules`     | `"$hg_branch$hg_state"`                                     | Modules to show when a Mercurial repository is found. |
| `pijul_modules`  | `"$pijul_channel"`                                          | Modules to show when a Pijul repository is found.     |
| `disabled`       | `false`                                                     | Disables the `vcs` module.                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[vcs]
# Will look for Git then Pijul if not found but not for other VCSes at all
order = [
  "git",
  "pijul",
]
# Any module (except `$vcs` itself to avoid infinite loops) can be included here
git_modules = "$git_branch${custom.foo}"

# See documentation for custom modules
[custom.foo]
command = 'echo foo'
detect_files = ['foo']
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository.
El módulo sólo se mostrará si un repositorio está actualmente en uso.

### Opciones

| Opción     | Predeterminado                   | Descripción                                                                  |
| ---------- | -------------------------------- | ---------------------------------------------------------------------------- |
| `simbol`   | `''`                             | El símbolo usado antes de mostrar el nombre del repositorio. |
| `style`    | `'bold yellow'`                  | El estilo del módulo.                                        |
| `format`   | `'vcsh [$symbol$repo]($style) '` | El formato del módulo.                                       |
| `disabled` | `false`                          | Disables the `vcsh` module.                                  |

### Variables

| Variable | Ejemplo                                     | Descripción                            |
| -------- | ------------------------------------------- | -------------------------------------- |
| repo     | `dotfiles` if in a VCSH repo named dotfiles | El nombre del repositorio activo       |
| symbol   |                                             | Refleja el valor de la opción `symbol` |
| style\*  | `black bold dimmed`                         | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## XMake

The `xmake` module shows the currently installed version of [XMake](https://xmake.io/). Por defecto el módulo se activará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `xmake.lua` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'△ '`                               | El símbolo usado antes de la versión de cmake.                                                   |
| `detect_extensions` | `[]`                                 | Qué extensiones deben activar este módulo                                                                        |
| `detect_files`      | `['xmake.lua']`                      | Qué nombres de archivo deben activar este módulo                                                                 |
| `detect_folders`    | `[]`                                 | Qué carpetas deben activar este módulo                                                                           |
| `style`             | `'bold green'`                       | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `xmake` module.                                                                     |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v2.9.5` | The version of xmake                   |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## Zig

By default the `zig` module shows the currently installed version of [Zig](https://ziglang.org/).
El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- The current directory contains a `.zig` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                           |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Available vars are `raw`, `major`, `minor`, & `patch` |
| `simbol`            | `'↯ '`                               | El símbolo usado antes de mostrar la versión de Zig.                                             |
| `style`             | `'bold yellow'`                      | El estilo del módulo.                                                                            |
| `disabled`          | `false`                              | Disables the `zig` module.                                                                       |
| `detect_extensions` | `['zig']`                            | Qué extensiones deberían activar este módulo.                                                    |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                                       |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v0.6.0` | The version of `zig`                   |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style\*  |          | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## Comandos personalizados

The `custom` modules show the output of some arbitrary commands.

Estos módulos se mostrarán si se cumple alguna de las siguientes condiciones:

- The current directory contains a file whose name is in `detect_files`
- The current directory contains a directory whose name is in `detect_folders`
- The current directory contains a file whose extension is in `detect_extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

> [!TIP]
> Multiple custom modules can be defined by using a `.`.

> [!TIP]
> The order in which custom modules are shown can be individually set by including
> `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `custom` module will simply show all custom modules in the order they were defined.

> [!TIP]
> [Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules.
> ¡Si tienes un ejemplo interesante no cubierto, siéntete libre de compartirlo ahí!

> [!WARNING]
> If `unsafe_no_escape` is enabled or prior to starship v1.20 command output is printed unescaped to the prompt.
>
> Cualquiera que sea la salida que genere el comando se imprime sin modificar en el prompt. This means if the output
> contains shell-specific interpretable sequences, they could be interpreted on display.
> Depending on the shell, this can mean that e.g. strings enclosed by backticks are executed by the shell.
> Such sequences are usually shell specific, e.g. you can write a command module that writes bash sequences,
> e.g. `\h`, but this module will not work in a fish or zsh shell.
>
> Format strings can also contain shell specific prompt sequences, e.g.
> [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html),
> [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

### Opciones

| Opción              | Predeterminado                  | Descripción                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | El comando cuya salida debe ser impresa. El comando se pasará en stdin al intérprete de comandos.                                                                                                                                                                                                                                                                   |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the `shell` returns a `0` status code from executing it.                                                                                                           |
| `require_repo`      | `false`                         | If `true`, the module will only be shown in paths containing a (git) repository. This option alone is not sufficient display condition in absence of other options.                                                                                                                                                                              |
| `shell`             |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                                                                                                                                  |
| `descripción`       | `'<custom module>'`             | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                                                                                                        |
| `unsafe_no_escape`  | `false`                         | When set, command output is not escaped of characters that could be interpreted by the shell.                                                                                                                                                                                                                                                                                       |
| `detect_files`      | `[]`                            | Los archivos que se buscarán en el directorio de trabajo para obtener una coincidencia.                                                                                                                                                                                                                                                                                             |
| `detect_folders`    | `[]`                            | Los directorios que se buscarán en el directorio de trabajo para una coincidencia.                                                                                                                                                                                                                                                                                                  |
| `detect_extensions` | `[]`                            | Las extensiones que se buscarán en el directorio de trabajo para obtener una coincidencia.                                                                                                                                                                                                                                                                                          |
| `simbol`            | `''`                            | El símbolo usado antes de mostrar la salida del comando.                                                                                                                                                                                                                                                                                                                            |
| `style`             | `'bold green'`                  | El estilo del módulo.                                                                                                                                                                                                                                                                                                                                                               |
| `format`            | `'[$symbol($output )]($style)'` | El formato del módulo.                                                                                                                                                                                                                                                                                                                                                              |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                                                                                                      |
| `os`                |                                 | Nombre del sistema operativo en el que se mostrará el módulo (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                               |
| `use_stdin`         |                                 | Un valor booleano opcional que anula si los comandos deben ser reenviados al shell a través de la entrada estándar o como argumento. Si la entrada estándar unset es usada de manera predeterminada, a menos que el shell no lo soporte (cmd, nushell). Configurar esto desactiva el manejo de argumentos específicos del shell. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                                                                                                           |

### Variables

| Variable | Descripción                            |
| -------- | -------------------------------------- |
| salida   | The output of `command` run in `shell` |
| symbol   | Refleja el valor de la opción `symbol` |
| style\*  | Refleja el valor de la opción `style`  |

\*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

#### Comando personalizado del intérprete de comandos

`shell` accepts a non-empty list of strings, where:

- La primera cadena es la ruta al intérprete de comandos a usar para ejecutar el comando.
- Otros argumentos siguientes son pasados al intérprete de comandos.

If unset, it will fallback to STARSHIP_SHELL and then to 'sh' on Linux, and 'cmd /C' on Windows.

The `command` (and `when`, if applicable) will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used,
the following arguments will automatically be added: `-NoProfile -Command -`.
If `shell` is not given or only contains one element and Starship detects Cmd will be used,
the following argument will automatically be added: `/C` and `stdin` will be set to `false`.
If `shell` is not given or only contains one element and Starship detects Nushell will be used,
the following arguments will automatically be added: `-c` and `stdin` will be set to `false`.
Este comportamiento puede evitarse pasando explícitamente argumentos al intérprete de comandos, p.ej.

```toml
shell = ['pwsh', '-Command', '-']
```

> [!WARNING]
> Make sure your custom shell configuration exits gracefully
>
> If you set a custom command, make sure that the default Shell used by starship
> will properly execute the command with a graceful exit (via the `shell`
> option).
>
> For example, PowerShell requires the `-Command` parameter to execute a one
> liner. Omitir este parámetro puede arrojar a Starship a un bucle recursivo donde el intérprete de comandos podría intentar cargar un entorno de perfil completo con Starship en sí misma y volver a ejecutar el comando personalizado, entrando en un bucle infinito.
>
> Parameters similar to `-NoProfile` in PowerShell are recommended for other
> shells as well to avoid extra loading time of a custom profile on every
> starship invocation.
>
> La detección automática de intérpretes de comandos y la adición adecuada de parámetros están actualmente implementados, pero es posible que no todos los intérpretes de comandos estén cubiertos.
> [Please open an issue](https://github.com/starship/starship/issues/new/choose)
> with shell details and starship configuration if you hit such scenario.

### Ejemplo

```toml
# ~/.config/starship.toml

[custom.foo]
command = 'echo foo' # shows output of command
detect_files = ['foo'] # can specify filters but wildcards are not supported
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'

[custom.time]
command = 'time /T'
detect_extensions = ['pst'] # filters *.pst files
shell = ['pwsh.exe', '-NoProfile', '-Command', '-']

[custom.time-as-arg]
command = 'time /T'
detect_extensions = ['pst'] # filters *.pst files
shell = ['pwsh.exe', '-NoProfile', '-Command']
use_stdin = false
```
