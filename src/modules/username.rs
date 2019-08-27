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

    let mut module_color = Color::Yellow.bold();

    if user != logname || ssh_connection.is_some() || is_root(&mut module_color) {
        let mut module = context.new_module("username")?;
        module.set_style(module_color);
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

fn is_root(style: &mut Style) -> bool {
    match get_uid() {
        Some(uid) if uid == 0 => {
            style.clone_from(&Color::Red.bold());

            true
        }
        _ => false,
    }
}
