use byte_unit::{Byte, ByteUnit};
use sysinfo::{RefreshKind, SystemExt};

use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig};

use crate::configs::memory_usage::MemoryConfig;
use crate::segment::Segment;

fn format_kib(n_kib: u64) -> String {
    let byte = Byte::from_unit(n_kib as f64, ByteUnit::KiB).unwrap_or_else(|_| Byte::from_bytes(0));
    let mut display_bytes = byte.get_appropriate_unit(true).format(0);
    display_bytes.retain(|c| c != ' ');
    display_bytes
}

/// Creates a module with system memory usage information
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("memory_usage");
    let config = MemoryConfig::try_load(module.config);

    // TODO: Update when v1.0 printing refactor is implemented to only
    // print escapes in a prompt context.
    let shell = std::env::var("STARSHIP_SHELL").unwrap_or_default();
    let percent_sign = match shell.as_str() {
        "zsh" => "%%", // % is an escape in zsh, see PROMPT in `man zshmisc`
        "powershell" => "`%",
        _ => "%",
    };

    if config.disabled {
        return None;
    }

    let system = sysinfo::System::new_with_specifics(RefreshKind::new().with_system());

    let used_memory_kib = system.get_used_memory();
    let total_memory_kib = system.get_total_memory();

    let percent_mem_used = (used_memory_kib as f64 / total_memory_kib as f64) * 100.;

    let threshold = config.threshold;

    if percent_mem_used.round() < threshold as f64 {
        return None;
    }

    // swap only shown if enabled and there is swap on the system
    let total_swap_kib = system.get_total_swap();

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "ram%" => Some(Segment {
                _name: "ram%".to_string(),
                value: format!("{:.0}{}", percent_mem_used, percent_sign),
                style,
            }),
            "ram" => Some(Segment {
                _name: "ram".to_string(),
                value: format!(
                    "{}/{}",
                    format_kib(used_memory_kib),
                    format_kib(total_memory_kib)
                ),
                style,
            }),
            "swap%" => {
                if total_swap_kib > 0 {
                    let used_swap_kib = system.get_used_swap();
                    let percent_swap_used = (used_swap_kib as f64 / total_swap_kib as f64) * 100.;
                    Some(Segment {
                        _name: "swap".to_string(),
                        value: format!("{:.0}{}", percent_swap_used, percent_sign),
                        style,
                    })
                } else {
                    None
                }
            }
            "swap" => {
                if total_swap_kib > 0 {
                    let used_swap_kib = system.get_used_swap();
                    Some(Segment {
                        _name: "swap".to_string(),
                        value: format!(
                            "{}/{}",
                            format_kib(used_swap_kib),
                            format_kib(total_swap_kib)
                        ),
                        style,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}
