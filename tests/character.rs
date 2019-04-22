use ansi_term::Color;
use starship::segment::Segment;
use std::path::Path;

mod common;

#[test]
fn char_segment_success_status() {
    let dir = Path::new("~");
    let expected = Segment::new("char")
        .set_value("➜")
        .set_style(Color::Green)
        .set_prefix(None)
        .output();
    let actual = common::render_segment_with_status("char", &dir, "0");
    assert_eq!(expected, actual);
}

#[test]
fn char_segment_failure_status() {
    let dir = Path::new("~");
    let expected = Segment::new("char")
        .set_value("➜")
        .set_style(Color::Red)
        .set_prefix(None)
        .output();
    let actual = common::render_segment_with_status("char", &dir, "1");
    assert_eq!(expected, actual);
}
