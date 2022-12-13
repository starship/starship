use super::{Context, Module, ModuleConfig};

use crate::configs::nextflow::NextflowConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Nextflow version
///
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // 1. check if is nextflow project
    let mut module = context.new_module("nextflow");
    let config = NextflowConfig::try_load(module.config);

    let is_nextflow_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_nextflow_project {
        return None;
    }

    // 2. Check nextflow version
    let mut nextflow_version = context.get_env("NXF_VER").unwrap_or_default();

    if nextflow_version.trim().is_empty() {
        // run nextflow -version if environment variable not set
        if let Some(v) = execute_nextflow_version(context) {
            nextflow_version = v;
        }
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
            .map(|variable| match variable {
                "version" => Some(Ok(nextflow_version.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `nextflow`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn execute_nextflow_version(context: &Context) -> Option<String> {
    log::trace!("Running nextflow -version");

    let version = context
        .exec_cmd("nextflow", &["-version"])
        .map(|command_output| command_output.stdout)
        .and_then(|nf_version_output| parse_nf_version(&nf_version_output));
    version
}

fn parse_nf_version(nf_version_output: &str) -> Option<String> {
    nf_version_output
        .split("\n")
        .filter_map(|e| match e.trim().starts_with("version") {
            true => Some(
                e.trim()
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or_default()
                    .to_owned(),
            ),
            false => None,
        })
        .map(Some)
        .collect::<Option<String>>()
}

#[cfg(test)]
mod tests {
    use super::parse_nf_version;
    use crate::test::ModuleRenderer;
    use std::fs::File;
    use std::io;
    use nu_ansi_term::Color;

    #[test]
    fn nextflow_version() {
        let sample_nextflow_output = "

                N E X T F L O W
                version 22.04.5 build 5708
                created 15-07-2022 16:09 UTC (18:09 CEST)
                cite doi:10.1038/nbt.3820
                http://nextflow.io

        ";
        assert_eq!(
            Some(String::from("22.04.5")),
            parse_nf_version(sample_nextflow_output)
        )
    }

    #[test]
    fn folder_without_nextflow_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("nextflow.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("nextflow").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nextflow_config_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("nextflow.config"))?.sync_all()?;
        let actual = ModuleRenderer::new("nextflow").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" 22.04.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nf_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("workflow.nf"))?.sync_all()?;
        let actual = ModuleRenderer::new("nextflow").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" 22.04.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
