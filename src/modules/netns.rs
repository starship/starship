use super::{Context, Module};

#[cfg(not(target_os = "linux"))]
pub fn module<'a>(_context: &'a Context) -> Option<Module<'a>> {
    None
}

#[cfg(target_os = "linux")]
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    use crate::{config::ModuleConfig, configs::netns::NetnsConfig, formatter::StringFormatter};

    fn netns_name(context: &Context) -> Option<String> {
        context
            .exec_cmd("ip", &["netns", "identify"])
            .map(|output| output.stdout.trim().to_string())
            .filter(|name| !name.is_empty())
    }

    let mut module = context.new_module("netns");
    let config = NetnsConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let netns_name = netns_name(context)?;

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
            .map(|variable| match variable {
                "name" => Some(Ok(&netns_name)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `netns`: \n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    #[cfg(target_os = "linux")]
    use crate::utils::CommandOutput;
    #[cfg(target_os = "linux")]
    use nu_ansi_term::Color;

    #[cfg(target_os = "linux")]
    fn mock_ip_netns_identify(netns_name: &str) -> Option<CommandOutput> {
        Some(CommandOutput {
            stdout: format!("{netns_name}\n"),
            stderr: String::new(),
        })
    }

    #[test]
    fn test_none_if_disabled() {
        let expected = None;
        let actual = ModuleRenderer::new("netns")
            .config(toml::toml! {
               [netns]
               disabled = true
            })
            .collect();

        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_netns_identify() {
        let actual = ModuleRenderer::new("netns")
            .config(toml::toml! {
               [netns]
               disabled = false
            })
            .cmd("ip netns identify", mock_ip_netns_identify("test_netns"))
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Blue.bold().dimmed().paint("🛜 [test_netns]")
        ));

        assert_eq!(actual, expected);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_netns_identify_empty() {
        let actual = ModuleRenderer::new("netns")
            .config(toml::toml! {
               [netns]
               disabled = false
            })
            .cmd("ip netns identify", mock_ip_netns_identify(""))
            .collect();

        let expected = None;

        assert_eq!(actual, expected);
    }
}
