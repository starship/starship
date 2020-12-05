use byte_unit::{Byte, ByteUnit};

use super::{Context, Module, RootModuleConfig, Shell};

use crate::configs::memory_usage::MemoryConfig;
use crate::formatter::StringFormatter;

fn format_kib(n_kib: u64) -> String {
    let byte = Byte::from_unit(n_kib as f64, ByteUnit::KiB).unwrap_or_else(|_| Byte::from_bytes(0));
    let mut display_bytes = byte.get_appropriate_unit(true).format(0);
    display_bytes.retain(|c| c != ' ');
    display_bytes
}

fn format_pct(pct_number: f64, pct_sign: &str) -> String {
    format!("{:.0}{}", pct_number, pct_sign)
}

fn format_usage_total(usage: u64, total: u64) -> String {
    format!("{}/{}", format_kib(usage), format_kib(total))
}

/// Creates a module with system memory usage information
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("memory_usage");
    let config = MemoryConfig::try_load(module.config);

    // TODO: Update when v1.0 printing refactor is implemented to only
    // print escapes in a prompt context.
    let pct_sign = match context.shell {
        Shell::Zsh => "%%", // % is an escape in zsh, see PROMPT in `man zshmisc`
        _ => "%",
    };

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    }

    let system = match sys_info::mem_info() {
        Ok(info) => info,
        Err(err) => {
            log::warn!("Unable to access memory usage information:\n{}", err);
            return None;
        }
    };

    // avail includes reclaimable memory, but isn't supported on all platforms
    let avail_memory_kib = match system.avail {
        0 => system.free,
        _ => system.avail,
    };
    let used_memory_kib = system.total - avail_memory_kib;
    let total_memory_kib = system.total;
    let ram_used = (used_memory_kib as f64 / total_memory_kib as f64) * 100.;
    let ram_pct = format_pct(ram_used, pct_sign);

    let threshold = config.threshold;
    if ram_used.round() < threshold as f64 {
        return None;
    }

    let ram = format_usage_total(used_memory_kib, total_memory_kib);
    let total_swap_kib = system.swap_total;
    let used_swap_kib = system.swap_total - system.swap_free;
    let percent_swap_used = (used_swap_kib as f64 / total_swap_kib as f64) * 100.;
    let swap_pct = format_pct(percent_swap_used, pct_sign);
    let swap = format_usage_total(used_swap_kib, total_swap_kib);

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
                "ram" => Some(Ok(&ram)),
                "ram_pct" => Some(Ok(&ram_pct)),
                // swap only shown if there is swap on the system
                "swap" if total_swap_kib > 0 => Some(Ok(&swap)),
                "swap_pct" if total_swap_kib > 0 => Some(Ok(&swap_pct)),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `memory_usage`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
