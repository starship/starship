use super::Segment;
use crate::context::Context;
use ansi_term::Color;
use std::path::PathBuf;
use std::process::Command;

/// Creates a segment with the current package version
pub fn segment(context: &Context) -> Option<Segment> {
    match get_package_version(context) {
        Some(package_version) => {
            const RUST_CHAR: &str = "📦";
            const SEGMENT_COLOR: Color = Color::Red;

            let mut segment = Segment::new("package");
            // TODO: Should have prefix "is "
            segment.set_style(SEGMENT_COLOR);

            segment.set_value(format!("{} {}", RUST_CHAR, package_version));

            Some(segment)
        }
        None => None,
    }
}

fn has_rs_files(dir_entry: &PathBuf) -> bool {
    let is_rs_file =
        |d: &PathBuf| -> bool { d.is_file() && d.extension().unwrap_or_default() == "rs" };
    let is_cargo_toml =
        |d: &PathBuf| -> bool { d.is_file() && d.file_name().unwrap_or_default() == "Cargo.toml" };

    is_rs_file(&dir_entry) || is_cargo_toml(&dir_entry)
}

fn has_js_files(dir_entry: &PathBuf) -> bool {
    let is_js_file =
        |d: &PathBuf| -> bool { d.is_file() && d.extension().unwrap_or_default() == "js" };
    let is_node_modules =
        |d: &PathBuf| -> bool { d.is_dir() && d.file_name().unwrap_or_default() == "node_modules" };
    let is_package_json = |d: &PathBuf| -> bool {
        d.is_file() && d.file_name().unwrap_or_default() == "package.json"
    };

    is_js_file(&dir_entry) || is_node_modules(&dir_entry) || is_package_json(&dir_entry)
}

fn get_package_version(context: &Context) -> Option<String> {
    let is_rs_project = context.dir_files.iter().any(has_rs_files);
    if !is_rs_project {
        let is_js_project = context.dir_files.iter().any(has_js_files);
        // TODO: Implement for nodejs
    } else {
        match Command::new("cargo").arg("pkgid").output() {
            Ok(output) => Some(format_rust_version(
                String::from_utf8(output.stdout).unwrap(),
            )),
            Err(_) => None,
        }
    }
}

fn format_rust_version(mut rust_version: String) -> String {
    let offset = &rust_version.find('#').unwrap();
    let _text: String = rust_version.drain(..offset).collect();

    format!("v{}", rust_version.replace("#", "").trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_rust_version() {
        let input = String::from("file:///Users/john/Desktop/starship#0.1.0");
        assert_eq!(format_rust_version(input), "v0.1.0");
    }
}
