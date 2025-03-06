use super::{Context, Module};

use crate::config::ModuleConfig;
use crate::configs::hostname::HostnameConfig;
use crate::formatter::StringFormatter;

#[cfg(not(windows))]
use whoami::fallible::hostname;
// On Windows, whoami::hostname() returns the NetBIOS name,
// but we prefer the "hostname" returned by whoami::devicname()
// which does a better job of preserving case and returns the
// DNS name.
#[cfg(windows)]
use whoami::fallible::devicename as hostname;

/// Creates a module with the system hostname
///
/// Will display the hostname if all of the following criteria are met:
///     - `hostname.disabled` is absent or false
///     - `hostname.ssh_only` is false OR the user is currently connected as an SSH session (`$SSH_CONNECTION`)
///     - `hostname.ssh_only` is false AND `hostname.detect_env_vars` is either empty or contains a defined environment variable
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("hostname");
    let config: HostnameConfig = HostnameConfig::try_load(module.config);

    let ssh_connection = context.get_env("SSH_CONNECTION");

    if (config.ssh_only && ssh_connection.is_none())
        || !context.detect_env_vars(&config.detect_env_vars)
    {
        return None;
    }

    let host = hostname()
        .inspect_err(|e| log::warn!("Failed to get hostname: {e}"))
        .ok()?;

    //rustc doesn't let you do an "if" and an "if let" in the same if statement
    // if this changes in the future this can become a lot cleaner
    let mut host = if !config.trim_at.is_empty() {
        if let Some(index) = host.find(config.trim_at) {
            host.split_at(index).0
        } else {
            host.as_ref()
        }
    } else {
        host.as_ref()
    };

    if let Some(&alias) = config.aliases.get(host) {
        host = alias;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "ssh_symbol" => {
                    if ssh_connection.is_some() {
                        Some(config.ssh_symbol)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "hostname" => Some(Ok(host)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `hostname`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use super::hostname;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::{Color, Style};
    use unicode_segmentation::UnicodeSegmentation;

    macro_rules! get_hostname {
        () => {
            if let Ok(hostname) = hostname() {
                hostname
            } else {
                println!(
                    "hostname was not tested because gethostname failed! \
                     This could be caused by your hostname containing invalid UTF."
                );
                return;
            }
        };
    }

    #[test]
    fn ssh_only_false_with_empty_detect_env_vars() {
        let hostname = get_hostname!();
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = false
                trim_at = ""
                detect_env_vars = []
            })
            .collect();

        let expected = Some(format!("{} in ", style().paint(hostname)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn ssh_only_false_with_matching_negated_env_var() {
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = false
                trim_at = ""
                detect_env_vars = ["!NEGATED"]
            })
            .env("NEGATED", "true")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn ssh_only_false_with_only_negated_env_vars() {
        let hostname = get_hostname!();
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = false
                trim_at = ""
                detect_env_vars = ["!NEGATED_ONE", "!NEGATED_TWO", "!NEGATED_THREE"]
            })
            .collect();

        let expected = Some(format!("{} in ", style().paint(hostname)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn ssh_only_false_with_matching_env_var() {
        let hostname = get_hostname!();
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = false
                trim_at = ""
                detect_env_vars = ["FORCE_HOSTNAME"]
            })
            .env("FORCE_HOSTNAME", "true")
            .collect();

        let expected = Some(format!("{} in ", style().paint(hostname)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn ssh_only_false_without_matching_env_vars() {
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = false
                trim_at = ""
                detect_env_vars = ["FORCE_HOSTNAME", "!NEGATED"]
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn ssh_only_false_ssh() {
        let hostname = get_hostname!();
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = false
                trim_at = ""
            })
            .collect();
        let expected = Some(format!("{} in ", style().paint(hostname)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_ssh() {
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = true
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn ssh() {
        let hostname = get_hostname!();
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = true
                trim_at = ""
            })
            .env("SSH_CONNECTION", "something")
            .collect();
        let expected = Some(format!(
            "{} in ",
            style().paint("🌐 ".to_owned() + hostname.as_str())
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_trim_at() {
        let hostname = get_hostname!();
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = false
                trim_at = ""
            })
            .collect();
        let expected = Some(format!("{} in ", style().paint(hostname)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn trim_at() {
        let hostname = get_hostname!();
        let mut hostname_iter = hostname.graphemes(true);
        let remainder = hostname_iter.next().unwrap_or_default();
        let trim_at = hostname_iter.collect::<String>();
        let actual = ModuleRenderer::new("hostname")
            .config(toml::toml! {
                [hostname]
                ssh_only = false
                trim_at = trim_at
            })
            .collect();
        let expected = Some(format!("{} in ", style().paint(remainder)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_alias() {
        let hostname = get_hostname!();
        let mut toml_config = toml::toml!(
            [hostname]
            ssh_only = false
            trim_at = ""
            aliases = {}
        );
        toml_config["hostname"]["aliases"]
            .as_table_mut()
            .unwrap()
            .insert(hostname, toml::Value::String("homeworld".to_string()));
        let actual = ModuleRenderer::new("hostname")
            .config(toml_config)
            .collect();

        let expected = Some(format!("{} in ", style().paint("homeworld")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_alias_with_trim_at() {
        let hostname = get_hostname!();

        let mut hostname_iter = hostname.graphemes(true);
        let remainder = hostname_iter.next().unwrap_or_default();
        let trim_at = hostname_iter.collect::<String>();

        // Trimmed hostname needs to be non-empty
        if remainder.is_empty() {
            log::warn!("Skipping test_alias_with_trim_at because hostname is too short");
            return;
        }
        let mut toml_config = toml::toml!(
            [hostname]
            ssh_only = false
            trim_at = trim_at
            aliases = {}
        );
        toml_config["hostname"]["aliases"]
            .as_table_mut()
            .unwrap()
            .insert(remainder.to_string(), toml::Value::String("🌍".to_string()));
        let actual = ModuleRenderer::new("hostname")
            .config(toml_config)
            .collect();

        let expected = Some(format!("{} in ", style().paint("🌍")));
        assert_eq!(expected, actual);
    }

    fn style() -> Style {
        Color::Green.bold().dimmed()
    }
}
