use regex::Regex;

use crate::{
    config::ModuleConfig,
    configs::spin::SpinConfig,
    context::Context,
    formatter::{StringFormatter, VersionFormatter},
    module::Module,
};

use crate::utils::get_command_string_output;
const SPIN_CANARY: &str = "canary";
const SPIN_CANARY_IDENTIFIER: &str = "pre";
const SPIN_BINARY: &str = "spin";

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("spin");
    let config = SpinConfig::try_load(module.config);
    if !config.permanent {
        let is_spin_project = context
            .try_begin_scan()?
            .set_files(&config.detect_files)
            .set_extensions(&config.detect_extensions)
            .set_folders(&config.detect_folders)
            .is_match();

        if !is_spin_project {
            return None;
        }
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
                    let spin_version_stdout =
                        get_command_string_output(context.exec_cmd(SPIN_BINARY, &["--version"])?);
                    let spin_version = parse_spin_version(spin_version_stdout.as_str())?;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &spin_version,
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
            log::warn!("Error in module `spin`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_spin_version(spin_version_stdout: &str) -> Option<String> {
    let semver_regex = Regex::new(r"\d+\.\d+\.\d+").ok()?;
    let input = spin_version_stdout;
    let capture = semver_regex.find(input)?.as_str();
    if input.contains(SPIN_CANARY_IDENTIFIER) {
        Some(format!("{} ({})", capture, SPIN_CANARY))
    } else {
        Some(capture.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_spin_version() {
        let input = "spin 2.7.0 (a111517 2024-07-30)";
        assert_eq!(
            super::parse_spin_version(input),
            Some(String::from("2.7.0"))
        );

        let input = "spin 2.8.0-pre0 (3e62d2e 2024-09-04)";
        assert_eq!(
            super::parse_spin_version(input),
            Some(String::from("2.8.0 (canary)"))
        );
    }
}
