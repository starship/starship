use super::{Context, Module, ModuleConfig};

use crate::configs::username::UsernameConfig;
use crate::formatter::StringFormatter;

#[cfg(not(target_os = "windows"))]
const USERNAME_ENV_VAR: &str = "USER";

#[cfg(target_os = "windows")]
const USERNAME_ENV_VAR: &str = "USERNAME";

/// Creates a module with the current user's username
///
/// Will display the username if any of the following criteria are met:
///     - The current user is root (UID = 0) [1]
///     - The current user isn't the same as the one that is logged in (`$LOGNAME` != `$USER`) [2]
///     - The user is currently connected as an SSH session (`$SSH_CONNECTION`) [3]
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut username = context.get_env(USERNAME_ENV_VAR)?;

    let mut module = context.new_module("username");
    let config: UsernameConfig = UsernameConfig::try_load(module.config);

    let is_root = is_root_user();
    if cfg!(target_os = "windows") && is_root {
        username = "Administrator".to_string();
    }
    let show_username = config.show_always
        || is_root // [1]
        || !is_login_user(context, &username) // [2]
        || is_ssh_session(context); // [3]

    if !show_username {
        return None;
    }

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
            .parse(None, Some(context))
    });
    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `username`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn is_login_user(context: &Context, username: &str) -> bool {
    context
        .get_env("LOGNAME")
        .map_or(true, |logname| logname == username)
}

#[cfg(all(target_os = "windows", not(test)))]
fn is_root_user() -> bool {
    use deelevate::{PrivilegeLevel, Token};
    let token = match Token::with_current_process() {
        Ok(token) => token,
        Err(e) => {
            log::warn!("Failed to get process token: {e:?}");
            return false;
        }
    };
    matches!(
        match token.privilege_level() {
            Ok(level) => level,
            Err(e) => {
                log::warn!("Failed to get privilege level: {e:?}");
                return false;
            }
        },
        PrivilegeLevel::Elevated | PrivilegeLevel::HighIntegrityAdmin
    )
}

#[cfg(all(target_os = "windows", test))]
fn is_root_user() -> bool {
    false
}

#[cfg(not(target_os = "windows"))]
fn is_root_user() -> bool {
    nix::unistd::geteuid() == nix::unistd::ROOT
}

fn is_ssh_session(context: &Context) -> bool {
    let ssh_env = ["SSH_CONNECTION", "SSH_CLIENT", "SSH_TTY"];
    ssh_env.iter().any(|env| context.get_env_os(env).is_some())
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;

    // TODO: Add tests for if root user (UID == 0)
    // Requires mocking

    #[test]
    fn no_env_variables() {
        let actual = ModuleRenderer::new("username").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn no_logname_env_variable() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn logname_equals_user() {
        let actual = ModuleRenderer::new("username")
            .env("LOGNAME", "astronaut")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn ssh_wo_username() {
        // SSH connection w/o username
        let actual = ModuleRenderer::new("username")
            .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn current_user_not_logname() {
        let actual = ModuleRenderer::new("username")
            .env("LOGNAME", "astronaut")
            .env(super::USERNAME_ENV_VAR, "cosmonaut")
            // Test output should not change when run by root/non-root user
            .config(toml::toml! {
                [username]
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("cosmonaut in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn ssh_connection() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
            // Test output should not change when run by root/non-root user
            .config(toml::toml! {
                [username]
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("astronaut in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn ssh_connection_tty() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_TTY", "/dev/pts/0")
            // Test output should not change when run by root/non-root user
            .config(toml::toml! {
                [username]
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("astronaut in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn ssh_connection_client() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_CLIENT", "192.168.0.101 39323 22")
            // Test output should not change when run by root/non-root user
            .config(toml::toml! {
                [username]
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("astronaut in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_always() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            // Test output should not change when run by root/non-root user
            .config(toml::toml! {
                [username]
                show_always = true

                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("astronaut in ");

        assert_eq!(expected, actual.as_deref());
    }
}
