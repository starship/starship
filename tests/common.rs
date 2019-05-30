use clap::{App, Arg};
use starship::context::Context;
use starship::modules;
use std::io;
use std::path::PathBuf;

#[allow(dead_code)]
pub fn render_module<T>(module: &str, path: T) -> String
where
    T: Into<PathBuf>,
{
    render_module_with_status(module, path.into(), "0")
}

pub fn render_module_with_status<T>(module: &str, path: T, status: &str) -> String
where
    T: Into<PathBuf>,
{
    // Create an `Arg` with status_code of "0"
    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", status]);
    let context = Context::new_with_dir(args, path.into());

    let module = modules::handle(module, &context);

    module.unwrap().to_string()
}

/// Create a temporary directory with full access permissions (rwxrwxrwt).
pub fn new_tempdir() -> io::Result<tempfile::TempDir> {
    //  Using `tempfile::TempDir` directly creates files on macOS within
    // "/var/folders", which provides us with restricted permissions (rwxr-xr-x)
    tempfile::tempdir_in("/tmp")
}
