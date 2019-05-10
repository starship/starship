use ansi_term::Color;
use std::path::PathBuf;
use std::process::Command;

use super::{Context, Module};

/// Creates a segment with the current Python version
///
/// Will display the Python version if any of the following criteria are met:
///     - Current directory contains a `.go` file
///     - Current directory contains a `go.mod` file
///     - Current directory contains a `go.sum` file
///     - Current directory contains a `Godeps` directory
///     - Current directory contains a `glide.yaml` file
///     - Current directory contains a `Gopkg.yml` file
///     - Current directory contains a `Gopkg.lock` file
pub fn segment(context: &Context) -> Option<Module> {
    let is_go_project = context.dir_files.iter().any(has_go_files);
    if !is_go_project {
        return None;
    }

    const GO_CHAR: &str = "🐹 ";
    let module_color = Color::Cyan.bold();

    let mut module = Module::new("go");
    module.set_style(module_color);

    let go_version = get_go_version()?;
    let formatted_version = format_go_version(go_version)?;
    module.new_segment("symbol", GO_CHAR);
    module.new_segment("version", formatted_version);

    Some(module)
}

fn has_go_files(dir_entry: &PathBuf) -> bool {
    let is_go_mod =
        |d: &PathBuf| -> bool { d.is_file() && d.file_name().unwrap_or_default() == "go.mod" };
    let is_go_sum =
        |d: &PathBuf| -> bool { d.is_file() && d.file_name().unwrap_or_default() == "go.sum" };
    let is_godeps =
        |d: &PathBuf| -> bool { d.is_dir() && d.file_name().unwrap_or_default() == "Godeps" };
    let is_glide_yaml =
        |d: &PathBuf| -> bool { d.is_file() && d.file_name().unwrap_or_default() == "glide.yaml" };
    let is_go_file =
        |d: &PathBuf| -> bool { d.is_file() && d.extension().unwrap_or_default() == "go" };
    let is_gopkg_yml =
        |d: &PathBuf| -> bool { d.is_file() && d.file_name().unwrap_or_default() == "Gopkg.yml" };
    let is_gopkg_lock =
        |d: &PathBuf| -> bool { d.is_file() && d.file_name().unwrap_or_default() == "Gopkg.lock" };

    is_go_mod(&dir_entry)
        || is_go_sum(&dir_entry)
        || is_godeps(&dir_entry)
        || is_glide_yaml(&dir_entry)
        || is_go_file(&dir_entry)
        || is_gopkg_yml(&dir_entry)
        || is_gopkg_lock(&dir_entry)
}

fn get_go_version() -> Option<String> {
    Command::new("go")
        .arg("version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
}

fn format_go_version(go_stdout: String) -> Option<String> {
    let mut version = go_stdout
        // split into ["", "1.12.4 linux/amd64"]
        .splitn(2, "go version go")
        // return "1.12.4 linux/amd64"
        .nth(1)?
        // split into ["1.12.4", "linux/amd64"]
        .split_whitespace()
        // return "1.12.4"
        .next()?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_go_version() {
        let input = String::from("go version go1.12 darwin/amd64");
        assert_eq!(format_go_version(input), "v1.12");
    }
}
