# Configuración

Para iniciar la configuración de starship, crea el siguiente fichero: `~/.config.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toda la configuración de starship se incluye en este fichero [TOML](https://github.com/toml-lang/toml):

```toml
# Get editor completions based on the config schema
"$schema" = 'https://starship.rs/config-schema.json'

# Inserts a blank line between shell prompts
add_newline = true

# Replace the '❯' symbol in the prompt with '➜'
[character] # The name of the module we are configuring is 'character'
success_symbol = '[➜](bold green)' # The 'success_symbol' segment is being set to '➜' with the color 'bold green'

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

### Config File Location

Puedes cambiar la ubicación por defecto del archivo de configuración con la variable de entorno `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

El equivalente en PowerShell (Windows) es añadir esta línea a tu `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

O para Cmd (Windows) añadiría esta línea a su `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Registros

Starship registra por defecto los mensajes de advertencia y error en un fichero con nombre `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, en el que la clave de sesión corresponde con una instancia de tu terminal. Esto, sin embargo, puede ser cambiado usando la variable de entorno `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

El equivalente en PowerShell (Windows) es añadir esta línea a tu `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

O para Cmd (Windows) añadiría esta línea a su `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminología

**Módulo**: un componente en el promt que provee información basada en información contextual de tu sistema operativo. Por ejemplo, el módulo "nodejs" muestra la versión de NodeJS que tienes actualmente instalada en tu ordenador, si el directorio actual es un proyecto NodeJS.

**Variable**: subcomponentes más pequeños que contienen información proporcionada por el módulo. Por ejemplo, la variable "version" en el módulo "nodejs" contiene la versión actual de NodeJS.

Por convención, la mayoría de los módulos tienen un prefijo del color por defecto de la terminal (por ejemplo, `vía` en "nodejs") y un espacio vacío como sufijo.

### Strings

In TOML syntax, [text values](https://toml.io/en/v1.0.0#string) are declared with `'`, `"`, `'''`, or `"""`.

The following Starship syntax symbols have special usage in a format string and must be escaped to display as that character: `$ [ ] ( )`.

| Symbol | Type                      | Notes                                                  |
| ------ | ------------------------- | ------------------------------------------------------ |
| `'`    | literal string            | less escaping                                          |
| `"`    | string                    | more escaping                                          |
| `'''`  | multi-line literal string | less escaping                                          |
| `"""`  | multi-line string         | more escaping, newlines in declarations can be ignored |

Por ejemplo:

```toml
# literal string
format = '☺\☻ '

# regular string
format = "☺\\☻ "

# escaping Starship symbols
format = '\[\$\] '
```

When using line breaks, multi-line declarations can be used. For example, if you want to print a `$` symbol on a new line, the following values for `format` are equivalent:

```toml
# with literal string
format = '''

\$'''

# with multiline basic string
format = """

\\$"""

# with basic string
format = "\n\\$"
```

In multiline basic strings, newlines can be used for formatting without being present in the value by escaping them.

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

Es el formato con el que un módulo imprime todas sus variables. La mayoría de los módulos tienen una entrada llamada `format` que configura el formato de visualización del módulo. Se puede utilizar textos, variables y grupos de texto.

#### Variable

Una variable contiene un símbolo `$` seguido por el nombre de la variable. El nombre de una variable solamente puede contener letras, números y `_`.

Por ejemplo:

- `'$version'` is a format string with a variable named `version`.
- `'$git_branch$git_commit'` is a format string with two variables named `git_branch` and `git_commit`.
- `'$git_branch $git_commit'` has the two variables separated with a space.

#### Grupo de Texto

Un grupo de texto se compone de dos partes diferentes.

La primera parte, que está encerrada en un `[]`, es una [cadena de formato](#format-strings). Se puede agregar textos, variables, o incluso grupos de texto anidados.

En la segunda parte, que está encerrada entre `()`, es una [cadena de estilo](#style-strings). Esto se puede utilizar para diseñar la primera parte.

Por ejemplo:

- `'[on](red bold)'` will print a string `on` with bold text colored red.
- `'[⌘ $version](bold green)'` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `'[a [b](red) c](green)'` will print `a b c` with `b` red, and `a` and `c` green.

#### Cadenas de Estilo

La mayoría de los módulos de starship permiten configurar sus estilos de visualización. Esto se consigue con una entrada (normalmente llamada `style`) que no es más que un texto donde se especifica la configuración. A continuación mostramos algunos ejemplos de cadenas de estilo junto con su funcionalidad. Para más detalles sobre la sintaxis completa, consultar [la guía de configuración avanzada](/advanced-config/).

- `'fg:green bg:blue'` sets green text on a blue background
- `'bg:blue fg:bright-green'` sets bright green text on a blue background
- `'bold fg:27'` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `'underline bg:#bf5700'` sets underlined text on a burnt orange background
- `'bold italic fg:purple'` sets bold italic purple text
- `''` explicitly disables all styling

Nótese que el estilo es similar a como se controlaría por el emulador de terminal. Por ejemplo, algunos emuladores de terminal harán los colores más brillantes en lugar de más gruesos, y algunos temas de colores usan los mismos valores para texto normal y colores brillantes. Además, para mostrar textos en cursiva tu terminal debe tener soporte para hacerlo.

#### Cadenas de Formato Condicional

Una cadena de formato condicional envuelto en `(` y `)` no se renderizará si todas las variables dentro están vacías.

Por ejemplo:

- `'(@$region)'` will show nothing if the variable `region` is `None` or empty string, otherwise `@` followed by the value of region.
- `'(some text)'` will always show nothing since there are no variables wrapped in the braces.
- When `$combined` is a shortcut for `\[$a$b\]`, `'($combined)'` will show nothing only if `$a` and `$b` are both `None`. This works the same as `'(\[$a$b\] )'`.

### Coincidencia negativa

Varios módulos tienen variables `detect_extensions`, `detect_files`y `detect_folders`. Estas toman listas de cadenas de texto para que coincidan o no coincidan. "Negative" options, those which should not be matched, are indicated with a leading '!' character. La presencia de _cualquier_ indicador negativo en el directorio hará que el módulo no coincida.

Las extensiones coinciden tanto con los caracteres después del último punto en un nombre de archivo, como con los caracteres después del primer punto en un nombre de archivo. Por ejemplo, `foo.bar.tar.gz` será emparejado contra `bar.tar.gz` y `gz` en la variable `detect_extensions`. Los archivos cuyo nombre comienza con un punto no se consideran extensiones en absoluto.

Para ver cómo funciona esto en la práctica, puede hacer coincidir con archivos TypeScript, pero no con archivos MPEG Transport Stream, así:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Prompt

Esta es la lista de opciones de configuración del prompt.

### Opciones

| Opción            | Predeterminado                     | Descripción                                                                                                                                                                                                                           |
| ----------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [ver aquí](#default-prompt-format) | Configura el formato del prompt.                                                                                                                                                                                                      |
| `right_format`    | `''`                               | Ver [Habilitar prompt derecho](/advanced-config/#enable-right-prompt)                                                                                                                                                                 |
| `scan_timeout`    | `30`                               | Tiempo de espera tras el que Starship escanea archivos (en milisegundos).                                                                                                                                                             |
| `command_timeout` | `500`                              | Tiempo de espera para los comandos ejecutados por Starship (en milisegundos).                                                                                                                                                         |
| `add_newline`     | `true`                             | Inserta un línea en blanco entre las instrucciones del intérprete de comandos.                                                                                                                                                        |
| `paleta`          | `''`                               | Establece la paleta de color de `paletas` a utilizar.                                                                                                                                                                                 |
| `paletas`         | `{}`                               | Colección de paletas de colores que asignan [colores](/advanced-config/#style-strings) a nombres definidos por el usuario. Tenga en cuenta que las paletas de colores no pueden hacer referencia a sus propias definiciones de color. |

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

La varieble `format` por defecto se utiliza para definir el formato del prompt, si está vacía o `format` no se proporciona. El valor predeterminado es el siguiente:

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
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$docker_context\
$package\
$c\
$cmake\
$cobol\
$daml\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$guix_shell\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$gradle\
$lua\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vlang\
$vagrant\
$zig\
$buf\
$nix_shell\
$conda\
$meson\
$spack\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
$env_var\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$os\
$container\
$shell\
$character"""
```

Si solo quieres extender el formato predeterminado, puedes usar `$all`; los módulos que se añaden explícitamente al formato no serán duplicados. Ej.

```toml
# Move the directory to the second line
format = '$all$directory$character'
```

## AWS

El módulo `aws` muestra la región y el perfil actual de AWS y un temporizador de vencimiento cuando se utilizan credenciales temporales. La salida del módulo utiliza las variables de entorno `AWS_REGION`, `AWS_DEFAULT_REGION` y `AWS_PROFILE`, y los archivos `~/. ws/config` y `~/.aws/credenciales` según sea necesario.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process` or `sso_start_url` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice. If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

Cuando se utiliza [aws-vault](https://github.com/99designs/aws-vault), el perfil se obtiene de la variable de entorno `AWS_VAULT` y la fecha de expiración de credenciales se obtiene de la variable de entorno `AWS_SESSION_EXPIRATION`.

Cuando se utiliza [awsu](https://github.com/kreuzwerker/awsu) el perfil se lee de la variable de entorno `AWSU_PROFILE`.

Cuando se utiliza [AWSume](https://awsu.me), el perfil se obtiene de la variable de entorno `AWSUME_PROFILE` y la fecha de expiración de credenciales se obtiene de la variable de entorno `AWSUME_EXPIRATION`.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials` falls back to the `x_security_token_expires` key.

### Opciones

| Opción              | Predeterminado                                                        | Descripción                                                                                                              |
| ------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | El formato del módulo.                                                                                                   |
| `symbol`            | `'☁️ '`                                                               | El símbolo que se muestra antes del perfil de AWS.                                                                       |
| `region_aliases`    | `{}`                                                                  | Tabla de alias de región para mostrar además del nombre AWS.                                                             |
| `profile_aliases`   | `{}`                                                                  | Tabla de alias de región para mostrar además del nombre AWS.                                                             |
| `style`             | `'bold yellow'`                                                       | El estilo del módulo.                                                                                                    |
| `expiration_symbol` | `X`                                                                   | El símbolo mostrado cuando las credenciales temporales han caducado.                                                     |
| `disabled`          | `false`                                                               | Desactiva el módulo AWS.                                                                                                 |
| `force_display`     | `false`                                                               | Si `true` muestra información incluso si `credentials`, `credential_process` o `sso_start_url` no han sido configuradas. |

