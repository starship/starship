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
            log::warn!("Error in module `netns`: \n{}", error);
            return None;
        }
    });

    Some(module)
}
