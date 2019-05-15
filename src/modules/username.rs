use std::env;
use std::process::Command;
use ansi_term::Color;

use super::{Context, Module};

/// Creates a segment with the current user's username
///
/// Will display the Node.js version if any of the following criteria are met:
///     - The current user isn't the same as the one that is logged in ($LOGNAME != $USER)
///     - The current user is root (UID = 0)
///     - The user is currently connected as an SSH session ($SSH_CONNECTION)
pub fn segment(_context: &Context) -> Option<Module> {
    let user = env::var("USER").unwrap_or("".to_string());
    let logname = env::var("LOGNAME").unwrap_or("".to_string());
    let ssh_connection = env::var("SSH_CONNECTION").unwrap_or("".to_string());
    let uid = get_uid().unwrap_or(1000);

    if user != logname || uid == 0 || !ssh_connection.is_empty() {
        let module_color = if uid == 0 { Color::Red.bold() } else { Color::Yellow.bold() };

        let mut module = Module::new("username");
        module.set_style(module_color);
        module.get_suffix().set_value(" on ");
        module.new_segment("username", user);

        return Some(module);
    }

    None
}

fn get_uid() -> Option<u32> {
    match Command::new("id").arg("-u").output() {
        Ok(output) => {
            let uid = String::from_utf8(output.stdout)
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap();
            Some(uid)
        },
        Err(_) => None,
    }
}