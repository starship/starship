use super::{Context, Module, RootModuleConfig};

use crate::configs::status::StatusConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the status of the last command
///
/// Will display the status only if it is not 0
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let exit_code = context
        .properties
        .get("status_code")
        .map_or("0", String::as_str);

    if exit_code == "0" {
        None
    } else {
        let mut module = context.new_module("status");
        let config = StatusConfig::try_load(module.config);

        // As we default to disabled=true, we have to check here after loading our config module,
        // before it was only checking against whatever is in the config starship.toml
        if config.disabled {
            return None;
        };

        let exit_code_str = match config.meaning {
            true => signal_name_from_exit_code(exit_code),
            false => exit_code,
        };
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
                    "status" => Some(Ok(exit_code_str)),
                    _ => None,
                })
                .parse(None)
        });

        module.set_segments(match parsed {
            Ok(segments) => segments,
            Err(_error) => {
                log::warn!("Error parsing format string in `status.format`");
                return None;
            }
        });
        Some(module)
    }
}

fn signal_name_from_exit_code(exit_code: &str) -> &str {
    match exit_code {
        "1" => "ERROR",
        "2" => "USAGE",
        "126" => "NOPERM",
        "127" => "NOTFOUND",
        "129" => "SIGHUP",    // 128 + 1
        "130" => "SIGINT",    // 128 + 2
        "131" => "SIGQUIT",   // 128 + 3
        "132" => "SIGILL",    // 128 + 4
        "133" => "SIGTRAP",   // 128 + 5
        "134" => "SIGIOT",    // 128 + 6
        "135" => "SIGBUS",    // 128 + 7
        "136" => "SIGFPE",    // 128 + 8
        "137" => "SIGKILL",   // 128 + 9
        "138" => "SIGUSR1",   // 128 + 10
        "139" => "SIGSEGV",   // 128 + 11
        "140" => "SIGUSR2",   // 128 + 12
        "141" => "SIGPIPE",   // 128 + 13
        "142" => "SIGALRM",   // 128 + 14
        "143" => "SIGTERM",   // 128 + 15
        "144" => "SIGSTKFLT", // 128 + 16
        "145" => "SIGCHLD",   // 128 + 17
        "146" => "SIGCONT",   // 128 + 18
        "147" => "SIGSTOP",   // 128 + 19
        "148" => "SIGTSTP",   // 128 + 20
        "149" => "SIGTTIN",   // 128 + 21
        "150" => "SIGTTOU",   // 128 + 22
        _ => exit_code,
    }
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::io;

    use crate::test::ModuleRenderer;

    #[test]
    fn success_status() -> io::Result<()> {
        let expected = None;

        // Status code 0
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                disabled = false
            })
            .status(0)
            .collect();
        assert_eq!(expected, actual);

        // No status code
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                disabled = false
            })
            .collect();
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn not_enabled() -> io::Result<()> {
        let expected = None;

        let actual = ModuleRenderer::new("status").status(1).collect();
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn failure_status() -> io::Result<()> {
        let exit_values = [1, 2, 130];

        for status in exit_values.iter() {
            let expected = Some(format!(
                "{} ",
                Color::Red.bold().paint(format!("✖{}", status))
            ));
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }

        Ok(())
    }

    #[test]
    fn signal_name_enabled() -> io::Result<()> {
        let exit_values = [1, 2, 126, 127, 130, 101];
        let exit_values_name = ["ERROR", "USAGE", "NOPERM", "NOTFOUND", "SIGINT", "101"];

        for (status, name) in exit_values.iter().zip(exit_values_name.iter()) {
            let expected = Some(name.to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$status"
                    meaning = true
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }

        Ok(())
    }
}
