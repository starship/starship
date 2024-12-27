use super::{Context, Module};

#[cfg(not(target_os = "linux"))]
pub fn module<'a>(_context: &'a Context) -> Option<Module<'a>> {
    None
}

#[cfg(target_os = "linux")]
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    use crate::{config::ModuleConfig, configs::netns::NetnsConfig};

    let module = context.new_module("netns");
    let config = NetnsConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    Some(module)
}
