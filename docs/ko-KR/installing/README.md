# 고급 설치

Starship을 설치하려면 다음 두 가지를 수행해주세요

1. **Starship** 바이너리를 당신의 컴퓨터에 받으세요
2. 셸의 초기화 스크립트를 수정하여 starship 바이너리를 프롬프트로 사용하도록 셸에 지시하세요.

대부분의 사용자에게는 [메인 페이지](../guide/#🚀-installation)의 지침이 잘 작동할 것입니다. 그러나 일부 더 전문화된 플랫폼의 경우 다른 지침이 필요합니다.

너무 많은 플랫폼이 있어서 메인 README.md 파일에 모두 포함할 수 없었으므로, 다음은 커뮤니티에서 제공하는 다른 플랫폼에 대한 설치 지침입니다. Is yours not here? Please do add it here if you figure it out!

## [Chocolatey](https://chocolatey.org)

### 준비 사항

[Chocolatey 설치 페이지](https://chocolatey.org/install)로 이동하여 Chocolatey 설치 지침을 따르세요.

### 설치

```powershell
choco install starship
```

## [termux](https://termux.com)

### 설치

```sh
pkg install starship
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### 설치

Funtoo Linux에서는 Portage를 통해 [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship)에서 starship을 설치할 수 있습니다.

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### Getting the Binary

#### Imperatively

```sh
nix-env -iA nixos.starship
```

#### 선언형, 단일 사용자, [home-manager](https://github.com/nix-community/home-manager)를 통해

`home.nix` 파일에서 `programs.starship` 모듈을 활성화하고 설정을 추가하세요.

```nix
{
  programs.starship = {
    enable = true;
    # ~/.config/starship.toml에 작성된 설정
    settings = {
      # add_newline = false;

      # character = {
      #   success_symbol = "[➜](bold green)";
      #   error_symbol = "[➜](bold red)";
      # };

      # package.disabled = true;
    };
  };
}
```

그런 다음 실행하세요.

```sh
home-manager switch
```

#### 선언형, 시스템 전체, NixOS 사용

`configuration.nix` 파일의 `environment.systemPackages`에 `pkgs.starship`을 추가한 다음 실행하세요.

```sh
sudo nixos-rebuild switch
```