### Variables

| Variable  | Ejemplo          | Descripción                                              |
| --------- | ---------------- | -------------------------------------------------------- |
| region    | `ap-northeast-1` | La región actual de AWS                                  |
| profile   | `astronauts`     | El perfil actual de AWS                                  |
| duration  | `2h27m20s`       | La duración de la validez de las credenciales temporales |
| symbol    |                  | Refleja el valor de la opción `symbol`                   |
| style\* |                  | Refleja el valor de la opción `style`                    |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

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

El módulo `azure` muestra la suscripción Azure actual. This is based on showing the name of the default subscription or the username, as defined in the `~/.azure/azureProfile.json` file.

### Opciones

| Variable   | Predeterminado                           | Descripción                                 |
| ---------- | ---------------------------------------- | ------------------------------------------- |
| `format`   | `'on [$symbol($subscription)]($style) '` | El formato para renderizar el módulo Azure. |
| `symbol`   | `'ﴃ '`                                   | El símbolo utilizado en el formato.         |
| `style`    | `'blue bold'`                            | El estilo utilizado en el formato.          |
| `disabled` | `true`                                   | Deshabilita el módulo `azure`.              |

### Ejemplos

#### Display Subscription Name

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = 'ﴃ '
style = 'blue bold'
```

#### Display Username

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## Battery

El módulo `battery` muestra la cantidad de batería y si se está cargando o no. El módulo solamente es visible cuando la batería del dispositivo está por debajo del 10%.

### Opciones

| Opción               | Predeterminado                    | Descripción                                                              |
| -------------------- | --------------------------------- | ------------------------------------------------------------------------ |
| `full_symbol`        | `' '`                            | Se muestra cuando la batería está cargada.                               |
| `charging_symbol`    | `' '`                            | Se muestra cuando la batería se está cargando.                           |
| `discharging_symbol` | `' '`                            | Se muestra cuando la batería se está descargando.                        |
| `unknown_symbol`     | `' '`                            | El símbolo que se muestra cuando el estado de la batería es desconocido. |
| `empty_symbol`       | `' '`                            | El símbolo que se muestra cuando el estado de la batería está vacío.     |
| `format`             | `'[$symbol$percentage]($style) '` | El formato del módulo.                                                   |
| `display`            | [ver aquí](#battery-display)      | Define cuándo mostrar el indicador y el estilo.                          |
| `disabled`           | `false`                           | Desactiva el módulo `battery`.                                           |

### Ejemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Indicador de batería

La opción de configuración `display` se utiliza para definir cuándo debe mostrarse el indicador de batería (threshold), cuál símbolo se utilizaría (symbol), y cómo sería (style). Si no se provee ningún valor para `display`. El valor predeterminado es el siguiente:

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

El valor por defecto para las opciones `charging_symbol` y `discharging_symbol` son respectivamente los valores `charging_symbol` y `discharging_symbol` de las opciones de `battery`.

#### Opciones

La opción `display` es un arreglo de la siguiente tabla.

| Opción               | Predeterminado | Descripción                                                                                                                             |
| -------------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`           | El umbral para la opción de visualización.                                                                                              |
| `style`              | `'red bold'`   | El estilo usado cuando si la opción <0>display</0> está activa.                                                                         |
| `charging_symbol`    |                | Símbolo opcional que se muestra si la opción de visualización está en uso, por defecto en la opción `charging_symbol` de la batería.    |
| `discharging_symbol` |                | Símbolo opcional que se muestra si la opción de visualización está en uso, por defecto en la opción `discharging_symbol` de la batería. |

#### Ejemplo

```toml
[[battery.display]] # 'bold red' style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = 'bold red'

[[battery.display]] # 'bold yellow' style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦'

# when capacity is over 30%, the battery indicator will not be displayed
```

## Buf

