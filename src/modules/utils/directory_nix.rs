use nix::errno::Errno::ENOENT;
use nix::unistd::access;
use nix::unistd::AccessFlags;
use std::path::Path;

/// Checks if the current user can write to the `folder_path`.
pub fn is_write_allowed(folder_path: &Path) -> Result<bool, String> {
    match access(folder_path, AccessFlags::W_OK) {
        Ok(()) => Ok(true),
        Err(ENOENT) => Err(format!("Unable to stat() directory: {}", ENOENT)),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn read_only_test() {
        assert_eq!(is_write_allowed(Path::new("/etc")), Ok(false));
        assert!(match is_write_allowed(Path::new("/i_dont_exist")) {
            Ok(_) => false,
            Err(e) => e.starts_with("Unable to stat() directory"),
        });
    }
}
