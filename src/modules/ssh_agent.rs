use super::{Context, Module, ModuleConfig};

use crate::configs::ssh_agent::SshAgentConfig;
use crate::formatter::StringFormatter;

/// Creates a module with status of ssh-agent added key identities
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("ssh_agent");
    let config = SshAgentConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let num_keys = context
        .exec_cmd("ssh-add", &["-l"])
        .map(|out| out.stdout.split_terminator('\n').count());

    let display_symbol = match num_keys {
        Some(1) => config.symbol,
        Some(n) if n != 0 => config.symbol_multi,
        _ => config.none_symbol,
    };

    let display_style = match num_keys {
        Some(n) if n > 0 => config.style,
        _ => config.none_style,
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(display_symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(display_style)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `ssh_agent`:\n{}", error);
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
    fn test_ssh_key_not_added() {
        let actual = ModuleRenderer::new("ssh_agent")
            .cmd("klist -s", None)
            .config(toml::toml! {
                [ssh_agent]
                disabled = false
            })
            .collect();
        let expected = Some(format!("{}", Color::Red.bold().paint("ssh  ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ssh_key_added() {
        let actual = ModuleRenderer::new("ssh_agent")
            .cmd(
                "ssh-add -l",
                Some(CommandOutput {
                    stdout:
                        "256 SHA256:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx x@x.x (ED25519)\n"
                            .to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .config(toml::toml! {
                [ssh_agent]
                disabled = false
            })
            .collect();
        let expected = Some(format!("{}", Color::Green.bold().paint("ssh  ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_muliple_ssh_keys_added() {
        let actual = ModuleRenderer::new("ssh_agent")
            .cmd(
                "ssh-add -l",
                Some(CommandOutput {
                    stdout:
                        "256 SHA256:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx x@x.x (ED25519)\n256 SHA256:yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy y@y.y (ED25519)\n"
                            .to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .config(toml::toml! {
                [ssh_agent]
                disabled = false
            })
            .collect();
        let expected = Some(format!("{}", Color::Green.bold().paint("ssh  ")));

        assert_eq!(expected, actual);
    }
}
