use super::Segment;
use ansi_term::{Color, Style};
use clap::ArgMatches;
use dirs;
use std::env;

/// Creates a segment with the current directory
pub fn segment(_: &ArgMatches) -> Segment {
    const COLOR_DIR: Color = Color::Cyan;
    const HOME_SYMBOL: char = '~';

    let current_dir = env::current_dir().unwrap();
    let mut dir_string = String::from(current_dir.to_str().unwrap());

    if let Some(home_dir) = dirs::home_dir() {
        if current_dir.starts_with(&home_dir) {
            let current_dir = current_dir.to_str().unwrap();
            let home_dir = home_dir.to_str().unwrap();

            dir_string = current_dir.replace(home_dir, &HOME_SYMBOL.to_string());
        }
    }

    Segment {
        value: dir_string,
        style: Style::from(COLOR_DIR).bold(),
        ..Default::default()
    }
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
