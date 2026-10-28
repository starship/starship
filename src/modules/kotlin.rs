use super::{Context, Module, ModuleConfig};
use crate::formatter::VariableHolder;

use crate::configs::kotlin::KotlinConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils::get_command_string_output;

use regex::Regex;
const KOTLIN_VERSION_PATTERN: &str = "(?P<version>[\\d\\.]+[\\d\\.]+[\\d\\.]+)";

/// Creates a module with the current Kotlin version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("kotlin");
    let config = KotlinConfig::try_load(module.config);

    let formatter = StringFormatter::new(config.format);
    let wants_local = formatter
        .as_ref()
        .is_ok_and(|formatter| formatter.get_variables().contains("local_version"));
    let local_project = wants_local && context.jvm_versions().kotlin.is_some();

    let is_kotlin_project = local_project
        || context
            .try_begin_scan()?
            .set_files(&config.detect_files)
            .set_extensions(&config.detect_extensions)
            .set_folders(&config.detect_folders)
            .is_match();

    if !is_kotlin_project {
        return None;
    }

    let parsed = formatter.and_then(|formatter| {
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
                    let kotlin_version = get_kotlin_version(context, config.kotlin_binary)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &kotlin_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                "local_version" => {
                    let kotlin_version = context.jvm_versions().kotlin.as_deref()?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        kotlin_version,
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
            log::warn!("Error in module `kotlin`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_kotlin_version(context: &Context, kotlin_binary: &str) -> Option<String> {
    let command = context.exec_cmd(kotlin_binary, &["-version"])?;
    let kotlin_version_string = get_command_string_output(command);

    parse_kotlin_version(&kotlin_version_string)
}

fn parse_kotlin_version(kotlin_stdout: &str) -> Option<String> {
    // kotlin -version output looks like this:
    // Kotlin version 1.4.21-release-411 (JRE 14.0.1+7)
    // kotlinc -version output looks like this:
    // info: kotlinc-jvm 1.4.21 (JRE 14.0.1+7)
    let re = Regex::new(KOTLIN_VERSION_PATTERN).ok()?;
    let captures = re.captures(kotlin_stdout)?;
    let version = &captures["version"];
    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn local_and_installed_versions_remain_distinct() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let build = dir.path().join("build.gradle.kts");
        std::fs::write(&build, r#"plugins { kotlin("jvm") version "2.1.20" }"#)?;
        File::open(&build)?.sync_all()?;
        let actual = ModuleRenderer::new("kotlin")
            .path(dir.path())
            .config(toml::toml! { [kotlin] format = "${local_version}|${version}" })
            .collect();
        assert_eq!(actual.as_deref(), Some("v2.1.20|v1.4.21"));
        dir.close()
    }

    #[test]
    fn unresolved_local_version_does_not_fall_back_to_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let build = dir.path().join("build.gradle.kts");
        std::fs::write(&build, "// No declared compiler version")?;
        File::open(&build)?.sync_all()?;
        let actual = ModuleRenderer::new("kotlin")
            .path(dir.path())
            .config(toml::toml! { [kotlin] format = "$local_version" })
            .collect();
        assert_eq!(actual, None);
        dir.close()
    }

    #[test]
    fn local_version_uses_project_declaration_without_installed_compiler() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        std::fs::write(
            dir.path().join("build.gradle.kts"),
            r#"plugins { kotlin("jvm") version "2.1.20" }"#,
        )?;
        File::open(dir.path().join("build.gradle.kts"))?.sync_all()?;
        let actual = ModuleRenderer::new("kotlin")
            .path(dir.path())
            .config(toml::toml! { [kotlin] format = "$local_version" })
            .cmd("kotlin -version", None)
            .cmd("java -Xinternalversion", None)
            .collect();
        assert_eq!(actual.as_deref(), Some("v2.1.20"));
        dir.close()
    }

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
    fn test_parse_kotlin_version_from_runtime() {
        let kotlin_input = "Kotlin version 1.4.21-release-411 (JRE 14.0.1+7)";
        assert_eq!(
            parse_kotlin_version(kotlin_input),
            Some("1.4.21".to_string())
        );
    }

    #[test]
    fn test_parse_kotlin_version_from_compiler() {
        let kotlin_input = "info: kotlinc-jvm 1.4.21 (JRE 14.0.1+7)";
        assert_eq!(
            parse_kotlin_version(kotlin_input),
            Some("1.4.21".to_string())
        );
    }
}
