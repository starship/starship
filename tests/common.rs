use clap::{App, Arg};
use starship::modules;
use git2::Repository;
use std::path::Path;
use std::fs;
use std::io;

pub fn create_git_repo(repo_dir: &Path) -> io::Result<(&Path)> {
    fs::create_dir(&repo_dir)?;

    match Repository::init(&repo_dir) {
        Ok(repo) => repo.path(),
        Err(e) => panic!("failed to init: {}", e),
    }
}

pub fn render_segment(module: &str, path: &Path) -> String {
    // Create an `Arg` with status_code of "0"
    let args = App::new("starship")
        .arg(Arg::with_name("status_code"))
        .get_matches_from(vec!["starship", "0"]);

    let segment = modules::handle(module, path, &args);

    segment.unwrap().output()
}
