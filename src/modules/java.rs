use crate::configs::java::JavaConfig;
use crate::formatter::StringFormatter;

use super::{Context, Module, RootModuleConfig};

use crate::utils;

use regex::Regex;
const JAVA_VERSION_PATTERN: &str = "(?P<version>[\\d\\.]+)[^\\s]*\\s(?:built|from)";

/// Creates a module with the current Java version
///
/// Will display the Java version if any of the following criteria are met:
///     - Current directory contains a file with a `.java`, `.class`, `.gradle` or `.jar` extension
///     - Current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_java_project = context
        .try_begin_scan()?
        .set_files(&["pom.xml", "build.gradle.kts", "build.sbt", ".java-version"])
        .set_extensions(&["java", "class", "jar", "gradle"])
        .is_match();

    if !is_java_project {
        return None;
    }

    let java_version = get_java_version()?;

    let mut module = context.new_module("java");
    let config: JavaConfig = JavaConfig::try_load(module.config);

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
                "version" => Some(Ok(&java_version)),
                _ => None,
            })
            .parse(None)
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

fn get_java_version() -> Option<String> {
    let java_command = match std::env::var("JAVA_HOME") {
        Ok(java_home) => format!("{}/bin/java", java_home),
        Err(_) => String::from("java"),
    };

    let output = utils::exec_cmd(&java_command.as_str(), &["-Xinternalversion"])?;
    let java_version = if output.stdout.is_empty() {
        output.stderr
    } else {
        output.stdout
    };

    parse_java_version(&java_version)
}

