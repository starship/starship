use super::Segment;
use dirs;
use std::env;
use ansi_term::{Color, Style};
use clap::ArgMatches;

/// Creates a segment with the current directory
pub fn segment(args: &ArgMatches) -> Segment {
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

// fn truncate_dir(directory: PathBuf, truncate_to: u8) {

// }

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{App, Arg};

    #[test]
    fn truncate_home_dir() {
        let args = App::new("starship")
            .arg(Arg::with_name("status_code"))
            .get_matches_from(vec!["starship", "0"]);

        env::set_current_dir("~/dev/");
        let segment = segment(&args);
        assert_eq!(segment.style, Style::from(Color::Green));
    }
}
