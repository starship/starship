use std::env;
use std::path::Path;
use std::process::Command;

use super::{Context, Module, RootModuleConfig, SegmentConfig};
use crate::configs::python::PythonConfig;

/// Creates a module with the current Python version
///
/// Will display the Python version if any of the following criteria are met:
///     - Current directory contains a `.python-version` file
///     - Current directory contains a `requirements.txt` file
///     - Current directory contains a `pyproject.toml` file
///     - Current directory contains a file with the `.py` extension
///     - Current directory contains a `Pipfile` file
///     - Current directory contains a `tox.ini` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_py_project = context
        .try_begin_scan()?
        .set_files(&[
            "requirements.txt",
            ".python-version",
            "pyproject.toml",
            "Pipfile",
            "tox.ini",
        ])
        .set_extensions(&["py"])
        .is_match();

    let is_venv = env::var("VIRTUAL_ENV").ok().is_some();

    if !is_py_project && !is_venv {
        return None;
    }

    let mut module = context.new_module("python");
    let config: PythonConfig = PythonConfig::try_load(module.config);

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    if config.pyenv_version_name {
        let python_version = get_pyenv_version()?;
        module.create_segment("pyenv_prefix", &config.pyenv_prefix);
        module.create_segment("version", &SegmentConfig::new(&python_version.trim()));
    } else {
        let python_version = get_python_version()?;
        let formatted_version = format_python_version(&python_version);
        module.create_segment("version", &SegmentConfig::new(&formatted_version));

        if let Some(virtual_env) = get_python_virtual_env() {
            module.create_segment(
                "virtualenv",
                &SegmentConfig::new(&format!(" ({})", virtual_env)),
            );
        };
    };

    Some(module)
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
            if !output.status.success() {
                log::warn!(
                    "Non-Zero exit code '{}' when executing `python --version`",
                    output.status
                );
                return None;
            }
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
