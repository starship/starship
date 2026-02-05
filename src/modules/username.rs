use super::{Context, Detected, Module, ModuleConfig};
use gix::bstr::ByteSlice;

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
///     - The option `username.detect_env_vars` is set with a not negated environment variable [4]
///     - The option `username.show_git_user` is enabled and we're in a Git repository [5]
/// Does not display the username:
///     - If the option `username.detect_env_vars` is set with a negated environment variable [A]
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    #[cfg(not(any(test, target_os = "android")))]
    let mut username = whoami::username()
        .inspect_err(|e| log::debug!("Failed to get username {e:?}"))
        .ok()
        .or_else(|| context.get_env(USERNAME_ENV_VAR))?;

    #[cfg(any(test, target_os = "android"))]
    let mut username = context.get_env(USERNAME_ENV_VAR)?;

    let mut module = context.new_module("username");
    let config: UsernameConfig = UsernameConfig::try_load(module.config);
    let has_detected_env_var = context.detect_env_vars2(&config.detect_env_vars);

    let is_root = is_root_user();
    if cfg!(target_os = "windows") && is_root {
        username = "Administrator".to_string();
    }

    // Check if we're in a Git repository and get Git user if configured
    let is_git_repo = is_git_repository(context);
    let git_user = if config.show_git_user && is_git_repo {
        get_git_user(context)
    } else {
        None
    };

    // If we have a git user and show_git_user is enabled, use that instead of system username
    if let Some(git_username) = git_user {
        username = git_username;
    }

    let show_username = config.show_always
        || is_root // [1] Always show for root user
        || !is_login_user(context, &username) // [2] Show if current user != logged-in user
        || is_ssh_session(context) // [3] Show for SSH sessions
        || has_detected_env_var == Detected::Yes // [4] Show if env var detection matches
        || (config.show_git_user && is_git_repo); // [5] Show if in Git repo and feature enabled

    // Don't show username if conditions aren't met or if negated env var is detected
    if !show_username || has_detected_env_var == Detected::Negated {
        return None; // [A]
    }

    // Apply aliases if configured (e.g., map "john" to "john.doe")
    if let Some(&alias) = config.aliases.get(&username) {
        username = alias.to_string();
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
            log::warn!("Error in module `username`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn is_login_user(context: &Context, username: &str) -> bool {
    context
        .get_env("LOGNAME")
        .is_none_or(|logname| logname == username)
}

/// Check if current directory is a Git repository
#[cfg(not(test))]
fn is_git_repository(context: &Context) -> bool {
    gix::discover(&context.current_dir).is_ok()
}

/// Check if current directory is a Git repository (test version using mock env var)
#[cfg(test)]
fn is_git_repository(context: &Context) -> bool {
    context.get_env("MOCK_IS_GIT_REPO").is_some()
}

/// Get Git user information from environment variables or Git config
#[cfg(not(test))]
fn get_git_user(context: &Context) -> Option<String> {
    // Priority 1: Check GIT_AUTHOR_NAME environment variable
    if let Some(git_author) = context.get_env("GIT_AUTHOR_NAME") {
        return Some(git_author);
    }

    // Priority 2: Check GIT_COMMITTER_NAME environment variable
    if let Some(git_committer) = context.get_env("GIT_COMMITTER_NAME") {
        return Some(git_committer);
    }

    // Priority 3: Check Git config user.name setting
    use gix::discover;

    if let Ok((repo_path, _)) = discover::upwards(&context.current_dir) {
        if let Ok(repo) = gix::open(repo_path.as_ref()) {
            let config = repo.config_snapshot();
            if let Some(name) = config.string("user.name") {
                if let Ok(name_str) = name.to_str() {
                    return Some(name_str.to_string());
                }
            }
        }
    }

    None
}

/// Get Git user information (test version using mock env vars)
#[cfg(test)]
fn get_git_user(context: &Context) -> Option<String> {
    // Check environment variables first - including MOCK_GIT_USER_NAME for tests
    context
        .get_env("GIT_AUTHOR_NAME")
        .or_else(|| context.get_env("GIT_COMMITTER_NAME"))
        .or_else(|| context.get_env("MOCK_GIT_USER_NAME"))
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

#[cfg(test)]
fn is_root_user() -> bool {
    false
}

#[cfg(all(not(target_os = "windows"), not(test)))]
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
    fn ssh_with_empty_detect_env_vars() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
            // Test output should not change when run by root/non-root user
            .config(toml::toml! {
                [username]
                style_root = ""
                style_user = ""
                detect_env_vars = []
            })
            .collect();

        let expected = Some("astronaut in ");
        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn ssh_with_matching_detect_env_vars() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
            .env("FORCE_USERNAME", "true")
            // Test output should not change when run by root/non-root user
            .config(toml::toml! {
                [username]
                style_root = ""
                style_user = ""
                detect_env_vars = ["FORCE_USERNAME"]
            })
            .collect();

        let expected = Some("astronaut in ");
        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn ssh_with_matching_negated_detect_env_vars() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
            .env("NEGATED", "true")
            // Test output should not change when run by root/non-root user
            .config(toml::toml! {
                [username]
                style_root = ""
                style_user = ""
                detect_env_vars = ["!NEGATED"]
            })
            .collect();

        let expected = None;
        assert_eq!(expected, actual.as_deref());
    }

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

    #[test]
    fn show_always_false() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            // Test output should not change when run by root/non-root user
            .config(toml::toml! {
                [username]
                show_always = false

                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn test_alias() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .config(toml::toml! {
                [username]
                show_always = true
                aliases = { "astronaut" = "skywalker" }

                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("skywalker in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn test_alias_emoji() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "kaas")
            .config(toml::toml! {
                [username]
                show_always = true
                aliases = { "a" = "b", "kaas" = "🧀" }

                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("🧀 in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_disabled_by_default() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("MOCK_IS_GIT_REPO", "true")
            .env("MOCK_GIT_USER_NAME", "git_user")
            .config(toml::toml! {
                [username]
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_enabled_in_git_repo() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("MOCK_IS_GIT_REPO", "true")
            .env("MOCK_GIT_USER_NAME", "git_user")
            .config(toml::toml! {
                [username]
                show_git_user = true
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("git_user in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_enabled_not_in_git_repo() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .config(toml::toml! {
                [username]
                show_git_user = true
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_with_show_always() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("MOCK_IS_GIT_REPO", "true")
            .env("MOCK_GIT_USER_NAME", "git_user")
            .config(toml::toml! {
                [username]
                show_always = true
                show_git_user = true
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("git_user in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_prefers_git_author_name() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("MOCK_IS_GIT_REPO", "true")
            .env("GIT_AUTHOR_NAME", "author_name")
            .env("GIT_COMMITTER_NAME", "committer_name")
            .env("MOCK_GIT_USER_NAME", "config_name")
            .config(toml::toml! {
                [username]
                show_git_user = true
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("author_name in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_falls_back_to_committer_name() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("MOCK_IS_GIT_REPO", "true")
            .env("GIT_COMMITTER_NAME", "committer_name")
            .env("MOCK_GIT_USER_NAME", "config_name")
            .config(toml::toml! {
                [username]
                show_git_user = true
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("committer_name in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_falls_back_to_git_config() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("MOCK_IS_GIT_REPO", "true")
            .env("MOCK_GIT_USER_NAME", "config_name")
            .config(toml::toml! {
                [username]
                show_git_user = true
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("config_name in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_with_alias() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("MOCK_IS_GIT_REPO", "true")
            .env("MOCK_GIT_USER_NAME", "git_user")
            .config(toml::toml! {
                [username]
                show_git_user = true
                aliases = { "git_user" = "👨‍💻" }
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("👨‍💻 in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_no_git_user_found() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("MOCK_IS_GIT_REPO", "true")
            // No git user environment variables or mock config
            .config(toml::toml! {
                [username]
                show_git_user = true
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("astronaut in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_overrides_system_username() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "system_user")
            .env("MOCK_IS_GIT_REPO", "true")
            .env("MOCK_GIT_USER_NAME", "git_developer")
            .config(toml::toml! {
                [username]
                show_git_user = true
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("git_developer in ");

        assert_eq!(expected, actual.as_deref());
    }

    #[test]
    fn show_git_user_with_ssh_connection() {
        let actual = ModuleRenderer::new("username")
            .env(super::USERNAME_ENV_VAR, "astronaut")
            .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
            .env("MOCK_IS_GIT_REPO", "true")
            .env("MOCK_GIT_USER_NAME", "remote_git_user")
            .config(toml::toml! {
                [username]
                show_git_user = true
                style_root = ""
                style_user = ""
            })
            .collect();
        let expected = Some("remote_git_user in ");

        assert_eq!(expected, actual.as_deref());
    }
}
