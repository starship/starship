use super::{Context, Module, RootModuleConfig};

use crate::configs::kotlin::KotlinConfig;
use crate::formatter::StringFormatter;

use regex::Regex;
const KOTLIN_VERSION_PATTERN: &str = "(?P<version>[\\d\\.]+[\\d\\.]+[\\d\\.]+)";

/// Creates a module with the current Kotlin version
pub async fn module<'a>(context: &'a Context<'a>) -> Option<Module<'a>> {
    let mut module = context.new_module("kotlin");
    let config = KotlinConfig::try_load(module.config);

    let is_kotlin_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_kotlin_project {
        return None;
    }

    let parsed = match StringFormatter::new(config.format) {
        Ok(formatter) => formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .async_map(|variable| {
                let config = &config;
                async move {
                    match variable.as_ref() {
                        "version" => {
                            let kotlin_version = format_kotlin_version(
                                &get_kotlin_version(context, &config.kotlin_binary).await?,
                            )?;
                            Some(Ok(kotlin_version))
                        }
                        _ => None,
                    }
                }
            })
            .await
            .parse(None),
        Err(e) => Err(e),
    };

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `kotlin`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

async fn get_kotlin_version(context: &Context<'_>, kotlin_binary: &str) -> Option<String> {
    let output = context.async_exec_cmd(kotlin_binary, &["-version"]).await?;
    if output.stdout.is_empty() {
        Some(output.stderr)
    } else {
        Some(output.stdout)
    }
}

fn format_kotlin_version(kotlin_stdout: &str) -> Option<String> {
    // kotlin -version output looks like this:
    // Kotlin version 1.4.21-release-411 (JRE 14.0.1+7)

    // kotlinc -version output looks like this:
    // info: kotlinc-jvm 1.4.21 (JRE 14.0.1+7)
    let re = Regex::new(KOTLIN_VERSION_PATTERN).ok()?;
    let captures = re.captures(kotlin_stdout)?;
    let version = &captures["version"];
    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_kotlin_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("kotlin").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_kotlin_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.kt"))?.sync_all()?;
        let actual = ModuleRenderer::new("kotlin").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🅺 v1.4.21 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_kotlin_script_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.kts"))?.sync_all()?;
        let actual = ModuleRenderer::new("kotlin").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🅺 v1.4.21 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn kotlin_binary_is_kotlin_runtime() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.kt"))?.sync_all()?;

        let config = toml::toml! {
             [kotlin]
             kotlin_binary = "kotlin"
        };

        let actual = ModuleRenderer::new("kotlin")
            .path(dir.path())
            .config(config)
            .collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint("🅺 v1.4.21 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn kotlin_binary_is_kotlin_compiler() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.kt"))?.sync_all()?;

        let config = toml::toml! {
             [kotlin]
             kotlin_binary = "kotlinc"
        };

        let actual = ModuleRenderer::new("kotlin")
            .path(dir.path())
            .config(config)
            .collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint("🅺 v1.4.21 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_kotlin_version_from_runtime() {
        let kotlin_input = "Kotlin version 1.4.21-release-411 (JRE 14.0.1+7)";
        assert_eq!(
            format_kotlin_version(kotlin_input),
            Some("v1.4.21".to_string())
        );
    }

    #[test]
    fn test_format_kotlin_version_from_compiler() {
        let kotlin_input = "info: kotlinc-jvm 1.4.21 (JRE 14.0.1+7)";
        assert_eq!(
            format_kotlin_version(kotlin_input),
            Some("v1.4.21".to_string())
        );
    }
}
