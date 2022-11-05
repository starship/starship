use std::string::ToString;

use super::{Context, Module, ModuleConfig};

use crate::configs::status::StatusConfig;
use crate::formatter::{string_formatter::StringFormatterError, StringFormatter};
use crate::segment::Segment;

type ExitCode = i32;
type SignalNumber = u32;
#[derive(PartialEq)]
enum PipeStatusStatus<'a> {
    Disabled,
    NoPipe,
    Pipe(&'a Vec<std::string::String>),
}

/// Creates a module with the status of the last command
///
/// Will display the status
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("status");
    let config = StatusConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    let exit_code = context.properties.status_code.as_deref().unwrap_or("0");

    let pipestatus_status = match &context.properties.pipestatus {
        None => PipeStatusStatus::Disabled,
        Some(ps) => match ps.len() > 1 {
            true => PipeStatusStatus::Pipe(ps),
            false => PipeStatusStatus::NoPipe,
        },
    };

    let pipestatus_status = match config.pipestatus {
        true => pipestatus_status,
        false => PipeStatusStatus::Disabled,
    };

    // Exit code is zero while success_symbol and pipestatus are all zero or disabled/missing
    if exit_code == "0"
        && config.success_symbol.is_empty()
        && (match pipestatus_status {
            PipeStatusStatus::Pipe(ps) => ps.iter().all(|s| s == "0"),
            _ => true,
        })
    {
        return None;
    }

    let segment_format = config.pipestatus_segment_format.unwrap_or(config.format);
    let segment_format_with_separator = [segment_format, config.pipestatus_separator].join("");

    // Create pipestatus string
    let pipestatus = match pipestatus_status {
        PipeStatusStatus::Pipe(pipestatus) => pipestatus
            .iter()
            .enumerate()
            .filter_map(|(i, ec)| {
                format_exit_code(
                    ec.as_str(),
                    if i == pipestatus.len() - 1 {
                        segment_format
                    } else {
                        &segment_format_with_separator
                    },
                    None,
                    &config,
                    context,
                )
                .ok()
                .map(|segments| segments.into_iter().map(|s| s.to_string()))
            })
            .flatten()
            .collect::<String>(),
        _ => String::new(),
    };

    let main_format = match pipestatus_status {
        PipeStatusStatus::Pipe(_) => config.pipestatus_format,
        _ => config.format,
    };
    let parsed = format_exit_code(exit_code, main_format, Some(&pipestatus), &config, context);

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(_error) => {
            log::warn!("Error parsing format string in `status.format`");
            return None;
        }
    });
    Some(module)
}

fn format_exit_code<'a>(
    exit_code: &'a str,
    format: &'a str,
    pipestatus: Option<&str>,
    config: &'a StatusConfig,
    context: &'a Context,
) -> Result<Vec<Segment>, StringFormatterError> {
    // First, parse as i64 to accept both i32 or u32, then normalize to i32.
    let exit_code_int: ExitCode = match exit_code.parse::<i64>() {
        Ok(i) => i as ExitCode,
        Err(_) => {
            log::warn!("Error parsing exit_code string to int");
            return Ok(Vec::new());
        }
    };

    let hex_status = format!("0x{exit_code_int:X}");

    let common_meaning = status_common_meaning(exit_code_int);

    let raw_signal_number = match config.recognize_signal_code {
        true => status_to_signal(exit_code_int),
        false => None,
    };
    let signal_number = raw_signal_number.map(|sn| sn.to_string());
    let signal_name =
        raw_signal_number.and_then(|sn| status_signal_name(sn).or(signal_number.as_deref()));

    // If not a signal and not a common meaning, it should at least print the raw exit code number
    let maybe_exit_code_number = match common_meaning.is_none() && signal_name.is_none() {
        true => Some(exit_code),
        false => None,
    };

    StringFormatter::new(format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => match exit_code_int {
                    0 => Some(config.success_symbol),
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
                "hex_status" => Some(Ok(hex_status.as_ref())),
                "int" => Some(Ok(exit_code)),
                "maybe_int" => Ok(maybe_exit_code_number).transpose(),
                "common_meaning" => Ok(common_meaning).transpose(),
                "signal_number" => Ok(signal_number.as_deref()).transpose(),
                "signal_name" => Ok(signal_name).transpose(),
                "pipestatus" => {
                    let pipestatus = pipestatus.unwrap_or_else(|| {
                        // We might enter this case if pipestatus hasn't
                        // been processed yet, which means that it has been
                        // set in format
                        log::warn!("pipestatus variable is only available in pipestatus_format");
                        ""
                    });
                    Some(Ok(pipestatus))
                }
                _ => None,
            })
            .parse(None, Some(context))
    })
}

