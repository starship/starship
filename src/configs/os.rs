use indexmap::{indexmap, IndexMap};
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

impl<'a> Default for OSConfig<'a> {
    fn default() -> Self {
        OSConfig {
            format: "[$symbol]($style)",
            style: "bold white",
            symbols: indexmap! {
                Type::Alpine => "🏔️ ",
                Type::Amazon => "🙂 ",
                Type::Android => "🤖 ",
                Type::Arch => "🎗️ ",
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
                Type::Linux => "🐧 ",
                Type::Macos => "🍎 ",
                Type::Manjaro => "🥭 ",
                Type::Mariner => "🌊 ",
                Type::MidnightBSD => "🌘 ",
                Type::Mint => "🌿 ",
                Type::NetBSD => "🚩 ",
                Type::NixOS => "❄️ ",
                Type::OpenBSD => "🐡 ",
                Type::OpenCloudOS => "☁️ ",
                Type::openEuler => "🦉 ",
                Type::openSUSE => "🦎 ",
                Type::OracleLinux => "🦴 ",
                Type::Pop => "🍭 ",
                Type::Raspbian => "🍓 ",
                Type::Redhat => "🎩 ",
                Type::RedHatEnterprise => "🎩 ",
                Type::Redox => "🧪 ",
                Type::Solus => "⛵ ",
                Type::SUSE => "🦎 ",
                Type::Ubuntu => "🎯 ",
                Type::Unknown => "❓ ",
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
                //void =>       " ",
                //solaris =>    " ",
            },
            disabled: true,
        }
    }
}
