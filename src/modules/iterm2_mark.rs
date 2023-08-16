use super::{Context, Module};
use crate::config::ModuleConfig;
use crate::configs::iterm2_mark::ITerm2MarkConfig;
use crate::segment::Segment;

/// The Control Sequence used by iTerm2.app
const MARKER: &str = "\x1b]133;A\x07 ";

/// Creates a module for the prompt character
///
/// The character segment prints a marker at the beginning of the prompt.
/// The marker is recognizable by iTerm2.app on macOS, such that we can
/// navigate between the marked lines by keystroke Cmd+Shift+Up/Down.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module_ = context.new_module("iterm2_mark");
    let config: ITerm2MarkConfig = ITerm2MarkConfig::try_load(module_.config);

    // When using tmux, the TERM_PROGRAM value will be set to 'tmux' instead of 'iTerm.app'.
    // It is more reliable to checking the ITERM_PROFILE environment variable.
    let iterm_profile = context.get_env("ITERM_PROFILE");
    // If this module is activated forcedly, or the ITERM_PROFILE env variable exists,
    // there is no need to activate it by looking the TERM_PROGRAM env variable.
    if config.force || iterm_profile.is_some() {
        module_.set_segments(Segment::from_text(None, MARKER));
        return Some(module_);
    }

    let term_program = context.get_env("TERM_PROGRAM");
    if let Some(program) = &term_program {
        // This module works only in iTerm2.
        // Make sure that the 'iTerm.app' always exists in the supported program list.
        // Even if the user makes a wrong configuration by `term_programs',
        // it still works correctly.
        let term_programs = {
            let mut tmp = config.term_programs.clone();
            tmp.push("iTerm.app");
            tmp
        };
        if term_programs.contains(&program.as_str()) || config.force {
            module_.set_segments(Segment::from_text(None, MARKER));
            return Some(module_);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::MARKER;
    use crate::test::ModuleRenderer;

    #[test]
    fn iterm2_app() {
        let expected = Some(MARKER.to_string());

        // Status code 0
        let actual = ModuleRenderer::new("iterm2_mark")
            .status(0)
            .env("TERM_PROGRAM", "iTerm.app")
            .collect();
        assert_eq!(expected, actual);

        // No status code
        let actual = ModuleRenderer::new("iterm2_mark")
            .status(127)
            .env("TERM_PROGRAM", "iTerm.app")
            .collect();
        assert_eq!(expected, actual);

        // No status code
        let actual = ModuleRenderer::new("iterm2_mark")
            .env("TERM_PROGRAM", "iTerm.app")
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn non_iterm2_app() {
        let expected = None;

        // For macOS Terminal.app with success exit.
        let actual = ModuleRenderer::new("iterm2_mark")
            .status(0)
            .env("TERM_PROGRAM", "Apple_Terminal")
            .collect();
        assert_eq!(expected, actual);

        // For macOS Terminal.app with failure exit.
        let actual = ModuleRenderer::new("iterm2_mark")
            .status(127)
            .env("TERM_PROGRAM", "Apple_Terminal")
            .collect();
        assert_eq!(expected, actual);

        // For macOS Terminal.app with default exit status.
        let actual = ModuleRenderer::new("iterm2_mark")
            .env("TERM_PROGRAM", "Apple_Terminal")
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn non_iterm2_app_enabled_by_env() {
        let expected = Some(MARKER.to_string());

        let actual = ModuleRenderer::new("iterm2_mark")
            .env("ITERM_PROFILE", "Default")
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn non_iterm2_app_enabled_by_extra_programs() {
        let expected = Some(MARKER.to_string());
        // Add extra supported Terminal program names.
        let config = toml::toml! {
            iterm2_mark = { term_programs = ["iTerm.app", "Apple_Terminal"] }
        };
        let actual = ModuleRenderer::new("iterm2_mark")
            .status(0)
            .env("TERM_PROGRAM", "Apple_Terminal")
            .config(config)
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn non_iterm2_app_force() {
        let expected = Some(MARKER.to_string());

        let config = toml::toml! {
            iterm2_mark = { force = true }
        };

        let actual = ModuleRenderer::new("iterm2_mark")
            .status(0)
            .env("TERM_PROGRAM", "Apple_Terminal")
            .config(config)
            .collect();
        assert_eq!(expected, actual);
    }
}
