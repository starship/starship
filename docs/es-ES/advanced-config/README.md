# Configuración Avanzada

A pesar de que Starship es una prompt versátil, a veces necesitas hacer más que editar `starhip.toml` para que haga ciertas cosas. Esta página detalla algunas de las técnicas de configuración más avanzadas en Starship.

> [!ADVERTENCIA] Las configuraciones de esta sección están sujetas a cambios en futuras versiones de Starship.

## Prompt Transitoria en PowerShell

Con una cadena personalizada, es posible reemplazar la prompt anteriormente impresa. Esto es útil en los casos en que toda la información de la prompt no es siempre necesaria. Para habilitar esto, ejecuta `Enable-TransientPrompt` en la línea de comandos. Para hacerlo permanente, pon esta misma sentencia en tu `$PROFILE`. La transitoriedad puede ser desactivada al momento con `Disable-TransientPrompt`.

Por defecto, el lado izquierdo de la prompt es reemplazado por `>`. Para personalizar esto, defina una nueva función llamada `Invoke-Starship-TransientFunction`. Por ejemplo, para mostrar el módulo `character` de Starship aquí, harías

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt y TransientRight Prompt en Cmd

Clink re permite reemplazar el prompt impreso anteriormente con cadenas personalizadas. Esto es útil en los casos que toda la información de la entrada no es siempre necesaria. Para habilitar esto, ejecuta `Clink set prompt.transitent <value>` donde \<value\> puede ser uno de:

- `always`: reemplazar siempre el prompt anterior
- `same_dir`: reemplazar el prompt anterior sólo si el directorio de trabajo es el mismo
- `off`: no reemplazar el prompt (es decir, desactivar la transitoriedad)

Necesitas hacer esto sólo una vez. Haz los siguientes cambios en tu `starship.lua` para personalizar lo que se muestra a la izquierda y a la derecha:

- Por defecto, el lado izquierdo de la entrada es reemplazado por `>`. Para personalizar esto, define una nueva función llamada `starship_transient_prompt_func`. Esta función recibe el prompt actual como una cadena que tú puedes utilizar. Por ejemplo, para mostrar el módulo `character` de Starship aquí, harías

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- Por defecto, el lado derecho de la entrada está vacío. Para personalizar esto, defina una nueva función llamada `starship_transient_rprompt_func`. Esta función recibe el prompt actual como una cadena que tú puedes utilizar. Por ejemplo, para mostrar la hora en la que se inició el último comando aquí, lo harías

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt y TransientRightPrompt en Fish

Es posible reemplazar la entrada impresa anteriormente con una cadena personalizada. Esto es útil en los casos que toda la información de la entrada no es siempre necesaria. Para habilitar esto, ejecute `enable_transience` en la sesión del interprete de comandos. Para hacerlo permanente, pon esta proposición en tu `~/.config/fish/config.fish`. La transitoriedad puede ser desactivada al momento con `disable-transience`.

Ten en cuenta que en el caso de Fish, el prompt transitorio sólo se imprime si el intérprete de comandos no está vacío, y sintácticamente correcta.

- Por defecto, el lado izquierdo de la entrada es reemplazado por una  `❯`. Para personalizar esto, define una nueva función llamada `starship_transient_prompt_func`. Por ejemplo, para mostrar el módulo `character` de Starship aquí, harías

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- Por defecto, el lado derecho de la entrada está vacío. Para personalizar esto, defina una nueva función llamada `starship_transient_rprompt_func`. Por ejemplo, para mostrar la hora en la que se inició el último comando aquí, lo harías

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt y TransientRightPrompt en Bash

