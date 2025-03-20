use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::{Context, Module, ModuleConfig};
use crate::configs::dbt::DbtConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils::{context_path, get_command_string_output, read_file, CommandOutput};

const DBT_PROJECT_YML: &str = "dbt_project.yml";

/// Creates a module with the current dbt version and dbt project
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("dbt");
    let config: DbtConfig = DbtConfig::try_load(module.config);

    let is_dbt_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_dbt_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let dbt_version = get_dbt_version(context, &config)?;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &dbt_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                "project" => read_project_name_from_dbt_project_yml(context, &config).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `dbt`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

/// Try to read a dbt project's name from a YAML file in the current directory or in config.project_dirs
fn read_project_name_from_dbt_project_yml(context: &Context, config: &DbtConfig) -> Option<String> {
    let file_contents = context
        .read_file_from_pwd(DBT_PROJECT_YML)
        .or_else(|| try_find_dbt_project_yaml(context, config))?;

    let dbt_project_yaml = yaml_rust::YamlLoader::load_from_str(&file_contents).ok()?;
    let dbt_project_name = dbt_project_yaml.first()?["name"].as_str()?;

    Some(dbt_project_name.to_string())
}

/// Try to find a dbt_project.yml file in one of the directories specified in config.project_dirs
/// We only consider directories that are also prefixes of our current directory
fn try_find_dbt_project_yaml(context: &Context, config: &DbtConfig) -> Option<String> {
    for project_dir in config.project_dirs.iter() {
        let dir = Context::expand_tilde(PathBuf::from_str(project_dir).ok()?);
        if !context.current_dir.starts_with(&dir) {
            continue;
        }

        let dbt_project_yaml_path = dir.join(DBT_PROJECT_YML);
        return read_file(dbt_project_yaml_path).ok();
    }

    None
}

/// Attempt to get the installed version of dbt-core by finding where dbt is installed
fn get_dbt_version(context: &Context, config: &DbtConfig) -> Option<String> {
    // Python requires a directory that specifies the version to be created where the package is installed
    // See: https://packaging.python.org/en/latest/specifications/recording-installed-packages/#the-dist-info-directory
    // We will rely on this directory existing to extract the version number
    // This is significantly faster (50ms) than using dbt --version (>500ms), pip freeze (450ms), or importing dbt
    let dbt_package_dir = config
        .python_binary
        .0
        .iter()
        .find_map(|binary| {
            context.exec_cmd(
                binary,
                &[
                    "-c",
                    "from importlib.resources import files;print(files('dbt') / '');",
                ],
            )
        })
        .map(get_pathbuf_from_string_output)??;
    // Example output: /home/username/.pyenv/versions/3.10.5/lib/python3.10/site-packages/dbt
    let python_packages_dir = context_path(context, dbt_package_dir.parent()?);

    try_find_dbt_package_version_python_packages_dir(&python_packages_dir)
}

fn get_pathbuf_from_string_output(command: CommandOutput) -> Option<PathBuf> {
    if command.stdout.is_empty() {
        None
    } else {
        PathBuf::from_str(get_command_string_output(command).trim()).ok()
    }
}

/// Try to find a dist-info directory where dbt was installed
/// If found, we parse it to quickly extract the version number
fn try_find_dbt_package_version_python_packages_dir(python_packages_dir: &Path) -> Option<String> {
    for entry in python_packages_dir.read_dir().ok()? {
        let path = entry.ok()?.path();
        let dir_name = path.file_name()?.to_str()?;

        if let Some(version) = parse_dbt_version(dir_name) {
            return Some(version);
        }
    }

    None
}

/// Parse a dbt version from a dbt dist-info directory
fn parse_dbt_version(dbt_dist_info: &str) -> Option<String> {
    // This directory has the following format: dbt_core-{version}.dist-info
    let version = dbt_dist_info
        .split_once("dbt_core-")?
        // ("", "{version}.dist_info")
        .1
        .split_once(".dist-info")?
        // ("{version}", "")
        .0;

    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{create_dir_all, File};
    use std::io;
    use std::io::Write;

    fn create_dbt_dist_info_dir(renderer: &ModuleRenderer) -> io::Result<()> {
        // We rely on a dist-info directory to read the version, and we use
        // context_path to create a path to the directory. During testing,
        // context_path will prefix the path return by the python command with
        // context.root_dir. As ModuleRenderer uses a default context which
        // creates it's own temporary directory as root_dir, we need to use that here.
        let site_packages_dir = if cfg!(target_os = "windows") {
            "site-packages\\dbt_core-1.2.2.dist-info"
        } else {
            "site-packages/dbt_core-1.2.2.dist-info"
        };
        let dbt_version_dir = renderer.root_path().join(site_packages_dir);
        create_dir_all(dbt_version_dir)
    }

    #[test]
    fn test_get_pathbuf_from_string_output() {
        let output = CommandOutput {
            stdout: "/home/username/lib".to_string(),
            stderr: "".to_string(),
        };
        assert_eq!(
            get_pathbuf_from_string_output(output),
            Some(PathBuf::from_str("/home/username/lib").unwrap())
        );

        let output = CommandOutput {
            stdout: "".to_string(),
            stderr: "ERROR".to_string(),
        };
        assert_eq!(get_pathbuf_from_string_output(output), None);
    }

    #[test]
    fn test_parse_dbt_version() {
        let dir_name = "dbt_core-1.2.2.dist-info";
        assert_eq!(parse_dbt_version(dir_name), Some("1.2.2".to_string()));
    }

    #[test]
    fn folder_with_sql_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("model.sql"))?.sync_all()?;

        let renderer = ModuleRenderer::new("dbt");
        create_dbt_dist_info_dir(&renderer)?;

        let actual = renderer.path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(255, 105, 74).bold().paint("📊 v1.2.2 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_sql_file_and_dbt_project_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dbt_project.yml"))?.write_all(b"name: my-test-project\n")?;

        let model_path = dir.path().join("models/model.sql");
        create_dir_all(model_path.parent().unwrap())?;
        File::create(model_path)?.sync_all()?;

        let config: toml::Value = toml::from_str(&format!(
            r#"
            [dbt]
            project_dirs = ["{}"]
        "#,
            dir.path().to_str().unwrap().replace('\\', r"\\"),
        ))?;

        let renderer = ModuleRenderer::new("dbt");
        create_dbt_dist_info_dir(&renderer)?;

        let actual = renderer
            .path(dir.path().join("models"))
            .config(config)
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(255, 105, 74)
                .bold()
                .paint("📊 v1.2.2 (my-test-project) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dbt_project_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dbt_project.yml"))?.write_all(b"name: my-test-project\n")?;

        let renderer = ModuleRenderer::new("dbt");
        create_dbt_dist_info_dir(&renderer)?;
        let actual = renderer.path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(255, 105, 74)
                .bold()
                .paint("📊 v1.2.2 (my-test-project) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_without_anything() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("dbt").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);

        dir.close()
    }
}
