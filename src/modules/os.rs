use super::{Context, Module, ModuleConfig};

use crate::configs::os::OSConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current operating system
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("os");
    let config: OSConfig = OSConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    #[cfg(not(test))]
    let os = os_info::get();

    #[cfg(test)]
    let os = os_info::Info::default();

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => get_symbol(&config, &os),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "bitness" => get_bitness(&os).map(Ok),
                "codename" => get_codename(&os).map(Ok),
                "edition" => get_edition(&os).map(Ok),
                "name" => get_name(&os).map(Ok),
                "type" => get_type(&os).map(Ok),
                "version" => get_version(&os).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });
    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `os`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_symbol<'a>(config: &'a OSConfig, os: &os_info::Info) -> Option<&'a str> {
    // String from os_info::Type
    let key = &format!("{:?}", os.os_type());
    config
        .symbols
        .get(key)
        .cloned()
        .or_else(|| OSConfig::default().symbols.get(key).cloned())
}

fn get_bitness(os: &os_info::Info) -> Option<String> {
    Some(os.bitness())
        .filter(|&x| x != os_info::Bitness::Unknown)
        .map(|x| x.to_string())
}

fn get_codename(os: &os_info::Info) -> Option<String> {
    os.codename().map(String::from)
}

fn get_edition(os: &os_info::Info) -> Option<String> {
    os.edition().map(String::from)
}

fn get_name(os: &os_info::Info) -> Option<String> {
    Some(os.os_type().to_string())
}

fn get_type(os: &os_info::Info) -> Option<String> {
    // String from os_info::Type
    Some(format!("{:?}", os.os_type()))
}

