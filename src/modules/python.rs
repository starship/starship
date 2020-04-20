use super::{Context, Module, RootModuleConfig};
use crate::configs::python::PythonConfig;
use crate::formatter::StringFormatter;
use crate::utils;

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
    let config: PythonConfig = PythonConfig::try_load(module.config);

    let is_py_project = {
        let base = context.try_begin_scan()?.set_files(&[
            "requirements.txt",
            ".python-version",
            "pyproject.toml",
            "Pipfile",
            "tox.ini",
            "setup.py",
            "__init__.py",
        ]);
        if config.scan_for_pyfiles {
            base.set_extensions(&["py"]).is_match()
        } else {
            base.is_match()
        }
    };

    if !is_py_project {
        return None;
    }

    let formatted_version = if config.pyenv_version_name {
        utils::exec_cmd("pyenv", &["version-name"])?.stdout
    } else {
        let python_version = get_python_version()?;
        format_python_version(&python_version)
    };

    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter.map(|variable| match variable {
            "version" => Some(formatted_version.clone()),
            _ => None,
        })
    } else {
        log::warn!("Error parsing format string in `python.format`");
        return None;
    };
    module.set_segments(formatter.parse(None));
    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

fn get_python_version() -> Option<String> {
    match utils::exec_cmd("python", &["--version"]) {
        Some(output) => {
            if output.stdout.is_empty() {
                Some(output.stderr)
            } else {
                Some(output.stdout)
            }
        }
        None => None,
    }
}

fn format_python_version(python_stdout: &str) -> String {
    format!(
        "v{}",
        python_stdout
            .trim_start_matches("Python ")
            .trim_end_matches(":: Anaconda, Inc.")
            .trim()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_python_version() {
        let input = "Python 3.7.2";
        assert_eq!(format_python_version(input), "v3.7.2");
    }

    #[test]
    fn test_format_python_version_anaconda() {
        let input = "Python 3.6.10 :: Anaconda, Inc.";
        assert_eq!(format_python_version(input), "v3.6.10");
    }
}
