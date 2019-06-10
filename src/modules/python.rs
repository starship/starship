use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a segment with the current Python version
///
/// Will display the Python version if any of the following criteria are met:
///     - Current directory contains a `.py` file
///     - Current directory contains a `.python-version` file
///     - Current directory contains a `requirements.txt` file
///     - Current directory contains a `pyproject.toml` file
pub fn segment<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_py_project = context
        .new_scan_dir()
        .set_files(&["requirements.txt", ".python-version", "pyproject.toml"])
        .set_extensions(&["py"])
        .scan();

    if !is_py_project {
        return None;
    }

    match get_python_version() {
        Some(python_version) => {
            const PYTHON_CHAR: &str = "🐍 ";
            let module_color = Color::Yellow.bold();

            let mut module = context.new_module("python");
            module.set_style(module_color);

            let formatted_version = format_python_version(python_version);
            module.new_segment("symbol", PYTHON_CHAR);
            module.new_segment("version", formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_python_version() -> Option<String> {
    match Command::new("python").arg("--version").output() {
        Ok(output) => {
            // We have to check both stdout and stderr since for Python versions
            // < 3.4, Python reports to stderr and for Python version >= 3.5,
            // Python reports to stdout
            if output.stdout.is_empty() {
                let stderr_string = String::from_utf8(output.stderr).unwrap();
                Some(stderr_string)
            } else {
                let stdout_string = String::from_utf8(output.stdout).unwrap();
                Some(stdout_string)
            }
        }
        Err(_) => None,
    }
}

fn format_python_version(python_stdout: String) -> String {
    format!("v{}", python_stdout.trim_start_matches("Python ").trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_python_version() {
        let input = String::from("Python 3.7.2");
        assert_eq!(format_python_version(input), "v3.7.2");
    }
}
