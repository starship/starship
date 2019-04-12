use super::Segment;
use ansi_term::{Color, Style};
use clap::ArgMatches;

/// Creates a segment for the prompt character
///
/// The char segment prints an arrow character in a color dependant on the exit-
/// code of the last executed command:
/// - If the exit-code was "0", the arrow will be formatted with `COLOR_SUCCESS`
/// (green by default)
/// - If the exit-code was anything else, the arrow will be formatted with
/// `COLOR_FAILURE` (red by default)
pub fn segment(args: &ArgMatches) -> Segment {
    const PROMPT_CHAR: &str = "➜";
    const COLOR_SUCCESS: Color = Color::Green;
    const COLOR_FAILURE: Color = Color::Red;

    let segment = Segment::new("char");

    if args.value_of("status_code").unwrap() == "0" {
        segment.set_style(COLOR_SUCCESS);
    } else {
        segment.set_style(COLOR_FAILURE);
    };

    segment
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
