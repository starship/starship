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

### Logging

Starship registra por defecto los mensajes de advertencia y error en un fichero con nombre `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, en el que la clave de sesión corresponde con una instancia de tu terminal. Esto, sin embargo, puede ser cambiado usando la variable de entorno `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

El equivalente en PowerShell (Windows) es añadir esta línea a tu `$PROFILE`:

```ps1
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### Terminología

**Módulo**: un componente en el símbolo de sistema que provee de información basada en información contextual de tu sistema operativo. Por ejemplo, el módulo "nodejs" muestra la versión de NodeJS que tienes actualmente instalada en tu ordenador, si el directorio actual es un proyecto NodeJS.

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

La mayoría de los módulos de starship permiten configurar los estilos de su cadenas texto. Esto se consigue con una entrada (normalmente llamada `style` - estilo) que no es más que un texto donde se especifica la configuración. A continuación mostramos algunos ejemplos de textos estilados junto con su funcionalidad. Para más detalles sobre la sintaxis completa, consulta [la guía de configuración avanzada](/advanced-config/).

- `"fg:green bg:blue"` pone texto verde sobre un fondo azul
- `"bg:blue fg:bright-green"` pone texto verde claro sobre un fondo azul
- `"bold fg:27"` pone texto en negrita con [color ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` subraya el texto sobre un fondo naranja oscuro
- `"bold italic fg:purple"` pone texto color morado, en negrita y cursiva
- `""` desactiva explícitamente cualquier estilo

Nótese que el estilo es similar a como se controlaría por el emulador de su terminal. Por ejemplo, algunos emuladores de terminal harán los colores más brillantes en lugar de más gruesos, y algunos temas usan los mismos valores para texto normal y colores brillantes. Además, para mostrar textos en cursiva tu terminal debe tener soporte para hacerlo.

#### Cadenas de formato condicionales

Una cadena de formato condicional envuelto en `(` y `)` no se renderizará si todas las variables dentro están vacías.

Por ejemplo:

- `(@$region)` no mostrará nada si la variable `region` es `None`, de lo contrario `@` seguido por el valor de la región.
- `(algún texto)` siempre mostrará nada ya que no hay variables envueltas entre llaves.
- Cuando `$all` es un atajo de teclado para `\[$a$b\]`, `($all)` no mostrará nada solo si `$a` y `$b` ambos son `None`. Esto funciona igual que `(\[$a$b\] )`.

#### Carácteres de escape

