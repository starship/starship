use systemstat::{
    Platform, System,
    data::{ByteSize, saturating_sub_bytes},
};

use super::{Context, Module, ModuleConfig};

use crate::configs::memory_usage::{MemoryConfig, ThresholdConfig};
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
fn format_usage_total(total: ByteSize, free: ByteSize) -> String {
    format!(
        "{}/{}",
        display_bs(saturating_sub_bytes(total, free)),
        display_bs(total)
    )
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
                "Failed to retrieve both memory and swap, falling back to memory only: {e}"
            );
            let mem = match system.memory() {
                Ok(mem) => mem,
                Err(e) => {
                    log::warn!("Failed to retrieve memory: {e}");
                    return None;
                }
            };

            (mem, None)
        }
    };

    let used_pct = pct(memory.total, memory.free);
    let used_rounded = used_pct.round() as i64;

    if !config.show_always && used_rounded < config.threshold {
        return None;
    }

    let style = config
        .threshold_style
        .map(|style| ThresholdConfig {
            value: config.threshold,
            style,
        })
        .iter()
        .chain(
            config
                .thresholds
                .filter(|_| config.show_always)
                .iter()
                .flat_map(|ts| ts.iter()),
        )
        .filter(|t| t.value <= used_rounded)
        .max_by_key(|t| t.value)
        .map(|t| t.style)
        .unwrap_or(config.style);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(style)),
                _ => None,
            })
            .map(|variable| match variable {
                "ram" => Some(Ok(format_usage_total(memory.total, memory.free))),
                "ram_pct" => Some(Ok(format!("{used_pct:.0}%"))),
                "swap" => Some(Ok(format_usage_total(
                    swap.as_ref()?.total,
                    swap.as_ref()?.free,
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
            log::warn!("Error in module `memory_usage`:\n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn test_format_usage_total() {
        assert_eq!(
            format_usage_total(ByteSize(1024 * 1024 * 1024), ByteSize(1024 * 1024 * 1024)),
            "0B/1GiB"
        );
        assert_eq!(
            format_usage_total(
                ByteSize(1024 * 1024 * 1024),
                ByteSize(1024 * 1024 * 1024 / 2)
            ),
            "512MiB/1GiB"
        );
        assert_eq!(
            format_usage_total(ByteSize(1024 * 1024 * 1024), ByteSize(0)),
            "1GiB/1GiB"
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

        assert!(output.is_some());
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

        assert!(output.is_none());
    }

    #[test]
    fn show_always() {
        let output = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                threshold = 101
            })
            .collect();
        assert!(output.is_none());

        let output = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true
                threshold = 101
            })
            .collect();
        assert!(output.is_some());
    }

    #[test]
    fn threshold_styling() {
        // Usage exceeds the threshold, `threshold_style` is not set, so `style` is used instead
        let expected = Some(format!("{}", Color::Green.paint("RAM")));
        let actual = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                style = "fg:green"
                threshold = -1
                format = "[RAM]($style)"
            })
            .collect();
        assert_eq!(expected, actual);

        // Usage exceeds the threshold, `threshold_style` is set
        let expected = Some(format!("{}", Color::Red.paint("RAM")));
        let actual = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                style = "fg:green"
                threshold = -1
                threshold_style = "fg:red"
                format = "[RAM]($style)"
            })
            .collect();
        assert_eq!(expected, actual);

        // Usage is below the threshold, `show_always` is true, `threshold_style` is set
        let expected = Some(format!("{}", Color::Green.paint("RAM")));
        let actual = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true
                style = "fg:green"
                threshold = 101
                threshold_style = "fg:red"
                format = "[RAM]($style)"
            })
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn optional_thresholds_missing_fields() {
        // Both `value` and `style` must be set
        let output = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true

                [[memory_usage.thresholds]]
                value = 0
            })
            .collect();
        assert!(output.is_none());

        let output = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true

                [[memory_usage.thresholds]]
                style = "fg:red"
            })
            .collect();
        assert!(output.is_none());

        let output = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true

                [[memory_usage.thresholds]]
                value = 0
                style = "fg:red"
            })
            .collect();
        assert!(output.is_some());
    }

    #[test]
    fn optional_thresholds() {
        // Usage is below the main threshold, optional thresholds are set,
        // `show_always` is false, so we don't expect any output
        let output = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                style = "fg:green"
                threshold = 101
                threshold_style = "fg:red"
                format = "[RAM]($style)"

                [[memory_usage.thresholds]]
                value = -7
                style = "fg:purple"
            })
            .collect();
        assert!(output.is_none());

        // Usage is below the main threshold, optional thresholds are set, `show_always` is true
        let expected = Some(format!("{}", Color::Cyan.paint("RAM")));
        let actual = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true
                style = "fg:green"
                threshold = 101
                threshold_style = "fg:red"
                format = "[RAM]($style)"

                [[memory_usage.thresholds]]
                value = -7
                style = "fg:purple"

                [[memory_usage.thresholds]]
                value = -5
                style = "fg:cyan"
            })
            .collect();
        assert_eq!(expected, actual);

        // Usage exceeds some thresholds, optional thresholds are set, `show_always` is true
        // We expect to see the style from the greatest exceeded threshold
        let expected = Some(format!("{}", Color::Red.paint("RAM")));
        let actual = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true
                style = "fg:green"
                threshold = -1
                threshold_style = "fg:red"
                format = "[RAM]($style)"

                [[memory_usage.thresholds]]
                value = 101
                style = "fg:purple"

                [[memory_usage.thresholds]]
                value = -5
                style = "fg:cyan"
            })
            .collect();
        assert_eq!(expected, actual);

        // Usage exceeds all threshold, `show_always` is true
        // We expect to see the style from the greatest exceeded threshold
        let expected = Some(format!("{}", Color::Purple.paint("RAM")));
        let actual = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true
                style = "fg:green"
                threshold = -3
                threshold_style = "fg:red"
                format = "[RAM]($style)"

                [[memory_usage.thresholds]]
                value = -1
                style = "fg:purple"

                [[memory_usage.thresholds]]
                value = -5
                style = "fg:cyan"
            })
            .collect();
        assert_eq!(expected, actual);

        // Usage exceeds the main and some optional thresholds,
        // `show_always` is true, `threshold_style` is not set,
        // we expect an appropriate style from one of the optional thresholds
        let expected = Some(format!("{}", Color::Cyan.paint("RAM")));
        let actual = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true
                style = "fg:green"
                threshold = -1
                format = "[RAM]($style)"

                [[memory_usage.thresholds]]
                value = 101
                style = "fg:purple"

                [[memory_usage.thresholds]]
                value = -5
                style = "fg:cyan"
            })
            .collect();
        assert_eq!(expected, actual);

        // Usage exceeds the main threshold, `show_always` is true
        // `threshold_style` is not set, we expect the default style
        let expected = Some(format!("{}", Color::Green.paint("RAM")));
        let actual = ModuleRenderer::new("memory_usage")
            .config(toml::toml! {
                [memory_usage]
                disabled = false
                show_always = true
                style = "fg:green"
                threshold = -1
                format = "[RAM]($style)"

                [[memory_usage.thresholds]]
                value = 101
                style = "fg:purple"

                [[memory_usage.thresholds]]
                value = 102
                style = "fg:cyan"
            })
            .collect();
        assert_eq!(expected, actual);
    }
}
