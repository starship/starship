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

        let hex_status = exit_code
            .parse::<i32>()
            .ok()
            .map(|code| format!("0x{:X}", code));

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
                    "hex_status" => Some(Ok(hex_status.as_ref().map_or(exit_code, String::as_str))),
                    "text_status" => to_text_status(exit_code).map(|s| Ok(s)),
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

#[cfg(not(windows))]
fn to_text_status(exit_code: &str) -> &str {
    ""
}

#[cfg(windows)]
fn to_text_status(exit_code: &str) -> Option<&str> {
    use winapi::shared::ntdef::NTSTATUS;
    use winapi::shared::ntstatus::*;

    exit_code
        .parse::<NTSTATUS>()
        .ok()
        .and_then(|code| match code {
            STATUS_ACCESS_VIOLATION => Some("ACCESS_VIOLATION"),
            STATUS_IN_PAGE_ERROR => Some("IN_PAGE_ERROR"),
            STATUS_INVALID_HANDLE => Some("INVALID_HANDLE"),
            STATUS_INVALID_PARAMETER => Some("INVALID_PARAMETER"),
            STATUS_NO_MEMORY => Some("NO_MEMORY"),
            STATUS_ILLEGAL_INSTRUCTION => Some("ILLEGAL_INSTRUCTION"),
            STATUS_NONCONTINUABLE_EXCEPTION => Some("NONCONTINUABLE_EXCEPTION"),
            STATUS_INVALID_DISPOSITION => Some("INVALID_DISPOSITION"),
            STATUS_ARRAY_BOUNDS_EXCEEDED => Some("ARRAY_BOUNDS_EXCEEDED"),
            STATUS_FLOAT_DENORMAL_OPERAND => Some("FLOAT_DENORMAL_OPERAND"),
            STATUS_FLOAT_DIVIDE_BY_ZERO => Some("FLOAT_DIVIDE_BY_ZERO"),
            STATUS_FLOAT_INEXACT_RESULT => Some("FLOAT_INEXACT_RESULT"),
            STATUS_FLOAT_INVALID_OPERATION => Some("FLOAT_INVALID_OPERATION"),
            STATUS_FLOAT_OVERFLOW => Some("FLOAT_OVERFLOW"),
            STATUS_FLOAT_STACK_CHECK => Some("FLOAT_STACK_CHECK"),
            STATUS_FLOAT_UNDERFLOW => Some("FLOAT_UNDERFLOW"),
            STATUS_INTEGER_DIVIDE_BY_ZERO => Some("INTEGER_DIVIDE_BY_ZERO"),
            STATUS_INTEGER_OVERFLOW => Some("INTEGER_OVERFLOW"),
            STATUS_PRIVILEGED_INSTRUCTION => Some("PRIVILEGED_INSTRUCTION"),
            STATUS_STACK_OVERFLOW => Some("STACK_OVERFLOW"),
            STATUS_DLL_NOT_FOUND => Some("DLL_NOT_FOUND"),
            STATUS_ORDINAL_NOT_FOUND => Some("ORDINAL_NOT_FOUND"),
            STATUS_ENTRYPOINT_NOT_FOUND => Some("ENTRYPOINT_NOT_FOUND"),
            STATUS_CONTROL_C_EXIT => Some("CONTROL_C_EXIT"),
            STATUS_DLL_INIT_FAILED => Some("DLL_INIT_FAILED"),
            STATUS_FLOAT_MULTIPLE_FAULTS => Some("FLOAT_MULTIPLE_FAULTS"),
            STATUS_FLOAT_MULTIPLE_TRAPS => Some("FLOAT_MULTIPLE_TRAPS"),
            STATUS_REG_NAT_CONSUMPTION => Some("REG_NAT_CONSUMPTION"),
            STATUS_HEAP_CORRUPTION => Some("HEAP_CORRUPTION"),
            STATUS_STACK_BUFFER_OVERRUN => Some("STACK_BUFFER_OVERRUN"),
            STATUS_ASSERTION_FAILURE => Some("ASSERTION_FAILURE"),
            _ => None,
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
                    symbol = "✖"
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }

        Ok(())
    }

    #[test]
    fn signal_name() -> io::Result<()> {
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

        Ok(())
    }

    #[test]
    fn exit_code_name_no_signal() -> io::Result<()> {
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

        Ok(())
    }

    #[test]
    fn maybe_exit_code_number() -> io::Result<()> {
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

        Ok(())
    }

    #[test]
    fn special_symbols_no_signals() -> io::Result<()> {
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

        Ok(())
    }

    #[test]
    fn failure_hex_status() -> io::Result<()> {
        let exit_values = [1, 2, 130];

        for status in exit_values.iter() {
            let expected = Some(format!(
                "{} ",
                Color::Red.bold().paint(format!("✖0x{:X}", status))
            ));
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    disabled = false
                    format = "[${symbol}${hex_status}]($style) "
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }

        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn failure_text_status_win() -> io::Result<()> {
        use winapi::shared::ntstatus::*;
        let exit_values = [
            (0x1, ""),
            (STATUS_CONTROL_C_EXIT, " (CONTROL_C_EXIT)"),
            (STATUS_DLL_NOT_FOUND, " (DLL_NOT_FOUND)"),
        ];

        for &(status, expected_text) in exit_values.iter() {
            let expected = Some(format!(
                "{} ",
                Color::Red
                    .bold()
                    .paint(format!("✖0x{:X}{}", status, expected_text))
            ));
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    disabled = false
                    format = "[${symbol}${hex_status}( \\(${text_status}\\))]($style) "
                })
                .status(status)
                .collect();
            assert_eq!(expected, actual);
        }

        Ok(())
    }
}