fn status_common_meaning(ex: ExitCode) -> Option<&'static str> {
    // Over 128 are Signal exit code
    if ex > 128 {
        return None;
    }
    match ex {
        0 => Some(""), // SUCCESS can be defined by $success_symbol if the user wishes too.
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
    use nu_ansi_term::{Color, Style};

    use crate::test::ModuleRenderer;

    #[test]
    fn success_status_success_symbol_empty() {
        let expected = None;

        // Status code 0 and success_symbol = ""
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                success_symbol = ""
                disabled = false
            })
            .status(0)
            .collect();
        assert_eq!(expected, actual);

        // Status code 0 and success_symbol is missing
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                disabled = false
            })
            .status(0)
            .collect();
        assert_eq!(expected, actual);

        // No status code and success_symbol = ""
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                success_symbol = ""
                disabled = false
            })
            .collect();
        assert_eq!(expected, actual);

        // No status code and success_symbol is missing
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                disabled = false
            })
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_status_success_symbol_filled() {
        let expected = Some(format!("{} ", Color::Red.bold().paint("✔️0")));

        // Status code 0
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                success_symbol = "✔️"
                disabled = false
            })
            .status(0)
            .collect();
        assert_eq!(expected, actual);

        // No status code
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                success_symbol = "✔️"
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

        for status in &exit_values {
            let expected = Some(format!(
                "{} ",
                Color::Red.bold().paint(format!("❌{status}"))
            ));
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    symbol = "❌"
                    disabled = false
                })
                .status(*status)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn failure_hex_status() {
        let exit_values = [1, 2, 130, -2_147_467_260, 2_147_500_036];
        let string_values = ["0x1", "0x2", "0x82", "0x80004004", "0x80004004"];

        for (exit_value, string_value) in exit_values.iter().zip(string_values) {
            let expected = Some(format!(
                "{} ",
                Color::Red.bold().paint(format!("❌{string_value}"))
            ));
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    symbol = "❌"
                    disabled = false
                    format = "[${symbol}${hex_status}]($style) "
                })
                .status(*exit_value)
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

        for (status, name) in exit_values.iter().zip(&exit_values_name) {
            let expected = name.map(std::string::ToString::to_string);
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

        for (status, name) in exit_values.iter().zip(&exit_values_name) {
            let expected = name.map(std::string::ToString::to_string);
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

        for (status, name) in exit_values.iter().zip(&exit_values_name) {
            let expected = name.map(std::string::ToString::to_string);
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

        for (status, name) in exit_values.iter().zip(&exit_values_name) {
            let expected = Some((*name).to_string());
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

        for (status, name) in exit_values.iter().zip(&exit_values_name) {
            let expected = Some((*name).to_string());
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

    #[test]
    fn pipeline_uses_pipestatus_format() {
        let exit_values = [
            [0, 1, 0, 0],
            [0, 1, 2, 3],
            [130, 126, 131, 127],
            [1, 1, 1, 1],
        ];
        let exit_values_rendered = [
            "PSF 🟢=🔴 🟢 🟢",
            "PSF 🟢=🔴 🔴 🔴",
            "PSF 🧱=🚫 ⚡ 🔍",
            "PSF 🔴=🔴 🔴 🔴",
        ];

        for (status, rendered) in exit_values.iter().zip(&exit_values_rendered) {
            let main_exit_code = status[0];
            let pipe_exit_code = &status[1..];

            let expected = Some((*rendered).to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$symbol"
                    symbol = "🔴"
                    success_symbol = "🟢"
                    not_executable_symbol = "🚫"
                    not_found_symbol = "🔍"
                    sigint_symbol = "🧱"
                    signal_symbol = "⚡"
                    recognize_signal_code = true
                    map_symbol = true
                    pipestatus = true
                    pipestatus_separator = " "
                    pipestatus_format = "PSF $symbol=$pipestatus"
                    disabled = false
                })
                .status(main_exit_code)
                .pipestatus(pipe_exit_code)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn pipeline_no_map_symbols() {
        let exit_values = [
            [0, 1, 0, 0],
            [0, 1, 2, 3],
            [130, 126, 131, 127],
            [1, 1, 1, 1],
        ];
        let exit_values_rendered = [
            "PSF 🟢=🔴1 🟢0 🟢0",
            "PSF 🟢=🔴1 🔴2 🔴3",
            "PSF INT🔴=🔴126 🔴1313 🔴127",
            "PSF 🔴=🔴1 🔴1 🔴1",
        ];

        for (status, rendered) in exit_values.iter().zip(&exit_values_rendered) {
            let main_exit_code = status[0];
            let pipe_exit_code = &status[1..];

            let expected = Some((*rendered).to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$symbol$int$signal_number"
                    symbol = "🔴"
                    success_symbol = "🟢"
                    not_executable_symbol = "🚫"
                    not_found_symbol = "🔍"
                    sigint_symbol = "🧱"
                    signal_symbol = "⚡"
                    recognize_signal_code = true
                    map_symbol = false
                    pipestatus = true
                    pipestatus_separator = " "
                    pipestatus_format = "PSF $signal_name$symbol=$pipestatus"
                    disabled = false
                })
                .status(main_exit_code)
                .pipestatus(pipe_exit_code)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn successful_pipeline() {
        let pipe_exit_code = [0, 0, 0];

        let main_exit_code = 0;

        let expected = None;

        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                disabled = false
            })
            .status(main_exit_code)
            .pipestatus(&pipe_exit_code)
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn successful_pipeline_pipestatus_enabled() {
        let pipe_exit_code = [0, 0, 0];

        let main_exit_code = 0;

        let expected = None;

        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                disabled = false
                pipestatus = true
            })
            .status(main_exit_code)
            .pipestatus(&pipe_exit_code)
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn pipeline_disabled() {
        let exit_values = [[130, 126, 131, 127], [1, 1, 1, 1]];
        let exit_values_rendered = ["F 🧱", "F 🔴"];

        for (status, rendered) in exit_values.iter().zip(&exit_values_rendered) {
            let main_exit_code = status[0];
            let pipe_exit_code = &status[1..];

            let expected = Some((*rendered).to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "F $symbol"
                    symbol = "🔴"
                    success_symbol = "🟢"
                    not_executable_symbol = "🚫"
                    not_found_symbol = "🔍"
                    sigint_symbol = "🧱"
                    signal_symbol = "⚡"
                    recognize_signal_code = true
                    map_symbol = true
                    pipestatus = false
                    pipestatus_separator = " "
                    pipestatus_format = "PSF $symbol=$pipestatus"
                    disabled = false
                })
                .status(main_exit_code)
                .pipestatus(pipe_exit_code)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn pipeline_long() {
        let exit_values = [
            [130, 0, 0, 0, 30, 1, 2, 3, 142, 0, 0, 0, 130],
            [1, 0, 0, 0, 30, 127, 126, 3, 142, 0, 230, 0, 2],
        ];
        let exit_values_rendered = [
            "PSF 130INT=🟢|🟢|🟢|🔴30|🔴|🔴|🔴3|⚡|🟢|🟢|🟢|🧱",
            "PSF 1ERROR=🟢|🟢|🟢|🔴30|🔍|🚫|🔴3|⚡|🟢|⚡|🟢|🔴",
        ];

        for (status, rendered) in exit_values.iter().zip(&exit_values_rendered) {
            let main_exit_code = status[0];
            let pipe_exit_code = &status[1..];

            let expected = Some((*rendered).to_string());
            let actual = ModuleRenderer::new("status")
                .config(toml::toml! {
                    [status]
                    format = "$symbol$maybe_int"
                    symbol = "🔴"
                    success_symbol = "🟢"
                    not_executable_symbol = "🚫"
                    not_found_symbol = "🔍"
                    sigint_symbol = "🧱"
                    signal_symbol = "⚡"
                    recognize_signal_code = true
                    map_symbol = true
                    pipestatus = true
                    pipestatus_separator = "|"
                    pipestatus_format = "PSF $int$common_meaning$signal_name=$pipestatus"
                    disabled = false
                })
                .status(main_exit_code)
                .pipestatus(pipe_exit_code)
                .collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn pipestatus_segment_format() {
        let pipe_exit_code = &[0, 1];
        let main_exit_code = 1;

        let expected = Some("[0]|[1] => <1>".to_string());
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                format = "\\($status\\)"
                pipestatus = true
                pipestatus_separator = "|"
                pipestatus_format = "$pipestatus => <$status>"
                pipestatus_segment_format = "\\[$status\\]"
                disabled = false
            })
            .status(main_exit_code)
            .pipestatus(pipe_exit_code)
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn pipestatus_separator_format() {
        let pipe_exit_code = &[0, 1, 2];
        let main_exit_code = 2;

        let expected_style = Style::new().on(Color::Red).fg(Color::White).bold();
        let expected = Some(format!(
            "{}{}{}{}{}{}{}",
            expected_style.paint("["),
            expected_style.paint("0"),
            expected_style.paint("|"),
            expected_style.paint("1"),
            expected_style.paint("|"),
            expected_style.paint("2"),
            expected_style.paint("] => <2>"),
        ));
        let actual = ModuleRenderer::new("status")
            .config(toml::toml! {
                [status]
                format = "\\($status\\)"
                style = "fg:white bg:red bold"
                pipestatus = true
                pipestatus_separator = "[|]($style)"
                pipestatus_format = "[\\[]($style)$pipestatus[\\] => <$status>]($style)"
                pipestatus_segment_format = "[$status]($style)"
                disabled = false
            })
            .status(main_exit_code)
            .pipestatus(pipe_exit_code)
            .collect();
        assert_eq!(expected, actual);
    }
}
