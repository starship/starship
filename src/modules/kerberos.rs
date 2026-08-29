use super::{Context, Module, ModuleConfig};

use crate::configs::kerberos::KerberosConfig;
use crate::formatter::StringFormatter;

/// Creates a module with kerberos ticket status
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("kerberos");
    let config = KerberosConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let ticket_is_valid = context.exec_cmd("klist", &["-s"]).is_some();

    let display_symbol = match ticket_is_valid {
        true => config.symbol,
        false => config.expired_symbol,
    };

    let display_style = match ticket_is_valid {
        true => config.style,
        false => config.expired_style,
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
            log::warn!("Error in module `kerberos`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use ansi_term::Color;

    #[test]
    fn test_kerberos_not_cached() {
        let actual = ModuleRenderer::new("kerberos")
            .cmd("klist -s", None)
            .config(toml::toml! {
                [kerberos]
                disabled = false
            })
            .collect();
        let expected = Some(format!("{}", Color::Red.bold().paint("krb5_❌ ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_kerberos_cached() {
        let actual = ModuleRenderer::new("kerberos")
            .cmd(
                "klist -s",
                Some(CommandOutput {
                    stdout: "".to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .config(toml::toml! {
                [kerberos]
                disabled = false
            })
            .collect();
        let expected = Some(format!("{}", Color::Green.bold().paint("krb5_🎫 ")));

        assert_eq!(expected, actual);
    }
}
