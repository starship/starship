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

| Opción               | Por defecto                       | Descripción                                         |
| -------------------- | --------------------------------- | --------------------------------------------------- |
| `full_symbol`        | `""`                             | Se muestra cuando la batería está cargada.          |
| `charging_symbol`    | `""`                             | Se muestra cuando la batería se está cargando.      |
| `discharging_symbol` | `""`                             | Se muestra cuando la batería se está descargando.   |
| `unknown_symbol`     | `""`                             | The symbol shown when the battery state is unknown. |
| `empty_symbol`       | `""`                             | The symbol shown when the battery state is empty.   |
| `format`             | `"[$symbol$percentage]($style) "` | El formato del módulo.                              |
| `display`            | [ver aquí](#battery-display)      | Display threshold and style for the module.         |
| `disabled`           | `false`                           | Disables the `battery` module.                      |


### Ejemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Indicador de batería

The `display` configuration option is used to define when the battery indicator should be shown (threshold) and what it looks like (style). If no `display` is provided. El valor por defecto es el siguiente:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Opciones

The `display` option is an array of the following table.

| Opción      | Descripción                                     |
| ----------- | ----------------------------------------------- |
| `threshold` | The upper bound for the display option.         |
| `style`     | The style used if the display option is in use. |

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

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways:

- Cambiando el color (`red`/`green`)
- Cambiando la forma (`.`/`✖`)

By default it only changes color. If you also want to change it's shape take a look at [this example](#with-custom-error-shape).

### Opciones

| Opción           | Por defecto         | Descripción                                                                      |
| ---------------- | ------------------- | -------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | The format string used before the text input.                                    |
| `success_symbol` | `"[❯](bold green)"` | The format string used before the text input if the previous command succeeded.  |
| `error_symbol`   | `"[❯](bold red)"`   | The format string used before the text input if the previous command failed.     |
| `vicmd_symbol`   | `"[❮](bold green)"` | The format string used before the text input if the shell is in vim normal mode. |
| `disabled`       | `false`             | Disables the `character` module.                                                 |

### Variables

| Variable | Ejemplo | Descripción                                                           |
| -------- | ------- | --------------------------------------------------------------------- |
| symbol   |         | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

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

| Opción     | Por defecto                        | Descripción                                  |
| ---------- | ---------------------------------- | -------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                       |
| `symbol`   | `"喝 "`                             | The symbol used before the version of cmake. |
| `style`    | `"bold blue"`                      | El estilo del módulo.                        |
| `disabled` | `false`                            | Disables the `cmake` module.                 |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | The version of cmake                   |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

## Tiempo de Ejecución

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Opciones

| Opción               | Por defecto                   | Descripción                                                |
| -------------------- | ----------------------------- | ---------------------------------------------------------- |
| `min_time`           | `2_000`                       | Shortest duration to show time for (in milliseconds).      |
| `show_milliseconds`  | `false`                       | Show milliseconds in addition to seconds for the duration. |
| `format`             | `"took [$duration]($style) "` | El formato del módulo.                                     |
| `style`              | `"bold yellow"`               | El estilo del módulo.                                      |
| `disabled`           | `false`                       | Disables the `cmd_duration` module.                        |
| `show_notifications` | `false`                       | Show desktop notifications when command completes.         |
| `min_time_to_notify` | `45_000`                      | Shortest duration for notification (in milliseconds).      |

::: tip

Showing desktop notifications requires starship to be built with `rust-notify` support. You check if your starship supports notifications by running `STARSHIP_LOG=debug starship module cmd_duration -d 60000` when `show_notifications` is set to `true`.

:::

### Variables

| Variable  | Ejemplo  | Descripción                             |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | Refleja el valor de la opción `style`   |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

| Opción              | Por defecto                            | Descripción                                                                                                                                                                                                 |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `"🅒 "`                                 | The symbol used before the environment name.                                                                                                                                                                |
| `style`             | `"bold green"`                         | El estilo del módulo.                                                                                                                                                                                       |
| `format`            | `"via [$symbol$environment]($style) "` | El formato del módulo.                                                                                                                                                                                      |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                  |
| `disabled`          | `false`                                | Disables the `conda` module.                                                                                                                                                                                |

### Variables

| Variable    | Ejemplo      | Descripción                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | The current conda environment          |
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

The `crystal` module shows the currently installed version of Crystal. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `shard.yml`
- El directorio actual contiene un fichero `.cr`

### Opciones

| Opción     | Por defecto                        | Descripción                                               |
| ---------- | ---------------------------------- | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`                             | The symbol used before displaying the version of crystal. |
| `style`    | `"bold red"`                       | El estilo del módulo.                                     |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                    |
| `disabled` | `false`                            | Disables the `crystal` module.                            |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | The version of `crystal`               |
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

The `dart` module shows the currently installed version of Dart. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo con la extensión `.dart`
- El directorio actual contiene un directorio `.dart_tool`
- El directorio actual contiene un archivo `pubspec.yaml` o `pubspec.lock`

### Opciones

| Opción     | Por defecto                        | Descripción                                     |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                          |
| `symbol`   | `"🎯 "`                             | A format string representing the symbol of Dart |
| `style`    | `"bold blue"`                      | El estilo del módulo.                           |
| `disabled` | `false`                            | Disables the `dart` module.                     |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.8.4` | The version of `dart`                  |
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

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Opciones

| Opción              | Por defecto                                        | Descripción                                                                      |
| ------------------- | -------------------------------------------------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`  | `true`                                             | Whether or not to truncate to the root of the git repo that you're currently in. |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | El formato del módulo.                                                           |
| `style`             | `"bold cyan"`                                      | El estilo del módulo.                                                            |
| `disabled`          | `false`                                            | Disables the `directory` module.                                                 |
| `read_only`         | `"🔒"`                                              | The symbol indicating current directory is read only.                            |
| `read_only_style`   | `"red"`                                            | The style for the read only symbol.                                              |
| `truncation_symbol` | `""`                                               | The symbol to prefix to truncated paths. eg: "…/"                                |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

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
| path      | `"D:/Projects"`       | The current directory path            |
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

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### Opciones

| Opción            | Por defecto                        | Descripción                                                                             |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | El formato del módulo.                                                                  |
| `symbol`          | `"🐳 "`                             | The symbol used before displaying the Docker context.                                   |
| `style`           | `"blue bold"`                      | El estilo del módulo.                                                                   |
| `only_with_files` | `false`                            | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `disabled`        | `true`                             | Disables the `docker_context` module.                                                   |

### Variables

| Variable  | Ejemplo        | Descripción                            |
| --------- | -------------- | -------------------------------------- |
| context   | `test_context` | The current docker context             |
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

| Opción      | Por defecto                             | Descripción                                              |
| ----------- | --------------------------------------- | -------------------------------------------------------- |
| `format`    | `"[$symbol$version( 🎯 $tfm)]($style) "` | El formato del módulo.                                   |
| `symbol`    | `"•NET "`                               | The symbol used before displaying the version of dotnet. |
| `heuristic` | `true`                                  | Use faster version detection to keep starship snappy.    |
| `style`     | `"bold blue"`                           | El estilo del módulo.                                    |
| `disabled`  | `false`                                 | Disables the `dotnet` module.                            |

### Variables

| Variable  | Ejemplo          | Descripción                                                        |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Refleja el valor de la opción `symbol`                             |
| style\* |                  | Refleja el valor de la opción `style`                              |

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

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `mix.exs`.

### Opciones

| Opción     | Por defecto                                               | Descripción                                                     |
| ---------- | --------------------------------------------------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                    | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"`                                           | El estilo del módulo.                                           |
| `format`   | `'via [$symbol$version \(OTP $otp_version\)]($style) '` | The format for the module elixir.                               |
| `disabled` | `false`                                                   | Disables the `elixir` module.                                   |

### Variables

| Variable    | Ejemplo | Descripción                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | The version of `elixir`                |
| otp_version |         | The otp version of `elixir`            |
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

The `elm` module shows the currently installed version of Elm. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `elm.json`
- El directorio actual contiene un fichero `elm-package.json`
- El directorio actual contiene un archivo `.elm-version`
- El directorio actual contiene una carpeta `elm-stuff`
- El directorio actual contiene archivos `*.elm`

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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Variable de entorno

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- La opción de configuración de `variable` coincide con una variable de entorno existente
- La opción de configuración de `variable` no está definida, pero la opción de configuración `predeterminada` se encuentra

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
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Refleja el valor de la opción `symbol`     |
| style\* | `black bold dimmed`                         | Refleja el valor de la opción `style`      |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `rebar.config`.
- El directorio actual contiene un fichero `erlang.mk`.

### Opciones

| Opción     | Por defecto                        | Descripción                                              |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `symbol`   | `" "`                             | The symbol used before displaying the version of erlang. |
| `style`    | `"bold red"`                       | El estilo del módulo.                                    |
| `format`   | `"via [$symbol$version]($style) "` | El formato del módulo.                                   |
| `disabled` | `false`                            | Disables the `erlang` module.                            |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | The version of `erlang`                |
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

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Opciones

| Opción           | Por defecto                                      | Descripción                                                     |
| ---------------- | ------------------------------------------------ | --------------------------------------------------------------- |
| `format`         | `'on [$symbol$account(\($region\))]($style) '` | El formato del módulo.                                          |
| `symbol`         | `"☁️ "`                                          | The symbol used before displaying the current GCP profile.      |
| `region_aliases` |                                                  | Table of region aliases to display in addition to the GCP name. |
| `style`          | `"bold blue"`                                    | El estilo del módulo.                                           |
| `disabled`       | `false`                                          | Disables the `gcloud` module.                                   |

### Variables

| Variable  | Ejemplo           | Descripción                                                        |
| --------- | ----------------- | ------------------------------------------------------------------ |
| region    | `us-central1`     | The current GCP region                                             |
| account   | `foo@example.com` | The current GCP profile                                            |
| project   |                   | The current GCP project                                            |
| active    | `default`         | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |                   | Refleja el valor de la opción `symbol`                             |
| style\* |                   | Refleja el valor de la opción `style`                              |

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

The `git_branch` module shows the active branch of the repo in your current directory.

### Opciones

| Opción               | Por defecto                      | Descripción                                                                              |
| -------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Shows the remote tracking branch name, even if it is equal to the local branch name.     |
| `format`             | `"on [$symbol$branch]($style) "` | El formato del módulo. Use `"$branch"` to refer to the current branch name.              |
| `symbol`             | `" "`                           | A format string representing the symbol of git branch.                                   |
| `style`              | `"bold purple"`                  | El estilo del módulo.                                                                    |
| `truncation_length`  | `2^63 - 1`                       | Truncates a git branch to X graphemes.                                                   |
| `truncation_symbol`  | `"…"`                            | The symbol used to indicate a branch name was truncated. You can use `""` for no symbol. |
| `only_attached`      | `false`                          | Only show the branch name when not in a detached HEAD state.                             |
| `disabled`           | `false`                          | Disables the `git_branch` module.                                                        |

### Variables

| Variable      | Ejemplo  | Descripción                                                                                          |
| ------------- | -------- | ---------------------------------------------------------------------------------------------------- |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached HEAD). |
| remote_name   | `origin` | The remote name.                                                                                     |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                     |
| symbol        |          | Refleja el valor de la opción `symbol`                                                               |
| style\*     |          | Refleja el valor de la opción `style`                                                                |

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

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Opciones

