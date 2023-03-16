use super::{Context, Module, ModuleConfig};
use crate::configs::package::PackageConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

use ini::Ini;
use quick_xml::events::Event as QXEvent;
use quick_xml::Reader as QXReader;
use regex::Regex;
use serde_json as json;
use std::fs;
use std::io::Read;

/// Creates a module with the current package version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("package");
    let config: PackageConfig = PackageConfig::try_load(module.config);
    let module_version = get_version(context, &config)?;

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
                "version" => Some(Ok(&module_version)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `package`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_node_package_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("package.json")?;
    let package_json: json::Value = json::from_str(&file_contents).ok()?;

    if !config.display_private
        && package_json.get("private").and_then(json::Value::as_bool) == Some(true)
    {
        return None;
    }

    let raw_version = package_json.get("version")?.as_str()?;
    if raw_version == "null" {
        return None;
    };

    let formatted_version = format_version(raw_version, config.version_format)?;
    if formatted_version == "v0.0.0-development" || formatted_version.starts_with("v0.0.0-semantic")
    {
        return Some("semantic".to_string());
    };

    Some(formatted_version)
}

fn get_poetry_version(pyproject: &toml::Table) -> Option<&str> {
    pyproject
        .get("tool")?
        .get("poetry")?
        .get("version")?
        .as_str()
}

fn get_pep621_version(pyproject: &toml::Table) -> Option<&str> {
    pyproject.get("project")?.get("version")?.as_str()
}

fn get_pyproject_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("pyproject.toml")?;
    let pyproject_toml: toml::Table = toml::from_str(&file_contents).ok()?;

    get_pep621_version(&pyproject_toml)
        .or_else(|| get_poetry_version(&pyproject_toml))
        .and_then(|raw_version| format_version(raw_version, config.version_format))
}

fn get_setup_cfg_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("setup.cfg")?;
    let ini = Ini::load_from_str(&file_contents).ok()?;
    let raw_version = ini.get_from(Some("metadata"), "version")?;

    if raw_version.starts_with("attr:") || raw_version.starts_with("file:") {
        None
    } else {
        format_version(raw_version, config.version_format)
    }
}

fn get_gradle_version(context: &Context, config: &PackageConfig) -> Option<String> {
    context
        .read_file_from_pwd("gradle.properties")
        .and_then(|contents| {
            let re = Regex::new(r"(?m)^\s*version\s*=\s*(?P<version>.*)").unwrap();
            let caps = re.captures(&contents)?;
            format_version(&caps["version"], config.version_format)
        }).or_else(|| {
            let build_file_contents = context.read_file_from_pwd("build.gradle")?;
            let re = Regex::new(r#"(?m)^version ['"](?P<version>[^'"]+)['"]$"#).unwrap(); /*dark magic*/
            let caps = re.captures(&build_file_contents)?;
            format_version(&caps["version"], config.version_format)

        })
}

fn get_composer_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("composer.json")?;
    let composer_json: json::Value = json::from_str(&file_contents).ok()?;
    let raw_version = composer_json.get("version")?.as_str()?;

    format_version(raw_version, config.version_format)
}

fn get_julia_project_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("Project.toml")?;
    let project_toml: toml::Table = toml::from_str(&file_contents).ok()?;
    let raw_version = project_toml.get("version")?.as_str()?;

    format_version(raw_version, config.version_format)
}

fn get_helm_package_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("Chart.yaml")?;
    let yaml = yaml_rust::YamlLoader::load_from_str(&file_contents).ok()?;
    let version = yaml.first()?["version"].as_str()?;

    format_version(version, config.version_format)
}

fn get_mix_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("mix.exs")?;
    let re = Regex::new(r#"(?m)version: "(?P<version>[^"]+)""#).unwrap();
    let caps = re.captures(&file_contents)?;

    format_version(&caps["version"], config.version_format)
}

