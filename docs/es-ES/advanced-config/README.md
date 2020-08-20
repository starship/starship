# Configuración Avanzada

Mientras que Starship es un prompt versátil, a veces necesitas más que editar `starhip.toml` para que haga ciertas cosas. Esta página detalla algunas de las técnicas de configuración más avanzadas en starship.

::: aviso

Las configuraciones de esta sección están sujetos a cambios en futuras versiones de Starship.

:::

## Comandos pre-prompt y pre-ejecucucióne personalizados en Bash

Bash no posee un framework formal de preexec/precmd como la mayoría de las demás shells. Por lo tanto, es complicado proveer una personalización completa en `bash`. Sin embargo, Starship te da la posibilidad de insertar de forma limitada tus propias funciones en el proceso de renderizado del prompt:

- Para ejecutar una función personalizada previa al renderizado del prompt, defina una nueva función y asigne su nombre a `starship_precmd_user_func`. Por ejemplo, para renderizar un cohete antes del prompt, se puede realizar así:

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Para ejecutar una función personalizada antes de que un comando sea ejecutado, es posible usar el [mecanismo trap `DEBUG`](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). No obstante, ¡**debes** atrapar la señal DEBUG *antes* de inicializar Starship! Starship puede preservar el valor de la trampa DEBUG, pero si el trampa es reemplazada después de que Starship inicie, alguna funcionalidad fallará.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
eval $(starship init bash)
```

## Cambiar el título de la ventana

Algunas shells van a cambiar automáticamente el título de la ventana por ti. (por ejemplo, para mostrar tu directorio actual). Fish incluso lo hace de forma predeterminada. Starship no hace esto, pero es bastante sencillo añadir esta funcionalidad a `bash` o `zsh`.

Primero, define una función para el cambio de titulo de la ventana (idéntico en bash y zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; TU_TÍTULO_DE_VENTANA_AQUÍ \007"
}
```

Puedes usar variables para personalizar este titulo (`$USER`, `$HOSTNAME` y `$PWD` son opciones populares).

En `bash`, establece que esta función sea la función precmd de Starship:

```bash
starship_precmd_user_func="set_win_title"
```

En `zsh`, añade esto al array `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

Si te gusta el resultado, añade estas líneas a tu archivo de configuración del shell (`~/.bashrc` o `~/.zsrhc`) para hacerlo permanente.

Por ejemplo, si quieres mostrar tu directorio actual en el título de la pestaña de la terminal, añade el siguiente fragmento a tu `~/.ashrc` o `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename $PWD) \007"
}
starship_precmd_user_func="set_win_title"
```

## Estilo de cadenas de texto

Los estilos de cadenas de texto son una lista de palabras, separadas por espacios en blanco. Las palabras no son sensibles a mayúsculas (es decir, `negrita` y `NeGriTa` se consideran la misma cadena). Cada palabra puede ser una de las siguientes:

  - `bold`
  - `underline`
  - `dimmed`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

donde `<color>` es un especificador de color (discutido a continuación). `fg:<color>` y `<color>` hacen actualmente lo mismo, aunque esto puede cambiar en el futuro. El orden de las palabras en la cadena no importa.

El estilo `none` anula todas los otros estilos en una cadena de texto, por lo que, por ejemplo, `fg:red none fg:blue` creará una cadena de texto sin ningún tipo de estilo. Puede convertirse en un error usar `none` junto con otros estilos en el futuro.

Un especificador de color puede ser uno de los siguientes:

 - Uno de los colores estándar del terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Opcionalmente puedes agregar el prefijo `bright-` para obtener la versión brillante (por ejemplo, `bright-white`).
 - Un `#` seguido de un número hexadecimal de seis dígitos. Esto especifica un [código hexadecimal de color RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Un número entre 0-255. Esto especifica un [Código de color ANSI de 8-bits](https://i.stack.imgur.com/KTSQa.png).

Si se especifican varios colores para el primer plano/fondo, el último en la cadena tendrá prioridad.
