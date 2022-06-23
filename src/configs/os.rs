use indexmap::{indexmap, IndexMap};
use os_info::Type;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct OSConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub symbols: IndexMap<String, &'a str>,
    pub disabled: bool,
}

impl<'a> Default for OSConfig<'a> {
    fn default() -> Self {
        OSConfig {
            format: "[$symbol]($style)",
            style: "bold white",
            symbols: indexmap! {
                String::from("Alpine") => " ",
                String::from("Amazon") => " ",
                String::from("Android") => " ",
                String::from("Arch") => " ",
                String::from("CentOS") => " ",
                String::from("Debian") => " ",
                String::from("DragonFly") => " ",
                String::from("Emscripten") => " ",
                String::from("EndeavourOS") => " ",
                String::from("Fedora") => " ",
                String::from("FreeBSD") => " ",
                String::from("Gentoo") => " ",
                String::from("HardenedBSD") => "ﲊ ",
                String::from("Illumos") => " ",
                String::from("Linux") => " ",
                String::from("Macos") => " ",
                String::from("Manjaro") => " ",
                String::from("Mariner") => " ",
                String::from("MidnightBSD") => " ",
                String::from("Mint") => " ",
                String::from("NetBSD") => " ",
                String::from("NixOS") => " ",
                String::from("OpenBSD") => " ",
                String::from("SUSE") => " ",
                String::from("OracleLinux") => " ",
                String::from("Pop") => " ",
                String::from("Raspbian") => " ",
                String::from("Redhat") => " ",
                String::from("RedHatEnterprise") => " ",
                String::from("Redox") => " ",
                String::from("Solus") => "ﴱ ",
                String::from("openSUSE") => " ",
                String::from("Ubuntu") => " ",
                String::from("Unknown") => " ",
                String::from("Windows") => " ",
                // Future symbols.
                //String::from("aosc") =>       " ",
                //String::from("artix") =>      " ",
                //String::from("coreos") =>     " ",
                //String::from("devuan") =>     " ",
                //String::from("elementary") => " ",
                //String::from("mageia") =>     " ",
                //String::from("mandriva") =>   " ",
                //String::from("sabayon") =>    " ",
                //String::from("slackwave") =>  " ",
                //String::from("void") =>       " ",
                //String::from("solaris") =>    " ",
            },
            disabled: true,
        }
    }
}

impl<'a> OSConfig<'a> {
    pub fn get_symbol(&self, os_type: &Type) -> Option<&'a str> {
        // String from os_info::Type
        let key = &format!("{:?}", os_type);
        self.symbols
            .get(key)
            .cloned()
            .or_else(|| OSConfig::default().symbols.get(key).cloned())
    }
}
