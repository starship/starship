use std::env;
use std::path::Path;
use std::process::Command;

use super::{Context, Module, RootModuleConfig};
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
    let mut module = context.new_module("python");
    let config = PythonConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

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

    if !is_py_project {
        return None;
    }

    let pyenv_version_name = config.pyenv_version_name;

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    let python_version = select_python_version(pyenv_version_name)?;

    if pyenv_version_name {
        module.create_segment("pyenv_prefix", &config.pyenv_prefix);
        module.create_segment(
            "version",
            &config.version.with_value(&python_version.trim()),
        );
    } else {
        let formatted_version = format_python_version(&python_version);
        module.create_segment("version", &config.version.with_value(&formatted_version));
        if let Some(venv_version) = get_python_virtual_env() {
            module.create_segment(
                "virtual_env",
                &config
                    .virtual_env
                    .with_value(&format!("({})", venv_version)),
            );
        }
    };

    Some(module)
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
