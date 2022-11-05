use super::{Context, Module, ModuleConfig};
use crate::configs::java::JavaConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use crate::utils::get_command_string_output;
use std::path::PathBuf;

use regex::Regex;
const JAVA_VERSION_PATTERN: &str = "(?P<version>[\\d\\.]+)[^\\s]*\\s(?:built|from)";

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
            log::warn!("Error in module `java`:\n{}", error);
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

    let output = context.exec_cmd(java_command, &["-Xinternalversion"])?;
    let java_version_string = get_command_string_output(output);

    parse_java_version(&java_version_string)
}

fn parse_java_version(java_version_string: &str) -> Option<String> {
    let re = Regex::new(JAVA_VERSION_PATTERN).ok()?;
    let captures = re.captures(java_version_string)?;
    let version = &captures["version"];

    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_java_version_openjdk() {
        let java_8 = "OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 10:18:43 by \"openjdk\" with gcc 4.4.7 20120313 (Red Hat 4.4.7-23)";
        let java_11 = "OpenJDK 64-Bit Server VM (11.0.4+11-post-Ubuntu-1ubuntu219.04) for linux-amd64 JRE (11.0.4+11-post-Ubuntu-1ubuntu219.04), built on Jul 18 2019 18:21:46 by \"build\" with gcc 8.3.0";
        assert_eq!(parse_java_version(java_8), Some("1.8.0".to_string()));
        assert_eq!(parse_java_version(java_11), Some("11.0.4".to_string()));
    }

    #[test]
    fn test_parse_java_version_oracle() {
        let java_8 = "Java HotSpot(TM) Client VM (25.65-b01) for linux-arm-vfp-hflt JRE (1.8.0_65-b17), built on Oct  6 2015 16:19:04 by \"java_re\" with gcc 4.7.2 20120910 (prerelease)";
        assert_eq!(parse_java_version(java_8), Some("1.8.0".to_string()));
    }

    #[test]
    fn test_parse_java_version_redhat() {
        let java_8 = "OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 20:48:53 by \"root\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)";
        let java_12 = "OpenJDK 64-Bit Server VM (12.0.2+10) for linux-amd64 JRE (12.0.2+10), built on Jul 18 2019 14:41:47 by \"jenkins\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)";
        assert_eq!(parse_java_version(java_8), Some("1.8.0".to_string()));
        assert_eq!(parse_java_version(java_12), Some("12.0.2".to_string()));
    }

    #[test]
    fn test_parse_java_version_zulu() {
        let java_8 = "OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (Zulu 8.40.0.25-CA-linux64) (1.8.0_222-b10), built on Jul 11 2019 11:36:39 by \"zulu_re\" with gcc 4.4.7 20120313 (Red Hat 4.4.7-3)";
        let java_11 = "OpenJDK 64-Bit Server VM (11.0.4+11-LTS) for linux-amd64 JRE (Zulu11.33+15-CA) (11.0.4+11-LTS), built on Jul 11 2019 21:37:17 by \"zulu_re\" with gcc 4.9.2 20150212 (Red Hat 4.9.2-6)";
        assert_eq!(parse_java_version(java_8), Some("1.8.0".to_string()));
        assert_eq!(parse_java_version(java_11), Some("11.0.4".to_string()));
    }

    #[test]
    fn test_parse_java_version_eclipse_openj9() {
        let java_8 = "Eclipse OpenJ9 OpenJDK 64-bit Server VM (1.8.0_222-b10) from linux-amd64 JRE with Extensions for OpenJDK for Eclipse OpenJ9 8.0.222.0, built on Jul 17 2019 21:29:18 by jenkins with g++ (GCC) 7.3.1 20180303 (Red Hat 7.3.1-5)";
        let java_11 = "Eclipse OpenJ9 OpenJDK 64-bit Server VM (11.0.4+11) from linux-amd64 JRE with Extensions for OpenJDK for Eclipse OpenJ9 11.0.4.0, built on Jul 17 2019 21:51:37 by jenkins with g++ (GCC) 7.3.1 20180303 (Red Hat 7.3.1-5)";
        assert_eq!(parse_java_version(java_8), Some("1.8.0".to_string()));
        assert_eq!(parse_java_version(java_11), Some("11.0.4".to_string()));
    }

    #[test]
    fn test_parse_java_version_graalvm() {
        let java_8 = "OpenJDK 64-Bit GraalVM CE 19.2.0.1 (25.222-b08-jvmci-19.2-b02) for linux-amd64 JRE (8u222), built on Jul 19 2019 17:37:13 by \"buildslave\" with gcc 7.3.0";
        assert_eq!(parse_java_version(java_8), Some("8".to_string()));
    }

    #[test]
    fn test_parse_java_version_amazon_corretto() {
        let java_8 = "OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 20:48:53 by \"root\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)";
        let java_11 = "OpenJDK 64-Bit Server VM (11.0.4+11-LTS) for linux-amd64 JRE (11.0.4+11-LTS), built on Jul 11 2019 20:06:11 by \"\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)";
        assert_eq!(parse_java_version(java_8), Some("1.8.0".to_string()));
        assert_eq!(parse_java_version(java_11), Some("11.0.4".to_string()));
    }

    #[test]
    fn test_parse_java_version_sapmachine() {
        let java_11 = "OpenJDK 64-Bit Server VM (11.0.4+11-LTS-sapmachine) for linux-amd64 JRE (11.0.4+11-LTS-sapmachine), built on Jul 17 2019 08:58:43 by \"\" with gcc 7.3.0";
        assert_eq!(parse_java_version(java_11), Some("11.0.4".to_string()));
    }

    #[test]
    fn test_parse_java_version_unknown() {
        let unknown_jre = "Unknown JRE";
        assert_eq!(parse_java_version(unknown_jre), None);
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
        let actual = ModuleRenderer::new("java").cmd("java -Xinternalversion", Some(CommandOutput {
            stdout: "OpenJDK 64-Bit Server VM (16+14) for bsd-aarch64 JRE (16+14), built on Jan 17 2021 07:19:47 by \"brew\" with clang Apple LLVM 12.0.0 (clang-1200.0.32.28)\n".to_owned(),
            stderr: String::new()
        })).path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v16 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_java_file_no_java_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.java"))?.sync_all()?;
        let actual = ModuleRenderer::new("java")
            .cmd("java -Xinternalversion", None)
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
            .cmd(&format!("{} -Xinternalversion", java_bin.to_str().unwrap()),
            Some(CommandOutput {
                stdout: "OpenJDK 64-Bit Server VM (11.0.4+11-LTS-sapmachine) for linux-amd64 JRE (11.0.4+11-LTS-sapmachine), built on Jul 17 2019 08:58:43 by \"\" with gcc 7.3.0".to_owned(),
                stderr: String::new(),
            }))
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Red.dimmed().paint("☕ v11.0.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
