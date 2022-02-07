# Configuración Avanzada

Mientras que Starship es un prompt versátil, a veces necesitas más que editar `starhip.toml` para que haga ciertas cosas. Esta página detalla algunas de las técnicas de configuración más avanzadas en Starship.

::: aviso

Las configuraciones de esta sección están sujetos a cambios en futuras versiones de Starship.

:::

## Custom pre-prompt and pre-execution Commands in Cmd

Clink provides extremely flexible APIs to run pre-prompt and pre-exec commands in Cmd shell. It is fairly simple to use with Starship. Make the following changes to your `starship.lua` file as per your requirements:

- To run a custom function right before the prompt is drawn, define a new function called `starship_preprompt_user_func`. This function receives the current prompt as a string that you can utilize. For example, to draw a rocket before the prompt, you would do

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- To run a custom function right before a command is executed, define a new function called `starship_precmd_user_func`. This function receives the current commandline as a string that you can utilize. For example, to print the command that's about to be executed, you would do

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

Algunos intérpretes de comandos van a cambiar automáticamente el título de la ventana por ti (p. ej., para mostrar tu directorio actual). Fish incluso lo hace por defecto. Starship does not do this, but it's fairly straightforward to add this functionality to `bash`, `zsh`, `cmd` or `powershell`.

Primero, define una función para el cambio de título de la ventana (idéntico en Bash y zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; TU_TÍTULO_DE_VENTANA_AQUÍ \007"
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

For Cmd, you can change the window title using the `starship_preprompt_user_func` function.

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

## Gabilitar Prompt Derecho

Algunos intérpretes de órdenes soportan un prompt derecho que se renderiza en la misma línea que la entrada. Starship puede establecer el contenido del prompt correcto usando la opción `right_format`. Cualquier módulo que pueda ser usado en `format` también es soportado en `right_format`. La variable `$all` solo contendrá módulos no utilizados explícitamente en `format` o `right_format`.

Nota: El prompt derecho es una sola línea siguiendo la ubicación de entrada. Para alinear módulos arriba de la línea de entrada en un prompt multi-línea, vea el [módulo fill](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd.

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

## Continuation Prompt

Some shells support a continuation prompt along with the normal prompt. This prompt is rendered instead of the normal prompt when the user has entered an incomplete statement (such as a single left parenthesis or quote).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `"[∙](bright-black) "`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Note: Continuation prompts are only available in the following shells:

- `bash`
- `zsh`
- `PowerShell`

### Ejemplo

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled in arrows
continuation_prompt = "▶▶"
```

## Cadenas de estilo

Las cadenas de estilo son una lista de palabras, separadas por espacios en blanco. Las palabras no son sensibles a mayúsculas (es decir, `negrita` y `NeGriTa` se consideran la misma cadena). Cada palabra puede ser una de las siguientes:

- `negrita`
- `cursiva`
- `subrayado`
- `atenuado`
- `invertido`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `ninguno`

donde `<color>` es un especificador de color (discutido a continuación). `fg:<color>` y `<color>` hacen actualmente lo mismo, aunque esto puede cambiar en el futuro. `inverted` cambia el fondo y los colores de primer plano. El orden de las palabras en la cadena no importa.

El token `none` anula todos los demás tokens en una cadena si no es parte de un especificador `bg:`, de modo que por ejemplo `fg:red none fg:blue` creará una cadena sin ningún estilo. `bg:none` establece el fondo al color por defecto, así que `fg:red bg:none` es equivalente a `red` o `fg:red` y `bg:green fg:red bg:none` también es equivalente a `fg:red` o `red`. Puede convertirse en un error usar `none` junto con otros tokens en el futuro.

Un especificador de color puede ser uno de los siguientes:

- One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
- Un `#` seguido de un número hexadecimal de seis dígitos. Esto especifica un [código hexadecimal de color RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Un número entre 0-255. Esto especifica un [Código de color ANSI de 8-bits](https://i.stack.imgur.com/KTSQa.png).

Si se especifican varios colores para el primer plano/fondo, el último en la cadena tendrá prioridad.
