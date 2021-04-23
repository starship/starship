# Configuración

Para comenzar a configurar Starship, crea el siguiente archivo: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && starship print-config --default > ~/.config/starship.toml
```

Toda la configuración de Starship se realiza en este archivo [TOML](https://github.com/toml-lang/toml):

```toml
# Inserta una línea en blanco al inicio del prompt
add_newline = true

# Reemplaza el símbolo "❯" por "➜" en el prompt
[character]                            # El nombre del módulo que se está configurandoes "character"
success_symbol = "[➜](bold green)"     # El segmento "success_symbol" es reemplzado por "➜" con el color "bold green"

# Deshabilita el módulo "package", ocultándolo por completo del prompt
[package]
disabled = true
```

Puedes cambiar la ubicación por defecto del archivo de configuración con la variable de entorno `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/.starship/config.toml
```

El equivalente en PowerShell (Windows) es añadir esta línea a tu `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
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

### Terminología

**Módulo**: un componente en el promt que provee información basada en información contextual de tu sistema operativo. Por ejemplo, el módulo "nodejs" muestra la versión de NodeJS que tienes actualmente instalada en tu ordenador, si el directorio actual es un proyecto NodeJS.

**Variable**: subcomponentes más pequeños que contienen información proporcionada por el módulo. Por ejemplo, la variable "version" en el módulo "nodejs" contiene la versión actual de NodeJS.

Por convención, la mayoría de los módulos tienen un prefijo del color por defecto de la terminal (por ejemplo, `vía` en "nodejs") y un espacio vacío como sufijo.

### Cadenas de Formato

Es el formato con el que un módulo imprime todas sus variables. La mayoría de los módulos tienen una entrada llamada `format` que configura el formato de visualización del módulo. Se puede utilizar textos, variables y grupos de texto.

#### Variable

Una variable contiene un símbolo `$` seguido por el nombre de la variable. El nombre de una variable solo contiene letras, números y `_`.

Por ejemplo:

- `$version` es una cadena de formato con una variable llamada `version`.
- `$git_branch$git_commit` es un formato de cadena de texto con dos variables nombradas `git_branch` y `git_commit`.
- `$git_branch $git_commit` tiene las dos variables separadas por un espacio.

#### Grupo de Texto

Un grupo de texto se compone de dos partes diferentes.

