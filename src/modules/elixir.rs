use regex::Regex;

use super::{Context, Module, RootModuleConfig};

use crate::configs::elixir::ElixirConfig;

const ELIXIR_VERSION_PATTERN: &str = "\
Erlang/OTP (?P<otp>\\d+)[^\\n]+

Elixir (?P<elixir>\\d[.\\d]+).*";

/// Create a module with the current Elixir version
///
/// Will display the Rust version if any of the following criteria are met:
///     - Current directory contains a `mix.exs` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_elixir_project = context.try_begin_scan()?.set_files(&["mix.exs"]).is_match();

    if !is_elixir_project {
        return None;
    }

    let (otp_version, elixir_version) = get_elixir_version()?;

    let mut module = context.new_module("elixir");
    let config = ElixirConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &config.version.with_value(&elixir_version));
    module.create_segment(
        "otp_version",
        &config
            .otp_version
            .with_value(&format!(" (OTP {})", otp_version)),
    );

    Some(module)
}

fn get_elixir_version() -> Option<(String, String)> {
    use crate::utils;

    let output = utils::exec_cmd("elixir", &["--version"])?.stdout;

    parse_elixir_version(&output)
}

fn parse_elixir_version(version: &str) -> Option<(String, String)> {
    let version_regex = Regex::new(ELIXIR_VERSION_PATTERN).ok()?;
    let captures = version_regex.captures(version)?;

    let otp_version = captures["otp"].to_owned();
    let elixir_version = captures["elixir"].to_owned();

    Some((otp_version, elixir_version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use tempfile;

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

        let expected = None;
        let output = render_module("elixir", dir.path());

        assert_eq!(output, expected);

        Ok(())
    }

    #[test]
    fn test_with_mix_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mix.exs"))?.sync_all()?;

        let expected = Some(format!(
            "via {} ",
            Color::Purple.bold().paint("💧 1.10 (OTP 22)")
        ));
        let output = render_module("elixir", dir.path());

        assert_eq!(output, expected);

        Ok(())
    }
}
