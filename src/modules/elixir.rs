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
    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter.map(|variable| match variable {
            "version" => Some(elixir_version.clone()),
            "otp_version" => Some(otp_version.clone()),
            _ => None,
        })
    } else {
        log::warn!("Error parsing format string in `elixir.format`");
        return None;
    };

    module.set_segments(formatter.parse(None));

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
    // use super::*;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_without_mix_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let expected = None;
        let output = render_module("elixir", dir.path(), None);

        assert_eq!(output, expected);

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_with_mix_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mix.exs"))?.sync_all()?;

        let actual = render_module("elixir", dir.path());

        let expected = Some(format!(
            "via {} ",
            Color::Purple.bold().paint("💧 1.10 (OTP 22)")
        ));
        let output = render_module("elixir", dir.path(), None);

        assert_eq!(output, expected);

        dir.close()
    }
}
