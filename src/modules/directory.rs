use super::Segment;
use ansi_term::{Color, Style};
use clap::ArgMatches;
use dirs;
use git2::Repository;
use std::env;
use std::path::PathBuf;

/// Creates a segment with the current directory
pub fn segment(_: &ArgMatches) -> Segment {
    const COLOR_DIR: Color = Color::Cyan;
    const DIR_TRUNCATION_LENGTH: usize = 3;
    const HOME_SYMBOL: &str = "~";

    let current_path = env::current_dir()
        .expect("Unable to identify current directory")
        .canonicalize()
        .expect("Unable to canonicalize current directory");

    let dir_string;
    if let Ok(repo) = git2::Repository::discover(&current_path) {
        let repo_root = get_repo_root(repo);

        // The folder name is the last component to the repo path
        let repo_folder_name = repo_root
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();

        dir_string = truncate_path(
            &DIR_TRUNCATION_LENGTH,
            &current_path,
            &repo_root,
            &repo_folder_name,
        );
    } else {
        let home_dir = dirs::home_dir().unwrap();

        dir_string = truncate_path(
            &DIR_TRUNCATION_LENGTH,
            &current_path,
            &home_dir,
            HOME_SYMBOL,
        );
    }

    Segment {
        value: dir_string,
        style: Style::from(COLOR_DIR).bold(),
        ..Default::default()
    }
}

/// Get the root directory of a git repo
fn get_repo_root(repo: Repository) -> PathBuf {
    if repo.is_bare() {
        // Bare repos will return the repo root
        repo.path().to_path_buf()
    } else {
        // Non-bare repos will return the path of `.git`
        repo.path().parent().unwrap().to_path_buf()
    }
}

/// Truncate a path to a predefined number of path components
fn truncate_path(
    length: &usize,
    full_path: &PathBuf,
    top_level_path: &PathBuf,
    top_level_replacement: &str,
) -> String {
    if full_path == top_level_path {
        return top_level_replacement.to_string();
    }

    let full_path_depth = full_path.components().count();
    let top_level_path_depth = top_level_path.components().count();

    // Don't bother with replacing top level path if length is long enough
    if full_path_depth - top_level_path_depth >= *length {
        return full_path
            .iter()
            .skip(full_path_depth - length)
            .collect::<PathBuf>()
            .to_str()
            .unwrap()
            .to_string();
    }

    format!(
        "{}{}{}",
        top_level_replacement,
        std::path::MAIN_SEPARATOR,
        full_path
            .iter()
            .skip(top_level_path_depth)
            .collect::<PathBuf>()
            .to_str()
            .unwrap()
    )
}

#[cfg(test)]
mod tests {
    // TODO: Look into stubbing `env` so that tests can be run in parallel
    use super::*;
    use clap::{App, Arg};
    use std::path::Path;

    #[test]
    fn truncate_home_dir() {
        let args = App::new("starship")
            .arg(Arg::with_name("status_code"))
            .get_matches_from(vec!["starship", "0"]);

        let home_dir = dirs::home_dir().unwrap();
        env::set_current_dir(&home_dir).unwrap();

        let segment = segment(&args);
        assert_eq!(segment.value, "~");
    }

    #[test]
    fn dont_truncate_non_home_dir() {
        let args = App::new("starship")
            .arg(Arg::with_name("status_code"))
            .get_matches_from(vec!["starship", "0"]);

        let root_dir = Path::new("/");
        env::set_current_dir(&root_dir).unwrap();

        let segment = segment(&args);
        assert_eq!(segment.value, "/");
    }
}
