use ansi_term::Color;
use std::io;

use crate::common;

#[test]
fn not_in_env() -> io::Result<()> {
    let output = common::render_module("conda")
        .env_clear()
        .env("PATH", env!("PATH"))
        .output()?;

    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();

    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn env_set() -> io::Result<()> {
    let output = common::render_module("conda")
        .env_clear()
        .env("CONDA_DEFAULT_ENV", "astronauts")
        .output()?;

    let expected = format!("via {} ", Color::Green.bold().paint("C astronauts"));
    let actual = String::from_utf8(output.stdout).unwrap();

    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn truncate() -> io::Result<()> {
    let output = common::render_module("conda").env_clear().env("CONDA_DEFAULT_ENV", "/some/really/long/and/really/annoying/path/that/shouldnt/be/displayed/fully/conda/my_env").output()?;

    let expected = format!("via {} ", Color::Green.bold().paint("C my_env"));
    let actual = String::from_utf8(output.stdout).unwrap();

    assert_eq!(expected, actual);
    Ok(())
}