Los siguientes símbolos tienen un uso especial en una cadena de formato. Si se quiere imprimir los siguientes símbolos, se tienen que escapar con una barra invertida (`\`).

- \$
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

Esta es la lista de opciones de configuración.

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

El `format` predeterminado se utiliza para definir el formato del prompt, si está vacío o no `format` se proporciona. El valor por defecto es el siguiente:

```toml
format = "$all"

# Que es equivalente a
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
$openstack\
$env_var\
$crystal\
$cmd_duration\
$custom\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$character"""
```

## AWS

El módulo `aws` muestra la región actual de AWS y el perfil. Éste se basa en las variables de entorno `AWS_REGION`, `AWS_DEFAULT_REGION`, y `AWS_PROFILE` del fichero `~/.aws/config`.

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
format = 'on [$symbol$profile(\($region\))]($style) '
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

El módulo `battery` muestra la cantidad de batería y si está cargando o no. El módulo es solo visible cuando la batería está por debajo del 10%.

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
<summary>Hay otras opciones para algunos estados de la batería menos comunes.</summary>

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

El módulo `cmake` muestra la versión instalada de CMake si se cumple alguna de las siguientes condiciones:

- El directorio actual contiene un archivo `CMakeLists.txt`
- El directorio actual contiene un archivo `CMakeCache.txt`

### Opciones

| Opción     | Por defecto                        | Descripción                                    |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                         |
| `symbol`   | `"喝 "`                             | El símbolo usado antes de la versión de cmake. |
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
| `show_notifications` | `false`                       | Mostrar notificaciones de escritorio cuando se complete el comando.   |
| `min_time_to_notify` | `45_000`                      | Duración más corta para la notificación (en milisegundos).            |

::: tip

Mostrar notificaciones de escritorio requiere que se construya starship con soporte de `rust-notify`. Comprueba si tu starship soporta notificaciones ejecutando `STARSHIP_LOG=debug starship module cmd_duration -d 60000` cuando `show_notifications` está establecido en `true`.

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

El módulo `conda` muestra el entorno conda actual, si `$CONDA_DEFAULT_ENV` está configurado.

::: tip

Esto no modifica el propio prompt de conda. En caso de querer suprimirlo, ejecuta `conda config --set changeps1 False`.

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

El módulo `crystal` muestra la versión actual instalada de Crystal. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

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

| Opción     | Por defecto                        | Descripción                                             |
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

El módulo `directory` muestra la ruta hasta el directorio actual, mostrando tres directorios padre como máximo. Tu directorio se truncará a la raíz del repositorio git en el que estés.

Cuando usas el estilo fish de la opción pwd, en lugar de ocultar la ruta truncada, verás una versión acortada del nombre de cada directorio basada en el número que activaste para la opción.

Por ejemplo, dado `~/Dev/Nix/nixpkgs/pkgs` donde `nixpkgs` es la raíz del repo y la opción establecida a `1`. Ahora verás `~/D/N/nixpkgs/pkgs`, mientras que antes habría sido `nixpkgs/pkgs`.

### Opciones

| Opción              | Por defecto                                        | Descripción                                                           |
| ------------------- | -------------------------------------------------- | --------------------------------------------------------------------- |
| `truncation_length` | `3`                                                | El número de carpetas a las que se debe truncar el directorio actual. |
| `truncate_to_repo`  | `true`                                             | Truncar o no hasta la raíz del repositorio git en el que se esté.     |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | El formato del módulo.                                                |
| `style`             | `"bold cyan"`                                      | El estilo del módulo.                                                 |
| `disabled`          | `false`                                            | Deshabilita el módulo `directory`.                                    |
| `read_only`         | `"🔒"`                                              | El símbolo que indica si el directorio actual es de sólo lectura.     |
| `read_only_style`   | `"red"`                                            | El estilo para el símbolo de sólo lectura.                            |
| `truncation_symbol` | `""`                                               | El símbolo a prefijar a las rutas truncadas. ej: "…/"                 |

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
truncation_symbol = "…/"
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

El módulo `dotnet` muestra la versión relevante de .NET Core SDK para el directorio actual. Si el SDK ha sido anclado en el directorio actual, se mostrará la versión fijada. De lo contrario, el módulo muestra la última versión instalada del SDK.

Este módulo solo se mostrará en tu mensaje cuando uno o más de los siguientes archivos estén presentes en el directorio actual:

- `global.json`
- `project.json`
- `Directoria.Build.props`
- `Directorio.Build.targets`
- `Packages.props`
- `*.sln`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

También necesitarás tener instalado .NET Core SDK para poder usarlo correctamente.

Internamente, este módulo utiliza su propio mecanismo para la detección de versiones. Normalmente es el doble de rápido que ejecutar `dotnet --version`, pero puede mostrar una versión incorrecta si tu proyecto .NET tiene un diseño de directorio inusual. Si la precisión es más importante que la velocidad, puede desactivar el mecanismo estableciendo `heurístico = false` en las opciones del módulo.

El módulo también mostrará el Target Framework Moniker ([https://docs.microsoft. om/es/dotnet/standard/frameworks#supported-target-framework-versions](https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions)) cuando exista un archivo csproj en el directorio actual.

### Opciones

| Opción      | Por defecto                              | Descripción                                                               |
| ----------- | ---------------------------------------- | ------------------------------------------------------------------------- |
| `format`    | `"v[$symbol$version( 🎯 $tfm)]($style) "` | El formato del módulo.                                                    |
| `symbol`    | `"•NET "`                                | El símbolo usado antes de mostrar la version de dotnet.                   |
| `heuristic` | `true`                                   | Usa una detección de versiones más rápida para mantener a starship veloz. |
| `style`     | `"bold blue"`                            | El estilo del módulo.                                                     |
| `disabled`  | `false`                                  | Deshabilita el módulo `dotnet`.                                           |

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
| `disabled` | `false`                                                   | Deshabilita el módulo `elixir`.                                |

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
| `disabled` | `false`                            | Deshabilita el módulo `elm`.                            |

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
| `disabled`    | `false`                        | Deshabilita el módulo `env_var`.                                                       |

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
| `symbol`   | `" "`                             | El símbolo usado antes de mostrar la versión de erlang. |
| `style`    | `"bold red"`                       | El estilo del módulo.                                   |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                  |
| `disabled` | `false`                            | Deshabilita el módulo `erlang`.                         |

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
| `disabled`       | `false`                                          | Deshabilita el módulo `gcloud`.                            |

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

| Opción              | Por defecto                      | Descripción                                                                                             |
| ------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `format`            | `"on [$symbol$branch]($style) "` | El formato del módulo. Use `"$branch"` para referirse al nombre de la rama actual.                      |
| `symbol`            | `" "`                           | Una cadena de formato que representa el símbolo de la rama git.                                         |
| `style`             | `"bold purple"`                  | El estilo del módulo.                                                                                   |
| `truncation_length` | `2^63 - 1`                       | Trunca una rama git a X grafemas.                                                                       |
| `truncation_symbol` | `"…"`                            | El símbolo usado para indicar que un nombre de rama fue truncado. Puedes usar `""` para ningún símbolo. |
| `disabled`          | `false`                          | Deshabilita el módulo `git_branch`.                                                                     |

### Variables

| Variable  | Ejemplo  | Descripción                                                                                                   |
| --------- | -------- | ------------------------------------------------------------------------------------------------------------- |
| branch    | `master` | El nombre de la rama actual, regresa a `HEAD` si no hay ninguna rama actual (por ejemplo, git detached HEAD). |
| symbol    |          | Refleja el valor de la opción `symbol`                                                                        |
| style\* |          | Refleja el valor de la opción `style`                                                                         |

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
| `disabled`           | `false`                                                | Deshabilita el módulo `git_commit`.                                          |

### Variables

| Variable  | Ejemplo   | Descripción                           |
| --------- | --------- | ------------------------------------- |
| hash      | `b703eb3` | El hash actual del commitn de git     |
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
| `disabled`     | `false`                                                         | Deshabilita el módulo `git_state`.                                                                  |

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
| `disabled`   | `false`                                         | Deshabilita el módulo `git_status`.      |

### Variables

Las siguientes variables se pueden utilizar en `format`:

| Variable       | Descripción                                                                                             |
| -------------- | ------------------------------------------------------------------------------------------------------- |
| `all_status`   | Atajo para `$conflicted$stashed$deleted$renamed$modified$staged$untracked`                              |
| `ahead_behind` | Muestra la cadena de formato de `diverged` `ahead` o `behind` basado enel estado actual del repositorio |
| `conflicted`   | Muestra `conflicted` cuando esta rama tiene conflictos de fusión.                                       |
| `untracked`    | Muestra `untracked` cuando hay archivos sin rastrear en el directorio de trabajo.                       |
| `stashed`      | Muestra `stashed` cuando existe un "stash" para el repositorio local.                                   |
| `modified`     | Muestra `modified` cuando hay modificaciones de archivo en el directorio de trabajo.                    |
| `staged`       | Muestra `staged` cuando se ha añadido un nuevo archivo al área de "stash".                              |
| `renamed`      | Muestra `renamed` cuando un archivo renombrado ha sido añadido al área de "stash".                      |
| `deleted`      | Muestra `deleted` cuando un archivo ha sido añadido al área de "stash".                                 |
| style\*      | Refleja el valor de la opción `style`                                                                   |

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
| `disabled` | `false`                            | Deshabilita el módulo de `golang`.                     |

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
| `disabled` | `false`                            | Deshabilita el módulo `helm`.                            |

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
| `disabled` | `false`                     | Deshabilita el módulo `hostname`.                                                                                                                                 |

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

- El directorio actual contiene un archivo `pom.xml`, `build.gradle.kts`, `build.sbt` o `.java-version`
- El directorio actual contiene un archivo con la extensión `.java`, `.class`, `.gradle` o `.jar`

### Opciones

| Opción     | Por defecto                            | Descripción                                             |
| ---------- | -------------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | El formato del módulo.                                  |
| `symbol`   | `"☕ "`                                 | Una cadena de formato que representa el símbolo de Java |
| `style`    | `"red dimmed"`                         | El estilo del módulo.                                   |
| `disabled` | `false`                                | Deshabilita el módulo `java`.                           |

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

El módulo `jobs` muestra el número actual de tareas en ejecución. El módulo se mostrará sólo si hay tareas en segundo plano ejecutándose. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Opciones

| Opción      | Por defecto                   | Descripción                                               |
| ----------- | ----------------------------- | --------------------------------------------------------- |
| `threshold` | `1`                           | Muestra el número de "jobs" si se exceden.                |
| `format`    | `"[$symbol$number]($style) "` | El formato del módulo.                                    |
| `symbol`    | `"✦"`                         | Una cadena de formato que representa el número de "jobs". |
| `style`     | `"bold blue"`                 | El estilo del módulo.                                     |
| `disabled`  | `false`                       | Deshabilita el módulo `jobs`.                             |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | El número de "jobs"                    |
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
| `disabled` | `false`                            | Deshabilita el módulo `julia`.                            |

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

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Opción            | Por defecto                                          | Descripción                                                                 |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | Una cadena de formato que representa el símbolo mostrado antes del Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | El formato del módulo.                                                      |
| `style`           | `"cyan bold"`                                        | El estilo del módulo.                                                       |
| `context_aliases` |                                                      | Tabla de alias de contexto a mostrar.                                       |
| `disabled`        | `true`                                               | Deshabilita el módulo `kubernetes`.                                         |

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
| `disabled`   | `false`                            | Deshabilita el módulo `lua`.                                            |

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

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Opción      | Por defecto                                   | Descripción                                                   |
| ----------- | --------------------------------------------- | ------------------------------------------------------------- |
| `threshold` | `75`                                          | Ocultar el uso de memoria a menos que supere este porcentaje. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | El formato del módulo.                                        |
| `symbol`    | `"🐏"`                                         | El símbolo usado antes de mostrar el uso de memoria.          |
| `style`     | `"bold dimmed white"`                         | El estilo del módulo.                                         |
| `disabled`  | `true`                                        | Deshabilita el módulo `memory_usage`.                         |

### Variables

| Variable         | Ejemplo       | Descripción                                                                        |
| ---------------- | ------------- | ---------------------------------------------------------------------------------- |
| ram              | `31GiB/65GiB` | La memoria RAM usada/total del sistema actual.                                     |
| ram_pct          | `48%`         | El porcentaje de la memoria actual del sistema.                                    |
| swap\*\*     | `1GiB/4GiB`   | El tamaño de la memoria de intercambio del archivo de memoria del sistema actual.  |
| swap_pct\*\* | `77%`         | El porcentaje de memoria de intercambio del archivo de memoria del sistema actual. |
| symbol           | `🐏`           | Refleja el valor de la opción `symbol`                                             |
| style\*        |               | Refleja el valor de la opción `style`                                              |

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
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`                            | El símbolo usado para indicar que un nombre de rama fue truncado.                            |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

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

The `nodejs` module shows the currently installed version of NodeJS. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `package.json`
- El directorio actual contiene un archivo `.node-version`
- El directorio actual contiene un directorio `node_modules`
- El directorio actual contiene un archivo con la extensión `.js`, `.mjs` o `.cjs`
- El directorio actual contiene un archivo con la extensión `.ts`

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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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
| `disabled` | `false`                                             | Disables the `OpenStack` module.                               |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | The current OpenStack cloud            |
| project   | `dev`   | The current OpenStack project          |
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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of Perl. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Opciones

| Opción     | Por defecto                        | Descripción                                           |
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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `spago.dhall` file
- The current directory contains a \*.purs files

### Opciones

| Opción     | Por defecto                        | Descripción                                                  |
| ---------- | ---------------------------------- | ------------------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                       |
| `symbol`   | `"<=> "`                     | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"`                     | El estilo del módulo.                                        |
| `disabled` | `false`                            | Disables the `purescript` module.                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | The version of `purescript`            |
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

| Opción               | Por defecto                                                               | Descripción                                                                   |
| -------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}${version}( \($virtualenv\))]($style) '` | El formato del módulo.                                                        |
| `symbol`             | `"🐍 "`                                                                    | A format string representing the symbol of Python                             |
| `style`              | `"yellow bold"`                                                           | El estilo del módulo.                                                         |
| `pyenv_version_name` | `false`                                                                   | Use pyenv to get Python version                                               |
| `pyenv_prefix`       | `pyenv`                                                                   | Prefix before pyenv version display, only used if pyenv is used               |
| `scan_for_pyfiles`   | `true`                                                                    | If false, Python files in the current directory will not show this module.    |
| `python_binary`      | `python`                                                                  | Configures the python binary that Starship executes when getting the version. |
| `disabled`           | `false`                                                                   | Disables the `python` module.                                                 |

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

Using the `python3` binary to get the version.

Note - The `python_binary` variable changes the binary that Starship executes to get the version of Python, it doesn't change the arguments that are used.

```toml
# ~/.config/starship.toml

[python]
python_binary = "python3"
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

The `shlvl` module shows the current SHLVL ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Opciones

| Opción      | Por defecto                  | Descripción                             |
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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file. :::

### Opciones

| Opción     | Por defecto                | Descripción                                            |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `[$symbol$status]($style)` | The format of the module                               |
| `symbol`   | `"✖"`                      | A format string representing the symbol for the status |
| `style`    | `"bold red"`               | El estilo del módulo.                                  |
| `disabled` | `true`                     | Disables the `status` module.                          |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| status    | `127`   | The exit code of the last command      |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "💣 "
format = '[\[$symbol$status\]]($style) '
disabled = false

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

| Opción     | Por defecto                          | Descripción                                           |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`   | `"💠 "`                               | A format string shown before the terraform workspace. |
| `style`    | `"bold 105"`                         | El estilo del módulo.                                 |
| `disabled` | `false`                              | Disables the `terraform` module.                      |

### Variables

| Variable  | Ejemplo       | Descripción                            |
| --------- | ------------- | -------------------------------------- |
| version   | `v0.12.24`    | The version of `terraform`             |
| workspace | `por defecto` | The current terraform workspace        |
| symbol    |               | Refleja el valor de la opción `symbol` |
| style\* |               | Refleja el valor de la opción `style`  |

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

## Username

El módulo `username` muestra el nombre de usuario activo. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El usuario actual es root
- El usuario actual no es el mismo que el que está conectado
- El usuario está actualmente conectado como una sesión SSH
- La variable `show_always` se establece en true

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

## Zig

El módulo `zig` muestra la versión instalada de Zig. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `.zig`

### Opciones

| Opción     | Por defecto                        | Descripción                                           |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`                             | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"`                    | El estilo del módulo.                                 |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                |
| `disabled` | `false`                            | Deshabilita el módulo `zig`.                          |

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

| Opción        | Por defecto                   | Descripción                                                                                                                |
| ------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                               | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
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
command = "echo foo"  # muestra la salida del comando
files = ["foo"]       # se pueden especificar filtros
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
