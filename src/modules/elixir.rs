use super::{Context, Module, ModuleConfig};

use crate::configs::elixir::ElixirConfig;
use crate::formatter::StringFormatter;

use crate::formatter::VersionFormatter;
use once_cell::sync::Lazy;
use std::ops::Deref;

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

    let versions = Lazy::new(|| match config.format.contains("$otp_version") {
        true => get_elixir_version(context),
        false => get_elixir_version_short(context),
    });

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
                "version" => versions
                    .deref()
                    .as_ref()
                    .map(|(_, elixir_version)| elixir_version)
                    .map(|elixir_version| {
                        VersionFormatter::format_module_version(
                            module.get_name(),
                            elixir_version,
                            config.version_format,
                        )
                    })?
                    .map(Ok),
                "otp_version" => versions
                    .deref()
                    .as_ref()
                    .map(|(otp_version, _)| match otp_version {
                        Some(version) => version.to_string(),
                        None => "".to_string(),
                    })
                    .map(Ok),
                _ => None,
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

fn get_elixir_version_short(context: &Context) -> Option<(Option<String>, String)> {
    context
        .exec_cmd("elixir", &["--short-version"])
        .map(|cmd| (None, cmd.stdout.to_string()))
}

fn get_elixir_version(context: &Context) -> Option<(Option<String>, String)> {
    let output = context.exec_cmd("elixir", &["--version"])?.stdout;
    parse_elixir_version(&output)
}

fn parse_elixir_version(version: &str) -> Option<(Option<String>, String)> {
    let mut lines = version.lines();

    // split line into ["Erlang/OTP", "22", "[erts-10.5]", ...], take "22"
    let otp_version = lines.next()?.split_whitespace().nth(1)?;
    // skip empty line
    let _ = lines.next()?;
    // split line into ["Elixir", "1.10", "(compiled", ...], take "1.10"
    let elixir_version = lines.next()?.split_whitespace().nth(1)?;

    Some((Some(otp_version.to_string()), elixir_version.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
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

        let short_input = "\
1.14.3
";

        assert_eq!(
            parse_elixir_version(stable_input),
            Some((Some("23".to_string()), "1.11.3".to_string()))
        );
        assert_eq!(
            parse_elixir_version(rc_input),
            Some((Some("23".to_string()), "1.12.0-rc.0".to_string()))
        );
        assert_eq!(
            parse_elixir_version(dev_input),
            Some((Some("23".to_string()), "1.13.0-dev".to_string()))
        );

        assert_eq!(
            parse_elixir_version(short_input),
            Some((None, "1.14.3".to_string()))
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
