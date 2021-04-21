use super::{Context, Module, RootModuleConfig};

use crate::configs::erlang::ErlangConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Create a module with the current Erlang version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("erlang");
    let config = ErlangConfig::try_load(module.config);

    let is_erlang_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_erlang_project {
        return None;
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
                "version" => {
                    let erlang_version = get_erlang_version(context)?;
                    format_erlang_version(&erlang_version, config.version_format).map(Ok)
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `erlang`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_erlang_version(context: &Context) -> Option<String> {
    Some(context.exec_cmd(
        "erl",
        &[
            "-noshell",
            "-eval",
            "Fn=filename:join([code:root_dir(),\"releases\",erlang:system_info(otp_release),\"OTP_VERSION\"]),\
             {ok,Content}=file:read_file(Fn),\
             io:format(\"~s\",[Content]),\
             halt(0)."
        ]
    )?.stdout.trim().to_string())
}

fn format_erlang_version(erlang_version: &str, version_format: &str) -> Option<String> {
    match VersionFormatter::format_version(erlang_version, version_format) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            log::warn!("Error formatting `erlang` version:\n{}", error);
            Some(format!("v{}", erlang_version))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::format_erlang_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_format_erlang_version() {
        assert_eq!(
            format_erlang_version("22.1.3", "v${major}.${minor}.${patch}"),
            Some("v22.1.3".to_string())
        )
    }

    #[test]
    fn test_format_erlang_version_truncated() {
        assert_eq!(
            format_erlang_version("22.1.3", "v${major}.${minor}"),
            Some("v22.1".to_string())
        );
    }

    #[test]
    fn test_format_erlang_version_is_malformed() {
        assert_eq!(
            format_erlang_version("22.1", "v${major}.${minor}.${patch}"),
            Some("v22.1.".to_string())
        );
    }

    #[test]
    fn test_without_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let expected = None;
        let output = ModuleRenderer::new("erlang").path(dir.path()).collect();

        assert_eq!(output, expected);

        dir.close()
    }

    #[test]
    fn test_with_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("rebar.config"))?.sync_all()?;

        let expected = Some(format!("via {}", Color::Red.bold().paint(" v22.1.3 ")));
        let output = ModuleRenderer::new("erlang").path(dir.path()).collect();

        assert_eq!(output, expected);

        dir.close()
    }
}