El módulo `buf` muestra la versión instalada de [Buf](https://buf.build). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- La CLI de [`buf`](https://github.com/bufbuild/buf) está instalada.
- El directorio actual contiene un archivo de configuración [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), o [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml).

### Opciones

| Opción              | Predeterminado                                  | Descripción                                          |
| ------------------- | ----------------------------------------------- | ---------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | El formato para el módulo `buf`.                     |
| `version_format`    | `'v${raw}'`                                     | El formato de versión.                               |
| `symbol`            | `'🐃 '`                                          | El símbolo usado antes de mostrar la versión de Buf. |
| `detect_extensions` | `[]`                                            | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                            | Qué carpetas deberían activar estos módulos.         |
| `style`             | `'bold blue'`                                   | El estilo del módulo.                                |
| `disabled`          | `false`                                         | Deshabilita el módulo `elixir`.                      |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| `version` | `v1.0.0` | La versión de `buf`                    |
| `symbol`  |          | Refleja el valor de la opción `symbol` |
| `style`*  |          | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `bun.lockb`
- El directorio actual contiene un archivo `bunfig.toml`

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🍞 '`                               | A format string representing the symbol of Bun.                                         |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['bun.lockb', 'bunfig.toml']`       | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold red'`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `bun` module.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.1.4` | The version of `bun`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

El módulo `c` muestra información sobre su compilador de C. Por defecto el módulo se mostrará si el directorio actual contiene un archivo `.c` o `.h`.

### Opciones

| Opción              | Predeterminado                                                              | Descripción                                                                             |
| ------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                | La cadena de formato para el módulo.                                                    |
| `version_format`    | `'v${raw}'`                                                                 | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'C '`                                                                      | El símbolo usado antes de mostrar los detalles del compilador                           |
| `detect_extensions` | `['c', 'h']`                                                                | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                                                        | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                        | Qué carpetas deberían activar este módulo.                                              |
| `commands`          | [ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ] | Cómo detectar cuál compilador es                                                        |
| `style`             | `'bold 149'`                                                                | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                     | Deshabilita el módulo `c`.                                                              |

### Variables

| Variable | Ejemplo | Descripción                            |
| -------- | ------- | -------------------------------------- |
| name     | clang   | El nombre del compilador               |
| version  | 13.0.0  | La versión del compilador              |
| symbol   |         | Refleja el valor de la opción `symbol` |
| style    |         | Refleja el valor de la opción `style`  |

NB que `versión` no está en el formato por defecto.

### Commands

La opción de `commands` acepta una lista de comandos para determinar la versión y el nombre del compilador.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship intentará ejecutar cada comando hasta que obtenga un resultado en STDOUT.

Si un compilador C no es compatible con este módulo, puede solicitarlo [planteando un problema en GitHub](https://github.com/starship/starship/).

### Ejemplo

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## Carácter

El módulo `character` muestra un carácter (normalmente una flecha) al lado del texto que introduces en la terminal.

El caracter te dirá si el último comando fue exitoso o no. Se puede hacer de dos maneras:

- Cambiando el color (`red`/`green`)
- Cambiando la forma (`.`/`✖`)

Por defecto sólo cambia el color. Si también se quiere cambiar su forma, ver [este ejemplo](#with-custom-error-shape).

::: warning

`vicmd_symbol` solo es compatible con cmd, fish y zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

:::

### Opciones

| Opción                      | Predeterminado       | Descripción                                                                                             |
| --------------------------- | -------------------- | ------------------------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | La cadena de formato usada antes de la entrada de texto.                                                |
| `success_symbol`            | `'[❯](bold green)'`  | La cadena de formato usada antes de la entrada de texto si el comando anterior tuvo éxito.              |
| `error_symbol`              | `'[❯](bold red)'`    | La cadena de formato usada antes de la entrada de texto si el comando anterior falló.                   |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | El cadena de formato antes de la entrada de texto si el intérprete de comandos está en modo vim normal. |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode.                 |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.                       |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.                        |
| `disabled`                  | `false`              | Desactiva el módulo `character`.                                                                        |

### Variables

| Variable | Ejemplo | Descripción                                                                                              |
| -------- | ------- | -------------------------------------------------------------------------------------------------------- |
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
vicmd_symbol = '[V](bold green) '
```

## CMake

El módulo `cmake` muestra la versión actualmente instalada de [CMake](https://cmake.org/). Por defecto el módulo se activará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `CMakeLists.txt`
- El directorio actual contiene un archivo `CMakeCache.txt`

### Opciones

| Opción              | Predeterminado                         | Descripción                                                                             |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`   | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                            | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'△ '`                                 | El símbolo usado antes de la versión de cmake.                                          |
| `detect_extensions` | `[]`                                   | Qué extensiones deben activar este módulo                                               |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | Qué nombres de archivo deben activar este módulo                                        |
| `detect_folders`    | `[]`                                   | Qué carpetas deben activar este módulo                                                  |
| `style`             | `'bold blue'`                          | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                | Deshabilita el módulo `cmake`.                                                          |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | La versión de cmake                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## COBOL / GNUCOBOL

El módulo `cobol` muestra la versión instalada de COBOL. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene cualquier archivo que termine en `.cob` o `.COB`
- El directorio actual contiene cualquier archivo que termine en `.cbl` o `.CBL`

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `symbol`            | `'⚙️ '`                              | El símbolo usado antes de mostrar la versión de COBOL.                                  |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | El estilo del módulo.                                                                   |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `disabled`          | `false`                              | Deshabilita el módulo `cobol`.                                                          |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v3.1.2.0` | La versión de `cobol`                  |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## Tiempo de Ejecución

El módulo `cmd_duration` muestra cuánto tiempo tomó ejecutarse el último comando. El módulo se mostrará solo si el comando llevó dos segundos o más, o el valor de `min_time` si existe.

::: warning No utilizar la trampa DEBUG en Bash

Si estás usando Starship con `Bash`, no uses `DEBUG` después de ejecutar `eval $(starship init $0)`, o el módulo **se romperá**.

:::

Los usuarios de Bash que necesiten la funcionalidad como preexec pueden usar el [framework bash_preexec de rcaloras](https://github.com/rcaloras/bash-preexec). Basta con definir los arreglos `preexec_functions` y `precmd_functions` antes de ejecutar `eval $(starship init $0)`, y luego proceder como siempre.

### Opciones

| Opción                 | Predeterminado                | Descripción                                                                                                                                                                                                  |
| ---------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `min_time`             | `2_000`                       | Duración más corta para mostrar el tiempo (en milisegundos).                                                                                                                                                 |
| `show_milliseconds`    | `false`                       | Mostrar milisegundos además de segundos para la duración.                                                                                                                                                    |
| `format`               | `'took [$duration]($style) '` | El formato del módulo.                                                                                                                                                                                       |
| `style`                | `'bold yellow'`               | El estilo del módulo.                                                                                                                                                                                        |
| `disabled`             | `false`                       | Deshabilita el módulo `cmd_duration`.                                                                                                                                                                        |
| `show_notifications`   | `false`                       | Muestra notificaciones de escritorio cuando se complete el comando.                                                                                                                                          |
| `min_time_to_notify`   | `45_000`                      | Duración más corta para la notificación (en milisegundos).                                                                                                                                                   |
| `notification_timeout` |                               | Duración para mostrar la notificación (en milisegundos). Si no se establece, el tiempo de espera para notificar será determinado por el demonio. No todos los demonios de notificaciones honran esta opción. |

### Variables

| Variable  | Ejemplo  | Descripción                                |
| --------- | -------- | ------------------------------------------ |
| duration  | `16m40s` | El tiempo que tardó en ejecutar el comando |
| style\* |          | Refleja el valor de la opción `style`      |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

El módulo `conda` muestra el entorno actual [Conda](https://docs.conda.io/en/latest/), si `$CONDA_DEFAULT_ENV` está configurado.

::: tip

Esto no modifica el propio símbolo de sistema de Conda. En caso de querer suprimirlo, ejecuta `conda config --set changeps1 False`.

:::

### Opciones

| Opción              | Predeterminado                         | Descripción                                                                                                                                                                                                            |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | El número de directorios a los que se debe truncar la ruta de entorno, si el entorno fue creado a través de `conda create -p [path]`. `0` significa sin truncamiento. Vea también el módulo [`directory`](#directory). |
| `symbol`            | `'🅒 '`                                 | El símbolo usado antes del nombre del entorno.                                                                                                                                                                         |
| `style`             | `'bold green'`                         | El estilo del módulo.                                                                                                                                                                                                  |
| `format`            | `'via [$symbol$environment]($style) '` | El formato del módulo.                                                                                                                                                                                                 |
| `ignore_base`       | `true`                                 | Ignora el entorno `base` cuando se activa.                                                                                                                                                                             |
| `disabled`          | `false`                                | Deshabilita el módulo `conda`.                                                                                                                                                                                         |

### Variables

| Variable    | Ejemplo      | Descripción                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | El entorno Conda actual                |
| symbol      |              | Refleja el valor de la opción `symbol` |
| style\*   |              | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Contenedor

El módulo `container` muestra el símbolo y nombre del contenedor, si está dentro de un contenedor.

### Opciones

| Opción     | Predeterminado                     | Descripción                                                      |
| ---------- | ---------------------------------- | ---------------------------------------------------------------- |
| `symbol`   | `'⬢'`                              | El símbolo mostrado, cuando se encuentra dentro de un contenedor |
| `style`    | `'bold red dimmed'`                | El estilo del módulo.                                            |
| `format`   | `'[$symbol \[$name\]]($style) '` | El formato del módulo.                                           |
| `disabled` | `false`                            | Deshabilita el módulo `container`.                               |

### Variables

| Variable  | Ejemplo             | Descripción                            |
| --------- | ------------------- | -------------------------------------- |
| name      | `fedora-toolbox:35` | El nombre del contenedor               |
| symbol    |                     | Refleja el valor de la opción `symbol` |
| style\* |                     | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

El módulo `cristal` muestra la versión instalada de [Crystal](https://crystal-lang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `shard.yml`
- El directorio actual contiene un fichero `.cr`

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `symbol`            | `'🔮 '`                               | El símbolo usado antes de mostrar la versión del crystal.                               |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | El estilo del módulo.                                                                   |
| `detect_extensions` | `['cr']`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['shard.yml']`                      | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `disabled`          | `false`                              | Deshabilita el módulo `crystal`.                                                        |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | La versión de `crystal`                |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers) SDK version when you are in the root directory of your Daml project. The `sdk-version` in the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION` environment variable. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `daml.yaml`

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'Λ '`                               | A format string representing the symbol of Daml                                         |
| `style`             | `'bold cyan'`                        | El estilo del módulo.                                                                   |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['daml.yaml']`                      | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `disabled`          | `false`                              | Deshabilita el módulo `daml`.                                                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.2.0` | La versión de `daml`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

El módulo `dart` muestra la versión instalada de [Dart](https://dart.dev/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo con la extensión `.dart`
- El directorio actual contiene un directorio `.dart_tool`
- El directorio actual contiene un archivo `pubspec.yaml`, `pubspec.yml` o `pubspec.lock`

### Opciones

| Opción              | Predeterminado                                    | Descripción                                                                             |
| ------------------- | ------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`              | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                                       | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🎯 '`                                            | Una cadena de formato que representa el símbolo de Dart                                 |
| `detect_extensions` | `['dart']`                                        | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `['.dart_tool']`                                  | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold blue'`                                     | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                           | Deshabilita el módulo `dart`.                                                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.8.4` | La versión de `dart`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

El módulo `deno` le muestra la versión instalada de [Deno](https://deno.land/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` o `deps.js`

### Opciones

| Opción              | Predeterminado                                                          | Descripción                                                                             |
| ------------------- | ----------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                    | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                                                             | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦕 '`                                                                  | Una cadena de formato que representa el símbolo de Deno                                 |
| `detect_extensions` | `[]`                                                                    | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                    | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'green bold'`                                                          | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                 | Deshabilita el módulo `deno`.                                                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.8.3` | La versión de `deno`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Directory

El módulo `directory` muestra la ruta a tu directorio actual, truncado a tres carpetas padres. Tu directorio se truncará a la raíz del repositorio git en el que te encuentres.

Cuando usas el estilo fish de la opción pwd, en lugar de ocultar la ruta truncada, verás una versión acortada del nombre de cada directorio basada en el número que activaste para la opción.

Por ejemplo, dado `~/Dev/Nix/nixpkgs/pkgs` donde `nixpkgs` es la raíz del repositorio y la opción establecida a `1`. Ahora verás `~/D/N/nixpkgs/pkgs`, mientras que antes habría sido `nixpkgs/pkgs`.

### Opciones

| Opción                   | Predeterminado                                                                                                               | Descripción                                                                                                                 |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | El número de carpetas a las que se debe truncar el directorio actual.                                                       |
| `truncate_to_repo`       | `true`                                                                                                                       | Truncar o no hasta la raíz del repositorio git en el que se esté.                                                           |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | El formato del módulo.                                                                                                      |
| `style`                  | `'bold cyan'`                                                                                                                | El estilo del módulo.                                                                                                       |
| `disabled`               | `false`                                                                                                                      | Deshabilita el módulo `directory`.                                                                                          |
| `read_only`              | `'🔒'`                                                                                                                        | El símbolo que indica el directorio actual es de sólo lectura.                                                              |
| `read_only_style`        | `'red'`                                                                                                                      | El estilo para el símbolo de sólo lectura.                                                                                  |
| `truncation_symbol`      | `''`                                                                                                                         | El símbolo a prefijar a las rutas truncadas. eg: '…/'                                                                       |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. El valor por defecto es equivalente al `style`.              |
| `repo_root_style`        |                                                                                                                              | El estilo para la raíz del repositorio de git. El valor por defecto es equivalente al `style`.                              |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                                    |
| `home_symbol`            | `'~'`                                                                                                                        | El símbolo que indica el directorio de inicio.                                                                              |
| `use_os_path_sep`        | `true`                                                                                                                       | Utiliza el separador de ruta del sistema operativo específico en lugar de usar siempre `/` (por ejemplo, `\` en Windows) |

<details>
<summary>Este módulo tiene algunas opciones avanzadas de configuración que controlan cómo se muestra el directorio.</summary>

| Opción avanzada             | Predeterminado | Descripción                                                                                                                                                                                                                  |
| --------------------------- | -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `sustituciones`             |                | Una tabla de sustituciones que se deben hacer a la ruta.                                                                                                                                                                     |
| `fish_style_pwd_dir_length` | `0`            | El número de caracteres a usar al aplicar la lógica de ruta pwd del intérprete de comandos de Fish.                                                                                                                          |
| `use_logical_path`          | `true`         | Si `true` renderiza la ruta lógica originada desde el intérprete de comandos a través de `PWD` o `--logical-path`. Si `false` en su lugar renderiza la ruta física del sistema de archivos con enlaces simbólicos resueltos. |

`substitutions` permite definir reemplazos arbitrarios para cadenas literales que ocurren en la ruta, por ejemplo prefijos largos de red o directorios de desarrollo (p. ej. Java). Ten en cuenta que esto desactivará el estilo PWD de fish.

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` interactúa con las opciones de truncamiento estándar de una manera que puede sorprenderse primero: si no es cero, los componentes de la ruta que normalmente se truncarían se muestran con esa cantidad de caracteres. Por ejemplo, la ruta `/built/this/city/on/rock/and/roll`, que normalmente se mostraría como `rock/and/roll`, se mostraría como `/b/t/c/o/rock/and/roll` con `fish_style_pwd_dir_length = 1`--los componentes de ruta que normalmente se eliminarían se muestran con un solo carácter. Para `fish_style_pwd_dir_length = 2`, sería `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable  | Ejemplo               | Descripción                           |
| --------- | --------------------- | ------------------------------------- |
| ruta      | `'D:/Projects'`       | La ruta del directorio actual         |
| style\* | `'black bold dimmed'` | Refleja el valor de la opción `style` |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

<details>
<summary>Los repositorios de git tienen variables adicionales.</summary>

Consideremos la ruta `/path/to/home/git_repo/src/lib`

| Variable           | Ejemplo               | Descripción                                         |
| ------------------ | --------------------- | --------------------------------------------------- |
| before_root_path | `'/path/to/home/'`    | La ruta antes de la ruta del directorio raíz de git |
| repo_root          | `'git_repo'`          | El nombre del directorio raíz de git                |
| ruta               | `'/src/lib'`          | La ruta restante                                    |
| style              | `'black bold dimmed'` | Refleja el valor de la opción `style`               |
| repo_root_style  | `'underline white'`   | Estilo para el nombre del directorio raíz de git    |

</details>

### Ejemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Contexto de Docker

El módulo `docker_context` muestra el [contexto de Docker](https://docs.docker.com/engine/context/working-with-contexts/) actualmente activo si no está definido en `default` o si las variables de entorno `DOCKER_MACHINE_NAME`, `DOCKER_HOST` o `DOCKER_CONTEXT` están definidas (como se entiende para sobrescribir el contexto en uso).

### Opciones

| Opción              | Predeterminado                                                | Descripción                                                                                                              |
| ------------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol$context]($style) '`                            | El formato del módulo.                                                                                                   |
| `symbol`            | `'🐳 '`                                                        | El símbolo usado antes de mostrar el contexto de Docker.                                                                 |
| `only_with_files`   | `true`                                                        | Mostrar solo cuando haya una coincidencia                                                                                |
| `detect_extensions` | `[]`                                                          | Qué extensiones deben activar este módulo (necesita `solly_with_files` para ser verdadero).                              |
| `detect_files`      | `['docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Qué nombres de archivo deberían activar este módulo (necesita que `only_with_files` sea verdadero, con un valor "true"). |
| `detect_folders`    | `[]`                                                          | Qué carpetas deberían activar este módulo (necesita que `only_with_files` sea verdadero, con un valor "true").           |
| `style`             | `'blue bold'`                                                 | El estilo del módulo.                                                                                                    |
| `disabled`          | `false`                                                       | Deshabilita el módulo `docker_context`.                                                                                  |

### Variables

| Variable  | Ejemplo        | Descripción                            |
| --------- | -------------- | -------------------------------------- |
| contexto  | `test_context` | El contexto actual de docker           |
| symbol    |                | Refleja el valor de la opción `symbol` |
| style\* |                | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

El módulo `dotnet` muestra la versión relevante del [.NET Core SDK](https://dotnet.microsoft.com/) para el directorio actual. Si el SDK ha sido anclado en el directorio actual, se mostrará la versión fijada. De lo contrario, el módulo muestra la última versión instalada del SDK.

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

Internamente, este módulo utiliza su propio mecanismo para la detección de versiones. Normalmente es el doble de rápido que ejecutar `dotnet --version`, pero puede mostrar una versión incorrecta si tu proyecto .NET tiene un diseño de directorio inusual. Si la precisión es más importante que la velocidad, puedes desactivar el mecanismo estableciendo `heuristic = false` en las opciones del módulo.

El módulo también mostrará el Target Framework Moniker ([https://docs.microsoft. om/es/dotnet/standard/frameworks#supported-target-framework-versions](https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks)) cuando exista un archivo `.csproj` en el directorio actual.

### Opciones

| Opción              | Predeterminado                                                                                          | Descripción                                                                             |
| ------------------- | ------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                           | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                                                                                             | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'.NET '`                                                                                               | El símbolo usado antes de mostrar la version de dotnet.                                 |
| `heuristic`         | `true`                                                                                                  | Usa una detección de versiones más rápida para mantener la nave espacial veloz.         |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                    | Qué carpetas deberían activar estos módulos.                                            |
| `style`             | `'bold blue'`                                                                                           | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                                 | Desactiva el módulo `dotnet`.                                                           |

### Variables

| Variable  | Ejemplo          | Descripción                                                     |
| --------- | ---------------- | --------------------------------------------------------------- |
| version   | `v3.1.201`       | La version del SDK de `dotnet`                                  |
| tfm       | `netstandard2.0` | El Target Framework Moniker al que se dirige el proyecto actual |
| symbol    |                  | Refleja el valor de la opción `symbol`                          |
| style\* |                  | Refleja el valor de la opción `style`                           |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

El módulo `elixir` muestra la versión instalada de [Elixir](https://elixir-lang.org/) y [Erlang/OTP](https://erlang.org/doc/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `mix.exs`.

### Opciones

| Opción              | Predeterminado                                              | Descripción                                                                             |
| ------------------- | ----------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | El formato para el módulo de elixir.                                                    |
| `version_format`    | `'v${raw}'`                                                 | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💧 '`                                                      | El símbolo usado antes de mostrar la versión de Elixir/Erlang.                          |
| `detect_extensions` | `[]`                                                        | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['mix.exs']`                                               | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                        | Qué carpetas deberían activar estos módulos.                                            |
| `style`             | `'bold purple'`                                             | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                     | Deshabilita el módulo `elixir`.                                                         |

### Variables

| Variable    | Ejemplo | Descripción                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | La version de `elixir`                 |
| otp_version |         | La versión de otp de `elixir`          |
| symbol      |         | Refleja el valor de la opción `symbol` |
| style\*   |         | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

El módulo `elm` muestra la versión instalada de [Elm](https://elm-lang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `elm.json`
- El directorio actual contiene un archivo `elm-package.json`
- El directorio actual contiene un archivo `.elm-version`
- El directorio actual contiene una carpeta `elm-stuff`
- El directorio actual contiene archivos `*.elm`

### Opciones

| Opción              | Predeterminado                                     | Descripción                                                                             |
| ------------------- | -------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`               | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                                        | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🌳 '`                                             | Una cadena de formato que representa el símbolo de Elm.                                 |
| `detect_extensions` | `['elm']`                                          | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `['elm-stuff']`                                    | Qué carpetas deberían activar estos módulos.                                            |
| `style`             | `'cyan bold'`                                      | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                            | Deshabilita el módulo `elm`.                                                            |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.19.1` | La versión de `elm`                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Variable de entorno

El módulo `env_var` muestra el valor actual de una variable de entorno seleccionada. El módulo se mostrará sólo si se cumplen cualquiera de las siguientes condiciones:

- La opción de configuración de `variable` coincide con una variable de entorno existente
- La opción de configuración de `variable` no está definida, pero la opción de configuración se encuentra `por defecto`

::: tip

Múltiples variables de entorno pueden mostrarse usando una `.`. (ver ejemplo) Si la opción de configuración de la `variable` no está definida, el módulo mostrará el valor de la variable bajo el nombre del texto después del caracter `.`.

Ejemplo: la siguiente configuración mostrará el valor de la variable de entorno USER

```toml
# ~/.config/starship.toml

[env_var.USER]
default = 'unknown user'
```

:::

### Opciones

| Opción     | Predeterminado                | Descripción                                                                            |
| ---------- | ----------------------------- | -------------------------------------------------------------------------------------- |
| `symbol`   | `''`                          | El símbolo usado antes de mostrar el valor de la variable.                             |
| `variable` |                               | La variable de entorno a mostrar.                                                      |
| `default`  |                               | El valor por defecto que se mostrará cuando la variable seleccionada no está definida. |
| `format`   | `'con [$env_value]($style) '` | El formato del módulo.                                                                 |
| `disabled` | `false`                       | Deshabilita el módulo `env_var`.                                                       |

### Variables

| Variable  | Ejemplo                                     | Descripción                                 |
| --------- | ------------------------------------------- | ------------------------------------------- |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | El valor de entorno de la opción `variable` |
| symbol    |                                             | Refleja el valor de la opción `symbol`      |
| style\* | `black bold dimmed`                         | Refleja el valor de la opción `style`       |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

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

El módulo `erlang` muestra la versión instalada de [Erlang/OTP](https://erlang.org/doc/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `rebar.config`.
- El directorio actual contiene un fichero `erlang.mk`.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `' '`                               | El símbolo usado antes de mostrar la versión de Erlang.                                 |
| `style`             | `'bold red'`                         | El estilo del módulo.                                                                   |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                                            |
| `disabled`          | `false`                              | Deshabilita el módulo `erlang`.                                                         |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | La versión de `erlang`                 |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Rellenar

El módulo `fill` llena cualquier espacio extra en la línea con un símbolo. Si múltiples módulos `fill` están presentes en una línea, dividirán el espacio equitativamente entre ellos. Esto es útil para alinear otros módulos.

### Opciones

| Opción     | Predeterminado | Descripción                                |
| ---------- | -------------- | ------------------------------------------ |
| `symbol`   | `'.'`          | El símbolo utilizado para llenar la línea. |
| `style`    | `'bold black'` | El estilo del módulo.                      |
| `disabled` | `false`        | Deshabilita el módulo `fill`               |

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

## Google Cloud (`gcloud`)

El módulo `gcloud` muestra la configuración actual para el CLI de [`gcloud`](https://cloud.google.com/sdk/gcloud). Esto se basa en el archivo `~/.config/gcloud/active_config`, el archivo `~/.config/gcloud/configurations/config_{CONFIG NAME}` y la variable de entorno `CLOUDSDK_CONFIG`.

### Opciones

| Opción            | Predeterminado                                             | Descripción                                                  |
| ----------------- | ---------------------------------------------------------- | ------------------------------------------------------------ |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | El formato del módulo.                                       |
| `symbol`          | `'☁️  '`                                                   | El símbolo usado antes de mostrar el perfil actual de GCP.   |
| `region_aliases`  | `{}`                                                       | Tabla de alias de región a mostrar además del nombre GCP.    |
| `project_aliases` | `{}`                                                       | Tabla de alias del proyecto a mostrar además del nombre GCP. |
| `style`           | `'bold blue'`                                              | El estilo del módulo.                                        |
| `disabled`        | `false`                                                    | Deshabilita el módulo `gcloud`.                              |

### Variables

| Variable  | Ejemplo       | Descripción                                                                   |
| --------- | ------------- | ----------------------------------------------------------------------------- |
| region    | `us-central1` | La actual región GCP                                                          |
| cuenta    | `foo`         | El perfil actual de GCP                                                       |
| dominio   | `ejemplo.com` | El dominio actual del perfil GCP                                              |
| proyecto  |               | El proyecto GCP actual                                                        |
| activo    | `default`     | El nombre de configuración activo escrito en `~/.config/gcloud/active_config` |
| symbol    |               | Refleja el valor de la opción `symbol`                                        |
| style\* |               | Refleja el valor de la opción `style`                                         |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

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

El módulo `git_branch` muestra la rama activa del repositorio en tu directorio actual.

### Opciones

| Opción               | Predeterminado                                    | Descripción                                                                                         |
| -------------------- | ------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Muestra el nombre de la rama de seguimiento remoto, incluso si es igual al nombre de la rama local. |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | El formato del módulo. Use `'$branch'` to refer to the current branch name.                         |
| `symbol`             | `' '`                                            | Una cadena de formato que representa el símbolo de la rama git.                                     |
| `style`              | `'bold purple'`                                   | El estilo del módulo.                                                                               |
| `truncation_length`  | `2^63 - 1`                                        | Trunca el nombre de la rama a `N` grafemas.                                                         |
| `truncation_symbol`  | `'…'`                                             | El símbolo usado para indicar que un nombre de rama fue truncado. You can use `''` for no symbol.   |
| `only_attached`      | `false`                                           | Mostrar solo el hash de la confirmación de git cuando esté en estado "detached `HEAD`"              |
| `ignore_branches`    | `[]`                                              | Una lista de nombres a evitar ser visualizados. Useful for 'master' or 'main'.                      |
| `disabled`           | `false`                                           | Deshabilita el módulo `git_branch`.                                                                 |

### Variables

| Variable      | Ejemplo   | Descripción                                                                                                    |
| ------------- | --------- | -------------------------------------------------------------------------------------------------------------- |
| rama          | `maestro` | El nombre de la rama actual, vuelve a `HEAD` si no hay ninguna rama actual (por ejemplo, git detached `HEAD`). |
| remote_name   | `origen`  | El nombre remoto.                                                                                              |
| remote_branch | `maestro` | El nombre de la rama rastreada en `remote_name`.                                                               |
| symbol        |           | Refleja el valor de la opción `symbol`                                                                         |
| style\*     |           | Refleja el valor de la opción `style`                                                                          |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

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

El módulo `git_commit` muestra el hash de la confirmación actual y también la etiqueta (si existe) del repositorio en su directorio actual.

### Opciones

| Opción               | Predeterminado                 | Descripción                                                                                            |
| -------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                            | La longitud del hash de la confirmación de git mostrado.                                               |
| `format`             | `'[\($hash$tag\)]($style) '` | El formato del módulo.                                                                                 |
| `style`              | `'bold green'`                 | El estilo del módulo.                                                                                  |
| `only_detached`      | `true`                         | Mostrar solo el hash de la confirmación de git cuando esté en estado "detached `HEAD`"                 |
| `tag_disabled`       | `true`                         | Deshabilita mostrar información de etiquetas en el módulo `git_commit`.                                |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. El valor por defecto sólo permite coincidencias exactas. |
| `tag_symbol`         | `' 🏷 '`                        | Símbolo de etiqueta prefijando la información mostrada                                                 |
| `disabled`           | `false`                        | Deshabilita el módulo `git_commit`.                                                                    |

### Variables

| Variable  | Ejemplo   | Descripción                              |
| --------- | --------- | ---------------------------------------- |
| hash      | `b703eb3` | El hash actual de la confirmación de git |
| style\* |           | Refleja el valor de la opción `style`    |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git State

El módulo `git_state` se mostrará en directorios que son parte de un repositorio git, y donde hay una operación en curso, tales como: _REBASING_, _BISECTING_, etc. Si hay información de progreso (por ejemplo, REBASING 3/10), esa información será mostrada también.

### Opciones

| Opción         | Predeterminado                                                  | Descripción                                                                                         |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | Una cadena de formato que se muestra cuando un `rebase` está en progreso.                           |
| `fusionar`     | `'FUSIONANDO'`                                                  | Una cadena de formato que se muestra cuando un `merge` está en progreso.                            |
| `revertir`     | `'REVERTING'`                                                   | Una cadena de formato mostrada cuando un `revert` está en progreso.                                 |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | Una cadena de formato que se muestra cuando un `cherry-pick` está en progreso.                      |
| `bisect`       | `'BISECTING'`                                                   | Una cadena de formato que se muestra cuando un `bisect` está en progreso.                           |
| `am`           | `'AM'`                                                          | Una cadena de formato que se muestra cuando un `apply-mailbox` (`git am`) está en progeso.          |
| `am_or_rebase` | `'AM/REBASE'`                                                   | Una cadena de formato que se muestra cuando un ambiguo `apply-mailbox` o `rebase` está en progreso. |
| `style`        | `'bold yellow'`                                                 | El estilo del módulo.                                                                               |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | El formato del módulo.                                                                              |
| `disabled`     | `false`                                                         | Deshabilita el módulo `git_state`.                                                                  |

### Variables

| Variable         | Ejemplo    | Descripción                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | The current state of the repo         |
| progress_current | `1`        | El progreso de la operación actual    |
| progress_total   | `2`        | El progreso total de la operación     |
| style\*        |            | Refleja el valor de la opción `style` |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Métricas de Git

El módulo `git_metrics` mostrará el número de líneas añadidas y eliminadas en el repositorio git actual.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` como `false` en tu archivo de configuración.

:::

### Opciones

| Opción               | Predeterminado                                               | Descripción                                        |
| -------------------- | ------------------------------------------------------------ | -------------------------------------------------- |
| `added_style`        | `'bold green'`                                               | El estilo para el recuento añadido.                |
| `deleted_style`      | `'bold red'`                                                 | El estilo para el recuento eliminado.              |
| `only_nonzero_diffs` | `true`                                                       | Mostrar sólo el estado de los elementos cambiados. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | El formato del módulo.                             |
| `disabled`           | `true`                                                       | Deshabilita el módulo `git_metrics`.               |

### Variables

| Variable          | Ejemplo | Descripción                                   |
| ----------------- | ------- | --------------------------------------------- |
| añadido           | `1`     | El número actual de líneas añadidas           |
| borrado           | `2`     | El número actual de líneas eliminadas         |
| added_style\*   |         | Refleja el valor de la opción `added_style`   |
| deleted_style\* |         | Refleja el valor de la opción `deleted_style` |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git status

El módulo `git_status` muestra símbolos que representan el estado del repositorio en su directorio actual.

::: tip

El módulo Git Status es muy lento en los directorios de Windows (por ejemplo bajo `/mnt/c/`) en un entorno WSL. Puedes desactivar el módulo o utilizar la opción `windows_starship` para usar un ejecutable de la Starship nativa de Windows para calcular `git_status` para esas rutas.

:::

### Opciones

| Opción              | Predeterminado                                  | Descripción                                                                                                                               |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | El formato predeterminado para `git_status`                                                                                               |
| `conflicted`        | `'='`                                           | Esta rama tiene conflictos de fusión.                                                                                                     |
| `ahead`             | `'⇡'`                                           | El formato de `ahead`                                                                                                                     |
| `behind`            | `'⇣'`                                           | El formato de `behind`                                                                                                                    |
| `diverged`          | `'⇕'`                                           | El formato de `diverged`                                                                                                                  |
| `up_to_date`        | `''`                                            | El formato de `up_to_date`                                                                                                                |
| `sin seguimiento`   | `'?'`                                           | El formato de `untracked`                                                                                                                 |
| `stashed`           | `'$'`                                           | El formato de `stashed`                                                                                                                   |
| `modificado`        | `'!'`                                           | El formato de `modified`                                                                                                                  |
| `staged`            | `'+'`                                           | El formato de `staged`                                                                                                                    |
| `renamed`           | `'»'`                                           | El formato de `renamed`                                                                                                                   |
| `borrado`           | `'✘'`                                           | El formato de `deleted`                                                                                                                   |
| `style`             | `'bold red'`                                    | El estilo del módulo.                                                                                                                     |
| `ignore_submodules` | `false`                                         | Ignorar cambios a los submódulos.                                                                                                         |
| `disabled`          | `false`                                         | Deshabilita el módulo `git_status`.                                                                                                       |
| `windows_starship`  |                                                 | Utiliza esta ruta (Linux) a un ejecutable de Starship de Windows para renderizar `git_status` cuando está en las rutas de Windows en WSL. |

### Variables

Las siguientes variables se pueden utilizar en `format`:

| Variable          | Descripción                                                                                                              |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `all_status`      | Atajo para `$conflicted$stashed$deleted$renamed$modified$staged$untracked`                                               |
| `ahead_behind`    | Muestra la cadena de formato de `diverged` `ahead` o `behind` o `up_to_date` basado en el estado actual del repositorio. |
| `conflicted`      | Muestra `conflicted` cuando esta rama tiene conflictos de fusión.                                                        |
| `sin seguimiento` | Muestra `untracked` cuando hay archivos sin rastrear en el directorio de trabajo.                                        |
| `stashed`         | Muestra `stashed` cuando existe un archivo en el área de preparación para el repositorio local.                          |
| `modificado`      | Muestra `modified` cuando hay modificaciones de archivo en el directorio de trabajo.                                     |
| `staged`          | Muestra `staged` cuando se ha añadido un nuevo archivo al área de preparación.                                           |
| `renamed`         | Muestra `renamed` cuando un archivo renombrado ha sido añadido al área de preparación.                                   |
| `borrado`         | Muestra `deleted` cuando un archivo ha sido añadido al área de preparación.                                              |
| style\*         | Refleja el valor de la opción `style`                                                                                    |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

Las siguientes variables pueden ser usadas en `diverged`:

| Variable       | Descripción                                                    |
| -------------- | -------------------------------------------------------------- |
| `ahead_count`  | Número de confirmaciones por delante de la rama de seguimiento |
| `behind_count` | Número de confirmaciones detrás de la rama de seguimiento      |

Las siguientes variales pueden ser usadas en `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` y `deleted`:

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

## Go

El módulo `golang` muestra la versión instalada de [Go](https://golang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `go.mod`
- El directorio actual contiene un archivo `go.sum`
- El directorio actual contiene un archivo `go.work`
- El directorio actual contiene un archivo `glide.yaml`
- El directorio actual contiene un archivo `Gopkg.yml`
- El directorio actual contiene un archivo `Gopkg.lock`
- El directorio actual contiene un archivo `.go-version`
- El directorio actual contiene un directorio `Godeps`
- El directorio actual contiene un archivo con la extensión `.go`

### Opciones

| Opción              | Predeterminado                                                                            | Descripción                                                                             |
| ------------------- | ----------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                                                                               | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐹 '`                                                                                    | Una cadena de formato que representa el símbolo de Go.                                  |
| `detect_extensions` | `['go']`                                                                                  | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `['Godeps']`                                                                              | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold cyan'`                                                                             | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                   | Deshabilita el módulo de `golang`.                                                      |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | La versión de `go`                     |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment. The module will be shown when inside a guix-shell environment.

### Opciones

| Opción     | Predeterminado             | Descripción                                            |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'vía [$symbol]($style) '` | El formato del módulo.                                 |
| `symbol`   | `"🐃 "`                     | A format string representing the symbol of guix-shell. |
| `style`    | `"yellow bold"`            | El estilo del módulo.                                  |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) currently used in the project directory.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🅶 "`                               | A format string representing the symbol of Gradle.                                      |
| `detect_extensions` | `["gradle", "gradle.kts"]`           | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `["gradle"]`                         | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold bright-cyan"`                 | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `gradle` module.                                                           |
| `recursive`         | `false`                              | Enables recursive finding for the `gradle` directory.                                   |

### Variables

| Variable | Ejemplo  | Descripción                            |
| -------- | -------- | -------------------------------------- |
| version  | `v7.5.1` | The version of `gradle`                |
| symbol   |          | Refleja el valor de la opción `symbol` |
| style*   |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `stack.yaml` file
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                               |
| `symbol`            | `'λ '`                               | A format string representing the symbol of Haskell   |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.           |
| `style`             | `'bold purple'`                      | El estilo del módulo.                                |
| `disabled`          | `false`                              | Disables the `haskell` module.                       |

### Variables

| Variable       | Ejemplo     | Descripción                                                                             |
| -------------- | ----------- | --------------------------------------------------------------------------------------- |
| version        |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot       | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbol         |             | Refleja el valor de la opción `symbol`                                                  |
| style\*      |             | Refleja el valor de la opción `style`                                                   |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### Opciones

| Opción              | Predeterminado                                                                                  | Descripción                                                                             |
| ------------------- | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                            | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                                                                                     | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["hx", "hxml"]`                                                                                | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["project.xml", "Project.xml", "application.xml", "haxelib.json", "hxformat.json", ".haxerc"]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[".haxelib", "haxe_libraries"]`                                                                | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `"⌘ "`                                                                                          | Una cadena de formato que representa el símbolo de Helm.                                |
| `style`             | `"bold fg:202"`                                                                                 | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                         | Disables the `haxe` module.                                                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v4.2.5` | The version of `haxe`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `'⎈ '`                               | A format string representing the symbol of Helm.                                        |
| `style`             | `'bold white'`                       | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `helm` module.                                                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | The version of `helm`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Hostname

The `hostname` module shows the system hostname.

### Opciones

| Opción       | Predeterminado                         | Descripción                                                                                                                          |
| ------------ | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only`   | `true`                                 | Only show hostname when connected to an SSH session.                                                                                 |
| `ssh_symbol` | `'🌐 '`                                 | A format string representing the symbol when connected to SSH session.                                                               |
| `trim_at`    | `'.'`                                  | String that the hostname is cut off at, after the first match. `'.'` will stop after the first dot. `''` will disable any truncation |
| `format`     | `'[$ssh_symbol$hostname]($style) in '` | El formato del módulo.                                                                                                               |
| `style`      | `'bold dimmed green'`                  | El estilo del módulo.                                                                                                                |
| `disabled`   | `false`                                | Disables the `hostname` module.                                                                                                      |

### Variables

| Variable   | Ejemplo    | Descripción                                           |
| ---------- | ---------- | ----------------------------------------------------- |
| hostname   | `computer` | The hostname of the computer                          |
| style\*  |            | Refleja el valor de la opción `style`                 |
| ssh_symbol | `'🌏 '`     | The symbol to represent when connected to SSH session |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Opciones

| Opción              | Predeterminado                                                                                           | Descripción                                                                             |
| ------------------- | -------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                 | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                                                                                              | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                     | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot']` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                     | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `'☕ '`                                                                                                   | A format string representing the symbol of Java                                         |
| `style`             | `'red dimmed'`                                                                                           | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                                  | Disables the `java` module.                                                             |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| version   | `v14`   | The version of `java`                  |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to _always_ show the symbol and number of jobs, even if there are 0 jobs running.

The default functionality is:

- 0 jobs -> Nothing is shown.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

::: warning

This module is not supported on tcsh and nu.

:::

::: warning

The `threshold` option is deprecated, but if you want to use it, the module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

:::

### Opciones

| Opción             | Predeterminado                | Descripción                                                              |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | El formato del módulo.                                                   |
| `symbol`           | `'✦'`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `'bold blue'`                 | El estilo del módulo.                                                    |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | The number of jobs                     |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `'ஃ '`                               | A format string representing the symbol of Julia.                                       |
| `style`             | `'bold purple'`                      | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `julia` module.                                                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.4.0` | The version of `julia`                 |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.kt` or a `.kts` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['kt', 'kts']`                      | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `'🅺 '`                               | A format string representing the symbol of Kotlin.                                      |
| `style`             | `'bold blue'`                        | El estilo del módulo.                                                                   |
| `kotlin_binary`     | `'kotlin'`                           | Configures the kotlin binary that Starship executes when getting the version.           |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                           |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`                |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[kotlin]
symbol = '🅺 '
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = 'kotlinc'
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-context --namespace astronaut`. Similarly the user and cluster can be set with `kubectl config set-context starship-context --user starship-user` and `kubectl config set-context starship-context --cluster starship-cluster`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` como `false` en tu archivo de configuración.

When the module is enabled it will always be active, unless any of `detect_extensions`, `detect_files` or `detect_folders` have been set in which case the module will only be active in directories that match those conditions.

:::

### Opciones

| Opción              | Predeterminado                                       | Descripción                                                           |
| ------------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`            | `'☸ '`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | El formato del módulo.                                                |
| `style`             | `'cyan bold'`                                        | El estilo del módulo.                                                 |
| `context_aliases`   | `{}`                                                 | Table of context aliases to display.                                  |
| `user_aliases`      | `{}`                                                 | Table of user aliases to display.                                     |
| `detect_extensions` | `[]`                                                 | Qué extensiones deberían activar este módulo.                         |
| `detect_files`      | `[]`                                                 | Qué nombres de archivo deberían activar este módulo.                  |
| `detect_folders`    | `[]`                                                 | Qué carpetas deberían activar estos módulos.                          |
| `disabled`          | `true`                                               | Disables the `kubernetes` module.                                     |

### Variables

| Variable  | Ejemplo              | Descripción                              |
| --------- | -------------------- | ---------------------------------------- |
| contexto  | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | Refleja el valor de la opción `symbol`   |
| style\* |                      | Refleja el valor de la opción `style`    |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
'dev.local.cluster.k8s' = 'dev'
'.*/openshift-cluster/.*' = 'openshift'
'gke_.*_(?P<var_cluster>[\\w-]+)' = 'gke-$var_cluster'
[kubernetes.user_aliases]
'dev.local.cluster.k8s' = 'dev'
'root/.*' = 'root'
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Busqueda por Regex

Additional to simple aliasing, `context_aliases` and `user_aliases` also supports extended matching and renaming using regular expressions.

The regular expression must match on the entire kube context, capture groups can be referenced using `$name` and `$N` in the replacement. This is more explained in the [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace) documentation.

Long and automatically generated cluster names can be identified and shortened using regular expressions:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
'.*/openshift-cluster/.*' = 'openshift'
# Or better, to rename every OpenShift cluster at once:
'.*/(?P<var_cluster>[\\w-]+)/.*' = '$var_cluster'

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
'gke_.*_(?P<var_cluster>[\\w-]+)' = 'gke-$var_cluster'
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Opciones

| Opción     | Predeterminado | Descripción                                                        |
| ---------- | -------------- | ------------------------------------------------------------------ |
| `disabled` | `false`        | Disables the `line_break` module, making the prompt a single line. |

### Ejemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

The `localip` module shows the IPv4 address of the primary network interface.

### Opciones

| Opción     | Predeterminado            | Descripción                                            |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `'[$localipv4]($style) '` | El formato del módulo.                                 |
| `style`    | `'bold yellow'`           | El estilo del módulo.                                  |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### Variables

| Variable  | Ejemplo      | Descripción                           |
| --------- | ------------ | ------------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address     |
| style\* |              | Refleja el valor de la opción `style` |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🌙 '`                               | A format string representing the symbol of Lua.                                         |
| `detect_extensions` | `['lua']`                            | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['.lua-version']`                   | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `['lua']`                            | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold blue'`                        | El estilo del módulo.                                                                   |
| `lua_binary`        | `'lua'`                              | Configures the lua binary that Starship executes when getting the version.              |
| `disabled`          | `false`                              | Disables the `lua` module.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | The version of `lua`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` como `false` en tu archivo de configuración.

:::

### Opciones

| Opción      | Predeterminado                                  | Descripción                                              |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | El formato del módulo.                                   |
| `symbol`    | `'🐏'`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `'bold dimmed white'`                           | El estilo del módulo.                                    |
| `disabled`  | `true`                                          | Disables the `memory_usage` module.                      |

### Variables

| Variable         | Ejemplo       | Descripción                                                        |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Refleja el valor de la opción `symbol`                             |
| style\*        |               | Refleja el valor de la opción `style`                              |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

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

| Opción              | Predeterminado                     | Descripción                                                                               |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | El formato del módulo.                                                                    |
| `symbol`            | `'⬢ '`                             | The symbol used before displaying the project name.                                       |
| `style`             | `'blue bold'`                      | El estilo del módulo.                                                                     |
| `disabled`          | `false`                            | Disables the `meson` module.                                                              |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| proyecto  | `starship` | The current Meson project name         |
| symbol    | `🐏`        | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Opciones

| Opción              | Predeterminado                   | Descripción                                                                                  |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `' '`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `'bold purple'`                  | El estilo del módulo.                                                                        |
| `format`            | `'on [$symbol$branch]($style) '` | El formato del módulo.                                                                       |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `'…'`                            | El símbolo usado para indicar que un nombre de rama fue truncado.                            |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| rama      | `maestro` | The active mercurial branch            |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module                                                               |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'👑 '`                               | The symbol used before displaying the version of Nim.                                   |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['nim.cfg']`                        | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold yellow'`                      | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `nim` module.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.2.0` | The version of `nimc`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. The module will be shown when inside a nix-shell environment.

### Opciones

| Opción       | Predeterminado                                 | Descripción                                           |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | El formato del módulo.                                |
| `symbol`     | `'❄️ '`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `'bold blue'`                                  | El estilo del módulo.                                 |
| `impure_msg` | `'impure'`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `'pure'`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| state     | `pure`  | The state of the nix-shell             |
| name      | `lorri` | The name of the nix-shell              |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = '[impure shell](bold red)'
pure_msg = '[pure shell](bold green)'
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

### Opciones

| Opción              | Predeterminado                             | Descripción                                                                                           |
| ------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | El formato del módulo.                                                                                |
| `version_format`    | `'v${raw}'`                                | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch`               |
| `symbol`            | `' '`                                     | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']` | Qué extensiones deberían activar este módulo.                                                         |
| `detect_files`      | `['package.json', '.node-version']`        | Qué nombres de archivo deberían activar este módulo.                                                  |
| `detect_folders`    | `['node_modules']`                         | Qué carpetas deberían activar este módulo.                                                            |
| `style`             | `'bold green'`                             | El estilo del módulo.                                                                                 |
| `disabled`          | `false`                                    | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                                 | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | The version of `node`                  |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Opciones

| Opción                    | Predeterminado                                                             | Descripción                                                                             |
| ------------------------- | -------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | La cadena de formato para el módulo.                                                    |
| `version_format`          | `'v${raw}'`                                                                | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `'🐫 '`                                                                     | The symbol used before displaying the version of OCaml.                                 |
| `global_switch_indicator` | `''`                                                                       | The format string used to represent global OPAM switch.                                 |
| `local_switch_indicator`  | `'*'`                                                                      | The format string used to represent local OPAM switch.                                  |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | Qué carpetas deberían activar este módulo.                                              |
| `style`                   | `'bold yellow'`                                                            | El estilo del módulo.                                                                   |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                                            |

### Variables

| Variable         | Ejemplo      | Descripción                                                       |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Refleja el valor de la opción `symbol`                            |
| style\*        |              | Refleja el valor de la opción `style`                             |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool. By default the module will be shown if the current directory contains a `.rego` file.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🪖  '`                              | A format string representing the symbol of OPA.                                         |
| `detect_extensions` | `['rego']`                           | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold blue'`                        | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `opa` module.                                                              |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.44.0` | The version of `opa`                   |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Opciones

| Opción     | Predeterminado                                  | Descripción                                                    |
| ---------- | ----------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | El formato del módulo.                                         |
| `symbol`   | `'☁️ '`                                         | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `'bold yellow'`                                 | El estilo del módulo.                                          |
| `disabled` | `false`                                         | Disables the `openstack` module.                               |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | The current OpenStack cloud            |
| proyecto  | `dev`   | The current OpenStack project          |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## OS

The `os` module shows the current operating system. OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

::: warning

The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

:::

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` como `false` en tu archivo de configuración.

:::

### Opciones

| Opción     | Predeterminado        | Descripción                                            |
| ---------- | --------------------- | ------------------------------------------------------ |
| `format`   | `"[$symbol]($style)"` | El formato del módulo.                                 |
| `style`    | `"bold white"`        | El estilo del módulo.                                  |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type. Operating system types not defined by your configuration use the default symbols table below. All operating systems currently supported by the module are listed below. If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
[os.symbols]
Alpine = "🏔️ "
Amazon = "🙂 "
Android = "🤖 "
Arch = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Linux = "🐧 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
OpenBSD = "🐡 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Unknown = "❓ "
Windows = "🪟 "
```

### Variables

| Variable  | Ejemplo      | Descripción                                                        |
| --------- | ------------ | ------------------------------------------------------------------ |
| symbol    | `🎗️`         | The current operating system symbol from advanced option `symbols` |
| name      | `Arch Linux` | The current operating system name                                  |
| type      | `Arch`       | The current operating system type                                  |
| codename  |              | The current operating system codename, if applicable               |
| edition   |              | The current operating system edition, if applicable                |
| version   |              | The current operating system version, if applicable                |
| style\* |              | Refleja el valor de la opción `style`                              |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[os]
format = "on [($name )]($style)"
style = "bold blue"
disabled = false

[os.symbols]
Windows = " "
Arch = "Arch is the best! "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ La versión que se muestra es la del paquete cuyo código fuente está en tu directorio actual, no en tu gestor de paquetes.

### Opciones

| Opción            | Predeterminado                    | Descripción                                                                             |
| ----------------- | --------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `'is [$symbol$version]($style) '` | El formato del módulo.                                                                  |
| `symbol`          | `'📦 '`                            | The symbol used before displaying the version the package.                              |
| `version_format`  | `'v${raw}'`                       | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | El estilo del módulo.                                                                   |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.                               |
| `disabled`        | `false`                           | Disables the `package` module.                                                          |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.0.0` | The version of your package            |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Opciones

| Opción              | Predeterminado                                                                                           | Descripción                                                                             |
| ------------------- | -------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | La cadena de formato para el módulo.                                                    |
| `version_format`    | `'v${raw}'`                                                                                              | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐪 '`                                                                                                   | The symbol used before displaying the version of Perl                                   |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                     | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold 149'`                                                                                             | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                                             |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v5.26.1` | The version of `perl`                  |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐘 '`                               | The symbol used before displaying the version of PHP.                                   |
| `detect_extensions` | `['php']`                            | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['composer.json', '.php-version']`  | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'147 bold'`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `php` module.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v7.3.8` | The version of `php`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

::: tip

By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). If you still want to enable it, [follow the example shown below](#with-pulumi-version).

:::

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Opciones

| Opción           | Predeterminado                               | Descripción                                                                             |
| ---------------- | -------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | La cadena de formato para el módulo.                                                    |
| `version_format` | `'v${raw}'`                                  | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `' '`                                       | A format string shown before the Pulumi stack.                                          |
| `style`          | `'bold 5'`                                   | El estilo del módulo.                                                                   |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.                          |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                                           |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `pulumi`                |
| stack     | `dev`      | The current Pulumi stack               |
| username  | `alice`    | The current Pulumi username            |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `spago.dhall` file
- The current directory contains a file with the `.purs` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'<=> '`                       | The symbol used before displaying the version of PureScript.                            |
| `detect_extensions` | `['purs']`                           | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['spago.dhall']`                    | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold white'`                       | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `purescript` module.                                                       |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | The version of `purescript`            |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- A virtual environment is currently activated

### Opciones

| Opción               | Predeterminado                                                                                               | Descripción                                                                             |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | El formato del módulo.                                                                  |
| `version_format`     | `'v${raw}'`                                                                                                  | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`             | `'🐍 '`                                                                                                       | A format string representing the symbol of Python                                       |
| `style`              | `'yellow bold'`                                                                                              | El estilo del módulo.                                                                   |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                         |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                         |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should executes when getting the version.  |
| `detect_extensions`  | `['py']`                                                                                                     | Qué extensiones deben activar este módulo                                               |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Qué nombres de archivo deben activar este módulo                                        |
| `detect_folders`     | `[]`                                                                                                         | Qué carpetas deben activar este módulo                                                  |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                           |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `'python3'` will hide any Python version 2, see example below.

:::

### Variables

| Variable     | Ejemplo         | Descripción                                |
| ------------ | --------------- | ------------------------------------------ |
| version      | `'v3.8.1'`      | The version of `python`                    |
| symbol       | `'🐍 '`          | Refleja el valor de la opción `symbol`     |
| style        | `'yellow bold'` | Refleja el valor de la opción `style`      |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `'venv'`        | The current `virtualenv` name              |

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
# Solo usa el binario `python3` para obtener la versión.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# No se dispara con archivos con extensión py
detect_extensions = []
```

```toml
# ~/.config/starship.toml

[python]
# Muestra la versión de python desde dentro de un entorno virtual local.
#
# Ten en cuenta que esto solo funcionará cuando el venv esté dentro del proyecto y sólo
# funcionará en el directorio que contiene el directorio venv dir pero ¿tal vez esté bien?
python_binary = ['./venv/bin/python', 'python', 'python3', 'python2']
```

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'📐'`                                | A format string representing the symbol of R.                                           |
| `style`             | `'blue bold'`                        | El estilo del módulo.                                                                   |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Qué extensiones deben activar este módulo                                               |
| `detect_files`      | `['.Rprofile']`                      | Qué nombres de archivo deben activar este módulo                                        |
| `detect_folders`    | `['.Rproj.user']`                    | Qué carpetas deben activar este módulo                                                  |
| `disabled`          | `false`                              | Disables the `r` module.                                                                |

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

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Opciones

| Opción              | Predeterminado                                   | Descripción                                                                             |
| ------------------- | ------------------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | La cadena de formato para el módulo.                                                    |
| `version_format`    | `'v${raw}'`                                      | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦋 '`                                           | The symbol used before displaying the version of Raku                                   |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['META6.json']`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                             | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold 149'`                                     | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                          | Disables the `raku` module.                                                             |

### Variables

| Variable   | Ejemplo | Descripción                            |
| ---------- | ------- | -------------------------------------- |
| version    | `v6.d`  | The version of `raku`                  |
| vm_version | `moar`  | The version of VM `raku` is built on   |
| symbol     |         | Refleja el valor de la opción `symbol` |
| style\*  |         | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.red` or `.reds` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🔺 '`                               | A format string representing the symbol of Red.                                         |
| `detect_extensions` | `['red']`                            | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'red bold'`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `red` module.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `red`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💎 '`                               | A format string representing the symbol of Ruby.                                        |
| `detect_extensions` | `['rb']`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Which environment variables should trigger this module.                                 |
| `style`             | `'bold red'`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `ruby` module.                                                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `ruby`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦀 '`                               | A format string representing the symbol of Rust                                         |
| `detect_extensions` | `['rs']`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['Cargo.toml']`                     | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold red'`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `rust` module.                                                             |

### Variables

| Variable  | Ejemplo           | Descripción                                  |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | Refleja el valor de la opción `symbol`       |
| style\* |                   | Refleja el valor de la opción `style`        |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Opciones

| Opción              | Predeterminado                           | Descripción                                                                             |
| ------------------- | ---------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                              | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `['.metals']`                            | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `'🆂 '`                                   | A format string representing the symbol of Scala.                                       |
| `style`             | `'red dimmed'`                           | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                  | Disables the `scala` module.                                                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `2.13.5` | The version of `scala`                 |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` como `false` en tu archivo de configuración.

:::

### Opciones

| Opción                 | Predeterminado            | Descripción                                                  |
| ---------------------- | ------------------------- | ------------------------------------------------------------ |
| `bash_indicator`       | `'bsh'`                   | A format string used to represent bash.                      |
| `fish_indicator`       | `'fsh'`                   | A format string used to represent fish.                      |
| `zsh_indicator`        | `'zsh'`                   | A format string used to represent zsh.                       |
| `powershell_indicator` | `'psh'`                   | A format string used to represent powershell.                |
| `ion_indicator`        | `'ion'`                   | A format string used to represent ion.                       |
| `elvish_indicator`     | `'esh'`                   | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `'tsh'`                   | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `'xsh'`                   | A format string used to represent xonsh.                     |
| `cmd_indicator`        | `'cmd'`                   | A format string used to represent cmd.                       |
| `nu_indicator`         | `'nu'`                    | A format string used to represent nu.                        |
| `unknown_indicator`    | `''`                      | The default value to be displayed when the shell is unknown. |
| `format`               | `'[$indicator]($style) '` | El formato del módulo.                                       |
| `style`                | `'white bold'`            | El estilo del módulo.                                        |
| `disabled`             | `true`                    | Disables the `shell` module.                                 |

### Variables

| Variable  | Predeterminado | Descripción                                                |
| --------- | -------------- | ---------------------------------------------------------- |
| indicator |                | Mirrors the value of `indicator` for currently used shell. |
| style\* |                | Mirrors the value of option `style`.                       |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplos

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ''
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Opciones

| Opción      | Predeterminado               | Descripción                                                   |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `'[$symbol$shlvl]($style) '` | El formato del módulo.                                        |
| `symbol`    | `'↕️  '`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `'bold yellow'`              | El estilo del módulo.                                         |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | The current value of `SHLVL`           |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable solamente puede ser usada como parte de una cadena de caracteres de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### Opciones

| Opción     | Predeterminado                   | Descripción                                      |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | El formato del módulo.                           |
| `symbol`   | `''`                             | A format string displayed before the image name. |
| `style`    | `'bold dimmed blue'`             | El estilo del módulo.                            |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

| Variable  | Ejemplo      | Descripción                            |
| --------- | ------------ | -------------------------------------- |
| env       | `centos.img` | The current Singularity image          |
| symbol    |              | Refleja el valor de la opción `symbol` |
| style\* |              | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Opciones

| Opción              | Predeterminado                         | Descripción                                                                                                                                             |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to. `0` significa sin truncamiento. Vea también el módulo [`directory`](#directory). |
| `symbol`            | `'🅢  '`                                | El símbolo usado antes del nombre del entorno.                                                                                                          |
| `style`             | `'bold blue'`                          | El estilo del módulo.                                                                                                                                   |
| `format`            | `'via [$symbol$environment]($style) '` | El formato del módulo.                                                                                                                                  |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                            |

### Variables

| Variable    | Ejemplo      | Descripción                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | The current spack environment          |
| symbol      |              | Refleja el valor de la opción `symbol` |
| style\*   |              | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Status

The `status` module displays the exit code of the previous command. If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`. The status code will cast to a signed 32-bit integer.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` como `false` en tu archivo de configuración.

:::

### Opciones

| Opción                      | Predeterminado                                                                     | Descripción                                                           |
| --------------------------- | ---------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                      | El formato del módulo                                                 |
| `symbol`                    | `'❌'`                                                                              | The symbol displayed on program error                                 |
| `success_symbol`            | `''`                                                                               | The symbol displayed on program success                               |
| `not_executable_symbol`     | `'🚫'`                                                                              | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `'🔍'`                                                                              | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `'🧱'`                                                                              | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `'⚡'`                                                                              | The symbol displayed on any signal                                    |
| `style`                     | `'bold red'`                                                                       | El estilo del módulo.                                                 |
| `recognize_signal_code`     | `true`                                                                             | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                            | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                            | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                          | The symbol used to separate pipestatus segments (supports formatting) |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style)'` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                    | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                             | Disables the `status` module.                                         |

### Variables

| Variable       | Ejemplo | Descripción                                                                                 |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                           |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                    |
| int            | `127`   | The exit code of the last command                                                           |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                         |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                |
| pipestatus     |         | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbol         |         | Refleja el valor de la opción `symbol`                                                      |
| style\*      |         | Refleja el valor de la opción `style`                                                       |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

The `sudo` module displays if sudo credentials are currently cached. The module will only be shown if credentials are cached.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` como `false` en tu archivo de configuración.

:::

### Opciones

| Opción          | Predeterminado           | Descripción                                             |
| --------------- | ------------------------ | ------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | El formato del módulo                                   |
| `symbol`        | `'🧙 '`                   | The symbol displayed when credentials are cached        |
| `style`         | `'bold blue'`            | El estilo del módulo.                                   |
| `allow_windows` | `false`                  | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                   | Disables the `sudo` module.                             |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[sudo]
style = 'bold green'
symbol = '👩‍💻 '
disabled = false
```

```toml
# En Windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐦 '`                               | A format string representing the symbol of Swift                                        |
| `detect_extensions` | `['swift']`                          | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['Package.swift']`                  | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold 202'`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `swift` module.                                                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.2.4` | The version of `swift`                 |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.

::: tip

By default the Terraform version is not shown, since this is slow for current versions of Terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

:::

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '` | La cadena de formato para el módulo.                                                    |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💠'`                                | A format string shown before the terraform workspace.                                   |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`        | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `['.terraform']`                     | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'bold 105'`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `terraform` module.                                                        |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `terraform`             |
| workspace | `default`  | The current Terraform workspace        |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

#### Con la versión de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $version$workspace]($style) '
```

#### Sin la versión de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $workspace]($style) '
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` como `false` en tu archivo de configuración.

:::

### Opciones

| Opción            | Predeterminado          | Descripción                                                                                                                        |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | La cadena de formato para el módulo.                                                                                               |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `'bold yellow'`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `'local'`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `'-'`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable  | Ejemplo    | Descripción                           |
| --------- | ---------- | ------------------------------------- |
| time      | `13:08:10` | The current time.                     |
| style\* |            | Refleja el valor de la opción `style` |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = '%T'
utc_time_offset = '-5'
time_range = '10:00:00-14:00:00'
```

## Username

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- The current user is root/admin
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Opciones

| Opción        | Predeterminado          | Descripción                                 |
| ------------- | ----------------------- | ------------------------------------------- |
| `style_root`  | `'bold red'`            | The style used when the user is root/admin. |
| `style_user`  | `'bold yellow'`         | The style used for non-root users.          |
| `format`      | `'[$user]($style) in '` | El formato del módulo.                      |
| `show_always` | `false`                 | Always shows the `username` module.         |
| `disabled`    | `false`                 | Disables the `username` module.             |

### Variables

| Variable | Ejemplo      | Descripción                                                                                 |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `'matchai'`  | The currently logged-in user ID.                                                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[username]
style_user = 'white bold'
style_root = 'black bold'
format = 'user: [$user]($style) '
disabled = false
show_always = true
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `Vagrantfile` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⍱ '`                               | A format string representing the symbol of Vagrant.                                     |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['Vagrantfile']`                    | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'cyan bold'`                        | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                                          |

### Variables

| Variable  | Ejemplo          | Descripción                            |
| --------- | ---------------- | -------------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`               |
| symbol    |                  | Refleja el valor de la opción `symbol` |
| style\* |                  | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Opciones

| Opción              | Predeterminado                               | Descripción                                                                             |
| ------------------- | -------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`         | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                                  | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'V '`                                       | A format string representing the symbol of V                                            |
| `detect_extensions` | `['v']`                                      | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                         | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `'blue bold'`                                | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                                            |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| version   | `v0.2`  | The version of `v`                     |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### Opciones

| Opción     | Predeterminado                   | Descripción                                            |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   | `''`                             | The symbol used before displaying the repository name. |
| `style`    | `'bold yellow'`                  | El estilo del módulo.                                  |
| `format`   | `'vcsh [$symbol$repo]($style) '` | El formato del módulo.                                 |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variables

| Variable  | Ejemplo                                     | Descripción                            |
| --------- | ------------------------------------------- | -------------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name             |
| symbol    |                                             | Refleja el valor de la opción `symbol` |
| style\* | `black bold dimmed`                         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | El formato del módulo.                                                                  |
| `version_format`    | `'v${raw}'`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'↯ '`                               | The symbol used before displaying the version of Zig.                                   |
| `style`             | `'bold yellow'`                      | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `zig` module.                                                              |
| `detect_extensions` | `['zig']`                            | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.6.0` | The version of `zig`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `detect_files`
- The current directory contains a directory whose name is in `detect_folders`
- The current directory contains a file whose extension is in `detect_extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

::: warning Command output is printed unescaped to the prompt

Whatever output the command generates is printed unmodified in the prompt. This means if the output contains special sequences that are interpreted by your shell they will be expanded when displayed. These special sequences are shell specific, e.g. you can write a command module that writes bash sequences, e.g. `\h`, but this module will not work in a fish or zsh shell.

Format strings can also contain shell specific prompt sequences, e.g. [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

:::

### Opciones

| Opción              | Predeterminado                  | Descripción                                                                                                                                                                                                                                                                                   |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                                                                                                                                 |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the command returns a `0` status code.                                                                          |
| `shell`             |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                            |
| `description`       | `'<custom module>'`       | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                  |
| `detect_files`      | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                         |
| `detect_folders`    | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                   |
| `detect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                    |
| `symbol`            | `''`                            | The symbol used before displaying the command output.                                                                                                                                                                                                                                         |
| `style`             | `'bold green'`                  | El estilo del módulo.                                                                                                                                                                                                                                                                         |
| `format`            | `'[$symbol($output )]($style)'` | El formato del módulo.                                                                                                                                                                                                                                                                        |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                |
| `os`                |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                 |
| `use_stdin`         |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                     |

### Variables

| Variable  | Descripción                            |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Refleja el valor de la opción `symbol` |
| style\* | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

#### Comando personalizado del intérprete de comandos

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to 'sh' on Linux, and 'cmd /C' on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. If `shell` is not given or only contains one element and Starship detects Cmd will be used, the following argument will automatically be added: `/C` and `stdin` will be set to `false`. If `shell` is not given or only contains one element and Starship detects Nushell will be used, the following arguments will automatically be added: `-c` and `stdin` will be set to `false`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ['pwsh', '-Command', '-']
```

::: warning Make sure your custom shell configuration exits gracefully

If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).

For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.

Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.

Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

:::

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
