use std::env;

use super::{Context, Module, RootModuleConfig, SegmentConfig};
use crate::configs::username::UsernameConfig;

#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, PBOOL, PDWORD, TRUE};
#[cfg(target_os = "windows")]
use winapi::um::errhandlingapi::GetLastError;
#[cfg(target_os = "windows")]
use winapi::um::winnt::PVOID;

/// Creates a module with the current user's username
///
/// Will display the username if any of the following criteria are met:
///     - The current user isn't the same as the one that is logged in (`$LOGNAME` != `$USER`)
///     - The current user is root (UID = 0)
///     - The user is currently connected as an SSH session (`$SSH_CONNECTION`)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("username");
    let config: UsernameConfig = UsernameConfig::try_load(module.config);

    let user = get_username();
    user.as_ref()?;

    let domain = get_userdomain();

    let logname = match env::var("LOGNAME") {
        Ok(logname) => Some(logname),
        Err(_e) => user.clone(),
    };
    let ssh_connection = env::var("SSH_CONNECTION").ok();

    let user_is_admin = is_admin();
    log::debug!("user is admin: {}", user_is_admin);

    if user != logname || ssh_connection.is_some() || user_is_admin || config.show_always {
        let module_style = if user_is_admin {
            config.style_root
        } else {
            config.style_user
        };

        module.set_style(module_style);

        match domain {
            Some(domain_name) => {
                module.create_segment(
                    "username",
                    &SegmentConfig::new(&format!(
                        "{domain}\\{user}",
                        domain = domain_name,
                        user = user?
                    )),
                );
            }
            None => {
                module.create_segment("username", &SegmentConfig::new(&user?));
            }
        }

        Some(module)
    } else {
        None
    }
}

#[cfg(not(target_os = "windows"))]
fn is_admin() -> bool {
    const ROOT_UID: u32 = 0u32;
    unsafe {
        match libc::geteuid() {
            ROOT_UID => true,
            _ => false,
        }
    }
}

#[cfg(target_os = "windows")]
fn is_admin() -> bool {
    use winapi::um::securitybaseapi::{CheckTokenMembership, CreateWellKnownSid};
    use winapi::um::winnt::{WinBuiltinAdministratorsSid, PSID, SECURITY_MAX_SID_SIZE};

    let mut is_member: BOOL = FALSE;

    unsafe {
        let mut admin_sid = vec![0u8; SECURITY_MAX_SID_SIZE];
        let admin_sid_ptr = admin_sid.as_mut_ptr() as PVOID;
        let mut sid_size = (std::mem::size_of::<u8>() * SECURITY_MAX_SID_SIZE) as DWORD;

        if CreateWellKnownSid(
            WinBuiltinAdministratorsSid,
            null_mut(),
            admin_sid_ptr,
            &mut sid_size,
        ) == 0i32
        {
            log::debug!(
                "Error getting wellknown administators sid, GetLastError() : {}",
                GetLastError()
            );
            return false;
        }

        if CheckTokenMembership(
            std::ptr::null_mut(),
            admin_sid.as_mut_ptr() as PSID,
            &mut is_member as PBOOL,
        ) == 0i32
        {
            log::debug!(
                "Error checking token's membership to admins group, GetLastError() : {}",
                GetLastError()
            );
            return false;
        }
    }
    is_member == TRUE
}

/// Get the user name on Windows systems
///
/// On non-Windows OS, does nothing
#[cfg(target_os = "windows")]
fn get_userdomain() -> Option<String> {
    match std::env::var("USERDOMAIN") {
        Ok(name) => {
            match std::env::var("COMPUTERNAME") {
                Ok(computer) => {
                    match computer {
                        _ if computer == name => None, // if computer name & domain name are identical, then the user is a local account
                        _ => Some(name),
                    }
                }
                Err(e) => {
                    log::debug!("COMPUTERNAME is undefined: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            log::debug!("USERDOMAIN is undefined: {}", e);
            None
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn get_userdomain() -> Option<String> {
    None
}

#[cfg(target_os = "windows")]
fn get_username() -> Option<String> {
    match std::env::var("USERNAME") {
        Ok(name) => Some(name),
        Err(e) => {
            log::debug!("USERNAME is undefined: {}", e);
            unsafe {
                use winapi::um::winbase::GetUserNameW;

                const BUFFER_SIZE: usize = 256 as usize;
                let mut output_string = vec![0u16; BUFFER_SIZE + 1];
                let mut length: DWORD = BUFFER_SIZE as u32;

                match GetUserNameW(output_string.as_mut_ptr(), &mut length as PDWORD) {
                    0i32 => {
                        log::debug!("Error getting GetUserNameW");
                        None
                    }
                    _ if length == 0 => {
                        log::debug!("Invalid username size returned from GetUserNameW");
                        None
                    }
                    _ => {
                        output_string.truncate((length - 1) as usize);
                        Some(String::from_utf16(&output_string).unwrap())
                    }
                }
            }
        }
    }
}

/// Get the user name on non-Windows OS
///
#[cfg(not(target_os = "windows"))]
fn get_username() -> Option<String> {
    match env::var("USER") {
        Ok(username) => Some(username),
        Err(ref e) => {
            log::debug!("Error getting USER environment variable: {}", e);
            None
        }
    }
}
