# Preguntas frecuentes

## ¿Cuál es la configuración usada en el GIF de demostración?

- **Emulador de terminal**: [iTerm2](https://iterm2.com/)
  - **Tema**: Mínimo
  - **Esquema de color**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Font**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Intérprete de comandos**: [Fish Shell](https://fishshell.com/)
  - **Configuración**: [archivos de configuración de matchai](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Prompt**: [Starship](https://starship.rs/)

## ¿Cómo obtengo el autocompletado del comando como se muestra en el GIF?

El soporte de terminación, o autocompletado, es proporcionado por tu intérprete de comandos de elección. En el caso de la demo, la demo se realizó con [Fish Shell](https://fishshell.com/), que proporciona el completado por defecto. Si usas Z Shell (zsh), te sugerimos echar un vistazo a [zsh-autosuggeries,](https://github.com/zsh-users/zsh-autosuggestions).

## ¿`prompt_order` y `<module>.disabled` hacen lo mismo?

Sí, se pueden usar ambos para desactivar los módulos en el prompt. Si todo lo que planeas es desactivar módulos, `<module>.disabled` es el método preferido por las siguientes razones:

- Deshabilitar módulos es más explícito que omitirlos del nivel superior `format`
- Los nuevos módulos se añadirán al prompt en cuanto Starship se actualice

## La documentación dice que Starship es "cross-shell". ¿Por qué no es compatible mi intérprete de comandos preferido?

Por la forma en que Starship está construído, debería ser posible añadir soporte para prácticamente cualquier intérprete de comandos. El binario de Starship es sin estado y agnóstico, por lo que mientras que tu intérprete de comandos se pueda ampliar y soporte la personalización del prompt, puedes utilizar Starship.

Aquí tienes un pequeño ejemplo haciendo que Starship funcione con bash:

```sh
# Obtener el código de estado del último comando ejecutado
STATUS=$?

# Calcula el número de tareas ejecutándose.
NUM_JOBS=$(jobs -p | wc -l)

# Establece el prompt a la salida de `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

La [implementación de Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) en Starship es sensíblemente is ligeramente más compleja para permitir características avanzadas como [el módulo de duración de comando](https://starship.rs/config/#command-duration) y para asegurar que Starship es compatible con las configuraciones preinstaladas de Bash.

Para obtener una lista de todos los parámetros aceptados por `el prompt de Starship`, usa el siguiente comando:

```sh
starship prompt --help
```

El símbolo de sistema usará tanto contexto como le proveas, pero no hay parámetros "obligatorios".

## ¿Cómo lanzo Starship en distribuciones Linux con versiones antiguas de glibc?

Si obtienes un error como "_version 'GLIBC_2.18' not found (required by starship)_" al usar el binario precompilado (por ejemplo, en CentOS 6 o 7), puedes usar el binario compilado con `musl` en vez de `glibc`:

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## ¿Por qué veo advertencias `Executing command"..." timed out.`?

Starship ejecuta diferentes comandos para obtener la información que se muestra en el prompt, por ejemplo la versión de un programa o el estado actual de git. Para asegurarse que starship no se quede congelado mientras trata de ejecutar estos comandos, establecimos un límite de tiempo, si un comando tarda más tiempo que este límite starship detendrá la ejecución del comando y mostrara la advertencia anterior, este es el comportamiento esperado. Este límite de tiempo es configurable usando la [llave `command_timeout`](/config/#prompt) así que si lo desea puede aumentar el límite de tiempo. También puedes seguir los pasos de depuración a continuación para ver qué comando está siendo lento y ver si puedes optimizarlo. Finalmente, puedes establecer la variable de entorno `STARSHIP_LOG` a `error` para ocultar estas advertencias.

## Veo símbolos que no entiendo ni espero, ¿qué significan?

Si ves símbolos que no reconoces, puedes usar `starship explain` para explicar los módulos que se muestran actualmente.

## Starship está haciendo algo inesperado, ¿cómo puedo depurarlo?

Puede habilitar los registros de depuración usando la variable de entorno `STARSHIP_LOG`. Estos registros pueden ser muy detallados así que a menudo es útil utilizar el comando `module` si estás intentando depurar un módulo en particular, por ejemplo, si está intentando depurar el módulo `rust` podrías ejecutar el siguiente comando para obtener los registros de seguimiento y salida del módulo.

```sh
env STARSHIP_LOG=trace starship module rust
```

Si starship está siendo lento, puedes intentar usar el comando `timings` para ver si hay un módulo en particular o un comando a culpar.

```sh
env STARSHIP_LOG=trace starship timings
```

Esto producirá el registro de rastreo y un desglose de todos los módulos que tomaron más de 1 ms para ejecutarse o producir alguna salida.

Finalmente si encuentras un error puedes usar el comando `bug-report` para crear un issue en GitHub.

```sh
starship bug-report
```

## ¿Por qué no veo un símbolo de glifo en mi prompt?

La causa más común de esto es la mala configuración del sistema. Algunas distribuciones de Linux en particular no vienen con soporte de fuentes listos para usarse. Tienes que garantizar que:

- Tu configuración regional está establecida con un valor UTF-8, como `de_DE.UTF-8` o `ja_JP.UTF-8`. Si `LC_ALL` no es un valor UTF-8, [necesitarás cambiarlo](https://www.tecmint.com/set-system-locales-in-linux/).
- Tienes una fuente emoji instalada. La mayoría de los sistemas vienen con una fuente emoji por defecto, pero algunos (notablemente Arch Linux) no. Generalmente puedes instalar uno a través del gestor de paquetes del sistema --[noto emoji](https://www.google.com/get/noto/help/emoji/) es una elección popular.
- Estás usando un [Nerd Font](https://www.nerdfonts.com/).

Para probar tu sistema, ejecua los siguientes comandos en un terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

La primera línea debe producir un [emoji snake](https://emojipedia.org/snake/), mientras que la segunda debe producir un [símbolo de rama de Powerline(e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

Si cualquiera de los dos símbolos no se puede mostrar correctamente, su sistema todavía está mal configurado. Desafortunadamente, obtener la configuración correcta de las fuentes es a veces difícil. Los usuarios en el Discord pueden ayudar. Si ambos símbolos se muestran correctamente, pero todavía no los ves en Starship, [¡envía un informe de error!](https://github.com/starship/starship/issues/new/choose)

## ¿Cómo puedo desinstalar Starship?

Starship es tan fácil de desinstalar como de instalar en primer lugar.

1. Elimina cualquier línea de tu configuración de intérprete de comandos (por ejemplo, `~/.bashrc`) usada para inicializar Starship.
1. Elimina el binario de Starship.

Si Starship fue instalado usando un gestor de paquetes, por favor refiérete a sus documentos para instrucciones de desinstalación.

Si Starship fue instalado usando el guión de instalación, el siguiente comando eliminará el binario:

```sh
# Localiza y elimina el binario de starship
sh -c 'rm "$(comando -v 'starship')"'
```
