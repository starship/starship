use super::utils::format;
use super::{Context, Module, ModuleConfig};

use crate::configs::guix_shell::GuixShellConfig;
use crate::formatter::StringFormatter;

/// Creates a module showing if inside a guix-shell
///
/// The module will use the `$GUIX_ENVIRONMENT` environment variable to determine if it's
/// inside a guix-shell and the name of it.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("guix_shell");
    let config: GuixShellConfig = GuixShellConfig::try_load(module.config);

    let is_guix_shell = context.get_env("GUIX_ENVIRONMENT").is_some();

    if !is_guix_shell {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_variables_to_segments(|variable| match variable {
                "symbol" => Some(format::styled(config.symbol, config.style, context, |_| {
                    None
                })),
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
            log::warn!("Error in module `guix_shell`:\n{error}");
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
    fn conditional_environment_symbol() {
        for (symbol, expected) in [("guix", Some("guix ".to_owned())), ("", None)] {
            let actual = ModuleRenderer::new("guix_shell")
                .env("GUIX_ENVIRONMENT", "/gnu/store/test-profile")
                .config(toml::toml! {
                    [guix_shell]
                    symbol = symbol
                    format = "($symbol )"
                })
                .collect();
            assert_eq!(expected, actual, "symbol: {symbol}");
        }
    }

    #[test]
    fn no_env_variables() {
        let actual = ModuleRenderer::new("guix_shell").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn env_variables() {
        let actual = ModuleRenderer::new("guix_shell")
            .env(
                "GUIX_ENVIRONMENT",
                "/gnu/store/7vmfs4khf4fllsh83kqkxssbw3437qsh-profile",
            )
            .collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐃 ")));

        assert_eq!(expected, actual);
    }
}
