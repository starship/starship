use std::io;

mod common;

#[test]
fn line_break_module() -> io::Result<()> {
    let output = common::render_module("line_break").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "\n";
    assert_eq!(expected, actual);
    Ok(())
}
