use super::{Context, Module, SegmentConfig};

use crate::config::RootModuleConfig;
use crate::configs::exit_code::ExitCodeConfig;

/// Outputs the exit status of the previous command if non-zero
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("exit_code");
    let config: ExitCodeConfig = ExitCodeConfig::try_load(module.config);

    if config.disabled {
        return None;
    };

    let props = &context.properties;
    let exit_code_default = std::string::String::from("0");
    let exit_code = props.get("status_code").unwrap_or(&exit_code_default);

    if exit_code == &exit_code_default {
        return None;
    }

    module.get_prefix().set_value(config.prefix);
    module.get_suffix().set_value(config.suffix);
    module.set_style(config.style);
    module.create_segment("exit_code", &SegmentConfig::new(&exit_code));

    Some(module)
}
