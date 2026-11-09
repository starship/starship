use std::env;

use super::{Context, Module, ModuleConfig};

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

    let sudo_flags = if config.use_legacy_check {
        "-nv"
    } else {
        "-Nnv"
    };

    let is_sudo_cached = context.exec_cmd("sudo", &[sudo_flags]).is_some();

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
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `sudo`:\n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;

    #[test]
    fn test_sudo_not_cached() {
        fn test_new_sudo() {
            let actual = ModuleRenderer::new("sudo")
                .cmd("sudo -Nnv", None)
                .config(toml::toml! {
                    [sudo]
                    disabled = false
                    allow_windows = true
                })
                .collect();
            let expected = None;

            assert_eq!(expected, actual);
        }

        fn test_legacy_sudo() {
            let actual = ModuleRenderer::new("sudo")
                .cmd("sudo -nv", None)
                .config(toml::toml! {
                    [sudo]
                    disabled = false
                    allow_windows = true
                    use_legacy_check = true
                })
                .collect();
            let expected = None;

            assert_eq!(expected, actual);
        }

        test_new_sudo();
        test_legacy_sudo();
    }

    #[test]
    fn test_sudo_cached() {
        fn test_new_sudo() {
            let actual = ModuleRenderer::new("sudo")
                .cmd(
                    "sudo -Nnv",
                    Some(CommandOutput {
                        stdout: String::new(),
                        stderr: String::new(),
                    }),
                )
                .config(toml::toml! {
                    [sudo]
                    disabled = false
                    allow_windows = true
                })
                .collect();
            let expected = Some(format!("{}", Color::Blue.bold().paint("as 🧙 ")));

            assert_eq!(expected, actual);
        }

        fn test_legacy_sudo() {
            let actual = ModuleRenderer::new("sudo")
                .cmd(
                    "sudo -nv",
                    Some(CommandOutput {
                        stdout: String::new(),
                        stderr: String::new(),
                    }),
                )
                .config(toml::toml! {
                    [sudo]
                    disabled = false
                    allow_windows = true
                    use_legacy_check = true
                })
                .collect();
            let expected = Some(format!("{}", Color::Blue.bold().paint("as 🧙 ")));

            assert_eq!(expected, actual);
        }

        test_new_sudo();
        test_legacy_sudo();
    }

    #[test]
    #[cfg(windows)]
    fn test_allow_windows_disabled_blocks_windows() {
        let actual = ModuleRenderer::new("sudo")
            .cmd(
                "sudo -Nnv",
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::new(),
                }),
            )
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