El marco estructura [Ble.sh](https://github.com/akinomyoga/ble.sh) en v0.4 o superior le permite reemplazar el mensaje impreso previamente con cadenas personalizadas. Esto es útil en los casos en que la información del prompt no es siempre necesaria. Para habilitar esto, coloque esto en `~/.bashrc` `bleopt prompt_ps1_transient=<value>`:

El \<value\> aquí es una lista separada por dos puntos de `siempre`, `mismo-dir` y `recortar`. Cuando `prompt_ps1_final` está vacío y la opción `prompt_ps1_transient` tiene un \<value\> no vacío, el mensaje especificado por `PS1` se borra al salir de la línea de comando actual. Si \<value\> contiene un campo `trim`, solo se conserva la última línea de la multilínea `PS1` y las demás líneas se borran. De lo contrario, la línea de comando se volverá a dibujar como si se hubiera especificado `PS1=`. Cuando un campo `same-dir` está contenido en \<value\> y el directorio de trabajo actual es diferente del directorio final de la línea de comando anterior, esta opción `prompt_ps1_transient` se ignora.

Realice los siguientes cambios en su `~/.blerc` (o en `~/.config/blesh/init.sh`) para personalizar lo que se muestra a la izquierda y a la derecha:

- Para personalizar con qué se reemplaza el lado izquierdo de la entrada, configure la opción `prompt_ps1_final` de Ble.sh. Por ejemplo, para mostrar el módulo de `personaje` de Starship aquí, harías

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- Para personalizar con qué se reemplaza el lado derecho de la entrada, configure la opción `prompt_rps1_final` de Ble.sh. Por ejemplo, para mostrar la hora en la que se inició el último comando aquí, lo harías

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Comandos pre-prompt y pre-ejecución personalizados en Cmd

Clink proporciona una API extremadamente flexible para ejecutar comandos pre-prompt y pre-ejecución en la shell de Cmd. Es bastante sencillo de usar con Starship. Haz los siguientes cambios a tu archivo `starship.lua` según tus requisitos:

- Para ejecutar una función personalizada justo antes de dibujar la shell, defina una nueva función llamada `starship_preprompt_user_func`. Esta función recibe el prompt actual como una cadena que puedes utilizar. Por ejemplo, para dibujar un cohete antes del prompt, podrías hacer lo siguiente

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- Para ejecutar una función personalizada justo antes de ejecutar un comando, define una nueva función llamada `starship_precmd_user_func`. Esta función recibe la línea de comandos actual como una cadena que se puede utilizar. Por ejemplo, para imprimir el comando que va a ser ejecutado, escribirías

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Comandos pre-prompt y pre-ejecucución personalizados en Bash

Bash no posee un framework oficial de preexec/precmd como la mayoría de los demás intérpretes de comandos. Debido a esto, es difícil proporcionar "hooks" totalmente personalizables en `Bash`. Sin embargo, Starship te da la posibilidad de insertar de forma limitada tus propias funciones en el proceso de renderizado del prompt:

- Para ejecutar una función personalizada previa al renderizado del prompt, define una nueva función y asigna su nombre a `starship_precmd_user_func`. Por ejemplo, para dibujar un cohete antes del prompt, se puede realizar así:

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Para ejecutar una función personalizada antes de que un comando sea ejecutado, es posible usar el [mecanismo de trampa `DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). No obstante, ¡**debes** atrapar la señal DEBUG _antes_ de inicializar Starship! Starship puede preservar el valor de la trampa DEBUG, pero si la trampa es reemplazada después de que Starship inicie, alguna funcionalidad fallará.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Comandos pre-prompt y pre-ejecución personalizados en PowerShell

Powershell no posee un framework oficial de preexec/precmd como la mayoría de los demás intérpretes de comandos. Debido a esto, es difícil proporcionar "hooks" totalmente personalizables en `Powershell`. Sin embargo, Starship te da la posibilidad de insertar de forma limitada tus propias funciones en el proceso de renderizado del prompt:

Crea una función llamada `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Cambiar título de la ventana

Algunos intérpretes de comandos van a cambiar automáticamente el título de la ventana por ti (p. ej., para mostrar tu directorio actual). Fish incluso lo hace por defecto. Starship no hace esto, pero es bastante sencillo añadir esta funcionalidad a `bash`, `zsh`, `cmd` o `powershell`.

Primero, define una función para el cambio de título de la ventana (idéntico en Bash y zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; TU_TITULO_DE_VENTANA_AQUI \007"
}
```

Puedes usar variables para personalizar este título (`$USER`, `$HOSTNAME` y `$PWD` son opciones populares).

En `Bash`, establece que esta función sea la función precmd de Starship:

```bash
starship_precmd_user_func="set_win_title"
```

En `zsh`, añade esto al array `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

Si te gusta el resultado, añade estas líneas a tu archivo de configuración del intérprete de comandos (`~/.bashrc` o `~/.zsrhc`) para hacerlo permanente.

Por ejemplo, si quieres mostrar tu directorio actual en el título de la pestaña de la terminal, añade el siguiente fragmento a tu `~/.bashrc` o `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

Para Cmd, puedes cambiar el título de la ventana usando la función `starship_preprompt_user_func`.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

También puede establecer una salida similar con PowerShell creando una función llamada `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Habilitar Prompt a la Derecha

Algunos intérpretes de comandos soportan un prompt derecho que se renderiza en la misma línea que la entrada. Starship puede establecer el contenido del prompt derecho usando la opción `right_format`. Cualquier módulo que pueda ser usado en `format` también es soportado en `right_format`. La variable `$all` solo contendrá módulos no utilizados explícitamente en `format` o `right_format`.

Nota: El prompt derecho es una sola línea siguiendo la ubicación de entrada. Para alinear los módulos arriba de la línea de entrada en un prompt multi-línea, vea el [módulo de `relleno`](../config/#fill).

`right_format` actualmente es compatible con los siguientes shells: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Nota: Se debe instalar el framework [Ble.sh](https://github.com/akinomyoga/ble.sh) v0.4 o superior para poder utilizar el indicador correcto en bash.

### Ejemplo

```toml
# ~/.config/starship. oml

# Un prompt izquierdo mínimo
format = """$character"""

# mueve el resto del mensaje a la derecha
right_format = """$all"""
```

Produce un prompt como el siguiente:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Prompt de continuación

Algunos intérpretes de comandos admiten un prompt de continuacion junto con el prompt normal. Este prompt es renderizado en lugar del prompt normal cuando el usuario ha introducido una orden incompleta (como solamente un paréntesis izquierdo o comilla).

Starship puede establecer el prompt de continuación usando la opción `continuation_prompt`. El indicador por defecto es `'[∙](bright-black) '`.

Nota: `continuation_prompt` debe establecerse en una cadena literal sin ninguna variable.

Nota: Los prompts de continuación solo están disponibles en los siguientes intérpretes de comandos:

- `bash`
- `zsh`
- `PowerShell`

### Ejemplo

```toml
# ~/.config/starship.toml

# Un mensaje de continuación que muestra dos flechas rellenas
continuation_prompt = '▶▶ '
```

## Statusline for Claude Code

Starship supports displaying a custom statusline when running inside Claude Code, Anthropic's CLI tool for interactive coding with Claude. This statusline provides real-time information about your Claude session, including the model being used, context window usage, and session costs.

For more information about the Claude Code statusline feature, see the [Claude Code statusline documentation](https://code.claude.com/docs/en/statusline).

### Setup

To use Starship as your Claude Code statusline:

1. Run `/statusline` in Claude Code and ask it to configure Starship, or manually add the following to your `.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "starship statusline claude-code"
  }
}
```

2. Customize the statusline appearance in your `~/.config/starship.toml` (see [Configuration](#configuration) below)

### Overview

When invoked with `starship statusline claude-code`, Starship receives Claude Code session data via stdin and renders a statusline using a dedicated profile named `claude-code`.

The profile includes three specialized modules:

- `claude_model`: Displays the current Claude model being used
- `claude_context`: Shows context window usage with a visual gauge
- `claude_cost`: Displays session cost and statistics

The default profile format is:

```toml
[profiles]
claude-code = "$claude_model$git_branch$claude_context$claude_cost"
```

### Configuración

You can customize the Claude Code statusline by modifying the `claude-code` profile and individual module configurations in your `~/.config/starship.toml`:

```toml
# ~/.config/starship.toml

# Customize the claude-code profile
[profiles]
claude-code = "$claude_model$claude_context$claude_cost"

# Configure individual modules
[claude_model]
format = "[$symbol$model]($style) "
symbol = "🤖 "
style = "bold blue"

[claude_context]
format = "[$gauge $percentage]($style) "
gauge_width = 10

[claude_cost]
format = "[$symbol$cost]($style) "
symbol = "💰 "
```

### Claude Model

The `claude_model` module displays the current Claude model being used in the session.

#### Opciones

| Opción          | Predeterminado               | Descripción                                                                               |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------------------- |
| `format`        | `'[$symbol$model]($style) '` | El formato del módulo.                                                                    |
| `symbol`        | `'🤖 '`                       | The symbol shown before the model name.                                                   |
| `style`         | `'bold blue'`                | El estilo del módulo.                                                                     |
| `model_aliases` | `{}`                         | Map of model IDs or display names to shorter aliases. Checks ID first, then display name. |
| `disabled`      | `false`                      | Disables the `claude_model` module.                                                       |

#### Variables

| Variable  | Ejemplo             | Descripción                            |
| --------- | ------------------- | -------------------------------------- |
| model     | `Claude 3.5 Sonnet` | The display name of the current model  |
| model_id  | `claude-3-5-sonnet` | The model ID                           |
| symbol    |                     | Refleja el valor de la opción `symbol` |
| style\* |                     | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

#### Ejemplos

```toml
# ~/.config/starship.toml

# Basic customization
[claude_model]
format = "on [$symbol$model]($style) "
symbol = "🧠 "
style = "bold cyan"

# Using model aliases for vendor-specific model names
# You can alias by model ID or display name
[claude_model.model_aliases]
# Alias by vendor model ID (e.g. AWS Bedrock)
"global.anthropic.claude-sonnet-4-5-20250929-v1:0" = "Sonnet 4.5"
# Alias by display name
"Claude Sonnet 4.5 (Vendor Proxy)" = "Sonnet"
```

### Claude Context

The `claude_context` module displays context window usage as a percentage and visual gauge. The style automatically changes based on configurable thresholds.

#### Opciones

| Opción                 | Predeterminado                    | Descripción                                        |
| ---------------------- | --------------------------------- | -------------------------------------------------- |
| `format`               | `'[$gauge $percentage]($style) '` | El formato del módulo.                             |
| `symbol`               | `''`                              | The symbol shown before the gauge.                 |
| `gauge_width`          | `5`                               | The width of the gauge in characters.              |
| `gauge_full_symbol`    | `'█'`                             | The symbol used for filled segments of the gauge.  |
| `gauge_partial_symbol` | `'▒'`                             | The symbol used for partial segments of the gauge. |
| `gauge_empty_symbol`   | `'░'`                             | The symbol used for empty segments of the gauge.   |
| `display`              | [see below](#display)             | Threshold and style configurations.                |
| `disabled`             | `false`                           | Disables the `claude_context` module.              |

##### Display

The `display` option is an array of objects that define thresholds and styles for different usage levels. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Opción      | Predeterminado | Descripción                                                              |
| ----------- | -------------- | ------------------------------------------------------------------------ |
| `threshold` | `0.0`          | The minimum context windows usage percentage to match this configuration |
| `style`     | `bold green`   | The value of `style` if this display configuration is matched            |
| `hidden`    | `false`        | Hide this module if this the configuration is matched.                   |

```toml
[[claude_context.display]]
threshold = 0
hidden = true

[[claude_context.display]]
threshold = 30
style = "bold green"

[[claude_context.display]]
threshold = 60
style = "bold yellow"

[[claude_context.display]]
threshold = 80
style = "bold red"
```

#### Variables

| Variable                     | Ejemplo | Descripción                                           |
| ---------------------------- | ------- | ----------------------------------------------------- |
| gauge                        | `██▒░░` | Visual representation of context usage                |
| percentage                   | `65%`   | Context usage as a percentage                         |
| input_tokens                 | `45.2k` | Total input tokens in conversation                    |
| output_tokens                | `12.3k` | Total output tokens in conversation                   |
| curr_input_tokens          | `5.1k`  | Input tokens from most recent API call                |
| curr_output_tokens         | `1.2k`  | Output tokens from most recent API call               |
| curr_cache_creation_tokens | `1.5k`  | Cache creation tokens from most recent API call       |
| curr_cache_read_tokens     | `23.4k` | Cache read tokens from most recent API call           |
| total_tokens                 | `200k`  | Total context window size                             |
| symbol                       |         | Refleja el valor de la opción `symbol`                |
| style\*                    |         | Mirrors the style from the matching display threshold |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

#### Ejemplos

**Minimal gauge-only display**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$gauge]($style) "
gauge_width = 10
```

**Detailed token information**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$percentage ($input_tokens in / $output_tokens out)]($style) "
```

**Custom gauge symbols**

```toml
# ~/.config/starship.toml

[claude_context]
gauge_full_symbol = "▰"
gauge_partial_symbol = ""
gauge_empty_symbol = "▱"
gauge_width = 10
format = "[$gauge]($style) "
```

**Custom thresholds**

```toml
# ~/.config/starship.toml

[[claude_context.display]]
threshold = 0
style = "bold green"

[[claude_context.display]]
threshold = 50
style = "bold yellow"

[[claude_context.display]]
threshold = 75
style = "bold orange"

[[claude_context.display]]
threshold = 90
style = "bold red"
```

### Claude Cost

The `claude_cost` module displays the total cost of the current Claude Code session in USD. Like `claude_context`, it supports threshold-based styling.

#### Opciones

| Opción     | Predeterminado                     | Descripción                         |
| ---------- | ---------------------------------- | ----------------------------------- |
| `format`   | `'[$symbol(\\$$cost)]($style) '` | El formato del módulo.              |
| `symbol`   | `'💰 '`                             | The symbol shown before the cost.   |
| `display`  | [see below](#display-1)            | Threshold and style configurations. |
| `disabled` | `false`                            | Disables the `claude_cost` module.  |

##### Display

The `display` option is an array of objects that define cost thresholds and styles. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Opción      | Predeterminado | Descripción                                                   |
| ----------- | -------------- | ------------------------------------------------------------- |
| `threshold` | `0.0`          | The minimum cost in USD to match this configuration           |
| `style`     | `bold green`   | The value of `style` if this display configuration is matched |
| `hidden`    | `false`        | Hide this module if this configuration is matched.            |

**Default configuration:**

```toml
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 1.0
style = "bold yellow"

[[claude_cost.display]]
threshold = 5.0
style = "bold red"
```

#### Variables

| Variable      | Ejemplo  | Descripción                                           |
| ------------- | -------- | ----------------------------------------------------- |
| cost          | `1.23`   | Total session cost in USD (formatted to 2 decimals)   |
| duration      | `1m 30s` | Total session duration                                |
| api_duration  | `45s`    | Total API call duration                               |
| lines_added   | `1.2k`   | Total lines of code added                             |
| lines_removed | `500`    | Total lines of code removed                           |
| symbol        |          | Refleja el valor de la opción `symbol`                |
| style\*     |          | Mirrors the style from the matching display threshold |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

#### Ejemplos

```toml
# ~/.config/starship.toml

# Cost with code change statistics
[claude_cost]
format = "[$symbol$cost (+$lines_added -$lines_removed)]($style) "

# Hide module until cost exceeds $0.10
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 0.10
style = "bold yellow"

[[claude_cost.display]]
threshold = 2.0
style = "bold red"

# Show duration information
[claude_cost]
format = "[$symbol$cost ($duration)]($style) "
```

## Cadenas de Estilo

Las cadenas de estilo son una lista de palabras, separadas por espacios en blanco. Las palabras no son sensibles a mayúsculas (es decir, `bold` y `BoLd` se consideran la misma cadena). Cada palabra puede ser una de las siguientes:

- `negrita`
- `cursiva`
- `subrayado`
- `atenuado`
- `invertido`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `ninguno`

donde `<color>` es un especificador de color (discutido a continuación). `fg:<color>` y `<color>` hacen actualmente lo mismo, aunque esto puede cambiar en el futuro. `<color>` también se puede configurar como `prev_fg` o `prev_bg`, que evalúa el color de primer plano o de fondo del elemento anterior respectivamente si está disponible o `none` en caso contrario. `inverted` cambia el fondo y los colores de primer plano. El orden de las palabras en la cadena no importa.

El token `none` anula todos los demás tokens en una cadena si no es parte de un especificador `bg:`, de modo que por ejemplo `fg:red none fg:blue` creará una cadena sin ningún estilo. `bg:none` establece el fondo al color por defecto, así que `fg:red bg:none` es equivalente a `red` o `fg:red` y `bg:green fg:red bg:none` también es equivalente a `fg:red` o `red`. Puede convertirse en un error usar `none` junto con otros estilos en el futuro.

Un especificador de color puede ser uno de los siguientes:

- Uno de los colores estándar del terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Opcionalmente puede prefijar estos con `bright-` para obtener la versión brillante (por ejemplo, `bright-white`).
- Un `#` seguido de un número hexadecimal de seis dígitos. Esto especifica un [código hexadecimal de color RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Un número entre 0-255. Esto especifica un [Código de color ANSI de 8-bits](https://i.stack.imgur.com/KTSQa.png).

Si se especifican varios colores para el primer plano/fondo, el último en la cadena tendrá prioridad.

No todas las cadenas de estilo se mostrarán correctamente en cada terminal. En particular, existen las siguientes rarezas conocidas:

- Muchos terminales deshabilitan el soporte para `parpadear` por defecto.
- `hiden` no es [compatible con iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` no está soportado por macOS Terminal.app por defecto.
