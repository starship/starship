# Configuración Avanzada

Mientras que Starship es un prompt versátil, a veces necesitas más que editar `starhip.toml` para que haga ciertas cosas. Esta página detalla algunas de las técnicas de configuración más avanzadas en Starship.

::: aviso

Las configuraciones de esta sección están sujetos a cambios en futuras versiones de Starship.

:::

## Comandos pre-prompt y pre-ejecucución personalizados en Bash

Bash no posee un framework oficial de preexec/precmd como la mayoría de los demás intérpretes de comandos. Debido a esto, es difícil proporcionar "hooks" totalmente personalizables en `Bash`. Sin embargo, Starship te da la posibilidad de insertar de forma limitada tus propias funciones en el proceso de renderizado del prompt:

- Para ejecutar una función personalizada previa al renderizado del prompt, define una nueva función y asigna su nombre a `starship_precmd_user_func`. Por ejemplo, para dibujar un cohete antes del prompt, se puede realizar así:

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). No obstante, ¡**debes** atrapar la señal DEBUG *antes* de inicializar Starship! Starship puede preservar el valor de la trampa DEBUG, pero si la trampa es reemplazada después de que Starship inicie, alguna funcionalidad fallará.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trampa DEBUG *antes* de la ejecución de Starship
eval $(starship init bash)
```

## Cambiar el Título de la Ventana

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish incluso lo hace por defecto. Starship no hace esto, pero es bastante sencillo añadir esta funcionalidad a `Bash` o `zsh`.

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

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zshrc`) to make it permanent.

Por ejemplo, si quieres mostrar tu directorio actual en el título de la pestaña de la terminal, añade el siguiente fragmento a tu `~/.bashrc` o `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

## Cadenas de Estilo

Las cadenas de estilo son una lista de palabras, separadas por espacios en blanco. Las palabras no son sensibles a mayúsculas (es decir, `bold` y `BoLd` se consideran la misma cadena). Cada palabra puede ser una de las siguientes:

  - `bold`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

donde `<color>` es un especificador de color (discutido a continuación). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

El token `none` anula todos los demás tokens en una cadena si no es parte de un especificador `bg:`, de modo que por ejemplo `fg:red none fg:blue` creará una cadena sin ningún estilo. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. Puede convertirse en un error usar `none` junto con otros tokens en el futuro.

Un especificador de color puede ser uno de los siguientes:

 - Uno de los colores estándar del terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Opcionalmente puedes agregar el prefijo `bright-` para obtener la versión brillante (por ejemplo, `bright-white`).
 - Un `#` seguido de un número hexadecimal de seis dígitos. Esto especifica un [código hexadecimal de color RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Un número entre 0-255. Esto especifica un [Código de color ANSI de 8-bits](https://i.stack.imgur.com/KTSQa.png).

Si se especifican varios colores para el primer plano/fondo, el último en la cadena tendrá prioridad.
