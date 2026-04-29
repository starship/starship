use std::time::{SystemTime, UNIX_EPOCH};

use super::{Context, Module, ModuleConfig};
use crate::configs::claude_usage::ClaudeUsageConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("claude_usage");
    let config = ClaudeUsageConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let claude_data = context.claude_code_data.as_ref()?;
    let rate_limits = claude_data.rate_limits.as_ref()?;

    let five_hour_pct = rate_limits.five_hour.used_percentage;
    let seven_day_pct = rate_limits.seven_day.used_percentage;
    let max_pct = five_hour_pct.max(seven_day_pct);

    let display_style = config
        .display
        .iter()
        .filter(|s| max_pct >= s.threshold)
        .max_by(|a, b| {
            a.threshold
                .partial_cmp(&b.threshold)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

    if display_style.is_some_and(|s| s.hidden) {
        return None;
    }

    let display_style = display_style?;

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let five_hour_reset = format_duration(rate_limits.five_hour.resets_at.saturating_sub(now_secs));
    let seven_day_reset = format_duration(rate_limits.seven_day.resets_at.saturating_sub(now_secs));

    let five_hour_pct_str = format!("{:.0}", five_hour_pct);
    let seven_day_pct_str = format!("{:.0}", seven_day_pct);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(display_style.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "five_hour_pct" => Some(Ok(five_hour_pct_str.as_str())),
                "five_hour_reset" => Some(Ok(five_hour_reset.as_str())),
                "seven_day_pct" => Some(Ok(seven_day_pct_str.as_str())),
                "seven_day_reset" => Some(Ok(seven_day_reset.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `claude_usage`: {error}");
            return None;
        }
    });

    Some(module)
}

fn format_duration(secs: u64) -> String {
    if secs >= 86400 {
        format!("{}d{}h", secs / 86400, (secs % 86400) / 3600)
    } else if secs >= 3600 {
        format!("{}h{}m", secs / 3600, (secs % 3600) / 60)
    } else {
        format!("{}m", secs / 60)
    }
}

#[cfg(test)]
mod tests {
    use super::format_duration;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn test_without_data() {
        let actual = ModuleRenderer::new("claude_usage").collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn test_without_rate_limits() {
        let data = make_data(0.0, 0.0, u64::MAX, u64::MAX);
        // rate_limits is present but hidden below 70% threshold
        let actual = ModuleRenderer::new("claude_usage")
            .claude_code_data(data)
            .collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn test_disabled() {
        let data = make_data(80.0, 10.0, u64::MAX, u64::MAX);
        let actual = ModuleRenderer::new("claude_usage")
            .config(toml::toml! {
                [claude_usage]
                disabled = true
            })
            .claude_code_data(data)
            .collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn test_hidden_below_threshold() {
        let data = make_data(50.0, 30.0, u64::MAX, u64::MAX);
        let actual = ModuleRenderer::new("claude_usage")
            .claude_code_data(data)
            .collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn test_warn_threshold() {
        let data = make_data(75.0, 10.0, u64::MAX, u64::MAX);
        let actual = ModuleRenderer::new("claude_usage")
            .config(toml::toml! {
                [claude_usage]
                format = "[$five_hour_pct%]($style) "
                [[claude_usage.display]]
                threshold = 0.0
                hidden = true
                [[claude_usage.display]]
                threshold = 70.0
                style = "bold yellow"
                [[claude_usage.display]]
                threshold = 90.0
                style = "bold red"
            })
            .claude_code_data(data)
            .collect();
        assert_eq!(
            actual,
            Some(format!("{} ", Color::Yellow.bold().paint("75%")))
        );
    }

    #[test]
    fn test_critical_threshold() {
        let data = make_data(92.0, 10.0, u64::MAX, u64::MAX);
        let actual = ModuleRenderer::new("claude_usage")
            .config(toml::toml! {
                [claude_usage]
                format = "[$five_hour_pct%]($style) "
                [[claude_usage.display]]
                threshold = 0.0
                hidden = true
                [[claude_usage.display]]
                threshold = 70.0
                style = "bold yellow"
                [[claude_usage.display]]
                threshold = 90.0
                style = "bold red"
            })
            .claude_code_data(data)
            .collect();
        assert_eq!(actual, Some(format!("{} ", Color::Red.bold().paint("92%"))));
    }

    #[test]
    fn test_seven_day_drives_threshold() {
        let data = make_data(50.0, 95.0, u64::MAX, u64::MAX);
        let actual = ModuleRenderer::new("claude_usage")
            .config(toml::toml! {
                [claude_usage]
                format = "[$seven_day_pct%]($style) "
                [[claude_usage.display]]
                threshold = 0.0
                hidden = true
                [[claude_usage.display]]
                threshold = 70.0
                style = "bold yellow"
                [[claude_usage.display]]
                threshold = 90.0
                style = "bold red"
            })
            .claude_code_data(data)
            .collect();
        assert_eq!(actual, Some(format!("{} ", Color::Red.bold().paint("95%"))));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "0m");
        assert_eq!(format_duration(59), "0m");
        assert_eq!(format_duration(60), "1m");
        assert_eq!(format_duration(3599), "59m");
        assert_eq!(format_duration(3600), "1h0m");
        assert_eq!(format_duration(5025), "1h23m");
        assert_eq!(format_duration(86400), "1d0h");
        assert_eq!(format_duration(100000), "1d3h");
    }

    fn make_data(
        five_hour_pct: f32,
        seven_day_pct: f32,
        five_hour_resets_at: u64,
        seven_day_resets_at: u64,
    ) -> crate::context::ClaudeCodeData {
        use crate::context::{
            ClaudeCodeData, ContextWindow, CurrentUsage, ModelInfo, RateLimitWindow, RateLimits,
        };
        ClaudeCodeData {
            cwd: None,
            model: ModelInfo {
                id: "claude-sonnet-4-6".to_string(),
                display_name: "Claude Sonnet 4.6".to_string(),
            },
            context_window: ContextWindow {
                context_window_size: 200000,
                total_input_tokens: 1000,
                total_output_tokens: 500,
                used_percentage: 0.0,
                current_usage: CurrentUsage {
                    input_tokens: 100,
                    output_tokens: 50,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                },
            },
            cost: None,
            workspace: None,
            rate_limits: Some(RateLimits {
                five_hour: RateLimitWindow {
                    used_percentage: five_hour_pct,
                    resets_at: five_hour_resets_at,
                },
                seven_day: RateLimitWindow {
                    used_percentage: seven_day_pct,
                    resets_at: seven_day_resets_at,
                },
            }),
        }
    }
}
