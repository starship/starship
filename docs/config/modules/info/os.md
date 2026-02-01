# OS

The `os` module shows the current operating system.
OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

> [!WARNING]
> The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option     | Default               | Description                                            |
| ---------- | --------------------- | ------------------------------------------------------ |
| `format`   | `'[$symbol]($style)'` | The format for the module.                             |
| `style`    | `'bold white'`        | The style for the module.                              |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type.
Operating system types not defined by your configuration use the default symbols table below.
All operating systems currently supported by the module are listed below.
If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
[os.symbols]
AIX = "➿ "
Alpaquita = "🔔 "
AlmaLinux = "💠 "
Alpine = "🏔️ "
ALTLinux = "Ⓐ "
Amazon = "🙂 "
Android = "🤖 "
AOSC = "🐱 "
Arch = "🎗️ "
Artix = "🎗️ "
Bluefin = "🐟 "
CachyOS = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Elementary = "🍏 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Ios = "📱 "
InstantOS = "⏲️ "
Kali = "🐉 "
Linux = "🐧 "
Mabox = "📦 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
Nobara = "🎩 "
OpenBSD = "🐡 "
OpenCloudOS = "☁️ "
openEuler = "🦉 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
PikaOS = "🐤 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
RockyLinux = "💠 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Ultramarine = "🔷 "
Unknown = "❓ "
Uos = "🐲 "
Void = " "
Windows = "🪟 "
Zorin = "🔹 "
```

### Variables

| Variable | Example      | Description                                                        |
| -------- | ------------ | ------------------------------------------------------------------ |
| symbol   | `🎗️`          | The current operating system symbol from advanced option `symbols` |
| name     | `Arch Linux` | The current operating system name                                  |
| type     | `Arch`       | The current operating system type                                  |
| codename |              | The current operating system codename, if applicable               |
| edition  |              | The current operating system edition, if applicable                |
| version  |              | The current operating system version, if applicable                |
| style\*  |              | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[os]
format = "on [($name )]($style)"
style = "bold blue"
disabled = false

[os.symbols]
Windows = " "
Arch = "Arch is the best! "
```
