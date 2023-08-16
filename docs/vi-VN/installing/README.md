# Cài đặt nâng cao

Để cài đặt starship, bạn cần làm 2 thứ:

1. Lấy tệp tin **starship** nhị phân về máy tính của bạn
1. Nói với shell của bạn để sử dụng tệp tin starship nhị phân như là prompt của nó bằng việc chỉnh sửa những đoạn mã khởi tạo của nó

Đối với đa số người dùng, các hướng dẫn trên [trang chính](/guide/#🚀-installation) sẽ làm việc tốt. Tuy nhiên, với một vài nền tảng đặc biệt hơn, các hướng dẫn khác nhau là cần thiết.

Có rất nhiều nền tảng bên ngoài, rằng chúng đã không khớp như trong tệp tin README.md, do đó đây là vài hướng dẫn cài đặt cho những nền tảng khác đến từ cộng đồng. Của bạn không có ở đây? Xin hãy thêm nó vào đây nếu bạn tìm ra nó!

## [Chocolatey](https://chocolatey.org)

### Yêu cầu

Head over to the [Chocolatey installation page](https://chocolatey.org/install) and follow the instructions to install Chocolatey.

### Cài đặt

```powershell
choco install starship
```

## [termux](https://termux.com)

### Yêu cầu

```sh
pkg install getconf
```

### Cài đặt

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Cài đặt

On Funtoo Linux, starship can be installed from [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) via Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Lấy tệp tin nhị phân

#### Lệnh

```sh
nix-env -iA nixos.starship
```

#### Khai báo, người dùng đơn, thông qua [home-manager](https://github.com/nix-community/home-manager)

Kích hoạt mô đun `programs.starship` trong tệp `home.nix` của bạn, và thêm các cài đặt của bạn

```nix
{
  programs.starship = {
    enable = true;
    # Configuration written to ~/.config/starship.toml
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

sau đó chạy

```sh
home-manager switch
```

#### Khai báo, system-wide, với NixOS

Thêm`pkgs.starship` vào `environment.systemPackages` trong `configuration.nix` của bạn, sau đó chạy

```sh
sudo nixos-rebuild switch
```
