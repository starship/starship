# Advanced Configuration

While Starship is a versatile shell, sometimes you need to do more than edit
`starship.toml` to get it to do certain things. This page logs some of the more
advanced configuration techniques used in starship.

::: warning
The configurations in this section are subject to change in future releases of Starship.
:::

## Custom pre-prompt and pre-execution Commands in Bash

Bash does not have a formal preexec/precmd framework like most other shells.
Because of this, it is difficult to provide fully customizable hooks in `bash`.
However, Starship does give you limited ability to insert your own functions
into the prompt-rendering procedure:

- To run a custom function right before the prompt is drawn, define a new
  function and then assign its name to `starship_precmd_user_func`. For example,
  to draw a rocket before the prompt, you would do

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the 
  [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/).
  However, you **must** trap the DEBUG signal *before* initializing Starship!
  Starship can preserve the value of the DEBUG trap, but if the trap is overwritten
  after starship starts up, some functionality will break.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
eval $(starship init bash)
```

## Change Window Title

Some shell prompts will automatically change the window title for you (e.g. to 
reflect your working directory). Fish even does it by default.
Starship does not do this, but it's fairly straightforward to add this
functionality to `bash` or `zsh`.

First, define a window title change function (identical in bash and zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

You can use variables to customize this title (`$USER`, `$HOSTNAME`, and `$PWD`
are popular choices).

In `bash`, set this function to be the precmd starship function:

```bash
starship_precmd_user_func="set_win_title"
```

In `zsh`, add this to the `precmd_functions` array:

```bash
precmd_functions+=(set_win_title)
```

If you like the result, add these lines to your shell configuration file 
(`~/.bashrc` or `~/.zsrhc`) to make it permanent.