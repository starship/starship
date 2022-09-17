use super::{Context, Module, ModuleConfig};

use crate::configs::helm::HelmConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Helm version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("helm");
    let config = HelmConfig::try_load(module.config);

    let is_helm_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_helm_project {
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
                    let helm_version = parse_helm_version(
                        &context
                            .exec_cmd("helm", &["version", "--short", "--client"])?
                            .stdout,
                    )?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &helm_version,
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
            log::warn!("Error in module `helm`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_helm_version(helm_stdout: &str) -> Option<String> {
    // `helm version --short --client` output looks like this:
    // v3.1.1+gafe7058
    // `helm version --short --client` output looks like this for Helm 2:
    // Client: v2.16.9+g8ad7037
    let version = helm_stdout
        // split into ("v3.1.1","gafe7058") or ("Client: v3.1.1","gafe7058")
        .split_once('+')
        // return "v3.1.1" or "Client: v3.1.1"
        .map_or(helm_stdout, |x| x.0)
        // return "v3.1.1" or " v3.1.1"
        .trim_start_matches("Client: ")
        // return "v3.1.1"
        .trim_start_matches('v')
        .trim()
        .to_string();
    Some(version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_helm_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("helm").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_helm_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("helmfile.yaml"))?.sync_all()?;

        let actual = ModuleRenderer::new("helm").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::White.bold().paint("⎈ v3.1.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_chart_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Chart.yaml"))?.sync_all()?;

        let actual = ModuleRenderer::new("helm").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::White.bold().paint("⎈ v3.1.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_parse_helm_version() {
        let helm_2 = "Client: v2.16.9+g8ad7037";
        let helm_3 = "v3.1.1+ggit afe7058";
        assert_eq!(parse_helm_version(helm_2), Some("2.16.9".to_string()));
        assert_eq!(parse_helm_version(helm_3), Some("3.1.1".to_string()));
    }
}
