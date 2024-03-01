use super::{Context, Module, ModuleConfig};
use crate::configs::clojure::ClojureConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use crate::utils::get_command_string_output;

use regex::Regex;
const CLOJURE_VERSION_PATTERN: &str = "Clojure CLI version (?P<version>\\d+(?:\\.\\d+){0,2})";

/// Creates a module with the current Clojure version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("clojure");
    let config: ClojureConfig = ClojureConfig::try_load(module.config);

    let is_clojure_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_clojure_project {
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
                    let clojure_version = get_clojure_version(context)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &clojure_version,
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
            log::warn!("Error in module `clojure`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_clojure_version(context: &Context) -> Option<String> {
    let clojure_command = String::from("clojure");

    let output = context.exec_cmd(clojure_command, &["--version"])?;
    let clojure_version_string = get_command_string_output(output);

    parse_clojure_version(&clojure_version_string)
}

fn parse_clojure_version(clojure_version_string: &str) -> Option<String> {
    let re = Regex::new(CLOJURE_VERSION_PATTERN).ok()?;
    let captures = re.captures(clojure_version_string)?;
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
    fn test_parse_clojure_version_four() {
        let clojure_11 = "Clojure CLI version 1.11.1";
        assert_eq!(
            parse_clojure_version(clojure_11),
            Some("1.11.1".to_string())
        );
    }

    #[test]
    fn test_parse_clojure_version_three() {
        let clojure_12 = "Clojure CLI version 1.12.0-alpha5";
        assert_eq!(
            parse_clojure_version(clojure_12),
            Some("1.12.0".to_string())
        );
    }

    #[test]
    fn folder_without_clojure_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("clojure").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_clojure_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("core.clj"))?.sync_all()?;
        let actual = ModuleRenderer::new("clojure").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("λ̮̮̑̑ v1.11.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_clojure_file_no_clojure_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("core.clj"))?.sync_all()?;
        let actual = ModuleRenderer::new("clojure")
            .cmd("clojure --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("λ̮̮̑̑ ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_lein_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("project.clj"))?.sync_all()?;
        let actual = ModuleRenderer::new("clojure").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("λ̮̮̑̑ v1.11.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_clojure_cli_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("deps.edn"))?.sync_all()?;
        let actual = ModuleRenderer::new("clojure").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("λ̮̮̑̑ v1.11.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_babashka_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("bb.edn"))?.sync_all()?;
        let actual = ModuleRenderer::new("clojure").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("λ̮̮̑̑ v1.11.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_shadow_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("shadow-cljs.edn"))?.sync_all()?;
        let actual = ModuleRenderer::new("clojure").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("λ̮̮̑̑ v1.11.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
