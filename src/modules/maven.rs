use std::path::Path;

use crate::{
    config::ModuleConfig,
    configs::maven::MavenConfig,
    context::Context,
    formatter::{StringFormatter, VersionFormatter},
    module::Module,
    utils,
};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("maven");
    let config = MavenConfig::try_load(module.config);
    let is_maven_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_maven_project {
        return None;
    }

    let wrapper_properties = get_wrapper_properties_file(context, config.recursive);
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
                    let properties = wrapper_properties.as_deref()?;
                    let maven_version = parse_maven_version_from_properties(properties)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &maven_version,
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
            log::warn!("Error in module `maven`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn parse_maven_version_from_properties(wrapper_properties: &str) -> Option<String> {
    // Example `maven-wrapper.properties` content
    /*
        wrapperVersion=3.3.4
        distributionType=only-script
        distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.12/apache-maven-3.9.12-bin.zip
    */
    let version = wrapper_properties
        .lines()
        .find(|line| line.starts_with("distributionUrl="))?
        .rsplit_once('/')?
        .1
        .strip_prefix("apache-maven-")?
        .rsplit_once('-')?
        .0;
    Some(version.to_string())
}

/// Tries to find the maven-wrapper.properties file.
fn get_wrapper_properties_file(context: &Context, recursive: bool) -> Option<String> {
    let read_wrapper_properties = |base_dir: &Path| {
        utils::read_file(base_dir.join(".mvn/wrapper/maven-wrapper.properties")).ok()
    };

    // Try current directory first
    if context.try_begin_scan()?.set_folders(&[".mvn"]).is_match()
        && let Some(properties) = read_wrapper_properties(&context.current_dir)
    {
        return Some(properties);
    }

    // Try parent directories if recursive
    if recursive
        && let Some(base_dir) = context.begin_ancestor_scan().set_folders(&[".mvn"]).scan()
        && let Some(properties) = read_wrapper_properties(&base_dir)
    {
        return Some(properties);
    }

    None
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;

    use super::*;
    use crate::test::ModuleRenderer;
    use std::fs::{self, File};
    use std::io::{self, Write};

    #[test]
    fn folder_without_maven_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("maven").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_maven_wrapper_properties() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let properties = dir
            .path()
            .join(".mvn")
            .join("wrapper")
            .join("maven-wrapper.properties");
        fs::create_dir_all(properties.parent().unwrap())?;
        File::create(dir.path().join("pom.xml"))?.sync_all()?;
        let mut file = File::create(properties)?;
        file.write_all(
            b"\
wrapperVersion=3.3.4
distributionType=only-script
distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.12/apache-maven-3.9.12-bin.zip
",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("maven").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::LightCyan.bold().paint("🅼 v3.9.12 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn maven_wrapper_recursive() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let properties = dir
            .path()
            .join(".mvn")
            .join("wrapper")
            .join("maven-wrapper.properties");
        fs::create_dir_all(properties.parent().unwrap())?;
        File::create(dir.path().join("pom.xml"))?.sync_all()?;
        let mut file = File::create(properties)?;
        file.write_all(
            b"\
distributionUrl=https\\://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.4/apache-maven-3.9.4-bin.zip
wrapperVersion=3.3.4
",
        )?;
        file.sync_all()?;

        let target_dir = dir.path().join("working_dir");
        fs::create_dir(&target_dir)?;
        File::create(target_dir.join("pom.xml"))?.sync_all()?;

        let actual = ModuleRenderer::new("maven")
            .config(toml::toml! {
                [maven]
                recursive = true
            })
            .path(target_dir)
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::LightCyan.bold().paint("🅼 v3.9.4 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_wrapper_properties() {
        let input = "\
distributionUrl=https\\://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.4/apache-maven-3.9.4-bin.zip
wrapperVersion=3.3.4
        ";
        assert_eq!(
            parse_maven_version_from_properties(input),
            Some("3.9.4".to_string())
        );
    }

    #[test]
    fn test_format_wrapper_properties_unstable_versions() {
        let input = |version: &str| {
            format!(
                "\
distributionUrl=https\\://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/{version}/apache-maven-{version}-bin.zip
wrapperVersion=3.3.4
        "
            )
        };
        assert_eq!(
            parse_maven_version_from_properties(&input("4.0.0-rc-1")),
            Some("4.0.0-rc-1".to_string())
        );
        assert_eq!(
            parse_maven_version_from_properties(&input("3.9.0-SNAPSHOT")),
            Some("3.9.0-SNAPSHOT".to_string())
        );
    }
}
