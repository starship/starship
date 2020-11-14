pub mod directory;
pub mod meminfo;

#[cfg(target_os = "windows")]
pub mod directory_win;

#[cfg(not(target_os = "windows"))]
pub mod directory_nix;
