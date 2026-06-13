use super::{Context, Module, ModuleConfig};

use crate::configs::aspire::AspireConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Aspire CLI version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("aspire");
    let config = AspireConfig::try_load(module.config);

    let is_aspire_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_aspire_project {
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
                "version" => {
                    let aspire_version = get_aspire_version(context)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &aspire_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `aspire`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_aspire_version(context: &Context) -> Option<String> {
    // Run from a neutral working directory so the Aspire CLI doesn't try to
    // parse the project's `aspire.config.json` just to print its version. That
    // keeps the version readable even when the config is missing or invalid.
    #[cfg(not(test))]
    {
        use crate::utils::{create_command, exec_timeout};
        use std::time::Duration;

        let mut cmd = create_command("aspire").ok()?;
        cmd.arg("--version").current_dir(std::env::temp_dir());
        let output = exec_timeout(
            &mut cmd,
            Duration::from_millis(context.root_config.command_timeout),
        )?;
        Some(parse_aspire_version(&output.stdout))
    }
    #[cfg(test)]
    {
        context
            .exec_cmd("aspire", &["--version"])
            .map(crate::utils::get_command_string_output)
            .map(|s| parse_aspire_version(&s))
    }
}

fn parse_aspire_version(aspire_version: &str) -> String {
    aspire_version.trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_aspire_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("aspire").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_aspire_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("aspire.config.json"))?.sync_all()?;
        let actual = ModuleRenderer::new("aspire")
            .path(dir.path())
            .cmd(
                "aspire --version",
                Some(crate::utils::CommandOutput {
                    stdout: String::from("13.4.0-pr.17161.g80e4a73e\n"),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("▲ v13.4.0-pr.17161.g80e4a73e ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn no_aspire_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("aspire.config.json"))?.sync_all()?;
        let actual = ModuleRenderer::new("aspire")
            .path(dir.path())
            .cmd("aspire --version", None)
            .collect();
        let expected = Some(format!("via {}", Color::Purple.bold().paint("▲ ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
