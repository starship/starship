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

        let exit_code_int: u32 = match exit_code.parse() {
            Ok(i) => i,
            Err(_) => return None,
        };

        let scm = status_common_meaning(exit_code_int);
        let ssname = status_signal_int(exit_code_int);
        let ssnumber = status_signal_name(exit_code_int);

        let parsed = StringFormatter::new(config.format).and_then(|formatter| {
            formatter
                .map_meta(|var, _| match var {
                    "symbol" => match exit_code_int {
                        126 => Some(config.not_executable_symbol),
                        127 => Some(config.not_found_symbol),
                        130 => Some(config.sigint_symbol),
                        x if x > 128 && x < 256 => Some(config.signal_symbol),
                        _ => Some(config.program_error_symbol),
                    },
                    _ => None,
                })
                .map_style(|variable| match variable {
                    "style" => Some(Ok(config.style)),
                    _ => None,
                })
                .map(|variable| match variable {
                    "status" => Some(Ok(exit_code)),
                    "status_int" => Some(Ok(exit_code)),
                    "status_common_meaning" => Ok(scm.as_deref()).transpose(),
                    "status_signal_number" => Ok(ssname.as_deref()).transpose(),
                    "status_signal_name" => Ok(ssnumber.as_deref()).transpose(),
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

fn status_common_meaning(ex: u32) -> Option<String> {
    // Over 128 are Signal exit code
    if ex > 128 {
        return None;
    }
    Some(match ex {
        1 => "ERROR".to_string(),
        2 => "USAGE".to_string(),
        126 => "NOPERM".to_string(),
        127 => "NOTFOUND".to_string(),
        _ => ex.to_string(),
    })
}

fn status_signal_int(ex: u32) -> Option<String> {
    if ex < 129 {
        return None;
    }
    let ex = ex - 128;
    Some(ex.to_string())
}

fn status_signal_name(ex: u32) -> Option<String> {
    if ex < 129 {
        return None;
    }
    let ex = ex - 128;
    Some(match ex {
        1 => "HUP".to_string(),     // 128 + 1
        2 => "INT".to_string(),     // 128 + 2
        3 => "QUIT".to_string(),    // 128 + 3
        4 => "ILL".to_string(),     // 128 + 4
        5 => "TRAP".to_string(),    // 128 + 5
        6 => "IOT".to_string(),     // 128 + 6
        7 => "BUS".to_string(),     // 128 + 7
        8 => "FPE".to_string(),     // 128 + 8
        9 => "KILL".to_string(),    // 128 + 9
        10 => "USR1".to_string(),   // 128 + 10
        11 => "SEGV".to_string(),   // 128 + 11
        12 => "USR2".to_string(),   // 128 + 12
        13 => "PIPE".to_string(),   // 128 + 13
        14 => "ALRM".to_string(),   // 128 + 14
        15 => "TERM".to_string(),   // 128 + 15
        16 => "STKFLT".to_string(), // 128 + 16
        17 => "CHLD".to_string(),   // 128 + 17
        18 => "CONT".to_string(),   // 128 + 18
        19 => "STOP".to_string(),   // 128 + 19
        20 => "TSTP".to_string(),   // 128 + 20
        21 => "TTIN".to_string(),   // 128 + 21
        22 => "TTOU".to_string(),   // 128 + 22
        _ => ex.to_string(),
    })
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
                    program_error_symbol = "✖"
                    not_executable_symbol = "✖"
                    not_found_symbol = "✖"
                    sigint_symbol = "✖"
                    signal_symbol = "✖"
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
        let exit_values_name = ["ERROR", "USAGE", "NOPERM", "NOTFOUND", "INT", "101"];

        for (status, name) in exit_values.iter().zip(exit_values_name.iter()) {
            let expected = Some(name.to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$status_common_meaning$status_signal_name"
                    meaning = true
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }

        Ok(())
    }

    #[test]
    fn special_symbols() -> io::Result<()> {
        let exit_values = [1, 126, 127, 130, 131];
        let exit_values_name = ["🔴", "🚫", "🔍", "🧱", "⚡"];

        for (status, name) in exit_values.iter().zip(exit_values_name.iter()) {
            let expected = Some(name.to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$symbol"
                    program_error_symbol = "🔴"
                    not_executable_symbol = "🚫"
                    not_found_symbol = "🔍"
                    sigint_symbol = "🧱"
                    signal_symbol = "⚡"
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }

        Ok(())
    }
}
