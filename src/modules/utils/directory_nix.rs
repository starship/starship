use nix::sys::stat::Mode;
use nix::unistd;
use nix::unistd::{Gid, Uid};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

const USER_WRITE_BIT: u32 = Mode::S_IWUSR.bits();
const GROUP_WRITE_BIT: u32 = Mode::S_IWGRP.bits();
const OTHER_WRITE_BIT: u32 = Mode::S_IWOTH.bits();

pub fn is_write_allowed(folder_path: &str) -> Result<bool, &'static str> {
    let meta = fs::metadata(folder_path).map_err(|_| "Unable to stat() directory")?;
    let perms = meta.permissions().mode();

    let euid = Uid::effective();
    if euid.is_root() {
        return Ok(true);
    }
    if meta.uid() == euid.as_raw() {
        Ok(perms & USER_WRITE_BIT != 0)
    } else if (meta.gid() == Gid::effective().as_raw())
        || (get_supplementary_groups().contains(&meta.gid()))
    {
        Ok(perms & GROUP_WRITE_BIT != 0)
    } else {
        Ok(perms & OTHER_WRITE_BIT != 0)
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn get_supplementary_groups() -> Vec<u32> {
    match unistd::getgroups() {
        Err(_) => Vec::new(),
        Ok(v) => v.into_iter().map(|i| i.as_raw()).collect(),
    }
}

#[cfg(all(unix, target_os = "macos"))]
fn get_supplementary_groups() -> Vec<u32> {
    // at the moment nix crate does not provide it for macOS
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_test() {
        assert_eq!(is_write_allowed("/etc"), Ok(false));
        assert_eq!(
            is_write_allowed("/i_dont_exist"),
            Err("Unable to stat() directory")
        );
    }
}
