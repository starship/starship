use super::{Context, Module, RootModuleConfig};
use byte_unit::{Byte, ByteUnit};
use sysinfo::{RefreshKind, SystemExt};

use crate::config::SegmentConfig;
use crate::configs::memory_usage::MemoryUsageConfig;

/// Creates a module with system memory usage information
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("memory_usage");
    let config = MemoryUsageConfig::try_load(module.config);

    let system = sysinfo::System::new_with_specifics(RefreshKind::new().with_system());

    let used_memory_kib = system.get_used_memory();
    let total_memory_kib = system.get_total_memory();
    let used_swap_kib = system.get_used_swap();
    let total_swap_kib = system.get_total_swap();

    let percent_mem_used = (used_memory_kib as f64 / total_memory_kib as f64) * 100.;
    let percent_swap_used = (used_swap_kib as f64 / total_swap_kib as f64) * 100.;

    if percent_mem_used.round() < config.threshold as f64 {
        return None;
    }

    let (display_mem, display_swap) = if config.show_percentage {
        (
            format!("{:.0}%", percent_mem_used),
            format!("{:.0}%", percent_swap_used),
        )
    } else {
        fn format_kib(n_kib: u64) -> String {
            let byte = Byte::from_unit(n_kib as f64, ByteUnit::KiB)
                .unwrap_or_else(|_| Byte::from_bytes(0));
            let mut display_bytes = byte.get_appropriate_unit(true).format(0);
            display_bytes.retain(|c| c != ' ');
            display_bytes
        }
        (
            format!(
                "{}/{}",
                format_kib(used_memory_kib),
                format_kib(total_memory_kib)
            ),
            format!(
                "{}/{}",
                format_kib(used_swap_kib),
                format_kib(total_swap_kib)
            ),
        )
    };

    module.create_segment("symbol", &config.symbol);

    module.set_style(config.style);
    if config.show_swap {
        module.create_segment(
            "memory_usage",
            &SegmentConfig::new("").with_value(&format!("{} | {}", display_mem, display_swap)),
        );
    } else {
        module.create_segment(
            "memory_usage",
            &SegmentConfig::new("").with_value(&display_mem),
        );
    }

    module.get_prefix().set_value("");

    Some(module)
}
