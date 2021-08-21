pub mod directory;

#[cfg(target_os = "windows")]
pub mod directory_win;

#[cfg(not(target_os = "windows"))]
pub mod directory_nix;

#[cfg(target_os = "windows")]
pub mod version_win;

pub mod path;
