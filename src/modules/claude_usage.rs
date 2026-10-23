use std::time::{SystemTime, UNIX_EPOCH};

use super::{Context, Module, ModuleConfig};
use crate::configs::claude_usage::ClaudeUsageConfig;
use crate::context::RateLimitWindow;
use crate::formatter::StringFormatter;

/// Remaining time with at most two units, e.g. `2d7h`, `1h23m`, `12m`, `45s`.
fn short_duration(secs: u64) -> String {
    let (d, h, m) = (secs / 86400, secs % 86400 / 3600, secs % 3600 / 60);
    match (d, h, m) {
        (0, 0, 0) => format!("{secs}s"),
        (0, 0, m) => format!("{m}m"),
        (0, h, 0) => format!("{h}h"),
        (0, h, m) => format!("{h}h{m}m"),
        (d, 0, _) => format!("{d}d"),
        (d, h, _) => format!("{d}d{h}h"),
    }
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("claude_usage");
    let config = ClaudeUsageConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let claude_data = context.claude_code_data.as_ref()?;
    let rate_limits = claude_data.rate_limits.as_ref()?;

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // A window is skipped when absent, stale, or when its own matching
    // `display` entry is hidden. Each window is styled independently.
    let live = |w: &Option<RateLimitWindow>| {
        let w = w.as_ref().filter(|w| w.resets_at > now_secs)?;
        let pct = w.used_percentage.clamp(0.0, 100.0);
        let display = config
            .display
            .iter()
            .filter(|d| pct >= d.threshold)
            .max_by(|a, b| a.threshold.total_cmp(&b.threshold))
            .filter(|d| !d.hidden)?;
        Some((
            format!("{pct:.0}%"),
            short_duration(w.resets_at - now_secs),
            display,
        ))
    };
    let five_hour = live(&rate_limits.five_hour);
    let seven_day = live(&rate_limits.seven_day);

    // `$style` mirrors the most severe visible window.
    let overall = [&five_hour, &seven_day]
        .into_iter()
        .flatten()
        .map(|w| w.2)
        .max_by(|a, b| a.threshold.total_cmp(&b.threshold))?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(overall.style)),
                "five_hour_style" => five_hour.as_ref().map(|w| Ok(w.2.style)),
                "seven_day_style" => seven_day.as_ref().map(|w| Ok(w.2.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "five_hour_percentage" => five_hour.as_ref().map(|w| Ok(w.0.as_str())),
                "five_hour_reset" => five_hour.as_ref().map(|w| Ok(w.1.as_str())),
                "seven_day_percentage" => seven_day.as_ref().map(|w| Ok(w.0.as_str())),
                "seven_day_reset" => seven_day.as_ref().map(|w| Ok(w.1.as_str())),
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

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::short_duration;
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
                format = "[$five_hour_percentage]($style) "
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
                format = "[$five_hour_percentage]($style) "
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
                format = "[$seven_day_percentage]($style) "
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
    fn test_default_format_renders() {
        // Regression test: the default `format` string must be a valid
        // format string. A malformed default caused module() to log a parse
        // error and silently return None, so the module never rendered.
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let data = make_data(75.0, 10.0, now + 3600, now + 86400);
        let actual = ModuleRenderer::new("claude_usage")
            .claude_code_data(data)
            .collect()
            .expect("claude_usage should render with the default format");
        assert!(
            actual.contains("75%"),
            "rendered output should contain the 5h percentage: {actual}",
        );
    }

    // Claude Code may omit either window, and drops one once it has reset.
    fn render_json(rate_limits: &str) -> Option<String> {
        let payload = format!(
            r#"{{"model":{{"id":"m","display_name":"M"}},"context_window":{{}},"rate_limits":{rate_limits}}}"#
        );
        let data: crate::context::ClaudeCodeData = serde_json::from_str(&payload).unwrap();
        ModuleRenderer::new("claude_usage")
            .claude_code_data(data)
            .collect()
    }

    fn future(secs: u64) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + secs
    }

    #[test]
    fn test_only_five_hour_window() {
        let out = render_json(&format!(
            r#"{{"five_hour":{{"used_percentage":75,"resets_at":{}}}}}"#,
            future(3600)
        ))
        .expect("should render");
        assert!(out.contains("75%"), "{out}");
        assert_eq!(out.matches('%').count(), 1, "absent window shown: {out}");
    }

    #[test]
    fn test_only_seven_day_window() {
        let out = render_json(&format!(
            r#"{{"seven_day":{{"used_percentage":80,"resets_at":{}}}}}"#,
            future(86400)
        ))
        .expect("should render");
        assert!(out.contains("80%"), "{out}");
        assert_eq!(out.matches('%').count(), 1, "absent window shown: {out}");
    }

    #[test]
    fn test_empty_rate_limits() {
        assert_eq!(render_json("{}"), None);
    }

    #[test]
    fn test_expired_window_is_ignored() {
        // The expired 95% window must not drive the style or be displayed.
        let out = render_json(&format!(
            r#"{{"five_hour":{{"used_percentage":95,"resets_at":1}},"seven_day":{{"used_percentage":10,"resets_at":{}}}}}"#,
            future(86400)
        ));
        assert_eq!(out, None);
    }

    #[test]
    fn test_expired_window_hidden_when_other_is_visible() {
        let out = render_json(&format!(
            r#"{{"five_hour":{{"used_percentage":95,"resets_at":1}},"seven_day":{{"used_percentage":80,"resets_at":{}}}}}"#,
            future(86400)
        ))
        .expect("should render");
        assert!(out.contains("80%") && !out.contains("95%"), "{out}");
    }

    #[test]
    fn test_percentage_is_clamped() {
        let out = render_json(&format!(
            r#"{{"five_hour":{{"used_percentage":130,"resets_at":{}}}}}"#,
            future(3600)
        ))
        .expect("should render");
        assert!(out.contains("100%") && !out.contains("130"), "{out}");
    }

    #[test]
    fn test_short_duration() {
        assert_eq!(short_duration(45), "45s");
        assert_eq!(short_duration(720), "12m");
        assert_eq!(short_duration(3600), "1h");
        assert_eq!(short_duration(4985), "1h23m");
        assert_eq!(short_duration(86400), "1d");
        assert_eq!(short_duration(2 * 86400 + 7 * 3600 + 2000), "2d7h");
    }

    #[test]
    fn test_windows_styled_independently() {
        let data = make_data(75.0, 95.0, u64::MAX, u64::MAX);
        let out = ModuleRenderer::new("claude_usage")
            .claude_code_data(data)
            .collect()
            .expect("should render");
        let yellow = Color::Yellow.normal().prefix().to_string();
        let red = Color::Red.normal().prefix().to_string();
        assert_eq!(out.matches(&yellow).count(), 1, "{out}");
        assert_eq!(out.matches(&red).count(), 1, "{out}");
        assert!(out.find(&yellow) < out.find(&red), "{out}");
    }

    #[test]
    fn test_low_window_hidden_independently() {
        let data = make_data(75.0, 10.0, u64::MAX, u64::MAX);
        let out = ModuleRenderer::new("claude_usage")
            .claude_code_data(data)
            .collect()
            .expect("should render");
        assert!(out.contains("5h") && !out.contains("7d"), "{out}");
    }

    fn make_data(
        five_hour_percentage: f32,
        seven_day_percentage: f32,
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
            effort: None,
            rate_limits: Some(RateLimits {
                five_hour: Some(RateLimitWindow {
                    used_percentage: five_hour_percentage,
                    resets_at: five_hour_resets_at,
                }),
                seven_day: Some(RateLimitWindow {
                    used_percentage: seven_day_percentage,
                    resets_at: seven_day_resets_at,
                }),
            }),
        }
    }
}
