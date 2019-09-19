use std::fs::File;
use std::io;

use ansi_term::Color;

use crate::common;

#[test]
#[ignore]
fn folder_with_pom() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("pom.xml"))?;

    let output = common::render_module("java")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.dimmed().paint("☕ v12.0.2"));
    assert_eq!(expected, actual);
    Ok(())
}


#[test]
fn check_java_version1() -> io::Result<()> {
    let out = std::process::Command::new("java").arg("-Xinternalversion").output().unwrap().stdout;
    let ver = String::from_utf8(out).unwrap();
    assert_eq!(ver, "nope");
    Ok(())
}

#[test]
fn check_java_version2() -> io::Result<()> {
    let out = std::process::Command::new("/opt/hostedtoolcache/Java/12.0.2/x64/bin/java").arg("-Xinternalversion").output().unwrap().stdout;
    let ver = String::from_utf8(out).unwrap();
    assert_eq!(ver, "nope");
    Ok(())
}
