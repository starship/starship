use super::{Context, Module, ModuleConfig};

use crate::configs::devbox_shell::DevboxShellConfig;
use crate::formatter::StringFormatter;

/// Creates a module showing if inside a devbox-shell
///
/// The module will use the `$DEVBOX_SHELL_ENABLED` environment variable to determine if it's
/// inside a devbox-shell and the name of it.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("devbox_shell");
    let config: DevboxShellConfig = DevboxShellConfig::try_load(module.config);

    let is_devbox_shell = context.get_env("DEVBOX_SHELL_ENABLED").is_some();

    if !is_devbox_shell {
        return None;
    };

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
            log::warn!("Error in module `devbox_shell`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn no_env_variables() {
        let actual = ModuleRenderer::new("devbox_shell").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn env_variables() {
        let actual = ModuleRenderer::new("devbox_shell")
            .env(
                "DEVBOX_SHELL_ENABLED",
                "1",
            )
            .collect();
        let expected = Some(format!("via {} ", Color::Purple.bold().paint("</> ")));

        assert_eq!(expected, actual);
    }
}
