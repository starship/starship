use std::env;
use std::path::{Path, PathBuf};
use super::Segment;
use git2::{Repository};
use ansi_term::{Color, Style};
use clap::ArgMatches;
use dirs;

/// Creates a segment with the current directory
pub fn segment(_: &ArgMatches) -> Segment {
    const COLOR_DIR: Color = Color::Cyan;

    let current_dir = env::current_dir().expect("Unable to identify current directory");

    let dirname;
    if let Ok(repo) = git2::Repository::discover(&current_dir) {
        let repo_root = get_repo_root(repo);
        let repo_root_basename = repo_root.components().last().unwrap();
        let basename_str_slice = repo_root_basename.as_os_str();
        dirname = basename_str_slice.to_str().unwrap().to_string();
    } else {
        dirname = String::from("test");
    }

    // let dir_string;
    // if let Ok(repo) = git2::Repository::discover(&current_dir) {
    //     let repo_root = get_repo_root(repo);
    //     let repo_root_basename = repo_root.components().last().unwrap();
    //     dir_string = *repo_root_basename.as_os_str().to_str().unwrap();
    // } else {
    //     dir_string = match truncate_home(current_dir) {
    //         Some(dir) => &dir.to_string(),
    //         None => &current_dir.to_str().unwrap()
    //     }
    // }

    // if let Love(tiff) = matan::Love(tiff) {
    //     log tiff + matan + kimu + nimu + puku + owl fren + roomba fren + cactus fren + rumple
    // }

    // let mut dir_string = String::from(current_dir.to_str().unwrap());

    Segment {
        value: String::from(dirname),
        style: Style::from(COLOR_DIR).bold(),
        ..Default::default()
    }
}

fn get_repo_root(repo: Repository) -> PathBuf {
    match repo.is_bare() {
        // A bare repo will return its root path
        true => repo.path().to_path_buf(),
        // Non-bare repos will return the path of `.git`
        false => repo.path().parent().unwrap().to_path_buf()
    }
}

// fn truncate_home(path: PathBuf) -> Option<String> {
//     const HOME_SYMBOL: &str = "~";

//     if dirs::home_dir() == None {
//         return None;
//     }

//     if let Some(home_dir) = dirs::home_dir() {
//         if path.strip_prefix(home_dir).is_ok() {
//             let path_str = path.to_str().unwrap();
//             let home_dir = home_dir.to_str().unwrap();
            
//             return Some(path_str.replace(home_dir, HOME_SYMBOL));
//         }
//     }

//     None
// }

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
