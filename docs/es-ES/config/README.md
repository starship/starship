# Configuración

Para comenzar a configurar Starship, crea el siguiente archivo: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toda la configuración de Starship se realiza en este archivo [TOML](https://github.com/toml-lang/toml):

```toml
# Evita imprimir una nueva línea al inicio del prompt
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

### Registros

Starship registra por defecto los mensajes de advertencia y error en un fichero con nombre `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, en el que la clave de sesión corresponde con una instancia de tu terminal. Esto, sin embargo, puede ser cambiado usando la variable de entorno `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

El equivalente en PowerShell (Windows) es añadir esta línea a tu `$PROFILE`:

```ps1
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### Terminología

**Módulo**: un componente en el promt que provee información basada en información contextual de tu sistema operativo. Por ejemplo, el módulo "nodejs" muestra la versión de NodeJS que tienes actualmente instalada en tu ordenador, si el directorio actual es un proyecto NodeJS.

**Variable**: Smaller sub-components that contain information provided by the module. Por ejemplo, la variable "version" en el módulo "nodejs" contiene la versión actual de NodeJS.

Por convención, la mayoría de los módulos tienen un prefijo del color predeterminado de la terminal (por ejemplo, `vía` en "nodejs") y un espacio vacío como sufijo.

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
- `[⬢ $version](bold green)` imprimirá un símbolo `⬢` seguido por el contenido de la variable `version`, con texto en negrita color verde.
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

Esta es la lista de opciones de configuración.

### Opciones

| Opción         | Por defecto                        | Descripción                                                                   |
| -------------- | ---------------------------------- | ----------------------------------------------------------------------------- |
| `format`       | [ver aquí](#default-prompt-format) | Configura el formato del prompt.                                              |
| `scan_timeout` | `30`                               | Tiempo de espera tras el que Starship escanea los archivos (en milisegundos). |
| `add_newline`  | `true`                             | Añade una nueva línea antes del prompt.                                       |

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
$kotlin\
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
$character"""
```

## AWS

El módulo `aws` muestra la región y el perfil actual de AWS. Éste se basa en las variables de entorno `AWS_REGION`, `AWS_DEFAULT_REGION`, y `AWS_PROFILE` del fichero `~/.aws/config`.

Cuando uses [aws-vault](https://github.com/99designs/aws-vault) el perfil se obtiene de la variable de entorno `AWS_VAULT`.

### Opciones

| Opción           | Por defecto                                      | Descripción                                                  |
| ---------------- | ------------------------------------------------ | ------------------------------------------------------------ |
| `format`         | `'on [$symbol$profile(\($region\))]($style) '` | El formato del módulo.                                       |
| `symbol`         | `"☁️ "`                                          | El símbolo que se muestra antes del perfil de AWS.           |
| `region_aliases` |                                                  | Tabla de alias de región para mostrar además del nombre AWS. |
| `style`          | `"bold yellow"`                                  | El estilo del módulo.                                        |
| `disabled`       | `false`                                          | Desactiva el módulo AWS.                                     |

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
format = 'en [$symbol$profile(\($region\))]($style) '
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

El módulo `battery` muestra la cantidad de batería y si se está cargando o no. El módulo es solo visible cuando la batería está por debajo del 10%.

### Opciones

| Opción               | Por defecto                       | Descripción                                                              |
| -------------------- | --------------------------------- | ------------------------------------------------------------------------ |
| `full_symbol`        | `""`                             | Se muestra cuando la batería está cargada.                               |
| `charging_symbol`    | `""`                             | Se muestra cuando la batería se está cargando.                           |
| `discharging_symbol` | `""`                             | Se muestra cuando la batería se está descargando.                        |
| `unknown_symbol`     | `""`                             | El símbolo que se muestra cuando el estado de la batería es desconocido. |
| `empty_symbol`       | `""`                             | El símbolo que se muestra cuando el estado de la batería está vacío.     |
| `format`             | `"[$symbol$percentage]($style) "` | El formato del módulo.                                                   |
| `display`            | [ver aquí](#battery-display)      | Define cuándo mostrar el indicador y el estilo.                          |
| `disabled`           | `false`                           | Desactiva el módulo `battery`.                                           |


### Ejemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Indicador de batería

La configuración de la opción `display` es usada para definir cuándo se debe mostrar el indicador de batería y cómo debe mostrarse. Si no se provee ningún valor para `display`  El valor por defecto es el siguiente:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Opciones

La opción `display` es un array de la siguiente tabla.

| Opción      | Descripción                                                     |
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

El módulo `character` muestra un carater (normalmente una flecha) tras el texto que introduces en el terminal.

El carácter te dirá si el último comando funcionó o no. Se puede hacer de dos maneras:

- Cambiando el color (`red`/`green`)
- Cambiando la forma (`.`/`✖`)

Por defecto sólo cambia el color. Si también se quiere cambiar su forma, ver [este ejemplo](#with-custom-error-shape).

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

The `cmake` module shows the currently installed version of CMake if any of the following conditions are met:

- El directorio actual contiene un archivo `CMakeLists.txt`
- El directorio actual contiene un archivo `CMakeCache.txt`

### Opciones

| Opción     | Por defecto                        | Descripción                                    |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                         |
| `symbol`   | `"喝 "`                             | El símbolo usado antes de la versión de cmake. |
| `style`    | `"bold blue"`                      | El estilo del módulo.                          |
| `disabled` | `false`                            | Desactiva el módulo `cmake`.                   |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | La versión de cmake                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

## Tiempo de Ejecución

El módulo `cmd_duration` muestra cuánto tiempo tardó el último comando en ejecutarse. El módulo se mostrará solo si el comando llevó dos segundos o más, o el valor de `min_time`, si existe.

::: warning No utilizar DEBUG en Bash

Si estás usando Starship con `bash`, no uses `DEBUG` después de ejecutar `eval $(starship init $0)`, o el módulo **se romperá**.

:::

Los usuarios de bash que necesiten la funcionalidad preexec-like pueden usar el framework rcaloras's bash_preexec. Simplemente define los arrays preexec_functions y precmd_functions antes de ejecutar eval $(starship init $0), y continúa con normalidad. Basta con definir los arrays `preexec_functions` y `precmd_functions` antes de ejecutar `eval $(starship init $0)`, y luego proceder como siempre.

### Opciones

| Opción               | Por defecto                   | Descripción                                                           |
| -------------------- | ----------------------------- | --------------------------------------------------------------------- |
| `min_time`           | `2_000`                       | Duración mínima para mostrar el tiempo de ejecución (en milisegundos) |
| `show_milliseconds`  | `false`                       | Muestra la duración con precisión en milisegundos.                    |
| `format`             | `"took [$duration]($style) "` | El formato del módulo.                                                |
| `style`              | `"bold yellow"`               | El estilo del módulo.                                                 |
| `disabled`           | `false`                       | Desactiva el módulo `cmd_duration`.                                   |
| `show_notifications` | `false`                       | Muestra notificaciones de escritorio cuando se complete el comando.   |
| `min_time_to_notify` | `45_000`                      | Shortest duration for notification (in milliseconds).                 |

::: tip

Mostrar notificaciones de escritorio requiere que se construya starship con soporte de `rust-notify`. Comprueba si tu Starship soporta notificaciones ejecutando `STARSHIP_LOG=debug starship module cmd_duration -d 60000` cuando `show_notifications` está establecido en `true`.

:::

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

El módulo `conda` muestra el actual entorno conda, si la variable `$CONDA_DEFAULT_ENV` existe.

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

| Opción     | Por defecto                          | Descripción                                   |
| ---------- | ------------------------------------ | --------------------------------------------- |
| `symbol`   | `"🔮 "`                               | Símbolo usado antes de la versión de Crystal. |
| `style`    | `"bold red"`                         | El estilo del módulo.                         |
| `format`   | `"via [$symbol($version )]($style)"` | El formato del módulo.                        |
| `disabled` | `false`                              | Desactiva el módulo `crystal`.                |

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

| Opción     | Por defecto                        | Descripción                                             |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                  |
| `symbol`   | `"🎯 "`                             | Una cadena de formato que representa el símbolo de Dart |
| `style`    | `"bold blue"`                      | El estilo del módulo.                                   |
| `disabled` | `false`                            | Desactiva el módulo `dart`.                             |

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

El módulo `directory` muestra la ruta hasta el directorio actual, mostrando tres directorios padre como máximo. Tu directorio se truncará a la raíz del repositorio git en el que estés.

Cuando usas el estilo fish de la opción pwd, en lugar de ocultar la ruta truncada, verás una versión acortada del nombre de cada directorio basada en el número que activa la opción.

Por ejemplo, dado `~/Dev/Nix/nixpkgs/pkgs` donde `nixpkgs` es la raíz del repositorio y el valor de la opción es `1`. En ese caso, verás `~/D/N/nixpkgs/pkgs`, cuando antes hubiera sido `nixpkgs/pkgs`.

### Opciones

| Opción              | Por defecto                                        | Descripción                                                                    |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------ |
| `truncation_length` | `3`                                                | El número de directorios padre a los que se debe truncar el directorio actual. |
| `truncate_to_repo`  | `true`                                             | Trunca o no hasta la raíz del repositorio git en el que estés.                 |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | El formato del módulo.                                                         |
| `style`             | `"bold cyan"`                                      | El estilo del módulo.                                                          |
| `disabled`          | `false`                                            | Desactiva el módulo `directory`.                                               |
| `read_only`         | `"🔒"`                                              | El símbolo que indica si el directorio actual es de sólo lectura.              |
| `read_only_style`   | `"red"`                                            | El estilo para el símbolo de sólo lectura.                                     |
| `truncation_symbol` | `""`                                               | El símbolo a prefijar a las rutas truncadas. ej: "…/"                          |

<details>
<summary>Este módulo tiene algunas opciones avanzadas de configuración que controlan cómo se muestra el directorio.</summary>

| Opciones avanzadas          | Por defecto | Descripción                                                                                                           |
| --------------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |             | Una tabla de sustituciones que se deben hacer a la ruta.                                                              |
| `fish_style_pwd_dir_length` | `0`         | El número de caracteres a usar al aplicar la lógica de ruta pwd de la shell de fish.                                  |
| `use_logical_path`          | `true`      | Muestra la ruta lógica proporcionada por el intérprete de comandos (`PWD`) en lugar de la ruta del sistema operativo. |

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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

El módulo `docker_context` muestra el [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) activo si no está a `default`.

### Opciones

| Opción            | Por defecto                        | Descripción                                                                                                     |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | El formato del módulo.                                                                                          |
| `symbol`          | `"🐳 "`                             | El símbolo usado antes de mostrar el contexto de Docker.                                                        |
| `style`           | `"blue bold"`                      | El estilo del módulo.                                                                                           |
| `only_with_files` | `false`                            | Only show when there's a `docker-compose.yml`, `docker-compose.yaml`, or `Dockerfile` in the current directory. |
| `disabled`        | `true`                             | Desactiva el módulo `docker_context`.                                                                           |

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

El módulo `dotnet` muestra la versión usada de .NET Core SDK para el directorio actual. Si el SDK ha sido anclado en el directorio actual, se mostrará la versión fijada. De lo contrario, el módulo muestra la última versión instalada del SDK.

Este módulo solo se mostrará en tu mensaje cuando uno o más de los siguientes archivos estén presentes en el directorio actual:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.sln`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

También necesitará tener instalado .NET Core SDK para poder usarlo correctamente.

Internamente, este módulo utiliza su propio mecanismo para la detección de versiones. Normalmente es el doble de rápido que ejecutar `dotnet --version`, pero puede mostrar una versión incorrecta si tu proyecto .NET tiene un diseño de directorio inusual. Si la precisión es más importante que la velocidad, puede desactivar el mecanismo estableciendo `heuristic = false` en las opciones del módulo.

El módulo también mostrará el Target Framework Moniker ([https://docs.microsoft. om/es/dotnet/standard/frameworks#supported-target-framework-versions](https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions)) cuando exista un archivo csproj en el directorio actual.

### Opciones

| Opción      | Por defecto                             | Descripción                                                               |
| ----------- | --------------------------------------- | ------------------------------------------------------------------------- |
| `format`    | `"[$symbol$version( 🎯 $tfm)]($style) "` | El formato del módulo.                                                    |
| `symbol`    | `"•NET "`                               | Símbolo usado antes de mostrar la versión de .NET                         |
| `heuristic` | `true`                                  | Usa una detección de versiones más rápida para mantener a starship veloz. |
| `style`     | `"bold blue"`                           | El estilo del módulo.                                                     |
| `disabled`  | `false`                                 | Deshabilita el módulo `dotnet`.                                           |

### Variables

| Variable  | Ejemplo          | Descripción                                                     |
| --------- | ---------------- | --------------------------------------------------------------- |
| version   | `v3.1.201`       | La version del sdk de `dotnet`                                  |
| tfm       | `netstandard2.0` | El Target Framework Moniker al que se dirige el proyecto actual |
| symbol    |                  | Refleja el valor de la opción `symbol`                          |
| style\* |                  | Refleja el valor de la opción `style`                           |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

El módulo `elixir` muestra la version instalada actualmente de Elixir y Erlang/OTP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `mix.exs`.

### Opciones

| Opción     | Por defecto                                               | Descripción                                                    |
| ---------- | --------------------------------------------------------- | -------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                    | El símbolo usado antes de mostrar la version de Elixir/Erlang. |
| `style`    | `"bold purple"`                                           | El estilo del módulo.                                          |
| `format`   | `'via [$symbol$version \(OTP $otp_version\)]($style) '` | El formato para el módulo elixir.                              |
| `disabled` | `false`                                                   | Desactiva el módulo `elixir`.                                  |

### Variables

| Variable    | Ejemplo | Descripción                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | La version de `elixir`                 |
| otp_version |         | La version de otp de `elixir`          |
| symbol      |         | Refleja el valor de la opción `symbol` |
| style\*   |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

El módulo `elm` muestra la versión actualmente instalada de Elm. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `elm.json`
- El directorio actual contiene un fichero `elm-package.json`
- El directorio actual contiene un archivo `.elm-version`
- El directorio actual contiene una carpeta `elm-stuff`
- El directorio actual contiene archivos `*.elm`

### Opciones

| Opción     | Por defecto                        | Descripción                                             |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                  |
| `symbol`   | `"🌳 "`                             | Una cadena de formato que representa el símbolo de Elm. |
| `style`    | `"cyan bold"`                      | El estilo del módulo.                                   |
| `disabled` | `false`                            | Desactiva el módulo `elm`.                              |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.19.1` | La versión de `elm`                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

### Opciones

| Opción        | Por defecto                    | Descripción                                                                            |
| ------------- | ------------------------------ | -------------------------------------------------------------------------------------- |
| `symbol`      |                                | El símbolo usado antes de mostrar el valor de la variable.                             |
| `variable`    |                                | La variable de entorno a mostrar.                                                      |
| `por defecto` |                                | El valor por defecto que se mostrará cuando la variable seleccionada no está definida. |
| `format`      | `"with [$env_value]($style) "` | El formato del módulo.                                                                 |
| `disabled`    | `false`                        | Desactiva el módulo `env_var`.                                                         |

### Variables

| Variable  | Ejemplo                               | Descripción                                 |
| --------- | ------------------------------------- | ------------------------------------------- |
| env_value | `Windows NT` (si _variable_ es `$OS`) | El valor de entorno de la opción `variable` |
| symbol    |                                       | Refleja el valor de la opción `symbol`      |
| style\* | `black bold dimmed`                   | Refleja el valor de la opción `style`       |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

El módulo `erlang` muestra la versión instalada de Erlang/OTP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `rebar.config`.
- El directorio actual contiene un fichero `erlang.mk`.

### Opciones

| Opción     | Por defecto                        | Descripción                                             |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `symbol`   | `" "`                             | El símbolo usado antes de mostrar la versión de Erlang. |
| `style`    | `"bold red"`                       | El estilo del módulo.                                   |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                  |
| `disabled` | `false`                            | Desactiva el módulo `erlang`.                           |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | La versión de `erlang`                 |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Gcloud

El módulo `gcloud` muestra la configuración actual para el CLI de [`gcloud`](https://cloud.google.com/sdk/gcloud). Esto se basa en el archivo `~/.config/gcloud/active_config`, el archivo `~/.config/gcloud/configurations/config_{CONFIG NAME}` y la varieble de entorno `CLOUDSDK_CONFIG`.

### Opciones

| Opción           | Por defecto                                      | Descripción                                                |
| ---------------- | ------------------------------------------------ | ---------------------------------------------------------- |
| `format`         | `'on [$symbol$account(\($region\))]($style) '` | El formato del módulo.                                     |
| `symbol`         | `"☁️ "`                                          | El símbolo usado antes de mostrar el perfil actual de GCP. |
| `region_aliases` |                                                  | Tabla de alias de región a mostrar además del nombre GCP.  |
| `style`          | `"bold blue"`                                    | El estilo del módulo.                                      |
| `disabled`       | `false`                                          | Desactiva el módulo `gcloud`.                              |

### Variables

| Variable  | Ejemplo           | Descripción                                                                   |
| --------- | ----------------- | ----------------------------------------------------------------------------- |
| region    | `us-central1`     | La región GCP actual                                                          |
| account   | `foo@example.com` | El perfil actual de GCP                                                       |
| project   |                   | El proyecto GCP actual                                                        |
| active    | `por defecto`     | El nombre de configuración activo escrito en `~/.config/gcloud/active_config` |
| symbol    |                   | Refleja el valor de la opción `symbol`                                        |
| style\* |                   | Refleja el valor de la opción `style`                                         |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

El módulo `git_branch` muestra la rama activa del repositorio en su directorio actual.

### Opciones

| Opción               | Por defecto                      | Descripción                                                                                             |
| -------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Muestra el nombre de la rama de seguimiento remoto, incluso si es igual al nombre de la rama local.     |
| `format`             | `"on [$symbol$branch]($style) "` | El formato del módulo. Use `"$branch"` para referirse al nombre de la rama actual.                      |
| `symbol`             | `" "`                           | Una cadena de formato que representa el símbolo de la rama git.                                         |
| `style`              | `"bold purple"`                  | El estilo del módulo.                                                                                   |
| `truncation_length`  | `2^63 - 1`                       | Trunca una rama git a X grafemas.                                                                       |
| `truncation_symbol`  | `"…"`                            | El símbolo usado para indicar que un nombre de rama fue truncado. Puedes usar `""` para ningún símbolo. |
| `only_attached`      | `false`                          | Muestra sólo el nombre de la rama cuando no esté en un estado detached HEAD.                            |
| `disabled`           | `false`                          | Desactiva el módulo `git_branch`.                                                                       |

### Variables

| Variable      | Ejemplo  | Descripción                                                                                                   |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------------- |
| branch        | `master` | El nombre de la rama actual, regresa a `HEAD` si no hay ninguna rama actual (por ejemplo, git detached HEAD). |
| remote_name   | `origin` | The remote name.                                                                                              |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                              |
| symbol        |          | Refleja el valor de la opción `symbol`                                                                        |
| style\*     |          | Refleja el valor de la opción `style`                                                                         |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git commit

El módulo `git_commit` muestra el hash de commit actual y también la etiqueta (si existe) del repositorio en su directorio actual.

### Opciones

| Opción               | Por defecto                                            | Descripción                                                                  |
| -------------------- | ------------------------------------------------------ | ---------------------------------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | La longitud del hash del commit de git mostrado.                             |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | El formato del módulo.                                                       |
| `style`              | `"bold green"`                                         | El estilo del módulo.                                                        |
| `only_detached`      | `true`                                                 | Mostrar solo el hash del commit de git cuando esté en estado "detached HEAD" |
| `tag_disabled`       | `true`                                                 | Deshabilita mostrar información de etiquetas en el módulo `git_commit`.      |
| `tag_symbol`         | `"🏷 "`                                                 | Símbolo de etiqueta prefijando la información mostrada                       |
| `disabled`           | `false`                                                | Desactiva el módulo `git_commit`.                                            |

### Variables

| Variable  | Ejemplo   | Descripción                           |
| --------- | --------- | ------------------------------------- |
| hash      | `b703eb3` | El hash actual del commit de git      |
| style\* |           | Refleja el valor de la opción `style` |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

| Opción         | Por defecto                                                     | Descripción                                                                                         |
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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git status

El módulo `git_status` muestra símbolos que representan el estado del repositorio en su directorio actual.

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
| `disabled`   | `false`                                         | Desactiva el módulo `git_status`.        |

### Variables

Las siguientes variables se pueden utilizar en `format`:

| Variable       | Descripción                                                                                              |
| -------------- | -------------------------------------------------------------------------------------------------------- |
| `all_status`   | Atajo para `$conflicted$stashed$deleted$renamed$modified$staged$untracked`                               |
| `ahead_behind` | Muestra la cadena de formato de `diverged` `ahead` o `behind` basado en el estado actual del repositorio |
| `conflicted`   | Muestra `conflicted` cuando esta rama tiene conflictos de fusión.                                        |
| `untracked`    | Muestra `untracked` cuando hay archivos sin rastrear en el directorio de trabajo.                        |
| `stashed`      | Muestra `stashed` cuando existe un "stash" para el repositorio local.                                    |
| `modified`     | Muestra `modified` cuando hay modificaciones de archivo en el directorio de trabajo.                     |
| `staged`       | Muestra `staged` cuando se ha añadido un nuevo archivo al área de "stash".                               |
| `renamed`      | Muestra `renamed` cuando un archivo renombrado ha sido añadido al área de "stash".                       |
| `deleted`      | Muestra `deleted` cuando un archivo ha sido añadido al área de "stash".                                  |
| style\*      | Refleja el valor de la opción `style`                                                                    |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

Las siguientes variables pueden ser usadas en `diverged`:

| Variable       | Descripción                                             |
| -------------- | ------------------------------------------------------- |
| `ahead_count`  | Número de commits por delante de la rama de seguimiento |
| `behind_count` | Número de commits detrás de la rama de seguimiento      |

Las siguientes variales pueden ser usadas en `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

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

Mostrar el recuento delante/detrás de la rama que está siendo rastreada

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

El módulo `golang` muestra la versión actualmente instalada de Golang. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `go.mod`
- El directorio actual contiene un fichero `go.sum`
- El directorio actual contiene un fichero `glide.yaml`
- El directorio actual contiene un archivo `Gopkg.yml`
- El directorio actual contiene un archivo `Gopkg.lock`
- El directorio actual contiene un archivo `.go-version`
- El directorio actual contiene un directorio `Godeps`
- El directorio actual contiene un archivo con la extensión `.go`

### Opciones

| Opción     | Por defecto                        | Descripción                                            |
| ---------- | ---------------------------------- | ------------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                 |
| `symbol`   | `"🐹 "`                             | Una cadena de formato que representa el símbolo de Go. |
| `style`    | `"bold cyan"`                      | El estilo del módulo.                                  |
| `disabled` | `false`                            | Desactiva el módulo de `golang`.                       |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | La versión de `go`                     |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

El módulo `helm` muestra la versión instalada de Helm. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `helmfile.yaml`
- El directorio actual contiene un archivo `Chart.yaml`

### Opciones

| Opción     | Por defecto                        | Descripción                                              |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                   |
| `symbol`   | `"⎈ "`                             | Una cadena de formato que representa el símbolo de Helm. |
| `style`    | `"bold white"`                     | El estilo del módulo.                                    |
| `disabled` | `false`                            | Desactiva el módulo `helm`.                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | La versión de `helm`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

El módulo `hostname` muestra el nombre de host del sistema.

### Opciones

| Opción     | Por defecto                 | Descripción                                                                                                                                                       |
| ---------- | --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only` | `true`                      | Mostrar sólo el nombre de host cuando esté conectado a una sesión SSH.                                                                                            |
| `trim_at`  | `"."`                       | Cadena en la que el nombre de host se corta, después de la primera partida. `"."` se detendrá después del primer punto. `""` deshabilitará cualquier truncamiento |
| `format`   | `"[$hostname]($style) in "` | El formato del módulo.                                                                                                                                            |
| `style`    | `"bold dimmed green"`       | El estilo del módulo.                                                                                                                                             |
| `disabled` | `false`                     | Desactiva el módulo `hostname`.                                                                                                                                   |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

El módulo `java` muestra la versión actualmente instalada de Java. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, o `build.boot`
- El directorio actual contiene un archivo con la extensión `.java`, `.class`, `.gradle` o `.jar`, `.clj` o `.cljc`

### Opciones

| Opción     | Por defecto                            | Descripción                                             |
| ---------- | -------------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | El formato del módulo.                                  |
| `symbol`   | `"☕ "`                                 | Una cadena de formato que representa el símbolo de Java |
| `style`    | `"red dimmed"`                         | El estilo del módulo.                                   |
| `disabled` | `false`                                | Desactiva el módulo `java`.                             |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| version   | `v14`   | La versión de `java`                   |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

El módulo `jobs` muestra el número actual de tareas en ejecución. El módulo se mostrará sólo si hay tareas en segundo plano ejecutándose. El módulo mostrará el número de tareas en ejecución si hay más de 1 tarea o más que el valor configurado de `threshold`, si existe.

### Opciones

| Opción      | Por defecto                   | Descripción                                               |
| ----------- | ----------------------------- | --------------------------------------------------------- |
| `threshold` | `1`                           | Muestra el número de tareas si se exceden.                |
| `format`    | `"[$symbol$number]($style) "` | El formato del módulo.                                    |
| `symbol`    | `"✦"`                         | Una cadena de formato que representa el número de tareas. |
| `style`     | `"bold blue"`                 | El estilo del módulo.                                     |
| `disabled`  | `false`                       | Desactiva el módulo `jobs`.                               |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | El número de tareas                    |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

El módulo `Julia` muestra la versión actualmente instalada de Julia. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `Project.toml`
- El directorio actual contiene un archivo `Manifest.toml`
- El directorio actual contiene un archivo con la extensión `.jl`

### Opciones

| Opción     | Por defecto                        | Descripción                                               |
| ---------- | ---------------------------------- | --------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                    |
| `symbol`   | `"ஃ "`                             | Una cadena de formato que representa el símbolo de Julia. |
| `style`    | `"bold purple"`                    | El estilo del módulo.                                     |
| `disabled` | `false`                            | Desactiva el módulo `julia`.                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.4.0` | La versión de `julia`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of Kotlin. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `.kt` or a `.kts` file

### Opciones

| Opción          | Por defecto                        | Descripción                                                                   |
| --------------- | ---------------------------------- | ----------------------------------------------------------------------------- |
| `format`        | `"via [$symbol$version]($style) "` | El formato del módulo.                                                        |
| `symbol`        | `"🅺 "`                             | A format string representing the symbol of Kotlin.                            |
| `style`         | `"bold blue"`                      | El estilo del módulo.                                                         |
| `kotlin_binary` | `"kotlin"`                         | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`      | `false`                            | Disables the `kotlin` module.                                                 |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`                |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

Muestra el nombre del contexto actual de Kubernetes y, si se establece, el espacio de nombres del archivo kubeconfig. El espacio de nombres necesita establecerse en el archivo kubeconfig, esto puede hacerse mediante `kubectl config set-context starship-cluster --namespace astronaut`. Si se establece la variable de entorno `$KUBECONFIG`, el módulo usará eso si no usará el `~/.kube/config`.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción            | Por defecto                                          | Descripción                                                                 |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | Una cadena de formato que representa el símbolo mostrado antes del Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | El formato del módulo.                                                      |
| `style`           | `"cyan bold"`                                        | El estilo del módulo.                                                       |
| `context_aliases` |                                                      | Tabla de alias de contexto a mostrar.                                       |
| `disabled`        | `true`                                               | Desactiva el módulo `kubernetes`.                                           |

### Variables

| Variable  | Ejemplo              | Descripción                                                 |
| --------- | -------------------- | ----------------------------------------------------------- |
| context   | `starship-cluster`   | El contexto actual de kubernetes                            |
| namespace | `starship-namespace` | Si se establece, el espacio de nombres actual de kubernetes |
| symbol    |                      | Refleja el valor de la opción `symbol`                      |
| style\* |                      | Refleja el valor de la opción `style`                       |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Salto de línea

El módulo `line_break` separa el indicador en dos líneas.

### Opciones

| Opción     | Por defecto | Descripción                                                                     |
| ---------- | ----------- | ------------------------------------------------------------------------------- |
| `disabled` | `false`     | Deshabilita el módulo `line_break`, haciendo que el mensaje sea una sola línea. |

### Ejemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

El módulo `lua` muestra la versión instalada de Lua. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `.lua-version`
- El directorio actual contiene un directorio `lua`
- El directorio actual contiene un archivo con la extensión `.lua`

### Opciones

| Opción       | Por defecto                        | Descripción                                                             |
| ------------ | ---------------------------------- | ----------------------------------------------------------------------- |
| `format`     | `"via [$symbol$version]($style) "` | El formato del módulo.                                                  |
| `symbol`     | `"🌙 "`                             | Una cadena de formato que representa el símbolo de Lua.                 |
| `style`      | `"bold blue"`                      | El estilo del módulo.                                                   |
| `lua_binary` | `"lua"`                            | Configura el binario de lua que Starship ejecuta al obtener la versión. |
| `disabled`   | `false`                            | Desactiva el módulo `lua`.                                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | La versión de `lua`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

| Opción      | Por defecto                                   | Descripción                                                   |
| ----------- | --------------------------------------------- | ------------------------------------------------------------- |
| `threshold` | `75`                                          | Ocultar el uso de memoria a menos que supere este porcentaje. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | El formato del módulo.                                        |
| `symbol`    | `"🐏"`                                         | El símbolo usado antes de mostrar el uso de memoria.          |
| `style`     | `"bold dimmed white"`                         | El estilo del módulo.                                         |
| `disabled`  | `true`                                        | Desactiva el módulo `memory_usage`.                           |

### Variables

| Variable         | Ejemplo       | Descripción                                                                        |
| ---------------- | ------------- | ---------------------------------------------------------------------------------- |
| ram              | `31GiB/65GiB` | La memoria RAM usada/total del sistema actual.                                     |
| ram_pct          | `48%`         | El porcentaje de la memoria actual del sistema.                                    |
| swap\*\*     | `1GiB/4GiB`   | El tamaño de la memoria de intercambio del archivo de memoria del sistema actual.  |
| swap_pct\*\* | `77%`         | El porcentaje de memoria de intercambio del archivo de memoria del sistema actual. |
| symbol           | `🐏`           | Refleja el valor de la opción `symbol`                                             |
| style\*        |               | Refleja el valor de la opción `style`                                              |

\*: Esta variable sólo puede utilizarse como parte de una cadena de estilo \*\*: La información del archivo SWAP sólo se muestra si se detecta en el sistema actual

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
| `truncation_length` | `2^63 - 1`                       | Trunca el nombre de la rama hg a X grafemas                                                         |
| `truncation_symbol` | `"…"`                            | El símbolo usado para indicar que un nombre de rama fue truncado.                                   |
| `disabled`          | `true`                           | Desactiva el módulo `hg_branch`.                                                                    |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| branch    | `master` | La rama de mercurial activa            |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

El módulo `nim` muestra la versión instalada de Nim. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `nim.cfg`
- El directorio actual contiene un archivo con la extensión `.nim`
- El directorio actual contiene un archivo con la extensión `.nims`
- El directorio actual contiene un archivo con la extensión `.nimble`

### Opciones

| Opción     | Por defecto                        | Descripción                                          |
| ---------- | ---------------------------------- | ---------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo                                |
| `symbol`   | `"👑 "`                             | El símbolo usado antes de mostrar la versión de Nim. |
| `style`    | `"bold yellow"`                    | El estilo del módulo.                                |
| `disabled` | `false`                            | Desactiva el módulo `nim`.                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.2.0` | La versión de `nimc`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

El módulo `nix_shell` muestra el entorno nix-shell. El módulo se mostrará dentro de un entorno nix-shell.

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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

El módulo `nodejs` muestra la versión instalada de NodeJS. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `package.json`
- El directorio actual contiene un archivo `.node-version`
- El directorio actual contiene un directorio `node_modules`
- El directorio actual contiene un archivo con la extensión `.js`, `.mjs` o `.cjs`
- El directorio actual contiene un archivo con la extensión `.ts`

### Opciones

| Opción              | Por defecto                        | Descripción                                                                                                     |
| ------------------- | ---------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$version]($style) "` | El formato del módulo.                                                                                          |
| `symbol`            | `"⬢ "`                             | Una cadena de formato que representa el símbolo de NodeJS.                                                      |
| `style`             | `"bold green"`                     | El estilo del módulo.                                                                                           |
| `disabled`          | `false`                            | Desactiva el módulo `nodejs`.                                                                                   |
| `not_capable_style` | `bold red`                         | El estilo para el módulo cuando una propiedad de motores en Packages.json no coincide con la versión de NodeJS. |

###  Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | La versión de `node`                   |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

El módulo `ocaml` muestra la versión actualmente instalada de OCaml. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo con extensión `.opam` o directorio `_opam`
- El directorio actual contiene un directorio `esy.lock`
- El directorio actual contiene un archivo `dune` o `dune-project`
- El directorio actual contiene un archivo `jbuild` o `jbuild-ignore`
- El directorio actual contiene un archivo `.merlin`
- El directorio actual contiene un archivo con la extensión `.ml`, `.mli`, `.re` o `.rei`

### Opciones

| Opción     | Por defecto                        | Descripción                                            |
| ---------- | ---------------------------------- | ------------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | La cadena de formato para el módulo.                   |
| `symbol`   | `"🐫 "`                             | El símbolo usado antes de mostrar la versión de OCaml. |
| `style`    | `"bold yellow"`                    | El estilo del módulo.                                  |
| `disabled` | `false`                            | Desactiva el módulo `ocaml`.                           |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v4.10.0` | La versión de `ocaml`                  |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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
| `disabled` | `false`                                             | Desactiva el módulo `OpenStack`.                            |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | La nube OpenStack actual               |
| project   | `dev`   | El proyecto OpenStack actual           |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Versión del paquete

El módulo `package` se muestra cuando el directorio actual es el repositorio de un paquete, y muestra su versión actual. El módulo soporta actualmente los paquetes `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` y `helm`.

- **npm** – La versión del paquete `npm` se extrae del `package.json` presente en el directorio actual
- **cargo** – La versión del paquete `cargo` se extrae del `Cargo.toml` presente en el directorio actual
- **poetry** – La versión del paquete `poetry` es extraída del `pyproject.toml` presente en el directorio actual
- **composer** - La versión del paquete `composer` se extrae del `composer.json` presente en el directorio actual
- **gradle** – La versión del paquete `gradle` es extraída del `build.gradle` presente
- **julia** - La versión del paquete se extrae del `Project.toml` presente
- **mix** - La versión del paquete `mix` se extrae del `mix.exs` presente
- **helm** - La versión del gráfico `helm` se extrae del `Chart.yaml` presente
- **maven** - La versión del paquete `maven` es extraída del `pom.xml` presente
- **meson** - The `meson` package version is extracted from the `meson.build` present

> ⚠️ La versión que se muestra es la del paquete cuyo código fuente está en tu directorio actual, no en tu gestor de paquetes.

### Opciones

| Opción            | Por defecto                       | Descripción                                                                      |
| ----------------- | --------------------------------- | -------------------------------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | El formato del módulo.                                                           |
| `symbol`          | `"📦 "`                            | El símbolo usado antes de mostrar la versión del paquete.                        |
| `style`           | `"bold 208"`                      | El estilo del módulo.                                                            |
| `display_private` | `false`                           | Activar la visualización de la versión para los paquetes marcados como privados. |
| `disabled`        | `false`                           | Desactiva el módulo `package`.                                                   |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.0.0` | La versión de su paquete               |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

El módulo `perl` muestra la versión actualmente instalada de Perl. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `Makefile.PL` o `Build.PL`
- El directorio actual contiene un archivo `cpanfile` o `cpanfile.snapshot`
- El directorio actual contiene un archivo `META.json` o `META.yml`
- El directorio actual contiene un archivo `.perl-version`
- El directorio actual contiene un `.pl`, `.pm` o `.pod`

### Opciones

| Opción     | Por defecto                        | Descripción                                          |
| ---------- | ---------------------------------- | ---------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | La cadena de formato para el módulo.                 |
| `symbol`   | `"🐪 "`                             | El símbolo usado antes de mostrar la versión de Perl |
| `style`    | `"bold 149"`                       | El estilo del módulo.                                |
| `disabled` | `false`                            | Desactiva el módulo `perl`.                          |

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

El módulo `php` muestra la versión instalada de PHP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `composer.json`
- El directorio actual contiene un archivo `.php-version`
- El directorio actual contiene un archivo `.php`

### Opciones

| Opción     | Por defecto                        | Descripción                                          |
| ---------- | ---------------------------------- | ---------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                               |
| `symbol`   | `"🐘 "`                             | El símbolo usado antes de mostrar la versión de PHP. |
| `style`    | `"147 bold"`                       | El estilo del módulo.                                |
| `disabled` | `false`                            | Desactiva el módulo `php`.                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v7.3.8` | La versión de `php`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
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

## Python

El módulo `python` muestra la versión actualmente instalada de Python y el actual entorno virtual de Python si uno está activado.

Si `pyenv_version_name` se establece en `true`, mostrará el nombre de la versión de pyenv. De lo contrario, se mostrará el número de versión de `python --version`.

El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `.python-version`
- El directorio actual contiene un archivo `requirements.txt`
- El directorio actual contiene un archivo `pyproject.toml`
- El directorio actual contiene un archivo con la extensión `.py` (y `scan_for_pyfiles` es verdadero)
- El directorio actual contiene un archivo `Pipfile`
- El directorio actual contiene un archivo `tox.ini`
- El directorio actual contiene un archivo `setup.py`
- El directorio actual contiene un archivo `__init__.py`
- Un entorno virtual está activado actualmente

### Opciones

| Opción               | Por defecto                                                               | Descripción                                                                           |
| -------------------- | ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}${version}( \($virtualenv\))]($style) '` | El formato del módulo.                                                                |
| `symbol`             | `"🐍 "`                                                                    | Una cadena de formato que representa el símbolo de Python                             |
| `style`              | `"yellow bold"`                                                           | El estilo del módulo.                                                                 |
| `pyenv_version_name` | `false`                                                                   | Usar pyenv para obtener la versión de Python                                          |
| `pyenv_prefix`       | `pyenv`                                                                   | Prefijo antes de mostrar la versión de pyenv sólo se utiliza si se utiliza pyenv      |
| `scan_for_pyfiles`   | `true`                                                                    | Si es falso, los archivos Python en el directorio actual no mostrarán este módulo.    |
| `python_binary`      | `["python", "python3, "python2"]`                                         | Configura los binarios de python que Starship debería ejecutar al obtener la versión. |
| `disabled`           | `false`                                                                   | Desactiva el módulo `python`.                                                         |

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

## Ruby

El módulo `ruby` muestra la versión actualmente instalada de Ruby. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `Gemfile`
- El directorio actual contiene un archivo `.ruby-version`
- El directorio actual contiene un archivo `.rb`

### Opciones

| Opción     | Por defecto                        | Descripción                                              |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                   |
| `symbol`   | `"💎 "`                             | Una cadena de formato que representa el símbolo de Ruby. |
| `style`    | `"bold red"`                       | El estilo del módulo.                                    |
| `disabled` | `false`                            | Desactiva el módulo `ruby`.                              |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | La versión de `ruby`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

El módulo `rust` muestra la versión instalada de Rust. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `Cargo.toml`
- El directorio actual contiene un archivo con la extensión `.rs`

### Opciones

| Opción     | Por defecto                        | Descripción                                             |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                  |
| `symbol`   | `"🦀 "`                             | Una cadena de formato que representa el símbolo de Rust |
| `style`    | `"bold red"`                       | El estilo del módulo.                                   |
| `disabled` | `false`                            | Desactiva el módulo `rust`.                             |

### Variables

| Variable  | Ejemplo           | Descripción                            |
| --------- | ----------------- | -------------------------------------- |
| version   | `v1.43.0-nightly` | La versión de `rustc`                  |
| symbol    |                   | Refleja el valor de la opción `symbol` |
| style\* |                   | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## SHLVL

El módulo `shlvl` muestra la variable de entorno actual SHLVL ("nivel de shell"), si está establecido en un número y conoce o supera el umbral especificado.

### Opciones

| Opción      | Por defecto                  | Descripción                                                 |
| ----------- | ---------------------------- | ----------------------------------------------------------- |
| `threshold` | `2`                          | Mostrar umbral.                                             |
| `format`    | `"[$symbol$shlvl]($style) "` | El formato del módulo.                                      |
| `symbol`    | `"↕️ "`                      | El símbolo usado para representar el SHLVL.                 |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current SHLVL amount. |
| `style`     | `"bold yellow"`              | El estilo del módulo.                                       |
| `disabled`  | `true`                       | Desactiva el módulo `shlvl`.                                |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | El valor actual de SHLVL               |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

El módulo `singularity` muestra la imagen de singularity actual, si se encuentra dentro de un contenedor y `$SINGULARITY_NAME` está establecido.

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
| env       | `centos.img` | La imagen de singularity actual        |
| symbol    |              | Refleja el valor de la opción `symbol` |
| style\* |              | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

El módulo `status` muestra el código de salida del comando anterior. El módulo se mostrará sólo si el código de salida no es `0`.

::: tip

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción                  | Por defecto                | Descripción                                          |
| ----------------------- | -------------------------- | ---------------------------------------------------- |
| `format`                | `[$symbol$status]($style)` | El formato del módulo                                |
| `symbol`                | `"✖"`                      | The symbol displayed on program error                |
| `not_executable_symbol` | `"🚫"`                      | The symbol displayed when file isn't executable      |
| `not_found_symbol`      | `"🔍"`                      | The symbol displayed when the command can't be found |
| `sigint_symbol`         | `"🧱"`                      | The symbol displayed on SIGINT (Ctrl + c)            |
| `signal_symbol`         | `"⚡"`                      | The symbol displayed on any signal                   |
| `style`                 | `"bold red"`               | El estilo del módulo.                                |
| `recognize_signal_code` | `true`                     | Enable signal mapping from exit code                 |
| `map_symbol`            | `false`                    | Enable symbols mapping from exit code                |
| `disabled`              | `true`                     | Desactiva el módulo `status`.                        |

### Variables

| Variable       | Ejemplo | Descripción                                                          |
| -------------- | ------- | -------------------------------------------------------------------- |
| status         | `127`   | El código de salida del último comando                               |
| int            | `127`   | El código de salida del último comando                               |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                  |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled      |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found         |
| symbol         |         | Refleja el valor de la opción `symbol`                               |
| style\*      |         | Refleja el valor de la opción `style`                                |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $status_common_meaning$status_signal_name$status_maybe_int\]]($style) '
map_symbol = true
disabled = false

```

## Swift

The `swift` module shows the currently installed version of Swift. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `Package.swift`
- El directorio actual contiene un archivo con la extensión `.swift`

### Opciones

| Opción     | Por defecto                        | Descripción                                              |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                   |
| `symbol`   | `"🐦 "`                             | Una cadena de formato que representa el símbolo de Swift |
| `style`    | `"bold 202"`                       | El estilo del módulo.                                    |
| `disabled` | `false`                            | Desactiva el módulo `swift`.                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.2.4` | La versión de `swift`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-version). El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene una carpeta `.terraform`
- El directorio actual contiene un archivo con las extensiones `.tf` o `.hcl`

### Opciones

| Opción     | Por defecto                          | Descripción                                                                     |
| ---------- | ------------------------------------ | ------------------------------------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | La cadena de formato para el módulo.                                            |
| `symbol`   | `"💠 "`                               | Una cadena de formato que se muestra antes del espacio de trabajo de terraform. |
| `style`    | `"bold 105"`                         | El estilo del módulo.                                                           |
| `disabled` | `false`                              | Desactiva el módulo `terraform`.                                                |

### Variables

| Variable  | Ejemplo       | Descripción                               |
| --------- | ------------- | ----------------------------------------- |
| version   | `v0.12.24`    | La versión de `terraform`                 |
| workspace | `por defecto` | El espacio de trabajo actual de terraform |
| symbol    |               | Refleja el valor de la opción `symbol`    |
| style\* |               | Refleja el valor de la opción `style`     |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

Este módulo está deshabilitado por defecto. Para activarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción            | Por defecto             | Descripción                                                                                                                                                                 |
| ----------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | La cadena de formato para el módulo.                                                                                                                                        |
| `use_12hr`        | `false`                 | Activa el formato de 12 horas                                                                                                                                               |
| `time_format`     | see below               | La [cadena de formato de chrono](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) utilizada para formatear la hora.                                          |
| `style`           | `"bold yellow"`         | El estilo para la hora del módulo                                                                                                                                           |
| `utc_time_offset` | `"local"`               | Establece el desplazamiento UTC a utilizar. Rango de -24 &lt; x &lt; 24. Permite a los flotantes acomodar los desplazamientos de zona horaria de 30/45 minutos. |
| `disabled`        | `true`                  | Desactiva el módulo `time`.                                                                                                                                                 |
| `time_range`      | `"-"`                   | Establece el intervalo de tiempo durante el cual el módulo se mostrará. La hora debe ser especificada en formato de 24 horas                                                |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable  | Ejemplo    | Descripción                           |
| --------- | ---------- | ------------------------------------- |
| time      | `13:08:10` | La hora actual.                       |
| style\* |            | Refleja el valor de la opción `style` |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

The `username` module shows active user's username. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El usuario actual es root
- El usuario actual no es el mismo que el que está conectado
- El usuario está actualmente conectado como una sesión SSH
- La variable `show_always` se establece en true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Opciones

| Opción        | Por defecto             | Descripción                                |
| ------------- | ----------------------- | ------------------------------------------ |
| `style_root`  | `"bold red"`            | El estilo usado cuando el usuario es root. |
| `style_user`  | `"bold yellow"`         | El estilo usado para usuarios no root.     |
| `format`      | `"[$user]($style) in "` | El formato del módulo.                     |
| `show_always` | `false`                 | Siempre muestra el módulo `username`.      |
| `disabled`    | `false`                 | Desactiva el módulo `username`.            |

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

## Zig

The `zig` module shows the currently installed version of Zig. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `.zig`

### Opciones

| Opción     | Por defecto                        | Descripción                                          |
| ---------- | ---------------------------------- | ---------------------------------------------------- |
| `symbol`   | `"↯ "`                             | El símbolo usado antes de mostrar la versión de Zig. |
| `style`    | `"bold yellow"`                    | El estilo del módulo.                                |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                               |
| `disabled` | `false`                            | Desactiva el módulo `zig`.                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.6.0` | La versión de `zig`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Comandos personalizados

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- El directorio actual contiene un archivo cuyo nombre está en `files`
- El directorio actual contiene un directorio cuyo nombre está en `directories`
- El directorio actual contiene un archivo cuya extensión está en `extensions`
- El comando `when` devuelve 0

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

| Opción        | Por defecto                   | Descripción                                                                                                                         |
| ------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                               | El comando cuya salida debe ser impresa. El comando se pasará en stdin al shell.                                                    |
| `when`        |                               | Comando de shell usado como condición para mostrar el módulo. El módulo se mostrará si el comando devuelve un código de estado `0`. |
| `shell`       |                               | [Ver abajo](#custom-command-shell)                                                                                                  |
| `description` | `"<custom module>"`     | La descripción del módulo que se muestra al ejecutar `starship explain`.                                                            |
| `files`       | `[]`                          | Los archivos que se buscarán en el directorio de trabajo para obtener una coincidencia.                                             |
| `directories` | `[]`                          | Los directorios que se buscarán en el directorio de trabajo para una coincidencia.                                                  |
| `extensions`  | `[]`                          | Las extensiones que se buscarán en el directorio de trabajo para obtener una coincidencia.                                          |
| `symbol`      | `""`                          | El símbolo usado antes de mostrar la salida del comando.                                                                            |
| `style`       | `"bold green"`                | El estilo del módulo.                                                                                                               |
| `format`      | `"[$symbol$output]($style) "` | El formato del módulo.                                                                                                              |
| `disabled`    | `false`                       | Desactiva este módulo `custom`.                                                                                                     |

### Variables

| Variable  | Descripción                               |
| --------- | ----------------------------------------- |
| output    | La salida del comando de shell en `shell` |
| symbol    | Refleja el valor de la opción `symbol`    |
| style\* | Refleja el valor de la opción `style`     |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

#### Comando personalizado de shell

`shell` accepts a non-empty list of strings, where:

- La primera cadena es la ruta al intérprete de comandos a usar para ejecutar el comando.
- Otros argumentos siguientes son pasados al shell.

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
command = "echo foo"  # muestra la salida del comando
files = ["foo"]       # se pueden especificar filtros
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
