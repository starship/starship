use super::{Context, Module, RootModuleConfig};

use crate::configs::elixir::ElixirConfig;
use crate::formatter::StringFormatter;
use crate::utils;

use regex::Regex;
const ELIXIR_VERSION_PATTERN: &str = "\
Erlang/OTP (?P<otp>\\d+)[^\\n]+

Elixir (?P<elixir>\\d[.\\d]+).*";

/// Create a module with the current Elixir version
///
/// Will display the Elixir version if any of the following criteria are met:
///     - Current directory contains a `mix.exs` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_elixir_project = context.try_begin_scan()?.set_files(&["mix.exs"]).is_match();

    if !is_elixir_project {
        return None;
    }

    let (elixir_version, otp_version) =
        parse_elixir_version(&utils::exec_cmd("elixir", &["version"])?.stdout.as_str())?;

    let mut module = context.new_module("elixir");
    let config = ElixirConfig::try_load(module.config);
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
                "version" => Some(Ok(elixir_version.clone())),
                _ => None,
            })
            .map(|variable| match variable {
                "otp_version" => Some(Ok(otp_version.clone())),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `elixir`:\n{}", error);
            return None;
        }
    });

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

fn parse_elixir_version(version: &str) -> Option<(String, String)> {
    let version_regex = Regex::new(ELIXIR_VERSION_PATTERN).ok()?;
    let captures = version_regex.captures(version)?;

    let elixir_version = captures["elixir"].to_owned();
    let otp_version = captures["otp"].to_owned();

    Some((elixir_version, otp_version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_elixir_version() {
        const OUTPUT: &str = "\
Erlang/OTP 22 [erts-10.5] [source] [64-bit] [smp:8:8] [ds:8:8:10] [async-threads:1] [hipe]
Elixir 1.10 (compiled with Erlang/OTP 22)
";

        assert_eq!(
            parse_elixir_version(OUTPUT),
            Some(("22".to_owned(), "1.10".to_owned()))
        );
    }

    #[test]
    fn test_without_mix_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = render_module("elixir", dir.path(), None);

        let expected = None;
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_with_mix_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mix.exs"))?.sync_all()?;

        let expected = Some(format!(
            "via {} ",
            Color::Purple.bold().paint("💧 1.10 (OTP 22)")
        ));
        let output = render_module("elixir", dir.path(), None);

        assert_eq!(output, expected);

        dir.close()
    }
}
