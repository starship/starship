use super::{Context, Module, RootModuleConfig};

use crate::configs::status::StatusConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the status of the last command
///
/// Will display the status only if it is not 0
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let exit_code_default = std::string::String::from("0");
    let exit_code = context
        .properties
        .get("status_code")
        .unwrap_or(&exit_code_default);

    if exit_code == "0" {
        None
    } else {
        let mut module = context.new_module("status");
        let config = StatusConfig::try_load(module.config);

        let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
            formatter.map(|variable| match variable {
                "status" => Some(exit_code.to_owned()),
                _ => None,
            })
        } else {
            log::warn!("Error parsing format string in `status.format`");
            return None;
        };
        module.set_segments(formatter.parse(None));
        module.get_prefix().set_value("");
        module.get_suffix().set_value("");
        Some(module)
    }
}
