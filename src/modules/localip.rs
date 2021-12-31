use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::localip::LocalipConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the ipv4 address of the local machine.
///
/// The `local_ipaddress` crate is used to determine the local IP address of your machine.
/// An accurate and fast way, especially if there are multiple IP addresses available,
/// is to connect a UDP socket and then reading its local endpoint.
///
/// Will display the ip if all of the following criteria are met:
///     - localip.disabled is absent or false
///     - localip.ssh_only is false OR the user is currently connected as an SSH session (`$SSH_CONNECTION`)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("localip");
    let config: LocalipConfig = LocalipConfig::try_load(module.config);

    let ssh_connection = context.get_env("SSH_CONNECTION");
    if config.ssh_only && ssh_connection.is_none() {
        return None;
    }

    let localip = local_ipaddress::get().unwrap();
    if localip.is_empty() {
        log::warn!("unable to determine local ipv4 address");
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "localipv4" => Some(Ok(&localip)),
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
    use ansi_term::{Color, Style};

    macro_rules! get_localip {
        () => {
            if let Some(localip) = local_ipaddress::get() {
                localip
            } else {
                println!(
                    "localip was not tested because socket connection failed! \
                     This could be caused by an unconventional network setup."
                );
                return;
            }
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
            })
            .collect();
        let expected = Some(format!("@{}", style().paint(localip)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_ssh() {
        let actual = ModuleRenderer::new("localip")
            .config(toml::toml! {
                [localip]
                ssh_only = true
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
                trim_at = ""
            })
            .env("SSH_CONNECTION", "something")
            .collect();
        let expected = Some(format!("@{}", style().paint(localip)));

        assert_eq!(expected, actual);
    }

    fn style() -> Style {
        Color::Yellow.bold()
    }
}
