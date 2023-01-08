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

Clink allows you to replace the previous-printed prompt with custom strings. Esto es útil en los casos que toda la información de la entrada no es siempre necesaria. To enable this, run `clink set prompt.transient <value>` where \<value\> can be one of:

- `always`: always replace the previous prompt
- `same_dir`: replace the previous prompt only if the working directory is same
- `off`: do not replace the prompt (i.e. turn off transience)

You need to do this only once. Make the following changes to your `starship.lua` to customize what gets displayed on the left and on the right:

- Por defecto, el lado izquierdo de la entrada es reemplazado por `>`. To customize this, define a new function called `starship_transient_prompt_func`. This function receives the current prompt as a string that you can utilize. Por ejemplo, para mostrar el módulo `character` de Starship aquí, harías

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- Por defecto, el lado derecho de la entrada está vacío. Para personalizar esto, defina una nueva función llamada `starship_transient_rprompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display the time at which the last command was started here, you would do

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt and TransientRightPrompt in Fish

Es posible reemplazar la entrada impresa anteriormente con una cadena personalizada. Esto es útil en los casos que toda la información de la entrada no es siempre necesaria. To enable this, run `enable_transience` in the shell session. To make it permanent, put this statement in your `~/.config/fish/config.fish`. Transience can be disabled on-the-fly with `disable_transience`.

Note that in case of Fish, the transient prompt is only printed if the commandline is non-empty, and syntactically correct.

- By default, the left side of input gets replaced with a bold-green `❯`. To customize this, define a new function called `starship_transient_prompt_func`. Por ejemplo, para mostrar el módulo `character` de Starship aquí, harías

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- Por defecto, el lado derecho de la entrada está vacío. Para personalizar esto, defina una nueva función llamada `starship_transient_rprompt_func`. For example, to display the time at which the last command was started here, you would do

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
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
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## Habilitar Prompt a la Derecha

Algunos intérpretes de comandos soportan un prompt derecho que se renderiza en la misma línea que la entrada. Starship puede establecer el contenido del prompt derecho usando la opción `right_format`. Cualquier módulo que pueda ser usado en `format` también es soportado en `right_format`. La variable `$all` solo contendrá módulos no utilizados explícitamente en `format` o `right_format`.

Nota: El prompt derecho es una sola línea siguiendo la ubicación de entrada. Para alinear los módulos arriba de la línea de entrada en un prompt multi-línea, vea el [módulo `fill`](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell.

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

Starship puede establecer el prompt de continuación usando la opción `continuation_prompt`. The default prompt is `'[∙](bright-black) '`.

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

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app