fn parse_java_version(java_version: &str) -> Option<String> {
    let re = Regex::new(JAVA_VERSION_PATTERN).ok()?;
    let captures = re.captures(java_version)?;
    let version = &captures["version"];

    Some(format!("v{}", &version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_java_version_openjdk() {
        let java_8 = "OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 10:18:43 by \"openjdk\" with gcc 4.4.7 20120313 (Red Hat 4.4.7-23)";
        let java_11 = "OpenJDK 64-Bit Server VM (11.0.4+11-post-Ubuntu-1ubuntu219.04) for linux-amd64 JRE (11.0.4+11-post-Ubuntu-1ubuntu219.04), built on Jul 18 2019 18:21:46 by \"build\" with gcc 8.3.0";
        assert_eq!(parse_java_version(java_11), Some("v11.0.4".to_string()));
        assert_eq!(parse_java_version(java_8), Some("v1.8.0".to_string()));
    }

    #[test]
    fn test_parse_java_version_oracle() {
        let java_8 = "Java HotSpot(TM) Client VM (25.65-b01) for linux-arm-vfp-hflt JRE (1.8.0_65-b17), built on Oct  6 2015 16:19:04 by \"java_re\" with gcc 4.7.2 20120910 (prerelease)";
        assert_eq!(parse_java_version(java_8), Some("v1.8.0".to_string()));
    }

    #[test]
    fn test_parse_java_version_redhat() {
        let java_8 = "OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 20:48:53 by \"root\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)";
        let java_12 = "OpenJDK 64-Bit Server VM (12.0.2+10) for linux-amd64 JRE (12.0.2+10), built on Jul 18 2019 14:41:47 by \"jenkins\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)";
        assert_eq!(parse_java_version(java_8), Some("v1.8.0".to_string()));
        assert_eq!(parse_java_version(java_12), Some("v12.0.2".to_string()));
    }

    #[test]
    fn test_parse_java_version_zulu() {
        let java_8 = "OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (Zulu 8.40.0.25-CA-linux64) (1.8.0_222-b10), built on Jul 11 2019 11:36:39 by \"zulu_re\" with gcc 4.4.7 20120313 (Red Hat 4.4.7-3)";
        let java_11 = "OpenJDK 64-Bit Server VM (11.0.4+11-LTS) for linux-amd64 JRE (Zulu11.33+15-CA) (11.0.4+11-LTS), built on Jul 11 2019 21:37:17 by \"zulu_re\" with gcc 4.9.2 20150212 (Red Hat 4.9.2-6)";
        assert_eq!(parse_java_version(java_8), Some("v1.8.0".to_string()));
        assert_eq!(parse_java_version(java_11), Some("v11.0.4".to_string()));
    }

    #[test]
    fn test_parse_java_version_eclipse_openj9() {
        let java_8 = "Eclipse OpenJ9 OpenJDK 64-bit Server VM (1.8.0_222-b10) from linux-amd64 JRE with Extensions for OpenJDK for Eclipse OpenJ9 8.0.222.0, built on Jul 17 2019 21:29:18 by jenkins with g++ (GCC) 7.3.1 20180303 (Red Hat 7.3.1-5)";
        let java_11 = "Eclipse OpenJ9 OpenJDK 64-bit Server VM (11.0.4+11) from linux-amd64 JRE with Extensions for OpenJDK for Eclipse OpenJ9 11.0.4.0, built on Jul 17 2019 21:51:37 by jenkins with g++ (GCC) 7.3.1 20180303 (Red Hat 7.3.1-5)";
        assert_eq!(parse_java_version(java_8), Some("v1.8.0".to_string()));
        assert_eq!(parse_java_version(java_11), Some("v11.0.4".to_string()));
    }

    #[test]
    fn test_parse_java_version_graalvm() {
        let java_8 = "OpenJDK 64-Bit GraalVM CE 19.2.0.1 (25.222-b08-jvmci-19.2-b02) for linux-amd64 JRE (8u222), built on Jul 19 2019 17:37:13 by \"buildslave\" with gcc 7.3.0";
        assert_eq!(parse_java_version(java_8), Some("v8".to_string()));
    }

    #[test]
    fn test_parse_java_version_amazon_corretto() {
        let java_8 = "OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 20:48:53 by \"root\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)";
        let java_11 = "OpenJDK 64-Bit Server VM (11.0.4+11-LTS) for linux-amd64 JRE (11.0.4+11-LTS), built on Jul 11 2019 20:06:11 by \"\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)";
        assert_eq!(parse_java_version(java_8), Some("v1.8.0".to_string()));
        assert_eq!(parse_java_version(java_11), Some("v11.0.4".to_string()));
    }

    #[test]
    fn test_parse_java_version_sapmachine() {
        let java_11 = "OpenJDK 64-Bit Server VM (11.0.4+11-LTS-sapmachine) for linux-amd64 JRE (11.0.4+11-LTS-sapmachine), built on Jul 17 2019 08:58:43 by \"\" with gcc 7.3.0";
        assert_eq!(parse_java_version(java_11), Some("v11.0.4".to_string()));
    }

    #[test]
    fn test_parse_java_version_unknown() {
        let unknown_jre = "Unknown JRE";
        assert_eq!(parse_java_version(unknown_jre), None);
    }

    #[test]
    fn folder_without_java_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("java", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_java_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.java"))?.sync_all()?;
        let actual = render_module("java", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Red.dimmed().paint("☕ v13.0.2")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_class_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.class"))?.sync_all()?;
        let actual = render_module("java", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Red.dimmed().paint("☕ v13.0.2")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_gradle_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("build.gradle"))?.sync_all()?;
        let actual = render_module("java", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Red.dimmed().paint("☕ v13.0.2")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jar_archive() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("test.jar"))?.sync_all()?;
        let actual = render_module("java", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Red.dimmed().paint("☕ v13.0.2")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pom_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pom.xml"))?.sync_all()?;
        let actual = render_module("java", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Red.dimmed().paint("☕ v13.0.2")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_gradle_kotlin_build_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("build.gradle.kts"))?.sync_all()?;
        let actual = render_module("java", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Red.dimmed().paint("☕ v13.0.2")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_sbt_build_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("build.gradle.kts"))?.sync_all()?;
        let actual = render_module("java", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Red.dimmed().paint("☕ v13.0.2")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_java_version_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".java-version"))?.sync_all()?;
        let actual = render_module("java", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Red.dimmed().paint("☕ v13.0.2")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
