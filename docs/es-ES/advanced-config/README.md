# Configuración Avanzada

Mientras que Starship es un versátil intérprete de comandos, a veces necesitas más que editar `starhip.toml` para que haga ciertas cosas. Esta página detalla algunas de las técnicas de configuración más avanzadas en starship.

::: aviso

Las configuraciones de esta sección pueden sufrir cambios en futuras versiones de Starship.

:::

## Comandos pre-prompt y pre-ejecucucióne personalizados en Bash

Bash no posee un framework oficial de preexec/precmd como la mayoría de las demás shells. Por lo tanto, es complicado proveer una personalización completa en `bash`. Sin embargo, Starship te da la posibilidad de insertar de forma limitada tus propias funciones en el proceso de renderizado del prompt:

- Para ejecutar una función personalizada previa al renderizado del prompt, defina una nueva función y asigne su nombre a `starship_precmd_user_func`. Por ejemplo, para renderizar un cohete antes del prompt, se puede realizar así:

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Para ejecutar una función personalizada antes de que un comando sea ejecutado, es posible usar el [mecanismo `DEBUG` trap](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). No obstante, ¡es **necesario** atrapar la señal DEBUG *antes* de inicializar Starship! Starship puede preservar el valor del mecanismo, pero si el mecanismo es reemplazado después de que Starship se inicie, algunas funcionalidades fallarán.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
eval $(starship init bash)
```

## Cambiar el título de la ventana

Algunos shell prompts van a cambiar automáticamente el título de la ventana por ti. (Por ejemplo, para mostrar tu directorio actual). Fish incluso lo hace de forma predeterminada. Starship no hace esto, pero es bastante sencillo añadir esta funcionalidad a `bash` o `zsh`.

Primero defina una función para el cambio de titulo de la ventana (idéntico en bash y zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; TU_TÍTULO_DE_VENTANA_AQUÍ \007"
}
```

Puede usar variables para personalizar este titulo (`$USER`, `$HOSTNAME` y `$PWD` son opciones populares).

En `bash`, establezca que esta función sea la función precmd de Starship:

```bash
starship_precmd_user_func="set_win_title"
```

En `zsh`, añade esto al array `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

Si te gusta el resultado, añade estas líneas a tu archivo de configuración del shell (`~/.bashrc` o `~/.zsrhc`) para hacerlo permanente.

## Cadenas de estilo

Las cadenas de estilo son una lista de palabras, separadas por espacios en blanco. Las palabras no son sensibles a mayúsculas (es decir, `negrita` y `NeGriTa` se consideran la misma cadena). Cada palabra puede ser una de las siguientes:

  - `bold`
  - `underline`
  - `dimmed`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

donde `<color>` es un especificador de color (discutido a continuación). `fg:<color>` y `<color>` hacen actualmente lo mismo, aunque esto puede cambiar en el futuro. El orden de las palabras en la cadena no importa.

El token `none` anula todas las otras fichas en una cadena, por lo que, por ejemplo, `fg:red none fg:blue` creará una cadena sin ningún tipo de estilo. Puede convertirse en un error usar `none` junto con otros tokens en el futuro.

Un especificador de color puede ser uno de los siguientes:

 - Uno de los colores estándar del terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Opcionalmente puedes prefijar estos con `bright-` para obtener la versión brillante (por ejemplo, `bright-white`).
 - Un `#` seguido de un número hexadecimal de seis dígitos. Esto especifica un [código hexadecimal de color RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Un número entre 0-255. Esto especifica un [Código ANSI de color de 8-bits](https://i.stack.imgur.com/KTSQa.png).

Si se especifican varios colores para el primer plano/fondo, el último en la cadena tendrá prioridad.
