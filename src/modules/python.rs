use std::env;
use std::path::Path;

use super::{Context, Module, RootModuleConfig, SegmentConfig};
use crate::configs::python::PythonConfig;
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

    let is_venv = env::var("VIRTUAL_ENV").ok().is_some();

    if !is_py_project && !is_venv {
        return None;
    }

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    if config.pyenv_version_name {
        let python_version = utils::exec_cmd("pyenv", &["version-name"])?.stdout;
        module.create_segment("pyenv_prefix", &config.pyenv_prefix);
        module.create_segment("version", &SegmentConfig::new(&python_version.trim()));
    } else {
        let python_version = get_python_version(&config.python_binary)?;
        let formatted_version = format_python_version(&python_version);
        module.create_segment("version", &SegmentConfig::new(&formatted_version));
    };

    if let Some(virtual_env) = get_python_virtual_env() {
        module.create_segment(
            "virtualenv",
            &SegmentConfig::new(&format!(" ({})", virtual_env)),
        );
    };

    Some(module)
}

fn get_python_version(python_binary: &str) -> Option<String> {
    match utils::exec_cmd(python_binary, &["--version"]) {
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
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

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

    #[test]
    fn folder_without_python_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("python", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_python_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".python-version"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_requirements_txt() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("requirements.txt"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_pyproject_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pyproject.toml"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_pipfile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Pipfile"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_tox() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("tox.ini"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_setup_py() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("setup.py"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_init_py() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("__init__.py"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_py_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.py"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn disabled_scan_for_pyfiles_and_folder_with_ignored_py_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("foo.py"))?.sync_all()?;

        let expected = None;
        let config = toml::toml! {
            [python]
            scan_for_pyfiles = false
        };
        let actual = render_module("python", dir.path(), Some(config));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn disabled_scan_for_pyfiles_and_folder_with_setup_py() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("setup.py"))?.sync_all()?;

        let config = toml::toml! {
            [python]
            scan_for_pyfiles = false
        };

        check_python2_renders(&dir, Some(config));

        let config_python3 = toml::toml! {
            [python]
            python_binary = "python3"
            scan_for_pyfiles = false
        };

        check_python3_renders(&dir, Some(config_python3));

        dir.close()
    }

    fn check_python2_renders(dir: &tempfile::TempDir, starship_config: Option<toml::Value>) {
        let actual = render_module("python", dir.path(), starship_config);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐍 v2.7.17")));
        assert_eq!(expected, actual);
    }

    fn check_python3_renders(dir: &tempfile::TempDir, starship_config: Option<toml::Value>) {
        let config = Some(starship_config.unwrap_or(toml::toml! {
             [python]
             python_binary = "python3"
        }));

        let actual = render_module("python", dir.path(), config);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐍 v3.8.0")));
        assert_eq!(expected, actual);
    }
}
