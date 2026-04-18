[プリセット一覧に戻る](./#catppuccin-powerline)

# Catppuccin Powerline プリセット

[Catppuccin](https://github.com/catppuccin/catppuccin) のテーマを利用して [Gruvbox Rainbow](./gruvbox-rainbow.md) に最小限の変更を加えたプリセットです。

![Catppuccin Powerline プリセットのスクリーンショット](/presets/img/catppuccin-powerline.png)

### 必要なもの

- [Nerd Font](https://www.nerdfonts.com/) をインストールし、ターミナルで有効化する

### 設定

```sh
starship preset catppuccin-powerline -o ~/.config/starship.toml
```

デフォルトでは、このプリセットは Catppuchin の Mocha テーマを使用します。 `palette` の値を変更することで、他のテーマを使用することができます：

- `catppuccin_mocha`
- `catppuccin_frappe`
- `catppuccin_macciato`
- `catppuccin_latte`

[クリックしてTOMLファイルをダウンロード](/presets/toml/catppuccin-powerline.toml)

<<< @/public/presets/toml/catppuccin-powerline.toml
