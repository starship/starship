use super::{Context, Module, RootModuleConfig};

use crate::configs::elixir::ElixirConfig;
use crate::formatter::StringFormatter;

use futures::future::FutureExt;
use regex::Regex;

const ELIXIR_VERSION_PATTERN: &str = "\
Erlang/OTP (?P<otp>\\d+)[^\\n]+

Elixir (?P<elixir>\\d[.\\d]+).*";

/// Create a module with the current Elixir version
pub async fn module<'a>(context: &'a Context<'a>) -> Option<Module<'a>> {
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

    let versions = get_elixir_version(context).shared();

    let parsed = match StringFormatter::new(config.format) {
        Ok(formatter) => formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .async_map(|variable| {
                let versions = versions.clone();
                async move {
                    match variable.as_str() {
                        "version" => versions
                            .await
                            .as_ref()
                            .map(|(_, elixir_version)| elixir_version.to_owned())
                            .map(Ok),
                        "otp_version" => versions
                            .await
                            .as_ref()
                            .map(|(otp_version, _)| otp_version.to_owned())
                            .map(Ok),
                        _ => None,
                    }
                }
            })
            .await
            .parse(None),
        Err(e) => Err(e),
    };

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `elixir`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

async fn get_elixir_version<'a>(context: &'a Context<'a>) -> Option<(String, String)> {
    let output = context.exec_cmd("elixir", &["--version"]).await?.stdout;

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
    use crate::test::ModuleRenderer;
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
            Color::Purple.bold().paint("💧 1.10 (OTP 22) ")
        ));
        let output = ModuleRenderer::new("elixir").path(dir.path()).collect();

        assert_eq!(output, expected);

        dir.close()
    }
}
