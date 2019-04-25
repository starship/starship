use ansi_term::Color;
use starship::segment::Segment;
use std::fs::File;
use std::io;
use tempfile::TempDir;

mod common;

#[test]
#[ignore]
fn folder_with_package_json() -> io::Result<()> {
    let project_dir = TempDir::new()?;
    File::create(project_dir.path().join("package.json"))?;

    let expected = Segment::new("node")
        .set_value("⬢ v10.2.0")
        .set_style(Color::Green)
        .output();
    let actual = common::render_segment("node", &project_dir);
    assert_eq!(expected, actual);

    Ok(())
}
