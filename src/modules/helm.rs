use super::{Context, Module, RootModuleConfig};

use crate::configs::helm::HelmConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Helm version
///
/// Will display the Helm version if any of the following criteria are met:
///     - Current directory contains a `helmfile.yaml` file
///     - Current directory contains a `Chart.yaml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_helm_project = context
        .try_begin_scan()?
        .set_files(&["helmfile.yaml", "Chart.yaml"])
        .is_match();

    if !is_helm_project {
        return None;
    }

    let mut module = context.new_module("helm");
    let config = HelmConfig::try_load(module.config);
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
                "version" => format_helm_version(
                    &utils::exec_cmd("helm", &["version", "--short", "--client"])?
                        .stdout
                        .as_str(),
                )
                .map(Ok),
                _ => None,
            })
            .parse(None)
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

fn format_helm_version(helm_stdout: &str) -> Option<String> {
    // `helm version --short --client` output looks like this:
    // v3.1.1+gafe7058
    // `helm version --short --client` output looks like this for Helm 2:
    // Client: v2.16.9+g8ad7037

    Some(
        helm_stdout
            // split into ["v3.1.1","gafe7058"] or ["Client: v3.1.1","gafe7058"]
            .splitn(2, '+')
            // return "v3.1.1" or "Client: v3.1.1"
            .next()?
            // return "v3.1.1" or " v3.1.1"
            .trim_start_matches("Client: ")
            // return "v3.1.1"
            .trim()
            .to_owned(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_helm_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = render_module("helm", dir.path(), None);

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_helm_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("helmfile.yaml"))?.sync_all()?;

        let actual = render_module("helm", dir.path(), None);

        let expected = Some(format!("via {} ", Color::White.bold().paint("⎈ v3.1.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_chart_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Chart.yaml"))?.sync_all()?;

        let actual = render_module("helm", dir.path(), None);

        let expected = Some(format!("via {} ", Color::White.bold().paint("⎈ v3.1.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_helm_version() {
        let helm_2 = "Client: v2.16.9+g8ad7037";
        let helm_3 = "v3.1.1+ggit afe7058";
        assert_eq!(format_helm_version(helm_2), Some("v2.16.9".to_string()));
        assert_eq!(format_helm_version(helm_3), Some("v3.1.1".to_string()));
    }
}
