use std::env;

use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::username::UsernameConfig;
use crate::utils;

/// Creates a module with the current user's username
///
/// Will display the username if any of the following criteria are met:
///     - The current user isn't the same as the one that is logged in (`$LOGNAME` != `$USER`)
///     - The current user is root (UID = 0)
///     - The user is currently connected as an SSH session (`$SSH_CONNECTION`)
pub async fn module<'a>(context: &'a Context<'_>) -> Option<Module<'a>> {
    let user = env::var("USER").ok();
    let logname = env::var("LOGNAME").ok();
    let ssh_connection = env::var("SSH_CONNECTION").ok();

    const ROOT_UID: Option<u32> = Some(0);
    let user_uid = get_uid().await;

    let mut module = context.new_module("username");
    let config: UsernameConfig = UsernameConfig::try_load(module.config);

    if user != logname || ssh_connection.is_some() || user_uid == ROOT_UID || config.show_always {
        let module_style = match user_uid {
            Some(0) => config.style_root,
            _ => config.style_user,
        };

        module.set_style(module_style);
        module.create_segment("username", &SegmentConfig::new(&user?));

        Some(module)
    } else {
        None
    }
}

async fn get_uid() -> Option<u32> {
    utils::exec_cmd("id", &["-u"])
        .await?
        .stdout
        .trim()
        .parse::<u32>()
        .ok()
}
