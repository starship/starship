# Preguntas frecuentes (FAQ)

## ¿Cuál es la configuración usada en el GIF de demostración?

- **Emulador de terminal**: [iTerm2](https://iterm2.com/)
  - **Tema**: Mínimo
  - **Esquema de color**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Font**: [Fira Code](https://github.com/tonsky/FiraCode)
- **Interfaz de línea de comandos**: [Fish Shell](https://fishshell.com/)
  - **Configuración**: [archivos de configuración de matchai](https://github.com/matchai/dotfiles/blob/master/.config/fish/config.fish)
  - **Prompt**: [Starship](https://starship.rs/)

## ¿`prompt_order` y `<module>.disabled` hacen lo mismo?

Sí, se pueden usar ambos para desactivar los módulos en el símbolo del sistema. Si todo lo que planeas es desactivar módulos, `<module>.disabled` es el método preferido por las siguientes razones:

- Es más evidente desactivar módulos que omitirlos usando prompt_order
- Los nuevos módulos se añadirán al símbolo del sistema en cuanto Starship se actualice

## La documentación dice que Starship es compatible con cualquier intérprete de comandos pero no soporta X Shell. ¿Por qué?

Por la forma en que Starshp está construído, debería ser posible añadir soporte para prácticamente cualquier intérprete de comandos. El binario de Starship es sin estado y agnóstico, por lo que mientras que tu intérprete de comandos se pueda ampliar y soporte la personalización del símbolo del sistema, puede utilizar Starship.

Aquí tienes un pequeño ejemplo haciendo que Starship funcione con bash:

```sh
# Obtener el código de estado del último comando ejecutado
STATUS=$?

# Calcula el número de tareas ejecutándose.
NUM_JOBS=$(jobs -p | wc -l)

# Actualiza el símbolo del sistema para mostrar la salida de `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=NUM_JOBS)"
```

La [implementación de Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) en Starship es sensíblemente is ligeramente más compleja para permitir características avanzadas como [el módulo de duración de comando](https://starship.rs/config/#Command-Duration) y para asegurar que Starship es compatible con las configuraciones preinstaladas de Bash.

Para obtener una lista de todos los parámetros aceptados por `el símbolo del sistema starship`, usa el siguiente comando:

```sh
starship prompt --help
```

El símbolo de sistema usará tanto contexto como le proveas, pero no hay parámetros "obligatorios".

## ¿Cómo lanzo Starship en distribuciones Linux con versiones antiguas de glibc?

Si obtienes un error como "*version 'GLIBC_2.18' not found (required by starship)*" al usar el binario precompilado (por ejemplo, en CentOS 6 o 7), puedes usar el binario compilado con `musl` en vez de `glibc`:

```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- --platform unknown-linux-musl
```
