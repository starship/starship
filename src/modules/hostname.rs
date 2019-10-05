use std::env;

use super::{Context, Module};
use std::ffi::OsString;

use crate::config::RootModuleConfig;
use crate::configs::hostname::HostnameConfig;

/// Creates a module with the system hostname
///
/// Will display the hostname if all of the following criteria are met:
///     - hostname.disabled is absent or false
///     - hostname.ssh_only is false OR the user is currently connected as an SSH session (`$SSH_CONNECTION`)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("hostname");
    let config: HostnameConfig = HostnameConfig::try_load(module.config);

    let ssh_connection = env::var("SSH_CONNECTION").ok();
    if config.ssh_only && ssh_connection.is_none() {
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

    module.set_style(config.style);
    // TODO: Rewrite hostname segment with segment config
    module.new_segment(
        "hostname",
        &format!("{}{}{}", config.prefix, host, config.suffix),
    );
    module.get_prefix().set_value("on ");

    Some(module)
}
