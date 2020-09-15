use crate::configs::docker::DockerConfig;

use super::{Context, Module, RootModuleConfig};
use crate::formatter::StringFormatter;

/// Creates a segment to show if there is a Dockerfile or docker-compose.yml in current directory
///
/// Will display the symbol if any of the following criteria are met:
///     - Current directory contains a `Dockerfile` file
///     - Current directory contains a `docker-compose.yml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let docker_used = context
        .try_begin_scan()?
        .set_files(&["Dockerfile", "docker-compose.yml"])
        .is_match();

    if !docker_used {
        return None;
    }

    let mut module = context.new_module("docker");
    let config: DockerConfig = DockerConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `docker`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::fs::{self, File};
    use std::io::{self, Write};
    
    use crate::common;
    
    #[test]
    fn folder_without_docker_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let output = common::render_module("docker")
            .arg("--path")
            .arg(dir.path())
            .output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
    
        let expected = "";
        assert_eq!(expected, actual);
        dir.close()
    }
    
    #[test]
    fn folder_with_dockerfile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Dockerfile"))?;
    
        let output = common::render_module("docker")
            .arg("--path")
            .arg(dir.path())
            .output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
    
        let expected = format!("{} ", Color::Blue.bold().paint("🐳"));
        assert_eq!(expected, actual);
        dir.close()
    }
    
    #[test]
    fn folder_with_docker_compose_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("docker-compose.yml"))?;
    
        let output = common::render_module("docker")
            .arg("--path")
            .arg(dir.path())
            .output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
    
        let expected = format!("{} ", Color::Blue.bold().paint("🐳"));
        assert_eq!(expected, actual);
        dir.close()
    }
}
