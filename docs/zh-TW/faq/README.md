# 常見問答

## 主頁示例圖中的效果是使用哪些配置達成的？

- **終端模擬器**: [iTerm2](https://iterm2.com/)
  - **模擬器主題**: Minimal
  - **配色方案**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **字型**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **fish 設定**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Prompt**: [Starship](https://starship.rs/)

## 我該如何做出示例圖中的命令自動補齊效果？

有關補全或是自動補齊的效果，主要是借助你使用的 Shell 本身提供的服務來達成。 而示例中環境是使用 [Fish Shell](https://fishshell.com/)，原生就提供了補全功能。 如果你的環境是使用 Z Shell (zsh)，建議參考一下 [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions)。

## Do top level `format` and `<module>.disabled` do the same thing?

對，他們都可以被用來關閉提示字元中的 module。 如果你單純只是想關閉 modules，推薦使用 `<module>.disabled`，原因如下所述：

- 明確性：關閉 modules 的動作比在 top level `format` 標記忽略更爲清楚易懂
- 當 Starship 更新後，新組件能被自動加入到提示字元中

## The docs say Starship is cross-shell. Why isn't my preferred shell supported?

Starship 構建方式基本上確立了他應當能支援所有 shell 的基礎。 Starship 的執行檔是不會紀錄狀態且不假設底下是哪種 Shell 的，所以只要你的 Shell 支援客製化命令提示字元以及 shell expansion，就應該要能使用 Starship。

以下是在 bash 上使用 Starship 的簡單例子：

```sh
# Get the status code from the last command executed
STATUS=$?

# Get the number of jobs running.
NUM_JOBS=$(jobs -p | wc -l)

# Set the prompt to the output of `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

順帶一提，其中 Starship [針對 Bash shell 的實作](https://github.com/starship/starship/blob/master/src/init/starship.bash) 稍微複雜一點，實踐了一些如 [Command Duration module](https://starship.rs/config/#command-duration) 的進階功能，也實踐了確保 Starship 設定能夠與系統上的 Bash 相兼容的功能。

使用以下指令來獲得 `starship prompt` 支援的所有參數。

```sh
starship prompt --help
```

The prompt will use as much context as is provided, but no flags are "required".

## How do I run Starship on Linux distributions with older versions of glibc?

If you get an error like "_version 'GLIBC_2.18' not found (required by starship)_" when using the prebuilt binary (for example, on CentOS 6 or 7), you can use a binary compiled with `musl` instead of `glibc`:

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --platform unknown-linux-musl
```

## I see symbols I don't understand or expect, what do they mean?

If you see symbols that you don't recognise you can use `starship explain` to explain the currently showing modules.

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

- Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. If `LC_ALL` is not a UTF-8 value, [you will need to change it](https://www.tecmint.com/set-system-locales-in-linux/).
- You have an emoji font installed. Most systems come with an emoji font by default, but some (notably Arch Linux) do not. You can usually install one through your system's package manager--[noto emoji](https://www.google.com/get/noto/help/emoji/) is a popular choice.
- You are using a [Nerd Font](https://www.nerdfonts.com/).

To test your system, run the following commands in a terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## 我要如何從電腦中移除 Starship？

移除 Starship 的過程與安裝過程一樣簡單。

1. 刪除 Shell 的設定檔案 (比如 `~/.bashrc`) 中用來初始化 Starship 的部分。
1. 刪除 Starship 的執行檔

如果你是透過套件管理器安裝 Starship 的，請到套件管理器的文件中尋找相關的移除步驟指示。

如果你是透過安裝腳本來安裝 Starship 的，可以執行以下的命令來移除執行檔。

```sh
# Locate and delete the starship binary
sh -c 'rm "$(which starship)"'
```
