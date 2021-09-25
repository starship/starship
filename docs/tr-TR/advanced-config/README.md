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

- Özel bir işlemi başlatmadan hemen önce komut istemini çalıştırıp, [`DEBUG`filtreleme mekanizmasını](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/) kullanabilirsiniz. Bununla birlikte, Starship başlatılmadan hemen *önce*, </strong>DEBUG sinyalini filtrelemek<1>**zorundasınız.</0>! Starship, DEBUG filtrelemesinin ardından bazı değerleri içerisinde barındırabilir ancak filtreleme işlemi starship başlatıldıktan sonra yazılırsa bazı fonksiyonlar devre dışı kalabilir.</li> </ul>

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
eval $(starship init bash)
```

## Pencere Başlığını Değiştirme

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

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh.

### Example

```toml
# ~/.config/starship.toml

# A minimal left prompt
format = """$character"""

# move the rest of the prompt to the right
right_format = """$all"""
```

Produces a prompt like the following:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
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
