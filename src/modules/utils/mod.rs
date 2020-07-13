pub mod directory;
pub mod java_version_parser;

#[cfg(target_os = "windows")]
pub mod directory_win;

#[cfg(not(target_os = "windows"))]
pub mod directory_nix;
