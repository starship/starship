use indexmap::{indexmap, IndexMap};
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
    pub symbols: IndexMap<String, &'a str>,
    pub disabled: bool,
}

impl<'a> OSConfig<'a> {
    pub fn get_symbol(&self, key: &str) -> Option<&'a str> {
        self.symbols.get(key).cloned()
    }
}

impl<'a> Default for OSConfig<'a> {
    fn default() -> Self {
        OSConfig {
            format: "[$symbol]($style)",
            style: "bold white",
            symbols: indexmap! {
                "Alpine".to_owned() => "🏔️ ",
                "Amazon".to_owned() => "🙂 ",
                "Android".to_owned() => "🤖 ",
                "Arch".to_owned() => "🎗️ ",
                "CentOS".to_owned() => "💠 ",
                "Debian".to_owned() => "🌀 ",
                "DragonFly".to_owned() => "🐉 ",
                "Emscripten".to_owned() => "🔗 ",
                "EndeavourOS".to_owned() => "🚀 ",
                "Fedora".to_owned() => "🎩 ",
                "FreeBSD".to_owned() => "😈 ",
                "Gentoo".to_owned() => "🗜️ ",
                "HardenedBSD".to_owned() => "🛡️ ",
                "Illumos".to_owned() => "🐦 ",
                "Linux".to_owned() => "🐧 ",
                "Macos".to_owned() => "🍎 ",
                "Manjaro".to_owned() => "🥭 ",
                "Mariner".to_owned() => "🌊 ",
                "MidnightBSD".to_owned() => "🌘 ",
                "Mint".to_owned() => "🌿 ",
                "NetBSD".to_owned() => "🚩 ",
                "NixOS".to_owned() => "❄️ ",
                "OpenBSD".to_owned() => "🐡 ",
                "openSUSE".to_owned() => "🦎 ",
                "OracleLinux".to_owned() => "🦴 ",
                "Pop".to_owned() => "🍭 ",
                "Raspbian".to_owned() => "🍓 ",
                "Redhat".to_owned() => "🎩 ",
                "RedHatEnterprise".to_owned() => "🎩 ",
                "Redox".to_owned() => "🧪 ",
                "Solus".to_owned() => "⛵ ",
                "SUSE".to_owned() => "🦎 ",
                "Ubuntu".to_owned() => "🎯 ",
                "Unknown".to_owned() => "❓ ",
                "Windows".to_owned() => "🪟 ",
                // Future symbols.
                //"aosc".to_owned() =>       " ",
                //"artix".to_owned() =>      " ",
                //"coreos".to_owned() =>     " ",
                //"devuan".to_owned() =>     " ",
                //"elementary".to_owned() => " ",
                //"mageia".to_owned() =>     " ",
                //"mandriva".to_owned() =>   " ",
                //"sabayon".to_owned() =>    " ",
                //"slackwave".to_owned() =>  " ",
                //"void".to_owned() =>       " ",
                //"solaris".to_owned() =>    " ",
            },
            disabled: true,
        }
    }
}
