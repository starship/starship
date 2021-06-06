use super::{Context, Module, RootModuleConfig, Shell};

use crate::configs::shell::ShellConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("shell");
    let config: ShellConfig = ShellConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let shell = context.shell;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "indicator" => match shell {
                    Shell::Bash => Some(config.bash_indicator),
                    Shell::Fish => Some(config.fish_indicator),
                    Shell::Zsh => Some(config.zsh_indicator),
                    Shell::PowerShell => Some(config.powershell_indicator),
                    Shell::Ion => Some(config.ion_indicator),
                    Shell::Elvish => Some(config.elvish_indicator),
                    Shell::Tcsh => Some(config.tcsh_indicator),
                    Shell::Nu => Some(config.nu_indicator),
                    Shell::Unknown => Some(config.unknown_indicator),
                },
                _ => None,
            })
            .map(|var| match var {
                "bash_indicator" => Some(Ok(config.bash_indicator)),
                "fish_indicator" => Some(Ok(config.fish_indicator)),
                "zsh_indicator" => Some(Ok(config.zsh_indicator)),
                "powershell_indicator" => Some(Ok(config.powershell_indicator)),
                "ion_indicator" => Some(Ok(config.ion_indicator)),
                "elvish_indicator" => Some(Ok(config.elvish_indicator)),
                "tcsh_indicator" => Some(Ok(config.tcsh_indicator)),
                "unknown_indicator" => Some(Ok(config.unknown_indicator)),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `shell`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::context::Shell;
    use crate::test::TestRenderer;
    use ansi_term::Color;

    #[test]
    fn test_none_if_disabled() {
        let expected = None;
        let actual = TestRenderer::new().shell(Shell::Bash).module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_none_if_unknown_shell() {
        let expected = None;
        let actual = TestRenderer::new().shell(Shell::Unknown).module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bash_default_format() {
        let expected = Some(format!("{} ", "bsh"));
        let actual = TestRenderer::new()
            .shell(Shell::Bash)
            .config(toml::toml! {
                [shell]
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bash_custom_format() {
        let expected = Some(format!("{} ", Color::Cyan.bold().paint("bash")));
        let actual = TestRenderer::new()
            .shell(Shell::Bash)
            .config(toml::toml! {
                [shell]
                bash_indicator = "[bash](bold cyan)"
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_fish_default_format() {
        let expected = Some(format!("{} ", "fsh"));
        let actual = TestRenderer::new()
            .shell(Shell::Fish)
            .config(toml::toml! {
                [shell]
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_fish_custom_format() {
        let expected = Some(format!("{} ", Color::Cyan.bold().paint("fish")));
        let actual = TestRenderer::new()
            .shell(Shell::Fish)
            .config(toml::toml! {
                [shell]
                fish_indicator = "[fish](cyan bold)"
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_zsh_default_format() {
        let expected = Some(format!("{} ", "zsh"));
        let actual = TestRenderer::new()
            .shell(Shell::Zsh)
            .config(toml::toml! {
                [shell]
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_zsh_custom_format() {
        let expected = Some(format!("{} ", Color::Cyan.bold().paint("zsh")));
        let actual = TestRenderer::new()
            .shell(Shell::Bash)
            .config(toml::toml! {
                [shell]
                bash_indicator = "[zsh](bold cyan)"
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_powershell_default_format() {
        let expected = Some(format!("{} ", "psh"));
        let actual = TestRenderer::new()
            .shell(Shell::PowerShell)
            .config(toml::toml! {
                [shell]
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_powershell_custom_format() {
        let expected = Some(format!("{} ", Color::Cyan.bold().paint("powershell")));
        let actual = TestRenderer::new()
            .shell(Shell::PowerShell)
            .config(toml::toml! {
                [shell]
                powershell_indicator = "[powershell](bold cyan)"
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ion_default_format() {
        let expected = Some(format!("{} ", "ion"));
        let actual = TestRenderer::new()
            .shell(Shell::Ion)
            .config(toml::toml! {
                [shell]
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ion_custom_format() {
        let expected = Some(format!("{} ", Color::Cyan.bold().paint("ion")));
        let actual = TestRenderer::new()
            .shell(Shell::Ion)
            .config(toml::toml! {
                [shell]
                ion_indicator = "[ion](bold cyan)"
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_elvish_default_format() {
        let expected = Some(format!("{} ", "esh"));
        let actual = TestRenderer::new()
            .shell(Shell::Elvish)
            .config(toml::toml! {
                [shell]
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_elvish_custom_format() {
        let expected = Some(format!("{} ", Color::Cyan.bold().paint("elvish")));
        let actual = TestRenderer::new()
            .shell(Shell::Elvish)
            .config(toml::toml! {
                [shell]
                elvish_indicator = "[elvish](bold cyan)"
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nu_default_format() {
        let expected = Some(format!("{} ", "nu"));
        let actual = TestRenderer::new()
            .shell(Shell::Nu)
            .config(toml::toml! {
                [shell]
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nu_custom_format() {
        let expected = Some(format!("{} ", Color::Cyan.bold().paint("nu")));
        let actual = TestRenderer::new()
            .shell(Shell::Nu)
            .config(toml::toml! {
                [shell]
                nu_indicator = "[nu](bold cyan)"
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_custom_format_conditional_indicator_match() {
        let expected = Some(format!("{} ", "B"));
        let actual = TestRenderer::new()
            .shell(Shell::Bash)
            .config(toml::toml! {
                [shell]
                bash_indicator = "B"
                format = "($bash_indicator )"
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_custom_format_conditional_indicator_no_match() {
        let expected = None;
        let actual = TestRenderer::new()
            .shell(Shell::Fish)
            .config(toml::toml! {
                [shell]
                bash_indicator = "B"
                format = "($indicator )"
                disabled = false
            })
            .module("shell");

        assert_eq!(expected, actual);
    }
}
