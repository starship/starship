use crate::{
    config::ModuleConfig,
    configs::gradle::GradleConfig,
    context::Context,
    formatter::{StringFormatter, VersionFormatter},
    module::Module,
    utils,
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
                    let gradle_version = {
                        let properties = get_wrapper_properties_file(context, config.recursive)?;
                        parse_gradle_version_from_properties(&properties)?
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
        .lines()
        .find(|line| line.starts_with("distributionUrl="))?
        .rsplit_once('/')?
        .1
        .strip_prefix("gradle-")?
        .split_once('-')?
        .0;
    Some(version.to_string())
}

/// Tries to find the gradle-wrapper.properties file.
fn get_wrapper_properties_file(context: &Context, recursive: bool) -> Option<String> {
    let mut properties = None;
    if context
        .try_begin_scan()?
        .set_folders(&["gradle"])
        .is_match()
    {
        properties = utils::read_file(
            context
                .current_dir
                .join("gradle/wrapper/gradle-wrapper.properties"),
        )
        .ok();
    };
    if recursive && properties.is_none() {
        for dir in context.current_dir.ancestors().skip(1) {
            properties =
                utils::read_file(dir.join("gradle/wrapper/gradle-wrapper.properties")).ok();
            if properties.is_some() {
                break;
            }
        }
    }
    properties
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;

    use super::*;
    use crate::test::ModuleRenderer;
    use std::fs::{self, File};
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
    fn gradle_wrapper_recursive() -> io::Result<()> {
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

        let target_dir = dir.path().join("working_dir");
        fs::create_dir(&target_dir)?;
        File::create(target_dir.join("build.gradle.kts"))?.sync_all()?;

        let actual = ModuleRenderer::new("gradle")
            .config(toml::toml! {
                [gradle]
                recursive = true
            })
            .path(target_dir)
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::LightCyan.bold().paint("🅶 v7.5.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
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