fn get_maven_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("pom.xml")?;
    let mut reader = QXReader::from_str(&file_contents);
    reader.trim_text(true);

    let mut buf = vec![];
    let mut in_ver = false;
    let mut depth = 0;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(QXEvent::Start(ref e)) => {
                in_ver = depth == 1 && e.name().as_ref() == b"version";
                depth += 1;
            }
            Ok(QXEvent::End(_)) => {
                in_ver = false;
                depth -= 1;
            }
            Ok(QXEvent::Text(t)) if in_ver => {
                let ver = t.unescape().ok().map(std::borrow::Cow::into_owned);
                return match ver {
                    // Ignore version which is just a property reference
                    Some(ref v) if !v.starts_with('$') => format_version(v, config.version_format),
                    _ => None,
                };
            }
            Ok(QXEvent::Eof) => break,
            Ok(_) => (),

            Err(err) => {
                log::warn!("Error parsing pom.xml`:\n{}", err);
                break;
            }
        }
    }

    None
}

fn get_meson_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context
        .read_file_from_pwd("meson.build")?
        .split_ascii_whitespace()
        .collect::<String>();

    let re = Regex::new(r#"project\([^())]*,version:'(?P<version>[^']+)'[^())]*\)"#).unwrap();
    let caps = re.captures(&file_contents)?;

    format_version(&caps["version"], config.version_format)
}

fn get_vmod_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("v.mod")?;
    let re = Regex::new(r"(?m)^\s*version\s*:\s*'(?P<version>[^']+)'").unwrap();
    let caps = re.captures(&file_contents)?;
    format_version(&caps["version"], config.version_format)
}

fn get_vpkg_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("vpkg.json")?;
    let vpkg_json: json::Value = json::from_str(&file_contents).ok()?;
    let raw_version = vpkg_json.get("version")?.as_str()?;

    format_version(raw_version, config.version_format)
}

fn get_sbt_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("build.sbt")?;
    let re = Regex::new(r"(?m)^(.*/)*\s*version\s*:=\s*.(?P<version>[\d\.]+)").unwrap();
    let caps = re.captures(&file_contents)?;
    format_version(&caps["version"], config.version_format)
}

fn get_cargo_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let mut file_contents = context.read_file_from_pwd("Cargo.toml")?;

    let mut cargo_toml: toml::Table = toml::from_str(&file_contents).ok()?;
    let cargo_version = cargo_toml.get("package").and_then(|p| p.get("version"));
    let raw_version = if let Some(v) = cargo_version.and_then(toml::Value::as_str) {
        // regular version string
        v
    } else if cargo_version
        .and_then(|v| v.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or_default()
    {
        // workspace version string (`package.version.worspace = true`)
        // need to read the Cargo.toml file from the workspace root
        let mut version = None;
        // disover the workspace root
        for path in context.current_dir.ancestors().skip(1) {
            // Assume the workspace root is the first ancestor that contains a Cargo.toml file
            if let Ok(mut file) = fs::File::open(path.join("Cargo.toml")) {
                file.read_to_string(&mut file_contents).ok()?;
                cargo_toml = toml::from_str(&file_contents).ok()?;
                // Read workspace.package.version
                version = cargo_toml
                    .get("workspace")?
                    .get("package")?
                    .get("version")?
                    .as_str();
                break;
            }
        }
        version?
    } else {
        // This might be a workspace file
        cargo_toml
            .get("workspace")?
            .get("package")?
            .get("version")?
            .as_str()?
    };

    format_version(raw_version, config.version_format)
}

fn get_nimble_version(context: &Context, config: &PackageConfig) -> Option<String> {
    if !context
        .try_begin_scan()?
        .set_extensions(&["nimble"])
        .is_match()
    {
        return None;
    };

    let cmd_output = context.exec_cmd("nimble", &["dump", "--json"])?;
    let nimble_json: json::Value = json::from_str(&cmd_output.stdout).ok()?;

    let raw_version = nimble_json.get("version")?.as_str()?;

    format_version(raw_version, config.version_format)
}

fn get_shard_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("shard.yml")?;

    let data = yaml_rust::YamlLoader::load_from_str(&file_contents).ok()?;
    let raw_version = data.first()?["version"].as_str()?;

    format_version(raw_version, config.version_format)
}

fn get_daml_project_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("daml.yaml")?;

    let daml_yaml = yaml_rust::YamlLoader::load_from_str(&file_contents).ok()?;
    let raw_version = daml_yaml.first()?["version"].as_str()?;

    format_version(raw_version, config.version_format)
}

