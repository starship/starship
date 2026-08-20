use super::{Context, Module, ModuleConfig};
use crate::configs::java::JavaConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use crate::utils::get_command_string_output;
use std::path::PathBuf;

/// Creates a module with the current Java version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("java");
    let config: JavaConfig = JavaConfig::try_load(module.config);

    let is_java_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_java_project {
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
                    let java_version = get_java_version(context)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &java_version,
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
            log::warn!("Error in module `java`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_java_version(context: &Context) -> Option<String> {
    let java_command = context
        .get_env("JAVA_HOME")
        .map(PathBuf::from)
        .and_then(|path| {
            path.join("bin")
                .join("java")
                .into_os_string()
                .into_string()
                .ok()
        })
        .unwrap_or_else(|| String::from("java"));

    let output = context.exec_cmd(java_command, &["-version"])?;
    let java_version_output = get_command_string_output(output);

    parse_java_version_output(&java_version_output)
}

fn parse_java_version_output(java_version_output: &str) -> Option<String> {
    let version_line = java_version_output
        .lines()
        .find(|line| {
            line.starts_with("java version \"") || line.starts_with("openjdk version \"")
        })?;

    parse_java_version_line(version_line)
        .map(ToString::to_string)
}

fn parse_java_version_line(java_version_line: &str) -> Option<&str> {
    java_version_line
        .split('"')
        .nth(1)
        .filter(|v| !v.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_java_version_output_openjdk() {
        let java_version_output = "openjdk version \"25.0.4.1\" 2026-08-18 LTS\nOpenJDK Runtime Environment Zulu25.36+205-CA (build 25.0.4.1+1-LTS)\nOpenJDK 64-Bit Server VM Zulu25.36+205-CA (build 25.0.4.1+1-LTS, mixed mode, sharing)";
        assert_eq!(parse_java_version_output(java_version_output), Some("25.0.4.1".to_string()));
    }

    #[test]
    fn test_parse_java_version_output_oracle_jdk() {
        let java_version_output = "java version \"17.0.8\" 2023-07-18 LTS\nJava(TM) SE Runtime Environment (build 17.0.8+9-LTS-211)\nJava HotSpot(TM) 64-Bit Server VM (build 17.0.8+9-LTS-211, mixed mode, sharing)";
        assert_eq!(parse_java_version_output(java_version_output), Some("17.0.8".to_string()));
    }

    #[test]
    fn test_parse_java_version_output_with_version_not_on_first_line() {
        let java_version_output = "Picked up JAVA_TOOL_OPTIONS: -Xmx2g\nopenjdk version \"25.0.4.1\" 2026-08-18 LTS\nOpenJDK Runtime Environment Zulu25.36+205-CA (build 25.0.4.1+1-LTS)\nOpenJDK 64-Bit Server VM Zulu25.36+205-CA (build 25.0.4.1+1-LTS, mixed mode, sharing)";
        assert_eq!(parse_java_version_output(java_version_output), Some("25.0.4.1".to_string()));
    }

    #[test]
    fn test_parse_java_version_line_openjdk() {
        let java_8 = "openjdk version \"1.8.0_222\"";
        let java_11 = "openjdk version \"11.0.4\" 2019-07-18";
        let java_25 = "openjdk version \"25.0.4.1\" 2026-08-18 LTS";
        assert_eq!(parse_java_version_line(java_8), Some("1.8.0_222"));
        assert_eq!(parse_java_version_line(java_11), Some("11.0.4"));
        assert_eq!(parse_java_version_line(java_25), Some("25.0.4.1"));
    }

    #[test]
    fn test_parse_java_version_line_oracle_jdk() {
        let java_8 = "java version \"1.8.0_65\"";
        assert_eq!(parse_java_version_line(java_8), Some("1.8.0_65"));
    }

    #[test]
    fn folder_without_java_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_java_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.java"))?.sync_all()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v13.0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_java_file_preview() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.java"))?.sync_all()?;
        let actual = ModuleRenderer::new("java")
            .cmd(
                "java -version",
                Some(CommandOutput {
                    stdout: "openjdk version \"16\" 2021-01-17".to_owned(),
                    stderr: String::new(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v16 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_java_file_no_java_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.java"))?.sync_all()?;
        let actual = ModuleRenderer::new("java")
            .cmd("java -version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_class_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.class"))?.sync_all()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v13.0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_gradle_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("build.gradle"))?.sync_all()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v13.0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jar_archive() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("test.jar"))?.sync_all()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v13.0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pom_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pom.xml"))?.sync_all()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v13.0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_sdkman_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".sdkmanrc"))?.sync_all()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v13.0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_gradle_kotlin_build_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("build.gradle.kts"))?.sync_all()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v13.0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_sbt_build_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("build.gradle.kts"))?.sync_all()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v13.0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_java_version_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".java-version"))?.sync_all()?;
        let actual = ModuleRenderer::new("java").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v13.0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_java_home() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.java"))?.sync_all()?;
        let java_home: PathBuf = ["a", "b", "c"].iter().collect();
        let java_bin = java_home.join("bin").join("java");

        let actual = ModuleRenderer::new("java")
            .env("JAVA_HOME", java_home.to_str().unwrap())
            .cmd(
                &format!("{} -version", java_bin.to_str().unwrap()),
                Some(CommandOutput {
                    stdout: "openjdk version \"11.0.4\" 2019-07-16\nOpenJDK Runtime Environment (build 11.0.4+11)\nOpenJDK 64-Bit Server VM (build 11.0.4+11, mixed mode, sharing)".to_owned(),
                    stderr: String::new(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v11.0.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
