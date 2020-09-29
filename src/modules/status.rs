use super::{Context, Module, RootModuleConfig};

use crate::configs::status::StatusConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the status of the last command
///
/// Will display the status only if it is not 0
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let exit_code = context
        .properties
        .get("status_code")
        .map_or("0", String::as_str);

    if exit_code == "0" {
        None
    } else {
        let mut module = context.new_module("status");
        let config = StatusConfig::try_load(module.config);

        // As we default to disabled=true, we have to check here after loading our config module,
        // before it was only checking against whatever is in the config starship.toml
        if config.disabled {
            return None;
        };

        let parsed = StringFormatter::new(config.format).and_then(|formatter| {
            formatter
                .map_meta(|var, _| match var {
                    "symbol" => Some(config.symbol),
                    _ => None,
                })
                .map_style(|variable| match variable {
                    "style" => Some(Ok(config.style)),
                    _ => None,
                })
                .map(|variable| match variable {
                    "status" => Some(Ok(exit_code)),
                    _ => None,
                })
                .parse(None)
        });

        module.set_segments(match parsed {
            Ok(segments) => segments,
            Err(_error) => {
                log::warn!("Error parsing format string in `status.format`");
                return None;
            }
        });
        Some(module)
    }
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::io;

    use crate::test::ModuleRenderer;

    #[test]
    fn success_status() -> io::Result<()> {
        let expected = None;

        // Status code 0
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                disabled = false
            })
            .status(0)
            .collect();
        assert_eq!(expected, actual);

        // No status code
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                disabled = false
            })
            .collect();
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn not_enabled() -> io::Result<()> {
        let expected = None;

        let actual = ModuleRenderer::new("status").status(1).collect();
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn failure_status() -> io::Result<()> {
        let exit_values = [1, 2, 130];

        for status in exit_values.iter() {
            let expected = Some(format!(
                "{} ",
                Color::Red.bold().paint(format!("✖{}", status))
            ));
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }

        Ok(())
    }
}
