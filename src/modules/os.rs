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
                "symbol" => get_symbol(&config, &os.os_type()),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
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

// Get the operating system symbol from user config, or else default config
// when user has not defined a symbol for the operating system.
fn get_symbol<'a>(config: &'a OSConfig, os_type: &os_info::Type) -> Option<&'a str> {
    config
        .get_symbol(os_type)
        .or_else(|| OSConfig::default().get_symbol(os_type))
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
        .map(os_info::Version::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use os_info::Type;

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
            .config(toml::toml! {
                [os]
                disabled = false
                format = "[$symbol($codename )($edition )($name )($type )($version )]($style)"
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
        let config = OSConfig::default();

        let type_expected_pairs = [
            (Type::Alpine, Some("🏔️ ")),
            (Type::Amazon, Some("🙂 ")),
            (Type::Android, Some("🤖 ")),
            (Type::Arch, Some("🎗️ ")),
            (Type::CentOS, Some("💠 ")),
            (Type::Debian, Some("🌀 ")),
            (Type::DragonFly, Some("🐉 ")),
            (Type::Emscripten, Some("🔗 ")),
            (Type::EndeavourOS, Some("🚀 ")),
            (Type::Fedora, Some("🎩 ")),
            (Type::FreeBSD, Some("😈 ")),
            (Type::Garuda, Some("🦅 ")),
            (Type::Gentoo, Some("🗜️ ")),
            (Type::HardenedBSD, Some("🛡️ ")),
            (Type::Illumos, Some("🐦 ")),
            (Type::Linux, Some("🐧 ")),
            (Type::Macos, Some("🍎 ")),
            (Type::Manjaro, Some("🥭 ")),
            (Type::Mariner, Some("🌊 ")),
            (Type::MidnightBSD, Some("🌘 ")),
            (Type::Mint, Some("🌿 ")),
            (Type::NetBSD, Some("🚩 ")),
            (Type::NixOS, Some("❄️ ")),
            (Type::OpenBSD, Some("🐡 ")),
            (Type::openSUSE, Some("🦎 ")),
            (Type::OracleLinux, Some("🦴 ")),
            (Type::Pop, Some("🍭 ")),
            (Type::Raspbian, Some("🍓 ")),
            (Type::Redhat, Some("🎩 ")),
            (Type::RedHatEnterprise, Some("🎩 ")),
            (Type::Redox, Some("🧪 ")),
            (Type::Solus, Some("⛵ ")),
            (Type::SUSE, Some("🦎 ")),
            (Type::Ubuntu, Some("🎯 ")),
            (Type::Unknown, Some("❓ ")),
            (Type::Windows, Some("🪟 ")),
        ];

        for (t, e) in type_expected_pairs {
            assert_eq!(get_symbol(&config, &t), e);
        }
    }

    #[test]
    fn get_symbol_custom() {
        let config_toml = toml::toml! {
            [symbols]
            Alpine = " "
            Amazon = " "
            Android = " "
            Arch = " "
            CentOS = " "
            Debian = " "
            DragonFly = " "
            Emscripten = " "
            EndeavourOS = " "
            Fedora = " "
            FreeBSD = " "
            Garuda = "󰛓 "
            Gentoo = " "
            HardenedBSD = "󰞌 "
            Illumos = "󰈸 "
            Linux = " "
            Macos = " "
            Manjaro = " "
            Mariner = " "
            MidnightBSD = " "
            Mint = " "
            NetBSD = " "
            NixOS = " "
            OpenBSD = "󰈺 "
            SUSE = " "
            OracleLinux = "󰌷 "
            Pop = " "
            Raspbian = " "
            Redhat = " "
            RedHatEnterprise = " "
            Redox = "󰀘 "
            Solus = "󰠳 "
            openSUSE = " "
            Ubuntu = " "
            Unknown = " "
            Windows = "󰍲 "
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
            (Type::Garuda, Some("󰛓 ")),
            (Type::Gentoo, Some(" ")),
            (Type::HardenedBSD, Some("󰞌 ")),
            (Type::Illumos, Some("󰈸 ")),
            (Type::Linux, Some(" ")),
            (Type::Macos, Some(" ")),
            (Type::Manjaro, Some(" ")),
            (Type::Mariner, Some(" ")),
            (Type::MidnightBSD, Some(" ")),
            (Type::Mint, Some(" ")),
            (Type::NetBSD, Some(" ")),
            (Type::NixOS, Some(" ")),
            (Type::OpenBSD, Some("󰈺 ")),
            (Type::SUSE, Some(" ")),
            (Type::OracleLinux, Some("󰌷 ")),
            (Type::Pop, Some(" ")),
            (Type::Raspbian, Some(" ")),
            (Type::Redhat, Some(" ")),
            (Type::RedHatEnterprise, Some(" ")),
            (Type::Redox, Some("󰀘 ")),
            (Type::Solus, Some("󰠳 ")),
            (Type::openSUSE, Some(" ")),
            (Type::Ubuntu, Some(" ")),
            (Type::Unknown, Some(" ")),
            (Type::Windows, Some("󰍲 ")),
        ];

        for (t, e) in type_expected_pairs {
            assert_eq!(get_symbol(&config, &t), e);
        }
    }

    #[test]
    fn get_symbol_fallback() {
        let config_toml = toml::toml! {
            [symbols]
            Unknown = ""
            Arch = "Arch is the best!"
        };

        let config = OSConfig::load(&config_toml);

        let type_expected_pairs = [
            (Type::Alpine, Some("🏔️ ")),
            (Type::Amazon, Some("🙂 ")),
            (Type::Android, Some("🤖 ")),
            (Type::Arch, Some("Arch is the best!")),
            (Type::CentOS, Some("💠 ")),
            (Type::Debian, Some("🌀 ")),
            (Type::DragonFly, Some("🐉 ")),
            (Type::Emscripten, Some("🔗 ")),
            (Type::EndeavourOS, Some("🚀 ")),
            (Type::Fedora, Some("🎩 ")),
            (Type::FreeBSD, Some("😈 ")),
            (Type::Garuda, Some("🦅 ")),
            (Type::Gentoo, Some("🗜️ ")),
            (Type::HardenedBSD, Some("🛡️ ")),
            (Type::Illumos, Some("🐦 ")),
            (Type::Linux, Some("🐧 ")),
            (Type::Macos, Some("🍎 ")),
            (Type::Manjaro, Some("🥭 ")),
            (Type::Mariner, Some("🌊 ")),
            (Type::MidnightBSD, Some("🌘 ")),
            (Type::Mint, Some("🌿 ")),
            (Type::NetBSD, Some("🚩 ")),
            (Type::NixOS, Some("❄️ ")),
            (Type::OpenBSD, Some("🐡 ")),
            (Type::openSUSE, Some("🦎 ")),
            (Type::OracleLinux, Some("🦴 ")),
            (Type::Pop, Some("🍭 ")),
            (Type::Raspbian, Some("🍓 ")),
            (Type::Redhat, Some("🎩 ")),
            (Type::RedHatEnterprise, Some("🎩 ")),
            (Type::Redox, Some("🧪 ")),
            (Type::Solus, Some("⛵ ")),
            (Type::SUSE, Some("🦎 ")),
            (Type::Ubuntu, Some("🎯 ")),
            (Type::Unknown, Some("")),
            (Type::Windows, Some("🪟 ")),
        ];

        for (t, e) in type_expected_pairs {
            assert_eq!(get_symbol(&config, &t), e);
        }
    }

    #[test]
    fn warn_on_os_info_update() {
        #[deny(clippy::wildcard_enum_match_arm)]
        // This closure is the same as the default config symbols list.
        // When this clippy test fails, a new default symbol should be added to
        // `config/os.rs` to exhaustively match new possible `os_info::Type` cases.
        // Affects:
        // - crate::configs::os::OSConfig::default()
        // - crate::modules::os::tests
        // - docs/config/README.md/#Configuration/#OS/#Options
        // - docs/config/README.md/#Configuration/#OS/#Example
        // - docs/public/presets/toml/plain-text-symbols.toml
        // - dosc/public/presets/toml/nerd-font-symbols.toml
        // - .github/config-schema.json
        let _ = |t: Type| match t {
            Type::AIX => "➿ ",
            Type::Alpaquita => "🔔 ",
            Type::Alpine => "🏔️ ",
            Type::Amazon => "🙂 ",
            Type::Android => "🤖 ",
            Type::AOSC => "",
            Type::Arch | Type::Artix | Type::CachyOS => "🎗️ ",
            Type::Bluefin => "🐟 ",
            Type::CentOS | Type::AlmaLinux | Type::RockyLinux => "💠 ",
            Type::Debian => "🌀 ",
            Type::DragonFly => "🐉 ",
            Type::Emscripten => "🔗 ",
            Type::EndeavourOS => "🚀 ",
            Type::Fedora | Type::Nobara => "🎩 ",
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
            Type::Ultramarine => "🔷 ",
            Type::Unknown => "❓ ",
            Type::Uos => "🐲 ",
            Type::Void => "  ",
            Type::Windows => "🪟 ",
            _ => "",
        };
    }
}
