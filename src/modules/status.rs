use super::{Context, Module, RootModuleConfig};

use crate::configs::status::StatusConfig;
use crate::formatter::StringFormatter;

type ExitCode = i64;
type SignalNumber = u32;

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

        let exit_code_int: ExitCode = match exit_code.parse() {
            Ok(i) => i,
            Err(_) => return None,
        };

        let common_meaning = status_common_meaning(exit_code_int);

        let raw_signal_number = match config.recognize_signal_code {
            true => status_to_signal(exit_code_int),
            false => None,
        };
        let signal_number = raw_signal_number.map(|sn| sn.to_string());
        let signal_name = raw_signal_number
            .and_then(|sn| status_signal_name(sn).or_else(|| signal_number.as_deref()));

        // If not a signal and not a common meaning, it should at least print the raw exit code number
        let maybe_exit_code_number = match common_meaning.is_none() && signal_name.is_none() {
            true => Some(exit_code),
            false => None,
        };

        let parsed = StringFormatter::new(config.format).and_then(|formatter| {
            formatter
                .map_meta(|var, _| match var {
                    "symbol" => match exit_code_int {
                        126 if config.map_symbol => Some(config.not_executable_symbol),
                        127 if config.map_symbol => Some(config.not_found_symbol),
                        130 if config.recognize_signal_code && config.map_symbol => {
                            Some(config.sigint_symbol)
                        }
                        x if (129..256).contains(&x)
                            && config.recognize_signal_code
                            && config.map_symbol =>
                        {
                            Some(config.signal_symbol)
                        }
                        _ => Some(config.symbol),
                    },
                    _ => None,
                })
                .map_style(|variable| match variable {
                    "style" => Some(Ok(config.style)),
                    _ => None,
                })
                .map(|variable| match variable {
                    "status" => Some(Ok(exit_code)),
                    "int" => Some(Ok(exit_code)),
                    "maybe_int" => Ok(maybe_exit_code_number.as_deref()).transpose(),
                    "common_meaning" => Ok(common_meaning.as_deref()).transpose(),
                    "signal_number" => Ok(signal_number.as_deref()).transpose(),
                    "signal_name" => Ok(signal_name.as_deref()).transpose(),
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

fn status_common_meaning(ex: ExitCode) -> Option<&'static str> {
    // Over 128 are Signal exit code
    if ex > 128 {
        return None;
    }
    match ex {
        1 => Some("ERROR"),
        2 => Some("USAGE"),
        126 => Some("NOPERM"),
        127 => Some("NOTFOUND"),
        _ => None,
    }
}

fn status_to_signal(ex: ExitCode) -> Option<SignalNumber> {
    if ex < 129 {
        return None;
    }
    let sn = ex - 128;
    Some(sn as u32)
}

fn status_signal_name(signal: SignalNumber) -> Option<&'static str> {
    match signal {
        1 => Some("HUP"),     // 128 + 1
        2 => Some("INT"),     // 128 + 2
        3 => Some("QUIT"),    // 128 + 3
        4 => Some("ILL"),     // 128 + 4
        5 => Some("TRAP"),    // 128 + 5
        6 => Some("IOT"),     // 128 + 6
        7 => Some("BUS"),     // 128 + 7
        8 => Some("FPE"),     // 128 + 8
        9 => Some("KILL"),    // 128 + 9
        10 => Some("USR1"),   // 128 + 10
        11 => Some("SEGV"),   // 128 + 11
        12 => Some("USR2"),   // 128 + 12
        13 => Some("PIPE"),   // 128 + 13
        14 => Some("ALRM"),   // 128 + 14
        15 => Some("TERM"),   // 128 + 15
        16 => Some("STKFLT"), // 128 + 16
        17 => Some("CHLD"),   // 128 + 17
        18 => Some("CONT"),   // 128 + 18
        19 => Some("STOP"),   // 128 + 19
        20 => Some("TSTP"),   // 128 + 20
        21 => Some("TTIN"),   // 128 + 21
        22 => Some("TTOU"),   // 128 + 22
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;

    use crate::test::ModuleRenderer;

    #[test]
    fn success_status() {
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
    }

    #[test]
    fn not_enabled() {
        let expected = None;

        let actual = ModuleRenderer::new("status").status(1).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn failure_status() {
        let exit_values = [1, 2, 130];

        for status in exit_values.iter() {
            let expected = Some(format!(
                "{} ",
                Color::Red.bold().paint(format!("✖{}", status))
            ));
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    symbol = "✖"
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn signal_name() {
        let exit_values = [1, 2, 126, 127, 130, 101];
        let exit_values_name = [
            Some("ERROR"),
            Some("USAGE"),
            Some("NOPERM"),
            Some("NOTFOUND"),
            Some("INT"),
            None,
        ];

        for (status, name) in exit_values.iter().zip(exit_values_name.iter()) {
            let expected = name.map(|n| n.to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$common_meaning$signal_name"
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn exit_code_name_no_signal() {
        let exit_values = [1, 2, 126, 127, 130, 101, 132];
        let exit_values_name = [
            Some("ERROR"),
            Some("USAGE"),
            Some("NOPERM"),
            Some("NOTFOUND"),
            None,
            None,
            None,
        ];

        for (status, name) in exit_values.iter().zip(exit_values_name.iter()) {
            let expected = name.map(|n| n.to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$common_meaning$signal_name"
                    recognize_signal_code = false
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn maybe_exit_code_number() {
        let exit_values = [1, 2, 126, 127, 130, 101, 6, -3];
        let exit_values_name = [
            None,
            None,
            None,
            None,
            None,
            Some("101"),
            Some("6"),
            Some("-3"),
        ];

        for (status, name) in exit_values.iter().zip(exit_values_name.iter()) {
            let expected = name.map(|n| n.to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$maybe_int"
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn special_symbols() {
        let exit_values = [1, 126, 127, 130, 131];
        let exit_values_name = ["🔴", "🚫", "🔍", "🧱", "⚡"];

        for (status, name) in exit_values.iter().zip(exit_values_name.iter()) {
            let expected = Some(name.to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$symbol"
                    symbol = "🔴"
                    not_executable_symbol = "🚫"
                    not_found_symbol = "🔍"
                    sigint_symbol = "🧱"
                    signal_symbol = "⚡"
                    recognize_signal_code = true
                    map_symbol = true
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn special_symbols_no_signals() {
        let exit_values = [1, 126, 127, 130, 131];
        let exit_values_name = ["🔴", "🚫", "🔍", "🔴", "🔴"];

        for (status, name) in exit_values.iter().zip(exit_values_name.iter()) {
            let expected = Some(name.to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$symbol"
                    symbol = "🔴"
                    not_executable_symbol = "🚫"
                    not_found_symbol = "🔍"
                    sigint_symbol = "🧱"
                    signal_symbol = "⚡"
                    recognize_signal_code = false
                    map_symbol = true
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }
    }
}
