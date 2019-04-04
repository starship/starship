use crate::Segment;
use ansi_term::{Color, Style};
use std::env;

pub fn segment() -> Segment {
    const PROMPT_CHAR: &str = "➜ ";
    const COLOR_SUCCESS: Color = Color::Green;
    const COLOR_FAILURE: Color = Color::Red;

    let default_prefix = Segment {
        value: String::from("testPrefix"),
        style: Style::default(),
        prefix: None,
        suffix: None,
    };

    let color;
    if let Ok(status) = env::var("status") {
        if status == "0" {
            color = COLOR_SUCCESS;
        } else {
            color = COLOR_FAILURE;
        }
    } else {
        panic!("No status environment variable provided");
    }

    Segment {
        prefix: Some(Box::new(default_prefix)),
        value: String::from(PROMPT_CHAR),
        style: Style::new().fg(color),
        suffix: None,
    }
}
