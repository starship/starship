use std::env;

use super::{Context, Module, RootModuleConfig};

use crate::configs::sudo::SudoConfig;
use crate::formatter::StringFormatter;

/// Creates a module with sudo credential cache status
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("sudo");
    let config = SudoConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    if !config.allow_windows && env::consts::FAMILY == "windows" {
        return None;
    }

    let binary = config.binary.trim();
    let is_sudo_cached = context.exec_cmd(binary, &["-n", "true"]).is_some();

    if !is_sudo_cached {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `sudo`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;

    #[test]
    #[cfg(not(windows))]
    fn test_sudo_not_cached() {
        let actual = ModuleRenderer::new("sudo")
            .cmd("sudo -n true", None)
            .config(toml::toml! {
                [sudo]
                disabled = false
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(not(windows))]
    fn test_sudo_cached() {
        let actual = ModuleRenderer::new("sudo")
            .cmd("sudo -v", None)
            .config(toml::toml! {
                [sudo]
                disabled = false
            })
            .collect();
        let expected = Some(format!("{}", Color::Blue.bold().paint("as 🧙‍ ")));

        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(not(windows))]
    fn test_doas_not_cached() {
        let actual = ModuleRenderer::new("sudo")
            .cmd("doas -n true", None)
            .config(toml::toml! {
                [sudo]
                disabled = false
                binary = "doas"
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(not(windows))]
    fn test_doas_cached() {
        let actual = ModuleRenderer::new("sudo")
            .cmd("doas true", None)
            .config(toml::toml! {
                [sudo]
                disabled = false
                binary = "doas"
            })
            .collect();
        let expected = Some(format!("{}", Color::Blue.bold().paint("as 🧙‍ ")));

        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(windows)]
    fn test_allow_windows_disabled_blocks_windows() {
        let actual = ModuleRenderer::new("sudo")
            .cmd("sudo -v", None)
            .config(toml::toml! {
                [sudo]
                disabled = false
                allow_windows = false
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }
}
