# 常見問答

## 主頁示例圖中的效果是使用哪些配置達成的？

- **終端模擬器**：[iTerm2](https://iterm2.com/)
  - **模擬器主題**：Minimal
  - **配色方案**：[Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **字型**：[FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**：[Fish Shell](https://fishshell.com/)
  - **Fish 設定**：[matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **提示字元**：[Starship](https://starship.rs/)

## 我該如何做出示例圖中的命令自動補齊效果？

有關補全或是自動補齊的效果，主要是借助你使用的 Shell 本身提供的服務來達成。 而示例中環境是使用 [Fish Shell](https://fishshell.com/)，原生就提供了補全功能。 如果你的環境是使用 Z Shell (zsh)，建議參考一下 [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions)。

## 最上層的 `format` 與 `<module>.disabled` 的功能一樣嗎？

對，他們都可以被用來關閉提示字元中的 module。 如果你單純只是想關閉 modules，推薦使用 `<module>.disabled`，原因如下所述：

- 明確性：關閉 modules 的動作比在 top level `format` 標記忽略更爲清楚易懂
- 當 Starship 更新後，新組件能被自動加入到提示字元中

## 文件中提到 Starship 是跨 shell 的。 為什麼我偏好的 shell 沒有被支持？

Starship 構建方式基本上確立了他應當能支援所有 shell 的基礎。 Starship 的執行檔是不會紀錄狀態且不假設底下是哪種 Shell 的，所以只要你的 Shell 支援客製化命令提示字元以及 shell expansion，就應該要能使用 Starship。

以下是在 bash 上使用 Starship 的簡單例子：

```sh
# 從最後一個執行的指令獲得當前 status code
STATUS=$?

# 獲得當前正在執行的工作數量
NUM_JOBS=$(jobs -p | wc -l)

# 設置提示字元的輸出為 `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

順帶一提，其中 Starship [針對 Bash shell 的實作](https://github.com/starship/starship/blob/master/src/init/starship.bash) 稍微複雜一點，實踐了一些如 [Command Duration module](https://starship.rs/config/#command-duration) 的進階功能，也實踐了確保 Starship 設定能夠與系統上的 Bash 相兼容的功能。

使用以下指令來獲得 `starship prompt` 支援的所有參數。

```sh
starship prompt --help
```

Starship prompt 會盡可能的使用被提供的上下文參數，但使用者也不一定要提供任何參數。

## 我要如何在一些配有更舊版本 glibc 的 Linux 發行版上執行 Starship？

如果在你使用的環境 (比如：CentOS 6 或 7) 下使用預編好的 Starship 執行檔時會產生一些像 "_version 'GLIBC_2.18' not found (required by starship)_" 的內容，你可以在執行安裝指令時嘗試選用不同的函式庫預先編譯而成的 Starship 版本，比如說 `musl` 而非 `glibc`，如下所示：

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
starship bug-report
```

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

- 你的當地語言設置為 UTF-8，如 `de_DE.UTF-8` 或 `ja_JP.UTF-8`。 如果 `LC_ALL` 不是 UTF-8，[你會需要改變他](https://www.tecmint.com/set-system-locales-in-linux/)。
- 你已經安裝一個表情符號字體。 大多數的系統在預設情況下皆有支持表情符號字體，然後有些（尤其 Arch Linux）沒有。 你通常可以透過系統的套件管理器安裝一個表情符號字體 —— [noto emoji](https://www.google.com/get/noto/help/emoji/) 是個受歡迎的選擇。
- 你正在使用 [Nerd Font](https://www.nerdfonts.com/)。

To test your system, run the following commands in a terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## How do I uninstall Starship?

Starship is just as easy to uninstall as it is to install in the first place.

1. 刪除 Shell 的設定檔案 (比如 `~/.bashrc`) 中用來初始化 Starship 的部分。
1. 刪除 Starship 的執行檔

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

If Starship was installed using the install script, the following command will delete the binary:

```sh
# Locate and delete the starship binary
sh -c 'rm "$(which starship)"'
```
