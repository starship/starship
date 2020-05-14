# FAQ

## GIF樣品是用了哪個設定檔？
- **Terminal Emulator**: [iTerm2](https://iterm2.com/)
  - 主題**︰ Minimal
  - **色彩主題︰[Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **安型︰[Fira Code](https://github.com/tonsky/FiraCode)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **設定檔︰[matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/master/.config/fish/config.fish)
  - 提示字符︰[Starship](https://starship.rs/)

## `prompt_order`與`<module>.disabled`是相同的東西嗎？

是的，它們皆為取消提示字符模組的方法。如果你打算去關閉模組`<module>.disabled`是比較建議的方法，原因是︰

- 從`prompt_order`中明確地關閉模組
- Starship更新時，新加入的模組可以自動加入提示字符中

## 文件中說Starship是跨shell的，但為什麼不支持X shell？

建造Starship的方式應該幾乎可以支持任何Shell。 Starship二進製文件是無狀態的，並且與Shell無關，因此，只要您的Shell支持快速自定義和Shell擴展，就可以使用Starship。

此為一個Starship支技bash的範例設定

```sh
# 從最後一個命令中拿到狀態碼
STATUS=$?

# 從在執行的工作數目
NUM_JOBS=$(jobs -p | wc -l)

# 在提示字符中設定為`starship prompt`的輸出結果
PS1="$(starship prompt --status=$STATUS --jobs=NUM_JOBS)"
```

這個內建在starship的[Bash設定檔](https://github.com/starship/starship/blob/master/src/init/starship.bash)是稍微有些複雜的，並且使用了[Command Duration module](https://starship.rs/config/#Command-Duration)來確保跟原生Bash設定相容。

用下列命令列出所有`starship prompt`可接受的標誌

```sh
starship prompt --help
```

這個命令提示符將使用更多的上下文，但不需要標誌

## 我如何在使用舊版glibc的Linux發行版上使用Starship？

如果你使用預編譯的執行檔中（例如，在CentOS 6或7中），碰到了類似"*version 'GLIBC_2.18' not found (required by starship)*"錯誤︰

```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- --platform unknown-linux-musl
```
