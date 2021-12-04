# Preguntas frecuentes (FAQ)

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

La [implementación de Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) integrada en Starship es un poco más compleja para permitir funciones avanzadas como el [módulo Duración del Comando](https://starship.rs/config/#command-duration) y para garantizar que Starship sea compatible con las configuraciones de Bash preinstaladas.

Para obtener una lista de todos los parámetros aceptados por `el prompt de Starship`, usa el siguiente comando:

```sh
starship prompt --help
```

El símbolo de sistema usará tanto contexto como le proveas, pero no hay parámetros "obligatorios".

## ¿Cómo lanzo Starship en distribuciones Linux con versiones antiguas de glibc?

Si obtienes un error como "_version 'GLIBC_2.18' not found (required by starship)_" al usar el binario precompilado (por ejemplo, en CentOS 6 o 7), puedes usar el binario compilado con `musl` en vez de `glibc`:

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --platform unknown-linux-musl
```

## Why do I see `Executing command "..." timed out.` warnings?

Starship executes different commands to get information to display in the prompt, for example the version of a program or the current git status. To make sure starship doesn't hang while trying to execute these commands we set a time limit, if a command takes longer than this limit starship will stop the execution of the command and output the above warning, this is expected behaviour. This time limit is configurable using the [`command_timeout` key](/config/#prompt) so if you want you can increase the time limit. You can also follow the debugging steps below to see which command is being slow and see if you can optimise it. Finally you can set the `STARSHIP_LOG` env var to `error` to hide these warnings.

## I see symbols I don't understand or expect, what do they mean?

If you see symbols that you don't recognise you can use `starship explain` to explain the currently showing modules.

## Starship is doing something unexpected, how can I debug it?

You can enable the debug logs by using the `STARSHIP_LOG` env var. These logs can be very verbose so it is often useful to use the `module` command if you are trying to debug a particular module, for example, if you are trying to debug the `rust` module you could run the following command to get the trace logs and output from the module.

```sh
env STARHIP_LOG=trace starship module rust
```

If starship is being slow you can try using the `timings` command to see if there is a particular module or command that to blame.

```sh
env STARHIP_LOG=trace starship timings
```

This will output the trace log and a breakdown of all modules that either took more than 1ms to execute or produced some output.

Finally if you find a bug you can use the `bug-report` command to create a Github issue.

```sh
reporte de error starship
```

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

- Tu configuración regional está establecida con un valor UTF-8, como `de_DE.UTF-8` o `ja_JP.UTF-8`. Si `LC_ALL` no es un valor UTF-8, [necesitarás cambiarlo](https://www.tecmint.com/set-system-locales-in-linux/).
- Tienes una fuente emoji instalada. La mayoría de los sistemas vienen con una fuente emoji por defecto, pero algunos (notablemente Arch Linux) no. Generalmente puedes instalar uno a través del gestor de paquetes del sistema --[noto emoji](https://www.google.com/get/noto/help/emoji/) es una elección popular.
- Estás usando un [Nerd Font](https://www.nerdfonts.com/).

To test your system, run the following commands in a terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## How do I uninstall Starship?

Starship is just as easy to uninstall as it is to install in the first place.

1. Elimina cualquier línea de tu configuración de intérprete de comandos (por ejemplo, `~/.bashrc`) usada para inicializar Starship.
1. Elimina el binario de Starship.

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

If Starship was installed using the install script, the following command will delete the binary:

```sh
# Locate and delete the starship binary
sh -c 'rm "$(which starship)"'
```
