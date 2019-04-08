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
    const DIR_TRUNCATION_LENGTH: u8 = 3;

    let current_dir = env::current_dir()
        .expect("Unable to identify current directory")
        .canonicalize()
        .expect("Unable to canonicalize current directory");

    let dir_string;
    if let Ok(repo) = git2::Repository::discover(&current_dir) {
        let repo_root = get_repo_root(repo);
        let repo_root_depth = repo_root.components().count();

        // Skip the path components that are before the repo root
        let path = current_dir
            .iter()
            .skip(repo_root_depth - 1)
            .collect::<PathBuf>();

        dir_string = path.to_str().unwrap().to_string();
    } else {
        dir_string = match truncate_home(&current_dir) {
            Some(dir) => dir,
            None => current_dir.to_str().unwrap().to_string(),
        }
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

/// Replace the home directory in the path with "~"
fn truncate_home(path: &PathBuf) -> Option<String> {
    const HOME_SYMBOL: &str = "~";

    if dirs::home_dir() == None {
        return None;
    }

    if let Some(home_dir) = dirs::home_dir() {
        if path.strip_prefix(&home_dir).is_ok() {
            let path_str = path.to_str().unwrap();
            let home_dir = home_dir.to_str().unwrap();

            return Some(path_str.replace(home_dir, HOME_SYMBOL));
        }
    }

    None
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
