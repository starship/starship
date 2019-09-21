use ansi_term::Color;

use super::{Context, Module};
use byte_unit::{Byte, ByteUnit};
use sysinfo::SystemExt;

/// Creates a module with system memory usage information
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("memory_usage");

    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::White.bold().dimmed());

    let system = sysinfo::System::new();

    let free_memory_kib = system.get_free_memory();
    let total_memory_kib = system.get_total_memory();
    let free_swap_kib = system.get_free_swap();
    let total_swap_kib = system.get_total_swap();

    let percent_mem_free = (free_memory_kib as f64 / total_memory_kib as f64) * 100.;
    let percent_swap_free = (free_swap_kib as f64 / total_swap_kib as f64) * 100.;

    let show_percentage = module.config_value_bool("show_percentage").unwrap_or(false);

    let (display_mem, display_swap) = if show_percentage {
        (
            format!("{:.0}%", percent_mem_free),
            format!("{:.0}%", percent_swap_free),
        )
    } else {
        fn format_kib(n_kib: u64) -> String {
            let byte = Byte::from_unit(n_kib as f64, ByteUnit::KiB).unwrap_or(Byte::from_bytes(0));
            byte.get_appropriate_unit(true).format(0).replace(" ", "")
        }
        (
            format!(
                "{}/{}",
                format_kib(free_memory_kib),
                format_kib(total_memory_kib)
            ),
            format!(
                "{}/{}",
                format_kib(free_swap_kib),
                format_kib(total_swap_kib)
            ),
        )
    };

    let show_swap = module
        .config_value_bool("show_swap")
        .unwrap_or(total_swap_kib != 0);

    module.set_style(module_style);
    if show_swap {
        module.new_segment(
            "memory_usage",
            &format!("{} | {}", display_mem, display_swap),
        );
    } else {
        module.new_segment("memory_usage", &display_mem);
    }

    module.get_prefix().set_value("using ");

    Some(module)
}
