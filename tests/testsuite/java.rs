use std::{fs::File, io};

use ansi_term::Color;

use crate::common;

// When these tests run in the CI, they use Azul's Zulu Java distribution.
// The version format returned by this java package is irregular and not
// supported (yet).

//#[test]
//#[ignore]
// fn folder_with_pom() -> io::Result<()> {
//    let dir = common::new_tempdir()?;
//    File::create(dir.path().join("pom.xml"))?;
//
//    let output = common::render_module("java")
//        .arg("--path")
//        .arg(dir.path())
//        .output()?;
//    let actual = String::from_utf8(output.stdout).unwrap();
//
//    let expected = format!("via {} ", Color::Red.dimmed().paint("☕ "));
//    assert_eq!(expected, actual);
//    Ok(())
//}
