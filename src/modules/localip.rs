use super::{Context, Module};

use crate::config::ModuleConfig;
use crate::configs::localip::LocalipConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the ipv4 address of the local machine.
///
/// The `local-ip-address` crate is used to determine the local IP address of your machine.
/// Avoid any external network connections to not compromise privacy.
///
/// Will display the ip if all of the following criteria are met:
///     - localip.disabled is false
///     - `localip.ssh_only` is false OR the user is currently connected as an SSH session (`$SSH_CONNECTION`)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("localip");
    let config: LocalipConfig = LocalipConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    let ssh_connection = context.get_env("SSH_CONNECTION");
    if config.ssh_only && ssh_connection.is_none() {
        return None;
    }

    let localip = match local_ip_address::local_ip() {
        Ok(ip) => ip,
        Err(err) => {
            log::warn!("unable to determine local ipv4 address: {}", err);
            return None;
        }
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "localipv4" => Some(Ok(localip.to_string())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `localip`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::{Color, Style};

    macro_rules! get_localip {
        () => {
            local_ip_address::local_ip().unwrap().to_string()
        };
    }

    #[test]
    fn is_ipv4_format() {
        let localip = get_localip!();
        assert!(regex::Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$")
            .unwrap()
            .is_match(&localip));
    }

    #[test]
    fn ssh_only_false() {
        let localip = get_localip!();
        let actual = ModuleRenderer::new("localip")
            .config(toml::toml! {
                [localip]
                ssh_only = false
                disabled = false
            })
            .collect();
        let expected = Some(format!("{} ", style().paint(localip)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_ssh() {
        let actual = ModuleRenderer::new("localip")
            .config(toml::toml! {
                [localip]
                ssh_only = true
                disabled = false
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn ssh() {
        let localip = get_localip!();
        let actual = ModuleRenderer::new("localip")
            .config(toml::toml! {
                [localip]
                ssh_only = true
                disabled = false
            })
            .env("SSH_CONNECTION", "something")
            .collect();
        let expected = Some(format!("{} ", style().paint(localip)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank() {
        let actual = ModuleRenderer::new("localip")
            .env("SSH_CONNECTION", "something")
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    fn style() -> Style {
        Color::Yellow.bold()
    }
}
