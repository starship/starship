use super::{Context, Module, ModuleConfig, Shell};
use crate::configs::character::CharacterConfig;
use crate::formatter::StringFormatter;

/// Creates a module for the prompt character
///
/// The character segment prints an arrow character in a color dependent on the
/// exit-code of the last executed command:
/// - If the exit-code was "0", it will be formatted with `success_symbol`
///   (green arrow by default)
/// - If the exit-code was anything else, it will be formatted with
///   `error_symbol` (red arrow by default)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    enum ShellEditMode {
        Normal,
        Visual,
        Replace,
        ReplaceOne,
        Insert,
    }
    const ASSUMED_MODE: ShellEditMode = ShellEditMode::Insert;
    // TODO: extend config to more modes

    let mut module = context.new_module("character");
    let config: CharacterConfig = CharacterConfig::try_load(module.config);

    let props = &context.properties;
    let exit_code = props.status_code.as_deref().unwrap_or("0");
    let keymap = props.keymap.as_str();
    let exit_success = exit_code == "0";

    // Match shell "keymap" names to normalized vi modes
    // NOTE: in vi mode, fish reports normal mode as "default".
    // Unfortunately, this is also the name of the non-vi default mode.
    // We do some environment detection in src/init.rs to translate.
    // The result: in non-vi fish, keymap is always reported as "insert"
    let mode = match (&context.shell, keymap) {
        (Shell::Fish, "default") | (Shell::Zsh, "vicmd") | (Shell::Cmd, "vi") => {
            ShellEditMode::Normal
        }
        (Shell::Fish, "visual") => ShellEditMode::Visual,
        (Shell::Fish, "replace") => ShellEditMode::Replace,
        (Shell::Fish, "replace_one") => ShellEditMode::ReplaceOne,
        _ => ASSUMED_MODE,
    };

    let symbol = match mode {
        ShellEditMode::Normal => config.vimcmd_symbol,
        ShellEditMode::Visual => config.vimcmd_visual_symbol,
        ShellEditMode::Replace => config.vimcmd_replace_symbol,
        ShellEditMode::ReplaceOne => config.vimcmd_replace_one_symbol,
        ShellEditMode::Insert => {
            if exit_success {
                config.success_symbol
            } else {
                config.error_symbol
            }
        }
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(symbol),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `character`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod test {
    use crate::context::Shell;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn success_status() {
        let expected = Some(format!("{} ", Color::Green.bold().paint("❯")));

        // Status code 0
        let actual = ModuleRenderer::new("character").status(0).collect();
        assert_eq!(expected, actual);

        // No status code
        let actual = ModuleRenderer::new("character").collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn failure_status() {
        let expected = Some(format!("{} ", Color::Red.bold().paint("❯")));

        let exit_values = [1, 54321, -5000];

        for status in &exit_values {
            let actual = ModuleRenderer::new("character").status(*status).collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn custom_symbol() {
        let expected_fail = Some(format!("{} ", Color::Red.bold().paint("✖")));
        let expected_success = Some(format!("{} ", Color::Green.bold().paint("➜")));

        let exit_values = [1, 54321, -5000];

        // Test failure values
        for status in &exit_values {
            let actual = ModuleRenderer::new("character")
                .config(toml::toml! {
                    [character]
                    success_symbol = "[➜](bold green)"
                    error_symbol = "[✖](bold red)"
                })
                .status(*status)
                .collect();
            assert_eq!(expected_fail, actual);
        }

        // Test success
        let actual = ModuleRenderer::new("character")
            .config(toml::toml! {
                [character]
                success_symbol = "[➜](bold green)"
                error_symbol = "[✖](bold red)"
            })
            .status(0)
            .collect();
        assert_eq!(expected_success, actual);
    }

    #[test]
    fn zsh_keymap() {
        let expected_vicmd = Some(format!("{} ", Color::Green.bold().paint("❮")));
        let expected_specified = Some(format!("{} ", Color::Green.bold().paint("V")));
        let expected_other = Some(format!("{} ", Color::Green.bold().paint("❯")));

        // zle keymap is vicmd
        let actual = ModuleRenderer::new("character")
            .shell(Shell::Zsh)
            .keymap("vicmd")
            .collect();
        assert_eq!(expected_vicmd, actual);

        // specified vicmd character
        let actual = ModuleRenderer::new("character")
            .config(toml::toml! {
                [character]
                vicmd_symbol = "[V](bold green)"
            })
            .shell(Shell::Zsh)
            .keymap("vicmd")
            .collect();
        assert_eq!(expected_specified, actual);

        // zle keymap is other
        let actual = ModuleRenderer::new("character")
            .shell(Shell::Zsh)
            .keymap("visual")
            .collect();
        assert_eq!(expected_other, actual);
    }

    #[test]
    fn fish_keymap() {
        let expected_vicmd = Some(format!("{} ", Color::Green.bold().paint("❮")));
        let expected_specified = Some(format!("{} ", Color::Green.bold().paint("V")));
        let expected_visual = Some(format!("{} ", Color::Yellow.bold().paint("❮")));
        let expected_replace = Some(format!("{} ", Color::Purple.bold().paint("❮")));
        let expected_replace_one = expected_replace.clone();
        let expected_other = Some(format!("{} ", Color::Green.bold().paint("❯")));

        // fish keymap is default
        let actual = ModuleRenderer::new("character")
            .shell(Shell::Fish)
            .keymap("default")
            .collect();
        assert_eq!(expected_vicmd, actual);

        // specified vicmd character
        let actual = ModuleRenderer::new("character")
            .config(toml::toml! {
                [character]
                vicmd_symbol = "[V](bold green)"
            })
            .shell(Shell::Fish)
            .keymap("default")
            .collect();
        assert_eq!(expected_specified, actual);

        // fish keymap is visual
        let actual = ModuleRenderer::new("character")
            .shell(Shell::Fish)
            .keymap("visual")
            .collect();
        assert_eq!(expected_visual, actual);

        // fish keymap is replace
        let actual = ModuleRenderer::new("character")
            .shell(Shell::Fish)
            .keymap("replace")
            .collect();
        assert_eq!(expected_replace, actual);

        // fish keymap is replace_one
        let actual = ModuleRenderer::new("character")
            .shell(Shell::Fish)
            .keymap("replace_one")
            .collect();
        assert_eq!(expected_replace_one, actual);

        // fish keymap is other
        let actual = ModuleRenderer::new("character")
            .shell(Shell::Fish)
            .keymap("other")
            .collect();
        assert_eq!(expected_other, actual);
    }

    #[test]
    fn cmd_keymap() {
        let expected_vicmd = Some(format!("{} ", Color::Green.bold().paint("❮")));
        let expected_specified = Some(format!("{} ", Color::Green.bold().paint("V")));
        let expected_other = Some(format!("{} ", Color::Green.bold().paint("❯")));

        // cmd keymap is vi
        let actual = ModuleRenderer::new("character")
            .shell(Shell::Cmd)
            .keymap("vi")
            .collect();
        assert_eq!(expected_vicmd, actual);

        // specified vicmd character
        let actual = ModuleRenderer::new("character")
            .config(toml::toml! {
                [character]
                vicmd_symbol = "[V](bold green)"
            })
            .shell(Shell::Cmd)
            .keymap("vi")
            .collect();
        assert_eq!(expected_specified, actual);

        // cmd keymap is other
        let actual = ModuleRenderer::new("character")
            .shell(Shell::Cmd)
            .keymap("visual")
            .collect();
        assert_eq!(expected_other, actual);
    }
}
