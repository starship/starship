# Tích hợp sang v0.45.0

Starship v0.45.0 là một bản phát hành chứa nhiều thay đổi trong việc chuẩn bị cho bản thay đổi lớn v1.0.0. Chúng tôi tạo một vài thay đổi xung quanh cách cấu hình được thực hiện trên dấu nhắc lệnh, cho phép tùy biến theo góc độ tốt hơn.

Hướng dẫn này nhằm hướng dẫn bạn vượt qua những sự thay đổi.

## `prompt_order` được thay thế bởi một root-level `format`

Trước phiên bản v0.45.0, `prompt_order` sẽ chấp nhận một mảng các tên mô đun theo thứ tự mà chúng nên được kết xuất bởi Starship.

Starship v0.45.0 thay vì chấp nhận một giá trị `format`, nó cho phép tùy biến dấu nhắc lệnh bên ngoài chính các mô đun đó.

**Ví dụ của cấu hình pre-v0.45.0**

```toml
prompt_order = [
  "username",
  "hostname",
  "directory",
  "git_branch",
  "git_commit",
  "git_state",
  "git_status",
  "cmd_duration",
  "custom",
  "line_break",
  "jobs",
  "battery",
  "time",
  "character",
]
```

**Ví dụ của cấu hình v0.45.0**

```toml
format = """\
  $username\
  $hostname\
  $directory\
  $git_branch\
  $git_commit\
  $git_state\
  $git_status\
  $cmd_duration\
  $custom\
  $line_break\
  $jobs\
  $battery\
  $time\
  $character\
  """
```

## Mô đun `prefix` và `suffix` thay bằng `format`

Từ trước tới v0.45.0, một vài mô đun sẽ chấp nhận `prefix` và/hoặc `suffix` theo thứ tự để stylize các mà các mô đun được render.

Starship v0.45.0 instead accepts a `format` value, allowing for further customization of how modules are rendered. Thay vì định nghĩa một tiền tố và hậu tố cho các giá trị context-based, các giá trị bây giờ có thể được thay thế với một format string, cái đại diện cho đầu ra của module.

**Ví dụcủa cấu hình pre-v0.45.0**

```toml
[cmd_duration]
prefix = "took "
```

**Ví dụcủa cấu hình v0.45.0**

```toml
[cmd_duration]
# $duration – Thời gian câu lệnh dùng để thực thi (e.g. "15s")
# $style    – Style mặc định của mô đun (e.g. "bold yellow")
format = "took [$duration]($style) "
```

### Các mô đun ảnh hưởng

#### Character

| Thuộc tính bị gỡ bỏ     | Thay thế bằng    |
| ----------------------- | ---------------- |
| `symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Các thay đổi về cấu hình mặc định**

```diff
[character]
-- symbol = "❯"
-- error_symbol = "✖"
-- use_symbol_for_status = true
-- vicmd_symbol = "❮"
++ success_symbol = "[❯](bold green)"
++ error_symbol = "[❯](bold red)"
++ vicmd_symbol = "[❮](bold green)"
```

Trước đây, thuộc tính `use_symbol_for_status` được sử dụng để cấu hình dấu nhắc lệnh hiển thị `error_symbol` khi câu lệnh cuối cùng trả về kết quả có status code khác 0.

Với bản hát hành v0.45.0, chúng ta bây giờ luôn sử dụng `error_symbol` sau các status khác 0, thống nhất các thuộc tính `use_symbol_for_status` và `error_symbol`.

Cấu hình dâu nhắc lệnh để sử dụng cấu hình `use_symbol_for_status = true`, thêm đoạn dưới vào tệp cấu hình của bạn:

```toml
[character]
error_symbol = "[✖](bold red)"
```

_Lưu ý:_ Phần tử `character` tự động thêm vào một khoảng trắng phía sau, so unlike the other `format` strings, we specifically do not add one in the above examples.

#### Command Duration

| Thuộc tính bị gỡ bỏ | Thay thế bằng |
| ------------------- | ------------- |
| `prefix`            | `format`      |

**Các thay đổi về cấu hình mặc định**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Đường dẫn

| Thuộc tính bị gỡ bỏ | Thay thế bằng |
| ------------------- | ------------- |
| `prefix`            | `format`      |

**Các thay đổi về cấu hình mặc định**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Biến môi trường

| Thuộc tính bị gỡ bỏ | Thay thế bằng |
| ------------------- | ------------- |
| `prefix`            | `format`      |
| `suffix`            | `format`      |

**Các thay đổi về cấu hình mặc định**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git Commit

| Thuộc tính bị gỡ bỏ | Thay thế bằng |
| ------------------- | ------------- |
| `prefix`            | `format`      |
| `suffix`            | `format`      |

**Các thay đổi về cấu hình mặc định**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git Status

| Thuộc tính bị gỡ bỏ | Thay thế bằng |
| ------------------- | ------------- |
| `prefix`            | `format`      |
| `suffix`            | `format`      |
| `show_sync_count`   | `format`      |

**Các thay đổi về cấu hình mặc định**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Trước đây, thuộc tính `show_sync_count` được sử dụng để cấu hình dấu nhắc lệnh hiển thị số commit của nhánh ahead hoặc số lượng behind của remote branch.

Với bản phát hành v0.45.0, cái này được thay thế bằng ba thuộc tính rời `ahead`, `behind`, và `diverged`.

Cấu hình dấu nhắc lệnh sử dụng cấu hình `show_sync_count = true` cũ hơn, thiết lâp như dưới đây trong tệp cấu hình của bạn:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

| Thuộc tính bị gỡ bỏ | Thay thế bằng |
| ------------------- | ------------- |
| `prefix`            | `format`      |
| `suffix`            | `format`      |

**Các thay đổi về cấu hình mặc định**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| Thuộc tính bị gỡ bỏ | Thay thế bằng |
| ------------------- | ------------- |
| `nhãn`              | `format`      |
| `prefix`            | `format`      |
| `suffix`            | `format`      |

**Các thay đổi về cấu hình mặc định**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Thời gian

| Thuộc tính bị gỡ bỏ | Thay thế bằng |
| ------------------- | ------------- |
| `format`            | `time_format` |

**Các thay đổi về cấu hình mặc định**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Các câu lệnh tùy biến

| Thuộc tính bị gỡ bỏ | Thay thế bằng |
| ------------------- | ------------- |
| `prefix`            | `format`      |
| `suffix`            | `format`      |

**Các thay đổi về cấu hình mặc định**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
