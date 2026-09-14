use super::{Context, Module, ModuleConfig};

use crate::configs::upterm::UptermConfig;
use crate::formatter::StringFormatter;

/// Creates a module showing if inside an upterm session
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("upterm");
    let config: UptermConfig = UptermConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let is_upterm = context.get_env("UPTERM_ADMIN_SOCKET").is_some();

    if !is_upterm {
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
            log::warn!("Error in module `upterm`:\n{error}");
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
        let actual = ModuleRenderer::new("upterm").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn env_variables() {
        let actual = ModuleRenderer::new("upterm")
            .env("UPTERM_ADMIN_SOCKET", "/tmp/upterm.sock")
            .collect();
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🆙 ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn disabled() {
        let actual = ModuleRenderer::new("upterm")
            .env("UPTERM_ADMIN_SOCKET", "/tmp/upterm.sock")
            .config(toml::toml! {
                [upterm]
                disabled = true
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }
}
