use regex::Regex;
use std::process::Command;

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
    let output = Command::new("elixir")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())?;

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

    #[test]
    fn test_parse_elixir_version() {
        const OUTPUT: &str = "\
Erlang/OTP 22 [erts-10.5] [source] [64-bit] [smp:8:8] [ds:8:8:10] [async-threads:1] [hipe]

Elixir 1.9.0 (compiled with Erlang/OTP 22)
";

        assert_eq!(
            parse_elixir_version(OUTPUT),
            Some(("22".to_owned(), "1.9.0".to_owned()))
        );
    }
}
