# Cấu hình

Để bắt đầu cấu hình starship, tạo tập tin sau: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Tất cả cấu hình của starship đã xong trong tập tin này: [TOML](https://github.com/toml-lang/toml):

```toml
# Không in dòng mới tại điểm bắt đầu của prompt
add_newline = false

# Thay thế kí hiệu "❯" trong prompt với "➜"
[character]                            # Tên module chúng ta đang cấu hình là "character"
success_symbol = "[➜](bold green)"     # "success_symbol" được thiết lập là "➜" với màu "bold green"

# Vô hiệu hoá module, ẩn nó từ prompt hoàn chỉnh
[package]
disabled = true
```

Bạn có thể thay đổi đường dẫn mặc định của tệp tin `starship.toml` quan biến môi trường `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/.starship
```

Tương đương trong PowerShell (Windows) sẽ được thêm dòng này vào `$PROFILE` của bạn:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### Logging

Mặc định, starship logs các cảnh báo và các lỗi trong một tập tin tên là `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, nơi đó khoá của phiên làm việc tương ứng với thực thể terminal của bạn. Cái này, tuy nhiên có thể được thay đổi bằng cách sử dụng biến môi trường `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Tương đương trong PowerShell (Windows) sẽ được thêm dòng này vào `$PROFILE` của bạn:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### Thuật ngữ

**Module**: Một thành phần trong prompt, thông tin lấy được dựa trên thông tin ngữ cảnh từ hệ điều hành của bạn. Cho ví dụ, module "nodejs" cho biết phiên bản của NodeJS, cái hiện tại được cài đặt trên máy tính của bạn, nếu đường dẫn hiện tại của bạn là một dự án NodeJS.

**Variable**: Các thành phần con nhỏ hơn chứa thông tin cung cấp bởi module. Cho ví dụ, biến "version" trong "nodejs" module chứa phiên bản hiện tại của NodeJS.

Bằng việc quy ước, đa số các module có một tiền tố của terminal mặc định (ví dụ `via` trong "nodejs") và một khoảng trắng như là một hậu tố.

### Định dạng các chuỗi

Định dạng các chuỗi là định dạng một module với việc in ra tất cả các biến của nó. Đa số các module có một cái bắt đầu gọi là `format`, cái đó cấu hình việc hiển thị định dạng của module. Bạn có thể sử dụng các văn bản, các biến và các nhóm văn bản trong một định dạng chuỗi.

#### Biến

Một biến chứa một kí hiệu `$` theo sau bởi tên biến. Tên của một biến chỉ chứa các kí tự, các số và `_`.

Ví dụ:

- `$version` là một đính dạng chuỗi với một biến đặt tên là `version`.
- `$git_branch$git_commit` là một định dạng chuỗi với hai biến named `git_branch` và `git_commit`.
- `$git_branch $git_commit` có hai biến phân cách bằng một khoảng trắng.

#### Nhóm văn bản

Một nhóm văn bản được tạo nên bởi hai phần khác nhau.

Phần đầu tiên, cái được bao bọc trong một `[]`, là một [định dạng chuỗi](#format-strings). Bạn có thể thêm các văn bản, các biến, hoặc thậm chí các nhóm văn bản lồng nhau vào trong nó.

Phần thứ hai, cái được bao bọc trong một `()`, là một [chuỗi kiểu](#style-strings). Cái này có thể được sử dụng để quy định kiểu của phần đầu tiên.

Ví dụ:

- `[on](red bold)` sẽ in một chuỗi `on` với chữ đậm tô màu đỏ.
- `[⬢ $version](bold green)` sẽ in một kí hiệu `⬢` theo sao bởi nội dung của biến `version`, với chữ đậm tô màu xanh lá cây.
- `[a [b](red) c](green)` sẽ in `a b c` với `b` màu đỏ, `a` và `c` màu xanh lá cây.

#### Các chuỗi kiểu

Đa số các module trong starship cho phép bạn cấu hình kiểu hiển thị của chúng. This is done with an entry (thường được gọi là `kiểu`) cái là một cuỗi cấu hình đặc biệt. Đây là vài ví dụ của các chuỗi kiểu cũng với những gì chúng làm. Cú pháp chi tiết đầy đủ, tham khảo [hướng dẫn cấu hình nâng cao](/advanced-config/).

- `"fg:green bg:blue"` thiết lập chữ màu xanh lá cây trên nền màu xanh nước biển
- `"bg:blue fg:bright-green"` thiết lập chữ màu xanh lá cây sáng trên nền màu canh nước biển
- `"bold fg:27"` thiết lập chữ đậm với [màu ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` thiết lập chữ gạch chân trên một nền màu da cam
- `"bold italic fg:purple"` thiết lập chữa nghiêng đậm có màu tím
- `""` vô hiệu hoá tất cả các kiểu

Lưu ý rằng những style trông như thế nào sẽ được điều khiển bởi giả lập terminal của bạn. Ví dụ, một vài giả lập terminal sẽ làm sáng những màu thay vì làm đậm chữ, và một vài theme màu sử dụng cũng các giá trị cho các màu thường và màu sáng. Tương tự, để có được chữ nghiêng, terminal của bạn phải hỗ trợ các kiểu chữ nghiêng.

#### Điều kiện định dạng chuỗi

Một điều kiện định dạng chuỗi bọc trong `(` và `)` sẽ không render nếu tất cả các biến bên trong là rỗng.

Ví dụ:

- `(@$region)` sẽ không hiển thị gì nếu biến `region` là `None`, ngược lại `@` theo sao bởi giá trị của region.
- `(một vài văn bản)` sẽ không hiển thị thứ gì khi không có những biến bọc trong các dấu ngoặc.
- Khi `$all` là một shortcut cho `\[$a$b\]`, `($all)` sẽ không hiển thị chỉ khi `$a` và `$b` đều là `None`. Cái này làm việc giống như `(\[$a$b\] )`.

#### Các kí tự Escapable

Các kí hiệu sau có các sử dụng đặc biệt trong một định dạng chuỗi. Nếu bạn muốn in các kí tự sau, bạn phải đặt trước chúng kí tự backslash (`\`).

- \$
- \\
- [
- ]
- (
- )

Lưu ý rằng `toml` có [cú pháp escape riêng của nó](https://github.com/toml-lang/toml#user-content-string). Nó được khuyến nghị để sử dụng một literal string (`''`) trong cấu hình của bạn. Nếu bạn muốn sử dụng một kí tự cơ bản (`""`), chú ý đặt backslash `\` trước nó.

Ví dụ, khi bạn muốn in một kí hiệu `$` trên một dòng mới, các cấu hình sau cho `định dạng` tương đương:

```toml
# với chuỗi cơ bản
format = "\n\\$"

# với chuỗi cơ bản trong nhiều dòng
format = """

\\$"""

# với chuỗi đặc biệt
format = '''

\$'''
```

## Prompt

Cái này là danh sách các tuỳ chọn cho cấu hình prompt-wide.

### Các tuỳ chọn

| Tuỳ chọn       | Mặc định                       | Mô tả                                                                    |
| -------------- | ------------------------------ | ------------------------------------------------------------------------ |
| `format`       | [link](#default-prompt-format) | Cấu hình định dạng của prompt.                                           |
| `scan_timeout` | `30`                           | Timeout của starship cho việc quét các tập tin (tính theo milliseconds). |
| `add_newline`  | `true`                         | Thêm một dòng mới trước khi bắt đầu một prompt.                          |

### Ví dụ

```toml
# ~/.config/starship.toml

# Sử dụng định dạng custom
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Chờ 10 milliseconds để starship kiểm tra các tập tin trong đường dẫn hiện tại.
scan_timeout = 10

# Vô hiệu hoá dòng mới tại điểm bắt đầu của prompt
add_newline = false
```

### Định dạng prompt mặc định

Mặc định `format` được sử dụng để định nghĩa định dạng của prompt, nếu rỗng hoặc không `format` được cung cấp. Mặc định như sau:

```toml
format = "$all"

# Cái đó tương đương với
format = """
$username\
$hostname\
$shlvl\
$kubernetes\
$directory\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$kotlin\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
$ruby\
$rust\
$swift\
$terraform\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$env_var\
$crystal\
$custom\
$cmd_duration\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$character"""
```

## AWS

`aws` module cho biết region và profile hiện tại của AWS. Cái này dựa trên các biến môi trường `AWS_REGION`, `AWS_DEFAULT_REGION`, và `AWS_PROFILE` với tập tin `~/.aws/config`.

Khi sử dụng [aws-vault](https://github.com/99designs/aws-vault) profile được đọc từ biến môt trường `AWS_VAULT`.

### Các tuỳ chọn

| Tuỳ chọn         | Mặc định                                         | Mô tả                                                |
| ---------------- | ------------------------------------------------ | ---------------------------------------------------- |
| `format`         | `'on [$symbol$profile(\($region\))]($style) '` | Định dạng cho module.                                |
| `symbol`         | `"☁️ "`                                          | Kí hiệu sử dụng hiển thị trước profile AWS hiện tại. |
| `region_aliases` |                                                  | Bảng của các region alias để hiển thị ngoài tên AWS. |
| `style`          | `"bold yellow"`                                  | Kiểu cho module.                                     |
| `disabled`       | `false`                                          | Vô hiệu `AWS` module.                                |

### Các biến

| Biến      | Ví dụ            | Mô tả                            |
| --------- | ---------------- | -------------------------------- |
| region    | `ap-northeast-1` | Region AWS hiện tại              |
| profile   | `astronauts`     | Profile AWS hiện tại             |
| symbol    |                  | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |                  | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Các vị dụ

#### Hiển thị mọi thứ

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$profile(\($region\))]($style) '
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Hiển thị region

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$region]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Hiển thị profile

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
```

## Battery

`battery` module cho biết cách sạc pin của thiết bị là gì và tình trạng sạc hiện tại của nó. Module chỉ được nhìn thấy khi pin của thiết bị dưới 10%.

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                          | Mô tả                                                    |
| -------------------- | --------------------------------- | -------------------------------------------------------- |
| `full_symbol`        | `""`                             | Kí hiệu cho biết khi pin đầy.                            |
| `charging_symbol`    | `""`                             | Kí hiệu cho biết khi ping đang sạc.                      |
| `discharging_symbol` | `""`                             | Kí hiệu cho biết khi pin đang không sạc.                 |
| `unknown_symbol`     | `""`                             | Kí hiệu cho biết khi trạng thái pin không được xác định. |
| `empty_symbol`       | `""`                             | Kí hiệu cho biết khi hết pin.                            |
| `format`             | `"[$symbol$percentage]($style) "` | Định dạng cho module.                                    |
| `display`            | [link](#battery-display)          | Ngưỡng hiển thị và kiểu cho module.                      |
| `disabled`           | `false`                           | Vô hiệu `battery` module.                                |


### Ví dụ

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Hiển thị pin

Tuỳ chọn cấu hình `display` được sử dụng để định nghĩa khi nào pin nên cho biết (threshold) và nó trông như thế nào (style). Nếu `display` không được cung cấp. Mặc định như sau:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Các tuỳ chọn

Tuỳ chọn `display` là một mảng của của bảng sau.

| Tuỳ chọn    | Mô tả                                                      |
| ----------- | ---------------------------------------------------------- |
| `threshold` | Cận trên của tuỳ chọn hiển thị.                            |
| `style`     | Kiểu sử dụng nếu tuỳ chọn hiển thị được sử dụng bên trong. |

#### Ví dụ

```toml
[[battery.display]]  # "bold red" style khi lượng pin nằm giữa 0% và 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" style khi lượng pin nằm giữa 10% và 30%
threshold = 30
style = "bold yellow"

#khi lượng pin trên 30%, pin sẽ không được hiển thị

```

## Character

Module `character` cho biết một kí tự (thường là một mũi tên) bên cạnh nơi văn bản được nhập trong terminal của bạn.

Kí tự sẽ nói cho bạn câu lệnh cuối liệu thành công hay thất bại. Nó có thể làm điều này bằng hai cách:

- thay đổi màu(`đỏ`/`xanh lá`)
- thay đổi hình dạng (`❯`/`✖`)

Mặc định, nó chỉ thay đổi màu. Nếu bạn cũng muốn thay đổi hình dạng, tham khảo [ví dụ này](#with-custom-error-shape).

### Các tuỳ chọn

| Tuỳ chọn         | Mặc định            | Mô tả                                                                                |
| ---------------- | ------------------- | ------------------------------------------------------------------------------------ |
| `format`         | `"$symbol "`        | Định dạng chuỗi sử dụng trước văn bản nhập vào.                                      |
| `success_symbol` | `"[❯](bold green)"` | Định dạng chuỗi sửa dụng trước văn bản nhập vào nếu câu lệnh trước đó đã thành công. |
| `error_symbol`   | `"[❯](bold red)"`   | Định dạng chuỗi sửa dụng trước văn bản nhập vào nếu câu lệnh trước đó đã thất bại.   |
| `vicmd_symbol`   | `"[❮](bold green)"` | Định dạng chuỗi sửa dụng trước văn bản nhập vào nếu shell trong chế độ vim normal.   |
| `disabled`       | `false`             | Vô hiệu module `character`.                                                          |

### Các biến

| Biến   | Ví dụ | Mô tả                                                                 |
| ------ | ----- | --------------------------------------------------------------------- |
| symbol |       | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

### Các vị dụ

#### Có tuỳ chỉnh hình dạng lỗi

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Không có tuỳ chỉnh hình dạng lỗi

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### Có tuỳ chỉnh hình dạng vim

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

Module `cmake` cho biết phiên bản Cmake hiện tại được cài đặt nếu mọi điều kiện sau được thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `CmakeLists.txt`
- Đường dẫn hiện tại chứa một tập tin `CMakeCache.txt`

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                      |
| ---------- | ------------------------------------ | ------------------------------------------ |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                      |
| `symbol`   | `"喝 "`                               | Kí hiệu sử dụng trước phiên bản của cmake. |
| `style`    | `"bold blue"`                        | Kiểu cho module.                           |
| `disabled` | `false`                              | Vô hiệu module `cmake`.                    |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v3.17.3` | Phiên bản của cmake              |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

## Command Duration

Module `cmd_duration`. cho biết câu lệnh cuối cùng thực thi trong bao lâu. Module sẽ được hiện chỉ khi câu lệnh lấy nhiều hơn 2 giây, hoặc giá trị cấu hình `min_time`, nếu nó tồn tại.

::: cảnh báo Không thể hook DEBUG trap trong Bash

Nếu bạn đang chạy Starship trong `bash`, không thể hook `DEBUG` trap sau khi chạy `eval $(starship init $0)`, hoặc module này **sẽ** ngắt.

:::

Người dùng Bash, những người cần chức năng giống preexec có thể sử dụng [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Đơn giản là định nghĩa các mảng `preexec_functions` và `precmd_functions` trước khi chạy `eval $(starship init $0)`, và sau đó thực thi như bình thường.

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                      | Mô tả                                                                  |
| -------------------- | ----------------------------- | ---------------------------------------------------------------------- |
| `min_time`           | `2_000`                       | Khoảng thời gian ngắn nhất để hiện thời gian (tính bằng milliseconds). |
| `show_milliseconds`  | `false`                       | Hiện milliseconds.                                                     |
| `format`             | `"took [$duration]($style) "` | Định dạng cho module.                                                  |
| `style`              | `"bold yellow"`               | Kiểu cho module.                                                       |
| `disabled`           | `false`                       | Vô hiệu module `cmd_duration`.                                         |
| `show_notifications` | `false`                       | Hiện thông báo desktop khi câu lệnh hoàn thành.                        |
| `min_time_to_notify` | `45_000`                      | Khoảng thời gian ngắn nhất để thông báo (tính bằng milliseconds).      |

::: thử thuật

Hiện thông báo desktop yêu cầu starship được built với sự hỗ trợ của `rust-notify`. Bạn kiểm tra nếu starship hỗ trợ các thông báo bằng cách chạy `STARSHIP_LOG=debug starship module cmd_duration -d 60000` khi `show_notifications` được thiết lập là `true`.

:::

### Các biến

| Biến      | Ví dụ    | Mô tả                                 |
| --------- | -------- | ------------------------------------- |
| duration  | `16m40s` | Thời gian nó lấy để thực thi câu lệnh |
| style\* |          | Giá trị ghi đè của `style`            |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

Module `conda` cho biết môi trường conda hiện tại, nếu `$CONDA_DEFAULT_ENV` được thiết lập.

::: thủ thuật

Cái này không loại bỏ conda's prompt mà nó sở hữu, bạn có thể muốn chạy `conda config --set changeps1 False`.

:::

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                               | Mô tả                                                                                                                                                                                                       |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | Số lượng đường dẫn của biến môi trường nên được cắt bớt, nếu biến môi trường được tạo thông qua via `conda create -p [path]`. `0` nghĩa là không cắt bớt. Cũng thấy trong module [`directory`](#directory). |
| `symbol`            | `"🅒 "`                                 | Kí hiệu sử dụng trước tên biến môi trường.                                                                                                                                                                  |
| `style`             | `"bold green"`                         | Kiểu cho module.                                                                                                                                                                                            |
| `format`            | `"via [$symbol$environment]($style) "` | Định dạng cho module.                                                                                                                                                                                       |
| `ignore_base`       | `true`                                 | Bỏ qua biến môi trường `base` khi đã kích hoạt.                                                                                                                                                             |
| `disabled`          | `false`                                | Vô hiệu module `conda`.                                                                                                                                                                                     |

### Các biến

| Biến        | Ví dụ        | Mô tả                              |
| ----------- | ------------ | ---------------------------------- |
| environment | `astronauts` | Biến môi trường hiện tại của conda |
| symbol      |              | Giá trị ghi đè tuỳ chọn `symbol`   |
| style\*   |              | Giá trị ghi đè của `style`         |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

Module `crystal` cho biết phiên bản hiện tại của Crystal được đã cài đặt. Module sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `shard.yml`
- Đường dẫn hiện tại chứa một tập tin `.cr`

### Options

| Tuỳ chọn   | Mặc định                             | Mô tả                                                 |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `symbol`   | `"🔮 "`                               | Kí hiệu sử dụng trước phiên bản hiển thị của crystal. |
| `style`    | `"bold red"`                         | Kiểu cho module.                                      |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                 |
| `disabled` | `false`                              | Vô hiệu hoá module `crystal`.                         |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v0.32.1` | Phiên bản của `crystal`          |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

Module `dart` cho biết phiên bản của Dart đã cài đặt. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin với phần mở rộng `.dart`
- Đường dẫn hiện tại chứa một đường dẫn `.dart_tool`
- The current directory contains a `pubspec.yaml` or `pubspec.lock` file

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                           |
| ---------- | ------------------------------------ | ----------------------------------------------- |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                           |
| `symbol`   | `"🎯 "`                               | A format string representing the symbol of Dart |
| `style`    | `"bold blue"`                        | Kiểu cho module.                                |
| `disabled` | `false`                              | Disables the `dart` module.                     |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v2.8.4` | The version of `dart`            |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Directory

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                           | Mô tả                                                                            |
| ------------------- | -------------------------------------------------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`  | `true`                                             | Whether or not to truncate to the root of the git repo that you're currently in. |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | Định dạng cho module.                                                            |
| `style`             | `"bold cyan"`                                      | Kiểu cho module.                                                                 |
| `disabled`          | `false`                                            | Disables the `directory` module.                                                 |
| `read_only`         | `"🔒"`                                              | The symbol indicating current directory is read only.                            |
| `read_only_style`   | `"red"`                                            | The style for the read only symbol.                                              |
| `truncation_symbol` | `""`                                               | The symbol to prefix to truncated paths. eg: "…/"                                |
| `home_symbol`       | `"~"`                                              | The symbol indicating home directory.                                            |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | Mặc định | Mô tả                                                                                    |
| --------------------------- | -------- | ---------------------------------------------------------------------------------------- |
| `substitutions`             |          | A table of substitutions to be made to the path.                                         |
| `fish_style_pwd_dir_length` | `0`      | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`   | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Các biến

| Biến      | Ví dụ                 | Mô tả                      |
| --------- | --------------------- | -------------------------- |
| path      | `"D:/Projects"`       | The current directory path |
| style\* | `"black bold dimmed"` | Giá trị ghi đè của `style` |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### Các tuỳ chọn

| Tuỳ chọn          | Mặc định                           | Mô tả                                                                                                           |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | Định dạng cho module.                                                                                           |
| `symbol`          | `"🐳 "`                             | The symbol used before displaying the Docker context.                                                           |
| `style`           | `"blue bold"`                      | Kiểu cho module.                                                                                                |
| `only_with_files` | `true`                             | Only show when there's a `docker-compose.yml`, `docker-compose.yaml`, or `Dockerfile` in the current directory. |
| `disabled`        | `false`                            | Disables the `docker_context` module.                                                                           |

### Các biến

| Biến      | Ví dụ          | Mô tả                            |
| --------- | -------------- | -------------------------------- |
| context   | `test_context` | The current docker context       |
| symbol    |                | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |                | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one or more of the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.sln`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Các tuỳ chọn

| Tuỳ chọn    | Mặc định                                  | Mô tả                                                    |
| ----------- | ----------------------------------------- | -------------------------------------------------------- |
| `format`    | `"[$symbol($version )(🎯 $tfm )]($style)"` | Định dạng cho module.                                    |
| `symbol`    | `"•NET "`                                 | The symbol used before displaying the version of dotnet. |
| `heuristic` | `true`                                    | Use faster version detection to keep starship snappy.    |
| `style`     | `"bold blue"`                             | Kiểu cho module.                                         |
| `disabled`  | `false`                                   | Disables the `dotnet` module.                            |

### Các biến

| Biến      | Ví dụ            | Mô tả                                                              |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Giá trị ghi đè tuỳ chọn `symbol`                                   |
| style\* |                  | Giá trị ghi đè của `style`                                         |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `mix.exs`.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                                                    | Mô tả                                                           |
| ---------- | ----------------------------------------------------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                      | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"`                                             | Kiểu cho module.                                                |
| `format`   | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                               |
| `disabled` | `false`                                                     | Disables the `elixir` module.                                   |

### Các biến

| Biến        | Ví dụ   | Mô tả                            |
| ----------- | ------- | -------------------------------- |
| version     | `v1.10` | The version of `elixir`          |
| otp_version |         | The otp version of `elixir`      |
| symbol      |         | Giá trị ghi đè tuỳ chọn `symbol` |
| style\*   |         | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `elm.json`
- Đường dẫn hiện tại chứa một tập tin `elm-package.json`
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                           |
| ---------- | ------------------------------------ | ----------------------------------------------- |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                           |
| `symbol`   | `"🌳 "`                               | A format string representing the symbol of Elm. |
| `style`    | `"cyan bold"`                        | Kiểu cho module.                                |
| `disabled` | `false`                              | Disables the `elm` module.                      |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v0.19.1` | The version of `elm`             |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                       | Mô tả                                                                        |
| ---------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`   |                                | The symbol used before displaying the variable value.                        |
| `biến`     |                                | The environment variable to be displayed.                                    |
| `mặc định` |                                | The default value to be displayed when the selected variable is not defined. |
| `format`   | `"with [$env_value]($style) "` | Định dạng cho module.                                                        |
| `disabled` | `false`                        | Disables the `env_var` module.                                               |

### Các biến

| Biến      | Ví dụ                                       | Mô tả                                      |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Giá trị ghi đè tuỳ chọn `symbol`           |
| style\* | `black bold dimmed`                         | Giá trị ghi đè của `style`                 |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `rebar.config`.
- Đường dẫn hiện tại chứa một tập tin `erlang.mk`.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                                    |
| ---------- | ------------------------------------ | -------------------------------------------------------- |
| `symbol`   | `" "`                               | The symbol used before displaying the version of erlang. |
| `style`    | `"bold red"`                         | Kiểu cho module.                                         |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                    |
| `disabled` | `false`                              | Disables the `erlang` module.                            |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v22.1.3` | The version of `erlang`          |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Gcloud

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Các tuỳ chọn

| Tuỳ chọn         | Mặc định                                         | Mô tả                                                           |
| ---------------- | ------------------------------------------------ | --------------------------------------------------------------- |
| `format`         | `'on [$symbol$account(\($region\))]($style) '` | Định dạng cho module.                                           |
| `symbol`         | `"☁️ "`                                          | The symbol used before displaying the current GCP profile.      |
| `region_aliases` |                                                  | Table of region aliases to display in addition to the GCP name. |
| `style`          | `"bold blue"`                                    | Kiểu cho module.                                                |
| `disabled`       | `false`                                          | Disables the `gcloud` module.                                   |

### Các biến

| Biến      | Ví dụ             | Mô tả                                                              |
| --------- | ----------------- | ------------------------------------------------------------------ |
| region    | `us-central1`     | The current GCP region                                             |
| account   | `foo@example.com` | The current GCP profile                                            |
| project   |                   | The current GCP project                                            |
| active    | `mặc định`        | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |                   | Giá trị ghi đè tuỳ chọn `symbol`                                   |
| style\* |                   | Giá trị ghi đè của `style`                                         |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Các vị dụ

#### Display account and project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(\($project\))]($style) '
```

#### Display active config name only

```toml
# ~/.config/starship.toml

[gcloud]
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Display account and aliased region

```toml
# ~/.config/starship.toml

[gcloud]
symbol = "️🇬️ "
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                         | Mô tả                                                                                    |
| -------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Shows the remote tracking branch name, even if it is equal to the local branch name.     |
| `format`             | `"on [$symbol$branch]($style) "` | Định dạng cho module. Use `"$branch"` to refer to the current branch name.               |
| `symbol`             | `" "`                           | A format string representing the symbol of git branch.                                   |
| `style`              | `"bold purple"`                  | Kiểu cho module.                                                                         |
| `truncation_length`  | `2^63 - 1`                       | Truncates a git branch to X graphemes.                                                   |
| `truncation_symbol`  | `"…"`                            | The symbol used to indicate a branch name was truncated. You can use `""` for no symbol. |
| `only_attached`      | `false`                          | Only show the branch name when not in a detached HEAD state.                             |
| `disabled`           | `false`                          | Disables the `git_branch` module.                                                        |

### Các biến

| Biến          | Ví dụ    | Mô tả                                                                                                |
| ------------- | -------- | ---------------------------------------------------------------------------------------------------- |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached HEAD). |
| remote_name   | `origin` | The remote name.                                                                                     |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                     |
| symbol        |          | Giá trị ghi đè tuỳ chọn `symbol`                                                                     |
| style\*     |          | Giá trị ghi đè của `style`                                                                           |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                                               | Mô tả                                                 |
| -------------------- | ------------------------------------------------------ | ----------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | The length of the displayed git commit hash.          |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | Định dạng cho module.                                 |
| `style`              | `"bold green"`                                         | Kiểu cho module.                                      |
| `only_detached`      | `true`                                                 | Only show git commit hash when in detached HEAD state |
| `tag_disabled`       | `true`                                                 | Disables showing tag info in `git_commit` module.     |
| `tag_symbol`         | `"🏷 "`                                                 | Tag symbol prefixing the info shown                   |
| `disabled`           | `false`                                                | Disables the `git_commit` module.                     |

### Các biến

| Biến      | Ví dụ     | Mô tả                       |
| --------- | --------- | --------------------------- |
| hash      | `b703eb3` | The current git commit hash |
| style\* |           | Giá trị ghi đè của `style`  |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Các tuỳ chọn

| Tuỳ chọn       | Mặc định                                                        | Mô tả                                                                                   |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | Kiểu cho module.                                                                        |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Định dạng cho module.                                                                   |
| `disabled`     | `false`                                                         | Disables the `git_state` module.                                                        |

### Các biến

| Biến             | Ví dụ      | Mô tả                          |
| ---------------- | ---------- | ------------------------------ |
| state            | `REBASING` | The current state of the repo  |
| progress_current | `1`        | The current operation progress |
| progress_total   | `2`        | The total operation progress   |
| style\*        |            | Giá trị ghi đè của `style`     |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Các tuỳ chọn

| Tuỳ chọn     | Mặc định                                        | Mô tả                               |
| ------------ | ----------------------------------------------- | ----------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status` |
| `conflicted` | `"="`                                           | This branch has merge conflicts.    |
| `ahead`      | `"⇡"`                                           | The format of `ahead`               |
| `behind`     | `"⇣"`                                           | The format of `behind`              |
| `diverged`   | `"⇕"`                                           | The format of `diverged`            |
| `untracked`  | `"?"`                                           | The format of `untracked`           |
| `stashed`    | `"$"`                                           | The format of `stashed`             |
| `modified`   | `"!"`                                           | The format of `modified`            |
| `staged`     | `"+"`                                           | The format of `staged`              |
| `renamed`    | `"»"`                                           | The format of `renamed`             |
| `deleted`    | `"✘"`                                           | The format of `deleted`             |
| `style`      | `"bold red"`                                    | Kiểu cho module.                    |
| `disabled`   | `false`                                         | Disables the `git_status` module.   |

### Các biến

The following variables can be used in `format`:

| Biến           | Mô tả                                                                                         |
| -------------- | --------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                   |
| `ahead_behind` | Displays `diverged` `ahead` or `behind` format string based on the current status of the repo |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                   |
| `untracked`    | Displays `untracked` when there are untracked files in the working directory.                 |
| `stashed`      | Displays `stashed` when a stash exists for the local repository.                              |
| `modified`     | Displays `modified` when there are file modifications in the working directory.               |
| `staged`       | Displays `staged` when a new file has been added to the staging area.                         |
| `renamed`      | Displays `renamed` when a renamed file has been added to the staging area.                    |
| `deleted`      | Displays `deleted` when a file's deletion has been added to the staging area.                 |
| style\*      | Giá trị ghi đè của `style`                                                                    |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

The following variables can be used in `diverged`:

| Biến           | Mô tả                                          |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Biến    | Mô tả                    |
| ------- | ------------------------ |
| `count` | Show the number of files |

### Ví dụ

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
untracked = "🤷‍"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

The `golang` module shows the currently installed version of Golang. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `go.mod`
- Đường dẫn hiện tại chứa một tập tin `go.sum`
- Đường dẫn hiện tại chứa một tập tin `glide.yaml`
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                          |
| ---------- | ------------------------------------ | ---------------------------------------------- |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                          |
| `symbol`   | `"🐹 "`                               | A format string representing the symbol of Go. |
| `style`    | `"bold cyan"`                        | Kiểu cho module.                               |
| `disabled` | `false`                              | Disables the `golang` module.                  |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v1.12.1` | The version of `go`              |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of Helm. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `helmfile.yaml`
- The current directory contains a `Chart.yaml` file

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                            |
| ---------- | ------------------------------------ | ------------------------------------------------ |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                            |
| `symbol`   | `"⎈ "`                               | A format string representing the symbol of Helm. |
| `style`    | `"bold white"`                       | Kiểu cho module.                                 |
| `disabled` | `false`                              | Disables the `helm` module.                      |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v3.1.1` | The version of `helm`            |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

The `hostname` module shows the system hostname.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                    | Mô tả                                                                                                                                |
| ---------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                      | Only show hostname when connected to an SSH session.                                                                                 |
| `trim_at`  | `"."`                       | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `format`   | `"[$hostname]($style) in "` | Định dạng cho module.                                                                                                                |
| `style`    | `"bold dimmed green"`       | Kiểu cho module.                                                                                                                     |
| `disabled` | `false`                     | Disables the `hostname` module.                                                                                                      |

### Các biến

| Biến      | Ví dụ | Mô tả                            |
| --------- | ----- | -------------------------------- |
| symbol    |       | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |       | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                                 | Mô tả                                           |
| ---------- | ---------------------------------------- | ----------------------------------------------- |
| `format`   | `"via [${symbol}(${version} )]($style)"` | Định dạng cho module.                           |
| `symbol`   | `"☕ "`                                   | A format string representing the symbol of Java |
| `style`    | `"red dimmed"`                           | Kiểu cho module.                                |
| `disabled` | `false`                                  | Disables the `java` module.                     |

### Các biến

| Biến      | Ví dụ | Mô tả                            |
| --------- | ----- | -------------------------------- |
| version   | `v14` | The version of `java`            |
| symbol    |       | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |       | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Các tuỳ chọn

| Tuỳ chọn    | Mặc định                      | Mô tả                                            |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | Show number of jobs if exceeded.                 |
| `format`    | `"[$symbol$number]($style) "` | Định dạng cho module.                            |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | Kiểu cho module.                                 |
| `disabled`  | `false`                       | Disables the `jobs` module.                      |

### Các biến

| Biến      | Ví dụ | Mô tả                            |
| --------- | ----- | -------------------------------- |
| number    | `1`   | The number of jobs               |
| symbol    |       | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |       | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                             |
| ---------- | ------------------------------------ | ------------------------------------------------- |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                             |
| `symbol`   | `"ஃ "`                               | A format string representing the symbol of Julia. |
| `style`    | `"bold purple"`                      | Kiểu cho module.                                  |
| `disabled` | `false`                              | Disables the `julia` module.                      |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v1.4.0` | The version of `julia`           |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of Kotlin. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `.kt` or a `.kts` file

### Các tuỳ chọn

| Tuỳ chọn        | Mặc định                             | Mô tả                                                                         |
| --------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`        | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                         |
| `symbol`        | `"🅺 "`                               | A format string representing the symbol of Kotlin.                            |
| `style`         | `"bold blue"`                        | Kiểu cho module.                                                              |
| `kotlin_binary` | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`      | `false`                              | Disables the `kotlin` module.                                                 |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`          |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = "kotlinc"
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: thử thuật

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Các tuỳ chọn

| Tuỳ chọn          | Mặc định                                             | Mô tả                                                                 |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | Định dạng cho module.                                                 |
| `style`           | `"cyan bold"`                                        | Kiểu cho module.                                                      |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Disables the `kubernetes` module.                                     |

### Các biến

| Biến      | Ví dụ                | Mô tả                                    |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Giá trị ghi đè tuỳ chọn `symbol`         |
| style\* |                      | Giá trị ghi đè của `style`               |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định | Mô tả                                                              |
| ---------- | -------- | ------------------------------------------------------------------ |
| `disabled` | `false`  | Disables the `line_break` module, making the prompt a single line. |

### Ví dụ

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

The `lua` module shows the currently installed version of Lua. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Các tuỳ chọn

| Tuỳ chọn     | Mặc định                             | Mô tả                                                                      |
| ------------ | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`     | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                      |
| `symbol`     | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `style`      | `"bold blue"`                        | Kiểu cho module.                                                           |
| `lua_binary` | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`   | `false`                              | Disables the `lua` module.                                                 |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v5.4.0` | The version of `lua`             |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: thử thuật

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Các tuỳ chọn

| Tuỳ chọn    | Mặc định                                      | Mô tả                                                    |
| ----------- | --------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                          | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | Định dạng cho module.                                    |
| `symbol`    | `"🐏"`                                         | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                         | Kiểu cho module.                                         |
| `disabled`  | `true`                                        | Disables the `memory_usage` module.                      |

### Các biến

| Biến             | Ví dụ         | Mô tả                                                              |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Giá trị ghi đè tuỳ chọn `symbol`                                   |
| style\*        |               | Giá trị ghi đè của `style`                                         |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

### Ví dụ

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                         | Mô tả                                                                                        |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | Kiểu cho module.                                                                             |
| `format`            | `"on [$symbol$branch]($style) "` | Định dạng cho module.                                                                        |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| branch    | `master` | The active mercurial branch      |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                                 |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module                                  |
| `symbol`   | `"👑 "`                               | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"`                      | Kiểu cho module.                                      |
| `disabled` | `false`                              | Disables the `nim` module.                            |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v1.2.0` | The version of `nimc`            |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### Các tuỳ chọn

| Tuỳ chọn     | Mặc định                                       | Mô tả                                                 |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | Định dạng cho module.                                 |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | Kiểu cho module.                                      |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### Các biến

| Biến      | Ví dụ   | Mô tả                            |
| --------- | ------- | -------------------------------- |
| state     | `pure`  | The state of the nix-shell       |
| name      | `lorri` | The name of the nix-shell        |
| symbol    |         | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |         | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                                                 |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                                                 |
| `symbol`            | `"⬢ "`                               | A format string representing the symbol of NodeJS.                                                    |
| `style`             | `"bold green"`                       | Kiểu cho module.                                                                                      |
| `disabled`          | `false`                              | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in Packages.json does not match the NodeJS version. |

###  Variables

| Biến      | Ví dụ      | Mô tả                            |
| --------- | ---------- | -------------------------------- |
| version   | `v13.12.0` | The version of `node`            |
| symbol    |            | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |            | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                                   |
| ---------- | ------------------------------------ | ------------------------------------------------------- |
| `format`   | `"via [$symbol($version )]($style)"` | The format string for the module.                       |
| `symbol`   | `"🐫 "`                               | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"`                      | Kiểu cho module.                                        |
| `disabled` | `false`                              | Disables the `ocaml` module.                            |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v4.10.0` | The version of `ocaml`           |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                                            | Mô tả                                                          |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | Định dạng cho module.                                          |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | Kiểu cho module.                                               |
| `disabled` | `false`                                             | Disables the `OpenStack` module.                               |

### Các biến

| Biến      | Ví dụ  | Mô tả                            |
| --------- | ------ | -------------------------------- |
| cloud     | `corp` | The current OpenStack cloud      |
| project   | `dev`  | The current OpenStack project    |
| symbol    |        | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |        | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present
- **helm** - The `helm` chart version is extracted from the `Chart.yaml` present
- **maven** - The `maven` package version is extracted from the `pom.xml` present
- **meson** - The `meson` package version is extracted from the `meson.build` present

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Các tuỳ chọn

| Tuỳ chọn          | Mặc định                          | Mô tả                                                      |
| ----------------- | --------------------------------- | ---------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | Định dạng cho module.                                      |
| `symbol`          | `"📦 "`                            | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"`                      | Kiểu cho module.                                           |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`                           | Disables the `package` module.                             |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v1.0.0` | The version of your package      |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of Perl. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                                 |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol($version )]($style)"` | The format string for the module.                     |
| `symbol`   | `"🐪 "`                               | The symbol used before displaying the version of Perl |
| `style`    | `"bold 149"`                         | Kiểu cho module.                                      |
| `disabled` | `false`                              | Disables the `perl` module.                           |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v5.26.1` | The version of `perl`            |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

### Ví dụ

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of PHP. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                                 |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                 |
| `symbol`   | `"🐘 "`                               | The symbol used before displaying the version of PHP. |
| `style`    | `"147 bold"`                         | Kiểu cho module.                                      |
| `disabled` | `false`                              | Disables the `php` module.                            |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v7.3.8` | The version of `php`             |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `spago.dhall` file
- The current directory contains a \*.purs files

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                                        |
| ---------- | ------------------------------------ | ------------------------------------------------------------ |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                        |
| `symbol`   | `"<=> "`                       | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"`                       | Kiểu cho module.                                             |
| `disabled` | `false`                              | Disables the `purescript` module.                            |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `0.13.5` | The version of `purescript`      |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- Đường dẫn hiện tại chứa một tập tin `tox.ini`
- Đường dẫn hiện tại chứa một tập tin `setup.py`
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                                                                   | Mô tả                                                                                  |
| -------------------- | -------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\))]($style)'` | Định dạng cho module.                                                                  |
| `symbol`             | `"🐍 "`                                                                     | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                            | Kiểu cho module.                                                                       |
| `pyenv_version_name` | `false`                                                                    | Use pyenv to get Python version                                                        |
| `pyenv_prefix`       | `pyenv`                                                                    | Prefix before pyenv version display, only used if pyenv is used                        |
| `scan_for_pyfiles`   | `true`                                                                     | If false, Python files in the current directory will not show this module.             |
| `python_binary`      | `["python", "python3, "python2"]`                                          | Configures the python binaries that Starship should executes when getting the version. |
| `disabled`           | `false`                                                                    | Disables the `python` module.                                                          |

::: thử thuật

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Các biến

| Biến         | Ví dụ           | Mô tả                                      |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Giá trị ghi đè tuỳ chọn `symbol`           |
| style        | `"yellow bold"` | Giá trị ghi đè của `style`                 |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |


### Ví dụ

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = "python3"
```

## Ruby

The `ruby` module shows the currently installed version of Ruby. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                            |
| ---------- | ------------------------------------ | ------------------------------------------------ |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                            |
| `symbol`   | `"💎 "`                               | A format string representing the symbol of Ruby. |
| `style`    | `"bold red"`                         | Kiểu cho module.                                 |
| `disabled` | `false`                              | Disables the `ruby` module.                      |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v2.5.1` | The version of `ruby`            |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                           |
| ---------- | ------------------------------------ | ----------------------------------------------- |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                           |
| `symbol`   | `"🦀 "`                               | A format string representing the symbol of Rust |
| `style`    | `"bold red"`                         | Kiểu cho module.                                |
| `disabled` | `false`                              | Disables the `rust` module.                     |

### Các biến

| Biến      | Ví dụ             | Mô tả                            |
| --------- | ----------------- | -------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`           |
| symbol    |                   | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |                   | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## SHLVL

The `shlvl` module shows the current SHLVL ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Các tuỳ chọn

| Tuỳ chọn    | Mặc định                     | Mô tả                                                       |
| ----------- | ---------------------------- | ----------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                          |
| `format`    | `"[$symbol$shlvl]($style) "` | Định dạng cho module.                                       |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the SHLVL.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current SHLVL amount. |
| `style`     | `"bold yellow"`              | Kiểu cho module.                                            |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                |

### Các biến

| Biến      | Ví dụ | Mô tả                            |
| --------- | ----- | -------------------------------- |
| shlvl     | `3`   | The current value of SHLVL       |
| symbol    |       | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |       | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                         | Mô tả                                            |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Định dạng cho module.                            |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | Kiểu cho module.                                 |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Các biến

| Biến      | Ví dụ        | Mô tả                            |
| --------- | ------------ | -------------------------------- |
| env       | `centos.img` | The current singularity image    |
| symbol    |              | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |              | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: thử thuật

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Các tuỳ chọn

| Tuỳ chọn                | Mặc định                      | Mô tả                                                |
| ----------------------- | ----------------------------- | ---------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "` | The format of the module                             |
| `symbol`                | `"✖"`                         | The symbol displayed on program error                |
| `not_executable_symbol` | `"🚫"`                         | The symbol displayed when file isn't executable      |
| `not_found_symbol`      | `"🔍"`                         | The symbol displayed when the command can't be found |
| `sigint_symbol`         | `"🧱"`                         | The symbol displayed on SIGINT (Ctrl + c)            |
| `signal_symbol`         | `"⚡"`                         | The symbol displayed on any signal                   |
| `style`                 | `"bold red"`                  | Kiểu cho module.                                     |
| `recognize_signal_code` | `true`                        | Enable signal mapping from exit code                 |
| `map_symbol`            | `false`                       | Enable symbols mapping from exit code                |
| `disabled`              | `true`                        | Disables the `status` module.                        |

### Các biến

| Biến           | Ví dụ   | Mô tả                                                                |
| -------------- | ------- | -------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                    |
| int            | `127`   | The exit code of the last command                                    |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                  |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled      |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found         |
| symbol         |         | Giá trị ghi đè tuỳ chọn `symbol`                                     |
| style\*      |         | Giá trị ghi đè của `style`                                           |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $status_common_meaning$status_signal_name$status_maybe_int\]]($style) '
map_symbol = true
disabled = false

