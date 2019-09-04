use ansi_term::{Color, Style};
use std::env;
use std::process::Command;

use super::{Context, Module};
use std::ffi::OsString;

/// Creates a module with the system hostname
///
/// Will display the hostname if all of the following criteria are met:
///     - host.disabled is absent or false
///     - host.ssh_only is false OR the user is currently connected as an SSH session (`$SSH_CONNECTION`)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("host")?;

    if module.config_value_bool("disabled").unwrap_or(false) {
        return None;
    }

    let ssh_connection = env::var("SSH_CONNECTION").ok();
    if module.config_value_bool("ssh_only").unwrap_or(true) && ssh_connection.is_none() {
        return None;
    }

    let os_hostname: OsString = gethostname::gethostname();

    let (host, style) = match os_hostname.into_string() {
        Ok(host) => (host, Color::Green.bold().dimmed()),
        Err(_) => (String::from("<invalid UTF>"), Color::Red.bold()),
    };

    let prefix = module.config_value_str("prefix").unwrap_or("").to_owned();
    let suffix = module.config_value_str("suffix").unwrap_or("").to_owned();

    module.set_style(style);
    module.new_segment("host", &format!("{}{}{}", prefix, host, suffix));
    module.get_prefix().set_value("on ");

    Some(module)
}
