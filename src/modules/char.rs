use crate::Segment;
use ansi_term::{Color, Style};
use clap::ArgMatches;

pub fn segment(args: &ArgMatches) -> Segment {
    const PROMPT_CHAR: &str = "➜ ";
    const COLOR_SUCCESS: Color = Color::Green;
    const COLOR_FAILURE: Color = Color::Red;

    let color;
    if args.value_of("status_code").unwrap() == "0" {
        color = COLOR_SUCCESS;
    } else {
        color = COLOR_FAILURE;
    }

    Segment {
        prefix: None,
        value: String::from(PROMPT_CHAR),
        style: Style::from(color),
        suffix: None,
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
