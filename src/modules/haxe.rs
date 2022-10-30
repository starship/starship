use super::{Context, Module, ModuleConfig};

use crate::configs::haxe::HaxeConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use serde_json as json;

use regex::Regex;
const HAXERC_VERSION_PATTERN: &str = "(?:[0-9a-zA-Z][-+0-9.a-zA-Z]+)";

/// Creates a module with the current Haxe version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("haxe");
    let config = HaxeConfig::try_load(module.config);

    let is_haxe_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_haxe_project {
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
                    let haxe_version = get_haxe_version(context)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &haxe_version,
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
            log::warn!("Error in module `haxe`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_haxe_version(context: &Context) -> Option<String> {
    get_haxerc_version(context).or_else(|| {
        let cmd_output = context.exec_cmd("haxe", &["--version"])?;
        parse_haxe_version(cmd_output.stdout.as_str())
    })
}

fn get_haxerc_version(context: &Context) -> Option<String> {
    let raw_json = context.read_file_from_pwd(".haxerc")?;
    let package_json: json::Value = json::from_str(&raw_json).ok()?;

    let raw_version = package_json.get("version")?.as_str()?;
    if raw_version.contains('/') || raw_version.contains('\\') {
        return None;
    }
    Some(raw_version.to_string())
}

fn parse_haxe_version(raw_version: &str) -> Option<String> {
    let re = Regex::new(HAXERC_VERSION_PATTERN).ok()?;
    if !re.is_match(raw_version)  {
        return None;
    }
    Some(raw_version.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_haxe_version;
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use serde_json as json;
    use std::fs::File;
    use std::io;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn haxe_version() {
        let ok_versions = [
            "4.2.5",
            "4.3.0-rc.1+",
            "3.4.7abcdf",
            "779b005",
            "beta",
            "alpha",
            "latest",
            "/git/779b005/bin/haxe",
            "git/779b005/bin/haxe",
        ];

        let all_some = ok_versions.iter().all(|&v| parse_haxe_version(v).is_some());

        assert!(all_some);

        let sample_haxe_output = "4.3.0-rc.1+\n";

        assert_eq!(
            Some("4.3.0-rc.1+".to_string()),
            parse_haxe_version(sample_haxe_output)
        )
    }

    #[test]
    fn folder_without_haxe() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("haxe.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("haxe")
            .cmd(
                "haxe --version",
                Some(CommandOutput {
                    stdout: "4.3.0-rc.1+\n".to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_hxml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("build.hxml"))?.sync_all()?;
        let actual = ModuleRenderer::new("haxe")
            .cmd(
                "haxe --version",
                Some(CommandOutput {
                    stdout: "4.3.0-rc.1+\n".to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("⌘ v4.3.0-rc.1+ ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_haxe_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.hx"))?.sync_all()?;
        let actual = ModuleRenderer::new("haxe")
            .cmd(
                "haxe --version",
                Some(CommandOutput {
                    stdout: "4.3.0-rc.1+\n".to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("⌘ v4.3.0-rc.1+ ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_invalid_haxerc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let haxerc_name = ".haxerc";
        let haxerc_content = json::json!({
            "resolveLibs": "scoped"
        })
        .to_string();
        fill_config(&dir, haxerc_name, Some(&haxerc_content))?;
        let actual = ModuleRenderer::new("haxe")
            .cmd(
                "haxe --version",
                Some(CommandOutput {
                    stdout: "4.3.0-rc.1+\n".to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("⌘ v4.3.0-rc.1+ ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_haxerc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let haxerc_name = ".haxerc";
        let haxerc_content = json::json!({
            "version": "4.2.5",
            "resolveLibs": "scoped"
        })
        .to_string();
        fill_config(&dir, haxerc_name, Some(&haxerc_content))?;
        let actual = ModuleRenderer::new("haxe")
            .cmd(
                "haxe --version",
                Some(CommandOutput {
                    stdout: "4.3.0-rc.1+\n".to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("⌘ v4.2.5 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_haxerc_nightly_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let haxerc_name = ".haxerc";
        let haxerc_content = json::json!({
            "version": "779b005",
            "resolveLibs": "scoped"
        })
        .to_string();

        fill_config(&dir, haxerc_name, Some(&haxerc_content))?;
        let actual = ModuleRenderer::new("haxe")
            .cmd(
                "haxe --version",
                Some(CommandOutput {
                    stdout: "4.3.0-rc.1+\n".to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("⌘ v779b005 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_haxerc_with_path() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let haxerc_name = ".haxerc";
        let haxerc_content = json::json!({
            "version": "/home/git/haxe/haxe.executable",
            "resolveLibs": "scoped"
        })
        .to_string();

        fill_config(&dir, haxerc_name, Some(&haxerc_content))?;
        let actual = ModuleRenderer::new("haxe")
            .cmd(
                "haxe --version",
                Some(CommandOutput {
                    stdout: "4.3.0-rc.1+\n".to_owned(),
                    stderr: "".to_owned(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("⌘ v4.3.0-rc.1+ ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    fn fill_config(
        project_dir: &TempDir,
        file_name: &str,
        contents: Option<&str>,
    ) -> io::Result<()> {
        let mut file = File::create(project_dir.path().join(file_name))?;
        file.write_all(contents.unwrap_or("").as_bytes())?;
        file.sync_all()
    }
}
