# Configuración

Para comenzar a configurar starship, crea el siguiente archivo: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toda la configuración de starship se realiza en este archivo [TOML](https://github.com/toml-lang/toml):
```toml
# Evita imprimir una nueva linea al inicio del prompt
add_newline = false

# Reemplaza el símbolo "❯" por "➜" del prompt
[character]       # El nombre del módulo que se está configurando es "character"
symbol = "➜"     # El segmento "symbol" es reemplazado por "➜"
success_symbol = "[➜](bold green)"     # El segmento "success_symbol" es reemplazado por "➜" con el color "bold green"

# Desactiva el gestor de paquetes, ocultándolo por completo del prompt
[package]
disabled = true
```

Puedes cambiar la ubicación predeterminada del archivo `starship.toml` con la variable de entorno `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/.starship
```

El equivalente en PowerShell (Windows) es añadir esta línea a tu `$PROFILE`:

```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### Terminología

**Módulo**: Un componente en el prompt que da información basada en información contextual de tu sistema operativo. Por ejemplo, el módulo "nodejs" muestra la versión de NodeJS que se tiene actualmente instalada, si el directorio actual es un proyecto NodeJS.

**Variable**: subcomponentes más pequeños que contienen información proporcionada por el módulo. Por ejemplo, la variable "version" en el módulo "nodejs" contiene la versión actual de NodeJS.

Por convención, la mayoría de los módulos tienen un prefijo del color predeterminado de la terminal (por ejemplo, `vía` en "nodejs") y un espacio vacío como sufijo.

### Formato de cadenas de texto

Es el formato con el que un módulo imprime todas sus variables. La mayoría de los módulos tienen una entrada llamada `format` que configura el formato de visualización del módulo. Se puede utilizar textos, variables y grupos de texto.

#### Variable

Una variable contiene un símbolo `$` seguido por el nombre de la variable. El nombre de una variable solo contiene letras, números y `_`.

Por ejemplo:

- `$version` es un formato de cadena de texto con una variable llamada `version`.
- `$git_branch$git_commit` es un formato de cadena de texto con dos variables nombradas `git_branch` y `git_commit`.
- `$git_branch $git_commit` tiene las dos variables separadas por un espacio.

#### Grupo de texto

Un grupo de texto se compone de dos partes diferentes.

La primera parte, que está encerrada en un `[]`, es un [formato de cadena de texto](#format-strings). Se puede agregar textos, variables, o incluso grupos de texto anidados.

En la segunda parte, que está encerrada en un `()`, es un [formato de cadena de texto](#style-strings). Se puede utilizar el estilo de la primera parte.

Por ejemplo:

- `[on](bold red)` imprimirá una cadena `en` con texto en negrita color rojo.
- `[⬢ $version](bold green)` imprimirá un símbolo `⬢` seguido por el contenido de la variable `version`, con texto en negrita color verde.
- `[a [b](red) c](green)` imprimirá `a b c` con `b` rojo. y `a` y `c` verde.

#### Cadenas de estilo

La mayoría de los módulos de starship permiten configurar sus estilos de visualización. Esto se consigue con una entrada (normalmente llamada `style`) que no es más que un texto donde se especifica la configuración. Aquí hay algunos ejemplos de cadenas de estilo junto con lo que hacen. Para más detalles sobre la sintaxis completa, consultar [la guía de configuración avanzada](/advanced-config/).

- `"fg:green bg:blue"` pone texto verde sobre un fondo azul
- `"bg:blue fg:bright-green"` pone texto verde claro sobre un fondo azul
- `"bold fg:27"` pone texto en negrita con [color ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` subraya el texto sobre un fondo naranja oscuro
- `"bold italic fg:purple"` pone texto color morado, en negrita y cursiva
- `""` desactiva explícitamente cualquier estilo

Nótese que el estilo es similar a como se controlaría por el emulador de terminal. Por ejemplo, algunos emuladores de terminal harán los colores más brillantes en lugar de más gruesos, y algunos temas de colores usan los mismos valores para texto normal y colores brillantes. Además, para mostrar textos en cursiva tu terminal debe tener soporte para hacerlo.

#### Cadenas de formato condicionales

Una cadena de formato condicional envuelto en `(` y `)` no se renderizará si todas las variables dentro están vacías.

Por ejemplo:

- `(@$region)` no mostrará nada si la variable `region` es `None`, de lo contrario `@` seguido por el valor de la región.
- `(algún texto)` siempre mostrará nada ya que no hay variables envueltas entre llaves.
- Cuando `$all` es un atajo de teclado para `\[$a$b\]`, `($all)` no mostrará nada solo si `$a` y `$b` ambos son `None`. Esto funciona igual que `(\[$a$b\] )`.

#### Carácteres de escape

Los siguientes símbolos tienen un uso especial en una cadena de formato. Si se quiere imprimir los siguientes símbolos, se tienen que escapar con una barra invertida (`\`).

- $
- \\
- [
- ]
- (
- )

Se debe tener en cuenta que `toml` tiene [su propia sintaxis de escape](https://github.com/toml-lang/toml#user-content-string). Se recomienda usar una cadena literal (`''`) en la configuración. Si se desea utilizar una cadena básica (`""`), prestar atención para escapar la barra inversa `\`.

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

## Prompt

Esta es la lista de opciones de configuración del prompt.

### Opciones

| Opción         | Por defecto                    | Descripción                                                                   |
| -------------- | ------------------------------ | ----------------------------------------------------------------------------- |
| `format`       | [link](#default-prompt-format) | Configura el formato del prompt.                                              |
| `scan_timeout` | `30`                           | Tiempo de espera tras el que starship escanea los archivos (en milisegundos). |
| `add_newline`  | `true`                         | Añade una nueva línea antes del prompt.                                       |

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

# Desactiva la nueva línea al inicio del prompt
add_newline = false
```

### Formato predeterminado del Prompt

El `format` predeterminado se utiliza para definir el formato del prompt, si está vacío o no `format` se proporciona. El valor predeterminado es el siguiente:

```toml
format = "$all"

# Que es es equivalente a
format = """

$username\
$hostname\
$shlvl\
$kubernetes\
$directory\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
$ruby\
$rust\
$swift\
$terraform\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$env_var\
$crystal\
$cmd_duration\
$custom\
$line_break\
$jobs\
$battery\
$time\
$character"""
```

## AWS

El módulo `aws` muestra la región y el perfil AWS actuales. Éste se basa en las variables de entorno `AWS_REGION`, `AWS_DEFAULT_REGION`, y `AWS_PROFILE` del fichero `~/.aws/config`.

Cuando se utiliza [aws-vault](https://github.com/99designs/aws-vault), el perfil se lee de la variable de entorno `AWS_VAULT`.

### Opciones

| Opción           | Por defecto                                          | Descripción                                                  |
| ---------------- | ---------------------------------------------------- | ------------------------------------------------------------ |
| `format`         | `"on [$symbol$profile(\\($region\\))]($style) "` | El formato del módulo.                                       |
| `symbol`         | `"☁️ "`                                              | El símbolo que se muestra antes del perfil de AWS.           |
| `region_aliases` |                                                      | Tabla de alias de región para mostrar además del nombre AWS. |
| `style`          | `"bold yellow"`                                      | El estilo del módulo.                                        |
| `disabled`       | `false`                                              | Desactiva el módulo AWS.                                     |

### Variables

| Variable  | Ejemplo          | Descripción                            |
| --------- | ---------------- | -------------------------------------- |
| region    | `ap-northeast-1` | La región actual de AWS                |
| profile   | `astronauts`     | El perfil actual de AWS                |
| symbol    |                  | Refleja el valor de la opción `symbol` |
| style\* |                  | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplos

#### Mostrar todo

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile(\\($region\\))]($style) "
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
```

## Battery

El módulo `battery` muestra qué tan cargada está la batería del dispositivo y su estado de carga actual. El módulo sólo es visible cuando la batería del dispositivo está por debajo del 10%.

### Opciones

| Opción               | Por defecto                       | Descripción                                       |
| -------------------- | --------------------------------- | ------------------------------------------------- |
| `full_symbol`        | `"•"`                             | Se muestra cuando la batería está cargada.        |
| `charging_symbol`    | `"⇡"`                             | Se muestra cuando la batería está cargando.       |
| `discharging_symbol` | `"⇣"`                             | Se muestra cuando la batería se está descargando. |
| `format`             | `"[$symbol$percentage]($style) "` | El formato del módulo.                            |
| `display`            | [link](#battery-display)          | Define cuándo mostrar el indicador y el estilo.   |
| `disabled`           | `false`                           | Desactiva el módulo `battery`.                    |

<details>
<summary>También hay opciones para algunos estados de batería poco comunes.</summary>

| Variable         | Descripción                                         |
| ---------------- | --------------------------------------------------- |
| `unknown_symbol` | The symbol shown when the battery state is unknown. |
| `empty_symbol`   | The symbol shown when the battery state is empty.   |

Note: Battery indicator will be hidden if the status is `unknown` or `empty` unless you specify the option in the config.

</details>

### Ejemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Indicador de batería

La configuración de la opción `display` es usada para definir cuándo se debe mostrar el indicador de batería (umbral) y cómo debe mostrarse (estilo). Si `display` no se proporciona. El valor predeterminado es el siguiente:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Opciones

La opción `display` es un array de la siguiente tabla.

| Variable    | Descripción                                                     |
| ----------- | --------------------------------------------------------------- |
| `threshold` | El umbral para la opción de visualización.                      |
| `style`     | El estilo usado cuando si la opción <0>display</0> está activa. |

#### Ejemplo

```toml
[[battery.display]]  # "bold red" cuando la carga está entre 0% y 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" cuando la carga está entre 10% y 30%
threshold = 30
style = "bold yellow"

# cuando la carga está por encima del 30% el indicador no se mostrará

```

## Character

El módulo `character` muestra un caracter (normalmente una flecha) tras el texto que introduces en el terminal.

El carácter mostrará si el último comando funcionó o no. Se puede hacer de dos maneras:

- cambiando el color (`red`/`green`)
- cambiando la forma (`.`/`✖`)

Por defecto sólo cambia el color. Si también se quiere cambiar su forma, ver [este ejemplo](#with-custom-error-shape).

### Opciones

| Opción           | Por defecto         | Descripción                                                                                |
| ---------------- | ------------------- | ------------------------------------------------------------------------------------------ |
| `format`         | `"$symbol "`        | La cadena de formato usada antes de la entrada de texto.                                   |
| `success_symbol` | `"[❯](bold green)"` | La cadena de formato usada antes de la entrada de texto si el comando anterior tuvo éxito. |
| `error_symbol`   | `"[❯](bold red)"`   | La cadena de formato usada antes de la entrada de texto si el comando anterior falló.      |
| `vicmd_symbol`   | `"[❮](bold green)"` | El cadena de formato antes de la entrada de texto si la shell está en modo vim normal.     |
| `disabled`       | `false`             | Desactiva el módulo `character`.                                                           |

### Variables

| Variable | Ejemplo | Descripción                                                    |
| -------- | ------- | -------------------------------------------------------------- |
| symbol   |         | Un espejo de `success_symbol`, `error_symbol` o `vicmd_symbol` |

### Ejemplos

#### Con forma de error personalizada

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Sin forma de error personalizada

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### Con forma vim personalizada

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

El módulo `cmake` muestra la versión actualmente instalada de CMake si:

- El directorio actual contiene un archivo `CMakeLists.txt`

### Opciones

| Opción     | Por defecto                        | Descripción                                    |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                         |
| `symbol`   | `"🛆 "`                             | El símbolo usado antes de la versión de cmake. |
| `style`    | `"bold blue"`                      | El estilo del módulo.                          |
| `disabled` | `false`                            | Deshabilita el módulo `cmake`.                 |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | La versión de cmake                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

## Tiempo de ejecución

El módulo `cmd_duration` muestra cuánto tiempo tomó ejecutarse el último comando. El módulo se mostrará solo si el comando tomó más de dos segundos, o si el valor de configuración de `min_time` existe.

::: advertencia No utilizar la trampa DEBUG en Bash

Si se está usando Starship con `bash`, no usar `DEBUG` después de ejecutar `eval $(starship init $0)`, o el módulo **se romperá**.

:::

Los usuarios de bash que necesiten una funcionalidad como preexec pueden usar el [framework rcaloras's bash_preexec](https://github.com/rcaloras/bash-preexec). Simplemente definir los arreglos `preexec_functions` y `precmd_functions` antes de ejecutar `eval $(starship init $0)`, y luego proceder de forma normal.

### Opciones

| Opción              | Por defecto                   | Descripción                                                           |
| ------------------- | ----------------------------- | --------------------------------------------------------------------- |
| `min_time`          | `2_000`                       | Duración mínima para mostrar el tiempo de ejecución (en milisegundos) |
| `show_milliseconds` | `false`                       | Muestra la duración con precisión en milisegundos.                    |
| `format`            | `"took [$duration]($style) "` | El formato del módulo.                                                |
| `style`             | `"bold yellow"`               | El estilo del módulo.                                                 |
| `disabled`          | `false`                       | Desactiva el módulo `cmd_duration`.                                   |

### Variables

| Variable  | Ejemplo  | Descripción                                |
| --------- | -------- | ------------------------------------------ |
| duration  | `16m40s` | El tiempo que tardó en ejecutar el comando |
| style\* |          | Refleja el valor de la opción `style`      |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

El módulo `conda` muestra el entorno conda actual, si `$CONDA_DEFAULT_ENV` está configurado.

::: tip

Esto no modifica el propio prompt de conda. En caso de querer suprimirlo, ejecutar `conda config --set changeps1 False`.

:::

### Opciones

| Opción              | Por defecto                        | Descripción                                                                                                                                                                                                             |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                | El número de directorios a los que se debe truncar la variable de entorno, si el entorno fue creado usando `conda create -p [path]`. `0` significa sin truncamiento. Mirar también el módulo [`directory`](#directory). |
| `symbol`            | `"🅒 "`                             | El símbolo usado antes del nombre del entorno.                                                                                                                                                                          |
| `style`             | `"bold green"`                     | El estilo del módulo.                                                                                                                                                                                                   |
| `format`            | `"[$symbol$environment]($style) "` | El formato del módulo.                                                                                                                                                                                                  |
| `ignore_base`       | `true`                             | Ignora el entorno `base` cuando se activa.                                                                                                                                                                              |
| `disabled`          | `false`                            | Deshabilita el módulo `conda`.                                                                                                                                                                                          |

### Variables

| Variable    | Ejemplo      | Descripción                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | El entorno conda actual                |
| symbol      |              | Refleja el valor de la opción `symbol` |
| style\*   |              | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

El módulo `crystal` muestra la versión actual de Crystal. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `shard.yml`
- El directorio actual contiene un fichero `.cr`

### Opciones

| Opción     | Por defecto                        | Descripción                                   |
| ---------- | ---------------------------------- | --------------------------------------------- |
| `symbol`   | `"🔮 "`                             | Símbolo usado antes de la versión de Crystal. |
| `style`    | `"bold red"`                       | El estilo del módulo.                         |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                        |
| `disabled` | `false`                            | Desactiva el módulo `crystal`.                |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | La versión de `crystal`                |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

El módulo `dart` muestra la versión actualmente instalada de Dart. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo con la extensión `.dart`
- El directorio actual contiene un directorio `.dart_tool`
- El directorio actual contiene un archivo `pubspec.yaml` o `pubspec.lock`

### Opciones

| Variable   | Por defecto                        | Descripción                                             |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                  |
| `symbol`   | `"🎯 "`                             | Una cadena de formato que representa el símbolo de Dart |
| `style`    | `"bold blue"`                      | El estilo del módulo.                                   |
| `disabled` | `false`                            | Deshabilita el módulo `dart`.                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.8.4` | La versión de `dart`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Directory

El módulo `directory` muestra la ruta hasta el directorio actual, mostrando tres directorios padre como máximo. El directorio se truncará a la raíz del repositorio git en el que se esté.

Al usarse la opción pwd del estilo fish, en lugar de ocultar la ruta truncada, se mostrará un nombre abreviado de cada directorio según el número habilitado para la opción.

Por ejemplo, dado `~/Dev/Nix/nixpkgs/pkgs` donde `nixpkgs` es la raíz del repositorio y el valor de la opción es `1`. Ahora se mostrará `~/D/N/nixpkgs/pkgs`, mientras que antes habría sido `nixpkgs/pkgs`.

### Opciones

| Variable                 | Por defecto                                     | Descripción                                                           |
| ------------------------ | ----------------------------------------------- | --------------------------------------------------------------------- |
| `truncation_length`      | `3`                                             | El número de carpetas a las que se debe truncar el directorio actual. |
| `truncate_to_repo`       | `true`                                          | Truncar o no hasta la raíz del repositorio git en el que se esté.     |
| `format`                 | `"[$path]($style)[$lock_symbol]($lock_style) "` | El formato del módulo.                                                |
| `style`                  | `"bold cyan"`                                   | El estilo del módulo.                                                 |
| `disabled`               | `false`                                         | Deshabilita el módulo `directory`.                                    |
| `read_only_symbol`       | `"🔒"`                                           | El símbolo que indica si el directorio actual es de sólo lectura.     |
| `read_only_symbol_style` | `"red"`                                         | El estilo para el símbolo de sólo lectura.                            |

<details>
<summary>Este módulo tiene algunas opciones avanzadas de configuración que controlan cómo se muestra el directorio.</summary>

| Advanced Option             | Por defecto | Descripción                                                                              |
| --------------------------- | ----------- | ---------------------------------------------------------------------------------------- |
| `substitutions`             |             | A table of substitutions to be made to the path.                                         |
| `fish_style_pwd_dir_length` | `0`         | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`      | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

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
| path      | `"D:/Projects"`       | La ruta de directorio actual          |
| style\* | `"black bold dimmed"` | Refleja el valor de la opción `style` |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

El módulo `docker_context` muestra el [contexto de Docker](https://docs.docker.com/engine/context/working-with-contexts/) actualmente activo si no está establecido en `default`.

### Opciones

| Opción            | Por defecto                        | Descripción                                                                             |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | El formato del módulo.                                                                  |
| `symbol`          | `"🐳 "`                             | El símbolo usado antes de mostrar el contexto de Docker.                                |
| `style`           | `"blue bold"`                      | El estilo del módulo.                                                                   |
| `only_with_files` | `false`                            | Mostrar solo cuando hay un `docker-compose.yml` o `Dockerfile` en el directorio actual. |
| `disabled`        | `true`                             | Deshabilita el módulo `docker_context`.                                                 |

### Variables

| Variable  | Ejemplo        | Descripción                            |
| --------- | -------------- | -------------------------------------- |
| context   | `test_context` | El contexto actual de docker           |
| symbol    |                | Refleja el valor de la opción `symbol` |
| style\* |                | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one or more of the following files are present in the current directory:

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

| Opción      | Por defecto                              | Descripción                                              |
| ----------- | ---------------------------------------- | -------------------------------------------------------- |
| `format`    | `"v[$symbol$version( 🎯 $tfm)]($style) "` | El formato del módulo.                                   |
| `symbol`    | `"•NET "`                                | The symbol used before displaying the version of dotnet. |
| `heuristic` | `true`                                   | Use faster version detection to keep starship snappy.    |
| `style`     | `"bold blue"`                            | El estilo del módulo.                                    |
| `disabled`  | `false`                                  | Disables the `dotnet` module.                            |

### Variables

| Variable  | Ejemplo          | Descripción                                                        |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Refleja el valor de la opción `symbol`                             |
| style\* |                  | Refleja el valor de la opción `style`                              |

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

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `mix.exs` file.

### Opciones

| Opción     | Por defecto                                                   | Descripción                                                     |
| ---------- | ------------------------------------------------------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                        | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"`                                               | El estilo del módulo.                                           |
| `format`   | `"via [$symbol$version \\(OTP $otp_version\\)]($style) "` | The format for the module elixir.                               |
| `disabled` | `false`                                                       | Disables the `elixir` module.                                   |

### Variables

| Variable    | Ejemplo | Descripción                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | The version of `elixir`                |
| otp_version |         | The otp version of `elixir`            |
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

The `elm` module shows the currently installed version of Elm. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### Opciones

| Opción     | Por defecto                        | Descripción                                     |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                          |
| `symbol`   | `"🌳 "`                             | A format string representing the symbol of Elm. |
| `style`    | `"cyan bold"`                      | El estilo del módulo.                           |
| `disabled` | `false`                            | Disables the `elm` module.                      |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.19.1` | The version of `elm`                   |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### Opciones

| Opción     | Por defecto                    | Descripción                                                                  |
| ---------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`   |                                | The symbol used before displaying the variable value.                        |
| `variable` |                                | The environment variable to be displayed.                                    |
| `default`  |                                | The default value to be displayed when the selected variable is not defined. |
| `format`   | `"with [$env_value]($style) "` | El formato del módulo.                                                       |
| `disabled` | `false`                        | Disables the `env_var` module.                                               |

### Variables

| Variable  | Ejemplo                                     | Descripción                                |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if *variable* would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Refleja el valor de la opción `symbol`     |
| style\* | `black bold dimmed`                         | Refleja el valor de la opción `style`      |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

### Opciones

| Opción     | Por defecto                        | Descripción                                              |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`                             | The symbol used before displaying the version of erlang. |
| `style`    | `"bold red"`                       | El estilo del módulo.                                    |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                   |
| `disabled` | `false`                            | Disables the `erlang` module.                            |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | The version of `erlang`                |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Gcloud

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Opciones

| Variable         | Por defecto                                          | Descripción                                                     |
| ---------------- | ---------------------------------------------------- | --------------------------------------------------------------- |
| `format`         | `"on [$symbol$account(\\($region\\))]($style) "` | El formato del módulo.                                          |
| `symbol`         | `"☁️ "`                                              | The symbol used before displaying the current GCP profile.      |
| `region_aliases` |                                                      | Table of region aliases to display in addition to the GCP name. |
| `style`          | `"bold blue"`                                        | El estilo del módulo.                                           |
| `disabled`       | `false`                                              | Disables the `gcloud` module.                                   |

### Variables

| Variable  | Ejemplo           | Descripción                                                        |
| --------- | ----------------- | ------------------------------------------------------------------ |
| region    | `us-central1`     | The current GCP region                                             |
| account   | `foo@example.com` | The current GCP profile                                            |
| project   |                   | The current GCP project                                            |
| active    | `default`         | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |                   | Refleja el valor de la opción `symbol`                             |
| style\* |                   | Refleja el valor de la opción `style`                              |

\*: This variable can only be used as a part of a style string

### Ejemplos

#### Display account and project

```toml
# ~/.config/starship.toml

[gcloud]
format = "on [$symbol$account(\\($project\\))]($style) "
```

#### Display active config name only

```toml
# ~/.config/starship.toml

[gcloud]
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Display account and aliased region

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

| Opción              | Por defecto                      | Descripción                                                                              |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `format`            | `"on [$symbol$branch]($style) "` | El formato del módulo.  Use `"$branch"` to refer to the current branch name.             |
| `symbol`            | `" "`                           | A format string representing the symbol of git branch.                                   |
| `style`             | `"bold purple"`                  | El estilo del módulo.                                                                    |
| `truncation_length` | `2^63 - 1`                       | Truncates a git branch to X graphemes.                                                   |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated. You can use `""` for no symbol. |
| `disabled`          | `false`                          | Disables the `git_branch` module.                                                        |

### Variables

| Variable  | Ejemplo  | Descripción                                                                                          |
| --------- | -------- | ---------------------------------------------------------------------------------------------------- |
| branch    | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached HEAD). |
| symbol    |          | Refleja el valor de la opción `symbol`                                                               |
| style\* |          | Refleja el valor de la opción `style`                                                                |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash of the repo in your current directory.

### Opciones

| Opción               | Por defecto                    | Descripción                                           |
| -------------------- | ------------------------------ | ----------------------------------------------------- |
| `commit_hash_length` | `7`                            | The length of the displayed git commit hash.          |
| `format`             | `"[\\($hash\\)]($style) "` | El formato del módulo.                                |
| `style`              | `"bold green"`                 | El estilo del módulo.                                 |
| `only_detached`      | `true`                         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`                        | Disables the `git_commit` module.                     |

### Variables

| Variable  | Ejemplo   | Descripción                           |
| --------- | --------- | ------------------------------------- |
| hash      | `b703eb3` | The current git commit hash           |
| style\* |           | Refleja el valor de la opción `style` |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Opciones

| Opción         | Por defecto                                                         | Descripción                                                                             |
| -------------- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                        | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                         | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                       | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                                  | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                       | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                              | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                       | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                     | El estilo del módulo.                                                                   |
| `format`       | `"[\\($state( $progress_current/$progress_total)\\)]($style) "` | El formato del módulo.                                                                  |
| `disabled`     | `false`                                                             | Disables the `git_state` module.                                                        |

### Variables

| Variable         | Ejemplo    | Descripción                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | The current state of the repo         |
| progress_current | `1`        | The current operation progress        |
| progress_total   | `2`        | The total operation progress          |
| style\*        |            | Refleja el valor de la opción `style` |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[git_state]
format = "[\\($state( $progress_current of $progress_total)\\)]($style) "
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Opciones

| Opción            | Por defecto                                     | Descripción                                          |
| ----------------- | ----------------------------------------------- | ---------------------------------------------------- |
| `format`          | "([\[$all_status$ahead_behind\]]($style) )" | The default format for `git_status`                  |
| `conflicted`      | `"="`                                           | This branch has merge conflicts.                     |
| `ahead`           | `"⇡"`                                           | The format of `ahead`                                |
| `behind`          | `"⇣"`                                           | The format of `behind`                               |
| `diverged`        | `"⇕"`                                           | The format of `diverged`                             |
| `untracked`       | `"?"`                                           | The format of `untracked`                            |
| `stashed`         | `"$"`                                           | The format of `stashed`                              |
| `modified`        | `"!"`                                           | The format of `modified`                             |
| `staged`          | `"+"`                                           | The format of `staged`                               |
| `renamed`         | `"»"`                                           | The format of `renamed`                              |
| `deleted`         | `"✘"`                                           | The format of `deleted`                              |
| `show_sync_count` | `false`                                         | Show ahead/behind count of the branch being tracked. |
| `style`           | `"bold red"`                                    | El estilo del módulo.                                |
| `disabled`        | `false`                                         | Disables the `git_status` module.                    |

### Variables

The following variables can be used in `format`:

| Variable       | Descripción                                                                                   |
| -------------- | --------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                   |
| `ahead_behind` | Displays `diverged` `ahead` or `behind` format string based on the current status of the repo |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                   |
| `untracked`    | Displays `untracked`  when there are untracked files in the working directory.                |
| `stashed`      | Displays `stashed`    when a stash exists for the local repository.                           |
| `modified`     | Displays `modified`   when there are file modifications in the working directory.             |
| `staged`       | Displays `staged`     when a new file has been added to the staging area.                     |
| `renamed`      | Displays `renamed`    when a renamed file has been added to the staging area.                 |
| `deleted`      | Displays `deleted`    when a file's deletion has been added to the staging area.              |
| style\*      | Refleja el valor de la opción `style`                                                         |

\*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Variable       | Descripción                                    |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Variable | Descripción              |
| -------- | ------------------------ |
| `count`  | Show the number of files |

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

## Golang

The `golang` module shows the currently installed version of Golang. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Opciones

| Opción     | Por defecto                        | Descripción                                    |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                         |
| `symbol`   | `"🐹 "`                             | A format string representing the symbol of Go. |
| `style`    | `"bold cyan"`                      | El estilo del módulo.                          |
| `disabled` | `false`                            | Disables the `golang` module.                  |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | The version of `go`                    |
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

The `helm` module shows the currently installed version of Helm. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Opciones

| Opción     | Por defecto                        | Descripción                                      |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                           |
| `symbol`   | `"⎈ "`                             | A format string representing the symbol of Helm. |
| `style`    | `"bold white"`                     | El estilo del módulo.                            |
| `disabled` | `false`                            | Disables the `helm` module.                      |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | The version of `helm`                  |
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

| Opción     | Por defecto                 | Descripción                                                                                                                          |
| ---------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                      | Only show hostname when connected to an SSH session.                                                                                 |
| `trim_at`  | `"."`                       | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `format`   | `"on [$hostname]($style) "` | El formato del módulo.                                                                                                               |
| `style`    | `"bold dimmed green"`       | El estilo del módulo.                                                                                                                |
| `disabled` | `false`                     | Disables the `hostname` module.                                                                                                      |

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

The `java` module shows the currently installed version of Java. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### Opciones

| Opción     | Por defecto                            | Descripción                                     |
| ---------- | -------------------------------------- | ----------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | El formato del módulo.                          |
| `symbol`   | `"☕ "`                                 | A format string representing the symbol of Java |
| `style`    | `"red dimmed"`                         | El estilo del módulo.                           |
| `disabled` | `false`                                | Disables the `java` module.                     |

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

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

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

The `julia` module shows the currently installed version of Julia. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Opciones

| Opción     | Por defecto                        | Descripción                                       |
| ---------- | ---------------------------------- | ------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                            |
| `symbol`   | `"ஃ "`                             | A format string representing the symbol of Julia. |
| `style`    | `"bold purple"`                    | El estilo del módulo.                             |
| `disabled` | `false`                            | Disables the `julia` module.                      |

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

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Opción                  | Por defecto                                              | Descripción                                                           |
| ----------------------- | -------------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`                | `"☸ "`                                                   | A format string representing the symbol displayed before the Cluster. |
| `format`                | `"on [$symbol$context( \\($namespace\\))]($style) "` | El formato del módulo.                                                |
| `style`                 | `"cyan bold"`                                            | El estilo del módulo.                                                 |
| `namespace_spaceholder` | `none`                                                   | The value to display if no namespace was found.                       |
| `context_aliases`       |                                                          | Table of context aliases to display.                                  |
| `disabled`              | `true`                                                   | Disables the `kubernetes` module.                                     |

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
format = "on [⛵ $context \\($namespace\\)](dimmed green) "
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

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Opción      | Por defecto                                   | Descripción                                              |
| ----------- | --------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                          | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | El formato del módulo.                                   |
| `symbol`    | `"🐏"`                                         | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                         | El estilo del módulo.                                    |
| `disabled`  | `true`                                        | Disables the `memory_usage` module.                      |

### Variables

| Variable      | Ejemplo       | Descripción                                                        |
| ------------- | ------------- | ------------------------------------------------------------------ |
| ram           | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct       | `48%`         | The percentage of the current system memory.                       |
| swap\**     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\** | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol        | `🐏`           | Refleja el valor de la opción `symbol`                             |
| style\*     |               | Refleja el valor de la opción `style`                              |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

### Ejemplo

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
show_percentage = true
show_swap = true
threshold = -1
symbol = " "
separator = "/"
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
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated.                                     |
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

The `nim` module shows the currently installed version of Nim. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Opciones

| Opción     | Por defecto                        | Descripción                                           |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module                             |
| `symbol`   | `"👑 "`                             | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"`                    | El estilo del módulo.                                 |
| `disabled` | `false`                            | Disables the `nim` module.                            |

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

| Opción       | Por defecto                                        | Descripción                                           |
| ------------ | -------------------------------------------------- | ----------------------------------------------------- |
| `format`     | `"via [$symbol$state( \\($name\\))]($style) "` | El formato del módulo.                                |
| `symbol`     | `"❄️  "`                                           | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                      | El estilo del módulo.                                 |
| `impure_msg` | `"impure"`                                         | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                           | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                            | Disables the `nix_shell` module.                      |

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
format = "via [☃️ $state( \\($name\\))](bold blue) "
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Opciones

| Opción     | Por defecto                        | Descripción                                        |
| ---------- | ---------------------------------- | -------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                             |
| `symbol`   | `"⬢ "`                             | A format string representing the symbol of NodeJS. |
| `style`    | `"bold green"`                     | El estilo del módulo.                              |
| `disabled` | `false`                            | Disables the `nodejs` module.                      |

###  Variables

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

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Opciones

| Opción            | Por defecto                        | Descripción                                                |
| ----------------- | ---------------------------------- | ---------------------------------------------------------- |
| `format`          | `"via [$symbol$version]($style) "` | El formato del módulo.                                     |
| `symbol`          | `"📦 "`                             | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"`                       | El estilo del módulo.                                      |
| `display_private` | `false`                            | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`                            | Disables the `package` module.                             |

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

## OCaml

The `ocaml` module shows the currently installed version of OCaml. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Opciones

| Opción     | Por defecto                        | Descripción                                             |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                       |
| `symbol`   | `"🐫 "`                             | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"`                    | El estilo del módulo.                                   |
| `disabled` | `false`                            | Disables the `ocaml` module.                            |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v4.10.0` | The version of `ocaml`                 |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```


## Perl

The `perl` module shows the currently installed version of Perl. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Opciones

| Variable   | Por defecto                        | Descripción                                           |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                     |
| `symbol`   | `"🐪 "`                             | The symbol used before displaying the version of Perl |
| `style`    | `"bold 149"`                       | El estilo del módulo.                                 |
| `disabled` | `false`                            | Disables the `perl` module.                           |

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

The `php` module shows the currently installed version of PHP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### Opciones

| Opción     | Por defecto                        | Descripción                                           |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                |
| `symbol`   | `"🐘 "`                             | The symbol used before displaying the version of PHP. |
| `style`    | `"147 bold"`                       | El estilo del módulo.                                 |
| `disabled` | `false`                            | Disables the `php` module.                            |

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

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- The current directory contains a `setup.py` file
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### Opciones

| Opción               | Por defecto                                                    | Descripción                                                                |
| -------------------- | -------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `format`             | `"via [${symbol}${version}( \\($virtualenv\\))]($style) "` | El formato del módulo.                                                     |
| `symbol`             | `"🐍 "`                                                         | A format string representing the symbol of Python                          |
| `style`              | `"yellow bold"`                                                | El estilo del módulo.                                                      |
| `pyenv_version_name` | `false`                                                        | Use pyenv to get Python version                                            |
| `scan_for_pyfiles`   | `true`                                                         | If false, Python files in the current directory will not show this module. |
| `disabled`           | `false`                                                        | Disables the `python` module.                                              |

### Variables

| Variable   | Ejemplo         | Descripción                            |
| ---------- | --------------- | -------------------------------------- |
| version    | `"v3.8.1"`      | The version of `python`                |
| symbol     | `"🐍 "`          | Refleja el valor de la opción `symbol` |
| style      | `"yellow bold"` | Refleja el valor de la opción `style`  |
| virtualenv | `"venv"`        | The current `virtualenv` name          |

<details>
<summary>This module has some advanced configuration options.</summary>

| Variable        | Por defecto | Descripción                                                                   |
| --------------- | ----------- | ----------------------------------------------------------------------------- |
| `python_binary` | `python`    | Configures the python binary that Starship executes when getting the version. |

The `python_binary` variable changes the binary that Starship executes to get the version of Python, it doesn't change the arguments that are used.

```toml
# ~/.config/starship.toml

[python]
python_binary = "python3"
```

</details>

### Ejemplo

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

The `ruby` module shows the currently installed version of Ruby. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Opciones

| Opción     | Por defecto                        | Descripción                                      |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                           |
| `symbol`   | `"💎 "`                             | A format string representing the symbol of Ruby. |
| `style`    | `"bold red"`                       | El estilo del módulo.                            |
| `disabled` | `false`                            | Disables the `ruby` module.                      |

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

The `rust` module shows the currently installed version of Rust. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Opciones

| Opción     | Por defecto                        | Descripción                                     |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                          |
| `symbol`   | `"🦀 "`                             | A format string representing the symbol of Rust |
| `style`    | `"bold red"`                       | El estilo del módulo.                           |
| `disabled` | `false`                            | Disables the `rust` module.                     |

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

## SHLVL

The `shlvl` module shows the current SHLVL ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Opciones

| Variable    | Por defecto                  | Descripción                             |
| ----------- | ---------------------------- | --------------------------------------- |
| `threshold` | `2`                          | Display threshold.                      |
| `format`    | `"[$symbol$shlvl]($style) "` | El formato del módulo.                  |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the SHLVL. |
| `style`     | `"bold yellow"`              | El estilo del módulo.                   |
| `disabled`  | `true`                       | Disables the `shlvl` module.            |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | The current value of SHLVL             |
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

| Opción     | Por defecto                          | Descripción                                      |
| ---------- | ------------------------------------ | ------------------------------------------------ |
| `format`   | `"[$symbol\\[$env\\]]($style) "` | El formato del módulo.                           |
| `symbol`   | `""`                                 | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`                 | El estilo del módulo.                            |
| `disabled` | `false`                              | Disables the `singularity` module.               |

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
format = "[📦 \\[$env\\]]($style) "
```

## Swift

The `swift` module shows the currently installed version of Swift. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Opciones

| Opción     | Por defecto                        | Descripción                                      |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                           |
| `symbol`   | `"🐦 "`                             | A format string representing the symbol of Swift |
| `style`    | `"bold 202"`                       | El estilo del módulo.                            |
| `disabled` | `false`                            | Disables the `swift` module.                     |

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

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-version). El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### Opciones

