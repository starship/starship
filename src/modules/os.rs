use super::{Context, Module, ModuleConfig};

use crate::configs::os::OSConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current operating system
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("os");
    let config: OSConfig = OSConfig::try_load(module.config);

    let os = os_info::get();

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => config.get_symbol(&os.os_type()),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "bitness" => Some(Ok(os.bitness().to_string())),
                "codename" => os.codename().map(String::from).map(Ok),
                "edition" => os.edition().map(String::from).map(Ok),
                "name" => Some(Ok(os.os_type().to_string())),
                "type" => Some(Ok(format!("{:?}", os.os_type()))),
                "version" => Some(Ok(os.version().to_string())),
                _ => None,
            })
            .parse(None, Some(context))
    });
    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `os`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
