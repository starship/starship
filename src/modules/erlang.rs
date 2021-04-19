use super::{Context, Module, RootModuleConfig};

use crate::configs::erlang::ErlangConfig;
use crate::formatter::StringFormatter;

/// Create a module with the current Erlang version
///
/// Will display the Erlang version if any of the following criteria are met:
///     - Current directory contains a rebar.config file
///     - Current directory contains a erlang.mk file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_erlang_project = context
        .try_begin_scan()?
        .set_files(&["rebar.config", "erlang.mk"])
        .is_match();

    if !is_erlang_project {
        return None;
    }

    let mut module = context.new_module("erlang");
    let config = ErlangConfig::try_load(module.config);

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
                "version" => get_erlang_version().map(Ok),
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

fn get_erlang_version() -> Option<String> {
    use crate::utils;

    Some(utils::exec_cmd(
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

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

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

        let expected = Some(format!("via {} ", Color::Red.bold().paint(" 22.1.3")));
        let output = ModuleRenderer::new("erlang").path(dir.path()).collect();

        assert_eq!(output, expected);

        dir.close()
    }
}
