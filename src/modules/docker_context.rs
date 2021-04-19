use std::path::PathBuf;

use super::{Context, Module, RootModuleConfig};

use crate::configs::docker_context::DockerContextConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the currently active Docker context
///
/// Will display the Docker context if the following criteria are met:
///     - There is a file named `$HOME/.docker/config.json`
///     - Or a file named `$DOCKER_CONFIG/config.json`
///     - The file is JSON and contains a field named `currentContext`
///     - The value of `currentContext` is not `default`
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("docker_context");
    let config: DockerContextConfig = DockerContextConfig::try_load(module.config);

    if config.only_with_files
        && !context
            .try_begin_scan()?
            .set_files(&config.detect_files)
            .set_extensions(&config.detect_extensions)
            .set_folders(&config.detect_folders)
            .is_match()
    {
        return None;
    }

    let docker_config = PathBuf::from(
        &context
            .get_env_os("DOCKER_CONFIG")
            .unwrap_or(context.get_home()?.join(".docker").into_os_string()),
    )
    .join("config.json");

    if !docker_config.exists() {
        return None;
    }

    let json = utils::read_file(docker_config).ok()?;
    let parsed_json = serde_json::from_str(&json).ok()?;

    match parsed_json {
        serde_json::Value::Object(root) => {
            let current_context = root.get("currentContext")?;
            match current_context {
                serde_json::Value::String(ctx) => {
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
                                "context" => Some(Ok(ctx)),
                                _ => None,
                            })
                            .parse(None)
                    });

                    module.set_segments(match parsed {
                        Ok(segments) => segments,
                        Err(error) => {
                            log::warn!("Error in module `docker_context`:\n{}", error);
                            return None;
                        }
                    });

                    Some(module)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io::{self, Write};

    #[test]
    fn only_trigger_when_docker_config_exists() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONFIG", cfg_dir.path().to_string_lossy())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        cfg_dir.close()
    }

    #[test]
    fn test_with_docker_compose_yml() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;
        let cfg_file = cfg_dir.path().join("config.json");

        let pwd = tempfile::tempdir()?;
        File::create(pwd.path().join("docker-compose.yml"))?.sync_all()?;

        let config_content = serde_json::json!({
            "currentContext": "starship"
        });

        let mut docker_config = File::create(&cfg_file)?;
        docker_config.write_all(config_content.to_string().as_bytes())?;
        docker_config.sync_all()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONFIG", cfg_dir.path().to_string_lossy())
            .path(pwd.path())
            .collect();

        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🐳 starship")));

        assert_eq!(expected, actual);

        cfg_dir.close()?;
        pwd.close()
    }

    #[test]
    fn test_with_docker_compose_yaml() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;
        let cfg_file = cfg_dir.path().join("config.json");

        let pwd = tempfile::tempdir()?;
        File::create(pwd.path().join("docker-compose.yaml"))?.sync_all()?;

        let config_content = serde_json::json!({
            "currentContext": "starship"
        });

        let mut docker_config = File::create(&cfg_file)?;
        docker_config.write_all(config_content.to_string().as_bytes())?;
        docker_config.sync_all()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONFIG", cfg_dir.path().to_string_lossy())
            .path(pwd.path())
            .collect();

        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🐳 starship")));

        assert_eq!(expected, actual);

        cfg_dir.close()?;
        pwd.close()
    }

    #[test]
    fn test_with_dockerfile() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;
        let cfg_file = cfg_dir.path().join("config.json");

        let pwd = tempfile::tempdir()?;
        File::create(pwd.path().join("Dockerfile"))?.sync_all()?;

        let config_content = serde_json::json!({
            "currentContext": "starship"
        });

        let mut docker_config = File::create(&cfg_file)?;
        docker_config.write_all(config_content.to_string().as_bytes())?;
        docker_config.sync_all()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONFIG", cfg_dir.path().to_string_lossy())
            .path(pwd.path())
            .collect();

        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🐳 starship")));

        assert_eq!(expected, actual);

        cfg_dir.close()?;
        pwd.close()
    }

    #[test]
    fn test_no_docker_files() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;
        let cfg_file = cfg_dir.path().join("config.json");

        let config_content = serde_json::json!({
            "currentContext": "starship"
        });

        let mut docker_config = File::create(&cfg_file)?;
        docker_config.write_all(config_content.to_string().as_bytes())?;
        docker_config.sync_all()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONFIG", cfg_dir.path().to_string_lossy())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);

        cfg_dir.close()
    }

    #[test]
    fn test_no_scan_for_docker_files() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;
        let cfg_file = cfg_dir.path().join("config.json");

        let config_content = serde_json::json!({
            "currentContext": "starship"
        });

        let mut docker_config = File::create(&cfg_file)?;
        docker_config.write_all(config_content.to_string().as_bytes())?;
        docker_config.sync_all()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONFIG", cfg_dir.path().to_string_lossy())
            .config(toml::toml! {
                [docker_context]
                only_with_files = false
            })
            .collect();

        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🐳 starship")));

        assert_eq!(expected, actual);

        cfg_dir.close()
    }

    #[test]
    fn test_invalid_json() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;
        let cfg_file = cfg_dir.path().join("config.json");

        let config_content = "not valid json";

        let mut docker_config = File::create(&cfg_file)?;
        docker_config.write_all(config_content.to_string().as_bytes())?;
        docker_config.sync_all()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONFIG", cfg_dir.path().to_string_lossy())
            .config(toml::toml! {
                [docker_context]
                only_with_files = false
            })
            .collect();

        let expected = None;

        assert_eq!(expected, actual);

        cfg_dir.close()
    }
}
