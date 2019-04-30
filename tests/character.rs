use ansi_term::Color;
use std::path::Path;

mod common;

#[test]
fn char_module_success_status() {
    let dir = Path::new("~");
    let expected = format!("{} ", Color::Green.bold().paint("➜"));
    let actual = common::render_module_with_status("char", &dir, "0");
    assert_eq!(expected, actual);
}

#[test]
fn char_module_failure_status() {
    let dir = Path::new("~");
    let expected = format!("{} ", Color::Red.bold().paint("➜"));
    let actual = common::render_module_with_status("char", &dir, "1");
    assert_eq!(expected, actual);
}
