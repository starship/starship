# 進階設定

正因為 Starship 是一個多才多藝的 shell，有時候你必須要做比修改 `starship.toml` 更多事情來讓它完成特定工作。 這個頁面說明了一些用於 Starship 的進階設定技巧。

::: warning

這個章節內的設定可能會隨著未來 Starship 的版本發行而變動。

:::

## Bash 中的自定義預提示 (pre-prompt) 與預執行 (pre-execution) 指令

Bash 不像其他大多的 shell 具有正式的預執行/預指令框架。 因為這個原因，很難在 `bash` 中提供能完全自定義的 hook。 然而，Starship 有提供給你有限的能力來插入你自己的函式到渲染提示字元的程序中：

- 為了在畫出提示字元之前執行一個自定義的函式，請定義一個函式，並將它的名稱放入 `starship_precmd_user_func` 之中。 例如，為了要在提示字元前畫出一個火箭，你就要

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). 然而，你**必須**在初始化 Starship *之前* 對 DEBUG 訊號設下trap！ Starship 可以保留 DEBUG trap 的數值，但是如果該 trap 在 starship 啟動後被被覆寫，某些功能會損壞。

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # 在 Starship 啟用*前*對 DEBUG 訊號設下 trap
eval $(starship init bash)
```

## 改變視窗標題

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish 甚至預設就會這樣做。 Starship 沒有幫你這樣做，但是可以用直覺的方式加入這個功能到 `bash` 或 `zsh` 之中。

首先，定義一個改變視窗標題的函式（在 bash 與 zsh 之中都一樣）：

```bash
function set_win_title(){
    echo -ne "\033]0; 你的標題在此 \007"
}
```

你可以利用變數來自定義這個標題（`$USER`、`$HOSTNAME` 與 `$PWD` 是很受歡迎的選項）。

在 `bash` 中，將這個函式設定為 Starship 的預執行函式：

```bash
starship_precmd_user_func="set_win_title"
```

在 `zsh` 中，將這個函式加入 `precmd_functions` 陣列：

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

## 風格字串

風格字串是一個以空白分開的單詞清單。 單字並不會區分大小寫（換句話說，`bold` 與 `BoLd` 是被當作兩個相同的字串）。 每個單詞可以是下列其中之一：

  - `bold`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

其中 `<color>` 是指定顏色用的（下面解釋）。 `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. 未來可能會將 `none` 與其他符號一起使用的情形視為是一種錯誤。

一個顏色指定符號可以是下列其中之一：

 - 任一個標準終端機顏色：`black`、`red`、`green`、`blue`、`yellow`、`purple`、`cyan`、`white`。 你可以選擇性地加上前綴 `bright-` 來取得明亮版本的顏色（例如：`bright-white`）。
 - 一個 `#` 後面跟隨著六位數的十六進位數字。 這個指定了 [RGB 十六進制色碼](https://www.w3schools.com/colors/colors_hexadecimal.asp)。
 - 一個介於 0~255 之間的數字。 這個指定了 [8-bit ANSI 色碼](https://i.stack.imgur.com/KTSQa.png)。

如果前景/後景被指定了多種顏色，最後一個顏色具有最高優先性。