La primera parte, que está encerrada en un `[]`, es una [cadena de formato](#format-strings). Se puede agregar textos, variables, o incluso grupos de texto anidados.

En la segunda parte, que está encerrada entre `()`, es una [cadena de estilo](#style-strings). Se puede utilizar el estilo de la primera parte.

Por ejemplo:

- `[en](bold red)` imprimirá una cadena `en` con texto en negrita color rojo.
- `[⌘ $version](bold green)` imprimirá un símbolo `⌘` seguido por el contenido de la variable `version`, con texto en negrita color verde.
- `[a [b](red) c](green)` imprimirá `a b c` con `b` en rojo, `a` y `c` en verde.

#### Cadenas de estilo

La mayoría de los módulos de starship permiten configurar los estilos de su cadenas texto. Esto se consigue con una entrada (normalmente llamada `style` - estilo) que no es más que un texto donde se especifica la configuración. A continuación mostramos algunos ejemplos de cadenas de estilo junto con su funcionalidad. Para más detalles sobre la sintaxis completa, consulta [la guía de configuración avanzada](/advanced-config/).

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

- `(@$region)` no mostrará nada si la variable `region` es `None`, de lo contrario `@` seguido por el valor de la región.
- `(algún texto)` siempre mostrará nada ya que no hay variables envueltas entre llaves.
- Cuando `$all` es un atajo para `\[$a$b\]`, `($all)` no mostrará nada solo si `$a` y `$b` ambos son `None`. Esto funciona igual que `(\[$a$b\] )`.

#### Caracteres de escape

Los siguientes símbolos tienen un uso especial en una cadena de formato. Si se quiere imprimir los siguientes símbolos, se tienen que escapar con una barra invertida (`\`).

- \$
- \\
- [
- ]
- (
- )

Se debe tener en cuenta que `toml` tiene [su propia sintaxis de escape](https://github.com/toml-lang/toml#user-content-string). Se recomienda usar una cadena literal (`''`) en la configuración. Si se desea utilizar una cadena básica (`""`), prestar atención para escapar la barra invertida `\`.

Por ejemplo, cuando se desea imprimir un símbolo `$` en una nueva línea, las siguientes configuraciones para la variable `format` son equivalentes:

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

## Prompt

Esta es la lista de opciones de configuración del prompt.

### Opciones

| Opción            | Por defecto                        | Descripción                                                                   |
| ----------------- | ---------------------------------- | ----------------------------------------------------------------------------- |
| `format`          | [ver aquí](#default-prompt-format) | Configura el formato del prompt.                                              |
| `scan_timeout`    | `30`                               | Tiempo de espera tras el que Starship escanea los archivos (en milisegundos). |
| `command_timeout` | `500`                              | Timeout for commands executed by starship (in milliseconds).                  |
| `add_newline`     | `true`                             | Inserts blank line between shell prompts.                                     |

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

### Formato por Defecto del Prompt

La varieble `format` por defecto se utiliza para definir el formato del prompt, si está vacía o `format` no se proporciona. El valor por defecto es el siguiente:

```toml
format = "$all"

# Which is equivalent to
format = """
$username\
$hostname\
$shlvl\
$kubernetes\
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$kotlin\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vagrant\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$env_var\
$crystal\
$custom\
$cmd_duration\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

## AWS

El módulo `aws` muestra la región y el perfil actual de AWS. Éste se basa en las variables de entorno `AWS_REGION`, `AWS_DEFAULT_REGION`, y `AWS_PROFILE` del fichero `~/.aws/config`.

Cuando uses [aws-vault](https://github.com/99designs/aws-vault) el perfil se obtiene de la variable de entorno `AWS_VAULT`.

Cuando uses [awsu](https://github.com/kreuzwerker/awsu) el perfil se obtiene de la variable de entorno `AWSU_PROFILE`.

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var.

### Opciones

| Opción           | Por defecto                                         | Descripción                                                  |
| ---------------- | --------------------------------------------------- | ------------------------------------------------------------ |
| `format`         | `'on [$symbol($profile )(\($region\) )]($style)'` | El formato del módulo.                                       |
| `symbol`         | `"☁️ "`                                             | El símbolo que se muestra antes del perfil de AWS.           |
| `region_aliases` |                                                     | Tabla de alias de región para mostrar además del nombre AWS. |
| `style`          | `"bold yellow"`                                     | El estilo del módulo.                                        |
| `disabled`       | `false`                                             | Deshabilita el módulo `aws`.                                 |

### Variables

| Variable  | Ejemplo          | Descripción                            |
| --------- | ---------------- | -------------------------------------- |
| region    | `ap-northeast-1` | La región actual de AWS                |
| profile   | `astronauts`     | El perfil actual de AWS                |
| symbol    |                  | Refleja el valor de la opción `symbol` |
| style\* |                  | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

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
```

#### Mostrar región

```toml
# ~/.config/starship.toml

[aws]
format = "en [$symbol$region]($style) "
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
format = "en [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
```

## Battery

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### Opciones

| Opción               | Por defecto                       | Descripción                                                              |
| -------------------- | --------------------------------- | ------------------------------------------------------------------------ |
| `full_symbol`        | `" "`                            | Se muestra cuando la batería está cargada.                               |
| `charging_symbol`    | `" "`                            | Se muestra cuando la batería se está cargando.                           |
| `discharging_symbol` | `" "`                            | Se muestra cuando la batería se está descargando.                        |
| `unknown_symbol`     | `" "`                            | El símbolo que se muestra cuando el estado de la batería es desconocido. |
| `empty_symbol`       | `" "`                            | El símbolo que se muestra cuando el estado de la batería está vacío.     |
| `format`             | `"[$symbol$percentage]($style) "` | El formato del módulo.                                                   |
| `display`            | [ver aquí](#battery-display)      | Define cuándo mostrar el indicador y el estilo.                          |
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

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style). If no `display` is provided. El valor por defecto es el siguiente:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Opciones

The `display` option is an array of the following table.

| Opción               | Por defecto | Descripción                                                                                               |
| -------------------- | ----------- | --------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`        | The upper bound for the display option.                                                                   |
| `style`              | `bold red`  | The style used if the display option is in use.                                                           |
| `charging_symbol`    | `-`         | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` | `-`         | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### Ejemplo

```toml
[[battery.display]]  # "bold red" style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"
discharging_symbol = 💦

# when capacity is over 30%, the battery indicator will not be displayed

```

## Character

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways:

- Cambiando el color (`red`/`green`)
- Cambiando la forma (`.`/`✖`)

By default it only changes color. If you also want to change it's shape take a look at [this example](#with-custom-error-shape).

::: warning `error_symbol` is not supported on elvish shell. :::

### Opciones

| Opción           | Por defecto         | Descripción                                                                                             |
| ---------------- | ------------------- | ------------------------------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | La cadena de formato usada antes de la entrada de texto.                                                |
| `success_symbol` | `"[❯](bold green)"` | La cadena de formato usada antes de la entrada de texto si el comando anterior tuvo éxito.              |
| `error_symbol`   | `"[❯](bold red)"`   | La cadena de formato usada antes de la entrada de texto si el comando anterior falló.                   |
| `vicmd_symbol`   | `"[❮](bold green)"` | El cadena de formato antes de la entrada de texto si el intérprete de comandos está en modo vim normal. |
| `disabled`       | `false`             | Desactiva el módulo `character`.                                                                        |

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

The `cmake` module shows the currently installed version of CMake. By default the module will be activated if any of the following conditions are met:

- El directorio actual contiene un archivo `CMakeLists.txt`
- El directorio actual contiene un archivo `CMakeCache.txt`

### Opciones

| Opción              | Por defecto                            | Descripción                                      |
| ------------------- | -------------------------------------- | ------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`   | El formato del módulo.                           |
| `symbol`            | `"△ "`                                 | El símbolo usado antes de la versión de cmake.   |
| `detect_extensions` | `[]`                                   | Qué extensiones deben activar este módulo        |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Qué nombres de archivo deben activar este módulo |
| `detect_folders`    | `[]`                                   | Qué carpetas deben activar este módulo           |
| `style`             | `"bold blue"`                          | El estilo del módulo.                            |
| `disabled`          | `false`                                | Deshabilita el módulo `cmake`.                   |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | La versión de cmake                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

## Tiempo de Ejecución

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Opciones

| Opción               | Por defecto                   | Descripción                                                            |
| -------------------- | ----------------------------- | ---------------------------------------------------------------------- |
| `min_time`           | `2_000`                       | Duración mínima para mostrar el tiempo de ejecución (en milisegundos)  |
| `show_milliseconds`  | `false`                       | Muestra la duración con precisión en milisegundos.                     |
| `format`             | `"took [$duration]($style) "` | El formato del módulo.                                                 |
| `style`              | `"bold yellow"`               | El estilo del módulo.                                                  |
| `disabled`           | `false`                       | Desactiva el módulo `cmd_duration`.                                    |
| `show_notifications` | `false`                       | Muestra notificaciones de escritorio cuando se complete el comando.    |
| `min_time_to_notify` | `45_000`                      | Duración mínima para mostrar el tiempo de ejecución (en milisegundos). |

::: tip

Showing desktop notifications requires starship to be built with `rust-notify` support. You check if your starship supports notifications by running `STARSHIP_LOG=debug starship module cmd_duration -d 60000` when `show_notifications` is set to `true`.

:::

### Variables

| Variable  | Ejemplo  | Descripción                                |
| --------- | -------- | ------------------------------------------ |
| duration  | `16m40s` | El tiempo que tardó en ejecutar el comando |
| style\* |          | Refleja el valor de la opción `style`      |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

The `conda` module shows the current conda environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

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

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `shard.yml`
- El directorio actual contiene un fichero `.cr`

### Opciones

| Opción              | Por defecto                          | Descripción                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | Símbolo usado antes de la versión de Crystal.        |
| `style`             | `"bold red"`                         | El estilo del módulo.                                |
| `detect_extensions` | `["cr"]`                             | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `["shard.yml"]`                      | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.           |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                               |
| `disabled`          | `false`                              | Deshabilita el módulo `crystal`.                     |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | La versión de `crystal`                |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

The `dart` module shows the currently installed version of Dart. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo con la extensión `.dart`
- El directorio actual contiene un directorio `.dart_tool`
- El directorio actual contiene un archivo `pubspec.yaml`, `pubspec.yml` o `pubspec.lock`

### Opciones

| Opción              | Por defecto                                       | Descripción                                             |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | El formato del módulo.                                  |
| `symbol`            | `"🎯 "`                                            | Una cadena de formato que representa el símbolo de Dart |
| `detect_extensions` | `["dart"]`                                        | Qué extensiones deberían activar este módulo.           |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`    | `[".dart_tool"]`                                  | Qué carpetas deberían activar este módulo.              |
| `style`             | `"bold blue"`                                     | El estilo del módulo.                                   |
| `disabled`          | `false`                                           | Deshabilita el módulo `dart`.                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.8.4` | La versión de `dart`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

The `deno` module shows you your currently installed version of Deno. By default the module will be shown if any of the following conditions are met:
- El directorio actual contiene un archivo `mod.ts`, `mod.js`, `deps.ts` o `deps.js`

### Opciones

| Opción              | Por defecto                                  | Descripción                                             |
| ------------------- | -------------------------------------------- | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | El formato del módulo.                                  |
| `symbol`            | `"🦕 "`                                       | Una cadena de formato que representa el símbolo de Deno |
| `detect_extensions` | `[]`                                         | Qué extensiones deberían activar este módulo.           |
| `detect_files`      | `["mod.ts", "mod.js", "deps.ts", "deps.js"]` | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`    | `[]`                                         | Qué carpetas deberían activar este módulo.              |
| `style`             | `"green bold"`                               | El estilo del módulo.                                   |
| `disabled`          | `false`                                      | Deshabilita el módulo `deno`.                           |

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

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Opciones

| Opción              | Por defecto                                        | Descripción                                                                    |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------ |
| `truncation_length` | `3`                                                | El número de directorios padre a los que se debe truncar el directorio actual. |
| `truncate_to_repo`  | `true`                                             | Truncar o no hasta la raíz del repositorio git en el que se esté.              |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | El formato del módulo.                                                         |
| `style`             | `"bold cyan"`                                      | El estilo del módulo.                                                          |
| `disabled`          | `false`                                            | Deshabilita el módulo `directory`.                                             |
| `read_only`         | `"🔒"`                                              | El símbolo que indica si el directorio actual es de sólo lectura.              |
| `read_only_style`   | `"red"`                                            | El estilo para el símbolo de sólo lectura.                                     |
| `truncation_symbol` | `""`                                               | El símbolo a prefijar a las rutas truncadas. p. ej.: "…/"                      |
| `home_symbol`       | `"~"`                                              | El símbolo que indica el directorio personal.                                  |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Opciones Avanzadas          | Por defecto | Descripción                                                                                                                                                            |
| --------------------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |             | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`         | The number of characters to use when applying fish shell pwd path logic.                                                                                               |
| `use_logical_path`          | `true`      | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable  | Ejemplo               | Descripción                           |
| --------- | --------------------- | ------------------------------------- |
| path      | `"D:/Projects"`       | La ruta del directorio actual         |
| style\* | `"black bold dimmed"` | Refleja el valor de la opción `style` |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### Opciones

| Opción              | Por defecto                                                   | Descripción                                                                                                              |
| ------------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol$context]($style) "`                            | El formato del módulo.                                                                                                   |
| `symbol`            | `"🐳 "`                                                        | El símbolo usado antes de mostrar el contexto de Docker.                                                                 |
| `only_with_files`   | `true`                                                        | Mostrar solo cuando haya una coincidencia                                                                                |
| `detect_extensions` | `[]`                                                          | Qué extensiones deberían activar este módulo (necesita que `only_with_files` sea verdadero, con un valor "true").        |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Qué nombres de archivo deberían activar este módulo (necesita que `only_with_files` sea verdadero, con un valor "true"). |
| `detect_folders`    | `[]`                                                          | Qué carpetas deberían activar este módulo (necesita que `only_with_files` sea verdadero, con un valor "true").           |
| `style`             | `"blue bold"`                                                 | El estilo del módulo.                                                                                                    |
| `disabled`          | `false`                                                       | Deshabilita el módulo `docker_context`.                                                                                  |

### Variables

| Variable  | Ejemplo        | Descripción                            |
| --------- | -------------- | -------------------------------------- |
| context   | `test_context` | El contexto actual de docker           |
| symbol    |                | Refleja el valor de la opción `symbol` |
| style\* |                | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

By default this module will only be shown in your prompt when one or more of the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.sln`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Opciones

| Opción              | Por defecto                                                                                             | Descripción                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| `format`            | `"[$symbol($version )(🎯 $tfm )]($style)"`                                                               | El formato del módulo.                                                          |
| `symbol`            | `".NET "`                                                                                               | El símbolo usado antes de mostrar la version de dotnet.                         |
| `heuristic`         | `true`                                                                                                  | Usa una detección de versiones más rápida para mantener la nave espacial veloz. |
| `detect_extensions` | `["sln", "csproj", "fsproj", "xproj"]`                                                                  | Qué extensiones deberían activar este módulo.                                   |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Qué nombres de archivo deberían activar este módulo.                            |
| `detect_folders`    | `[]`                                                                                                    | Qué carpetas deberían activar estos módulos.                                    |
| `style`             | `"bold blue"`                                                                                           | El estilo del módulo.                                                           |
| `disabled`          | `false`                                                                                                 | Deshabilita el módulo `dotnet`.                                                 |

### Variables

| Variable  | Ejemplo          | Descripción                                                     |
| --------- | ---------------- | --------------------------------------------------------------- |
| version   | `v3.1.201`       | La version del SDK de `dotnet`                                  |
| tfm       | `netstandard2.0` | El Target Framework Moniker al que se dirige el proyecto actual |
| symbol    |                  | Refleja el valor de la opción `symbol`                          |
| style\* |                  | Refleja el valor de la opción `style`                           |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `mix.exs`.

### Opciones

| Opción              | Por defecto                                                 | Descripción                                                    |
| ------------------- | ----------------------------------------------------------- | -------------------------------------------------------------- |
| `symbol`            | `"💧 "`                                                      | El símbolo usado antes de mostrar la versión de Elixir/Erlang. |
| `detect_extensions` | `[]`                                                        | Qué extensiones deberían activar este módulo.                  |
| `detect_files`      | `["mix.exs"]`                                               | Qué nombres de archivo deberían activar este módulo.           |
| `detect_folders`    | `[]`                                                        | Qué carpetas deberían activar estos módulos.                   |
| `style`             | `"bold purple"`                                             | El estilo del módulo.                                          |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | El formato para el módulo elixir.                              |
| `disabled`          | `false`                                                     | Deshabilita el módulo `elixir`.                                |

### Variables

| Variable    | Ejemplo | Descripción                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | La version de `elixir`                 |
| otp_version |         | La versión de otp de `elixir`          |
| symbol      |         | Refleja el valor de la opción `symbol` |
| style\*   |         | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `elm.json`
- El directorio actual contiene un archivo `elm-package.json`
- El directorio actual contiene un archivo `.elm-version`
- El directorio actual contiene una carpeta `elm-stuff`
- El directorio actual contiene archivos `*.elm`

### Opciones

| Opción              | Por defecto                                        | Descripción                                             |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | El formato del módulo.                                  |
| `symbol`            | `"🌳 "`                                             | Una cadena de formato que representa el símbolo de Elm. |
| `detect_extensions` | `["elm"]`                                          | Qué extensiones deberían activar este módulo.           |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`    | `["elm-stuff"]`                                    | Qué carpetas deberían activar estos módulos.            |
| `style`             | `"cyan bold"`                                      | El estilo del módulo.                                   |
| `disabled`          | `false`                                            | Deshabilita el módulo `elm`.                            |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.19.1` | La versión de `elm`                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Variable de entorno

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- La opción de configuración de `variable` coincide con una variable de entorno existente
- La opción de configuración de `variable` no está definida, pero la opción de configuración se encuentra `por defecto`

### Opciones

| Opción     | Por defecto                    | Descripción                                                                            |
| ---------- | ------------------------------ | -------------------------------------------------------------------------------------- |
| `symbol`   |                                | El símbolo usado antes de mostrar el valor de la variable.                             |
| `variable` |                                | La variable de entorno a mostrar.                                                      |
| `default`  |                                | El valor por defecto que se mostrará cuando la variable seleccionada no está definida. |
| `format`   | `"with [$env_value]($style) "` | El formato del módulo.                                                                 |
| `disabled` | `false`                        | Deshabilita el módulo `env_var`.                                                       |

### Variables

| Variable  | Ejemplo                               | Descripción                                 |
| --------- | ------------------------------------- | ------------------------------------------- |
| env_value | `Windows NT` (si _variable_ es `$OS`) | El valor de entorno de la opción `variable` |
| symbol    |                                       | Refleja el valor de la opción `symbol`      |
| style\* | `black bold dimmed`                   | Refleja el valor de la opción `style`       |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `rebar.config`.
- El directorio actual contiene un fichero `erlang.mk`.

### Opciones

| Opción              | Por defecto                          | Descripción                                             |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| `symbol`            | `" "`                               | El símbolo usado antes de mostrar la versión de Erlang. |
| `style`             | `"bold red"`                         | El estilo del módulo.                                   |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.           |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.            |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                  |
| `disabled`          | `false`                              | Deshabilita el módulo `erlang`.                         |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | La versión de `erlang`                 |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Opciones

| Opción           | Por defecto                                      | Descripción                                                |
| ---------------- | ------------------------------------------------ | ---------------------------------------------------------- |
| `format`         | `'on [$symbol$account(\($region\))]($style) '` | El formato del módulo.                                     |
| `symbol`         | `"☁️ "`                                          | El símbolo usado antes de mostrar el perfil actual de GCP. |
| `region_aliases` |                                                  | Tabla de alias de región a mostrar además del nombre GCP.  |
| `style`          | `"bold blue"`                                    | El estilo del módulo.                                      |
| `disabled`       | `false`                                          | Deshabilita el módulo `gcloud`.                            |

### Variables

| Variable  | Ejemplo           | Descripción                                                                   |
| --------- | ----------------- | ----------------------------------------------------------------------------- |
| region    | `us-central1`     | La región GCP actual                                                          |
| account   | `foo@example.com` | El perfil actual de GCP                                                       |
| project   |                   | El proyecto GCP actual                                                        |
| active    | `default`         | El nombre de configuración activo escrito en `~/.config/gcloud/active_config` |
| symbol    |                   | Refleja el valor de la opción `symbol`                                        |
| style\* |                   | Refleja el valor de la opción `style`                                         |

\*: This variable can only be used as a part of a style string

### Ejemplos

#### Mostrar cuenta y proyecto

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(\($project\))]($style) '
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

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Opciones

| Opción               | Por defecto                      | Descripción                                                                                                   |
| -------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Muestra el nombre de la rama de seguimiento remoto, incluso si es igual al nombre de la rama local.           |
| `format`             | `"on [$symbol$branch]($style) "` | El formato del módulo. Usar `"$branch"` para referirse al nombre de la rama actual.                           |
| `symbol`             | `" "`                           | Una cadena de formato que representa el símbolo de la rama git.                                               |
| `style`              | `"bold purple"`                  | El estilo del módulo.                                                                                         |
| `truncation_length`  | `2^63 - 1`                       | Trunca el nombre de la rama a `N` grafemas.                                                                   |
| `truncation_symbol`  | `"…"`                            | El símbolo usado para indicar que un nombre de rama fue truncado. Se puede utilizar `""` para ningún símbolo. |
| `only_attached`      | `false`                          | Mostrar solo el hash de la confirmación de git cuando esté en estado "detached `HEAD`"                        |
| `disabled`           | `false`                          | Deshabilita el módulo `git_branch`.                                                                           |

### Variables

| Variable      | Ejemplo  | Descripción                                                                                                    |
| ------------- | -------- | -------------------------------------------------------------------------------------------------------------- |
| branch        | `master` | El nombre de la rama actual, vuelve a `HEAD` si no hay ninguna rama actual (por ejemplo, git detached `HEAD`). |
| remote_name   | `origin` | El nombre remoto.                                                                                              |
| remote_branch | `master` | El nombre de la rama rastreada en `remote_name`.                                                               |
| symbol        |          | Refleja el valor de la opción `symbol`                                                                         |
| style\*     |          | Refleja el valor de la opción `style`                                                                          |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Opciones

| Opción               | Por defecto                                            | Descripción                                                                            |
| -------------------- | ------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | La longitud del hash de la confirmación de git mostrado.                               |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | El formato del módulo.                                                                 |
| `style`              | `"bold green"`                                         | El estilo del módulo.                                                                  |
| `only_detached`      | `true`                                                 | Mostrar solo el hash de la confirmación de git cuando esté en estado "detached `HEAD`" |
| `tag_disabled`       | `true`                                                 | Deshabilita mostrar información de etiquetas en el módulo `git_commit`.                |
| `tag_symbol`         | `"🏷 "`                                                 | Símbolo de etiqueta prefijando la información mostrada                                 |
| `disabled`           | `false`                                                | Deshabilita el módulo `git_commit`.                                                    |

### Variables

| Variable  | Ejemplo   | Descripción                              |
| --------- | --------- | ---------------------------------------- |
| hash      | `b703eb3` | El hash actual de la confirmación de git |
| style\* |           | Refleja el valor de la opción `style`    |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Opciones

| Opción         | Por defecto                                                     | Descripción                                                                                         |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | Una cadena de formato que se muestra cuando un `rebase` está en progreso.                           |
| `merge`        | `"MERGING"`                                                     | Una cadena de formato que se muestra cuando un `merge` está en progreso.                            |
| `revert`       | `"REVERTING"`                                                   | Una cadena de formato mostrada cuando un `revert` está en progreso.                                 |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | Una cadena de formato que se muestra cuando un `cherry-pick` está en progreso.                      |
| `bisect`       | `"BISECTING"`                                                   | Una cadena de formato que se muestra cuando un `bisect` está en progreso.                           |
| `am`           | `"AM"`                                                          | Una cadena de formato que se muestra cuando un `apply-mailbox` (`git am`) está en progeso.          |
| `am_or_rebase` | `"AM/REBASE"`                                                   | Una cadena de formato que se muestra cuando un ambiguo `apply-mailbox` o `rebase` está en progreso. |
| `style`        | `"bold yellow"`                                                 | El estilo del módulo.                                                                               |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | El formato del módulo.                                                                              |
| `disabled`     | `false`                                                         | Deshabilita el módulo `git_state`.                                                                  |

### Variables

| Variable         | Ejemplo    | Descripción                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | El estado actual del repositorio      |
| progress_current | `1`        | El progreso de la operación actual    |
| progress_total   | `2`        | El progreso total de la operación     |
| style\*        |            | Refleja el valor de la opción `style` |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Opciones

| Opción       | Por defecto                                     | Descripción                              |
| ------------ | ----------------------------------------------- | ---------------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | El formato por defecto para `git_status` |
| `conflicted` | `"="`                                           | Esta rama tiene conflictos de fusión.    |
| `ahead`      | `"⇡"`                                           | El formato de `ahead`                    |
| `behind`     | `"⇣"`                                           | El formato de `behind`                   |
| `diverged`   | `"⇕"`                                           | El formato de `diverged`                 |
| `untracked`  | `"?"`                                           | El formato de `untracked`                |
| `stashed`    | `"$"`                                           | El formato de `stashed`                  |
| `modified`   | `"!"`                                           | El formato de `modified`                 |
| `staged`     | `"+"`                                           | El formato de `staged`                   |
| `renamed`    | `"»"`                                           | El formato de `renamed`                  |
| `deleted`    | `"✘"`                                           | El formato de `deleted`                  |
| `style`      | `"bold red"`                                    | El estilo del módulo.                    |
| `disabled`   | `false`                                         | Deshabilita el módulo `git_status`.      |

### Variables

The following variables can be used in `format`:

| Variable       | Descripción                                                                                              |
| -------------- | -------------------------------------------------------------------------------------------------------- |
| `all_status`   | Atajo para `$conflicted$stashed$deleted$renamed$modified$staged$untracked`                               |
| `ahead_behind` | Muestra la cadena de formato de `diverged` `ahead` o `behind` basado en el estado actual del repositorio |
| `conflicted`   | Muestra `conflicted` cuando esta rama tiene conflictos de fusión.                                        |
| `untracked`    | Muestra `untracked` cuando hay archivos sin rastrear en el directorio de trabajo.                        |
| `stashed`      | Muestra `stashed` cuando existe un archivo en el área de preparación para el repositorio local.          |
| `modified`     | Muestra `modified` cuando hay modificaciones de archivo en el directorio de trabajo.                     |
| `staged`       | Muestra `staged` cuando se ha añadido un nuevo archivo al área de preparación.                           |
| `renamed`      | Muestra `renamed` cuando un archivo renombrado ha sido añadido al área de preparación.                   |
| `deleted`      | Muestra `deleted` cuando un archivo ha sido añadido al área de preparación.                              |
| style\*      | Refleja el valor de la opción `style`                                                                    |

\*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Variable       | Descripción                                                    |
| -------------- | -------------------------------------------------------------- |
| `ahead_count`  | Número de confirmaciones por delante de la rama de seguimiento |
| `behind_count` | Número de confirmaciones detrás de la rama de seguimiento      |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

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
untracked = "🤷‍"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

The `golang` module shows the currently installed version of Golang. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `go.mod`
- El directorio actual contiene un archivo `go.sum`
- El directorio actual contiene un archivo `glide.yaml`
- El directorio actual contiene un archivo `Gopkg.yml`
- El directorio actual contiene un archivo `Gopkg.lock`
- El directorio actual contiene un archivo `.go-version`
- El directorio actual contiene un directorio `Godeps`
- El directorio actual contiene un archivo con la extensión `.go`

### Opciones

| Opción              | Por defecto                                                                    | Descripción                                            |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                           | El formato del módulo.                                 |
| `symbol`            | `"🐹 "`                                                                         | Una cadena de formato que representa el símbolo de Go. |
| `detect_extensions` | `["go"]`                                                                       | Qué extensiones deberían activar este módulo.          |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Qué nombres de archivo deberían activar este módulo.   |
| `detect_folders`    | `["Godeps"]`                                                                   | Qué carpetas deberían activar este módulo.             |
| `style`             | `"bold cyan"`                                                                  | El estilo del módulo.                                  |
| `disabled`          | `false`                                                                        | Deshabilita el módulo de `golang`.                     |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | La versión de `go`                     |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of Helm. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `helmfile.yaml`
- El directorio actual contiene un archivo `Chart.yaml`

### Opciones

| Opción              | Por defecto                          | Descripción                                              |
| ------------------- | ------------------------------------ | -------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                   |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.            |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Qué nombres de archivo deberían activar este módulo.     |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.             |
| `symbol`            | `"⎈ "`                               | Una cadena de formato que representa el símbolo de Helm. |
| `style`             | `"bold white"`                       | El estilo del módulo.                                    |
| `disabled`          | `false`                              | Deshabilita el módulo `helm`.                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | La versión de `helm`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

The `hostname` module shows the system hostname.

### Opciones

| Opción     | Por defecto                 | Descripción                                                                                                                                                            |
| ---------- | --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only` | `true`                      | Mostrar sólo el nombre de host cuando esté conectado a una sesión SSH.                                                                                                 |
| `trim_at`  | `"."`                       | Cadena en la que el nombre de host se corta, después de la primera coincidencia. `"."` se detendrá después del primer punto. `""` deshabilitará cualquier truncamiento |
| `format`   | `"[$hostname]($style) in "` | El formato del módulo.                                                                                                                                                 |
| `style`    | `"bold dimmed green"`       | El estilo del módulo.                                                                                                                                                  |
| `disabled` | `false`                     | Deshabilita el módulo `hostname`.                                                                                                                                      |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Opciones

| Opción              | Por defecto                                                                                               | Descripción                                                                             |
| ------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | El formato del módulo.                                                                  |
| `version_format`    | `v{raw}`                                                                                                  | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                      | Qué carpetas deberían activar estos módulos.                                            |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java                                         |
| `style`             | `"red dimmed"`                                                                                            | El estilo del módulo.                                                                   |
| `disabled`          | `false`                                                                                                   | Disables the `java` module.                                                             |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| version   | `v14`   | The version of `java`                  |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

::: aviso

This module is not supported on tcsh.

:::

### Opciones

| Opción      | Por defecto                   | Descripción                                      |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | Show number of jobs if exceeded.                 |
| `format`    | `"[$symbol$number]($style) "` | El formato del módulo.                           |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | El estilo del módulo.                            |
| `disabled`  | `false`                       | Disables the `jobs` module.                      |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | The number of jobs                     |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Opciones

| Opción              | Por defecto                          | Descripción                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                               |
| `detect_extensions` | `["jl"]`                             | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.         |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.    |
| `style`             | `"bold purple"`                      | El estilo del módulo.                                |
| `disabled`          | `false`                              | Disables the `julia` module.                         |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.4.0` | The version of `julia`                 |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of Kotlin. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.kt` or a `.kts` file

### Opciones

| Opción              | Por defecto                          | Descripción                                                                   |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                        |
| `detect_extensions` | `["kt", "kts"]`                      | Qué extensiones deberían activar este módulo.                                 |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                          |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                                  |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `"bold blue"`                        | El estilo del módulo.                                                         |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`                |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = "kotlinc"
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Opción            | Por defecto                                          | Descripción                                                           |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | El formato del módulo.                                                |
| `style`           | `"cyan bold"`                                        | El estilo del módulo.                                                 |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Disables the `kubernetes` module.                                     |

### Variables

| Variable  | Ejemplo              | Descripción                              |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Refleja el valor de la opción `symbol`   |
| style\* |                      | Refleja el valor de la opción `style`    |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Opciones

| Opción     | Por defecto | Descripción                                                        |
| ---------- | ----------- | ------------------------------------------------------------------ |
| `disabled` | `false`     | Disables the `line_break` module, making the prompt a single line. |

### Ejemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

The `lua` module shows the currently installed version of Lua. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Opciones

| Opción              | Por defecto                          | Descripción                                                                |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                     |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | Qué extensiones deberían activar este módulo.                              |
| `detect_files`      | `[".lua-version"]`                   | Qué nombres de archivo deberían activar este módulo.                       |
| `detect_folders`    | `["lua"]`                            | Qué carpetas deberían activar este módulo.                                 |
| `style`             | `"bold blue"`                        | El estilo del módulo.                                                      |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | The version of `lua`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Opción      | Por defecto                                     | Descripción                                              |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | El formato del módulo.                                   |
| `symbol`    | `"🐏"`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                           | El estilo del módulo.                                    |
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

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

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

The `hg_branch` module shows the active branch of the repo in your current directory.

### Opciones

| Opción              | Por defecto                      | Descripción                                                                                  |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | El estilo del módulo.                                                                        |
| `format`            | `"on [$symbol$branch]($style) "` | El formato del módulo.                                                                       |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `"…"`                            | El símbolo usado para indicar que un nombre de rama fue truncado.                            |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| branch    | `master` | The active mercurial branch            |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Opciones

| Opción              | Por defecto                          | Descripción                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module                             |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim. |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Qué extensiones deberían activar este módulo.         |
| `detect_files`      | `["nim.cfg"]`                        | Qué nombres de archivo deberían activar este módulo.  |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.            |
| `style`             | `"bold yellow"`                      | El estilo del módulo.                                 |
| `disabled`          | `false`                              | Disables the `nim` module.                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.2.0` | The version of `nimc`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### Opciones

| Opción       | Por defecto                                    | Descripción                                           |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | El formato del módulo.                                |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | El estilo del módulo.                                 |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| state     | `pure`  | The state of the nix-shell             |
| name      | `lorri` | The name of the nix-shell              |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Opciones

| Opción              | Por defecto                          | Descripción                                                                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                               |
| `version_format`    | `v{raw}`                             | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch`              |
| `symbol`            | `" "`                               | A format string representing the symbol of NodeJS.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Qué extensiones deberían activar este módulo.                                                        |
| `detect_files`      | `["package.json", ".node-version"]`  | Qué nombres de archivo deberían activar este módulo.                                                 |
| `detect_folders`    | `["node_modules"]`                   | Qué carpetas deberían activar este módulo.                                                           |
| `style`             | `"bold green"`                       | El estilo del módulo.                                                                                |
| `disabled`          | `false`                              | Disables the `nodejs` module.                                                                        |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in package.json does not match the NodeJS version. |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | The version of `node`                  |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Opciones

| Opción                    | Por defecto                                                                | Descripción                                             |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | La cadena de formato para el módulo.                    |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml. |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch. |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.  |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Qué extensiones deberían activar este módulo.           |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Qué carpetas deberían activar este módulo.              |
| `style`                   | `"bold yellow"`                                                            | El estilo del módulo.                                   |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                            |

### Variables

| Variable         | Ejemplo      | Descripción                                                       |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Refleja el valor de la opción `symbol`                            |
| style\*        |              | Refleja el valor de la opción `style`                             |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Opciones

| Opción     | Por defecto                                         | Descripción                                                    |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | El formato del módulo.                                         |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | El estilo del módulo.                                          |
| `disabled` | `false`                                             | Disables the `openstack` module.                               |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | The current OpenStack cloud            |
| project   | `dev`   | The current OpenStack project          |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present
- **helm** - The `helm` chart version is extracted from the `Chart.yaml` present
- **maven** - The `maven` package version is extracted from the `pom.xml` present
- **meson** - The `meson` package version is extracted from the `meson.build` present

> ⚠️ La versión que se muestra es la del paquete cuyo código fuente está en tu directorio actual, no en tu gestor de paquetes.

### Opciones

| Opción            | Por defecto                       | Descripción                                                |
| ----------------- | --------------------------------- | ---------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | El formato del módulo.                                     |
| `symbol`          | `"📦 "`                            | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"`                      | El estilo del módulo.                                      |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`                           | Disables the `package` module.                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.0.0` | The version of your package            |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of Perl. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Opciones

| Opción              | Por defecto                                                                                              | Descripción                                           |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | La cadena de formato para el módulo.                  |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Qué extensiones deberían activar este módulo.         |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Qué nombres de archivo deberían activar este módulo.  |
| `detect_folders`    | `[]`                                                                                                     | Qué carpetas deberían activar este módulo.            |
| `style`             | `"bold 149"`                                                                                             | El estilo del módulo.                                 |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                           |

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
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of PHP. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Opciones

| Opción              | Por defecto                          | Descripción                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP. |
| `detect_extensions` | `["php"]`                            | Qué extensiones deberían activar este módulo.         |
| `detect_files`      | `["composer.json", ".php-version"]`  | Qué nombres de archivo deberían activar este módulo.  |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.            |
| `style`             | `"147 bold"`                         | El estilo del módulo.                                 |
| `disabled`          | `false`                              | Disables the `php` module.                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v7.3.8` | The version of `php`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `spago.dhall` file
- The current directory contains a file with the `.purs` extension

### Opciones

| Opción              | Por defecto                          | Descripción                                                  |
| ------------------- | ------------------------------------ | ------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                       |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript. |
| `detect_extensions` | `["purs"]`                           | Qué extensiones deberían activar este módulo.                |
| `detect_files`      | `["spago.dhall"]`                    | Qué nombres de archivo deberían activar este módulo.         |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                   |
| `style`             | `"bold white"`                       | El estilo del módulo.                                        |
| `disabled`          | `false`                              | Disables the `purescript` module.                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | The version of `purescript`            |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

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

| Opción               | Por defecto                                                                                                  | Descripción                                                                             |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | El formato del módulo.                                                                  |
| `version_format`     | `v{raw}`                                                                                                     | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                       |
| `style`              | `"yellow bold"`                                                                                              | El estilo del módulo.                                                                   |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                         |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                         |
| `python_binary`      | `["python", "python3, "python2"]`                                                                            | Configures the python binaries that Starship should executes when getting the version.  |
| `detect_extensions`  | `["py"]`                                                                                                     | Qué extensiones deben activar este módulo                                               |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Qué nombres de archivo deben activar este módulo                                        |
| `detect_folders`     | `[]`                                                                                                         | Qué carpetas deben activar este módulo                                                  |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                           |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Variables

| Variable     | Ejemplo         | Descripción                                |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Refleja el valor de la opción `symbol`     |
| style        | `"yellow bold"` | Refleja el valor de la opción `style`      |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |

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
# Only use the `python3` binary to get the version.
python_binary = "python3"
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

## Red

By default the `red` module shows the currently installed version of Red. The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.red` or `.reds` extension

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `v{raw}`                             | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                                         |
| `detect_extensions` | `["red"]`                            | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"red bold"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `red` module.                                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `red`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of Ruby. The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `v{raw}`                             | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                                        |
| `detect_extensions` | `["rb"]`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold red"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `ruby` module.                                                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | The version of `ruby`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of Rust. The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Opciones

| Opción              | Por defecto                          | Descripción                                                                             |
| ------------------- | ------------------------------------ | --------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                  |
| `version_format`    | `v{raw}`                             | El formato de versión. Las variables disponibles son `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                                         |
| `detect_extensions` | `["rs"]`                             | Qué extensiones deberían activar este módulo.                                           |
| `detect_files`      | `["Cargo.toml"]`                     | Qué nombres de archivo deberían activar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                                              |
| `style`             | `"bold red"`                         | El estilo del módulo.                                                                   |
| `disabled`          | `false`                              | Disables the `rust` module.                                                             |

### Variables

| Variable  | Ejemplo           | Descripción                            |
| --------- | ----------------- | -------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                 |
| symbol    |                   | Refleja el valor de la opción `symbol` |
| style\* |                   | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of Scala. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Opciones

| Opción              | Por defecto                              | Descripción                                          |
| ------------------- | ---------------------------------------- | ---------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | El formato del módulo.                               |
| `detect_extensions` | `["sbt", "scala"]`                       | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[".metals"]`                            | Qué carpetas deberían activar estos módulos.         |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.    |
| `style`             | `"red dimmed"`                           | El estilo del módulo.                                |
| `disabled`          | `false`                                  | Disables the `scala` module.                         |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `2.13.5` | The version of `scala`                 |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Opción                 | Por defecto  | Descripción                                   |
| ---------------------- | ------------ | --------------------------------------------- |
| `bash_indicator`       | `bsh`        | A format string used to represent bash.       |
| `fish_indicator`       | `fsh`        | A format string used to represent fish.       |
| `zsh_indicator`        | `zsh`        | A format string used to represent zsh.        |
| `powershell_indicator` | `psh`        | A format string used to represent powershell. |
| `ion_indicator`        | `ion`        | A format string used to represent ion.        |
| `elvish_indicator`     | `esh`        | A format string used to represent elvish.     |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.       |
| `format`               | `$indicator` | El formato del módulo.                        |
| `disabled`             | `true`       | Disables the `shell` module.                  |

### Variables

| Variable  | Por defecto | Descripción                                                |
| --------- | ----------- | ---------------------------------------------------------- |
| indicator |             | Mirrors the value of `indicator` for currently used shell. |

### Ejemplos

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
disabled = false
```

## SHLVL

The `shlvl` module shows the current `SHLVL` ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Opciones

| Opción      | Por defecto                  | Descripción                                                   |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | El formato del módulo.                                        |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | El estilo del módulo.                                         |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | The current value of `SHLVL`           |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### Opciones

| Opción     | Por defecto                      | Descripción                                      |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | El formato del módulo.                           |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | El estilo del módulo.                            |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

| Variable  | Ejemplo      | Descripción                            |
| --------- | ------------ | -------------------------------------- |
| env       | `centos.img` | The current singularity image          |
| symbol    |              | Refleja el valor de la opción `symbol` |
| style\* |              | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

::: warning This module is not supported on elvish shell. :::

### Opciones

| Opción                  | Por defecto                   | Descripción                                          |
| ----------------------- | ----------------------------- | ---------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "` | The format of the module                             |
| `symbol`                | `"✖"`                         | The symbol displayed on program error                |
| `not_executable_symbol` | `"🚫"`                         | The symbol displayed when file isn't executable      |
| `not_found_symbol`      | `"🔍"`                         | The symbol displayed when the command can't be found |
| `sigint_symbol`         | `"🧱"`                         | The symbol displayed on SIGINT (Ctrl + c)            |
| `signal_symbol`         | `"⚡"`                         | The symbol displayed on any signal                   |
| `style`                 | `"bold red"`                  | El estilo del módulo.                                |
| `recognize_signal_code` | `true`                        | Enable signal mapping from exit code                 |
| `map_symbol`            | `false`                       | Enable symbols mapping from exit code                |
| `disabled`              | `true`                        | Disables the `status` module.                        |

### Variables

| Variable       | Ejemplo | Descripción                                                          |
| -------------- | ------- | -------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                    |
| int            | `127`   | The exit code of the last command                                    |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                  |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled      |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found         |
| symbol         |         | Refleja el valor de la opción `symbol`                               |
| style\*      |         | Refleja el valor de la opción `style`                                |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false

```

## Swift

By default the `swift` module shows the currently installed version of Swift. The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Opciones

| Opción              | Por defecto                          | Descripción                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                               |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift     |
| `detect_extensions` | `["swift"]`                          | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `["Package.swift"]`                  | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.           |
| `style`             | `"bold 202"`                         | El estilo del módulo.                                |
| `disabled`          | `false`                              | Disables the `swift` module.                         |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.2.4` | The version of `swift`                 |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version.

::: tip

By default the terraform version is not shown, since this is slow for current versions of terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-version).

:::

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

### Opciones

| Opción              | Por defecto                          | Descripción                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | La cadena de formato para el módulo.                  |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace. |
| `detect_extensions` | `["tf", "hcl"]`                      | Qué extensiones deberían activar este módulo.         |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.  |
| `detect_folders`    | `[".terraform"]`                     | Qué carpetas deberían activar este módulo.            |
| `style`             | `"bold 105"`                         | El estilo del módulo.                                 |
| `disabled`          | `false`                              | Disables the `terraform` module.                      |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `terraform`             |
| workspace | `default`  | The current terraform workspace        |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

#### Con la versión

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Sin la versión

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Hora

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Opción            | Por defecto             | Descripción                                                                                                                        |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | La cadena de formato para el módulo.                                                                                               |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable  | Ejemplo    | Descripción                           |
| --------- | ---------- | ------------------------------------- |
| time      | `13:08:10` | The current time.                     |
| style\* |            | Refleja el valor de la opción `style` |

\*: This variable can only be used as a part of a style string

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

## Username

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Opciones

| Opción        | Por defecto             | Descripción                           |
| ------------- | ----------------------- | ------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root. |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.    |
| `format`      | `"[$user]($style) in "` | El formato del módulo.                |
| `show_always` | `false`                 | Always shows the `username` module.   |
| `disabled`    | `false`                 | Disables the `username` module.       |

### Variables

| Variable | Ejemplo      | Descripción                                                                                 |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `"matchai"`  | The currently logged-in user ID.                                                            |

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

The `vagrant` module shows the currently installed version of Vagrant. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- The current directory contains a `Vagrantfile` file

### Opciones

| Opción              | Por defecto                          | Descripción                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                               |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.  |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `["Vagrantfile"]`                    | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.           |
| `style`             | `"cyan bold"`                        | El estilo del módulo.                                |
| `disabled`          | `false`                              | Disables the `vagrant` module.                       |

### Variables

| Variable  | Ejemplo          | Descripción                            |
| --------- | ---------------- | -------------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`               |
| symbol    |                  | Refleja el valor de la opción `symbol` |
| style\* |                  | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## VCSH

The `vcsh` module displays the current active VCSH repository. The module will be shown only if a repository is currently in use.

### Opciones

| Opción     | Por defecto                      | Descripción                                            |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   |                                  | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | El estilo del módulo.                                  |
| `format`   | `"vcsh [$symbol$repo]($style) "` | El formato del módulo.                                 |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variables

| Variable  | Ejemplo                                     | Descripción                            |
| --------- | ------------------------------------------- | -------------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name             |
| symbol    |                                             | Refleja el valor de la opción `symbol` |
| style\* | `black bold dimmed`                         | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of Zig. The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### Opciones

| Opción              | Por defecto                          | Descripción                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig. |
| `style`             | `"bold yellow"`                      | El estilo del módulo.                                 |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                |
| `disabled`          | `false`                              | Disables the `zig` module.                            |
| `detect_extensions` | `["zig"]`                            | Qué extensiones deberían activar este módulo.         |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.  |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.6.0` | The version of `zig`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `files`
- The current directory contains a directory whose name is in `directories`
- The current directory contains a file whose extension is in `extensions`
- The `when` command returns 0

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

### Opciones

| Opción        | Por defecto                     | Descripción                                                                                                                |
| ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                  | El estilo del módulo.                                                                                                      |
| `format`      | `"[$symbol($output )]($style)"` | El formato del módulo.                                                                                                     |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                             |

### Variables

| Variable  | Descripción                            |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Refleja el valor de la opción `symbol` |
| style\* | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

#### Comando personalizado del intérprete de comandos

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ["pwsh", "-Command", "-"]
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
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
