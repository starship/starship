use byte_unit::{Byte, ByteUnit};
use sysinfo::{RefreshKind, SystemExt};

use super::{Context, Module, RootModuleConfig};

use crate::configs::memory_usage::MemoryConfig;

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
        _ => "%",
    };

    if config.disabled {
        return None;
    }

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    let system = sysinfo::System::new_with_specifics(RefreshKind::new().with_memory());

    let used_memory_kib = system.get_used_memory();
    let total_memory_kib = system.get_total_memory();

    let percent_mem_used = (used_memory_kib as f64 / total_memory_kib as f64) * 100.;

    let threshold = config.threshold;

    if percent_mem_used.round() < threshold as f64 {
        return None;
    }

    let show_percentage = config.show_percentage;

    let ram = if show_percentage {
        format!("{:.0}{}", percent_mem_used, percent_sign)
    } else {
        format!(
            "{}/{}",
            format_kib(used_memory_kib),
            format_kib(total_memory_kib)
        )
    };
    module.create_segment("ram", &config.ram.with_value(&ram));

    // swap only shown if enabled and there is swap on the system
    let total_swap_kib = system.get_total_swap();
    if config.show_swap && total_swap_kib > 0 {
        let used_swap_kib = system.get_used_swap();
        let percent_swap_used = (used_swap_kib as f64 / total_swap_kib as f64) * 100.;

        let swap = if show_percentage {
            format!("{:.0}{}", percent_swap_used, percent_sign)
        } else {
            format!(
                "{}/{}",
                format_kib(used_swap_kib),
                format_kib(total_swap_kib)
            )
        };

        module.create_segment("separator", &config.separator);
        module.create_segment("swap", &config.swap.with_value(&swap));
    }

    Some(module)
}
