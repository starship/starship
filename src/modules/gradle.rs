use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    config::ModuleConfig,
    configs::gradle::{GradleConfig, GradleVersionStrategy},
    context::Context,
    formatter::{StringFormatter, VersionFormatter},
    module::Module,
};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("gradle");
    let config = GradleConfig::try_load(module.config);
    let is_gradle_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_gradle_project {
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
                    let gradle_version = match config.strategy {
                        GradleVersionStrategy::Executable => parse_gradle_version_from_stdout(
                            &context
                                .exec_cmds_return_first(vec![
                                    vec!["./gradlew", "--version"],
                                    vec!["gradle", "--version"],
                                ])?
                                .stdout,
                        )?,
                        GradleVersionStrategy::WrapperProperties => {
                            let properties_file =
                                find_wrapper_properties_file(&context.current_dir)?;
                            let properties = fs::read_to_string(properties_file).ok()?;
                            parse_gradle_version_from_properties(&properties)?
                        }
                    };
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &gradle_version,
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
            log::warn!("Error in module `gradle`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_gradle_version_from_stdout(gradle_stdout: &str) -> Option<String> {
    // example gradle --version output
    /*
       ------------------------------------------------------------
       Gradle 7.5.1
       ------------------------------------------------------------

       Build time:   2022-08-05 21:17:56 UTC
       Revision:     d1daa0cbf1a0103000b71484e1dbfe096e095918

       Kotlin:       1.6.21
       Groovy:       3.0.10
       Ant:          Apache Ant(TM) version 1.10.11 compiled on July 10 2021
       JVM:          17.0.4.1 (Microsoft 17.0.4.1+1-LTS)
       OS:           Linux 5.4.0-1090-azure amd64
    */
    let version = gradle_stdout
        .split_once("Gradle ")?
        .1
        .split_whitespace()
        .next()?;
    Some(version.to_string())
}

fn parse_gradle_version_from_properties(wrapper_properties: &str) -> Option<String> {
    // example gradle.properties content
    /*
        distributionBase=GRADLE_USER_HOME
        distributionPath=wrapper/dists
        distributionUrl=https\://services.gradle.org/distributions/gradle-7.5.1-bin.zip
        zipStoreBase=GRADLE_USER_HOME
        zipStorePath=wrapper/dists
    */
    let version = wrapper_properties
        .split_once("distributionUrl=")?
        .1
        .split_once("gradle-")?
        .1
        .split_once('-')?
        .0;
    Some(version.to_string())
}

/// Tries to find the gradle-wrapper.properties file recursively.
fn find_wrapper_properties_file(directory: &Path) -> Option<PathBuf> {
    let properties_path = directory.join("gradle/wrapper/gradle-wrapper.properties");
    if properties_path.exists() {
        return Some(properties_path);
    }

    find_wrapper_properties_file(directory.parent()?)
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;

    use super::*;
    use crate::test::ModuleRenderer;
    use std::fs::File;
    use std::io::{self, Write};

    #[test]
    fn folder_without_gradle_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("gradle").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_gradle_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("build.gradle"))?.sync_all()?;

        let actual = ModuleRenderer::new("gradle")
            .path(dir.path())
            .config(toml::toml! {
                [gradle]
                strategy = "executable"
            })
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::LightCyan.bold().paint("🅶 v7.5.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_gradle_wrapper_properties() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let properties = dir
            .path()
            .join("gradle")
            .join("wrapper")
            .join("gradle-wrapper.properties");
        // create gradle/wrapper/ directories
        fs::create_dir_all(properties.parent().unwrap())?;
        // create build.gradle file to mark it as a gradle project
        File::create(dir.path().join("build.gradle"))?.sync_all()?;
        let mut file = File::create(properties)?;
        file.write_all(
            b"\
distributionBase=GRADLE_USER_HOME
distributionPath=wrapper/dists
distributionUrl=https\\://services.gradle.org/distributions/gradle-7.5.1-bin.zip
zipStoreBase=GRADLE_USER_HOME
zipStorePath=wrapper/dists",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("gradle").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::LightCyan.bold().paint("🅶 v7.5.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_gradle_version() {
        let input = "\
\
------------------------------------------------------------
Gradle 7.5.1
------------------------------------------------------------

Build time:   2022-08-05 21:17:56 UTC
Revision:     d1daa0cbf1a0103000b71484e1dbfe096e095918

Kotlin:       1.6.21
Groovy:       3.0.10
Ant:          Apache Ant(TM) version 1.10.11 compiled on July 10 2021
JVM:          17.0.4.1 (Microsoft 17.0.4.1+1-LTS)
OS:           Linux 5.4.0-1090-azure amd64
\n";
        assert_eq!(
            parse_gradle_version_from_stdout(input),
            Some("7.5.1".to_string())
        );
    }

    #[test]
    fn test_format_wrapper_properties() {
        let input = "\
distributionBase=GRADLE_USER_HOME
distributionPath=wrapper/dists
distributionUrl=https\\://services.gradle.org/distributions/gradle-7.5.1-bin.zip
zipStoreBase=GRADLE_USER_HOME
zipStorePath=wrapper/dists
        ";
        assert_eq!(
            parse_gradle_version_from_properties(input),
            Some("7.5.1".to_string())
        );
    }
}