fn get_dart_pub_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let file_contents = context.read_file_from_pwd("pubspec.yaml")?;

    let data = yaml_rust::YamlLoader::load_from_str(&file_contents).ok()?;
    let raw_version = data.first()?["version"].as_str()?;

    format_version(raw_version, config.version_format)
}

fn get_version(context: &Context, config: &PackageConfig) -> Option<String> {
    let package_version_fn: Vec<fn(&Context, &PackageConfig) -> Option<String>> = vec![
        get_cargo_version,
        get_nimble_version,
        get_node_package_version,
        get_pyproject_version,
        get_setup_cfg_version,
        get_composer_version,
        get_gradle_version,
        get_julia_project_version,
        get_mix_version,
        get_helm_package_version,
        get_maven_version,
        get_meson_version,
        get_shard_version,
        get_vmod_version,
        get_vpkg_version,
        get_sbt_version,
        get_daml_project_version,
        get_dart_pub_version,
    ];

    package_version_fn.iter().find_map(|f| f(context, config))
}

fn format_version(version: &str, version_format: &str) -> Option<String> {
    let cleaned = version
        .replace('"', "")
        .trim()
        .trim_start_matches('v')
        .to_string();

    VersionFormatter::format_module_version("package", &cleaned, version_format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_format_version() {
        let raw_expected = Some(String::from("v1.2.3"));

        assert_eq!(format_version("1.2.3", "v${raw}"), raw_expected);
        assert_eq!(format_version(" 1.2.3 ", "v${raw}"), raw_expected);
        assert_eq!(format_version("1.2.3 ", "v${raw}"), raw_expected);
        assert_eq!(format_version(" 1.2.3", "v${raw}"), raw_expected);
        assert_eq!(format_version("\"1.2.3\"", "v${raw}"), raw_expected);

        assert_eq!(format_version("v1.2.3", "v${raw}"), raw_expected);
        assert_eq!(format_version(" v1.2.3 ", "v${raw}"), raw_expected);
        assert_eq!(format_version(" v1.2.3", "v${raw}"), raw_expected);
        assert_eq!(format_version("v1.2.3 ", "v${raw}"), raw_expected);
        assert_eq!(format_version("\"v1.2.3\"", "v${raw}"), raw_expected);

        let major_expected = Some(String::from("v1"));
        assert_eq!(format_version("1.2.3", "v${major}"), major_expected);
        assert_eq!(format_version(" 1.2.3 ", "v${major}"), major_expected);
        assert_eq!(format_version("1.2.3 ", "v${major}"), major_expected);
        assert_eq!(format_version(" 1.2.3", "v${major}"), major_expected);
        assert_eq!(format_version("\"1.2.3\"", "v${major}"), major_expected);

        assert_eq!(format_version("v1.2.3", "v${major}"), major_expected);
        assert_eq!(format_version(" v1.2.3 ", "v${major}"), major_expected);
        assert_eq!(format_version(" v1.2.3", "v${major}"), major_expected);
        assert_eq!(format_version("v1.2.3 ", "v${major}"), major_expected);
        assert_eq!(format_version("\"v1.2.3\"", "v${major}"), major_expected);
    }

    #[test]
    fn test_extract_cargo_version() -> io::Result<()> {
        let config_name = "Cargo.toml";
        let config_content = toml::toml! {
            [package]
            name = "starship"
            version = "0.1.0"
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_cargo_version_ws() -> io::Result<()> {
        let ws_config_name = "Cargo.toml";
        let ws_config_content = toml::toml! {
            [workspace.package]
            version = "0.1.0"
        }
        .to_string();

        let config_name = "member/Cargo.toml";
        let config_content = toml::toml! {
            [package]
            version.workspace = true
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fs::create_dir(project_dir.path().join("member"))?;

        fill_config(&project_dir, ws_config_name, Some(&ws_config_content))?;
        fill_config(&project_dir, config_name, Some(&config_content))?;

        // Version can be read both from the workspace and the member.
        expect_output(&project_dir, Some("v0.1.0"), None);
        let actual = ModuleRenderer::new("package")
            .path(project_dir.path().join("member"))
            .collect();
        let expected = Some(format!(
            "is {} ",
            Color::Fixed(208).bold().paint("📦 v0.1.0")
        ));
        assert_eq!(actual, expected);
        project_dir.close()
    }

    #[test]
    fn test_extract_cargo_version_ws_false() -> io::Result<()> {
        let ws_config_name = "Cargo.toml";
        let ws_config_content = toml::toml! {
            [workspace.package]
            version = "0.1.0"
        }
        .to_string();

        let config_name = "member/Cargo.toml";
        let config_content = toml::toml! {
            [package]
            version.workspace = false
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fs::create_dir(project_dir.path().join("member"))?;

        fill_config(&project_dir, ws_config_name, Some(&ws_config_content))?;
        fill_config(&project_dir, config_name, Some(&config_content))?;

        // Version can be read both from the workspace and the member.
        let actual = ModuleRenderer::new("package")
            .path(project_dir.path().join("member"))
            .collect();
        let expected = None;
        assert_eq!(actual, expected);
        project_dir.close()
    }

    #[test]
    fn test_extract_cargo_version_ws_missing_parent() -> io::Result<()> {
        let config_name = "Cargo.toml";
        let config_content = toml::toml! {
            [package]
            name = "starship"
            version.workspace = true
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_nimble_package_version() -> io::Result<()> {
        let config_name = "test_project.nimble";

        let config_content = r##"
version = "0.1.0"
author = "Mr. nimble"
description = "A new awesome nimble package"
license = "MIT"
"##;

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;

        let starship_config = toml::toml! {
            [package]
            disabled = false
        };
        let actual = ModuleRenderer::new("package")
            .cmd(
                "nimble dump --json",
                Some(CommandOutput {
                    stdout: r##"
{
  "name": "test_project.nimble",
  "version": "0.1.0",
  "author": "Mr. nimble",
  "desc": "A new awesome nimble package",
  "license": "MIT",
  "skipDirs": [],
  "skipFiles": [],
  "skipExt": [],
  "installDirs": [],
  "installFiles": [],
  "installExt": [],
  "requires": [],
  "bin": [],
  "binDir": "",
  "srcDir": "",
  "backend": "c"
}
"##
                    .to_owned(),
                    stderr: String::new(),
                }),
            )
            .path(project_dir.path())
            .config(starship_config)
            .collect();

        let expected = Some(format!(
            "is {} ",
            Color::Fixed(208).bold().paint(format!("📦 {}", "v0.1.0"))
        ));

        assert_eq!(actual, expected);
        project_dir.close()
    }

    #[test]
    fn test_extract_nimble_package_version_for_nimble_directory_when_nimble_is_not_available(
    ) -> io::Result<()> {
        let config_name = "test_project.nimble";

        let config_content = r##"
version = "0.1.0"
author = "Mr. nimble"
description = "A new awesome nimble package"
license = "MIT"
"##;

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;

        let starship_config = toml::toml! {
            [package]
            disabled = false
        };
        let actual = ModuleRenderer::new("package")
            .cmd("nimble dump --json", None)
            .path(project_dir.path())
            .config(starship_config)
            .collect();

        let expected = None;

        assert_eq!(actual, expected);
        project_dir.close()
    }

    #[test]
    fn test_extract_nimble_package_version_for_non_nimble_directory() -> io::Result<()> {
        // Only create an empty directory. There's no .nibmle file for this case.
        let project_dir = create_project_dir()?;

        let starship_config = toml::toml! {
            [package]
            disabled = false
        };
        let actual = ModuleRenderer::new("package")
            .cmd("nimble dump --json", None)
            .path(project_dir.path())
            .config(starship_config)
            .collect();

        let expected = None;

        assert_eq!(actual, expected);
        project_dir.close()
    }

    #[test]
    fn test_extract_package_version() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "0.1.0"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_package_version_without_version() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_package_version_with_null_version() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": null
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_package_version_with_null_string_version() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "null"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_private_package_version_with_default_config() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "0.1.0",
            "private": true
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_private_package_version_with_display_private() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "0.1.0",
            "private": true
        })
        .to_string();
        let starship_config = toml::toml! {
            [package]
            display_private = true
        };

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), Some(starship_config));
        project_dir.close()
    }

    #[test]
    fn test_node_package_version_semantic_development_version() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "0.0.0-development"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("semantic"), None);
        project_dir.close()
    }

    #[test]
    fn test_node_package_version_with_semantic_other_version() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "v0.0.0-semantically-released"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("semantic"), None);
        project_dir.close()
    }

    #[test]
    fn test_crystal_shard_version() -> io::Result<()> {
        let config_name = "shard.yml";
        let config_content = "name: starship\nversion: 1.2.3\n".to_string();

        let project_dir = create_project_dir()?;

        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v1.2.3"), None);

        project_dir.close()
    }

    #[test]
    fn test_node_package_version_with_non_semantic_tag() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "v0.0.0-alpha"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.0.0-alpha"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_poetry_version() -> io::Result<()> {
        let config_name = "pyproject.toml";
        let config_content = toml::toml! {
            [tool.poetry]
            name = "starship"
            version = "0.1.0"
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_poetry_version_without_version() -> io::Result<()> {
        let config_name = "pyproject.toml";
        let config_content = toml::toml! {
            [tool.poetry]
            name = "starship"
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_pep621_version() -> io::Result<()> {
        let config_name = "pyproject.toml";
        let config_content = toml::toml! {
            [project]
            name = "starship"
            version = "0.1.0"
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_pep621_version_without_version() -> io::Result<()> {
        let config_name = "pyproject.toml";
        let config_content = toml::toml! {
            [project]
            name = "starship"
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_pep621_version_attr_directive() -> io::Result<()> {
        let config_name = "pyproject.toml";
        let config_content = toml::toml! {
            [project]
            name = "starship"
            version = {attr = "starship.__version__"}
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_pep621_version_file_directive() -> io::Result<()> {
        let config_name = "pyproject.toml";
        let config_content = toml::toml! {
            [project]
            name = "starship"
            version = {file = "VERSION.txt"}
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_setup_cfg_version() -> io::Result<()> {
        let config_name = "setup.cfg";
        let config_content = String::from(
            "[metadata]
            version = 0.1.0",
        );

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_setup_cfg_version_without_version() -> io::Result<()> {
        let config_name = "setup.cfg";
        let config_content = String::from("[metadata]");

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_setup_cfg_version_attr() -> io::Result<()> {
        let config_name = "setup.cfg";
        let config_content = String::from(
            "[metadata]
            version = attr: mymod.__version__",
        );

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_setup_cfg_version_file() -> io::Result<()> {
        let config_name = "setup.cfg";
        let config_content = String::from(
            "[metadata]
            version = file: version.txt",
        );

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_gradle_version_single_quote() -> io::Result<()> {
        let config_name = "build.gradle";
        let config_content = "plugins {
    id 'java'
    id 'test.plugin' version '0.2.0'
}
version '0.1.0'
java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_gradle_version_double_quote() -> io::Result<()> {
        let config_name = "build.gradle";
        let config_content = "plugins {
    id 'java'
    id 'test.plugin' version '0.2.0'
}
version \"0.1.0\"
java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_gradle_version_rc_version() -> io::Result<()> {
        let config_name = "build.gradle";
        let config_content = "plugins {
    id 'java'
    id 'test.plugin' version '0.2.0'
}
version '0.1.0-rc1'
java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v0.1.0-rc1"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_gradle_version_without_version() -> io::Result<()> {
        let config_name = "build.gradle";
        let config_content = "plugins {
    id 'java'
    id 'test.plugin' version '0.2.0'
}
java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }
    #[test]
    fn test_extract_grade_version_from_properties() -> io::Result<()> {
        let config_name = "gradle.properties";
        let config_content = "
            version=1.2.3
            ";
        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v1.2.3"), None);
        project_dir.close()
    }
    #[test]
    fn test_extract_grade_version_from_properties_with_comment_and_whitespace() -> io::Result<()> {
        let config_name = "gradle.properties";
        let config_content = "
            # or use -Pversion=0.0.1
            version = 1.2.3
            ";
        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v1.2.3"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_mix_version() -> io::Result<()> {
        let config_name = "mix.exs";
        let config_content = "defmodule MyApp.MixProject do
  use Mix.Project

  def project do
    [
      app: :my_app,
      version: \"1.2.3\",
      elixir: \"~> 1.10\",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run \"mix help compile.app\" to learn about applications.
  def application do
    [extra_applications: [:logger]]
  end

  # Run \"mix help deps\" to learn about dependencies.
  defp deps do
    []
  end
end";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v1.2.3"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_mix_version_partial_online() -> io::Result<()> {
        let config_name = "mix.exs";
        let config_content = "  def project, do: [app: :my_app,version: \"3.2.1\"]";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v3.2.1"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_mix_version_rc_version() -> io::Result<()> {
        let config_name = "mix.exs";
        let config_content = "  def project do
    [
      app: :my_app,
      version: \"1.0.0-alpha.3\"
    ]
  end";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v1.0.0-alpha.3"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_mix_version_rc_with_build_version() -> io::Result<()> {
        let config_name = "mix.exs";
        let config_content = "  def project do
    [
      app: :my_app,
      version: \"0.9.9-dev+20130417140000.amd64\"
    ]
  end";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v0.9.9-dev+20130417140000.amd64"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_helm_chart_version() -> io::Result<()> {
        let config_name = "Chart.yaml";
        let config_content = "
        apiVersion: v1
        name: starship
        version: 0.2.0
        ";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v0.2.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_composer_version() -> io::Result<()> {
        let config_name = "composer.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "0.1.0"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_composer_version_without_version() -> io::Result<()> {
        let config_name = "composer.json";
        let config_content = json::json!({
            "name": "starship"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_julia_project_version() -> io::Result<()> {
        let config_name = "Project.toml";
        let config_content = toml::toml! {
            name = "starship"
            version = "0.1.0"
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_julia_project_version_without_version() -> io::Result<()> {
        let config_name = "Project.toml";
        let config_content = toml::toml! {
            name = "starship"
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_maven_version_with_deps() -> io::Result<()> {
        // pom.xml with common nested tags and dependencies
        let pom = "
            <project xmlns=\"http://maven.apache.org/POM/4.0.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4_0_0.xsd\">

              <modelVersion>4.0.0</modelVersion>
              <artifactId>parent</artifactId>
              <packaging>pom</packaging>

              <version>0.3.20-SNAPSHOT</version>

              <name>Test POM</name>
              <description>Test POM</description>

              <properties>
                <jdk.version>1.8</jdk.version>
                <jta.version>2.3.3</jta.version>
                <woodstox.version>4.13</woodstox.version>
                <jackson.version>3.3.3</jackson.version>
              </properties>

              <dependencyManagement>
                  <dependencies>
                      <dependency>
                          <groupId>jta</groupId>
                          <artifactId>jta</artifactId>
                          <version>${jta.version}</version>
                      </dependency>
                      <dependency>
                          <groupId>com.fasterxml.woodstox</groupId>
                          <artifactId>woodstox-core</artifactId>
                          <version>${woodstox.core.version}</version>
                      </dependency>
                      <dependency>
                          <groupId>com.fasterxml.jackson.dataformat</groupId>
                          <artifactId>jackson-dataformat-xml</artifactId>
                          <version>${jackson.version}</version>
                      </dependency>
                  </dependencies>
              </dependencyManagement>

              <build>
                <plugins>
                  <plugin>
                    <artifactId>maven-enforcer-plugin</artifactId>
                    <version>${maven.enforcer.version}</version>
                    <executions>
                      <execution>
                        <id>enforce-maven</id>
                        <goals>
                          <goal>enforce</goal>
                        </goals>
                        <configuration>
                          <rules>
                            <requireMavenVersion>
                              <version>3.0.5</version>
                            </requireMavenVersion>
                          </rules>
                        </configuration>
                      </execution>
                    </executions>
                  </plugin>
                </plugins>
              </build>

            </project>";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, "pom.xml", Some(pom))?;
        expect_output(&project_dir, Some("v0.3.20-SNAPSHOT"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_maven_version_no_version() -> io::Result<()> {
        // pom.xml with common nested tags and dependencies
        let pom = "
            <project xmlns=\"http://maven.apache.org/POM/4.0.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4_0_0.xsd\">

              <modelVersion>4.0.0</modelVersion>

              <dependencies>
                  <dependency>
                      <groupId>jta</groupId>
                      <artifactId>jta</artifactId>
                      <version>1.2.3</version>
                  </dependency>
              </dependencies>

            </project>";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, "pom.xml", Some(pom))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_maven_version_is_prop() -> io::Result<()> {
        // pom.xml with common nested tags and dependencies
        let pom = "
            <project xmlns=\"http://maven.apache.org/POM/4.0.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4_0_0.xsd\">

              <modelVersion>4.0.0</modelVersion>
              <version>${pom.parent.version}</version>

            </project>";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, "pom.xml", Some(pom))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_maven_version_no_version_but_deps() -> io::Result<()> {
        // pom.xml with common nested tags and dependencies
        let pom = "
            <project xmlns=\"http://maven.apache.org/POM/4.0.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4_0_0.xsd\">

              <modelVersion>4.0.0</modelVersion>
              <artifactId>parent</artifactId>
              <packaging>pom</packaging>

              <name>Test POM</name>
              <description>Test POM</description>

            </project>";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, "pom.xml", Some(pom))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_meson_version() -> io::Result<()> {
        let config_name = "meson.build";
        let config_content = "project('starship', 'rust', version: '0.1.0')".to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_meson_version_without_version() -> io::Result<()> {
        let config_name = "meson.build";
        let config_content = "project('starship', 'rust')".to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None);
        project_dir.close()
    }

    #[test]
    fn test_extract_meson_version_with_meson_version() -> io::Result<()> {
        let config_name = "meson.build";
        let config_content =
            "project('starship', 'rust', version: '0.1.0', meson_version: '>= 0.57.0')".to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_vmod_version() -> io::Result<()> {
        let config_name = "v.mod";
        let config_content = "\
Module {
    name: 'starship',
    author: 'matchai',
    version: '1.2.3'
}";
        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v1.2.3"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_vpkg_version() -> io::Result<()> {
        let config_name = "vpkg.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "0.1.0"
        })
        .to_string();
        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_sbt_version() -> io::Result<()> {
        let config_name = "build.sbt";
        let config_content = "\
name := \"starship\"
version := \"1.2.3\"
scalaVersion := \"2.13.7\"

assembly / assemblyMergeStrategy := {
  case PathList(\"META-INF\", _ @ _*) => MergeStrategy.discard
  case _ => MergeStrategy.first
}
";
        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v1.2.3"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_sbt_version_thisbuild() -> io::Result<()> {
        let config_name = "build.sbt";
        let config_content = "\
name := \"starship\"
ThisBuild / version := \"1.2.3\"
scalaVersion := \"2.13.7\"
";
        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v1.2.3"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_daml_project_version() -> io::Result<()> {
        let config_name = "daml.yaml";
        let config_content = "
sdk-version: 2.2.0
version: 6.8.65
";
        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v6.8.65"), None);
        project_dir.close()
    }

    #[test]
    fn test_extract_dart_pub_version() -> io::Result<()> {
        let config_name = "pubspec.yaml";
        let config_content = "
name: starship
version: 1.0.0

environment:
  sdk: '>=2.15.0 <3.0.0'
";
        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(config_content))?;
        expect_output(&project_dir, Some("v1.0.0"), None);
        project_dir.close()
    }

    fn create_project_dir() -> io::Result<TempDir> {
        tempfile::tempdir()
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

    fn expect_output(project_dir: &TempDir, contains: Option<&str>, config: Option<toml::Table>) {
        let starship_config = config.unwrap_or(toml::toml! {
            [package]
            disabled = false
        });

        let actual = ModuleRenderer::new("package")
            .path(project_dir.path())
            .config(starship_config)
            .collect();
        let text = String::from(contains.unwrap_or(""));
        let expected = Some(format!(
            "is {} ",
            Color::Fixed(208).bold().paint(format!("📦 {text}"))
        ));

        if contains.is_some() {
            assert_eq!(actual, expected);
        } else {
            assert_eq!(actual, None);
        }
    }
}
