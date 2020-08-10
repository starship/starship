use byte_unit::{Byte, ByteUnit};
#[cfg(not(windows))]
use sysinfo::{RefreshKind, SystemExt};
#[cfg(windows)]
use winapi::shared::minwindef::DWORD;
#[cfg(windows)]
use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

use super::{Context, Module, RootModuleConfig, Shell};

use crate::configs::memory_usage::MemoryConfig;
use crate::formatter::StringFormatter;

fn format_kb(n_kb: u64) -> String {
    let byte = Byte::from_unit(n_kb as f64, ByteUnit::KB).unwrap_or_else(|_| Byte::from_bytes(0));
    let mut display_bytes = byte.get_appropriate_unit(true).format(0);
    display_bytes.retain(|c| c != ' ');
    display_bytes
}

fn format_pct(pct_number: f64, pct_sign: &str) -> String {
    format!("{:.0}{}", pct_number, pct_sign)
}

fn format_usage_total(usage: u64, total: u64) -> String {
    format!("{}/{}", format_kb(usage), format_kb(total))
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

    if config.disabled {
        return None;
    }
    let used_memory_kb;
    let total_memory_kb;
    let total_swap_kb;
    let used_swap_kb;

    #[cfg(not(windows))]
    {
        let system = sysinfo::System::new_with_specifics(RefreshKind::new().with_memory());
        used_memory_kb = system.get_used_memory();
        total_memory_kb = system.get_total_memory();
        total_swap_kb = system.get_total_swap();
        used_swap_kb = system.get_used_swap();
    }

    // don't use sysinfo on windows because it's very slow there
    #[cfg(windows)]
    {
        let mut mem_info = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as DWORD,
            ..MEMORYSTATUSEX::default()
        };

        let ret = unsafe { GlobalMemoryStatusEx(&mut mem_info) };
        if ret == 0 {
            log::warn!(
                "Error in module `memory_usage`:\n{:?}",
                std::io::Error::last_os_error()
            );
            return None;
        }

        used_memory_kb = mem_info.ullAvailPhys / 1_000;
        total_memory_kb = mem_info.ullTotalPhys / 1_000;
        total_swap_kb = (mem_info.ullTotalPageFile - mem_info.ullTotalPhys) / 1_000;
        used_swap_kb = mem_info.ullAvailPageFile / 1_000;
    }

    let ram_used = (used_memory_kb as f64 / total_memory_kb as f64) * 100.;
    let ram_pct = format_pct(ram_used, pct_sign);

    let threshold = config.threshold;
    if ram_used.round() < threshold as f64 {
        return None;
    }

    let ram = format_usage_total(used_memory_kb, total_memory_kb);
    let percent_swap_used = (used_swap_kb as f64 / total_swap_kb as f64) * 100.;
    let swap_pct = format_pct(percent_swap_used, pct_sign);
    let swap = format_usage_total(used_swap_kb, total_swap_kb);

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
                "swap" if total_swap_kb > 0 => Some(Ok(&swap)),
                "swap_pct" if total_swap_kb > 0 => Some(Ok(&swap_pct)),
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
