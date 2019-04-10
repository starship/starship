use super::Segment;
use std::process::Command;
use ansi_term::{Color, Style};
use clap::ArgMatches;

/// Creates a segment with the current Node.js version
pub fn segment(args: &ArgMatches) -> Segment {
    const NODE_CHAR: &str = "⬢ ";
    const SECTION_COLOR: Color = Color::Green;

    let version = match Command::new("node").arg("--version").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(e) => {
            println!("{:?}", e);
            return Segment::default();
        }
    };

    Segment {
        value: format!("{}{}", NODE_CHAR, version),
        style: Style::from(SECTION_COLOR),
        ..Default::default()
    }
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
