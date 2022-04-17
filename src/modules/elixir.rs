use super::{Context, Module, ModuleConfig};

use crate::configs::elixir::ElixirConfig;
use crate::formatter::StringFormatter;

use crate::formatter::VersionFormatter;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

/// Create a module with the current Elixir version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("elixir");
    let config = ElixirConfig::try_load(module.config);

    let is_elixir_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_elixir_project {
        return None;
    }

    let versions: Lazy<String, _> = Lazy::new(|| {
        get_elixir_versions(context, config.version_command).unwrap_or_else(|| "".to_string())
    });
    let extracted_versions: Lazy<HashMap<&str, &str>, _> =
        Lazy::new(|| parse_elixir_versions(&versions, config.version_extract_map));

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
            .map(|variable| match extracted_versions.get(variable) {
                Some(version) => match config.version_format_map.get(variable) {
                    Some(format) => {
                        VersionFormatter::format_module_version(module.get_name(), version, format)
                            .map(Ok)
                    }
                    None => Some(Ok(version.to_string())),
                },
                None => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `elixir`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_elixir_versions(context: &Context, version_command: &str) -> Option<String> {
    Some(context.exec_cmd("elixir", &[version_command])?.stdout)
}

fn parse_elixir_versions<'a>(
    versions: &'a str,
    version_extract_map: HashMap<&'a str, &'a str>,
) -> HashMap<&'a str, &'a str> {
    let mut extracted_versions = HashMap::new();
    for (key, value) in version_extract_map.into_iter() {
        let regex = Regex::new(value).unwrap();
        if let Some(ref captures) = regex.captures(versions) {
            let option_option_match = captures.iter().nth(1);
            if let Some(Some(m)) = option_option_match {
                extracted_versions.insert(key, m.as_str());
            }
        }
    }
    extracted_versions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_elixir_version() {
        let stable_input = "\
Erlang/OTP 23 [erts-11.1.7] [source] [64-bit] [smp:4:4] [ds:4:4:10] [async-threads:1]

Elixir 1.11.3 (compiled with Erlang/OTP 21)
";
        let rc_input = "\
Erlang/OTP 23 [erts-11.1.7] [source] [64-bit] [smp:4:4] [ds:4:4:10] [async-threads:1]

Elixir 1.12.0-rc.0 (31d2b99) (compiled with Erlang/OTP 21)
";
        let dev_input = "\
Erlang/OTP 23 [erts-11.1.7] [source] [64-bit] [smp:8:8] [ds:8:8:10] [async-threads:1]

Elixir 1.13.0-dev (compiled with Erlang/OTP 23)
";
        assert_eq!(
            parse_elixir_versions(
                stable_input,
                HashMap::from([
                    ("version", "Elixir\\s([\\d\\.\\-\\w]+)"),
                    ("otp_version", "Erlang/OTP\\s([\\d\\.\\-\\w]+)"),
                ])
            ),
            HashMap::from([("version", "1.11.3"), ("otp_version", "23")])
        );

        assert_eq!(
            parse_elixir_versions(
                rc_input,
                HashMap::from([
                    ("version", "Elixir\\s([\\d\\.\\-\\w]+)"),
                    ("otp_version", "Erlang/OTP\\s([\\d\\.\\-\\w]+)"),
                ])
            ),
            HashMap::from([("version", "1.12.0-rc.0"), ("otp_version", "23")])
        );

        assert_eq!(
            parse_elixir_versions(
                dev_input,
                HashMap::from([
                    ("version", "Elixir\\s([\\d\\.\\-\\w]+)"),
                    ("otp_version", "Erlang/OTP\\s([\\d\\.\\-\\w]+)"),
                ])
            ),
            HashMap::from([("version", "1.13.0-dev"), ("otp_version", "23")])
        );
    }

    #[test]
    fn test_without_mix_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let expected = None;
        let output = ModuleRenderer::new("elixir").path(dir.path()).collect();

        assert_eq!(output, expected);

        dir.close()
    }

    #[test]
    fn test_with_mix_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mix.exs"))?.sync_all()?;

        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("💧 v1.10 (OTP 22) ")
        ));
        let output = ModuleRenderer::new("elixir").path(dir.path()).collect();

        assert_eq!(output, expected);

        dir.close()
    }
}
