use super::{Context, Module, ModuleConfig, Shell};

use crate::configs::shell::ShellConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("shell");
    let config: ShellConfig = ShellConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let shell = &context.shell;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "indicator" => match shell {
                    Shell::Bash => Some(config.bash_indicator),
                    Shell::Cmd => Some(config.cmd_indicator),
                    Shell::Csh => Some(config.csh_indicator),
                    Shell::Elvish => Some(config.elvish_indicator),
                    Shell::Fish => Some(config.fish_indicator),
                    Shell::Ion => Some(config.ion_indicator),
                    Shell::Ksh => Some(config.ksh_indicator),
                    Shell::Nu => Some(config.nu_indicator),
                    Shell::PowerShell => Some(config.powershell_indicator),
                    Shell::Pwsh => config.pwsh_indicator.or(Some(config.powershell_indicator)),
                    Shell::Tcsh => Some(config.tcsh_indicator),
                    Shell::Xonsh => Some(config.xonsh_indicator),
                    Shell::Zsh => Some(config.zsh_indicator),
                    Shell::Unknown => Some(config.unknown_indicator),
                },
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|var| match var {
                "bash_indicator" => Some(Ok(config.bash_indicator)),
                "cmd_indicator" => Some(Ok(config.cmd_indicator)),
                "csh_indicator" => Some(Ok(config.csh_indicator)),
                "elvish_indicator" => Some(Ok(config.elvish_indicator)),
                "fish_indicator" => Some(Ok(config.fish_indicator)),
                "ion_indicator" => Some(Ok(config.ion_indicator)),
                "ksh_indicator" => Some(Ok(config.ksh_indicator)),
                "powershell_indicator" => Some(Ok(config.powershell_indicator)),
                "pwsh_indicator" => config.pwsh_indicator.map(Ok),
                "tcsh_indicator" => Some(Ok(config.tcsh_indicator)),
                "xonsh_indicator" => Some(Ok(config.xonsh_indicator)),
                "zsh_indicator" => Some(Ok(config.zsh_indicator)),
                "unknown_indicator" => Some(Ok(config.unknown_indicator)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `shell`: \n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::context::Shell;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[derive(Copy, Clone)]
    struct ShellCase {
        name: &'static str,
        shell: Shell,
        default_indicator: &'static str,
    }

    const SHELL_CASES: &[ShellCase] = &[
        ShellCase {
            name: "bash",
            shell: Shell::Bash,
            default_indicator: "bsh",
        },
        ShellCase {
            name: "cmd",
            shell: Shell::Cmd,
            default_indicator: "cmd",
        },
        ShellCase {
            name: "csh",
            shell: Shell::Csh,
            default_indicator: "csh",
        },
        ShellCase {
            name: "elvish",
            shell: Shell::Elvish,
            default_indicator: "esh",
        },
        ShellCase {
            name: "fish",
            shell: Shell::Fish,
            default_indicator: "fsh",
        },
        ShellCase {
            name: "ion",
            shell: Shell::Ion,
            default_indicator: "ion",
        },
        ShellCase {
            name: "ksh",
            shell: Shell::Ksh,
            default_indicator: "ksh",
        },
        ShellCase {
            name: "nu",
            shell: Shell::Nu,
            default_indicator: "nu",
        },
        ShellCase {
            name: "powershell",
            shell: Shell::PowerShell,
            default_indicator: "psh",
        },
        ShellCase {
            name: "pwsh",
            shell: Shell::Pwsh,
            default_indicator: "psh",
        },
        ShellCase {
            name: "tcsh",
            shell: Shell::Tcsh,
            default_indicator: "tsh",
        },
        ShellCase {
            name: "xonsh",
            shell: Shell::Xonsh,
            default_indicator: "xsh",
        },
        ShellCase {
            name: "zsh",
            shell: Shell::Zsh,
            default_indicator: "zsh",
        },
    ];

    #[test]
    fn test_shell_indicators() {
        let config = toml::toml! {
            [shell]
            disabled = false
        };

        for case in SHELL_CASES {
            // Test default
            let expected_default = Some(format!(
                "{} ",
                Color::White.bold().paint(case.default_indicator)
            ));
            let actual_default = ModuleRenderer::new("shell")
                .shell(case.shell)
                .config(config.clone())
                .collect();
            assert_eq!(
                expected_default, actual_default,
                "Failed default for: {}",
                case.name
            );

            // Test custom
            let mut config_custom = config.clone();
            if let Some(tbl) = config_custom
                .get_mut("shell")
                .and_then(|v| v.as_table_mut())
            {
                tbl.insert(
                    format!("{}_indicator", case.name),
                    toml::Value::String(case.name.to_string()),
                );
            }

            let expected_custom = Some(format!("{} ", Color::White.bold().paint(case.name)));
            let actual_custom = ModuleRenderer::new("shell")
                .shell(case.shell)
                .config(config_custom)
                .collect();
            assert_eq!(
                expected_custom, actual_custom,
                "Failed custom for: {}",
                case.name
            );
        }
    }

    #[test]
    fn test_none_if_disabled() {
        let expected = None;
        let actual = ModuleRenderer::new("shell").shell(Shell::Bash).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_none_if_unknown_shell() {
        let expected = None;
        let actual = ModuleRenderer::new("shell").shell(Shell::Unknown).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pwsh_custom_format_fallback() {
        let expected = Some(format!("{} ", Color::Cyan.bold().paint("pwsh")));
        let actual = ModuleRenderer::new("shell")
            .shell(Shell::Pwsh)
            .config(toml::toml! {
                [shell]
                powershell_indicator = "[pwsh](bold cyan)"
                disabled = false
            })
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_custom_format_conditional_indicator_match() {
        let expected = Some(format!("{} ", "B"));
        let actual = ModuleRenderer::new("shell")
            .shell(Shell::Bash)
            .config(toml::toml! {
                [shell]
                bash_indicator = "B"
                format = "($bash_indicator )"
                disabled = false
            })
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_default_style() {
        let expected = Some(format!("{}", Color::White.bold().paint("fish")));
        let actual = ModuleRenderer::new("shell")
            .shell(Shell::Fish)
            .config(toml::toml! {
                [shell]
                format = "[fish]($style)"
                disabled = false
            })
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_custom_style() {
        let expected = Some(format!("{}", Color::Cyan.bold().paint("fish")));
        let actual = ModuleRenderer::new("shell")
            .shell(Shell::Fish)
            .config(toml::toml! {
                [shell]
                format = "[fish]($style)"
                style = "cyan bold"
                disabled = false
            })
            .collect();
        assert_eq!(expected, actual);
    }
}
