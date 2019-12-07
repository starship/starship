use std::env;
use std::path::Path;

use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig};
use crate::configs::python::PythonConfig;
use crate::segment::Segment;
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

    let segments: Vec<Segment> = format_segments_nested(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "version" => Some(vec![Segment {
                _name: "version".to_string(),
                value: if config.pyenv_version_name {
                    utils::exec_cmd("pyenv", &["version-name"])?.stdout
                } else {
                    let python_version = get_python_version()?;
                    format_python_version(&python_version)
                },
                style,
            }]),
            "pyenv" => {
                let venv = get_python_virtual_env()?;
                format_segments(config.pyenv_format, style, |name, query| {
                    let style = get_style_from_query(&query);
                    match name {
                        "name" => Some(Segment {
                            _name: "pyenv_name".to_string(),
                            value: venv.clone(),
                            style,
                        }),
                        _ => None,
                    }
                })
                .ok()
            }
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

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