```

## Swift

The `swift` module shows the currently installed version of Swift. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                            |
| ---------- | ------------------------------------ | ------------------------------------------------ |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                            |
| `symbol`   | `"🐦 "`                               | A format string representing the symbol of Swift |
| `style`    | `"bold 202"`                         | Kiểu cho module.                                 |
| `disabled` | `false`                              | Disables the `swift` module.                     |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v5.2.4` | The version of `swift`           |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version.

::: thử thuật

By default the terraform version is not shown, since this is slow for current versions of terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-version).

:::

Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                                 |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`   | `"💠 "`                               | A format string shown before the terraform workspace. |
| `style`    | `"bold 105"`                         | Kiểu cho module.                                      |
| `disabled` | `false`                              | Disables the `terraform` module.                      |

### Các biến

| Biến      | Ví dụ      | Mô tả                            |
| --------- | ---------- | -------------------------------- |
| version   | `v0.12.24` | The version of `terraform`       |
| workspace | `mặc định` | The current terraform workspace  |
| symbol    |            | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |            | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

#### With Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: thử thuật

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Các tuỳ chọn

| Tuỳ chọn          | Mặc định                | Mô tả                                                                                                                              |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                  |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Các biến

| Biến      | Ví dụ      | Mô tả                      |
| --------- | ---------- | -------------------------- |
| time      | `13:08:10` | The current time.          |
| style\* |            | Giá trị ghi đè của `style` |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

::: thử thuật

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Các tuỳ chọn

| Tuỳ chọn      | Mặc định                | Mô tả                                 |
| ------------- | ----------------------- | ------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root. |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.    |
| `format`      | `"[$user]($style) in "` | Định dạng cho module.                 |
| `show_always` | `false`                 | Always shows the `username` module.   |
| `disabled`    | `false`                 | Disables the `username` module.       |

### Các biến

| Biến    | Ví dụ        | Mô tả                                                                                       |
| ------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style` | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`  | `"matchai"`  | The currently logged-in user ID.                                                            |

### Ví dụ

```toml
# ~/.config/starship.toml

