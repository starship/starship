# Configuración Avanzada

A pesar de que Starship es una prompt versátil, a veces necesitas hacer más que editar `starhip.toml` para que haga ciertas cosas. Esta página detalla algunas de las técnicas de configuración más avanzadas en Starship.

::: warning

Las configuraciones de esta sección están sujetas a cambios en futuras versiones de Starship.

:::

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

## TransientPrompt and TransientRightPrompt in Bash

The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework at v0.4 or higher allows you to replace the previous-printed prompt with custom strings. Esto es útil en los casos en que la información del prompt no es siempre necesaria. To enable this, put this in `~/.bashrc` `bleopt prompt_ps1_transient=<value>`:

The \<value\> here is a colon-separated list of `always`, `same-dir` and `trim`. When `prompt_ps1_final` is empty and this option has a non-empty value, the prompt specified by `PS1` is erased on leaving the current command line. If the value contains a field `trim`, only the last line of multiline `PS1` is preserved and the other lines are erased. Otherwise, the command line will be redrawn as if `PS1=` is specified. When a field `same-dir` is contained in the value and the current working directory is different from the final directory of the previous command line, this option `prompt_ps1_transient` is ignored.

Make the following changes to your `~/.bashrc` to customize what gets displayed on the left and on the right:

- To customize what the left side of input gets replaced with, configure the `prompt_ps1_final` Ble.sh option. For example, to display Starship's `character` module here, you would do

```bash
bleopt prompt_ps1_final="$(starship module character)"
```

- To customize what the right side of input gets replaced with, configure the `prompt_rps1_final` Ble.sh option. Por ejemplo, para mostrar la hora en la que se inició el último comando aquí, lo harías

```bash
bleopt prompt_rps1_final="$(starship module time)"
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

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Note: The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework v0.4 or higher should be installed in order to use right prompt in bash.

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

# Un prompt de continuación que muestra dos flechas rellenas
continuation_prompt = '▶▶ '
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

donde `<color>` es un especificador de color (discutido a continuación). `fg:<color>` y `<color>` hacen actualmente lo mismo, aunque esto puede cambiar en el futuro. `inverted` cambia el fondo y los colores de primer plano. El orden de las palabras en la cadena no importa.

El token `none` anula todos los demás tokens en una cadena si no es parte de un especificador `bg:`, de modo que por ejemplo `fg:red none fg:blue` creará una cadena sin ningún estilo. `bg:none` establece el fondo al color por defecto, así que `fg:red bg:none` es equivalente a `red` o `fg:red` y `bg:green fg:red bg:none` también es equivalente a `fg:red` o `red`. Puede convertirse en un error usar `none` junto con otros estilos en el futuro.

Un especificador de color puede ser uno de los siguientes:

- Uno de los colores estándar del terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Opcionalmente puede prefijar estos con `bright-` para obtener la versión brillante (por ejemplo, `bright-white`).
- Un `#` seguido de un número hexadecimal de seis dígitos. Esto especifica un [código hexadecimal de color RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Un número entre 0-255. Esto especifica un [Código de color ANSI de 8-bits](https://i.stack.imgur.com/KTSQa.png).

Si se especifican varios colores para el primer plano/fondo, el último en la cadena tendrá prioridad.

No todas las cadenas de estilo se mostrarán correctamente en cada terminal. En particular, existen las siguientes rarezas conocidas:

- Muchos terminales deshabilitan el soporte para `parpadear` por defecto
- `hiden` no es [compatible con iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` no está soportado por macOS Terminal.app por defecto
