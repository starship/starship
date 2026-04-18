[프리셋으로 돌아가기](./#catppuccin-powerline)

# Catppuccin Powerline 프리셋

이 프리셋은 [Catppuccin](https://github.com/catppuccin/catppuccin) 테마 팔레트를 사용하여 [Gruvbox Rainbow](./gruvbox-rainbow.md)를 최소한으로 수정한 버전입니다.

![Catppuccin Powerline 프리셋 스크린샷](/presets/img/catppuccin-powerline.png)

### 준비 사항

- 터미널에 [Nerd Font](https://www.nerdfonts.com/) 설치 및 활성화

### 설정

```sh
starship preset catppuccin-powerline -o ~/.config/starship.toml
```

기본적으로 이 프리셋은 Catppuccin의 Mocha 플레이버를 사용하지만, `palette` 값을 수정하여 다음 플레이버 중 하나를 지정할 수 있습니다.

- `catppuccin_mocha`
- `catppuccin_frappe`
- `catppuccin_macchiato`
- `catppuccin_latte`

[클릭하여 TOML 다운로드](/presets/toml/catppuccin-powerline.toml)

<<< @/public/presets/toml/catppuccin-powerline.toml
