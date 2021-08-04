# Gelişmiş Yapılandırma

Starship çok yönlü bir kabuk olsa da, yine de belirli şeyleri yapması için `starship.toml`'u düzenlemek gerekebilir. Bu sayfa, Starship'i daha fazla yapılandırma tekniklerini anlatır.

::: uyarı

Bu bölümdeki yapılandırmalar, Starship'in gelecekteki sürümlerinde değişiklik gösterebilir.

:::

## Bash'e Özel ön-komut istemi(pre-prompt) ve ön-çalıştırma(pre-execution) Komutları

Bash, çoğu diğer kabuklar gibi resmi bir preexec/precmd çerçevesine sahip değildir. Bu yüzden, `Bash'i` tamamen özelleştirmek zordur. Ancak Starship, size istem oluşturma prosedürü sayesinde kendi işlevlerinizi ekleme konusunda sınırlı bir yetenek sağlar:

- Özel bir işlemi başlatmak için çizimin bitmesinden önce yeni bir işlev oluşturup adlandırmanız gerekmektedir.`starship_precmd_user_func`. Örneğin komut isteminden önce bir roket çizmek isterseniz

```bash
echo "🚀"
}
starship_precmd_user_func="fırlatıldı"
```

- To run a custom function right before a command runs, you can use the [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). However, you **must** trap the DEBUG signal *before* initializing Starship! Starship can preserve the value of the DEBUG trap, but if the trap is overwritten after starship starts up, some functionality will break.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
eval $(starship init bash)
```

## Change Window Title

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish even does it by default. Starship does not do this, but it's fairly straightforward to add this functionality to `bash` or `zsh`.

First, define a window title change function (identical in bash and zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

You can use variables to customize this title (`$USER`, `$HOSTNAME`, and `$PWD` are popular choices).

In `bash`, set this function to be the precmd starship function:

```bash
starship_precmd_user_func="set_win_title"
```

In `zsh`, add this to the `precmd_functions` array:

```bash
precmd_functions+=(set_win_title)
```

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zshrc`) to make it permanent.

For example, if you want to display your current directory in your terminal tab title, add the following snippet to your `~/.bashrc` or `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

## Style Strings

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
