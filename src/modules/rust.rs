use super::Segment;
use ansi_term::Color;
use std::fs::{self, DirEntry};
use crate::context::Context;
use std::process::Command;

/// Creates a segment with the current Rust version
///
/// Will display the Rust version if any of the following criteria are met:
///     - Current directory contains a `.rs` or 'Cargo.toml' file
pub fn segment(context: &Context) -> Option<Segment> {
    let files = fs::read_dir(&context.current_dir).unwrap();
    let is_rs_project = files.filter_map(Result::ok).any(has_rs_files);
    if !is_rs_project {
        return None;
    }

    match get_rust_version() {
        Some(rust_version) => {
            const RUST_LOGO: &str = "🦀";
            const SECTION_COLOR: Color = Color::Red;

            let mut segment = Segment::new("rust");
            segment.set_style(SECTION_COLOR);

            let formated_version = format_rustc_version(rust_version);
            segment.set_value(format!("{} {}", RUST_LOGO, formated_version));

            return Some(segment)
        },
        None => return None
    };
}

fn has_rs_files(dir_entry: DirEntry) -> bool {
    let is_rs_file = |d: &DirEntry| -> bool {
        d.path().is_file() && d.path().extension().unwrap_or_default() == "rs"
    };
    let is_cargo_toml = |d: &DirEntry| -> bool {
        d.path().is_file() && d.path().file_name().unwrap_or_default() == "Cargo.toml"
    };

    is_rs_file(&dir_entry) || is_cargo_toml(&dir_entry)
}

fn get_rust_version() -> Option<String> {
    match Command::new("rustc").arg("-V").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None
    }
}

fn format_rustc_version(mut rustc_stdout: String) -> String {
   let offset = &rustc_stdout.find('(').unwrap();
   let formated_version: String = rustc_stdout.drain(..offset).collect();

   format!(" v{}", formated_version.replace("rustc", "").trim())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_rustc_version() {
        let nightly_input = String::from("rustc 1.34.0-nightly (b139669f3 2019-04-10)");
        assert_eq!(format_rustc_version(nightly_input), " v1.34.0-nightly");

        let beta_input = String::from("rustc 1.34.0-beta.1 (2bc1d406d 2019-04-10)");
        assert_eq!(format_rustc_version(beta_input), " v1.34.0-beta.1");

        let stable_input = String::from("rustc 1.34.0 (91856ed52 2019-04-10)");
        assert_eq!(format_rustc_version(stable_input), " v1.34.0");
    }
}