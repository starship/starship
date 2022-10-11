pub mod directory;

#[cfg(target_os = "windows")]
pub mod directory_win;

#[cfg(not(target_os = "windows"))]
pub mod directory_nix;

pub mod path;

pub mod truncate;