| Opción     | Por defecto                          | Descripción                                           |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`   | `"💠 "`                               | A format string shown before the terraform workspace. |
| `style`    | `"bold 105"`                         | El estilo del módulo.                                 |
| `disabled` | `false`                              | Disables the `terraform` module.                      |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v0.12.24` | The version of `terraform`             |
| workspace | `default`  | The current terraform workspace        |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

\*: This variable can only be used as a part of a style string

### Ejemplo

#### With Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Opción            | Por defecto             | Descripción                                                                                                                        |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                  |
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
format = "🕙[\\[ $time \\]]($style) "
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### Opciones

| Opción        | Por defecto              | Descripción                           |
| ------------- | ------------------------ | ------------------------------------- |
| `style_root`  | `"bold red"`             | The style used when the user is root. |
| `style_user`  | `"bold yellow"`          | The style used for non-root users.    |
| `format`      | `"via [$user]($style) "` | El formato del módulo.                |
| `show_always` | `false`                  | Always shows the `username` module.   |
| `disabled`    | `false`                  | Disables the `username` module.       |

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

## Zig

The `zig` module shows the currently installed version of Zig. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `.zig` file

### Opciones

| Opción     | Por defecto                        | Descripción                                           |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`                             | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"`                    | El estilo del módulo.                                 |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                |
| `disabled` | `false`                            | Disables the `zig` module.                            |

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

