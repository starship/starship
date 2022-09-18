use std::path::PathBuf;

use super::{Context, Module, ModuleConfig};

use crate::configs::docker_context::DockerContextConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the currently active Docker context
///
/// Will display the Docker context if the following criteria are met:
///     - There is a non-empty environment variable named `DOCKER_HOST`
///     - Or there is a non-empty environment variable named `DOCKER_CONTEXT`
///     - Or there is a file named `$HOME/.docker/config.json`
///     - Or a file named `$DOCKER_CONFIG/config.json`
///     - The file is JSON and contains a field named `currentContext`
///     - The value of `currentContext` is not `default`
///     - If multiple criteria are met, we use the following order to define the docker context:
///     - `DOCKER_HOST`, `DOCKER_CONTEXT`, $HOME/.docker/config.json, $`DOCKER_CONFIG/config.json`
///     - (This is the same order docker follows, as `DOCKER_HOST` and `DOCKER_CONTEXT` override the
///     config)
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

    let docker_context_env = ["DOCKER_MACHINE_NAME", "DOCKER_HOST", "DOCKER_CONTEXT"]
        .into_iter()
        .find_map(|env| context.get_env(env));

    let ctx = match docker_context_env {
        Some(data) => data,
        _ => {
            if !docker_config.exists() {
                return None;
            }
            let json = utils::read_file(docker_config).ok()?;
            let parsed_json: serde_json::Value = serde_json::from_str(&json).ok()?;
            parsed_json.get("currentContext")?.as_str()?.to_owned()
        }
    };

    if ctx == "default" {
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
                "context" => Some(Ok(ctx.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
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

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
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

    #[test]
    fn test_docker_host_env() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_HOST", "udp://starship@127.0.0.1:53")
            .config(toml::toml! {
                [docker_context]
                only_with_files = false
            })
            .collect();
        let expected = Some(format!(
            "via {} ",
            Color::Blue.bold().paint("🐳 udp://starship@127.0.0.1:53")
        ));

        assert_eq!(expected, actual);

        cfg_dir.close()
    }

    #[test]
    fn test_docker_context_env() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONTEXT", "starship")
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
    fn test_docker_context_default() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONTEXT", "default")
            .config(toml::toml! {
                [docker_context]
                only_with_files = false
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);

        cfg_dir.close()
    }

    #[test]
    fn test_docker_context_overrides_config() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;

        let cfg_file = cfg_dir.path().join("config.json");

        let config_content = serde_json::json!({
            "currentContext": "starship"
        });

        let mut docker_config = File::create(&cfg_file)?;
        docker_config.write_all(config_content.to_string().as_bytes())?;
        docker_config.sync_all()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_CONTEXT", "starship")
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
    fn test_docker_host_overrides_docker_context_env_and_conf() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;

        let cfg_file = cfg_dir.path().join("config.json");

        let config_content = serde_json::json!({
            "currentContext": "starship"
        });

        let mut docker_config = File::create(&cfg_file)?;
        docker_config.write_all(config_content.to_string().as_bytes())?;
        docker_config.sync_all()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_HOST", "udp://starship@127.0.0.1:53")
            .env("DOCKER_CONTEXT", "starship")
            .env("DOCKER_CONFIG", cfg_dir.path().to_string_lossy())
            .config(toml::toml! {
                [docker_context]
                only_with_files = false
            })
            .collect();
        let expected = Some(format!(
            "via {} ",
            Color::Blue.bold().paint("🐳 udp://starship@127.0.0.1:53")
        ));

        assert_eq!(expected, actual);

        cfg_dir.close()
    }
    #[test]
    fn test_docker_machine_name_overrides_other_env_vars_and_conf() -> io::Result<()> {
        let cfg_dir = tempfile::tempdir()?;

        let cfg_file = cfg_dir.path().join("config.json");

        let config_content = serde_json::json!({
            "currentContext": "starship"
        });

        let mut docker_config = File::create(&cfg_file)?;
        docker_config.write_all(config_content.to_string().as_bytes())?;
        docker_config.sync_all()?;

        let actual = ModuleRenderer::new("docker_context")
            .env("DOCKER_MACHINE_NAME", "machine_name")
            .env("DOCKER_HOST", "udp://starship@127.0.0.1:53")
            .env("DOCKER_CONTEXT", "starship")
            .env("DOCKER_CONFIG", cfg_dir.path().to_string_lossy())
            .config(toml::toml! {
                [docker_context]
                only_with_files = false
            })
            .collect();
        let expected = Some(format!(
            "via {} ",
            Color::Blue.bold().paint("🐳 machine_name")
        ));

        assert_eq!(expected, actual);

        cfg_dir.close()
    }
}
