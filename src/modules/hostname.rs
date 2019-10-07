use ansi_term::Color;
use std::env;

use super::{Context, Module};
use std::ffi::OsString;

/// Creates a module with the system hostname
///
/// Will display the hostname if all of the following criteria are met:
///     - hostname.disabled is absent or false
///     - hostname.ssh_only is false OR the user is currently connected as an SSH session (`$SSH_CONNECTION`)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("hostname");
    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Green.bold().dimmed());

    let ssh_connection = env::var("SSH_CONNECTION").ok();
    if module.config_value_bool("ssh_only").unwrap_or(true) && ssh_connection.is_none() {
        return None;
    }

    let os_hostname: OsString = gethostname::gethostname();

    let host = match os_hostname.into_string() {
        Ok(host) => host,
        Err(bad) => {
            log::debug!("hostname is not valid UTF!\n{:?}", bad);
            return None;
        }
    };

    let trim_at = module.config_value_str("trim_at").unwrap_or(".");

    //rustc doesn't let you do an "if" and an "if let" in the same if statement
    // if this changes in the future this can become a lot cleaner
    let host = if trim_at != "" {
        if let Some(index) = host.find(trim_at) {
            host.split_at(index).0
        } else {
            host.as_ref()
        }
    } else {
        host.as_ref()
    };

    let prefix = module.config_value_str("prefix").unwrap_or("").to_owned();
    let suffix = module.config_value_str("suffix").unwrap_or("").to_owned();

    module.set_style(module_style);
    module.new_segment("hostname", &format!("{}{}{}", prefix, host, suffix));
    module.get_prefix().set_value("on ");

    Some(module)
}
