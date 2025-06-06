use indexmap::{IndexMap, indexmap};
use os_info::Type;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct OSConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub symbols: IndexMap<Type, &'a str>,
    pub disabled: bool,
}

impl<'a> OSConfig<'a> {
    pub fn get_symbol(&self, key: &Type) -> Option<&'a str> {
        self.symbols.get(key).copied()
    }
}

impl Default for OSConfig<'_> {
    fn default() -> Self {
        OSConfig {
            format: "[$symbol]($style)",
            style: "bold white",
            symbols: indexmap! {
                Type::AIX => "➿ ",
                Type::Alpaquita => "🔔 ",
                Type::AlmaLinux => "💠 ",
                Type::Alpine => "🏔️ ",
                Type::Amazon => "🙂 ",
                Type::Android => "🤖 ",
                Type::AOSC => "🐱 ",
                Type::Arch => "🎗️ ",
                Type::Artix => "🎗️ ",
                Type::Bluefin => "🐟 ",
                Type::CachyOS => "🎗️ ",
                Type::CentOS => "💠 ",
                Type::Debian => "🌀 ",
                Type::DragonFly => "🐉 ",
                Type::Emscripten => "🔗 ",
                Type::EndeavourOS => "🚀 ",
                Type::Fedora => "🎩 ",
                Type::FreeBSD => "😈 ",
                Type::Garuda => "🦅 ",
                Type::Gentoo => "🗜️ ",
                Type::HardenedBSD => "🛡️ ",
                Type::Illumos => "🐦 ",
                Type::Kali => "🐉 ",
                Type::Linux => "🐧 ",
                Type::Mabox => "📦 ",
                Type::Macos => "🍎 ",
                Type::Manjaro => "🥭 ",
                Type::Mariner => "🌊 ",
                Type::MidnightBSD => "🌘 ",
                Type::Mint => "🌿 ",
                Type::NetBSD => "🚩 ",
                Type::NixOS => "❄️ ",
                Type::Nobara =>  "🎩 ",
                Type::OpenBSD => "🐡 ",
                Type::OpenCloudOS => "☁️ ",
                Type::openEuler => "🦉 ",
                Type::openSUSE => "🦎 ",
                Type::OracleLinux => "🦴 ",
                Type::Pop => "🍭 ",
                Type::Raspbian => "🍓 ",
                Type::Redhat => "🎩 ",
                Type::RedHatEnterprise => "🎩 ",
                Type::RockyLinux => "💠 ",
                Type::Redox => "🧪 ",
                Type::Solus => "⛵ ",
                Type::SUSE => "🦎 ",
                Type::Ubuntu => "🎯 ",
                Type::Ultramarine => "🔷 ",
                Type::Unknown => "❓ ",
                Type::Uos => "🐲 ",
                Type::Void => "  ",
                Type::Windows => "🪟 ",
                // Future symbols.
                //aosc =>       " ",
                //artix =>      " ",
                //coreos =>     " ",
                //devuan =>     " ",
                //elementary => " ",
                //mageia =>     " ",
                //mandriva =>   " ",
                //sabayon =>    " ",
                //slackware =>  " ",
                //solaris =>    " ",
            },
            disabled: true,
        }
    }
}
