use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Deserializer, Serialize};

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
    #[serde(deserialize_with = "deserialize_symbols")]
    /// IndexMap from lowercase String to &str.
    pub symbols: IndexMap<String, &'a str>,
    pub disabled: bool,
}

// Deserializer for OSConfig.symbols.
// Makes the IndexMap keys lowercase.
fn deserialize_symbols<'de, D>(deserializer: D) -> Result<IndexMap<String, &'de str>, D::Error>
where
    D: Deserializer<'de>,
{
    IndexMap::deserialize(deserializer).map(|index_map: IndexMap<String, &'de str>| {
        index_map
            .iter()
            .map(|(k, &v)| (k.to_lowercase(), v))
            .collect::<IndexMap<String, &'de str>>()
    })
}

impl<'a> OSConfig<'a> {
    pub fn get_symbol(&self, key: &str) -> Option<&'a str> {
        self.symbols.get(&key.to_lowercase()).cloned()
    }
}

impl<'a> Default for OSConfig<'a> {
    fn default() -> Self {
        OSConfig {
            format: "[$symbol]($style)",
            style: "bold white",
            symbols: indexmap! {
                // Capitalization maintained for legibility,
                // and to_lowercase() for &str -> String.
                "Alpine".to_lowercase() => "🏔️ ",
                "Amazon".to_lowercase() => "🙂 ",
                "Android".to_lowercase() => "🤖 ",
                "Arch".to_lowercase() => "🎗️ ",
                "CentOS".to_lowercase() => "💠 ",
                "Debian".to_lowercase() => "🌀 ",
                "DragonFly".to_lowercase() => "🐉 ",
                "Emscripten".to_lowercase() => "🔗 ",
                "EndeavourOS".to_lowercase() => "🚀 ",
                "Fedora".to_lowercase() => "🎩 ",
                "FreeBSD".to_lowercase() => "😈 ",
                "Garuda".to_lowercase() => "🦅 ",
                "Gentoo".to_lowercase() => "🗜️ ",
                "HardenedBSD".to_lowercase() => "🛡️ ",
                "Illumos".to_lowercase() => "🐦 ",
                "Linux".to_lowercase() => "🐧 ",
                "Macos".to_lowercase() => "🍎 ",
                "Manjaro".to_lowercase() => "🥭 ",
                "Mariner".to_lowercase() => "🌊 ",
                "MidnightBSD".to_lowercase() => "🌘 ",
                "Mint".to_lowercase() => "🌿 ",
                "NetBSD".to_lowercase() => "🚩 ",
                "NixOS".to_lowercase() => "❄️ ",
                "OpenBSD".to_lowercase() => "🐡 ",
                "openSUSE".to_lowercase() => "🦎 ",
                "OracleLinux".to_lowercase() => "🦴 ",
                "Pop".to_lowercase() => "🍭 ",
                "Raspbian".to_lowercase() => "🍓 ",
                "Redhat".to_lowercase() => "🎩 ",
                "RedHatEnterprise".to_lowercase() => "🎩 ",
                "Redox".to_lowercase() => "🧪 ",
                "Solus".to_lowercase() => "⛵ ",
                "SUSE".to_lowercase() => "🦎 ",
                "Ubuntu".to_lowercase() => "🎯 ",
                "Unknown".to_lowercase() => "❓ ",
                "Windows".to_lowercase() => "🪟 ",
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
