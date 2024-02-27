use systemstat::{
    data::{saturating_sub_bytes, ByteSize},
    Platform, System,
};

use super::{Context, Module, ModuleConfig};

use crate::configs::memory_usage::MemoryConfig;
use crate::formatter::StringFormatter;

// Display a `ByteSize` in a human readable format.
fn display_bs(bs: ByteSize) -> String {
    let mut display_bytes = bs.to_string_as(true);
    let mut keep = true;
    // Skip decimals and the space before the byte unit.
    display_bytes.retain(|c| match c {
        ' ' => {
            keep = true;
            false
        }
        '.' => {
            keep = false;
            false
        }
        _ => keep,
    });
    display_bytes
}

// Calculate the memory usage from total and free memory
fn pct(total: ByteSize, free: ByteSize) -> f64 {
    100.0 * saturating_sub_bytes(total, free).0 as f64 / total.0 as f64
}

// Print usage string used/total
fn format_usage_total(total: ByteSize, free: ByteSize, single_unit: bool) -> String {
    let mut used = display_bs(saturating_sub_bytes(total, free));
    let total = display_bs(total);

    if single_unit {
        let used_unit = used.trim_start_matches(char::is_numeric);
        let total_unit = total.trim_start_matches(char::is_numeric);

        if used_unit == total_unit {
            used = used.strip_suffix(used_unit).unwrap().into();
        }
    }

    format!("{}/{}", used, total)
}

/// Creates a module with system memory usage information
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("memory_usage");
    let config = MemoryConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    }

    let system = System::new();

    // `memory_and_swap` only works on platforms that have an implementation for swap memory
    // But getting both together is faster on some platforms (Windows/Linux)
    let (memory, swap) = match system.memory_and_swap() {
        // Ignore swap if total is 0
        Ok((mem, swap)) if swap.total.0 > 0 => (mem, Some(swap)),
        Ok((mem, _)) => (mem, None),
        Err(e) => {
            log::debug!(
                "Failed to retrieve both memory and swap, falling back to memory only: {}",
                e
            );
            let mem = match system.memory() {
                Ok(mem) => mem,
                Err(e) => {
                    log::warn!("Failed to retrieve memory: {}", e);
                    return None;
                }
            };

            (mem, None)
        }
    };

    let used_pct = pct(memory.total, memory.free);

    if (used_pct.round() as i64) < config.threshold {
        return None;
    }

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
                "ram" => Some(Ok(format_usage_total(
                    memory.total,
                    memory.free,
                    config.single_unit_fractions,
                ))),
                "ram_pct" => Some(Ok(format!("{used_pct:.0}%"))),
                "swap" => Some(Ok(format_usage_total(
                    swap.as_ref()?.total,
                    swap.as_ref()?.free,
                    config.single_unit_fractions,
                ))),
                "swap_pct" => Some(Ok(format!(
                    "{:.0}%",
                    pct(swap.as_ref()?.total, swap.as_ref()?.free)
                ))),
                _ => None,
            })
            .parse(None, Some(context))
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

#[cfg(test)]
mod test {
    use super::*;

    use crate::test::ModuleRenderer;

    #[test]
    fn test_format_usage_total() {
        assert_eq!(
            format_usage_total(
                ByteSize(1024 * 1024 * 1024),
                ByteSize(1024 * 1024 * 1024),
                false
            ),
            "0B/1GiB"
        );
        assert_eq!(
            format_usage_total(
                ByteSize(1024 * 1024 * 1024),
                ByteSize(1024 * 1024 * 1024),
                true,
            ),
            "0B/1GiB"
        );
        assert_eq!(
            format_usage_total(
                ByteSize(1024 * 1024 * 1024),
                ByteSize(1024 * 1024 * 1024 / 2),
                false,
            ),
            "512MiB/1GiB"
        );
        assert_eq!(
            format_usage_total(
                ByteSize(1024 * 1024 * 1024),
                ByteSize(1024 * 1024 * 1024 / 2),
                true,
            ),
            "512MiB/1GiB"
        );
        assert_eq!(
            format_usage_total(ByteSize(1024 * 1024 * 1024), ByteSize(0), false),
            "1GiB/1GiB"
        );
        assert_eq!(
            format_usage_total(ByteSize(1024 * 1024 * 1024), ByteSize(0), true),
            "1/1GiB"
        );
    }

    #[test]
    fn test_pct() {
        assert_eq!(
            pct(ByteSize(1024 * 1024 * 1024), ByteSize(1024 * 1024 * 1024)),
            0.0
        );
        assert_eq!(
            pct(
                ByteSize(1024 * 1024 * 1024),
                ByteSize(1024 * 1024 * 1024 / 2)
            ),
            50.0
        );
        assert_eq!(pct(ByteSize(1024 * 1024 * 1024), ByteSize(0)), 100.0);
    }

    #[test]
    fn zero_threshold() {
        let output = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                threshold = 0
            })
            .collect();

        assert!(output.is_some())
    }

    #[test]
    fn impossible_threshold() {
        let output = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                threshold = 9999
            })
            .collect();

        assert!(output.is_none())
    }
}
