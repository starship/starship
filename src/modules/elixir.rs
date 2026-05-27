use super::{Context, Module, ModuleConfig};

use crate::configs::elixir::ElixirConfig;
use crate::formatter::StringFormatter;

use crate::formatter::VersionFormatter;
use std::ops::Deref;
use std::sync::LazyLock;

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

    let versions = LazyLock::new(|| get_elixir_version(context));

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
                    .and_then(|(otp_version, _)| otp_version.as_deref())
                    .map(|otp| Ok(otp.to_string())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `elixir`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_elixir_version(context: &Context) -> Option<(Option<String>, String)> {
    // Try --short-version first (Elixir 1.13+): handled by the shell wrapper,
    // so it never starts the BEAM VM and returns in ~9 ms instead of ~370 ms.
    if let Some(output) = context.exec_cmd("elixir", &["--short-version"]) {
        if let Some(version) = parse_elixir_short_version(&output.stdout) {
            return Some((None, version));
        }
    }

    // Fall back to --version for older Elixir or when --short-version is absent.
    let output = context.exec_cmd("elixir", &["--version"])?.stdout;
    parse_elixir_version(&output).map(|(otp, ver)| (Some(otp), ver))
}

fn parse_elixir_short_version(output: &str) -> Option<String> {
    let version = output.trim();
    // Guard: must start with a digit to be a bare version string like "1.14.0".
    // An unrecognised flag on pre-1.13 Elixir starts the VM and prints the full
    // --version banner instead, which begins with "Erlang".
    if version.starts_with(|c: char| c.is_ascii_digit()) {
        Some(version.to_string())
    } else {
        None
    }
}

fn parse_elixir_version(version: &str) -> Option<(String, String)> {
    let mut lines = version.lines();
    // split line into ["Erlang/OTP", "22", "[erts-10.5]", ...], take "22"
    let otp_version = lines.next()?.split_whitespace().nth(1)?;
    // skip empty line
    let _ = lines.next()?;
    // split line into ["Elixir", "1.10", "(compiled", ...], take "1.10"
    let elixir_version = lines.next()?.split_whitespace().nth(1)?;

    Some((otp_version.to_string(), elixir_version.to_string()))
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
        assert_eq!(
            parse_elixir_version(stable_input),
            Some(("23".to_string(), "1.11.3".to_string()))
        );
        assert_eq!(
            parse_elixir_version(rc_input),
            Some(("23".to_string(), "1.12.0-rc.0".to_string()))
        );
        assert_eq!(
            parse_elixir_version(dev_input),
            Some(("23".to_string(), "1.13.0-dev".to_string()))
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
        // The global mock returns "1.13.4\n" for `elixir --short-version`,
        // so the fast path is taken and OTP is not shown.
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mix.exs"))?.sync_all()?;

        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("💧 v1.13.4 ")
        ));
        let output = ModuleRenderer::new("elixir").path(dir.path()).collect();

        assert_eq!(output, expected);

        dir.close()
    }

    #[test]
    fn test_with_mix_file_version_fallback() -> io::Result<()> {
        // Simulate pre-1.13 Elixir where --short-version is unavailable.
        // The module should fall back to --version and show both versions.
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mix.exs"))?.sync_all()?;

        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("💧 v1.10 (OTP 22) ")
        ));
        let output = ModuleRenderer::new("elixir")
            .path(dir.path())
            .cmd("elixir --short-version", None) // simulate absent flag
            .collect();

        assert_eq!(output, expected);

        dir.close()
    }

    #[test]
    fn test_parse_elixir_short_version() {
        assert_eq!(
            parse_elixir_short_version("1.13.4\n"),
            Some("1.13.4".to_string())
        );
        assert_eq!(
            parse_elixir_short_version("1.14.0-rc.0\n"),
            Some("1.14.0-rc.0".to_string())
        );
        // Should reject the full --version banner (pre-1.13 fallback output)
        assert_eq!(
            parse_elixir_short_version(
                "Erlang/OTP 22 [erts-10.6.4]\n\nElixir 1.12.0 (compiled with Erlang/OTP 22)\n"
            ),
            None
        );
        assert_eq!(parse_elixir_short_version(""), None);
    }
}
