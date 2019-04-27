use super::Segment;
use crate::context::Context;
use crate::find_file;
use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a segment with the current Python version
///
/// Will display the Python version if any of the following criteria are met:
///     - Current directory contains a `.py` file
///     - Current directory contains a `.pyproject.toml` file
///     - Current directory contains a `requirements.txt` file
///     - Current directory contains a `pyproject.toml` file
pub fn segment(context: &Context) -> Option<Segment> {
    let python_criteria = find_file::Criteria {
        files: vec!["requirements.txt, pyproject.toml", ".pyproject.toml"],
        extension: "py".to_string(),
        folder: "".to_string(),
    };

    let is_py_project = find_file::is_lang_project(&context.dir_files, &python_criteria);;
    if !is_py_project {
        return None;
    }

    match get_python_version() {
        Some(python_version) => {
            const PYTHON_CHAR: &str = "🐍 ";
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
