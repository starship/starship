# Configuración

::: tip

🔥 Este apartado está bajo construcción. Habrá nuevas opciones de configuración disponibles en próximas versiones.

:::

Para iniciar la configuración de starship, crea el siguiente fichero: `~/.config.toml`.

```sh
$ mkdir -p ~/.config && touch ~/.config/starship.toml
```

Toda la configuración de starship se incluye en este fichero [TOML](https://github.com/toml-lang/toml):

```toml
# Evita imprimir una nueva linea al comiendo del símbolo del sistema
add_newline = false

# Reemplaza en el símbolo del sistema el carácter "❯" por "➜"
[character]       # El nombre del módulo que estamos configurando es "character"
symbol = "➜"     # El segmento "symbol" se actualiza por "➜"

# Desactiva el gestor de paquetes, ocultándolo del símbolo de sistema por completo
[package]
disabled = true
```

Puedes modificar la ubicación por defecto del fichero `starship.toml` con la variable de entorno `STARSHIP_CONFIG`:
```sh
export STARSHIP_CONFIG=~/.starship
```

### Terminology

**Módulo**: un componente en el símbolo de sistema que provee de información basada en información contextual de tu sistema operativo. Por ejemplo, el módulo "nodejs" muestra la versión de NodeJS que tienes actualmente instalada en tu ordenador, si el directorio actual es un proyecto NodeJS.

**Segmento**: sub-componentes más pequeños que forman un módulo. Por ejemplo, el segmento "symbol" en el módulo "nodejs" contiene el carácter que se muestra antes del número de versión (⬢ por defecto).

A continuación mostramos la representación del módulo "node". En el siguiente ejemplo, "symbol" y "version" son segmentos que lo contienen. Cada módulo tiene además un prefijo y un sufijo que indican el color por defecto en el terminal.

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### Dar estilo a cadenas de texto

La mayoría de los módulos de starship permiten configurar los estilos de su cadenas texto. Esto se consigue con una entrada (normalmente llamada `style` - estilo) que no es más que un texto donde se especifica la configuración. A continuación mostramos algunos ejemplos de textos estilados junto con su funcionalidad. Para más detalles sobre la sintaxis completa, consulta [la guía de configuración avanzada](/advanced-config/).

- `"fg:green bg:blue"` pone texto verde sobre un fondo azul
- `"bg:blue fg:bright-green"` pone texto verde claro sobre un fondo azul
- `"bold fg:27"` pone texto en negrita con [color ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` subraya el texto sobre un fondo naranja oscuro
- `"bold italic fg:purple"` pone texto color morado, en negrita y cursiva
- `""` desactiva explícitamente cualquier estilo

Nótese que el estilo es similar a como se controlaría por el emulador de su terminal. Por ejemplo, algunos emuladores de terminal harán los colores más brillantes en lugar de más gruesos, y algunos temas usan los mismos valores para texto normal y colores brillantes. Además, para mostrar textos en cursiva tu terminal debe tener soporte para hacerlo.

## Símbolo del sistema

Esta es la lista de opciones de configuración.

### Opciones

| Variable       | Por defecto                   | Descripción                                                                     |
| -------------- | ----------------------------- | ------------------------------------------------------------------------------- |
| `add_newline`  | `true`                        | Añade una nueva línea antes del símbolo de sistema.                             |
| `prompt_order` | [link](#default-prompt-order) | Configura el orden en el que se muestran los módulos en el símbolo del sistema. |
| `scan_timeout` | `30`                          | Tiempo de espera tras el que starship escanea archivos (en milisegundos).       |

### Ejemplo

```toml
# ~/.config/starship.toml

# Desactiva el salto de línea al comienzo del símbolo de sistema
add_newline = false
# Sobrescribe el orden por defecto de los módulos
prompt_order=["rust","line_break","package","line_break","character"]
# Starship espera 10 ms para comprobar los archivos del directorio actual.
scan_timeout = 10
```

### Ordenación por defecto

El `prompt_order` por defecto se usa para definir el orden en el que los módulos se muestran en la línea de comandos, en caso de que esté vacío o no existe `prompt_order`. El valor por defecto es el siguiente:

```toml
prompt_order = [
    "username",
    "hostname",
    "kubernetes",
    "directory",
    "git_branch",
    "git_commit",
    "git_state",
    "git_status",
    "hg_branch",
    "docker_context",
    "package",
    "dotnet",
    "elixir",
    "elm",
    "erlang",
    "golang",
    "haskell",
    "java",
    "julia",
    "nodejs",
    "php",
    "python",
    "ruby",
    "rust",
    "terraform",
    "nix_shell",
    "conda",
    "memory_usage",
    "aws",
    "env_var",
    "crystal",
    "cmd_duration",
    "custom",
    "line_break",
    "jobs",
    "battery",
    "time",
    "character",
]
```

## AWS

El módulo `aws` muestra la región actual de AWS y el perfil. Éste se basa en las variables de entorno `AWS_REGION`, `AWS_DEFAULT_REGION`, y `AWS_PROFILE` del fichero `~/.aws/config`.

Cuando uses [aws-vault](https://github.com/99designs/aws-vault) el perfil se obtiene de la variable de entorno `AWS_VAULT`.

### Opciones

| Variable          | Por defecto     | Descripción                                                   |
| ----------------- | --------------- | ------------------------------------------------------------- |
| `symbol`          | `"☁️ "`         | El símbolo que se muestra antes del perfil de AWS.            |
| `displayed_items` | `all`           | Elige qué item mostrar. Valores: [`all`, `profile`, `region`] |
| `region_aliases`  |                 | Tabla de alias de región para mostrar además del nombre AWS.  |
| `style`           | `"bold yellow"` | El estilo del módulo.                                         |
| `disabled`        | `false`         | Desactiva el módulo AWS.                                      |

### Ejemplo

```toml
# ~/.config/starship.toml

[aws]
style = "bold blue"
symbol = "🅰 "
displayed_items = "region"
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

## Battery

El módulo `battery` muestra la cantidad de batería y si está cargando o no. El módulo es solo visible cuando la batería está por debajo del 10%.

### Opciones

| Variable             | Por defecto              | Descripción                                       |
| -------------------- | ------------------------ | ------------------------------------------------- |
| `full_symbol`        | `"•"`                    | Se muestra cuando la batería está cargada.        |
| `charging_symbol`    | `"⇡"`                    | Se muestra cuando la batería está cargando.       |
| `discharging_symbol` | `"⇣"`                    | Se muestra cuando la batería se está descargando. |
| `display`            | [link](#battery-display) | Define cuándo mostrar el indicador y el estilo.   |
| `disabled`           | `false`                  | Desactiva el módulo `battery`.                    |

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

La configuración de la opción `display` es usada para definir cuándo se debe mostrar el indicador de batería y cómo debe mostrarse. Si no se provee ningún valor para `display`  el valor por defecto es el siguiente:

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

El módulo `character` muestra un carater (normalmente una flecha) tras el texto que introduces en el terminal.

El carácter te dirá si el último comando funcionó o no. Se puede hacer de dos formas: cambiando el color (rojo/verde) o cambiando su forma (❯/✖). Esto último solo se puede hacer si `use_symbol_for_status` tiene como valor `true`.

### Opciones

| Variable                | Por defecto    | Descripción                                                                                                                   |
| ----------------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `symbol`                | `"❯"`          | El símbolo usado antes de la entrada de texto en el símbolo del sistema.                                                      |
| `error_symbol`          | `"✖"`          | El símbolo usado antes de la entrada de texto si el comando anterior falló.                                                   |
| `use_symbol_for_status` | `false`        | Indica el estado del error usando un símbolo.                                                                                 |
| `vicmd_symbol`          | `"❮"`          | El símbolo usado antes de la entrada de texto en el símbolo del sistema si el intérprete de comandos está en modo vim normal. |
| `style_success`         | `"bold green"` | Estilo usado si el último comando se ejecutó con éxito.                                                                       |
| `style_failure`         | `"bold red"`   | Estilo usado si el último comando falló.                                                                                      |
| `disabled`              | `false`        | Desactiva el módulo `character`.                                                                                              |

### Ejemplo

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## Tiempo de ejecución

El módulo `cmd_duration` muestra cuánto tiempo tardó el último comando en ejecutarse. El módulo se mostrará solo si el comando llevó dos segundos o más, o el valor de `min_time`, si existe.

::: warning No utilizar DEBUG en Bash

Si estás usando Starship con `bash`, no uses `DEBUG` después de ejecutar `eval $(starship init $0)`, o el módulo **se romperá**.

:::

Los usuarios de bash que necesiten la funcionalidad preexec-like pueden usar el framework rcaloras's bash_preexec. Simplemente define los arrays preexec_functions y precmd_functions antes de ejecutar eval $(starship init $0), y continúa con normalidad. Basta con definir los arrays `preexec_functions` y `precmd_functions` antes de ejecutar `eval $(starship init $0)`, y luego proceder como siempre.

### Opciones

| Variable            | Por defecto     | Descripción                                                           |
| ------------------- | --------------- | --------------------------------------------------------------------- |
| `min_time`          | `2_000`         | Duración mínima para mostrar el tiempo de ejecución (en milisegundos) |
| `show_milliseconds` | `false`         | Muestra la duración con precisión en milisegundos.                    |
| `prefix`            | `took`          | Prefijo que se muestra antes del tiempo de ejecución.                 |
| `style`             | `"bold yellow"` | El estilo del módulo.                                                 |
| `disabled`          | `false`         | Desactiva el módulo `cmd_duration`.                                   |

### Ejemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
prefix = "underwent "
```

## Conda

El módulo `conda` muestra el actual entorno conda, si la variable `$CONDA_DEFAULT_ENV` existe.

::: tip

Esto no modifica el propio símbolo de sistema de conda. En caso de querer suprimirlo, ejecuta `conda config --set changeps1 False`.

:::

### Opciones

| Variable            | Por defecto    | Descripción                                                                                                                                                                                                             |
| ------------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`            | El número de directorios a los que se debe truncar la variable de entorno, si el entorno fue creado usando `conda create -p [path]`. `0` significa sin truncamiento. Mirar también el módulo [`directory`](#directory). |
| `symbol`            | `"C "`         | El símbolo usado antes del nombre del entorno.                                                                                                                                                                          |
| `style`             | `"bold green"` | El estilo del módulo.                                                                                                                                                                                                   |
| `disabled`          | `false`        | Desactiva el módulo `conda`.                                                                                                                                                                                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## Crystal

El módulo `crystal` muestra la versión actual de Crystal. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `shard.yml`
- El directorio actual contiene un fichero `.cr`

### Opciones

| Variable   | Por defecto  | Descripción                                   |
| ---------- | ------------ | --------------------------------------------- |
| `symbol`   | `"🔮 "`       | Símbolo usado antes de la versión de Crystal. |
| `style`    | `"bold red"` | El estilo del módulo.                         |
| `disabled` | `false`      | Desactiva el módulo `crystal`.                |

### Ejemplo

```toml
# ~/.config/starship.toml

[crystal]
symbol = "✨ "
style = "bold blue"
```

## Directory

El módulo `directory` muestra la ruta hasta el directorio actual, mostrando tres directorios padre como máximo. Tu directorio se truncará a la raíz del repositorio git en el que estés.

Cuando usas el estilo fish de la opción pwd, en lugar de ocultar la ruta truncada, verás una versión acortada del nombre de cada directorio basada en el número que activa la opción.

Por ejemplo, dado `~/Dev/Nix/nixpkgs/pkgs` donde `nixpkgs` es la raíz del repositorio y el valor de la opción es `1`. En ese caso, verás `~/D/N/nixpkgs/pkgs`, cuando antes hubiera sido `nixpkgs/pkgs`.

### Opciones

| Variable            | Por defecto   | Descripción                                                                    |
| ------------------- | ------------- | ------------------------------------------------------------------------------ |
| `truncation_length` | `3`           | El número de directorios padre a los que se debe truncar el directorio actual. |
| `truncate_to_repo`  | `true`        | Trunca o no hasta la raíz del repositorio git en el que estés.                 |
| `prefix`            | `"in "`       | Prefijo que se muestra inmediatamente antes del directorio.                    |
| `style`             | `"bold cyan"` | El estilo del módulo.                                                          |
| `disabled`          | `false`       | Desactiva el módulo `directory`.                                               |

<details>
<summary>Este módulo tiene algunas opciones avanzadas de configuración que controlan cómo se muestra el directorio.</summary>

| Variable                    | Por defecto | Descripción                                                                              |
| --------------------------- | ----------- | ---------------------------------------------------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`         | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`      | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Ejemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker context

El módulo `docker_context` muestra el [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) activo si no está a `default`.

### Opciones

| Variable          | Por defecto   | Descripción                                                                                        |
| ----------------- | ------------- | -------------------------------------------------------------------------------------------------- |
| `symbol`          | `"🐳 "`        | Símbolo usado antes de mostrar el Docker context.                                                  |
| `only_with_files` | `false`       | Solo lo muestra cuando hay un archivo `docker-compose.yml` o `Dockerfile` en el directorio actual. |
| `style`           | `"bold blue"` | El estilo del módulo.                                                                              |
| `disabled`        | `true`        | Desactiva el módulo `docker_context`.                                                              |

### Ejemplo

```toml
# ~/.config/starship.toml

[docker_context]
symbol = "🐋 "
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one of the following files are present in the current directory: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj`. You'll also need the .NET Core command-line tools installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

### Opciones

| Variable    | Por defecto   | Descripción                                              |
| ----------- | ------------- | -------------------------------------------------------- |
| `symbol`    | `"•NET "`     | The symbol used before displaying the version of dotnet. |
| `heuristic` | `true`        | Use faster version detection to keep starship snappy.    |
| `style`     | `"bold blue"` | El estilo del módulo.                                    |
| `disabled`  | `false`       | Disables the `dotnet` module.                            |

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

- El directorio actual contiene un fichero `mix.exs`.

### Opciones

| Variable   | Por defecto | Descripción                                                     |
| ---------- | ----------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`      | The symbol used before displaying the version of Elixir/Erlang. |
| `disabled` | `false`     | Disables the `elixir` module.                                   |

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
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### Opciones

| Variable   | Por defecto   | Descripción                                           |
| ---------- | ------------- | ----------------------------------------------------- |
| `symbol`   | `"🌳 "`        | The symbol used before displaying the version of Elm. |
| `style`    | `"bold cyan"` | El estilo del módulo.                                 |
| `disabled` | `false`       | Disables the `elm` module.                            |


### Ejemplo

```toml
# ~/.config/starship.toml

[elm]
symbol = " "
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

### Opciones

| Variable   | Por defecto | Descripción                                              |
| ---------- | ----------- | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`      | The symbol used before displaying the version of Erlang. |
| `disabled` | `false`     | Disables the `erlang` module.                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[erlang]
symbol = "e "
```
## Variable de entorno

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### Opciones

| Variable      | Por defecto      | Descripción                                                                  |
| ------------- | ---------------- | ---------------------------------------------------------------------------- |
| `symbol`      |                  | The symbol used before displaying the variable value.                        |
| `variable`    |                  | The environment variable to be displayed.                                    |
| `por defecto` |                  | The default value to be displayed when the selected variable is not defined. |
| `prefix`      | `""`             | Prefix to display immediately before the variable value.                     |
| `suffix`      | `""`             | Suffix to display immediately after the variable value.                      |
| `style`       | `"dimmed black"` | El estilo del módulo.                                                        |
| `disabled`    | `false`          | Disables the `env_var` module.                                               |

### Ejemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Opciones

| Variable            | Por defecto     | Descripción                                                                           |
| ------------------- | --------------- | ------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the branch name of the repo in your current directory.         |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                 |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol |
| `style`             | `"bold purple"` | El estilo del módulo.                                                                 |
| `disabled`          | `false`         | Disables the `git_branch` module.                                                     |

### Ejemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git commit

The `git_commit` module shows the current commit hash of the repo in your current directory.

### Opciones

| Variable             | Por defecto    | Descripción                                           |
| -------------------- | -------------- | ----------------------------------------------------- |
| `commit_hash_length` | `7`            | The length of the displayed git commit hash.          |
| `prefix`             | `"("`          | Prefix to display immediately before git commit.      |
| `suffix`             | `")"`          | Suffix to display immediately after git commit.       |
| `style`              | `"bold green"` | El estilo del módulo.                                 |
| `only_detached`      | `true`         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`        | Disables the `git_commit` module.                     |

### Ejemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git state

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Opciones

| Variable           | Por defecto        | Descripción                                                                                                      |
| ------------------ | ------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `rebase`           | `"REBASING"`       | The text displayed when a `rebase` is in progress.                                                               |
| `merge`            | `"MERGING"`        | The text displayed when a `merge` is in progress.                                                                |
| `revert`           | `"REVERTING"`      | The text displayed when a `revert` is in progress.                                                               |
| `cherry_pick`      | `"CHERRY-PICKING"` | The text displayed when a `cherry-pick` is in progress.                                                          |
| `bisect`           | `"BISECTING"`      | The text displayed when a `bisect` is in progress.                                                               |
| `am`               | `"AM"`             | The text displayed when an `apply-mailbox` (`git am`) is in progress.                                            |
| `am_or_rebase`     | `"AM/REBASE"`      | The text displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.                                 |
| `progress_divider` | `"/"`              | The symbol or text which will separate the current and total progress amounts. (e.g., `" of "`, for `"3 of 10"`) |
| `style`            | `"bold yellow"`    | El estilo del módulo.                                                                                            |
| `disabled`         | `false`            | Disables the `git_state` module.                                                                                 |

### Ejemplo

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Opciones

| Variable           | Por defecto                | Descripción                                             |
| ------------------ | -------------------------- | ------------------------------------------------------- |
| `conflicted`       | `"="`                      | This branch has merge conflicts.                        |
| `conflicted_count` | [link](#git-status-counts) | Show and style the number of conflicts.                 |
| `ahead`            | `"⇡"`                      | This branch is ahead of the branch being tracked.       |
| `behind`           | `"⇣"`                      | This branch is behind of the branch being tracked.      |
| `diverged`         | `"⇕"`                      | This branch has diverged from the branch being tracked. |
| `untracked`        | `"?"`                      | There are untracked files in the working directory.     |
| `untracked_count`  | [link](#git-status-counts) | Show and style the number of untracked files.           |
| `stashed`          | `"$"`                      | A stash exists for the local repository.                |
| `stashed_count`    | [link](#git-status-counts) | Show and style the number of stashes.                   |
| `modified`         | `"!"`                      | There are file modifications in the working directory.  |
| `modified_count`   | [link](#git-status-counts) | Show and style the number of modified files.            |
| `staged`           | `"+"`                      | A new file has been added to the staging area.          |
| `staged_count`     | [link](#git-status-counts) | Show and style the number of files staged files.        |
| `renamed`          | `"»"`                      | A renamed file has been added to the staging area.      |
| `renamed_count`    | [link](#git-status-counts) | Show and style the number of renamed files.             |
| `deleted`          | `"✘"`                      | A file's deletion has been added to the staging area.   |
| `deleted_count`    | [link](#git-status-counts) | Show and style the number of deleted files.             |
| `show_sync_count`  | `false`                    | Show ahead/behind count of the branch being tracked.    |
| `prefix`           | `[`                        | Prefix to display immediately before git status.        |
| `suffix`           | `]`                        | Suffix to display immediately after git status.         |
| `style`            | `"bold red"`               | El estilo del módulo.                                   |
| `disabled`         | `false`                    | Disables the `git_status` module.                       |

#### Contadores de git status

| Variable  | Por defecto | Descripción                                            |
| --------- | ----------- | ------------------------------------------------------ |
| `enabled` | `false`     | Show the number of files                               |
| `style`   |             | Optionally style the count differently than the module |

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
staged.value = "++"
staged.style = "green"
staged_count.enabled = true
staged_count.style = "green"
renamed = "👅"
deleted = "🗑"
```

## Golang

The `golang` module shows the currently installed version of Golang. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `go.mod`
- El directorio actual contiene un fichero `go.sum`
- El directorio actual contiene un fichero `glide.yaml`
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Opciones

| Variable   | Por defecto   | Descripción                                              |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | El estilo del módulo.                                    |
| `disabled` | `false`       | Disables the `golang` module.                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `stack.yaml`

### Opciones

| Variable   | Por defecto  | Descripción                                               |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"λ "`       | The symbol used before displaying the version of Haskell. |
| `style`    | `"bold red"` | El estilo del módulo.                                     |
| `disabled` | `false`      | Disables the `haskell` module.                            |


### Ejemplo

```toml
# ~/.config/starship.toml

[haskell]
symbol = " "
```

## Hostname

The `hostname` module shows the system hostname.

### Opciones

| Variable   | Por defecto           | Descripción                                                                                                                          |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session.                                                                                 |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.                                                                                   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.                                                                                    |
| `trim_at`  | `"."`                 | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `style`    | `"bold dimmed green"` | El estilo del módulo.                                                                                                                |
| `disabled` | `false`               | Disables the `hostname` module.                                                                                                      |

### Ejemplo

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
prefix = "⟪"
suffix = "⟫"
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### Opciones

| Variable   | Por defecto    | Descripción                                            |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"☕ "`         | The symbol used before displaying the version of Java. |
| `style`    | `"dimmed red"` | El estilo del módulo.                                  |
| `disabled` | `false`        | Disables the `java` module.                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Opciones

| Variable    | Por defecto   | Descripción                                           |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | El estilo del módulo.                                 |
| `disabled`  | `false`       | Disables the `jobs` module.                           |

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

| Variable   | Por defecto     | Descripción                                             |
| ---------- | --------------- | ------------------------------------------------------- |
| `symbol`   | `"ஃ "`          | The symbol used before displaying the version of Julia. |
| `style`    | `"bold purple"` | El estilo del módulo.                                   |
| `disabled` | `false`         | Disables the `julia` module.                            |

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

| Variable          | Por defecto   | Descripción                                         |
| ----------------- | ------------- | --------------------------------------------------- |
| `symbol`          | `"☸ "`        | The symbol used before displaying the Cluster info. |
| `context_aliases` |               | Table of context aliases to display                 |
| `style`           | `"bold blue"` | El estilo del módulo.                               |
| `disabled`        | `true`        | Disables the `kubernetes` module                    |

### Ejemplo

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dimmed green"
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Salto de línea

The `line_break` module separates the prompt into two lines.

### Opciones

| Variable   | Por defecto | Descripción                                                        |
| ---------- | ----------- | ------------------------------------------------------------------ |
| `disabled` | `false`     | Disables the `line_break` module, making the prompt a single line. |

### Ejemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Memoria utilizada

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Variable          | Por defecto           | Descripción                                                   |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | Display memory usage as a percentage of the available memory. |
| `show_swap`       | `true`                | Display swap usage if total swap is non-zero.                 |
| `threshold`       | `75`                  | Hide the memory usage unless it exceeds this percentage.      |
| `symbol`          | `"🐏 "`                | The symbol used before displaying the memory usage.           |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | El estilo del módulo.                                         |
| `disabled`        | `true`                | Disables the `memory_usage` module.                           |

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

| Variable            | Por defecto     | Descripción                                                                                  |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated.                                     |
| `style`             | `"bold purple"` | El estilo del módulo.                                                                        |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### Ejemplo

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### Opciones

| Variable     | Por defecto   | Descripción                                       |
| ------------ | ------------- | ------------------------------------------------- |
| `use_name`   | `false`       | Display the name of the nix-shell.                |
| `impure_msg` | `"impure"`    | Customize the "impure" msg.                       |
| `pure_msg`   | `"pure"`      | Customize the "pure" msg.                         |
| `symbol`     | `"❄️  "`      | The symbol used before displaying the shell name. |
| `style`      | `"bold blue"` | El estilo del módulo.                             |
| `disabled`   | `false`       | Disables the `nix_shell` module.                  |

### Ejemplo

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
symbol = "☃️  "
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `package.json`
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### Opciones

| Variable   | Por defecto    | Descripción                                              |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | El estilo del módulo.                                    |
| `disabled` | `false`        | Disables the `nodejs` module.                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## Versión del paquete

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Opciones

| Variable          | Por defecto  | Descripción                                                |
| ----------------- | ------------ | ---------------------------------------------------------- |
| `symbol`          | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`           | `"bold red"` | El estilo del módulo.                                      |
| `display_private` | `false`      | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`      | Disables the `package` module.                             |

### Ejemplo

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

The `php` module shows the currently installed version of PHP. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- El directorio actual contiene un fichero `composer.json`
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### Opciones

| Variable   | Por defecto  | Descripción                                           |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold red"` | El estilo del módulo.                                 |
| `disabled` | `false`      | Disables the `php` module.                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

The `python` module shows the currently installed version of Python.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name.

Otherwise, it will display the version number from `python --version` and show the current Python virtual environment if one is activated.

El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `.python-version` file
- El directorio actual contiene un fichero `requirements.txt`
- El directorio actual contiene un fichero `pyproject.toml`
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- El directorio actual contiene un fichero `tox.ini`
- El directorio actual contiene un fichero `setup.py`
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### Opciones

| Variable             | Por defecto     | Descripción                                                                 |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `scan_for_pyfiles`   | `true`          | If false, Python files in the current directory will not show this module.  |
| `style`              | `"bold yellow"` | El estilo del módulo.                                                       |
| `disabled`           | `false`         | Disables the `python` module.                                               |

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

| Variable   | Por defecto  | Descripción                                            |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`       | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | El estilo del módulo.                                  |
| `disabled` | `false`      | Disables the `ruby` module.                            |

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

| Variable   | Por defecto  | Descripción                                            |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`       | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | El estilo del módulo.                                  |
| `disabled` | `false`      | Disables the `rust` module.                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### Opciones

| Variable   | Por defecto          | Descripción                                      |
| ---------- | -------------------- | ------------------------------------------------ |
| `label`    | `""`                 | Prefix before the image name display.            |
| `prefix`   | `"["`                | Prefix to display immediately before image name. |
| `suffix`   | `"]"`                | Suffix to display immediately after image name.  |
| `symbol`   | `""`                 | The symbol used before the image name.           |
| `style`    | `"bold dimmed blue"` | El estilo del módulo.                            |
| `disabled` | `false`              | Disables the `singularity` module.               |

### Ejemplo

```toml
# ~/.config/starship.toml

[singularity]
symbol = "📦 "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### Opciones

| Variable       | Por defecto  | Descripción                                                 |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`       | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | El estilo del módulo.                                       |
| `disabled`     | `false`      | Disables the `terraform` module.                            |

### Ejemplo

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## Hora

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opciones

| Variable          | Por defecto     | Descripción                                                                                                         |
| ----------------- | --------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | Enables 12 hour formatting                                                                                          |
| `format`          | see below       | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `"bold yellow"` | The style for the module time                                                                                       |
| `utc_time_offset` | `"local"`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |
| `disabled`        | `true`          | Disables the `time` module.                                                                                         |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### Ejemplo

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
```

## Nombre de usuario

The `username` module shows active user's username. El módulo se muestra si algunas de las siguientes condiciones se cumplen:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### Opciones

| Variable      | Por defecto     | Descripción                           |
| ------------- | --------------- | ------------------------------------- |
| `style_root`  | `"bold red"`    | The style used when the user is root. |
| `style_user`  | `"bold yellow"` | The style used for non-root users.    |
| `show_always` | `false`         | Always shows the `username` module.   |
| `disabled`    | `false`         | Disables the `username` module.       |

### Ejemplo

```toml
# ~/.config/starship.toml

[username]
disabled = true
```

## Comandos personalizados

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

| Variable      | Por defecto               | Descripción                                                                                                                |
| ------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                           | The command whose output should be printed.                                                                                |
| `when`        |                           | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                           | The path to the shell to use to execute the command. If unset, it will fallback to STARSHIP_SHELL and then to "sh".        |
| `descripción` | `"<custom module>"` | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                      | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                      | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                      | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                      | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`            | El estilo del módulo.                                                                                                      |
| `prefix`      | `""`                      | Prefix to display immediately before the command output.                                                                   |
| `suffix`      | `""`                      | Suffix to display immediately after the command output.                                                                    |
| `disabled`    | `false`                   | Disables this `custom` module.                                                                                             |

### Ejemplo

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
prefix = " transcending "
```