| Opción               | Por defecto                                            | Descripción                                           |
| -------------------- | ------------------------------------------------------ | ----------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | The length of the displayed git commit hash.          |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | El formato del módulo.                                |
| `style`              | `"bold green"`                                         | El estilo del módulo.                                 |
| `only_detached`      | `true`                                                 | Only show git commit hash when in detached HEAD state |
| `tag_disabled`       | `true`                                                 | Disables showing tag info in `git_commit` module.     |
| `tag_symbol`         | `"🏷 "`                                                 | Tag symbol prefixing the info shown                   |
| `disabled`           | `false`                                                | Disables the `git_commit` module.                     |

### Variables

| Variable  | Ejemplo   | Descripción                           |
| --------- | --------- | ------------------------------------- |
| hash      | `b703eb3` | The current git commit hash           |
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

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Opciones

| Opción         | Por defecto                                                     | Descripción                                                                             |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | El estilo del módulo.                                                                   |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | El formato del módulo.                                                                  |
| `disabled`     | `false`                                                         | Disables the `git_state` module.                                                        |

### Variables

| Variable         | Ejemplo    | Descripción                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | The current state of the repo         |
| progress_current | `1`        | The current operation progress        |
| progress_total   | `2`        | The total operation progress          |
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

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Opciones

| Opción       | Por defecto                                     | Descripción                         |
| ------------ | ----------------------------------------------- | ----------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status` |
| `conflicted` | `"="`                                           | This branch has merge conflicts.    |
| `ahead`      | `"⇡"`                                           | The format of `ahead`               |
| `behind`     | `"⇣"`                                           | The format of `behind`              |
| `diverged`   | `"⇕"`                                           | The format of `diverged`            |
| `untracked`  | `"?"`                                           | The format of `untracked`           |
| `stashed`    | `"$"`                                           | The format of `stashed`             |
| `modified`   | `"!"`                                           | The format of `modified`            |
| `staged`     | `"+"`                                           | The format of `staged`              |
| `renamed`    | `"»"`                                           | The format of `renamed`             |
| `deleted`    | `"✘"`                                           | The format of `deleted`             |
| `style`      | `"bold red"`                                    | El estilo del módulo.               |
| `disabled`   | `false`                                         | Disables the `git_status` module.   |

### Variables

The following variables can be used in `format`:

| Variable       | Descripción                                                                                   |
| -------------- | --------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                   |
| `ahead_behind` | Displays `diverged` `ahead` or `behind` format string based on the current status of the repo |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                   |
| `untracked`    | Displays `untracked` when there are untracked files in the working directory.                 |
| `stashed`      | Displays `stashed` when a stash exists for the local repository.                              |
| `modified`     | Displays `modified` when there are file modifications in the working directory.               |
| `staged`       | Displays `staged` when a new file has been added to the staging area.                         |
| `renamed`      | Displays `renamed` when a renamed file has been added to the staging area.                    |
| `deleted`      | Displays `deleted` when a file's deletion has been added to the staging area.                 |
| style\*      | Refleja el valor de la opción `style`                                                         |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

The `golang` module shows the currently installed version of Golang. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `go.mod`
- El directorio actual contiene un fichero `go.sum`
- El directorio actual contiene un fichero `glide.yaml`
- El directorio actual contiene un archivo `Gopkg.yml`
- El directorio actual contiene un archivo `Gopkg.lock`
- El directorio actual contiene un archivo `.go-version`
- El directorio actual contiene un directorio `Godeps`
- El directorio actual contiene un archivo con la extensión `.go`

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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of Helm. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `helmfile.yaml`
- El directorio actual contiene un archivo `Chart.yaml`

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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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
| `format`   | `"[$hostname]($style) in "` | El formato del módulo.                                                                                                               |
| `style`    | `"bold dimmed green"`       | El estilo del módulo.                                                                                                                |
| `disabled` | `false`                     | Disables the `hostname` module.                                                                                                      |

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

The `java` module shows the currently installed version of Java. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, o `build.boot`
- El directorio actual contiene un archivo con la extensión `.java`, `.class`, `.gradle` o `.jar`, `.clj` o `.cljc`

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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

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

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un archivo `Project.toml`
- El directorio actual contiene un archivo `Manifest.toml`
- El directorio actual contiene un archivo con la extensión `.jl`

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

The `lua` module shows the currently installed version of Lua. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Opciones

| Opción       | Por defecto                        | Descripción                                                                |
| ------------ | ---------------------------------- | -------------------------------------------------------------------------- |
| `format`     | `"via [$symbol$version]($style) "` | El formato del módulo.                                                     |
| `symbol`     | `"🌙 "`                             | A format string representing the symbol of Lua.                            |
| `style`      | `"bold blue"`                      | El estilo del módulo.                                                      |
| `lua_binary` | `"lua"`                            | Configures the lua binary that Starship executes when getting the version. |
| `disabled`   | `false`                            | Disables the `lua` module.                                                 |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | The version of `lua`                   |
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

| Opción      | Por defecto                                   | Descripción                                              |
| ----------- | --------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                          | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | El formato del módulo.                                   |
| `symbol`    | `"🐏"`                                         | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                         | El estilo del módulo.                                    |
| `disabled`  | `true`                                        | Disables the `memory_usage` module.                      |

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
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| branch    | `master` | The active mercurial branch            |
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
| version   | `v1.2.0` | The version of `nimc`                  |
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

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Opciones

| Opción              | Por defecto                        | Descripción                                                                                           |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$version]($style) "` | El formato del módulo.                                                                                |
| `symbol`            | `"⬢ "`                             | A format string representing the symbol of NodeJS.                                                    |
| `style`             | `"bold green"`                     | El estilo del módulo.                                                                                 |
| `disabled`          | `false`                            | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                         | The style for the module when an engines property in Packages.json does not match the NodeJS version. |

