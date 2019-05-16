use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a segment with the current Go version
///
/// Will display the Go version if any of the following criteria are met:
///     - Current directory contains a `go.mod` file
///     - Current directory contains a `go.sum` file
///     - Current directory contains a `glide.yaml` file
///     - Current directory contains a `Gopkg.yml` file
///     - Current directory contains a `Gopkg.lock` file
///     - Current directory contains a `.go` file
///     - Current directory contains a `Godeps` directory
pub fn segment(context: &Context) -> Option<Module> {
    let is_go_project = context
        .new_scan_dir()
        .set_files(&["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock"])
        .set_extensions(&["go"])
        .set_folders(&["Godeps"])
        .scan();

    if !is_go_project {
        return None;
    }

    match get_go_version() {
        Some(go_version) => {
            const GO_CHAR: &str = "🐹 ";
            let module_color = Color::Cyan.bold();

            let mut module = Module::new("go");
            module.set_style(module_color);

            let formatted_version = format_go_version(go_version)?;
            module.new_segment("symbol", GO_CHAR);
            module.new_segment("version", formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_go_version() -> Option<String> {
    Command::new("go")
        .arg("version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
}

fn format_go_version(go_stdout: String) -> Option<String> {
    let version = go_stdout
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
        assert_eq!(format_go_version(input), Some("v1.12".to_string()));
    }
}
