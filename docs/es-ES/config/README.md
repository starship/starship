# Configuración

Para iniciar la configuración de starship, crea el siguiente fichero: `~/.config.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toda la configuración de starship se incluye en este fichero [TOML](https://github.com/toml-lang/toml):

```toml
# Obtiene las completaciones del editor basadas en el esquema de configuración
"$schema" = 'https://starship.rs/config-schema.json'

# Inserta una línea en blanco entre los prompts del intérprete de comandos
add_newline = true

# Reemplaza el símbolo "❯" en el prompt por "➜"
[character] # El nombre del módulo que estamos configurando es "character"
success_symbol = "[➜](bold green)" # El segmento "success_symbol" se está oonfigurando es "➜" con el color "bold green"

# Deshabilita el módulo "package", ocultándolo del prompt completamente
[package]
disabled = true
```

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

Por convención, la mayoría de los módulos tienen un prefijo del color predeterminado de la terminal (por ejemplo, `vía` en "nodejs") y un espacio vacío como sufijo.

### Cadenas de Formato

Es el formato con el que un módulo imprime todas sus variables. La mayoría de los módulos tienen una entrada llamada `format` que configura el formato de visualización del módulo. Se puede utilizar textos, variables y grupos de texto.

#### Variable

Una variable contiene un símbolo `$` seguido por el nombre de la variable. El nombre de una variable solamente puede contener letras, números y `_`.

Por ejemplo:

- `$version` es una cadena de formato con una variable llamada `version`.
- `$git_branch$git_commit` es un formato de cadena de texto con dos variables nombradas `git_branch` y `git_commit`.
- `$git_branch $git_commit` tiene las dos variables separadas por un espacio.

#### Grupo de Texto

Un grupo de texto se compone de dos partes diferentes.

La primera parte, que está encerrada en un `[]`, es un [formato de cadena de texto](#format-strings). Se puede agregar textos, variables, o incluso grupos de texto anidados.

En la segunda parte, que está encerrada en un `()`, es un [formato de cadena de texto](#style-strings). Esto se puede utilizar para diseñar la primera parte.

Por ejemplo:

- `[en](bold red)` imprimirá una cadena `en` con texto en negrita color rojo.
- `[⌘ $version](bold green)` imprimirá un símbolo `⌘` seguido por el contenido de la variable `version`, con texto en negrita color verde.
- `[a [b](red) c](green)` imprimirá `a b c` con `b` en rojo, `a` y `c` en verde.

#### Cadenas de Estilo

La mayoría de los módulos de starship permiten configurar los estilos de su cadenas texto. Esto se consigue con una entrada (normalmente llamada `style` - estilo) que no es más que un texto donde se especifica la configuración. A continuación mostramos algunos ejemplos de textos estilados junto con su funcionalidad. Para más detalles sobre la sintaxis completa, consulta [la guía de configuración avanzada](/advanced-config/).

- `"fg:green bg:blue"` pone texto verde sobre un fondo azul
- `"bg:blue fg:bright-green"` pone texto verde claro sobre un fondo azul
- `"bold fg:27"` pone texto en negrita con [color ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` subraya el texto sobre un fondo naranja oscuro
- `"bold italic fg:purple"` pone texto color morado, en negrita y cursiva
- `""` desactiva explícitamente cualquier estilo

Nótese que el estilo es similar a como se controlaría por el emulador de su terminal. Por ejemplo, algunos emuladores de terminal harán los colores más brillantes en lugar de más gruesos, y algunos temas usan los mismos valores para texto normal y colores brillantes. Además, para mostrar textos en cursiva tu terminal debe tener soporte para hacerlo.

#### Cadenas de Formato Condicional

Una cadena de formato condicional envuelto en `(` y `)` no se renderizará si todas las variables dentro están vacías.

Por ejemplo:

- `(@$region)` no mostrará nada si la variable `region` es `None` o una cadena vacía, de lo contrario `@` seguido por el valor de la región.
- `(algún texto)` siempre mostrará nada ya que no hay variables envueltas entre llaves.
- Cuando `$all` es un atajo para `\[$a$b\]`, `($all)` no mostrará nada solo si `$a` y `$b` ambos son `None`. Esto funciona igual que `(\[$a$b\] )`.

#### Caracteres especiales

Los siguientes símbolos tienen un uso especial en una cadena de formato y deben ser escapados: `$ \ [ ] ( )`.

