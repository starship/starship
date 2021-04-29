use std::path::Path;

use super::{Context, Module, RootModuleConfig};
use crate::configs::package::PackageConfig;
use crate::formatter::StringFormatter;
use crate::utils;

use quick_xml::events::Event as QXEvent;
use quick_xml::Reader as QXReader;
use regex::Regex;
use serde_json as json;

/// Creates a module with the current package version
///
/// Will display if a version is defined for your Node.js or Rust project (if one exists)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("package");
    let config: PackageConfig = PackageConfig::try_load(module.config);
    let module_version = get_package_version(&context.current_dir, &config)?;

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
            .parse(None)
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

fn extract_cargo_version(file_contents: &str) -> Option<String> {
    let cargo_toml: toml::Value = toml::from_str(file_contents).ok()?;
    let raw_version = cargo_toml.get("package")?.get("version")?.as_str()?;

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn extract_package_version(file_contents: &str, display_private: bool) -> Option<String> {
    let package_json: json::Value = json::from_str(file_contents).ok()?;

    if !display_private && package_json.get("private").and_then(json::Value::as_bool) == Some(true)
    {
        return None;
    }

    let raw_version = package_json.get("version")?.as_str()?;
    if raw_version == "null" {
        return None;
    };

    let formatted_version = format_version(raw_version);
    if formatted_version == "v0.0.0-development" || formatted_version.find("semantic").is_some()
    {
        return Some("semantic".to_string());
    };

    Some(formatted_version)
}

fn extract_poetry_version(file_contents: &str) -> Option<String> {
    let poetry_toml: toml::Value = toml::from_str(file_contents).ok()?;
    let raw_version = poetry_toml
        .get("tool")?
        .get("poetry")?
        .get("version")?
        .as_str()?;

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn extract_gradle_version(file_contents: &str) -> Option<String> {
    let re = Regex::new(r#"(?m)^version ['"](?P<version>[^'"]+)['"]$"#).unwrap();
    let caps = re.captures(file_contents)?;

    let formatted_version = format_version(&caps["version"]);
    Some(formatted_version)
}

fn extract_composer_version(file_contents: &str) -> Option<String> {
    let composer_json: json::Value = json::from_str(file_contents).ok()?;
    let raw_version = composer_json.get("version")?.as_str()?;
    if raw_version == "null" {
        return None;
    };

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn extract_project_version(file_contents: &str) -> Option<String> {
    let project_toml: toml::Value = toml::from_str(file_contents).ok()?;
    let raw_version = project_toml.get("version")?.as_str()?;

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn extract_helm_package_version(file_contents: &str) -> Option<String> {
    let yaml = yaml_rust::YamlLoader::load_from_str(file_contents).ok()?;
    let version = yaml.first()?["version"].as_str()?;
    Some(format_version(version))
}

fn extract_mix_version(file_contents: &str) -> Option<String> {
    let re = Regex::new(r#"(?m)version: "(?P<version>[^"]+)""#).unwrap();
    let caps = re.captures(file_contents)?;

    let formatted_version = format_version(&caps["version"]);
    Some(formatted_version)
}

fn extract_maven_version(file_contents: &str) -> Option<String> {
    let mut reader = QXReader::from_str(file_contents);
    reader.trim_text(true);

    let mut buf = vec![];
    let mut in_ver = false;
    let mut depth = 0;
    loop {
        match reader.read_event(&mut buf) {
            Ok(QXEvent::Start(ref e)) => {
                in_ver = depth == 1 && e.name() == b"version";
                depth += 1;
            }
            Ok(QXEvent::End(_)) => {
                in_ver = false;
                depth -= 1;
            }
            Ok(QXEvent::Text(t)) if in_ver => {
                let ver = t.unescape_and_decode(&reader).ok();
                return match ver {
                    // Ignore version which is just a property reference
                    Some(ref v) if !v.starts_with('$') => ver,
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

fn extract_meson_version(file_contents: &str) -> Option<String> {
    let file_contents = file_contents.split_ascii_whitespace().collect::<String>();

    let re = Regex::new(r#"project\([^())]*version:'(?P<version>[^']+)'[^())]*\)"#).unwrap();
    let caps = re.captures(&file_contents)?;

    let formatted_version = format_version(&caps["version"]);
    Some(formatted_version)
}

fn get_package_version(base_dir: &Path, config: &PackageConfig) -> Option<String> {
    if let Ok(cargo_toml) = utils::read_file(base_dir.join("Cargo.toml")) {
        extract_cargo_version(&cargo_toml)
    } else if let Ok(package_json) = utils::read_file(base_dir.join("package.json")) {
        extract_package_version(&package_json, config.display_private)
    } else if let Ok(poetry_toml) = utils::read_file(base_dir.join("pyproject.toml")) {
        extract_poetry_version(&poetry_toml)
    } else if let Ok(composer_json) = utils::read_file(base_dir.join("composer.json")) {
        extract_composer_version(&composer_json)
    } else if let Ok(build_gradle) = utils::read_file(base_dir.join("build.gradle")) {
        extract_gradle_version(&build_gradle)
    } else if let Ok(project_toml) = utils::read_file(base_dir.join("Project.toml")) {
        extract_project_version(&project_toml)
    } else if let Ok(mix_file) = utils::read_file(base_dir.join("mix.exs")) {
        extract_mix_version(&mix_file)
    } else if let Ok(chart_file) = utils::read_file(base_dir.join("Chart.yaml")) {
        extract_helm_package_version(&chart_file)
    } else if let Ok(pom_file) = utils::read_file(base_dir.join("pom.xml")) {
        extract_maven_version(&pom_file)
    } else if let Ok(meson_build) = utils::read_file(base_dir.join("meson.build")) {
        extract_meson_version(&meson_build)
    } else {
        None
    }
}

fn format_version(version: &str) -> String {
    let cleaned = version.replace('"', "").trim().to_string();
    if cleaned.starts_with('v') {
        cleaned
    } else {
        format!("v{}", cleaned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_format_version() {
        assert_eq!(format_version("0.1.0"), "v0.1.0");
        assert_eq!(format_version(" 0.1.0 "), "v0.1.0");
        assert_eq!(format_version("0.1.0 "), "v0.1.0");
        assert_eq!(format_version(" 0.1.0"), "v0.1.0");
        assert_eq!(format_version("\"0.1.0\""), "v0.1.0");

        assert_eq!(format_version("v0.1.0"), "v0.1.0");
        assert_eq!(format_version(" v0.1.0 "), "v0.1.0");
        assert_eq!(format_version(" v0.1.0"), "v0.1.0");
        assert_eq!(format_version("v0.1.0 "), "v0.1.0");
        assert_eq!(format_version("\"v0.1.0\""), "v0.1.0");
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
        expect_output(&project_dir, Some("v0.1.0"), None)?;
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
        expect_output(&project_dir, Some("v0.1.0"), None)?;
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
        expect_output(&project_dir, None, None)?;
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
        expect_output(&project_dir, None, None)?;
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
        expect_output(&project_dir, None, None)?;
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
        expect_output(&project_dir, None, None)?;
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
        expect_output(&project_dir, Some("v0.1.0"), Some(starship_config))?;
        project_dir.close()
    }

    #[test]
    fn test_package_version_semantic_development_version() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "0.0.0-development"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("semantic"), None)?;
        project_dir.close()
    }

    #[test]
    fn test_package_version_with_semantic_other_version() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "v0.0.0-semantically-released"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("semantic"), None)?;
        project_dir.close()
    }

    #[test]
    fn test_package_version_with_non_semantic_tag() -> io::Result<()> {
        let config_name = "package.json";
        let config_content = json::json!({
            "name": "starship",
            "version": "v0.0.0-alpha"
        })
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.0.0-alpha"), None)?;
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
        expect_output(&project_dir, Some("v0.1.0"), None)?;
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
        expect_output(&project_dir, None, None)?;
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
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None)?;
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
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None)?;
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
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0-rc1"), None)?;
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
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None)?;
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
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v1.2.3"), None)?;
        project_dir.close()
    }

    #[test]
    fn test_extract_mix_version_partial_online() -> io::Result<()> {
        let config_name = "mix.exs";
        let config_content = "  def project, do: [app: :my_app,version: \"3.2.1\"]";

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v3.2.1"), None)?;
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
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v1.0.0-alpha.3"), None)?;
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
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.9.9-dev+20130417140000.amd64"), None)?;
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
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.2.0"), None)?;
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
        expect_output(&project_dir, Some("v0.1.0"), None)?;
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
        expect_output(&project_dir, None, None)?;
        project_dir.close()
    }

    #[test]
    fn test_extract_project_version() -> io::Result<()> {
        let config_name = "Project.toml";
        let config_content = toml::toml! {
            name = "starship"
            version = "0.1.0"
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None)?;
        project_dir.close()
    }

    #[test]
    fn test_extract_project_version_without_version() -> io::Result<()> {
        let config_name = "Project.toml";
        let config_content = toml::toml! {
            name = "starship"
        }
        .to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None)?;
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
        fill_config(&project_dir, "pom.xml", Some(&pom))?;
        expect_output(&project_dir, Some("0.3.20-SNAPSHOT"), None)?;
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
        fill_config(&project_dir, "pom.xml", Some(&pom))?;
        expect_output(&project_dir, None, None)?;
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
        fill_config(&project_dir, "pom.xml", Some(&pom))?;
        expect_output(&project_dir, None, None)?;
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
        fill_config(&project_dir, "pom.xml", Some(&pom))?;
        expect_output(&project_dir, None, None)?;
        project_dir.close()
    }

    #[test]
    fn test_extract_meson_version() -> io::Result<()> {
        let config_name = "meson.build";
        let config_content = "project('starship', 'rust', version: '0.1.0')".to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, Some("v0.1.0"), None)?;
        project_dir.close()
    }

    #[test]
    fn test_extract_meson_version_without_version() -> io::Result<()> {
        let config_name = "meson.build";
        let config_content = "project('starship', 'rust')".to_string();

        let project_dir = create_project_dir()?;
        fill_config(&project_dir, config_name, Some(&config_content))?;
        expect_output(&project_dir, None, None)?;
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

    fn expect_output(
        project_dir: &TempDir,
        contains: Option<&str>,
        config: Option<toml::Value>,
    ) -> io::Result<()> {
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
            Color::Fixed(208).bold().paint(format!("📦 {}", text))
        ));

        if contains.is_some() {
            assert_eq!(actual, expected);
        } else {
            assert_eq!(actual, None);
        }

        Ok(())
    }
}
