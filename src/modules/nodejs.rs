use super::Segment;
use ansi_term::{Color, Style};
use clap::ArgMatches;
use std::env;
use std::fs::{self, DirEntry};
use std::process::Command;

/// Creates a segment with the current Node.js version
pub fn segment(_: &ArgMatches) -> Segment {
    const NODE_CHAR: &str = "⬢ ";
    const SECTION_COLOR: Color = Color::Green;

    let current_path = env::current_dir().expect("Unable to identify current directory");
    let files = fs::read_dir(&current_path).unwrap();

    let is_js_project = files.filter_map(Result::ok).any(has_js_files);

    if is_js_project {
        return Segment::default();
    }

    let version = match Command::new("node").arg("--version").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap().trim().to_string(),
        Err(_) => {
            return Segment::default();
        }
    };

    Segment {
        value: format!("{}{}", NODE_CHAR, version),
        style: Style::from(SECTION_COLOR),
        ..Default::default()
    }
}

fn has_js_files(dir_entry: DirEntry) -> bool {
    let is_js_file =
        |d: &DirEntry| d.path().is_file() && d.path().extension().unwrap_or_default() == "js";
    let is_node_modules = |d: &DirEntry| {
        d.path().is_dir() && d.path().file_name().unwrap_or_default() == "node_modules"
    };
    let is_package_json = |d: &DirEntry| {
        d.path().is_file() && d.path().file_name().unwrap_or_default() == "package.json"
    };

    is_js_file(&dir_entry) || is_node_modules(&dir_entry) || is_package_json(&dir_entry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{App, Arg};

    #[test]
    fn char_section_success_status() {
        let args = App::new("starship")
            .arg(Arg::with_name("status_code"))
            .get_matches_from(vec!["starship", "0"]);

        let segment = segment(&args);
        assert_eq!(segment.style, Style::from(Color::Green));
    }

    #[test]
    fn char_section_failure_status() {
        let args = App::new("starship")
            .arg(Arg::with_name("status_code"))
            .get_matches_from(vec!["starship", "1"]);

        let segment = segment(&args);
        assert_eq!(segment.style, Style::from(Color::Red));
    }
}
