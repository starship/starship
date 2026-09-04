use std::path::{Path, PathBuf};

use crate::{
    config::ModuleConfig,
    configs::maven::MavenConfig,
    context::Context,
    formatter::{StringFormatter, VersionFormatter},
    module::Module,
    utils,
};

use super::maven_cache;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("maven");
    let config = MavenConfig::try_load(module.config);
    let wrapper_properties = get_wrapper_properties_file(context, config.recursive);
    let is_maven_project = wrapper_properties.is_some()
        || context
            .try_begin_scan()?
            .set_files(&config.detect_files)
            .set_extensions(&config.detect_extensions)
            .set_folders(&config.detect_folders)
            .is_match();

    if !is_maven_project {
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
                    let maven_version = match wrapper_properties.as_deref() {
                        // Prefer the Maven version pinned by the project's wrapper, if any.
                        Some(properties) => parse_maven_version_from_properties(properties),
                        // Otherwise fall back to the resolved `mvn` binary version, using a
                        // short-lived persistent cache to avoid spawning the binary on each prompt.
                        None => get_mvn_version(context, config.cache, config.cache_ttl),
                    };
                    let maven_version = maven_version.as_deref()?;
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

/// The version of the `mvn` binary installed on the machine, resolved if available and not
/// served by the persistent cache.
fn get_mvn_version(
    context: &Context,
    cache_enabled: bool,
    cache_ttl: u64,
) -> Option<String> {
    // Resolve the concrete `mvn` binary so the cache can be keyed to a specific
    // installation (e.g. an SDKMAN-managed version). This also follows symlinks.
    let binary = resolve_mvn_binary();
    let binary_name = if cfg!(windows) { "mvn.cmd" } else { "mvn" };

    // Serve from the cache first when it is enabled and the entry is fresh enough.
    if cache_enabled
        && let Some(binary) = binary.as_ref()
        && let Some(version) = maven_cache::get(binary, cache_ttl)
    {
        return Some(version);
    }

    let version = parse_mvn_version(&context.exec_cmd(binary_name, &["--version"])?.stdout)?;

    if cache_enabled
        && let Some(binary) = binary.as_ref()
    {
        maven_cache::set(binary, version.clone());
    }

    Some(version)
}

/// The canonical location of the `mvn` binary, following symlinks (bounded to avoid cycles).
fn resolve_mvn_binary() -> Option<PathBuf> {
    let exe = if cfg!(windows) { "mvn.cmd" } else { "mvn" };
    let found = which::which(exe).ok()?;
    Some(resolve_symlinks_bounded(&found))
}

/// Resolves `path` to its canonical real location, following up to 10 symlink hops.
fn resolve_symlinks_bounded(path: &Path) -> PathBuf {
    let mut current = path.to_path_buf();
    for _ in 0..10 {
        match std::fs::read_link(&current) {
            Ok(target) => {
                current = if target.is_absolute() {
                    target
                } else {
                    match current.parent() {
                        Some(dir) => dir.join(target),
                        None => target,
                    }
                };
            }
            Err(_) => break,
        }
    }

    std::fs::canonicalize(&current).unwrap_or(current)
}

/// Parses the Maven version from the first line of `mvn --version`, e.g.
/// `Apache Maven 4.0.0-rc-6 (6a8189b24518daa120539fa41ce12f2b48ec09a8)`.
fn parse_mvn_version(mvn_stdout: &str) -> Option<String> {
    mvn_stdout
        .lines()
        .next()?
        .split_once("Apache Maven")?
        .1
        .split_whitespace()
        .next()
        .map(str::to_string)
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
    fn folder_with_maven_config_does_not_trigger_module() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let maven_config = dir.path().join(".mvn").join("maven.config");
        fs::create_dir_all(maven_config.parent().unwrap())?;
        File::create(maven_config)?.sync_all()?;

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
    fn folder_with_maven_and_no_wrapper_falls_back_to_mvn() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pom.xml"))?.sync_all()?;

        let actual = ModuleRenderer::new("maven")
            .config(toml::toml! {
                [maven]
                cache = false
            })
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::LightCyan.bold().paint("🅼 v4.0.0-rc-6 ")
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

    #[test]
    fn test_format_mvn_version_stable() {
        assert_eq!(
            parse_mvn_version("Apache Maven 3.9.12 (b89855c551a02db07e8f7b36c5e6a2e60f9e3a2b)\n"),
            Some("3.9.12".to_string())
        );
    }

    #[test]
    fn test_format_mvn_version_rc() {
        assert_eq!(
            parse_mvn_version("Apache Maven 4.0.0-rc-6 (6a8189b24518daa120539fa41ce12f2b48ec09a8)\n"),
            Some("4.0.0-rc-6".to_string())
        );
    }

    #[test]
    fn test_format_mvn_version_snapshot() {
        assert_eq!(
            parse_mvn_version("Apache Maven 3.9.0-SNAPSHOT (1234567890123456789012345678901234567890)\n"),
            Some("3.9.0-SNAPSHOT".to_string())
        );
    }

    #[test]
    fn test_format_mvn_version_garbage() {
        assert_eq!(parse_mvn_version("not a maven output\n"), None);
    }

    #[test]
    #[cfg(unix)]
    fn resolve_mvn_binary_is_bounded() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path();
        let real = root.join("real-mvn");
        File::create(&real)?.sync_all()?;

        // Build a chain of symlinks longer than the 10-hop bound.
        let mut current = real;
        for i in 0..15 {
            let link = root.join(format!("link-{i}"));
            std::os::unix::fs::symlink(&current, &link)?;
            current = link;
        }

        let resolved = resolve_symlinks_bounded(&current);
        // Bounded resolution must terminate; the final canonical path still exists.
        assert!(resolved.is_absolute());
        dir.close()
    }
}