[username]
style_user = "white bold"
style_root = "black bold"
format = "user: [$user]($style) "
disabled = false
show_always = true
```

## Zig

The `zig` module shows the currently installed version of Zig. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `.zig` file

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                             | Mô tả                                                 |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `symbol`   | `"↯ "`                               | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"`                      | Kiểu cho module.                                      |
| `format`   | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                 |
| `disabled` | `false`                              | Disables the `zig` module.                            |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v0.6.0` | The version of `zig`             |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `files`
- The current directory contains a directory whose name is in `directories`
- The current directory contains a file whose extension is in `extensions`
- The `when` command returns 0

::: thử thuật

Multiple custom modules can be defined by using a `.`.

:::

::: thử thuật

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: thử thuật

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

### Các tuỳ chọn

| Tuỳ chọn      | Mặc định                        | Mô tả                                                                                                                      |
| ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                         |
| `mô tả`       | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                  | Kiểu cho module.                                                                                                           |
| `format`      | `"[$symbol($output )]($style)"` | Định dạng cho module.                                                                                                      |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                             |

### Các biến

| Biến      | Mô tả                                  |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Giá trị ghi đè tuỳ chọn `symbol`       |
| style\* | Giá trị ghi đè của `style`             |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ["pwsh", "-Command", "-"]
```

::: warning Make sure your custom shell configuration exits gracefully

If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).

For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.

Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.

Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

:::

### Ví dụ

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
