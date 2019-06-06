use std::{io, process};

pub fn render_prompt() -> process::Command {
    let mut command = process::Command::new("./target/debug/starship");
    command.arg("prompt");

    command
}

pub fn render_module(module_name: &str) -> process::Command {
    let mut command = process::Command::new("./target/debug/starship");
    command.arg("module").arg(module_name);

    command
}

/// Create a temporary directory with full access permissions (rwxrwxrwt).
pub fn new_tempdir() -> io::Result<tempfile::TempDir> {
    //  Using `tempfile::TempDir` directly creates files on macOS within
    // "/var/folders", which provides us with restricted permissions (rwxr-xr-x)
    tempfile::tempdir_in("/tmp")
}
