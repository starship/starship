use ansi_term::Color;
use std::io;

mod common;

#[test]
#[ignore]
fn full_prompt_home() -> io::Result<()> {
    let expected = format!(
        "\n{dir} \n{char} ",
        dir = Color::Cyan.bold().paint("~"),
        char = Color::Green.bold().paint("➜")
    );

    let output = common::render_prompt("~")?;
    let actual = String::from_utf8(output.stdout).unwrap();

    assert_eq!(expected, actual);
    Ok(())
}
