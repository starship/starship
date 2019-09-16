use ansi_term::{Color, Style};
use std::env;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current user's username
///
/// Will display the username if any of the following criteria are met:
///     - The current user isn't the same as the one that is logged in (`$LOGNAME` != `$USER`)
///     - The current user is root (UID = 0)
///     - The user is currently connected as an SSH session (`$SSH_CONNECTION`)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let user = env::var("USER").ok();
    let logname = env::var("LOGNAME").ok();
    let ssh_connection = env::var("SSH_CONNECTION").ok();

    const ROOT_UID: Option<u32> = Some(0);
    let user_uid = get_uid();
    if user != logname || ssh_connection.is_some() || user_uid == ROOT_UID {
        let mut module = context.new_module("username");
        let module_style = get_mod_style(user_uid, &module);
        module.set_style(module_style);
        module.new_segment("username", &user?);

        return Some(module);
    }

    None
}

fn get_uid() -> Option<u32> {
    match Command::new("id").arg("-u").output() {
        Ok(output) => String::from_utf8(output.stdout)
            .map(|uid| uid.trim().parse::<u32>().ok())
            .ok()?,
        Err(_) => None,
    }
}

fn get_mod_style(user_uid: Option<u32>, module: &Module) -> Style {
    match user_uid {
        Some(0) => module
            .config_value_style("style_root")
            .unwrap_or_else(|| Color::Red.bold()),
        _ => module
            .config_value_style("style_user")
            .unwrap_or_else(|| Color::Yellow.bold()),
    }
}
