use ini::Ini;
use std::path::Path;

use super::{Context, Module, ModuleConfig};
use crate::configs::python::PythonConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils::get_command_string_output;

/// Creates a module with the current Python version and, if active, virtual environment.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("python");
    let config: PythonConfig = PythonConfig::try_load(module.config);

    let is_py_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    let is_venv = context.get_env("VIRTUAL_ENV").is_some();

    if !is_py_project && !is_venv {
        return None;
    };

    let pyenv_prefix = if config.pyenv_version_name {
        config.pyenv_prefix
    } else {
        ""
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    if config.pyenv_version_name {
                        return get_pyenv_version(context).map(Ok);
                    }
                    let python_version = get_python_version(context, &config)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &python_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                "virtualenv" => {
                    let virtual_env = get_python_virtual_env(context);
                    virtual_env.as_ref().map(|e| Ok(e.trim().to_string()))
                }
                "pyenv_prefix" => Some(Ok(pyenv_prefix.to_string())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `python`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_pyenv_version(context: &Context) -> Option<String> {
    let mut version_name = context.get_env("PYENV_VERSION");

    if version_name.is_none() {
        version_name = Some(
            context
                .exec_cmd("pyenv", &["version-name"])?
                .stdout
                .trim()
                .to_string(),
        )
    }

    version_name
}

fn get_python_version(context: &Context, config: &PythonConfig) -> Option<String> {
    let version = config
        .python_binary
        .0
        .iter()
        .find_map(|binary| context.exec_cmd(binary, &["--version"]))
        .map(get_command_string_output)?;

    parse_python_version(&version)
}

fn parse_python_version(python_version_string: &str) -> Option<String> {
    let version = python_version_string
        // split into ["Python", "3.8.6", ...]
        .split_whitespace()
        // get down to "3.8.6"
        .nth(1)?;

    Some(version.to_string())
}

fn get_python_virtual_env(context: &Context) -> Option<String> {
    context.get_env("VIRTUAL_ENV").and_then(|venv| {
        get_prompt_from_venv(Path::new(&venv)).or_else(|| {
            Path::new(&venv)
                .file_name()
                .map(|filename| String::from(filename.to_str().unwrap_or("")))
        })
    })
}
fn get_prompt_from_venv(venv_path: &Path) -> Option<String> {
    Ini::load_from_file(venv_path.join("pyvenv.cfg"))
        .ok()?
        .general_section()
        .get("prompt")
        .map(|prompt| String::from(prompt.trim_matches(&['(', ')'] as &[_])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{create_dir_all, File};
    use std::io;
    use std::io::Write;

    #[test]
    fn test_parse_python_version() {
        assert_eq!(
            parse_python_version("Python 3.7.2"),
            Some("3.7.2".to_string())
        );
    }

    #[test]
    fn test_parse_python_version_is_malformed() {
        assert_eq!(parse_python_version("Python 3.7"), Some("3.7".to_string()));
    }

    #[test]
    fn test_parse_python_version_anaconda() {
        assert_eq!(
            parse_python_version("Python 3.6.10 :: Anaconda, Inc.",),
            Some("3.6.10".to_string())
        );
    }

    #[test]
    fn test_parse_python_version_pypy() {
        assert_eq!(
            parse_python_version(
                "\
Python 3.7.9 (7e6e2bb30ac5fbdbd443619cae28c51d5c162a02, Nov 24 2020, 10:03:59)
[PyPy 7.3.3-beta0 with GCC 10.2.0]",
            ),
            Some("3.7.9".to_string())
        );
    }

    #[test]
    fn folder_without_python_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("python").path(dir.path()).collect();
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
        check_pyenv_renders(&dir, None);
        check_multiple_binaries_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_requirements_txt() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("requirements.txt"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        check_pyenv_renders(&dir, None);
        check_multiple_binaries_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_pyproject_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pyproject.toml"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        check_pyenv_renders(&dir, None);
        check_multiple_binaries_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_pipfile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Pipfile"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        check_pyenv_renders(&dir, None);
        check_multiple_binaries_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_tox() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("tox.ini"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        check_pyenv_renders(&dir, None);
        check_multiple_binaries_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_setup_py() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("setup.py"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        check_pyenv_renders(&dir, None);
        check_multiple_binaries_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_init_py() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("__init__.py"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        check_pyenv_renders(&dir, None);
        check_multiple_binaries_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn folder_with_py_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.py"))?.sync_all()?;

        check_python2_renders(&dir, None);
        check_python3_renders(&dir, None);
        check_pyenv_renders(&dir, None);
        check_multiple_binaries_renders(&dir, None);
        dir.close()
    }

    #[test]
    fn disabled_scan_for_pyfiles_and_folder_with_ignored_py_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("foo.py"))?.sync_all()?;

        let expected = None;
        let config = toml::toml! {
            [python]
            detect_extensions = []
        };
        let actual = ModuleRenderer::new("python")
            .path(dir.path())
            .config(config)
            .collect();
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn disabled_scan_for_pyfiles_and_folder_with_setup_py() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("setup.py"))?.sync_all()?;

        let config = toml::toml! {
            [python]
            python_binary = "python2"
            detect_extensions = []
        };

        check_python2_renders(&dir, Some(config));

        let config_python3 = toml::toml! {
            [python]
            python_binary = "python3"
            detect_extensions = []
        };

        check_python3_renders(&dir, Some(config_python3));

        let config_pyenv = toml::toml! {
            [python]
            pyenv_version_name = true
            pyenv_prefix = "test_pyenv "
            detect_extensions = []
        };
        check_pyenv_renders(&dir, Some(config_pyenv));

        let config_multi = toml::toml! {
            [python]
            python_binary = ["python", "python3"]
            detect_extensions = []
        };
        check_multiple_binaries_renders(&dir, Some(config_multi));

        dir.close()
    }

    #[test]
    fn with_virtual_env() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.py"))?.sync_all()?;
        let actual = ModuleRenderer::new("python")
            .path(dir.path())
            .env("VIRTUAL_ENV", "/foo/bar/my_venv")
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐍 v3.8.0 (my_venv) ")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn with_active_venv() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("python")
            .path(dir.path())
            .env("VIRTUAL_ENV", "/foo/bar/my_venv")
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐍 v3.8.0 (my_venv) ")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn with_active_venv_and_prompt() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        create_dir_all(dir.path().join("my_venv"))?;
        let mut venv_cfg = File::create(dir.path().join("my_venv").join("pyvenv.cfg"))?;
        venv_cfg.write_all(
            br#"
home = something
prompt = 'foo'
        "#,
        )?;
        venv_cfg.sync_all()?;

        let actual = ModuleRenderer::new("python")
            .path(dir.path())
            .env("VIRTUAL_ENV", dir.path().join("my_venv").to_str().unwrap())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐍 v3.8.0 (foo) ")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn with_active_venv_and_dirty_prompt() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        create_dir_all(dir.path().join("my_venv"))?;
        let mut venv_cfg = File::create(dir.path().join("my_venv").join("pyvenv.cfg"))?;
        venv_cfg.write_all(
            br#"
home = something
prompt = '(foo)'
        "#,
        )?;
        venv_cfg.sync_all()?;

        let actual = ModuleRenderer::new("python")
            .path(dir.path())
            .env("VIRTUAL_ENV", dir.path().join("my_venv").to_str().unwrap())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐍 v3.8.0 (foo) ")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    fn check_python2_renders(dir: &tempfile::TempDir, starship_config: Option<toml::Value>) {
        let config = starship_config.unwrap_or(toml::toml! {
            [python]
            python_binary = "python2"
        });

        let actual = ModuleRenderer::new("python")
            .path(dir.path())
            .config(config)
            .collect();

        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐍 v2.7.17 ")));
        assert_eq!(expected, actual);
    }

    fn check_python3_renders(dir: &tempfile::TempDir, starship_config: Option<toml::Value>) {
        let config = starship_config.unwrap_or(toml::toml! {
             [python]
             python_binary = "python3"
        });

        let actual = ModuleRenderer::new("python")
            .path(dir.path())
            .config(config)
            .collect();

        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐍 v3.8.0 ")));
        assert_eq!(expected, actual);
    }

    fn check_multiple_binaries_renders(
        dir: &tempfile::TempDir,
        starship_config: Option<toml::Value>,
    ) {
        let config = starship_config.unwrap_or(toml::toml! {
             [python]
             python_binary = ["python", "python3"]
        });

        let actual = ModuleRenderer::new("python")
            .path(dir.path())
            .config(config)
            .collect();

        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐍 v3.8.0 ")));
        assert_eq!(expected, actual);
    }

    fn check_pyenv_renders(dir: &tempfile::TempDir, starship_config: Option<toml::Value>) {
        let config = starship_config.unwrap_or(toml::toml! {
             [python]
             pyenv_version_name = true
             pyenv_prefix = "test_pyenv "
        });

        let actual = ModuleRenderer::new("python")
            .path(dir.path())
            .config(config)
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐍 test_pyenv system ")
        ));
        assert_eq!(expected, actual);
    }
}
