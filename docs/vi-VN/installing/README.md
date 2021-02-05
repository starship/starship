# 🚀 Cài đặt nâng cao

Để cài đặt starship, bạn cần làm 2 thứ:

1. Lấy tệp tin **starship** nhị phân về máy tính của bạn
1. Nói với shell của bạn để sử dụng tệp tin starship nhị phân như là prompt của nó bằng việc chỉnh sửa những đoạn mã khởi tạo của nó

Đối với đa số người dùng, các hướng dẫn trên [trang chính](/guide/#🚀-installation) sẽ làm việc tốt. Tuy nhiên, với một vài nền tảng đặc biệt hơn, các hướng dẫn khác nhau là cần thiết.

Có rất nhiều nền tảng bên ngoài, rằng chúng đã không khớp như trong tệp tin README.md, do đó đây là vài hướng dẫn cài đặt cho những nền tảng khác đến từ cộng đồng. Của bạn không có ở đây? Xin hãy thêm nó vào đây nếu bạn tìm ra nó!

## [Nix](https://nixos.wiki/wiki/Nix)

### Lấy tệp tin nhị phân

#### Lệnh

```sh
nix-env -iA nixos.starship
```

#### Khai báo, người dùng đơn, thông qua [home-manager](home-manager)

Thêm `pkgs.starship` vào `home.packages` trong tệp tin `home.nix` của bạn, sau đó chạy

```sh
home-manager switch
```

#### Khai báo, system-wide, với NixOS

Thêm `pkgs.starship` vào `environment.packages` trong `configuration.nix` của bạn, sau đó chạy

```sh
sudo nixos-rebuild switch
```

### Sửa những đoạn mã khởi tạo

#### Với Nix và home-manager, sử dụng zsh:

Thêm phần sau vào `programs.zsh.initExtra` trong tệp tin `home.nix` của bạn, sau đó chạy

```sh
home-manager switch
```
