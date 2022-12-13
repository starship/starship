# Frequently Asked Questions

## Cấu hình sử dụng trong demo GIF là gì?

- **Terminal Emulator**: [iTerm2](https://iterm2.com/)
  - **Theme**: Minimal
  - **Color Scheme**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Font**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **Cấu hình**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Prompt**: [Starship](https://starship.rs/)

## Làm thế nào tôi có được hoàn thiện câu lệnh như được hiển thị trong demo GIF?

Hỗ trợ hoàn thiện, hoặc tự động hoàn thiện, được cung cấp bởi shell của bạn lựa chọn. Trong trường hợp của demo, demo đã thực hiện với [Fish Shell](https://fishshell.com/), thứ mặc định cung cấp những completion. Nếu bạn sử dụng Z Shell (zsh), tôi muốn gợi ý xem qua [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Có phải level trên cùng `format` và `<module>.disable` thực hiện cùng một thứ giống nhau?

Vâng, chúng có thể cùng được sử dụng để vô hiệu hoá các module trong prompt. Nếu tất cả bạn những việc bạn làm để vô hiệu hoá các module, `<module>.disabled` là các được ưa thích hơn để thực hiện với những lí do sau:

- Các module vô hiệu là ro ràng hơn sót chúng từ level trên cùng `format`
- Những module mới tạo sẽ được thêm bào prompt khi Starship được cập nhật

## Các tài liệu nói Starship là cross-shell. Tại sao shell ưa thích của tôi không được hỗ trợ?

Cách Starship được xây dựng, nó nên khả thi để thêm hỗ trợ với bất kì shell ảo nào. Starship nhị phân là phi trạng thái và bất khả tri của shell, vì vậy, miễn là shell của bạn hỗ trợ tùy chỉnh nhanh chóng và mở rộng shell, thì Starship có thể được sử dụng.

Đây là một ví dụ nhỏ để Starship là việc với bash:

```sh
# Lấy mã trạng thái từ câu lệnh cuối đã được thực thi
STATUS=$?

# Lấy số lượng job đang chạy.
NUM_JOBS=$(jobs -p | wc -l)

# Thiết lập prompt thành đầu ra của `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

[Bản cài đặt Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) xây dựng bên trong Starship phực tạp hơn một chút để cho phép thực hiện các tính năng nâng cao hơn một chứt như [Command Duration module](https://starship.rs/config/#command-duration) và chắc chắn rằng Starship là tương thích với cấu hình Bash đã cài đặt trước đó.

Với một danh sách tất cả các cờ đã được chấp nhận bởi `starship prompt`, sử dụng lệnh sau:

```sh
starship prompt --help
```

Prompt sẽ sử dụng nhiều ngữ cảnh được cung câos, nhưng không có cờ nào là "yêu cầu".

## Làm thế nào để tôi chạy Starship trên các bản phân phối Linux với các phiên bản cũ của glibc?

Nếu bạn nhận được một lỗi giống như "_version 'GLIBC_2.18' not found (required by starship)_" khi sử dụng prebuilt binary (ví dụ trên CentOS 6 hoặc 7), bạn có thể sử dụng tập tin đã được dịch với `musl` thay vì `glibc`:

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## Why do I see `Executing command "..." timed out.` warnings?

Starship executes different commands to get information to display in the prompt, for example the version of a program or the current git status. To make sure starship doesn't hang while trying to execute these commands we set a time limit, if a command takes longer than this limit starship will stop the execution of the command and output the above warning, this is expected behaviour. This time limit is configurable using the [`command_timeout`key](/config/#prompt) so if you want you can increase the time limit. You can also follow the debugging steps below to see which command is being slow and see if you can optimise it. Finally you can set the `STARSHIP_LOG` env var to `error` to hide these warnings.

## Tôi thấy các biểu tượng tôi không hiểu hoặc không mong muốn, chúng có nghĩa là gì?

Nếu bạn thấy các biểu tượng bạn không biết, bạn có thể sử dụng `starship explain` để giải thích các mô đun hiện tại đang hiển thị.

## Starship is doing something unexpected, how can I debug it?

You can enable the debug logs by using the `STARSHIP_LOG` env var. These logs can be very verbose so it is often useful to use the `module` command if you are trying to debug a particular module, for example, if you are trying to debug the `rust` module you could run the following command to get the trace logs and output from the module.

```sh
env STARSHIP_LOG=trace starship module rust
```

If starship is being slow you can try using the `timings` command to see if there is a particular module or command that to blame.

```sh
env STARSHIP_LOG=trace starship timings
```

This will output the trace log and a breakdown of all modules that either took more than 1ms to execute or produced some output.

Finally if you find a bug you can use the `bug-report` command to create a Github issue.

```sh
starship bug-report
```

## Tại sao tôi không thấy một biểu tượng glyph trong dấu nhắc lệnh của tôi?

Đa số lí do phổ biến là do cái này mất cấu hình hệ thống. Một số bản phân phối Linux cụ thể không đi kèm việ hõ trợ font out-of-the-box. Bạn cần chắc chắn rằng:

- Mã ngôn ngữ của bạn được thiết lập là một giá trị UTF-8, giống như `de_DE.UTF-8` or `ja_JP.UTF-8`. Nếu `LO_ALL` không phải là một giá trị UTF-8, [ bạn sẽ cần thay đổi nó](https://www.tecmint.com/set-system-locales-in-linux/).
- Bạn đã cài đặt phông chữ biểu tượng cảm xúc. Đa số hệ thống đi kèm với một phông biểu tượng cảm xúc mặc định, nhưng một vài (đáng chú ý là Arch Linux) thì không. Bạn có thể thường cài đặt thông qua một trình quản lí gói hệ thống của bạn--[noto emoji](https://www.google.com/get/noto/help/emoji/) là một lựa chọn phổ biến.
- Bạn đang sử dụng một [Nerd Font](https://www.nerdfonts.com/).

Để kiểm tra hệ thống của bạn, chạy các câu lệnh bên dưới trong terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

Dòng đầu tiên nên sinh ra một [snake emoji](https://emojipedia.org/snake/), trong khi dòng thứ hai nên sinh ra một [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

Nếu biểu tượng không hiển thị đúng, hệ thống của bạn vẫn cấu hình sai. Thật không may, lấy cấu hình font đúng đôi khi khó khăn. Người dùng trên Discord có thể có giúp đỡ được. Nếu các biểu tượng hiển thị đúng, nhưng bạn vẫn không thấy chúng trong starship, [nộp một báo cáo lỗi](https://github.com/starship/starship/issues/new/choose)

## Là thế nào để tôi gỡ cài đặt Starship?

Starship thì dễ dàng gỡ cài đặt như cài đặt ngay từ đầu.

1. Gỡ mọi tập tin trong cấu hình shell của bạn (e.g. `~/.bashrc`) đã sử dụng để khởi tạo Starship.
1. Xoá tập tin Starship nhị phân.

Nếu Starship đã được cài đặt bằng việc sử dụng một trình quản lí gói, vui lòng tham khảo tài liệu của chúng để gỡ cài đặt.

If Starship was installed using the install script, the following command will delete the binary:

```sh
# Locate and delete the starship binary
sh -c 'rm "$(command -v 'starship')"'
```