Ten en cuenta que TOML tiene [cadenas de caracteres básicas y cadenas de caracteres literales](https://toml.io/en/v1.0.0#string). Se recomienda usar una cadena de caracteres literal (rodeada de comillas simples) en tu configuración. Si quieres utilizar una cadena de caracteres básica (rodeada de comillas dobles), debes escapar la barra inversa, en sí misma (es decir, utilizar `\\`).

Por ejemplo, cuando se desea imprimir un símbolo `$` en una nueva línea, las siguientes configuraciones para el `format` son equivalentes:

```toml
# con cadena básica
format = "\n\\$"

# con cadena básica multilínea
format = """

\\$"""

# con cadena literal
format = '''

\$'''
```

### Coincidencia negativa

Varios módulos tienen variables `detect_extensions`, `detect_files`y `detect_folders`. Estas toman listas de cadenas de texto para que coincidan o no coincidan. Las opciones "negativas", aquellas que no deben coincidir, están indicadas con un carácter "!" al inicio. La presencia de _cualquier_ indicador negativo en el directorio hará que el módulo no coincida.

Las extensiones coinciden tanto con los caracteres después del último punto en un nombre de archivo, como con los caracteres después del primer punto en un nombre de archivo. Por ejemplo, `foo.bar.tar.gz` será emparejado contra `bar.tar.gz` y `gz` en la variable `detect_extensions`. Los archivos cuyo nombre comienza con un punto no se consideran extensiones en absoluto.

Para ver cómo funciona esto en la práctica, puede hacer coincidir con archivos TypeScript, pero no con archivos MPEG Transport Stream, así:

```toml
detect_extensions = ["ts", "!video.ts", "!audio.ts"]
```

## Prompt

Esta es la lista de opciones de configuración.

### Opciones

| Opción            | Predeterminado                 | Descripción                                                                    |
| ----------------- | ------------------------------ | ------------------------------------------------------------------------------ |
| `format`          | [link](#default-prompt-format) | Configura el formato del prompt.                                               |
| `right_format`    | `""`                           | Ver [Habilitar prompt derecho](/advanced-config/#enable-right-prompt)          |
| `scan_timeout`    | `30`                           | Tiempo de espera tras el que starship escanea archivos (en milisegundos).      |
| `command_timeout` | `500`                          | Tiempo de espera para los comandos ejecutados por starship (en milisegundos).  |
| `add_newline`     | `true`                         | Inserta un línea en blanco entre las instrucciones del intérprete de comandos. |

### Ejemplo

```toml
# ~/.config/starship.toml

# Usar formato personalizado
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Espera 10 milisegundos para que starship revise los archivos del directorio actual.
scan_timeout = 10

# Deshabilita la línea en blanco al inicio del prompt
add_newline = false
```

### Formato predeterminado del Prompt

El `format` predeterminado se utiliza para definir el formato del prompt, si está vacío o no `format` se proporciona. El valor por defecto es el siguiente:

```toml
format = "$all"

# Lo cual es equivalente a:
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
$haskell\
$helm\
$java\
$julia\
$kotlin\
$lua\
$nim\
$nodejs\
$ocaml\
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
$container\
$shell\
$character"""
```

Si solo quieres extender el formato predeterminado, puedes usar `$all`; los módulos que se añaden explícitamente al formato no serán duplicados. Por ejemplo.

```toml
# Mueve el módulo directorio a la segunda línea
format="$all$directory$character"
```

## AWS

El módulo `aws` muestra la región y el perfil actual de AWS y un temporizador de vencimiento cuando se utilizan credenciales temporales. La salida del módulo utiliza las variables de entorno `AWS_REGION`, `AWS_DEFAULT_REGION` y `AWS_PROFILE`, y los archivos `~/. ws/config` y `~/.aws/credenciales` según sea necesario.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process` or `sso_start_url` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice. If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

Cuando se utiliza [aws-vault](https://github.com/99designs/aws-vault), el perfil se lee de la variable de entorno `AWS_VAULT` y la fecha de expiración de credenciales se lee de la variable de entorno `AWS_SESSION_EXPIRATION`.

Cuando uses [awsu](https://github.com/kreuzwerker/awsu) el perfil se obtiene de la variable de entorno `AWSU_PROFILE`.

Cuando se utiliza [AWSume](https://awsu.me), el perfil se lee de la variable de entorno `AWSUME_PROFILE` y la fecha de expiración de credenciales se lee de la variable de entorno `AWSUME_EXPIRATION`.

### Opciones

| Opción              | Por defecto                                                           | Descripción                                                                                                              |
| ------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | El formato del módulo.                                                                                                   |
| `symbol`            | `"☁️ "`                                                               | El símbolo que se muestra antes del perfil de AWS.                                                                       |
| `region_aliases`    |                                                                       | Tabla de alias de región para mostrar además del nombre AWS.                                                             |
| `profile_aliases`   |                                                                       | Tabla de alias de región para mostrar además del nombre AWS.                                                             |
| `style`             | `"bold yellow"`                                                       | El estilo del módulo.                                                                                                    |
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

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplos

#### Mostrar todo

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
```

#### Mostrar región

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$region]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Mostrar perfil

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

El módulo `azure` muestra la suscripción Azure actual. Esto consiste en mostrar el nombre de la suscripción predeterminada, como se define en el archivo `~/.azure/azureProfile.json`.

### Opciones

| Variable   | Por defecto                              | Descripción                                 |
| ---------- | ---------------------------------------- | ------------------------------------------- |
| `format`   | `"on [$symbol($subscription)]($style) "` | El formato para renderizar el módulo Azure. |
| `symbol`   | `"ﴃ "`                                   | El símbolo utilizado en el formato.         |
| `style`    | `"blue bold"`                            | El estilo utilizado en el formato.          |
| `disabled` | `true`                                   | Deshabilita el módulo `azure`.              |

### Ejemplo

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($subscription)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## Battery

El módulo `battery` muestra la cantidad de batería y si está cargando o no. El módulo es solo visible cuando la batería está por debajo del 10%.

### Opciones

| Opción               | Por defecto                       | Descripción                                                              |
| -------------------- | --------------------------------- | ------------------------------------------------------------------------ |
| `full_symbol`        | `" "`                            | Se muestra cuando la batería está cargada.                               |
| `charging_symbol`    | `" "`                            | Se muestra cuando la batería está cargando.                              |
| `discharging_symbol` | `" "`                            | Se muestra cuando la batería se está descargando.                        |
| `unknown_symbol`     | `" "`                            | El símbolo que se muestra cuando el estado de la batería es desconocido. |
| `empty_symbol`       | `" "`                            | El símbolo que se muestra cuando el estado de la batería está vacío.     |
| `format`             | `"[$symbol$percentage]($style) "` | El formato del módulo.                                                   |
| `display`            | [link](#battery-display)          | Define cuándo mostrar el indicador y el estilo.                          |
| `disabled`           | `false`                           | Desactiva el módulo `battery`.                                           |

### Ejemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### Indicador de batería

La opción de configuración `display` se utiliza para definir cuándo debe mostrarse el indicador de batería (threshold), qué symbol se utilizaría (symbol), y cómo sería (style). Si no se provee ningún valor para `display`  El valor por defecto es el siguiente:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

El valor por defecto para las opciones `charging_symbol` y `discharging_symbol` son respectivamente los valores `charging_symbol` y `discharging_symbol` de las opción de `battery`.

#### Opciones

La opción `display` es un array de la siguiente tabla.

| Opción               | Por defecto | Descripción                                                                                                                             |
| -------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`        | El umbral para la opción de visualización.                                                                                              |
| `style`              | `bold red`  | El estilo usado cuando si la opción <0>display</0> está activa.                                                                         |
| `charging_symbol`    | `-`         | Símbolo opcional que se muestra si la opción de visualización está en uso, por defecto en la opción `charging_symbol` de la batería.    |
| `discharging_symbol` | `-`         | Símbolo opcional que se muestra si la opción de visualización está en uso, por defecto en la opción `discharging_symbol` de la batería. |

#### Ejemplo

```toml
[[battery.display]] # Estilo "rojo fuerte" y símbolo de descarga cuando la capacidad está entre 0% y 10%
threshold = 10
style = "bold red"

[[battery.display]] # Estilo "amarillo fuerte" y símbolo 💦 cuando la capacidad está entre 10% y 30%
threshold = 30
style = "bold yellow"
discharging_symbol = "💦"

# cuando la capacidad está sobre el 30%, el indicador de batería no se visualizara
```

## Buf

El módulo `buf` muestra la versión instalada de [Buf](https://buf.build). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- La CLI de [`buf`](https://github.com/bufbuild/buf) está instalada.
- El directorio actual contiene un archivo de configuración [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), o [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml).

### Opciones

| Opción              | Por defecto                                                  | Descripción                                          |
| ------------------- | ------------------------------------------------------------ | ---------------------------------------------------- |
| `format`            | `'with [$symbol($version \(Buf $buf_version\) )]($style)'` | El formato para el módulo `buf`.                     |
| `version_format`    | `"v${raw}"`                                                  | El formato de versión.                               |
| `symbol`            | `"🦬 "`                                                       | El símbolo usado antes de mostrar la versión de Buf. |
| `detect_extensions` | `[]`                                                         | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `["buf.yaml", "buf.gen.yaml", "buf.work.yaml"]`              | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                                         | Qué carpetas deberían activar estos módulos.         |
| `style`             | `"bold blue"`                                                | El estilo del módulo.                                |
| `disabled`          | `false`                                                      | Desactiva el módulo `elixir`.                        |

### Variables

| Variable      | Ejemplo  | Descripción                            |
| ------------- | -------- | -------------------------------------- |
| `buf_version` | `v1.0.0` | La versión de `buf`                    |
| `symbol`      |          | Refleja el valor de la opción `symbol` |
| `style`*      |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[buf]
symbol = "🦬 "
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `bun.lockb`
- El directorio actual contiene un fichero `bunfig.toml`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🍞 "`                               | Una cadena de formato que representa el símbolo de Node.js.                             |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["bun.lockb", "bunfig.toml"]`       | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold red"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `bun` module.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.1.4` | The version of `bun`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[bun]
format = "via [🍔 $version](bold green) "
```

## C

El módulo `c` muestra información sobre su compilador de C. Por defecto el módulo se mostrará si el directorio actual contiene un archivo `.c` o `.h`.

### Opciones

| Opción              | Por defecto                                                                 | Descripción                                                                             |
| ------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version(-$name) )]($style)"`                                | La cadena de formato para el módulo.                                                    |
| `version_format`    | `"v${raw}"`                                                                 | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"C "`                                                                      | El símbolo usado antes de mostrar los detalles del compilador                           |
| `detect_extensions` | `["c", "h"]`                                                                | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                                                        | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                        | Qué carpetas deberían activar este módulo.                                              |
| `commands`          | [ [ "cc", "--version" ], [ "gcc", "--version" ], [ "clang", "--version" ] ] | Cómo detectar cuál compilador es                                                        |
| `style`             | `"bold 149"`                                                                | El estilo del módulo.                                                                   |
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

Cada comando se representa como una lista del nombre del ejecutable seguido de sus argumentos, generalmente algo como `["mycc", "--version"]`. Starship intentará ejecutar cada comando hasta que obtenga un resultado en STDOUT.

Si un compilador C no es compatible con este módulo, puede solicitarlo [planteando un problema en GitHub](https://github.com/starship/starship/).

### Ejemplo

```toml
# ~/.config/starship.toml

[c]
format = "via [$name $version]($style)"
```

## Character

El módulo `character` muestra un carater (normalmente una flecha) tras el texto que introduces en el terminal.

El carácter te dirá si el último comando funcionó o no. Se puede hacer de dos maneras:

- Cambiando el color (`red`/`green`)
- Cambiando la forma (`.`/`✖`)

Por defecto sólo cambia el color. Si también se quiere cambiar su forma, ver [este ejemplo](#with-custom-error-shape).

::: warning

`vicmd_symbol` solo es compatible con cmd, fish y zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

:::

### Opciones

| Opción                      | Por defecto          | Descripción                                                                                             |
| --------------------------- | -------------------- | ------------------------------------------------------------------------------------------------------- |
| `format`                    | `"$symbol "`         | La cadena de formato usada antes de la entrada de texto.                                                |
| `success_symbol`            | `"[❯](bold green)"`  | La cadena de formato usada antes de la entrada de texto si el comando anterior tuvo éxito.              |
| `error_symbol`              | `"[❯](bold red)"`    | La cadena de formato usada antes de la entrada de texto si el comando anterior falló.                   |
| `vimcmd_symbol`             | `"[❮](bold green)"`  | El cadena de formato antes de la entrada de texto si el intérprete de comandos está en modo vim normal. |
| `vimcmd_replace_one_symbol` | `"[❮](bold purple)"` | The format string used before the text input if the shell is in vim `replace_one` mode.                 |
| `vimcmd_replace_symbol`     | `"[❮](bold purple)"` | The format string used before the text input if the shell is in vim replace mode.                       |
| `vimcmd_visual_symbol`      | `"[❮](bold yellow)"` | The format string used before the text input if the shell is in vim replace mode.                       |
| `disabled`                  | `false`              | Desactiva el módulo `character`.                                                                        |

### Variables

| Variable | Ejemplo | Descripción                                                    |
| -------- | ------- | -------------------------------------------------------------- |
| symbol   |         | Un espejo de `success_symbol`, `error_symbol` o `vicmd_symbol` |

### Ejemplos

#### Con formato de error personalizado

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Sin formato de error personalizado

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### Con formato de vim personalizado

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

El módulo `cmake` muestra la versión actualmente instalada de [CMake](https://cmake.org/). Por defecto el módulo se activará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `CMakeLists.txt`
- El directorio actual contiene un archivo `CMakeCache.txt`

### Opciones

| Opción              | Por defecto                            | Descripción                                                                             |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`   | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                            | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"△ "`                                 | El símbolo usado antes de la versión de cmake.                                          |
| `detect_extensions` | `[]`                                   | Qué extensiones deben activar este módulo                                               |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Qué nombres de archivo deben activar este módulo                                        |
| `detect_folders`    | `[]`                                   | Qué carpetas deben activar este módulo                                                  |
| `style`             | `"bold blue"`                          | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                | Desactiva el módulo `cmake`.                                                            |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | La versión de cmake                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

## COBOL / GNUCOBOL

El módulo `cobol` muestra la versión instalada de COBOL. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene cualquier archivo que termine en `.cob` o `.COB`
- El directorio actual contiene cualquier archivo que termine en `.cbl` o `.CBL`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `symbol`            | `"⚙️ "`                              | El símbolo usado antes de mostrar la versión de COBOL.                                  |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold blue"`                        | El estilo del módulo.                                                                   |
| `detect_extensions` | `["cbl", "cob", "CBL", "COB"]`       | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `disabled`          | `false`                              | Deshabilita el módulo `cobol`.                                                          |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v3.1.2.0` | La versión de `cobol`                  |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

## Tiempo de ejecución

El módulo `cmd_duration` muestra cuánto tiempo tardó el último comando en ejecutarse. El módulo se mostrará solo si el comando llevó dos segundos o más, o el valor de `min_time`, si existe.

::: warning No utilizar DEBUG en Bash

Si estás usando Starship con `bash`, no uses `DEBUG` después de ejecutar `eval $(starship init $0)`, o el módulo **se romperá**.

:::

Los usuarios de bash que necesiten la funcionalidad preexec-like pueden usar el framework rcaloras's bash_preexec. Simplemente define los arrays preexec_functions y precmd_functions antes de ejecutar eval $(starship init $0), y continúa con normalidad. Basta con definir los arrays `preexec_functions` y `precmd_functions` antes de ejecutar `eval $(starship init $0)`, y luego proceder como siempre.

### Opciones

| Opción                 | Por defecto                   | Descripción                                                                                                                                                                                                  |
| ---------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `min_time`             | `2_000`                       | Duración mínima para mostrar el tiempo de ejecución (en milisegundos)                                                                                                                                        |
| `show_milliseconds`    | `false`                       | Muestra la duración con precisión en milisegundos.                                                                                                                                                           |
| `format`               | `"took [$duration]($style) "` | El formato del módulo.                                                                                                                                                                                       |
| `style`                | `"bold yellow"`               | El estilo del módulo.                                                                                                                                                                                        |
| `disabled`             | `false`                       | Desactiva el módulo `cmd_duration`.                                                                                                                                                                          |
| `show_notifications`   | `false`                       | Muestra notificaciones de escritorio cuando se complete el comando.                                                                                                                                          |
| `min_time_to_notify`   | `45_000`                      | Duración mínima para mostrar el tiempo de ejecución (en milisegundos).                                                                                                                                       |
| `notification_timeout` |                               | Duración para mostrar la notificación (en milisegundos). Si no se establece, el tiempo de espera para notificar será determinado por el demonio. No todos los demonios de notificaciones honran esta opción. |

### Variables

| Variable  | Ejemplo  | Descripción                                |
| --------- | -------- | ------------------------------------------ |
| duration  | `16m40s` | El tiempo que tardó en ejecutar el comando |
| style\* |          | Refleja el valor de la opción `style`      |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

El módulo `conda` muestra el entorno actual [Conda](https://docs.conda.io/en/latest/), si `$CONDA_DEFAULT_ENV` está configurado.

::: tip

Esto no modifica el propio símbolo de sistema de conda. En caso de querer suprimirlo, ejecuta `conda config --set changeps1 False`.

:::

### Opciones

| Opción              | Por defecto                            | Descripción                                                                                                                                                                                                             |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | El número de directorios a los que se debe truncar la variable de entorno, si el entorno fue creado usando `conda create -p [path]`. `0` significa sin truncamiento. Mirar también el módulo [`directory`](#directory). |
| `symbol`            | `"🅒 "`                                 | El símbolo usado antes del nombre del entorno.                                                                                                                                                                          |
| `style`             | `"bold green"`                         | El estilo del módulo.                                                                                                                                                                                                   |
| `format`            | `"via [$symbol$environment]($style) "` | El formato del módulo.                                                                                                                                                                                                  |
| `ignore_base`       | `true`                                 | Ignora el entorno `base` cuando se activa.                                                                                                                                                                              |
| `disabled`          | `false`                                | Desactiva el módulo `conda`.                                                                                                                                                                                            |

### Variables

| Variable    | Ejemplo      | Descripción                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | El entorno conda actual                |
| symbol      |              | Refleja el valor de la opción `symbol` |
| style\*   |              | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Contenedor

El módulo `container` muestra el símbolo y nombre del contenedor, si está dentro de un contenedor.

### Opciones

| Opción     | Por defecto                            | Descripción                                                      |
| ---------- | -------------------------------------- | ---------------------------------------------------------------- |
| `symbol`   | `"⬢"`                                  | El símbolo mostrado, cuando se encuentra dentro de un contenedor |
| `style`    | `"bold red dimmed"`                    | El estilo del módulo.                                            |
| `format`   | `"[$symbol \\[$name\\]]($style) "` | El formato del módulo.                                           |
| `disabled` | `false`                                | Deshabilita el módulo `container`.                               |

### Variables

| Variable  | Ejemplo             | Descripción                            |
| --------- | ------------------- | -------------------------------------- |
| name      | `fedora-toolbox:35` | El nombre del contenedor               |
| symbol    |                     | Refleja el valor de la opción `symbol` |
| style\* |                     | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[container]
format = "[$symbol \\[$name\\]]($style) "
```

## Crystal

El módulo `cristal` muestra la versión instalada de [Crystal](https://crystal-lang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `shard.yml`
- El directorio actual contiene un fichero `.cr`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | Símbolo usado antes de la versión de Crystal.                                           |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | El estilo del módulo.                                                                   |
| `detect_extensions` | `["cr"]`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["shard.yml"]`                      | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `disabled`          | `false`                              | Desactiva el módulo `crystal`.                                                          |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | La versión de `crystal`                |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers) SDK version when you are in the root directory of your Daml project. The `sdk-version` in the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION` environment variable. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `daml.yaml`

### Opciones

| Opción              | Predeterminado                     | Descripción                                                                             |
| ------------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `via [$symbol($version )]($style)` | El formato del módulo.                                                                  |
| `version_format`    | `v${raw}`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"Λ "`                             | A format string representing the symbol of Daml                                         |
| `style`             | `"bold cyan"`                      | El estilo del módulo.                                                                   |
| `detect_extensions` | `[]`                               | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["daml.yaml"]`                    | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                               | Qué carpetas deberían activar este módulo.                                              |
| `disabled`          | `false`                            | Deshabilita el módulo `daml`.                                                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.2.0` | La versión de `daml`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[daml]
format = "via [D $version](bold bright-green) "
```

## Dart

El módulo `dart` muestra la versión instalada de [Dart](https://dart.dev/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo con la extensión `.dart`
- El directorio actual contiene un directorio `.dart_tool`
- El directorio actual contiene un archivo `pubspec.yaml`, `pubspec.yml` o `pubspec.lock`

### Opciones

| Opción              | Predeterminado                                    | Descripción                                                                             |
| ------------------- | ------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                                       | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | Una cadena de formato que representa el símbolo de Dart                                 |
| `detect_extensions` | `["dart"]`                                        | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[".dart_tool"]`                                  | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold blue"`                                     | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                           | Desactiva el módulo `dart`.                                                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.8.4` | La versión de `dart`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

El módulo `deno` le muestra la versión instalada de [Deno](https://deno.land/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` o `deps.js`

### Opciones

| Opción              | Predeterminado                                                          | Descripción                                                                             |
| ------------------- | ----------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                    | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                                                             | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                                                  | Una cadena de formato que representa el símbolo de Deno                                 |
| `detect_extensions` | `[]`                                                                    | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["deno.json", "deno.jsonc", "mod.ts", "mod.js", "deps.ts", "deps.js"]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                    | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"green bold"`                                                          | El estilo del módulo.                                                                   |
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
format = "via [🦕 $version](green bold) "
```

## Directory

El módulo `directory` muestra la ruta hasta el directorio actual, mostrando tres directorios padre como máximo. Tu directorio se truncará a la raíz del repositorio git en el que estés.

Cuando usas el estilo fish de la opción pwd, en lugar de ocultar la ruta truncada, verás una versión acortada del nombre de cada directorio basada en el número que activa la opción.

Por ejemplo, dado `~/Dev/Nix/nixpkgs/pkgs` donde `nixpkgs` es la raíz del repositorio y el valor de la opción es `1`. En ese caso, verás `~/D/N/nixpkgs/pkgs`, cuando antes hubiera sido `nixpkgs/pkgs`.

### Opciones

| Opción              | Predeterminado                                                                                              | Descripción                                                                                                                 |
| ------------------- | ----------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                                                                         | El número de directorios padre a los que se debe truncar el directorio actual.                                              |
| `truncate_to_repo`  | `true`                                                                                                      | Trunca o no hasta la raíz del repositorio git en el que estés.                                                              |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "`                                                          | El formato del módulo.                                                                                                      |
| `style`             | `"bold cyan"`                                                                                               | El estilo del módulo.                                                                                                       |
| `disabled`          | `false`                                                                                                     | Desactiva el módulo `directory`.                                                                                            |
| `read_only`         | `"🔒"`                                                                                                       | El símbolo que indica si el directorio actual es de sólo lectura.                                                           |
| `read_only_style`   | `"red"`                                                                                                     | El estilo para el símbolo de sólo lectura.                                                                                  |
| `truncation_symbol` | `""`                                                                                                        | El símbolo a prefijar a las rutas truncadas. ej: "…/"                                                                       |
| `repo_root_style`   | `None`                                                                                                      | El estilo para la raíz del repositorio de git. El valor por defecto es equivalente al `style`.                              |
| `repo_root_format`  | `"[$before_root_path]($style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) "` | El formato de un repositorio de git cuando `repo_root_style` está definido.                                                 |
| `home_symbol`       | `"~"`                                                                                                       | El símbolo que indica el directorio personal.                                                                               |
| `use_os_path_sep`   | `true`                                                                                                      | Utiliza el separador de ruta del sistema operativo específico en lugar de usar siempre `/` (por ejemplo, `\` en Windows) |

<details>
<summary>Este módulo tiene algunas opciones avanzadas de configuración que controlan cómo se muestra el directorio.</summary>

| Opciones avanzadas          | Predeterminado | Descripción                                                                                                                                                                                                                  |
| --------------------------- | -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |                | Una tabla de sustituciones que se deben hacer a la ruta.                                                                                                                                                                     |
| `fish_style_pwd_dir_length` | `0`            | El número de caracteres a usar al aplicar la lógica de ruta pwd de la shell de fish.                                                                                                                                         |
| `use_logical_path`          | `true`         | Si `true` renderiza la ruta lógica originada desde el intérprete de comandos a través de `PWD` o `--logical-path`. Si `false` en su lugar renderiza la ruta física del sistema de archivos con enlaces simbólicos resueltos. |

`substitutions` permite definir reemplazos arbitrarios para cadenas literales que ocurren en la ruta, por ejemplo prefijos largos de red o directorios de desarrollo (p. ej. Java). Ten en cuenta que esto desactivará el estilo PWD de fish.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interactúa con las opciones de truncamiento estándar de una manera que puede sorprenderse primero: si no es cero, los componentes de la ruta que normalmente se truncarían se muestran con esa cantidad de caracteres. Por ejemplo, la ruta `/built/this/city/on/rock/and/roll`, que normalmente se mostraría como `rock/and/roll`, se mostraría como `/b/t/c/o/rock/and/roll` con `fish_style_pwd_dir_length = 1`--los componentes de ruta que normalmente se eliminarían se muestran con un solo carácter. Para `fish_style_pwd_dir_length = 2`, sería `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable  | Ejemplo               | Descripción                           |
| --------- | --------------------- | ------------------------------------- |
| path      | `"D:/Projects"`       | La ruta de directorio actual          |
| style\* | `"black bold dimmed"` | Refleja el valor de la opción `style` |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

<details>
<summary>Los repositorios de git tienen variables adicionales.</summary>

Consideremos la ruta `/path/to/home/git_repo/src/lib`

| Variable           | Ejemplo               | Descripción                                         |
| ------------------ | --------------------- | --------------------------------------------------- |
| before_root_path | `"/path/to/home/"`    | La ruta antes de la ruta del directorio raíz de git |
| repo_root          | `"git_repo"`          | El nombre del directorio raíz de git                |
| path               | `"/src/lib"`          | La ruta restante                                    |
| style              | `"black bold dimmed"` | Refleja el valor de la opción `style`               |
| repo_root_style  | `"underline white"`   | Estilo para el nombre del directorio raíz de git    |

</details>

### Ejemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker context

El módulo `docker_context` muestra el [contexto de Docker](https://docs.docker.com/engine/context/working-with-contexts/) actualmente activo si no está definido en `default` o si las variables de entorno `DOCKER_MACHINE_NAME`, `DOCKER_HOST` o `DOCKER_CONTEXT` están definidas (como se entiende para sobrescribir el contexto en uso).

### Opciones

| Opción              | Por defecto                                                   | Descripción                                                                                                               |
| ------------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | El formato del módulo.                                                                                                    |
| `symbol`            | `"🐳 "`                                                        | El símbolo usado antes de mostrar el contexto de Docker.                                                                  |
| `only_with_files`   | `true`                                                        | Mostrar solo cuando haya una coincidencia                                                                                 |
| `detect_extensions` | `[]`                                                          | Qué extensiones deberían activar este módulo (necesita que `solly_with_files` sea verdadero, con un valor "true").        |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Qué nombres de archivo deberían activar este módulo (necesita que `solly_with_files` sea verdadero, con un valor "true"). |
| `detect_folders`    | `[]`                                                          | Qué carpetas deberían activar este módulo (necesita que `solly_with_files` sea verdadero, con un valor "true").           |
| `style`             | `"blue bold"`                                                 | El estilo del módulo.                                                                                                     |
| `disabled`          | `false`                                                       | Desactiva el módulo `docker_context`.                                                                                     |

### Variables

| Variable  | Ejemplo        | Descripción                            |
| --------- | -------------- | -------------------------------------- |
| context   | `test_context` | El contexto actual de docker           |
| symbol    |                | Refleja el valor de la opción `symbol` |
| style\* |                | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

El módulo `dotnet` muestra la versión usada de .NET Core SDK para el directorio actual. Si el SDK ha sido anclado en el directorio actual, se mostrará la versión fijada. De lo contrario, el módulo muestra la última versión instalada del SDK.

Por defecto, este módulo solo se mostrará en tu prompt cuando uno o más de de los siguientes archivos estén presentes en el directorio actual:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

También necesitará tener instalado .NET Core SDK para poder usarlo correctamente.

Internamente, este módulo utiliza su propio mecanismo para la detección de versiones. Normalmente es el doble de rápido que ejecutar `dotnet --version`, pero puede mostrar una versión incorrecta si tu proyecto .NET tiene un diseño de directorio inusual. Si la precisión es más importante que la velocidad, puede desactivar el mecanismo estableciendo `heuristic = false` en las opciones del módulo.

El módulo también mostrará el Target Framework Moniker ([https://docs.microsoft. om/es/dotnet/standard/frameworks#supported-target-framework-versions](https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks)) cuando exista un archivo `.csproj` en el directorio actual.

### Opciones

| Opción              | Por defecto                                                                                             | Descripción                                                                             |
| ------------------- | ------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                                                                                             | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | Símbolo usado antes de mostrar la versión de .NET                                       |
| `heuristic`         | `true`                                                                                                  | Usa una detección de versiones más rápida para mantener a starship veloz.               |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                    | Qué carpetas deberían activar estos módulos.                                            |
| `style`             | `"bold blue"`                                                                                           | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                                 | Deshabilita el módulo `dotnet`.                                                         |

### Variables

| Variable  | Ejemplo          | Descripción                                                     |
| --------- | ---------------- | --------------------------------------------------------------- |
| version   | `v3.1.201`       | La version del sdk de `dotnet`                                  |
| tfm       | `netstandard2.0` | El Target Framework Moniker al que se dirige el proyecto actual |
| symbol    |                  | Refleja el valor de la opción `symbol`                          |
| style\* |                  | Refleja el valor de la opción `style`                           |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

El módulo `elixir` muestra la versión instalada de [Elixir](https://elixir-lang.org/) y [Erlang/OTP](https://erlang.org/doc/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `mix.exs`.

### Opciones

| Opción              | Por defecto                                                 | Descripción                                                                             |
| ------------------- | ----------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | El formato para el módulo elixir.                                                       |
| `version_format`    | `"v${raw}"`                                                 | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | El símbolo usado antes de mostrar la version de Elixir/Erlang.                          |
| `detect_extensions` | `[]`                                                        | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["mix.exs"]`                                               | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                        | Qué carpetas deberían activar estos módulos.                                            |
| `style`             | `"bold purple"`                                             | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                     | Desactiva el módulo `elixir`.                                                           |

### Variables

| Variable    | Ejemplo | Descripción                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | La version de `elixir`                 |
| otp_version |         | La version de otp de `elixir`          |
| symbol      |         | Refleja el valor de la opción `symbol` |
| style\*   |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
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
| `format`            | `"via [$symbol($version )]($style)"`               | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                                        | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | Una cadena de formato que representa el símbolo de Elm.                                 |
| `detect_extensions` | `["elm"]`                                          | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `["elm-stuff"]`                                    | Qué carpetas deberían activar estos módulos.                                            |
| `style`             | `"cyan bold"`                                      | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                            | Desactiva el módulo `elm`.                                                              |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.19.1` | La versión de `elm`                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Variable de entorno

El módulo `env_var` muestra el valor actual de una variable de entorno seleccionada. El módulo se mostrará sólo si se cumplen cualquiera de las siguientes condiciones:

- La opción de configuración de `variable` coincide con una variable de entorno existente
- La opción de configuración de `variable` no está definida, pero la opción de configuración `predeterminada` se encuentra

::: tip

Múltiples variables de entorno pueden mostrarse usando una `.`. (ver ejemplo) Si la opción de configuración de la `variable` no está definida, el módulo mostrará el valor de la variable bajo el nombre del texto después del caracter `.`.

Ejemplo: la siguiente configuración mostrará el valor de la variable de entorno USER

```toml
# ~/.config/starship.toml

[env_var.USER]
default = "unknown user"
```

:::

### Opciones

| Opción     | Predeterminado                 | Descripción                                                                            |
| ---------- | ------------------------------ | -------------------------------------------------------------------------------------- |
| `symbol`   | `""`                           | El símbolo usado antes de mostrar el valor de la variable.                             |
| `variable` |                                | La variable de entorno a mostrar.                                                      |
| `default`  |                                | El valor por defecto que se mostrará cuando la variable seleccionada no está definida. |
| `format`   | `"with [$env_value]($style) "` | El formato del módulo.                                                                 |
| `disabled` | `false`                        | Desactiva el módulo `env_var`.                                                         |

### Variables

| Variable  | Ejemplo                               | Descripción                                 |
| --------- | ------------------------------------- | ------------------------------------------- |
| env_value | `Windows NT` (si _variable_ es `$OS`) | El valor de entorno de la opción `variable` |
| symbol    |                                       | Refleja el valor de la opción `symbol`      |
| style\* | `black bold dimmed`                   | Refleja el valor de la opción `style`       |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

Mostrando múltiples variables de entorno:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = "SHELL"
default = "unknown shell"
[env_var.USER]
default = "unknown user"
```

## Erlang

El módulo `erlang` muestra la versión instalada de [Erlang/OTP](https://erlang.org/doc/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `rebar.config`.
- El directorio actual contiene un fichero `erlang.mk`.

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | El símbolo usado antes de mostrar la versión de Erlang.                                 |
| `style`             | `"bold red"`                         | El estilo del módulo.                                                                   |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                                            |
| `disabled`          | `false`                              | Desactiva el módulo `erlang`.                                                           |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | La versión de `erlang`                 |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Llenar

El módulo `fill` llena cualquier espacio extra en la línea con un símbolo. Si múltiples módulos `fill` están presentes en una línea, dividirán el espacio equitativamente entre ellos. Esto es útil para alinear otros módulos.

### Opciones

| Opción     | Predeterminado | Descripción                                |
| ---------- | -------------- | ------------------------------------------ |
| `symbol`   | `"."`          | El símbolo utilizado para llenar la línea. |
| `style`    | `"bold black"` | El estilo del módulo.                      |
| `disabled` | `false`        | Deshabilita el módulo `fill`               |

### Ejemplo

```toml
# ~/.config/starship.toml
format = "AA $fill BB $fill CC"

[fill]
symbol = "-"
style = "bold green"
```

Produce un prompt que se ve como:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Google Cloud (`gcloud`)

El módulo `gcloud` muestra la configuración actual para el CLI de [`gcloud`](https://cloud.google.com/sdk/gcloud). Esto se basa en el archivo `~/.config/gcloud/active_config`, el archivo `~/.config/gcloud/configurations/config_{CONFIG NAME}` y la variable de entorno `CLOUDSDK_CONFIG`.

### Opciones

| Opción            | Por defecto                                                | Descripción                                                  |
| ----------------- | ---------------------------------------------------------- | ------------------------------------------------------------ |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | El formato del módulo.                                       |
| `symbol`          | `"☁️  "`                                                   | El símbolo usado antes de mostrar el perfil actual de GCP.   |
| `region_aliases`  |                                                            | Tabla de alias de región a mostrar además del nombre GCP.    |
| `project_aliases` |                                                            | Tabla de alias del proyecto a mostrar además del nombre GCP. |
| `style`           | `"bold blue"`                                              | El estilo del módulo.                                        |
| `disabled`        | `false`                                                    | Desactiva el módulo `gcloud`.                                |

### Variables

| Variable  | Ejemplo       | Descripción                                                                   |
| --------- | ------------- | ----------------------------------------------------------------------------- |
| region    | `us-central1` | La región GCP actual                                                          |
| account   | `foo`         | El perfil actual de GCP                                                       |
| dominio   | `ejemplo.com` | El dominio actual del perfil GCP                                              |
| project   |               | El proyecto GCP actual                                                        |
| active    | `default`     | El nombre de configuración activo escrito en `~/.config/gcloud/active_config` |
| symbol    |               | Refleja el valor de la opción `symbol`                                        |
| style\* |               | Refleja el valor de la opción `style`                                         |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Mostrar los alias de cuenta y región

```toml
# ~/.config/starship.toml

[gcloud]
symbol = "️🇬️ "
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"
```

#### Mostrar cuenta y proyecto con alias

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = "vlpn"
```

## Git Branch

El módulo `git_branch` muestra la rama activa del repositorio en tu directorio actual.

### Opciones

| Opción               | Por defecto                                       | Descripción                                                                                             |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Muestra el nombre de la rama de seguimiento remoto, incluso si es igual al nombre de la rama local.     |
| `format`             | `"on [$symbol$branch(:$remote_branch)]($style) "` | El formato del módulo. Use `"$branch"` para referirse al nombre de la rama actual.                      |
| `symbol`             | `" "`                                            | Una cadena de formato que representa el símbolo de la rama git.                                         |
| `style`              | `"bold purple"`                                   | El estilo del módulo.                                                                                   |
| `truncation_length`  | `2^63 - 1`                                        | Trunca una rama git a grafemas `N`.                                                                     |
| `truncation_symbol`  | `"…"`                                             | El símbolo usado para indicar que un nombre de rama fue truncado. Puedes usar `""` para ningún símbolo. |
| `only_attached`      | `false`                                           | Mostrar solo el nombre de la rama cuando no esté en un estado `HEAD`.                                   |
| `ignore_branches`    | `[]`                                              | Una lista de nombres para evitar mostrar. Útil para "master" o "main".                                  |
| `disabled`           | `false`                                           | Desactiva el módulo `git_branch`.                                                                       |

### Variables

| Variable      | Ejemplo  | Descripción                                                                                                    |
| ------------- | -------- | -------------------------------------------------------------------------------------------------------------- |
| branch        | `master` | El nombre de la rama actual, vuelve a `HEAD` si no hay ninguna rama actual (por ejemplo, git detached `HEAD`). |
| remote_name   | `origin` | El nombre remoto.                                                                                              |
| remote_branch | `master` | El nombre de la rama rastreada en `remote_name`.                                                               |
| symbol        |          | Refleja el valor de la opción `symbol`                                                                         |
| style\*     |          | Refleja el valor de la opción `style`                                                                          |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
ignore_branches = ["master", "main"]
```

## Git commit

El módulo `git_commit` muestra el hash de la confirmación actual y también la etiqueta (si existe) del repositorio en su directorio actual.

### Opciones

| Opción               | Predeterminado                     | Descripción                                                                                            |
| -------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                                | La longitud del hash del commit de git mostrado.                                                       |
| `format`             | `"[\\($hash$tag\\)]($style) "` | El formato del módulo.                                                                                 |
| `style`              | `"bold green"`                     | El estilo del módulo.                                                                                  |
| `only_detached`      | `true`                             | Mostrar solo el hash de la confirmación de git cuando esté en estado "detached `HEAD`"                 |
| `tag_disabled`       | `true`                             | Deshabilita mostrar información de etiquetas en el módulo `git_commit`.                                |
| `tag_max_candidates` | `0`                                | How many commits to consider for tag display. El valor por defecto sólo permite coincidencias exactas. |
| `tag_symbol`         | `" 🏷 "`                            | Símbolo de etiqueta prefijando la información mostrada                                                 |
| `disabled`           | `false`                            | Deshabilita el módulo `git_commit`.                                                                    |

### Variables

| Variable  | Ejemplo   | Descripción                           |
| --------- | --------- | ------------------------------------- |
| hash      | `b703eb3` | El hash actual del commit de git      |
| style\* |           | Refleja el valor de la opción `style` |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git state

El módulo `git_state` se mostrará en directorios que son parte de un repositorio git, y donde hay una operación en curso, tales como: _REBASING_, _BISECTING_, etc. Si hay información de progreso (por ejemplo, REBASING 3/10), esa información será mostrada también.

### Opciones

| Opción         | Predeterminado                                                  | Descripción                                                                                         |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | Una cadena de formato que se muestra cuando un `rebase` está en progreso.                           |
| `merge`        | `"MERGING"`                                                     | Una cadena de formato que se muestra cuando un `merge` está en progreso.                            |
| `revert`       | `"REVERTING"`                                                   | Una cadena de formato mostrada cuando un `revert` está en progreso.                                 |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | Una cadena de formato que se muestra cuando un `cherry-pick` está en progreso.                      |
| `bisect`       | `"BISECTING"`                                                   | Una cadena de formato que se muestra cuando un `bisect` está en progreso.                           |
| `am`           | `"AM"`                                                          | Una cadena de formato que se muestra cuando un `apply-mailbox` (`git am`) está en progeso.          |
| `am_or_rebase` | `"AM/REBASE"`                                                   | Una cadena de formato que se muestra cuando un ambiguo `apply-builbox` o `rebase` está en progreso. |
| `style`        | `"bold yellow"`                                                 | El estilo del módulo.                                                                               |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | El formato del módulo.                                                                              |
| `disabled`     | `false`                                                         | Desactiva el módulo `git_state`.                                                                    |

### Variables

| Variable         | Ejemplo    | Descripción                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | El estado actual del repositorio      |
| progress_current | `1`        | El progreso de la operación actual    |
| progress_total   | `2`        | El progreso total de la operación     |
| style\*        |            | Refleja el valor de la opción `style` |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Métricas de Git

El módulo `git_metrics` mostrará el número de líneas añadidas y eliminadas en el repositorio git actual.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción               | Predeterminado                                               | Descripción                                        |
| -------------------- | ------------------------------------------------------------ | -------------------------------------------------- |
| `added_style`        | `"bold green"`                                               | El estilo para el recuento añadido.                |
| `deleted_style`      | `"bold red"`                                                 | El estilo para el recuento eliminado.              |
| `only_nonzero_diffs` | `true`                                                       | Mostrar sólo el estado de los elementos cambiados. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | El formato del módulo.                             |
| `disabled`           | `true`                                                       | Deshabilita el módulo `git_metrics`.               |

### Variables

| Variable          | Ejemplo | Descripción                                   |
| ----------------- | ------- | --------------------------------------------- |
| added             | `1`     | El número actual de líneas añadidas           |
| deleted           | `2`     | El número actual de líneas eliminadas         |
| added_style\*   |         | Refleja el valor de la opción `added_style`   |
| deleted_style\* |         | Refleja el valor de la opción `deleted_style` |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = "bold blue"
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
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | El formato por defecto para `git_status`                                                                                                  |
| `conflicted`        | `"="`                                           | Esta rama tiene conflictos de fusión.                                                                                                     |
| `ahead`             | `"⇡"`                                           | El formato de `ahead`                                                                                                                     |
| `behind`            | `"⇣"`                                           | El formato de `behind`                                                                                                                    |
| `diverged`          | `"⇕"`                                           | El formato de `diverged`                                                                                                                  |
| `up_to_date`        | `""`                                            | El formato de `up_to_date`                                                                                                                |
| `untracked`         | `"?"`                                           | El formato de `untracked`                                                                                                                 |
| `stashed`           | `"$"`                                           | El formato de `stashed`                                                                                                                   |
| `modified`          | `"!"`                                           | El formato de `modified`                                                                                                                  |
| `staged`            | `"+"`                                           | El formato de `staged`                                                                                                                    |
| `renamed`           | `"»"`                                           | El formato de `renamed`                                                                                                                   |
| `deleted`           | `"✘"`                                           | El formato de `deleted`                                                                                                                   |
| `style`             | `"bold red"`                                    | El estilo del módulo.                                                                                                                     |
| `ignore_submodules` | `false`                                         | Ignorar cambios a los submódulos.                                                                                                         |
| `disabled`          | `false`                                         | Desactiva el módulo `git_status`.                                                                                                         |
| `windows_starship`  |                                                 | Utiliza esta ruta (Linux) a un ejecutable de Starship de Windows para renderizar `git_status` cuando está en las rutas de Windows en WSL. |

### Variables

Las siguientes variables se pueden utilizar en `format`:

| Variable       | Descripción                                                                                                              |
| -------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `all_status`   | Atajo para `$conflicted$stashed$deleted$renamed$modified$staged$untracked`                                               |
| `ahead_behind` | Muestra la cadena de formato de `diverged` `ahead` o `behind` o `up_to_date` basado en el estado actual del repositorio. |
| `conflicted`   | Muestra `conflicted` cuando esta rama tiene conflictos de fusión.                                                        |
| `untracked`    | Muestra `untracked` cuando hay archivos sin rastrear en el directorio de trabajo.                                        |
| `stashed`      | Muestra `stashed` cuando existe un "stash" para el repositorio local.                                                    |
| `modified`     | Muestra `modified` cuando hay modificaciones de archivo en el directorio de trabajo.                                     |
| `staged`       | Muestra `staged` cuando se ha añadido un nuevo archivo al área de "stash".                                               |
| `renamed`      | Muestra `renamed` cuando un archivo renombrado ha sido añadido al área de "stash".                                       |
| `deleted`      | Muestra `deleted` cuando un archivo ha sido añadido al área de "stash".                                                  |
| style\*      | Refleja el valor de la opción `style`                                                                                    |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

Las siguientes variables pueden ser usadas en `diverged`:

| Variable       | Descripción                                             |
| -------------- | ------------------------------------------------------- |
| `ahead_count`  | Número de commits por delante de la rama de seguimiento |
| `behind_count` | Número de commits detrás de la rama de seguimiento      |

Las siguientes variales pueden ser usadas en `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` y `deleted`:

| Variable | Descripción                   |
| -------- | ----------------------------- |
| `count`  | Muestra el número de archivos |

### Ejemplo

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
up_to_date = "✓"
untracked = "🤷"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

Mostrar el recuento delante/detrás de la rama que está siendo rastreada

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
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

| Opción              | Por defecto                                                                               | Descripción                                                                             |
| ------------------- | ----------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                      | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                                                                               | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                                    | Una cadena de formato que representa el símbolo de Go.                                  |
| `detect_extensions` | `["go"]`                                                                                  | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["go.mod", "go.sum", "go.work", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `["Godeps"]`                                                                              | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold cyan"`                                                                             | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                   | Desactiva el módulo de `golang`.                                                        |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | La versión de `go`                     |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Haskell

El módulo `haskell` encuentra la versión GHC seleccionada y/o la instantánea de la pila seleccionada.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `stack.yaml`
- El directorio actual contiene cualquier archivo `.hs`, `.cabal` o `.hs-boot`

### Opciones

| Opción              | Por defecto                          | Descripción                                                |
| ------------------- | ------------------------------------ | ---------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                     |
| `symbol`            | `"λ "`                               | Una cadena de formato que representa el símbolo de Haskell |
| `detect_extensions` | `["hs", "cabal", "hs-boot"]`         | Qué extensiones deberían activar este módulo.              |
| `detect_files`      | `["stack.yaml", "cabal.project"]`    | Qué nombres de archivo deberían activar este módulo.       |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                 |
| `style`             | `"bold purple"`                      | El estilo del módulo.                                      |
| `disabled`          | `false`                              | Deshabilita el módulo `haskell`.                           |

### Variables

| Variable       | Ejemplo     | Descripción                                                                          |
| -------------- | ----------- | ------------------------------------------------------------------------------------ |
| version        |             | `ghc_version` o `snapshot` dependiendo de si el proyecto actual es un proyecto Stack |
| snapshot       | `lts-18.12` | Instantánea de Stack seleccionada actualmente                                        |
| ghc\_version | `9.2.1`     | Versión GHC instalada actualmente                                                    |
| symbol         |             | Refleja el valor de la opción `symbol`                                               |
| style\*      |             | Refleja el valor de la opción `style`                                                |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

## Helm

El módulo `helm` muestra la versión instalada de [Helm](https://helm.sh/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `helmfile.yaml`
- El directorio actual contiene un archivo `Chart.yaml`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `"⎈ "`                               | Una cadena de formato que representa el símbolo de Helm.                                |
| `style`             | `"bold white"`                       | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Desactiva el módulo `helm`.                                                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | La versión de `helm`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

El módulo `hostname` muestra el nombre de host del sistema.

### Opciones

| Opción       | Por defecto                                  | Descripción                                                                                                                                                       |
| ------------ | -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`   | `true`                                       | Mostrar sólo el nombre de host cuando esté conectado a una sesión SSH.                                                                                            |
| `ssh_symbol` | `"🌐 "`                                       | Una cadena de formato que representa el símbolo cuando se conecta a la sesión SSH.                                                                                |
| `trim_at`    | `"."`                                        | Cadena en la que el nombre de host se corta, después de la primera partida. `"."` se detendrá después del primer punto. `""` deshabilitará cualquier truncamiento |
| `format`     | `"[$ssh_symbol$nombre de host]($style) en "` | El formato del módulo.                                                                                                                                            |
| `style`      | `"bold dimmed green"`                        | El estilo del módulo.                                                                                                                                             |
| `disabled`   | `false`                                      | Desactiva el módulo `hostname`.                                                                                                                                   |

### Variables

| Variable        | Ejemplo    | Descripción                                                    |
| --------------- | ---------- | -------------------------------------------------------------- |
| nombre del host | `computer` | El nombre de host del ordenador                                |
| style\*       |            | Refleja el valor de la opción `style`                          |
| ssh_symbol      | `"🌏 "`     | El símbolo a representar cuando está conectado a la sesión SSH |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = "[$ssh_symbol](bold blue) on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

El módulo `java` muestra la versión instalada de [Java](https://www.oracle.com/java/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, o `build.boot`
- El directorio actual contiene un archivo con la extensión `.java`, `.class`, `.gradle` o `.jar`, `.clj` o `.cljc`

### Opciones

| Opción              | Por defecto                                                                                               | Descripción                                                                             |
| ------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                                                                                               | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                      | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `"☕ "`                                                                                                    | Una cadena de formato que representa el símbolo de Java                                 |
| `style`             | `"red dimmed"`                                                                                            | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                                   | Desactiva el módulo `java`.                                                             |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| version   | `v14`   | La versión de `java`                   |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

El módulo `jobs` muestra el número actual de tareas en ejecución. El módulo se mostrará sólo si hay tareas en segundo plano ejecutándose. El módulo mostrará el número de tareas ejecutados si hay al menos 2 tareas, o más del valor de configuración de `number_threshold`, si existe. El módulo mostrará un símbolo si hay al menos 1 tarea, o más del valor de configuración de `symbol_threshold`, si existe. Puedes establecer ambos valores a 0 para _siempre_ mostrar el símbolo y el número de tareas, incluso si hay 0 tareas en ejecución.

La funcionalidad por defecto es:

- 0 tareas -> Nada se muestra.
- 1 tarea -> `symbol` se muestra.
- 2 tareas o más -> `symbol` + `number` son mostrados.

::: warning

Este módulo no está soportado por tcsh y nu.

:::

::: warning

La opción `threshold` está obsoleta, pero si deseas usarla, el módulo mostrará el número de tareas en ejecución si hay más de 1 tarea, o más que el valor de configuración `threshold`, si existe. Si `threshold` se establece en 0, entonces el módulo también se mostrará cuando haya 0 tareas en ejecución.

:::

### Opciones

| Opción             | Predeterminado                | Descripción                                                                        |
| ------------------ | ----------------------------- | ---------------------------------------------------------------------------------- |
| `threshold`*       | `1`                           | Muestra el número de tareas si se exceden.                                         |
| `symbol_threshold` | `1`                           | Muestra `symbol` si el conteo de tareas es al menos `symbol_threshold`.            |
| `number_threshold` | `2`                           | Muestra el número de tareas si el conteo de tareas es al menos `symbol_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | El formato del módulo.                                                             |
| `symbol`           | `"✦"`                         | La cadena utilizada para representar la variable `symbol`.                         |
| `style`            | `"bold blue"`                 | El estilo del módulo.                                                              |
| `disabled`         | `false`                       | Desactiva el módulo `jobs`.                                                        |

*: Esta opción está desaprobada, por favor utiliza las opciones `number_threshold` y `symbol_threshold` en su lugar.

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | El número de tareas                    |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
number_threshold = 4
symbol_threshold = 0
```

## Julia

El módulo `julia` muestra la versión instalada de [Julia](https://julialang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `Project.toml`
- El directorio actual contiene un archivo `Manifest.toml`
- El directorio actual contiene un archivo con la extensión `.jl`

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `"ஃ "`                               | Una cadena de formato que representa el símbolo de Julia.                               |
| `style`             | `"bold purple"`                      | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Desactiva el módulo `julia`.                                                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.4.0` | La versión de `julia`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

El módulo `kotlin` muestra la versión instalada de [Kotlin](https://kotlinlang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `.kt` o `.kts`

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["kt", "kts"]`                      | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `"🅺 "`                               | Una cadena de formato que representa el símbolo de Kotlin.                              |
| `style`             | `"bold blue"`                        | El estilo del módulo.                                                                   |
| `kotlin_binary`     | `"kotlin"`                           | Configura el binario kotlin que Starship ejecuta al obtener la versión.                 |
| `disabled`          | `false`                              | Deshabilita el módulo `kotlin`.                                                         |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.4.21` | La versión de `kotlin`                 |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Utiliza el binario del Compilador de Kotlin para obtener la versión instalada
kotlin_binary = "kotlinc"
```

## Kubernetes

Muestra el nombre actual del [contexto de Kubernetes](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) y, si se establece, el espacio de nombres, el usuario y el clúster del archivo kubeconfig. El espacio de nombres necesita establecerse en el archivo kubeconfig, esto puede hacerse mediante `kubectl config set-context starship-context --namespace astronaut`. Del mismo modo, el usuario y clúster pueden establecerse con `kubectl config set-context starship-context --user starship-user` y `kubectl config set-context starship-context --cluster starship-cluster`. Si se establece la variable de entorno `$KUBECONFIG`, el módulo usará eso si no usará el `~/.kube/config`.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

When the module is enabled it will always be active, unless any of `detect_extensions`, `detect_files` or `detect_folders` have been st in which case the module will only be active in directories that match those conditions.

:::

### Opciones

| Opción              | Predeterminado                                       | Descripción                                                                 |
| ------------------- | ---------------------------------------------------- | --------------------------------------------------------------------------- |
| `symbol`            | `"☸ "`                                               | Una cadena de formato que representa el símbolo mostrado antes del Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | El formato del módulo.                                                      |
| `style`             | `"cyan bold"`                                        | El estilo del módulo.                                                       |
| `context_aliases`   |                                                      | Tabla de alias de contexto a mostrar.                                       |
| `user_aliases`      |                                                      | Table of user aliases to display.                                           |
| `detect_extensions` | `[]`                                                 | Qué extensiones deberían activar este módulo.                               |
| `detect_files`      | `[]`                                                 | Qué nombres de archivo deberían activar este módulo.                        |
| `detect_folders`    | `[]`                                                 | Qué carpetas deberían activar estos módulos.                                |
| `disabled`          | `true`                                               | Desactiva el módulo `kubernetes`.                                           |

### Variables

| Variable  | Ejemplo              | Descripción                                                 |
| --------- | -------------------- | ----------------------------------------------------------- |
| context   | `starship-context`   | El nombre del contexto actual de kubernetes                 |
| namespace | `starship-namespace` | Si se establece, el espacio de nombres actual de kubernetes |
| user      | `starship-user`      | Si se establece, el espacio de nombres actual de kubernetes |
| cluster   | `starship-cluster`   | Si se establece, el clúster actual de kubernetes            |
| symbol    |                      | Refleja el valor de la opción `symbol`                      |
| style\* |                      | Refleja el valor de la opción `style`                       |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
".*/openshift-cluster/.*" = "openshift"
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
[kubernetes.user_aliases]
"dev.local.cluster.k8s" = "dev"
"root/.*" = "root"
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

La expresión regular debe coincidir en todo el contexto de kube. los grupos de captura pueden ser referenciados usando `$name` y `$N` en el reemplazo. Esto está más explicado en la documentación del [crate regex](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace).

Los nombres de cluster generados de forma larga y automática pueden ser identificados y abreviados usando expresiones regulares:

```toml
[kubernetes.context_aliases]
# los contextos de OpenShift llevan el espacio de nombres y el usuario en el contexto de kube: `namespace/name/user`:
".*/openshift-cluster/.*" = "openshift"
# Or better, to rename every OpenShift cluster at once:
".*/(?P<var_cluster>[\\w-]+)/.*" = "$var_cluster"

# Los contextos de GKE, AWS y otros proveedores de nube normalmente llevan información adicional, como la región/zona.
# La siguiente entrada coincide con el formato GKE (`gke_projectname_zone_cluster-name`)
# y renombra cada contexto de kube coincidente a un formato más legible (`gke-cluster-name`):
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
```

## Salto de línea

El módulo `line_break` separa el indicador en dos líneas.

### Opciones

| Opción     | Predeterminado | Descripción                                                                     |
| ---------- | -------------- | ------------------------------------------------------------------------------- |
| `disabled` | `false`        | Deshabilita el módulo `line_break`, haciendo que el mensaje sea una sola línea. |

### Ejemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

El módulo `localip` muestra la dirección IPv4 de la interfaz de red principal.

### Opciones

| Opción     | Por defecto               | Descripción                                                             |
| ---------- | ------------------------- | ----------------------------------------------------------------------- |
| `ssh_only` | `true`                    | Solo muestra la direccion IP cuando se está conectado a una sesión SSH. |
| `format`   | `"[$localipv4]($style) "` | El formato del módulo.                                                  |
| `style`    | `"bold yellow"`           | El estilo del módulo.                                                   |
| `disabled` | `true`                    | Deshabilita el módulo `localip`.                                        |

### Variables

| Variable  | Ejemplo      | Descripción                           |
| --------- | ------------ | ------------------------------------- |
| localipv4 | 192.168.1.13 | Contiene la dirección IPv4 primaria   |
| style\* |              | Refleja el valor de la opción `style` |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = "@[$localipv4](bold red) "
disabled = false
```

## Lua

El módulo `lua` muestra la versión instalada de [Lua](http://www.lua.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `.lua-version`
- El directorio actual contiene un directorio `lua`
- El directorio actual contiene un archivo con la extensión `.lua`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌙 "`                               | Una cadena de formato que representa el símbolo de Lua.                                 |
| `detect_extensions` | `["lua"]`                            | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[".lua-version"]`                   | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `["lua"]`                            | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold blue"`                        | El estilo del módulo.                                                                   |
| `lua_binary`        | `"lua"`                              | Configura el binario de lua que Starship ejecuta al obtener la versión.                 |
| `disabled`          | `false`                              | Desactiva el módulo `lua`.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | La versión de `lua`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memoria utilizada

El módulo `memory_usage` muestra la memoria del sistema actual y el uso de memoria de intercambio.

Por defecto, el uso de swap se muestra si el intercambio total del sistema no es cero.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción      | Por defecto                                     | Descripción                                                   |
| ----------- | ----------------------------------------------- | ------------------------------------------------------------- |
| `threshold` | `75`                                            | Ocultar el uso de memoria a menos que supere este porcentaje. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | El formato del módulo.                                        |
| `symbol`    | `"🐏"`                                           | El símbolo usado antes de mostrar el uso de memoria.          |
| `style`     | `"bold dimmed white"`                           | El estilo del módulo.                                         |
| `disabled`  | `true`                                          | Desactiva el módulo `memory_usage`.                           |

### Variables

| Variable         | Ejemplo       | Descripción                                                                        |
| ---------------- | ------------- | ---------------------------------------------------------------------------------- |
| ram              | `31GiB/65GiB` | La memoria RAM usada/total del sistema actual.                                     |
| ram_pct          | `48%`         | El porcentaje de la memoria actual del sistema.                                    |
| swap\*\*     | `1GiB/4GiB`   | El tamaño de la memoria de intercambio del archivo de memoria del sistema actual.  |
| swap_pct\*\* | `77%`         | El porcentaje de memoria de intercambio del archivo de memoria del sistema actual. |
| symbol           | `🐏`           | Refleja el valor de la opción `symbol`                                             |
| style\*        |               | Refleja el valor de la opción `style`                                              |

*: Está variable solo puede utilizarse como parte de una cadena de estilo *\*: La información del archivo SWAP solo se muestra si se detecta en el sistema actual

### Ejemplo

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

El módulo `hg_branch` muestra la rama activa del repositorio en su directorio actual.

### Opciones

| Opción              | Por defecto                      | Descripción                                                                                         |
| ------------------- | -------------------------------- | --------------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | El símbolo usado antes del marcador hg o nombre de la rama del repositorio en su directorio actual. |
| `style`             | `"bold purple"`                  | El estilo del módulo.                                                                               |
| `format`            | `"on [$symbol$branch]($style) "` | El formato del módulo.                                                                              |
| `truncation_length` | `2^63 - 1`                       | Trunca el nombre de la rama hg a `N` grafemas                                                       |
| `truncation_symbol` | `"…"`                            | El símbolo usado para indicar que un nombre de rama fue truncado.                                   |
| `disabled`          | `true`                           | Desactiva el módulo `hg_branch`.                                                                    |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| branch    | `master` | La rama de mercurial activa            |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

El módulo `nim` muestra la versión instalada de [Nim](https://nim-lang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `nim.cfg`
- El directorio actual contiene un archivo con la extensión `.nim`
- El directorio actual contiene un archivo con la extensión `.nims`
- El directorio actual contiene un archivo con la extensión `.nimble`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo                                                                   |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | El símbolo usado antes de mostrar la versión de Nim.                                    |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["nim.cfg"]`                        | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold yellow"`                      | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Desactiva el módulo `nim`.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.2.0` | La versión de `nimc`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

El módulo `nix_shell` muestra el entorno [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html). El módulo se mostrará dentro de un entorno nix-shell.

### Opciones

| Opción       | Por defecto                                    | Descripción                                                                      |
| ------------ | ---------------------------------------------- | -------------------------------------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | El formato del módulo.                                                           |
| `symbol`     | `"❄️ "`                                        | Una cadena de formato que representa el símbolo de nix-shell.                    |
| `style`      | `"bold blue"`                                  | El estilo del módulo.                                                            |
| `impure_msg` | `"impure"`                                     | Una cadena de formato que se muestra cuando el intérprete de comandos es impuro. |
| `pure_msg`   | `"pure"`                                       | Una cadena de formato que se muestra cuando el intérprete de comandos es puro.   |
| `disabled`   | `false`                                        | Desactiva el módulo `nix_shell`.                                                 |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| state     | `pure`  | El estado de nix-shell                 |
| name      | `lorri` | El nombre de nix-shell                 |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

El módulo `nodejs` muestra la versión instalada de [Node.js](https://nodejs.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `package.json`
- El directorio actual contiene un archivo `.node-version`
- El directorio actual contiene un archivo `.nvmrc`
- El directorio actual contiene un directorio `node_modules`
- El directorio actual contiene un archivo con la extensión `.js`, `.mjs` o `.cjs`
- El directorio actual contiene un archivo con la extensión `.ts`, `.mts` o `.cts`

### Opciones

| Opción              | Por defecto                                | Descripción                                                                                                     |
| ------------------- | ------------------------------------------ | --------------------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`       | El formato del módulo.                                                                                          |
| `version_format`    | `"v${raw}"`                                | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch`                         |
| `symbol`            | `" "`                                     | Una cadena de formato que representa el símbolo de Node.js.                                                     |
| `detect_extensions` | `["js", "mjs", "cjs", "ts", "mts", "cts"]` | Qué extensiones deberían activar este módulo.                                                                   |
| `detect_files`      | `["package.json", ".node-version"]`        | Qué nombres de archivo deberían activar este módulo.                                                            |
| `detect_folders`    | `["node_modules"]`                         | Qué carpetas deberían activar este módulo.                                                                      |
| `style`             | `"bold green"`                             | El estilo del módulo.                                                                                           |
| `disabled`          | `false`                                    | Desactiva el módulo `nodejs`.                                                                                   |
| `not_capable_style` | `bold red`                                 | El estilo para el módulo cuando una propiedad de motores en package.json no coincide con la versión de Node.js. |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | La versión de `node`                   |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

El módulo `ocaml` muestra la versión instalada de [OCaml](https://ocaml.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo con extensión `.opam` o directorio `_opam`
- El directorio actual contiene un directorio `esy.lock`
- El directorio actual contiene un archivo `dune` o `dune-project`
- El directorio actual contiene un archivo `jbuild` o `jbuild-ignore`
- El directorio actual contiene un archivo `.merlin`
- El directorio actual contiene un archivo con la extensión `.ml`, `.mli`, `.re` o `.rei`

### Opciones

| Opción                    | Por defecto                                                                | Descripción                                                                             |
| ------------------------- | -------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | La cadena de formato para el módulo.                                                    |
| `version_format`          | `"v${raw}"`                                                                | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | El símbolo usado antes de mostrar la versión de OCaml.                                  |
| `global_switch_indicator` | `""`                                                                       | La cadena de formato usada para representar el interruptor global de OPAM.              |
| `local_switch_indicator`  | `"*"`                                                                      | La cadena de formato usada para representar el interruptor local de OPAM.               |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Qué carpetas deberían activar este módulo.                                              |
| `style`                   | `"bold yellow"`                                                            | El estilo del módulo.                                                                   |
| `disabled`                | `false`                                                                    | Desactiva el módulo `ocaml`.                                                            |

### Variables

| Variable         | Ejemplo      | Descripción                                                                 |
| ---------------- | ------------ | --------------------------------------------------------------------------- |
| version          | `v4.10.0`    | La versión de `ocaml`                                                       |
| switch_name      | `my-project` | El interruptor OPAM activo                                                  |
| switch_indicator |              | Refleja el valor de `indicator` para el interruptor OPAM activo actualmente |
| symbol           |              | Refleja el valor de la opción `symbol`                                      |
| style\*        |              | Refleja el valor de la opción `style`                                       |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

El módulo `openstack` muestra la nube OpenStack actual y el proyecto. El módulo solo está activo cuando la variable env `OS_CLOUD` está definida en cuyo caso leerá el archivo `nubes. aml` desde cualquiera de las [ubicaciones por defecto](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files) para obtener el proyecto actual en uso.

### Opciones

| Opción     | Por defecto                                         | Descripción                                                 |
| ---------- | --------------------------------------------------- | ----------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | El formato del módulo.                                      |
| `symbol`   | `"☁️ "`                                             | El símbolo usado antes de mostrar la nube OpenStack actual. |
| `style`    | `"bold yellow"`                                     | El estilo del módulo.                                       |
| `disabled` | `false`                                             | Deshabilita el módulo `openstack`.                          |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | La nube OpenStack actual               |
| project   | `dev`   | El proyecto OpenStack actual           |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Versión del paquete

El módulo `package` se muestra cuando el directorio actual es el repositorio de un paquete, y muestra su versión actual. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – La versión del paquete `npm` se extrae del `package.json` presente en el directorio actual
- [**Cargo**](https://doc.rust-lang.org/cargo/) – La versión del paquete `cargo` se extrae del `Cargo.toml` presente en el directorio actual
- [**Nimble**](https://github.com/nim-lang/nimble) - La versión del paquete `nimble` se extrae del archivo `*.nimble` presente en el directorio actual con el comando `nimble dump`
- [**Poetry**](https://python-poetry.org/) – La versión del paquete `poetry` se extrae del `pyproject.toml` presente en el directorio actual
- [**Python**](https://www.python.org) – La versión del paquete `python` se extrae del [pyproject.toml](https://peps.python.org/pep-0621/) presente en el directorio actual
- [**Composer**](https://getcomposer.org/) – La versión del paquete `composer` se extrae del `composer.json` presente en el directorio actual
- [**Gradle**](https://gradle.org/) – La versión `gradle` del paquete se extrae de `build.gradle` presente en el directorio actual
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - La versión del paquete se extrae de `Project.toml` presente en el directorio actual
- [**Mix**](https://hexdocs.pm/mix/) - La versión del paquete `mix` es extraída del `mix.exs` presente en el directorio actual
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - La versión del paquete `helm` se extrae de `Chart.yaml` presente en el directorio actual
- [**Maven**](https://maven.apache.org/) - La versión de paquete `maven` se extrae de `pom.xml` presente en el directorio actual
- [**Meson**](https://mesonbuild.com/) - La versión del paquete `meson` se extrae de `meson.build` presente en el directorio actual
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - La versión del paquete `shards` se extrae de `shard.yml` presente en el directorio actual
- [**V**](https://vlang.io) - La versión del paquete `vlang` se extrae de `v.mod` presente en el directorio actual
- [**SBT**](https://scala-sbt.org) - La versión del paquete `sbt` se extrae del archivo `build.sbt` presente en el directorio actual
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - La versión del paquete `dart` se extrae del archivo `pubspec.yaml` presente en el directorio actual

> ⚠️ La versión que se muestra es la del paquete cuyo código fuente está en tu directorio actual, no en tu gestor de paquetes.

### Opciones

| Opción            | Por defecto                       | Descripción                                                                             |
| ----------------- | --------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | El formato del módulo.                                                                  |
| `symbol`          | `"📦 "`                            | El símbolo usado antes de mostrar la versión del paquete.                               |
| `version_format`  | `"v${raw}"`                       | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `style`           | `"bold 208"`                      | El estilo del módulo.                                                                   |
| `display_private` | `false`                           | Activar la visualización de la versión para los paquetes marcados como privados.        |
| `disabled`        | `false`                           | Desactiva el módulo `package`.                                                          |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.0.0` | La versión de su paquete               |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

El módulo `perl` muestra la versión instalada de [Perl](https://www.perl.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `Makefile.PL` o `Build.PL`
- El directorio actual contiene un archivo `cpanfile` o `cpanfile.snapshot`
- El directorio actual contiene un archivo `META.json` o `META.yml`
- El directorio actual contiene un archivo `.perl-version`
- El directorio actual contiene un `.pl`, `.pm` o `.pod`

### Opciones

| Opción              | Por defecto                                                                                              | Descripción                                                                             |
| ------------------- | -------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | La cadena de formato para el módulo.                                                    |
| `version_format`    | `"v${raw}"`                                                                                              | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | El símbolo usado antes de mostrar la versión de Perl                                    |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                     | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold 149"`                                                                                             | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                                  | Desactiva el módulo `perl`.                                                             |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v5.26.1` | La versión de `perl`                   |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

El módulo `php` muestra la versión instalada de [PHP](https://www.php.net/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `composer.json`
- El directorio actual contiene un archivo `.php-version`
- El directorio actual contiene una extensión `.php`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | El símbolo usado antes de mostrar la versión de PHP.                                    |
| `detect_extensions` | `["php"]`                            | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["composer.json", ".php-version"]`  | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"147 bold"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Desactiva el módulo `php`.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v7.3.8` | La versión de `php`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## Pulumi

El módulo `pulumi` muestra el nombre de usuario actual, [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/)seleccionado y la versión.

::: tip

Por defecto la versión de Pulumi no se muestra, ya que toma un orden de magnitud más largo para cargar que la mayoría de los plugins (~70ms). Si aún deseas activarlo, [sigue el ejemplo que se muestra a continuación](#with-pulumi-version).

:::

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene `Pulumi.yaml` o `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Opciones

| Opción           | Por defecto                                  | Descripción                                                                             |
| ---------------- | -------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`         | `"via [$symbol($username@)$stack]($style) "` | La cadena de formato para el módulo.                                                    |
| `version_format` | `"v${raw}"`                                  | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `" "`                                       | Una cadena de formato que se muestra antes de la pila de Pulumi.                        |
| `style`          | `"bold 5"`                                   | El estilo del módulo.                                                                   |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.                          |
| `disabled`       | `false`                                      | Deshabilita el módulo `pulumi`.                                                         |

### Variables

| Variable          | Ejemplo    | Descripción                            |
| ----------------- | ---------- | -------------------------------------- |
| version           | `v0.12.24` | La versión de `pulumi`                 |
| stack             | `dev`      | La pila actual de Pulumi               |
| nombre de usuario | `alice`    | El usuario actual de Pulumi            |
| symbol            |            | Refleja el valor de la opción `symbol` |
| style\*         |            | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

#### Con la versión de Pulumi

```toml
# ~/.config/starship.toml

[pulumi]
format = "[🛥 ($version )$stack]($style) "
```

#### Sin versión de Pulumi

```toml
# ~/.config/starship.toml
[pulumi]
symbol = "🛥 "
format = "[$symbol$stack]($style) "
```

## PureScript

El módulo `purescript` muestra la versión instalada de [PureScript](https://www.purescript.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `spago.dhall`
- El directorio actual contiene un archivo con la extensión `.purs`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | El símbolo usado antes de mostrar la versión de PureScript.                             |
| `detect_extensions` | `["purs"]`                           | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["spago.dhall"]`                    | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold white"`                       | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Deshabilita el módulo `purescript`.                                                     |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | La versión de `purescript`             |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

El módulo `python` muestra la versión instalada de [Python](https://www.python.org/) y el entorno virtual actual [Python](https://docs.python.org/tutorial/venv.html) si uno está activado.

Si `pyenv_version_name` se establece en `true`, mostrará el nombre de la versión de pyenv. De lo contrario, se mostrará el número de versión de `python --version`.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `.python-version`
- El directorio actual contiene un archivo `Pipfile`
- El directorio actual contiene un archivo `__init__.py`
- El directorio actual contiene un archivo `pyproject.toml`
- El directorio actual contiene un archivo `requirements.txt`
- El directorio actual contiene un archivo `setup.py`
- El directorio actual contiene un archivo `tox.ini`
- El directorio actual contiene un archivo con la extensión `.py`.
- Un entorno virtual está activado actualmente

### Opciones

| Opción               | Por defecto                                                                                                  | Descripción                                                                             |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | El formato del módulo.                                                                  |
| `version_format`     | `"v${raw}"`                                                                                                  | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`             | `"🐍 "`                                                                                                       | Una cadena de formato que representa el símbolo de Python                               |
| `style`              | `"yellow bold"`                                                                                              | El estilo del módulo.                                                                   |
| `pyenv_version_name` | `false`                                                                                                      | Usar pyenv para obtener la versión de Python                                            |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefijo antes de mostrar la versión de pyenv sólo se utiliza si se utiliza pyenv        |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configura los binarios de python que Starship debería ejecutar al obtener la versión.   |
| `detect_extensions`  | `["py"]`                                                                                                     | Qué extensiones deben activar este módulo                                               |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Qué nombres de archivo deben activar este módulo                                        |
| `detect_folders`     | `[]`                                                                                                         | Qué carpetas deben activar este módulo                                                  |
| `disabled`           | `false`                                                                                                      | Desactiva el módulo `python`.                                                           |

::: tip

La variable `python_binary` acepta una cadena o una lista de cadenas. Starship intentará ejecutar cada binario hasta que obtenga un resultado. Ten en cuenta que sólo puedes cambiar el binario que Starship ejecuta para obtener la versión de Python no los argumentos que se utilizan.

Los valores por defecto y el orden para `python_binary` fue elegido para identificar primero la versión de Python en un entorno virtualenv/conda (que actualmente añade un `python`, no importa si apunta a `pithon3` o `pithon2`). Esto tiene el efecto secundario que si todavía tienes un sistema de Python 2 instalado, puede ser recogido antes de cualquier Python 3 (al menos en las Distros de Linux que siempre enlazan `/usr/bin/python` a Python 2). Si ya no trabajas con Python 2 pero no puedes removerlo del sistema, cambiando esto a `"python3"` ocultará cualquier versión de Python 2, ver ejemplo a continuación.

:::

### Variables

| Variable     | Ejemplo         | Descripción                                 |
| ------------ | --------------- | ------------------------------------------- |
| version      | `"v3.8.1"`      | La versión de `python`                      |
| symbol       | `"🐍 "`          | Refleja el valor de la opción `symbol`      |
| style        | `"yellow bold"` | Refleja el valor de la opción `style`       |
| pyenv_prefix | `"pyenv "`      | Ordena el valor de la opción `pyenv_prefix` |
| virtualenv   | `"venv"`        | El nombre actual del `virtualenv`           |

### Ejemplo

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Solo usa el binario `python3` para obtener la versión.
python_binary = "python3"
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
python_binary = ["./venv/bin/python", "python", "python3", "python2"]
```

## R

El módulo `rlang` muestra la versión instalada de [R](https://www.r-project.org/). El módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo con la extensión `.R`.
- El directorio actual contiene un archivo con la extensión `.Rd`.
- El directorio actual contiene un archivo con la extensión `.Rmd`.
- El directorio actual contiene un archivo con la extensión `.Rproj`.
- El directorio actual contiene un archivo con la extensión `.Rsx`.
- El directorio actual contiene un archivo `.Rprofile`
- El directorio actual contiene una carpeta `.Rproj.user`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"📐"`                                | Una cadena de formato que representa el símbolo de R.                                   |
| `style`             | `"blue bold"`                        | El estilo del módulo.                                                                   |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | Qué extensiones deben activar este módulo                                               |
| `detect_files`      | `[".Rprofile"]`                      | Qué nombres de archivo deben activar este módulo                                        |
| `detect_folders`    | `[".Rproj.user"]`                    | Qué carpetas deben activar este módulo                                                  |
| `disabled`          | `false`                              | Deshabilita el módulo `r`.                                                              |

### Variables

| Variable | Ejemplo       | Descripción                            |
| -------- | ------------- | -------------------------------------- |
| version  | `v4.0.5`      | La versión de `R`                      |
| symbol   |               | Refleja el valor de la opción `symbol` |
| style    | `"blue bold"` | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[rlang]
format = "with [📐 $version](blue bold) "
```

## Raku

El módulo `raku` muestra la versión instalada de [Raku](https://www.raku.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `META6.json`
- El directorio actual contiene un `.p6`, `.pm6`, `.raku`, `.rakumod` o `.pod6`

### Opciones

| Opción              | Por defecto                                      | Descripción                                                                             |
| ------------------- | ------------------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version-$vm_version )]($style)"` | La cadena de formato para el módulo.                                                    |
| `version_format`    | `"v${raw}"`                                      | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦋 "`                                           | El símbolo usado antes de mostrar la versión de Raku                                    |
| `detect_extensions` | `["p6", "pm6", "pod6", "raku", "rakumod"]`       | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["META6.json"]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                             | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold 149"`                                     | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                          | Deshabilita el módulo `raku`.                                                           |

### Variables

| Variable   | Ejemplo | Descripción                                                |
| ---------- | ------- | ---------------------------------------------------------- |
| version    | `v6.d`  | La versión de `raku`                                       |
| vm_version | `moar`  | La versión de la máquina virtual `raku` está construida en |
| symbol     |         | Refleja el valor de la opción `symbol`                     |
| style\*  |         | Refleja el valor de la opción `style`                      |

### Ejemplo

```toml
# ~/.config/starship.toml

[raku]
format = "vía [🦪 $version]($style) "
```

## Red

Por defecto, el módulo `red` muestra la versión actualmente instalada de [Red](https://www.red-lang.org/). El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo con extensión `.red` o `.Red`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | Una cadena de formato que representa el símbolo de Red.                                 |
| `detect_extensions` | `["red"]`                            | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"red bold"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Deshabilita el módulo `rojo`.                                                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | La versión de `red`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

Por defecto, el módulo `ruby` muestra la versión actualmente instalada de [Ruby](https://www.ruby-lang.org/). El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `Gemfile`
- El directorio actual contiene un archivo `.ruby-version`
- El directorio actual contiene un archivo `.rb`
- Las variables de entorno `RUBY_VERSION` o `RBENV_VERSION` están configuradas

Starship obtiene la versión actual de Ruby ejecutando `ruby -v`.

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | Una cadena de formato que representa el símbolo de Ruby.                                |
| `detect_extensions` | `["rb"]`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `detect_variables`  | `["RUBY_VERSION", "RBENV_VERSION"]`  | Qué variables de entorno deben activar este módulo.                                     |
| `style`             | `"bold red"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Desactiva el módulo `ruby`.                                                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | La versión de `ruby`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

Por defecto, el módulo `rust` muestra la versión instalada de [Rust](https://www.rust-lang.org/). El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `Cargo.toml`
- El directorio actual contiene un archivo con la extensión `.rs`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | Una cadena de formato que representa el símbolo de Rust                                 |
| `detect_extensions` | `["rs"]`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["Cargo.toml"]`                     | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold red"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Desactiva el módulo `rust`.                                                             |

### Variables

| Variable  | Ejemplo           | Descripción                                  |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | La versión de `rustc`                        |
| numver    | `1.51.0`          | El componente numérico de la versión `rustc` |
| toolchain | `beta`            | La versión de toolchain                      |
| symbol    |                   | Refleja el valor de la opción `symbol`       |
| style\* |                   | Refleja el valor de la opción `style`        |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

El módulo `scala` muestra la versión actualmente instalada de [Scala](https://www.scala-lang.org/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `build.sbt`, `.scalaenv` o `.sbtenv`
- El directorio actual contiene un archivo con la extensión `.scala` o `.sbt`
- El directorio actual contiene un directorio llamado `.metals`

### Opciones

| Opción              | Por defecto                              | Descripción                                                                             |
| ------------------- | ---------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                              | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[".metals"]`                            | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `"🆂 "`                                   | Una cadena de formato que representa el símbolo de Scala.                               |
| `style`             | `"red dimmed"`                           | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                  | Deshabilita el módulo `scala`.                                                          |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `2.13.5` | La versión de `scala`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

El módulo `shell` muestra un indicador para el intérprete de comandos actualmente utilizado.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción                 | Por defecto               | Descripción                                                          |
| ---------------------- | ------------------------- | -------------------------------------------------------------------- |
| `bash_indicator`       | `bsh`                     | Una cadena de formato usada para representar Bash.                   |
| `fish_indicator`       | `fsh`                     | Una cadena de formato usada para representar Fish.                   |
| `zsh_indicator`        | `zsh`                     | Una cadena de formato usada para representar Zsh.                    |
| `powershell_indicator` | `psh`                     | Una cadena de formato usada para representar Powershell.             |
| `ion_indicator`        | `ion`                     | Una cadena de formato usada para representar Ion.                    |
| `elvish_indicator`     | `esh`                     | Una cadena de formato usada para representar Elvish.                 |
| `tcsh_indicator`       | `tsh`                     | Una cadena de formato usada para representar tcsh.                   |
| `xonsh_indicator`      | `xsh`                     | Una cadena de formato usada para representar xonsh.                  |
| `cmd_indicator`        | `cmd`                     | Una cadena de formato usada para representar cmd.                    |
| `nu_indicator`         | `nu`                      | Una cadena de formato usada para representar nu.                     |
| `unknown_indicator`    |                           | El valor por defecto que se mostrará cuando se desconoce el comando. |
| `format`               | `"[$indicator]($style) "` | El formato del módulo.                                               |
| `style`                | `"white bold"`            | El estilo del módulo.                                                |
| `disabled`             | `true`                    | Deshabilita el módulo `shell`.                                       |

### Variables

| Variable  | Predeterminado | Descripción                                                                          |
| --------- | -------------- | ------------------------------------------------------------------------------------ |
| indicator |                | Ordena el valor de `indicator` para el intérprete de comandos actualmente utilizado. |
| style\* |                | Refleja el valor de la opción `style`.                                               |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplos

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
style = "cyan bold"
disabled = false
```

## SHLVL

El módulo `shlvl` muestra la variable de entorno [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ("shell level") actual, si está establecido en un número y reúne o supera el umbral especificado.

### Opciones

| Opción      | Por defecto                  | Descripción                                                       |
| ----------- | ---------------------------- | ----------------------------------------------------------------- |
| `threshold` | `2`                          | Mostrar umbral.                                                   |
| `format`    | `"[$symbol$shlvl]($style) "` | El formato del módulo.                                            |
| `symbol`    | `"↕️  "`                     | El símbolo utilizado para representar el `SHLVL`.                 |
| `repeat`    | `false`                      | Hace que el `symbol` se repita con la cantidad actual de `SHLVL`. |
| `style`     | `"bold yellow"`              | El estilo del módulo.                                             |
| `disabled`  | `true`                       | Desactiva el módulo `shlvl`.                                      |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | El valor actual de `SHLVL`             |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

El módulo `singularity` muestra la imagen de [singularity](https://sylabs.io/singularity/) actual, si se encuentra dentro de un contenedor y `$SINGULARITY_NAME` está establecido.

### Opciones

| Opción     | Por defecto                      | Descripción                                                         |
| ---------- | -------------------------------- | ------------------------------------------------------------------- |
| `format`   | `'[$symbol\[$env\]]($style) '` | El formato del módulo.                                              |
| `symbol`   | `""`                             | Una cadena de formato que se muestra antes del nombre de la imagen. |
| `style`    | `"bold dimmed blue"`             | El estilo del módulo.                                               |
| `disabled` | `false`                          | Desactiva el módulo `singularity`.                                  |

### Variables

| Variable  | Ejemplo      | Descripción                            |
| --------- | ------------ | -------------------------------------- |
| env       | `centos.img` | La imagen de Singularity actual        |
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

El módulo `spack` muestra el entorno actual [Spack](https://spack.readthedocs.io/en/latest/), si `$SPACK_ENV` está configurado.

### Opciones

| Opción              | Por defecto                            | Descripción                                                                                                                                               |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | El número de directorios a los que se debe truncar la ruta de entorno. `0` significa sin truncamiento. Mirar también el módulo [`directory`](#directory). |
| `symbol`            | `"🅢  "`                                | El símbolo usado antes del nombre del entorno.                                                                                                            |
| `style`             | `"bold blue"`                          | El estilo del módulo.                                                                                                                                     |
| `format`            | `"via [$symbol$environment]($style) "` | El formato del módulo.                                                                                                                                    |
| `disabled`          | `false`                                | Deshabilita el módulo `spack`.                                                                                                                            |

### Variables

| Variable    | Ejemplo      | Descripción                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | El entorno de spack actual             |
| symbol      |              | Refleja el valor de la opción `symbol` |
| style\*   |              | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[spack]
format = "[$symbol$environment](dimmed blue) "
```

## Status

El módulo `status` muestra el código de salida del comando anterior. Si $success_symbol está vacío (por defecto), el módulo solo se mostrará si el código de salida no es `0`. El código de estado se convertirá a un entero con signo de 32 bits.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción                      | Por defecto                                                                          | Descripción                                                                    |
| --------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------ |
| `format`                    | `"[$symbol$status]($style) "`                                                        | El formato del módulo                                                          |
| `symbol`                    | `"✖"`                                                                                | El símbolo mostrado en error del programa                                      |
| `success_symbol`            | `""`                                                                                 | El símbolo mostrado en el éxito del programa                                   |
| `not_executable_symbol`     | `"🚫"`                                                                                | El símbolo mostrado cuando el archivo no es ejecutable                         |
| `not_found_symbol`          | `"🔍"`                                                                                | El símbolo mostrado cuando no se encuentra el comando                          |
| `sigint_symbol`             | `"🧱"`                                                                                | El símbolo mostrado en SIGINT (Ctrl + c)                                       |
| `signal_symbol`             | `"⚡"`                                                                                | El símbolo mostrado en cualquier señal                                         |
| `style`                     | `"bold red"`                                                                         | El estilo del módulo.                                                          |
| `recognize_signal_code`     | `true`                                                                               | Activar mapeo de señales desde el código de salida                             |
| `map_symbol`                | `false`                                                                              | Activar mapeo de símbolos desde el código de salida                            |
| `pipestatus`                | `false`                                                                              | Habilita el reporte de pipstatus                                               |
| `pipestatus_separator`      | <code>&vert;</code>                                                            | El símbolo usado para separar segmentos de pipestatus                          |
| `pipestatus_format`         | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | El formato del módulo cuando el comando es un pipeline                         |
| `pipestatus_segment_format` |                                                                                      | Cuando se especifica, reemplaza `formato` al formatear segmentos de pipestatus |
| `disabled`                  | `true`                                                                               | Desactiva el módulo `status`.                                                  |

### Variables

| Variable       | Ejemplo | Descripción                                                                                                        |
| -------------- | ------- | ------------------------------------------------------------------------------------------------------------------ |
| status         | `127`   | El código de salida del último comando                                                                             |
| hex_status     | `0x7F`  | El código de salida del último comando en hexadecimal                                                              |
| int            | `127`   | El código de salida del último comando                                                                             |
| common_meaning | `ERROR` | Comprobación del código si no es una señal                                                                         |
| signal_number  | `9`     | Número de señal correspondiente al código de salida, sólo si está señalizada                                       |
| signal_name    | `KILL`  | Nombre de la señal correspondiente al código de salida, sólo si está señalizada                                    |
| maybe_int      | `7`     | Contiene el número de código de salida cuando no se ha encontrado ningún significado                               |
| pipestatus     |         | Renderizado en los códigos de salida de los programas del pipeline, esto sólo está disponible en pipestatus_format |
| symbol         |         | Refleja el valor de la opción `symbol`                                                                             |
| style\*      |         | Refleja el valor de la opción `style`                                                                              |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴 "
success_symbol = "🟢 SUCCESS"
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

El módulo `sudo` muestra si las credenciales de sudo están actualmente en caché. El módulo solo se mostrará si las credenciales están guardadas en caché.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción          | Por defecto             | Descripción                                                                                      |
| --------------- | ----------------------- | ------------------------------------------------------------------------------------------------ |
| `format`        | `[as $symbol]($style)"` | El formato del módulo                                                                            |
| `symbol`        | `"🧙 "`                  | El símbolo mostrado cuando las credenciales están guardadas en caché                             |
| `style`         | `"bold blue"`           | El estilo del módulo.                                                                            |
| `allow_windows` | `false`                 | Como Windows no tiene sudo de manera predeterminada, el valor predeterminado está deshabilitado. |
| `disabled`      | `true`                  | Deshabilita el módulo `sudo`.                                                                    |

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
style = "bold green"
symbol = "👩‍💻 "
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

Por defecto, el módulo `swift` muestra la versión instalada de [Swift](https://swift.org/). El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `Package.swift`
- El directorio actual contiene un archivo con la extensión `.swift`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | Una cadena de formato que representa el símbolo de Swift                                |
| `detect_extensions` | `["swift"]`                          | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["Package.swift"]`                  | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold 202"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Desactiva el módulo `swift`.                                                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.2.4` | La versión de `swift`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

El módulo `Terraform` muestra el espacio de trabajo de [Terraform](https://www.terraform.io/docs/language/state/workspaces.html) y la versión seleccionados actualmente.

::: tip

Por defecto no se muestra la versión de Terraform, ya que esto es lento para las versiones actuales de Terraform cuando hay muchos complementos en uso. Si aún deseas activarlo, [sigue el ejemplo que se muestra a continuación](#with-terraform-version).

:::

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene una carpeta `.terraform`
- El directorio actual contiene un archivo con las extensiones `.tf`, `.tfplan` o `.tfstate`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | La cadena de formato para el módulo.                                                    |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | Una cadena de formato que se muestra antes del espacio de trabajo de terraform.         |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]`        | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[".terraform"]`                     | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold 105"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Desactiva el módulo `terraform`.                                                        |

### Variables

| Variable  | Ejemplo    | Descripción                               |
| --------- | ---------- | ----------------------------------------- |
| version   | `v0.12.24` | La versión de `terraform`                 |
| workspace | `default`  | El espacio de trabajo actual de Terraform |
| symbol    |            | Refleja el valor de la opción `symbol`    |
| style\* |            | Refleja el valor de la opción `style`     |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

#### Con la versión de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Sin la versión de Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Hora

El módulo `time` muestra la hora **local** actual. El valor de configuración de `format` es usado por la caja de [`chrono`](https://crates.io/crates/chrono) para controlar cómo se muestra la hora. Echa un vistazo a [los documentos de chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) para ver qué opciones están disponibles.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción            | Predeterminado          | Descripción                                                                                                                                                                 |
| ----------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | La cadena de formato para el módulo.                                                                                                                                        |
| `use_12hr`        | `false`                 | Habilita el formato de 12 horas                                                                                                                                             |
| `time_format`     | see below               | La [cadena de formato de chrono](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) utilizada para formatear la hora.                                          |
| `style`           | `"bold yellow"`         | El estilo para la hora del módulo                                                                                                                                           |
| `utc_time_offset` | `"local"`               | Establece el desplazamiento UTC a utilizar. Rango de -24 &lt; x &lt; 24. Permite a los flotantes acomodar los desplazamientos de zona horaria de 30/45 minutos. |
| `disabled`        | `true`                  | Desactiva el módulo `time`.                                                                                                                                                 |
| `time_range`      | `"-"`                   | Establece el intervalo de tiempo durante el cual el módulo se mostrará. La hora debe ser especificada en formato de 24 horas                                                |

Si `use_12hr` es `true`, entonces `time_format` por defecto `"%r"`. De lo contrario, el valor por defecto es `"%T"`. Configurar manualmente `time_format` sobrescribirá la configuración `use_12hr`.

### Variables

| Variable  | Ejemplo    | Descripción                           |
| --------- | ---------- | ------------------------------------- |
| time      | `13:08:10` | La hora actual.                       |
| style\* |            | Refleja el valor de la opción `style` |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Nombre de usuario

El módulo `username` muestra el nombre de usuario activo. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El usuario actual es root/admin
- El usuario actual no es el mismo que el que está conectado
- El usuario está actualmente conectado como una sesión SSH
- La variable `show_always` se establece en true

::: tip

La conexión SSH se detecta comprobando las variables de entorno `SSH_CONNECTION`, `SSH_CLIENT`, y `SSH_TTY`. Si su host SSH no configura estas variables, una solución es establecer una de ellas con un valor tonto.

:::

### Opciones

| Opción        | Predeterminado          | Descripción                                      |
| ------------- | ----------------------- | ------------------------------------------------ |
| `style_root`  | `"bold red"`            | El estilo usado cuando el usuario es root/admin. |
| `style_user`  | `"bold yellow"`         | El estilo usado para usuarios no root.           |
| `format`      | `"[$user]($style) in "` | El formato del módulo.                           |
| `show_always` | `false`                 | Siempre muestra el módulo `username`.            |
| `disabled`    | `false`                 | Desactiva el módulo `username`.                  |

### Variables

| Variable | Ejemplo      | Descripción                                                                                         |
| -------- | ------------ | --------------------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Refleja el valor de la opción `style_root` cuando root inició sesión y `style_user` por otra parte. |
| `user`   | `"matchai"`  | El ID de usuario conectado actualmente.                                                             |

### Ejemplo

```toml
# ~/.config/starship.toml

[username]
style_user = "white bold"
style_root = "black bold"
format = "user: [$user]($style) "
disabled = false
show_always = true
```

## Vagrant

El módulo `vagrant` muestra la versión instalada de [Vagrant](https://www.vagrantup.com/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `Vagrantfile`

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | Una cadena de formato que representa el símbolo de Vagrant.                             |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["Vagrantfile"]`                    | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"cyan bold"`                        | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Deshabilita el módulo `vagrant`.                                                        |

### Variables

| Variable  | Ejemplo          | Descripción                            |
| --------- | ---------------- | -------------------------------------- |
| version   | `Vagrant 2.2.10` | La versión de `Vagrant`                |
| symbol    |                  | Refleja el valor de la opción `symbol` |
| style\* |                  | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## V

El módulo `vlang` te muestra la versión instalada de [V](https://vlang.io/). Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo con la extensión `.v`
- El directorio actual contiene un archivo `v.mod`, `vpkg.json` o `.vpkg-lock.json`

### Opciones

| Opción              | Predeterminado                               | Descripción                                                                             |
| ------------------- | -------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                                  | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"V "`                                       | Una cadena de formato que representa el símbolo de V                                    |
| `detect_extensions` | `["v"]`                                      | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                         | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"blue bold"`                                | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                      | Deshabilita el módulo `vlang`.                                                          |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| version   | `v0.2`  | La versión de `v`                      |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml
[vlang]
format = "via [V $version](blue bold) "
```

## VCSH

El módulo `vcsh` muestra el repositorio activo [VCSH](https://github.com/RichiH/vcsh) actual. El módulo sólo se mostrará si un repositorio está actualmente en uso.

### Opciones

| Opción     | Predeterminado                   | Descripción                                                  |
| ---------- | -------------------------------- | ------------------------------------------------------------ |
| `symbol`   |                                  | El símbolo usado antes de mostrar el nombre del repositorio. |
| `style`    | `"bold yellow"`                  | El estilo del módulo.                                        |
| `format`   | `"vcsh [$symbol$repo]($style) "` | El formato del módulo.                                       |
| `disabled` | `false`                          | Deshabilita el módulo `vcsh`.                                |

### Variables

| Variable    | Ejemplo                                                     | Descripción                            |
| ----------- | ----------------------------------------------------------- | -------------------------------------- |
| repositorio | `dotfiles` si está en un repositorio VCSH nombrado dotfiles | El nombre del repositorio activo       |
| symbol      |                                                             | Refleja el valor de la opción `symbol` |
| style\*   | `black bold dimmed`                                         | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

Por defecto, el módulo `zig` muestra la versión instalada de [Zig](https://ziglang.org/). El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `.zig`

### Opciones

| Opción              | Predeterminado                       | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `"v${raw}"`                          | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | El símbolo usado antes de mostrar la versión de Zig.                                    |
| `style`             | `"bold yellow"`                      | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Desactiva el módulo `zig`.                                                              |
| `detect_extensions` | `["zig"]`                            | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.6.0` | La versión de `zig`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Comandos personalizados

Los módulos `personalizados` muestran la salida de algunos comandos arbitrarios.

Estos módulos se mostrarán si se cumplen alguna de las siguientes condiciones:

- El directorio actual contiene un archivo cuyo nombre está en `detect_files`
- El directorio actual contiene un directorio cuyo nombre está en `detect_folders`
- El directorio actual contiene un archivo cuya extensión está en `detect_extensions`
- El comando `when` devuelve 0
- El sistema operativo actual (std::env::consts::OS) coincide con el campo `os` si está definido.

::: tip

Múltiples módulos personalizados pueden definirse usando una `.`.

:::

::: tip

El orden en el que se muestran los módulos personalizados se puede establecer individualmente incluyendo `${custom.foo}` en el `format` de nivel superior (ya que incluye un punto, necesita usar `${...}`). Por defecto, el módulo `custom` simplemente mostrará todos los módulos personalizados en el orden en que fueron definidos.

:::

::: tip

[El issue #1252](https://github.com/starship/starship/discussions/1252) contiene ejemplos de módulos personalizados. Si tiene un ejemplo interesante no cubierto allí, no dude en compartirlo allí!

:::

::: warning la salida de comandos se imprime sin escapar al prompt

Cualquiera que sea la salida que genere el comando se imprime sin modificar en el prompt. Esto significa que si la salida contiene secuencias especiales que son interpretadas por el intérprete, se expandirán cuando se muestre. Estas secuencias especiales son específicas del intérprete, por ejemplo, puedes escribir un módulo de comando que escribe secuencias de bash, por ejemplo, `\h`, pero este módulo no funcionará en el intérprete fish o zsh.

Las cadenas de formato también pueden contener secuencias específicas del intérprete, por ejemplo, [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

:::

### Opciones

| Opción              | Predeterminado                  | Descripción                                                                                                                                                                                                                                                                                                                      |
| ------------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `""`                            | El comando cuya salida debe ser impresa. El comando se pasará en stdin al shell.                                                                                                                                                                                                                                                 |
| `when`              | `false`                         | Valor booleano (`true` o `false`, sin comillas) o un comando de shell usado como una condición para mostrar el módulo. En caso de una cadena, el módulo se mostrará si el comando devuelve un código de estado `0`.                                                                                                              |
| `shell`             |                                 | [Ver abajo](#custom-command-shell)                                                                                                                                                                                                                                                                                               |
| `description`       | `"<custom module>"`       | La descripción del módulo que se muestra al ejecutar `starship explain`.                                                                                                                                                                                                                                                         |
| `detect_files`      | `[]`                            | Los archivos que se buscarán en el directorio de trabajo para obtener una coincidencia.                                                                                                                                                                                                                                          |
| `detect_folders`    | `[]`                            | Los directorios que se buscarán en el directorio de trabajo para una coincidencia.                                                                                                                                                                                                                                               |
| `detect_extensions` | `[]`                            | Las extensiones que se buscarán en el directorio de trabajo para obtener una coincidencia.                                                                                                                                                                                                                                       |
| `symbol`            | `""`                            | El símbolo usado antes de mostrar la salida del comando.                                                                                                                                                                                                                                                                         |
| `style`             | `"bold green"`                  | El estilo del módulo.                                                                                                                                                                                                                                                                                                            |
| `format`            | `"[$symbol($output )]($style)"` | El formato del módulo.                                                                                                                                                                                                                                                                                                           |
| `disabled`          | `false`                         | Desactiva este módulo `custom`.                                                                                                                                                                                                                                                                                                  |
| `os`                |                                 | Nombre del sistema operativo en el que se mostrará el módulo (unix, linux, macos, windows, ... ) [Ver valores posibles](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                                              |
| `use_stdin`         |                                 | Un valor booleano opcional que anula si los comandos deben ser reenviados al shell a través de la entrada estándar o como argumento. Si la entrada estándar unset es usada de manera predeterminada, a menos que el shell no lo soporte (cmd, nushell). Configurar esto desactiva el manejo de argumentos específicos del shell. |
| `ignore_timeout`    | `false`                         | Ignorar la configuración global de `command_timeout` y seguir ejecutando comandos externos, sin importar el tiempo que tarden.                                                                                                                                                                                                   |

### Variables

| Variable  | Descripción                               |
| --------- | ----------------------------------------- |
| output    | La salida del comando de shell en `shell` |
| symbol    | Refleja el valor de la opción `symbol`    |
| style\* | Refleja el valor de la opción `style`     |

*: Esta variable sólo puede ser usada como parte de una cadena de estilo

#### Comando personalizado del intérprete de comandos

`shell` acepta una lista no vacía de cadenas, donde:

- La primera cadena es la ruta al intérprete de comandos a usar para ejecutar el comando.
- Otros argumentos siguientes son pasados al shell.

Si no está activado, se retornará a STARSHIP_SHELL y luego a "sh" en Linux, y "cmd /C" en Windows.

El `comando` será pasado en stdin.

Si no se da el `shell` o solo contiene un elemento y Starship detecta PowerShell los siguientes argumentos se añadirán automáticamente: `-NoProfile -Command -`. If `shell` is not given or only contains one element and Starship detects Cmd will be used, the following argument will automatically be added: `/C` and `stdin` will be set to `false`. If `shell` is not given or only contains one element and Starship detects Nushell will be used, the following arguments will automatically be added: `-c` and `stdin` will be set to `false`. Este comportamiento puede evitarse pasando explícitamente argumentos al intérprete, p.ej.

```toml
shell = ["pwsh", "-Command", "-"]
```

::: warning Asegúrate de que tu configuración personalizada del intérprete de comandos salga con éxito

Si establece un comando personalizado, asegúrese de que el Shell por defecto usado por starship ejecutará correctamente el comando con una salida elgante (a través de la opción `shell`).

Por ejemplo, PowerShell requiere el parámetro `-Command` para ejecutar una sola línea. Omitir este parámetro puede arrojar a starchip a un bucle recursivo donde el shell podría intentar cargar un entorno de perfil completo con starship en sí misma y volver a ejecutar el comando personalizado, entrando en un bucle infinito.

Se recomiendan parámetros similares a `-NoProfile` en PowerShell para otros shells para evitar tiempo extra de carga de un perfil personalizado en cada invocación de starship.

La detección automática de shells y la adición adecuada de parámetros están actualmente implementados, pero es posible que no todos los shells estén cubiertos. Por favor, [abre un issue](https://github.com/starship/starship/issues/new/choose) con los detalles del intérprete de comandos y la configuración de Starship si te encuentras en tal escenario.

:::

### Ejemplo

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo" # shows output of command
detect_files = ["foo"] # can specify filters but wildcards are not supported
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
detect_extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]

[custom.time-as-arg]
command = "time /T"
detect_extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command"]
use_stdin = false
```
