use std::env;

use super::utils::query_parser::*;
use super::{Context, Module};
use std::ffi::OsString;

use crate::config::RootModuleConfig;
use crate::configs::hostname::HostnameConfig;
use crate::segment::Segment;

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

    //rustc doesn't let you do an "if" and an "if let" in the same if statement
    // if this changes in the future this can become a lot cleaner
    let host = if config.trim_at != "" {
        if let Some(index) = host.find(config.trim_at) {
            host.split_at(index).0
        } else {
            host.as_ref()
        }
    } else {
        host.as_ref()
    };

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "host" => Some(Segment {
                _name: "host".to_string(),
                value: host.to_string(),
                style,
            }),
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}
