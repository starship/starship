use super::{Context, Module, ModuleConfig};

use crate::configs::krypton::KryptonConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("krypton");
    let config = KryptonConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let is_krypton_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match()
        || context
            .get_env("KRYPTON_ROOT")
            .is_some_and(|value| !value.is_empty());

    if !is_krypton_project {
        return None;
    }

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
                "version" => context
                    .exec_cmd("kcc", &["--version"])
                    .map(|command_output| command_output.stdout)
                    .and_then(|version_output| {
                        let version = parse_krypton_version(&version_output)?;
                        VersionFormatter::format_module_version(
                            module.get_name(),
                            &version,
                            config.version_format,
                        )
                    })
                    .map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `krypton`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn parse_krypton_version(version_cmd_output: &str) -> Option<String> {
    version_cmd_output
        .lines()
        .next()?
        .split_once("kcc version ")
        .map(|(_, version)| version.trim().to_string())
        .filter(|v| !v.is_empty())
}

#[cfg(test)]
mod tests {
    use super::parse_krypton_version;
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    fn mock_kcc_output(version: &str) -> CommandOutput {
        CommandOutput {
            stdout: format!("kcc version {version}\n"),
            stderr: String::new(),
        }
    }

    #[test]
    fn parse_krypton_version_output() {
        let output = "kcc version 2.5.0-alpha\n";
        assert_eq!(
            parse_krypton_version(output),
            Some("2.5.0-alpha".to_string())
        );
        assert!(parse_krypton_version("kcc not a version string").is_none());
        assert!(parse_krypton_version("").is_none());
    }

    #[test]
    fn folder_without_krypton() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("plain.txt"))?.sync_all()?;

        let actual = ModuleRenderer::new("krypton").path(dir.path()).collect();

        assert_eq!(None, actual);
        dir.close()
    }

    #[test]
    fn folder_with_krypton_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.k"))?.sync_all()?;

        let actual = ModuleRenderer::new("krypton")
            .path(dir.path())
            .cmd("kcc --version", Some(mock_kcc_output("2.5.0-alpha")))
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("\u{E815} v2.5.0-alpha ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_krpton_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("krypton.toml"))?.sync_all()?;

        let actual = ModuleRenderer::new("krypton")
            .path(dir.path())
            .cmd("kcc --version", Some(mock_kcc_output("2.4.9")))
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("\u{E815} v2.4.9 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn krypton_root_env_triggers_detection() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("krypton")
            .path(dir.path())
            .env("KRYPTON_ROOT", "/somewhere/krypton")
            .cmd("kcc --version", Some(mock_kcc_output("2.5.0")))
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("\u{E815} v2.5.0 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn disabled_config_returns_none() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.k"))?.sync_all()?;

        let actual = ModuleRenderer::new("krypton")
            .path(dir.path())
            .cmd("kcc --version", Some(mock_kcc_output("2.5.0-alpha")))
            .config(toml::toml! {
                [krypton]
                disabled = true
            })
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }
}
