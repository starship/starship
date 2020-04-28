use super::{Context, Module, RootModuleConfig};

use crate::configs::erlang::ErlangConfig;

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

    let erlang_version = get_erlang_version()?;

    let mut module = context.new_module("erlang");
    let config = ErlangConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &config.version.with_value(&erlang_version));

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
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_without_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let expected = None;
        let output = render_module("erlang", dir.path(), None);

        assert_eq!(output, expected);

        dir.close()
    }

    #[test]
    fn test_with_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("rebar.config"))?.sync_all()?;

        let expected = Some(format!("via {} ", Color::Red.bold().paint("🖧 22.1.3")));
        let output = render_module("erlang", dir.path(), None);

        assert_eq!(output, expected);

        dir.close()
    }
}