###  Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | The version of `node`                  |
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
- **meson** - The `meson` package version is extracted from the `meson.build` present

> ⚠️ La versión que se muestra es la del paquete cuyo código fuente está en tu directorio actual, no en tu gestor de paquetes.

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

| Opción               | Por defecto                                                               | Descripción                                                                            |
| -------------------- | ------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}${version}( \($virtualenv\))]($style) '` | El formato del módulo.                                                                 |
| `symbol`             | `"🐍 "`                                                                    | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                           | El estilo del módulo.                                                                  |
| `pyenv_version_name` | `false`                                                                   | Use pyenv to get Python version                                                        |
| `pyenv_prefix`       | `pyenv`                                                                   | Prefix before pyenv version display, only used if pyenv is used                        |
| `scan_for_pyfiles`   | `true`                                                                    | If false, Python files in the current directory will not show this module.             |
| `python_binary`      | `["python", "python3, "python2"]`                                         | Configures the python binaries that Starship should executes when getting the version. |
| `disabled`           | `false`                                                                   | Disables the `python` module.                                                          |

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
| version   | `v1.43.0-nightly` | The version of `rustc`                 |
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

| Opción      | Por defecto                  | Descripción                                                 |
| ----------- | ---------------------------- | ----------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                          |
| `format`    | `"[$symbol$shlvl]($style) "` | El formato del módulo.                                      |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the SHLVL.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current SHLVL amount. |
| `style`     | `"bold yellow"`              | El estilo del módulo.                                       |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                |

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
| version   | `v5.2.4` | The version of `swift`                 |
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

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

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
| time      | `13:08:10` | The current time.                     |
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

The `username` module shows active user's username. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

::: tip SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value. :::

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

| Variable  | Descripción                            |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Refleja el valor de la opción `symbol` |
| style\* | Refleja el valor de la opción `style`  |

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
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
