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

    let is_sudo_cached = context.exec_cmd("sudo", &["-n", "true"]).is_some();

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
    use crate::{test::ModuleRenderer, utils::CommandOutput};

    #[test]
    fn test_sudo_not_cached() {
        let actual = ModuleRenderer::new("sudo")
            .cmd(
                "command -v sudo",
                Some(CommandOutput {
                    stdout: "/usr/bin/sudo".to_string(),
                    stderr: "".to_string(),
                }),
            )
            .config(toml::toml! {
                [sudo]
                disabled = false
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }
}