fn get_version(os: &os_info::Info) -> Option<String> {
    Some(os.version())
        .filter(|&x| x != &os_info::Version::Unknown)
        .map(|x| x.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use os_info::{Info, Type};

    #[test]
    fn default() {
        let actual = ModuleRenderer::new("os").collect();

        assert_eq!(actual, None);
    }

    #[test]
    fn default_enabled() {
        let actual = ModuleRenderer::new("os")
            .config(toml::toml! {
                [os]
                disabled = false
            })
            .collect();

        let expected = Some(format!("{}", Color::White.bold().paint("❓ ")));

        assert_eq!(actual, expected);
    }

    #[test]
    fn all_segments() {
        let actual = ModuleRenderer::new("os")
            .config(toml::toml!{
                [os]
                disabled = false
                format = "[$symbol($bitness )($codename )($edition )($name )($type )($version )]($style)"
            })
            .collect();

        let expected = Some(format!(
            "{}",
            Color::White.bold().paint("❓ Unknown Unknown ")
        ));

        assert_eq!(actual, expected);
    }

    #[test]
    fn get_symbol_default() {
        let config = OSConfig::try_load(None);

        let type_expected_pairs = [
            (Type::Alpine, Some("🏔️")),
            (Type::Amazon, Some("🙂")),
            (Type::Android, Some("🤖")),
            (Type::Arch, Some("🎗️")),
            (Type::CentOS, Some("💠")),
            (Type::Debian, Some("🌀")),
            (Type::DragonFly, Some("🐉")),
            (Type::Emscripten, Some("🔗")),
            (Type::EndeavourOS, Some("🚀")),
            (Type::Fedora, Some("🎩")),
            (Type::FreeBSD, Some("😈")),
            (Type::Gentoo, Some("🗜️")),
            (Type::HardenedBSD, Some("🛡️")),
            (Type::Illumos, Some("🐦")),
            (Type::Linux, Some("🐧")),
            (Type::Macos, Some("🍎")),
            (Type::Manjaro, Some("🥭")),
            (Type::Mariner, Some("🌊")),
            (Type::MidnightBSD, Some("🌘")),
            (Type::Mint, Some("🌿")),
            (Type::NetBSD, Some("🚩")),
            (Type::NixOS, Some("❄️")),
            (Type::OpenBSD, Some("🐡")),
            (Type::openSUSE, Some("🦎")),
            (Type::OracleLinux, Some("🦴")),
            (Type::Pop, Some("🍭")),
            (Type::Raspbian, Some("🍓")),
            (Type::Redhat, Some("🎩")),
            (Type::RedHatEnterprise, Some("🎩")),
            (Type::Redox, Some("🧪")),
            (Type::Solus, Some("⛵")),
            (Type::SUSE, Some("🦎")),
            (Type::Ubuntu, Some("🎯")),
            (Type::Unknown, Some("❓")),
            (Type::Windows, Some("🪟")),
        ];

        for (t, e) in type_expected_pairs {
            assert_eq!(get_symbol(&config, &Info::with_type(t)), e);
        }
    }

    #[test]
    fn get_symbol_custom() {
        let config_toml = toml::toml! {
            // I don't know why, but [os] seems to be implied
            [symbols]
            "Alpine" = " "
            "Amazon" = " "
            "Android" = " "
            "Arch" = " "
            "CentOS" = " "
            "Debian" = " "
            "DragonFly" = " "
            "Emscripten" = " "
            "EndeavourOS" = " "
            "Fedora" = " "
            "FreeBSD" = " "
            "Gentoo" = " "
            "HardenedBSD" = "ﲊ "
            "Illumos" = " "
            "Linux" = " "
            "Macos" = " "
            "Manjaro" = " "
            "Mariner" = " "
            "MidnightBSD" = " "
            "Mint" = " "
            "NetBSD" = " "
            "NixOS" = " "
            "OpenBSD" = " "
            "SUSE" = " "
            "OracleLinux" = " "
            "Pop" = " "
            "Raspbian" = " "
            "Redhat" = " "
            "RedHatEnterprise" = " "
            "Redox" = " "
            "Solus" = "ﴱ "
            "openSUSE" = " "
            "Ubuntu" = " "
            "Unknown" = " "
            "Windows" = " "
        };

        let config = OSConfig::load(&config_toml);

        let type_expected_pairs = [
            (Type::Alpine, Some(" ")),
            (Type::Amazon, Some(" ")),
            (Type::Android, Some(" ")),
            (Type::Arch, Some(" ")),
            (Type::CentOS, Some(" ")),
            (Type::Debian, Some(" ")),
            (Type::DragonFly, Some(" ")),
            (Type::Emscripten, Some(" ")),
            (Type::EndeavourOS, Some(" ")),
            (Type::Fedora, Some(" ")),
            (Type::FreeBSD, Some(" ")),
            (Type::Gentoo, Some(" ")),
            (Type::HardenedBSD, Some("ﲊ ")),
            (Type::Illumos, Some(" ")),
            (Type::Linux, Some(" ")),
            (Type::Macos, Some(" ")),
            (Type::Manjaro, Some(" ")),
            (Type::Mariner, Some(" ")),
            (Type::MidnightBSD, Some(" ")),
            (Type::Mint, Some(" ")),
            (Type::NetBSD, Some(" ")),
            (Type::NixOS, Some(" ")),
            (Type::OpenBSD, Some(" ")),
            (Type::SUSE, Some(" ")),
            (Type::OracleLinux, Some(" ")),
            (Type::Pop, Some(" ")),
            (Type::Raspbian, Some(" ")),
            (Type::Redhat, Some(" ")),
            (Type::RedHatEnterprise, Some(" ")),
            (Type::Redox, Some(" ")),
            (Type::Solus, Some("ﴱ ")),
            (Type::openSUSE, Some(" ")),
            (Type::Ubuntu, Some(" ")),
            (Type::Unknown, Some(" ")),
            (Type::Windows, Some(" ")),
        ];

        for (t, e) in type_expected_pairs {
            assert_eq!(get_symbol(&config, &Info::with_type(t)), e);
        }
    }

    #[test]
    fn get_symbol_fallback() {
        let config_toml = toml::toml! {
            [symbols]
            "Unknown" = ""
            "Arch" = "Arch is the best!"
        };

        let config = OSConfig::load(&config_toml);

        let type_expected_pairs = [
            (Type::Alpine, Some("🏔️")),
            (Type::Amazon, Some("🙂")),
            (Type::Android, Some("🤖")),
            (Type::Arch, Some("Arch is the best!")),
            (Type::CentOS, Some("💠")),
            (Type::Debian, Some("🌀")),
            (Type::DragonFly, Some("🐉")),
            (Type::Emscripten, Some("🔗")),
            (Type::EndeavourOS, Some("🚀")),
            (Type::Fedora, Some("🎩")),
            (Type::FreeBSD, Some("😈")),
            (Type::Gentoo, Some("🗜️")),
            (Type::HardenedBSD, Some("🛡️")),
            (Type::Illumos, Some("🐦")),
            (Type::Linux, Some("🐧")),
            (Type::Macos, Some("🍎")),
            (Type::Manjaro, Some("🥭")),
            (Type::Mariner, Some("🌊")),
            (Type::MidnightBSD, Some("🌘")),
            (Type::Mint, Some("🌿")),
            (Type::NetBSD, Some("🚩")),
            (Type::NixOS, Some("❄️")),
            (Type::OpenBSD, Some("🐡")),
            (Type::openSUSE, Some("🦎")),
            (Type::OracleLinux, Some("🦴")),
            (Type::Pop, Some("🍭")),
            (Type::Raspbian, Some("🍓")),
            (Type::Redhat, Some("🎩")),
            (Type::RedHatEnterprise, Some("🎩")),
            (Type::Redox, Some("🧪")),
            (Type::Solus, Some("⛵")),
            (Type::SUSE, Some("🦎")),
            (Type::Ubuntu, Some("🎯")),
            (Type::Unknown, Some("")),
            (Type::Windows, Some("🪟")),
        ];

        for (t, e) in type_expected_pairs {
            assert_eq!(get_symbol(&config, &Info::with_type(t)), e);
        }
    }

    #[test]
    fn get_bitness_unknown() {
        assert_eq!(get_bitness(&Info::unknown()), None);
    }
}
