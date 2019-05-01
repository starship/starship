use ansi_term::Color;
use std::path::PathBuf;
use std::process::Command;

use super::{Context, Module};

/// Creates a segment with the current Python version
///
/// Will display the Python version if any of the following criteria are met:
///     - Current directory contains a `.py` file
///     - Current directory contains a `.python-version` file
///     - Current directory contains a `requirements.txt` file
///     - Current directory contains a `pyproject.toml` file
pub fn segment(context: &Context) -> Option<Module> {
    let is_py_project = context.dir_files.iter().any(has_py_files);
    if !is_py_project {
        return None;
    }

    match get_python_version() {
        Some(python_version) => {
            const PYTHON_CHAR: &str = "🐍";
            let module_color = Color::Yellow.bold();

            let mut module = Module::new("python");
            module.set_style(module_color);

            let formatted_version = format_python_version(python_version);
            module.new_segment("symbol", PYTHON_CHAR);
            module.new_segment("version", formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn has_py_files(dir_entry: &PathBuf) -> bool {
    let is_py_file =
        |d: &PathBuf| -> bool { d.is_file() && d.extension().unwrap_or_default() == "py" };
    let is_python_version = |d: &PathBuf| -> bool {
        d.is_file() && d.file_name().unwrap_or_default() == ".python-version"
    };
    let is_requirements_txt = |d: &PathBuf| -> bool {
        d.is_file() && d.file_name().unwrap_or_default() == "requirements.txt"
    };
    let is_py_project = |d: &PathBuf| -> bool {
        d.is_file() && d.file_name().unwrap_or_default() == "pyproject.toml"
    };

    is_py_file(&dir_entry)
        || is_python_version(&dir_entry)
        || is_requirements_txt(&dir_entry)
        || is_py_project(&dir_entry)
}

fn get_python_version() -> Option<String> {
    match Command::new("python").arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
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
