/// Checks if the current user is a root/administrator user

#[cfg(all(target_os = "windows", not(test)))]
pub fn is_root_user() -> bool {
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
pub fn is_root_user() -> bool {
    false
}

#[cfg(all(not(target_os = "windows"), not(test)))]
pub fn is_root_user() -> bool {
    nix::unistd::geteuid() == nix::unistd::ROOT
}