The order in which custom modules are shown can be individually set by setting `custom.foo` in `prompt_order`. By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

### Opciones

| Opción        | Por defecto                   | Descripción                                                                                                                |
| ------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                               | The command whose output should be printed.                                                                                |
| `when`        |                               | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                               | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"`     | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                          | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                          | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                          | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                          | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                | El estilo del módulo.                                                                                                      |
| `format`      | `"[$symbol$output]($style) "` | El formato del módulo.                                                                                                     |
| `disabled`    | `false`                       | Disables this `custom` module.                                                                                             |

### Variables

| Variable  | Descripción                               |
| --------- | ----------------------------------------- |
| output    | La salida del comando de shell en `shell` |
| symbol    | Refleja el valor de la opción `symbol`    |
| style\* | Refleja el valor de la opción `style`     |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

#### Comando personalizado de shell

`shell` acepta una lista no vacía de cadenas, donde:

- La primera cadena es la ruta al intérprete de comandos a usar para ejecutar el comando.
- Otros argumentos siguientes son pasados al shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

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
command = "echo foo"  # muestra la salida del comando
files = ["foo"]       # se pueden especificar filtros
when = """ test "$HOME" == "$PWD" """
prefix = " transcending "

[custom.time]
command = "time /T"
files = ["*.pst"]
prefix = "transcending "
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```

## PureScript

El módulo `purescript` muestra la versión actualmente instalada de PureScript. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `spago.dhall`
- El directorio actual contiene un archivo \*.purs

### Opciones

| Opción     | Por defecto                        | Descripción                                                 |
| ---------- | ---------------------------------- | ----------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                      |
| `symbol`   | `"<=> "`                     | El símbolo usado antes de mostrar la versión de PureScript. |
| `style`    | `"bold white"`                     | El estilo del módulo.                                       |
| `disabled` | `false`                            | Deshabilita el módulo `purescript`.                         |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | La versión de `purescript`             |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```
