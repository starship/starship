use std::env;
use std::path::Path;
use std::process::Command;

use ansi_term::Color;

use super::{Context, Module};

/// Creates a module with the current Python version
///
/// Will display the Python version if any of the following criteria are met:
///     - Current directory contains a `.python-version` file
///     - Current directory contains a `requirements.txt` file
///     - Current directory contains a `pyproject.toml` file
///     - Current directory contains a file with the `.py` extension
///     - Current directory contains a `Pipfile` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_py_project = context
        .try_begin_scan()?
        .set_files(&[
            "requirements.txt",
            ".python-version",
            "pyproject.toml",
            "Pipfile",
        ])
        .set_extensions(&["py"])
        .is_match();

    if !is_py_project {
        return None;
    }

    let mut module = context.new_module("python");
    let pyenv_version_name = module
        .config_value_bool("pyenv_version_name")
        .unwrap_or(false);

    const PYTHON_CHAR: &str = "🐍 ";
    let module_color = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Yellow.bold());
    module.set_style(module_color);
    module.new_segment("symbol", PYTHON_CHAR);

    select_python_version(pyenv_version_name)
        .map(|python_version| python_module(module, pyenv_version_name, python_version))
}

fn python_module(mut module: Module, pyenv_version_name: bool, python_version: String) -> Module {
    const PYENV_PREFIX: &str = "pyenv ";

    if pyenv_version_name {
        module.new_segment("pyenv_prefix", PYENV_PREFIX);
        module.new_segment("version", &python_version.trim());
    } else {
        let formatted_version = format_python_version(&python_version);
        module.new_segment("version", &formatted_version);
        get_python_virtual_env()
            .map(|virtual_env| module.new_segment("virtualenv", &format!("({})", virtual_env)));
    };

    module
}

fn select_python_version(pyenv_version_name: bool) -> Option<String> {
    if pyenv_version_name {
        get_pyenv_version()
    } else {
        get_python_version()
    }
}

fn get_pyenv_version() -> Option<String> {
    Command::new("pyenv")
        .arg("version-name")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
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

fn format_python_version(python_stdout: &str) -> String {
    format!("v{}", python_stdout.trim_start_matches("Python ").trim())
}

fn get_python_virtual_env() -> Option<String> {
    env::var("VIRTUAL_ENV").ok().and_then(|venv| {
        Path::new(&venv)
            .file_name()
            .map(|filename| String::from(filename.to_str().unwrap_or("")))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_python_version() {
        let input = "Python 3.7.2";
        assert_eq!(format_python_version(input), "v3.7.2");
    }
}
