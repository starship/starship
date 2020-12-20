use super::{Context, Module, RootModuleConfig};

use crate::configs::username::UsernameConfig;
use crate::formatter::StringFormatter;
use crate::utils;

const ROOT_UID: Option<u32> = Some(0);
#[cfg(not(target_os = "windows"))]
const USERNAME_ENV_VAR: &str = "USER";

#[cfg(target_os = "windows")]
const USERNAME_ENV_VAR: &str = "USERNAME";

/// Creates a module with the current user's username
///
/// Will display the username if any of the following criteria are met:
///     - The current user isn't the same as the one that is logged in (`$LOGNAME` != `$USER`)
///     - The current user is root (UID = 0)
///     - The user is currently connected as an SSH session (`$SSH_CONNECTION`)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let username = context.get_env(USERNAME_ENV_VAR)?;
    let logname = context.get_env("LOGNAME");

    let user_uid = get_uid();

    let is_not_login = logname.is_some() && username != logname.unwrap_or_default();
    let is_root = user_uid == ROOT_UID;

    let mut module = context.new_module("username");
    let config: UsernameConfig = UsernameConfig::try_load(module.config);

    if is_not_login || is_ssh_connection(&context) || is_root || config.show_always {
        let parsed = StringFormatter::new(config.format).and_then(|formatter| {
            formatter
                .map_style(|variable| match variable {
                    "style" => {
                        let module_style = if is_root {
                            config.style_root
                        } else {
                            config.style_user
                        };
                        Some(Ok(module_style))
                    }
                    _ => None,
                })
                .map(|variable| match variable {
                    "user" => Some(Ok(&username)),
                    _ => None,
                })
                .parse(None)
        });
        module.set_segments(match parsed {
            Ok(segments) => segments,
            Err(error) => {
                log::warn!("Error in module `username`:\n{}", error);
                return None;
            }
        });

        Some(module)
    } else {
        None
    }
}

fn is_ssh_connection(context: &Context) -> bool {
    let ssh_env = ["SSH_CONNECTION", "SSH_CLIENT", "SSH_TTY"];
    ssh_env.iter().any(|env| context.get_env(env).is_some())
}

fn get_uid() -> Option<u32> {
    utils::exec_cmd("id", &["-u"])?
        .stdout
        .trim()
        .parse::<u32>()
        .ok()
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::io;

    // TODO: Add tests for if root user (UID == 0)
    // Requires mocking

    #[test]
    fn no_env_variables() -> io::Result<()> {
        let actual = ModuleRenderer::new("username").collect();
        let expected = None;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn no_logname_env_variable() -> io::Result<()> {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn logname_equals_user() -> io::Result<()> {
        let actual = ModuleRenderer::new("username")
            .env("LOGNAME", "astronaut")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn ssh_wo_username() -> io::Result<()> {
        // SSH connection w/o username
        let actual = ModuleRenderer::new("username")
            .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn current_user_not_logname() -> io::Result<()> {
        let actual = ModuleRenderer::new("username")
            .env("LOGNAME", "astronaut")
            .env(super::USERNAME_ENV_VAR, "cosmonaut")
            .collect();
        let expected = Some(format!("{} in ", Color::Yellow.bold().paint("cosmonaut")));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn ssh_connection() -> io::Result<()> {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
            .collect();
        let expected = Some(format!("{} in ", Color::Yellow.bold().paint("astronaut")));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn ssh_connection_tty() -> io::Result<()> {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_TTY", "/dev/pts/0")
            .collect();
        let expected = Some(format!("{} in ", Color::Yellow.bold().paint("astronaut")));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn ssh_connection_client() -> io::Result<()> {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_CLIENT", "192.168.0.101 39323 22")
            .collect();
        let expected = Some(format!("{} in ", Color::Yellow.bold().paint("astronaut")));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn show_always() -> io::Result<()> {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .config(toml::toml! {
                [username]
                show_always = true
            })
            .collect();
        let expected = Some(format!("{} in ", Color::Yellow.bold().paint("astronaut")));

        assert_eq!(expected, actual);
        Ok(())
    }
}
