use super::{Context, Module, ModuleConfig};
use crate::configs::claude_context::ClaudeDisplayConfig;
use crate::configs::claude_rate_limits::ClaudeRateLimitsConfig;
use crate::context::RateLimitWindow;
use crate::formatter::StringFormatter;
use crate::utils::render_gauge;
use jiff::Timestamp;
use std::cmp::Ordering;

/// Usage of a window, guarded against a payload reporting a percentage outside 0-100.
fn used(window: &RateLimitWindow) -> f32 {
    window.used_percentage.clamp(0.0, 100.0)
}

/// A window paired with the `display` entry that styles it: the highest threshold at or below its
/// own usage, so both windows share one configuration but are styled independently. `None` when
/// the window was not reported, when no threshold matches it, or when the match is `hidden`.
fn shown<'a>(
    display: &'a [ClaudeDisplayConfig<'a>],
    window: Option<&'a RateLimitWindow>,
) -> Option<(&'a RateLimitWindow, &'a ClaudeDisplayConfig<'a>)> {
    let window = window?;
    // TODO: this selection is duplicated in `claude_context` and `claude_cost`; it can be
    // hoisted onto `ClaudeDisplayConfig`.
    let style = display
        .iter()
        .filter(|s| used(window) >= s.threshold)
        .max_by(|a, b| {
            a.threshold
                .partial_cmp(&b.threshold)
                .unwrap_or(Ordering::Equal)
        })
        .filter(|s| !s.hidden)?;
    Some((window, style))
}

/// Time left until a window resets, as its two most significant units (`2d`, `1h30m`, `45m`).
/// Empty when the reset time is unknown (absent from the payload), already past, or under a
/// minute away. `utils::render_time` cannot be reused: it appends every lesser unit down to
/// seconds, with no way to cap the output at two of them.
fn render_reset(resets_at: i64, now: i64) -> String {
    let minutes = u64::try_from(resets_at.saturating_sub(now)).unwrap_or(0) / 60;
    [
        ("d", minutes / 1440),
        ("h", minutes / 60 % 24),
        ("m", minutes % 60),
    ]
    .iter()
    .filter(|(_, amount)| *amount != 0)
    .take(2)
    .map(|(unit, amount)| format!("{amount}{unit}"))
    .collect()
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("claude_rate_limits");
    let config = ClaudeRateLimitsConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    // Only reported for Claude.ai subscriptions, and only after the first API response
    let rate_limits = context.claude_code_data.as_ref()?.rate_limits.as_ref()?;
    let five_hour = shown(&config.display, rate_limits.five_hour.as_ref());
    let seven_day = shown(&config.display, rate_limits.seven_day.as_ref());

    // `$style` covers what sits outside either window's group, so it follows the window closest
    // to its limit among those being shown. With neither shown there is nothing left to render.
    let display_style = [five_hour, seven_day]
        .into_iter()
        .flatten()
        .max_by(|(a, _), (b, _)| used(a).partial_cmp(&used(b)).unwrap_or(Ordering::Equal))
        .map(|(_, style)| style)?;

    // A hidden window is left as unset as one that was never reported, which drops its group
    let five_hour_window = five_hour.map(|(window, _)| window);
    let seven_day_window = seven_day.map(|(window, _)| window);

    let now = Timestamp::now().as_second();
    let percentage = |window: Option<&RateLimitWindow>| {
        window.map(|w| Ok(format!("{}%", used(w).round() as u8)))
    };
    let gauge = |window: Option<&RateLimitWindow>| {
        window.map(|w| {
            Ok(render_gauge(
                f64::from(used(w).round()),
                config.gauge_width,
                config.gauge_full_symbol,
                config.gauge_partial_symbol,
                config.gauge_empty_symbol,
            ))
        })
    };
    let reset =
        |window: Option<&RateLimitWindow>| window.map(|w| Ok(render_reset(w.resets_at, now)));

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(display_style.style)),
                "five_hour_style" => five_hour.map(|(_, style)| Ok(style.style)),
                "seven_day_style" => seven_day.map(|(_, style)| Ok(style.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "five_hour_percentage" => percentage(five_hour_window),
                "five_hour_gauge" => gauge(five_hour_window),
                "five_hour_reset" => reset(five_hour_window),
                "seven_day_percentage" => percentage(seven_day_window),
                "seven_day_gauge" => gauge(seven_day_window),
                "seven_day_reset" => reset(seven_day_window),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `claude_rate_limits`: {error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use super::{render_reset, used};
    use crate::context::{ClaudeCodeData, RateLimitWindow, RateLimits};
    use crate::test::ModuleRenderer;
    use jiff::Timestamp;
    use nu_ansi_term::{AnsiStrings, Color};

    fn get_test_claude_data(
        five_hour: Option<RateLimitWindow>,
        seven_day: Option<RateLimitWindow>,
    ) -> ClaudeCodeData {
        ClaudeCodeData {
            rate_limits: Some(RateLimits {
                five_hour,
                seven_day,
            }),
            ..Default::default()
        }
    }

    fn window(used_percentage: f32) -> Option<RateLimitWindow> {
        Some(RateLimitWindow {
            used_percentage,
            resets_at: 0,
        })
    }

    #[test]
    fn test_without_data() {
        let actual = ModuleRenderer::new("claude_rate_limits").collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn test_without_rate_limits() {
        let actual = ModuleRenderer::new("claude_rate_limits")
            .claude_code_data(ClaudeCodeData::default())
            .collect();
        assert_eq!(
            actual, None,
            "module should be hidden for sessions without rate limit data"
        );
    }

    #[test]
    fn test_without_any_window() {
        let actual = ModuleRenderer::new("claude_rate_limits")
            .claude_code_data(get_test_claude_data(None, None))
            .collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn test_disabled() {
        let actual = ModuleRenderer::new("claude_rate_limits")
            .config(toml::toml! {
                [claude_rate_limits]
                disabled = true
            })
            .claude_code_data(get_test_claude_data(window(90.0), window(90.0)))
            .collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn test_render_with_data() {
        let actual = ModuleRenderer::new("claude_rate_limits")
            .claude_code_data(get_test_claude_data(window(94.5), window(72.0)))
            .collect();

        assert_eq!(
            actual,
            Some(format!(
                "{} ",
                AnsiStrings(&[
                    Color::Red.bold().paint("⏳ 5h 95%"),
                    Color::Yellow.bold().paint(" 7d 72%"),
                ])
            )),
            "each window should be styled by its own usage, the symbol by the busier one"
        );
    }

    #[test]
    fn test_hidden_window_drops_its_group() {
        let actual = ModuleRenderer::new("claude_rate_limits")
            .claude_code_data(get_test_claude_data(window(23.5), window(95.0)))
            .collect();

        assert_eq!(
            actual,
            Some(format!("{} ", Color::Red.bold().paint("⏳ 7d 95%"))),
            "a window below the lowest visible threshold should render like a missing one"
        );
    }

    #[test]
    fn test_hidden_below_threshold() {
        let actual = ModuleRenderer::new("claude_rate_limits")
            .claude_code_data(get_test_claude_data(window(23.5), window(41.2)))
            .collect();
        assert_eq!(
            actual, None,
            "module should be hidden below the 50% threshold"
        );
    }

    #[test]
    fn test_missing_window_drops_its_group() {
        let actual = ModuleRenderer::new("claude_rate_limits")
            .claude_code_data(get_test_claude_data(None, window(55.0)))
            .collect();

        assert_eq!(
            actual,
            Some(format!("{} ", Color::Green.bold().paint("⏳ 7d 55%")))
        );
    }

    // Reads the clock twice, as `aws::tests::expiration_date_set` does; the 30s of slack before
    // the rendered `1h` would tick down to `59m` is ample for an in-process render
    #[test]
    fn test_gauge_and_reset() {
        let data = get_test_claude_data(
            Some(RateLimitWindow {
                used_percentage: 55.0,
                resets_at: Timestamp::now().as_second() + 3630,
            }),
            window(75.0),
        );

        let actual = ModuleRenderer::new("claude_rate_limits")
            .config(toml::toml! {
                [claude_rate_limits]
                format = "[$five_hour_gauge $five_hour_reset]($five_hour_style)|[$seven_day_gauge $seven_day_reset]($seven_day_style) "
            })
            .claude_code_data(data)
            .collect();

        assert_eq!(
            actual,
            Some(format!(
                "{} ",
                AnsiStrings(&[
                    Color::Green.bold().paint("██▒░░ 1h"),
                    nu_ansi_term::Style::default().paint("|"),
                    Color::Yellow.bold().paint("███▒░ "),
                ])
            )),
            "an unknown reset time should render as nothing"
        );
    }

    #[test]
    fn test_render_with_full_window() {
        let actual = ModuleRenderer::new("claude_rate_limits")
            .claude_code_data(get_test_claude_data(window(95.0), window(91.0)))
            .collect();

        assert_eq!(
            actual,
            Some(format!("{} ", Color::Red.bold().paint("⏳ 5h 95% 7d 91%")))
        );
    }

    #[test]
    fn test_threshold_is_inclusive() {
        for (used, color) in [
            (50.0, Color::Green),
            (70.0, Color::Yellow),
            (90.0, Color::Red),
        ] {
            let actual = ModuleRenderer::new("claude_rate_limits")
                .claude_code_data(get_test_claude_data(window(used), None))
                .collect();

            assert_eq!(
                actual,
                Some(format!(
                    "{} ",
                    color.bold().paint(format!("⏳ 5h {used:.0}%"))
                )),
                "usage at exactly {used} should match the {used} threshold"
            );
        }
    }

    #[test]
    fn test_render_reset_days() {
        let data = get_test_claude_data(
            window(20.0),
            Some(RateLimitWindow {
                used_percentage: 55.0,
                resets_at: Timestamp::now().as_second() + 176_400,
            }),
        );

        let actual = ModuleRenderer::new("claude_rate_limits")
            .config(toml::toml! {
                [claude_rate_limits]
                format = "[7d $seven_day_reset]($style) "
            })
            .claude_code_data(data)
            .collect();

        assert_eq!(
            actual,
            Some(format!("{} ", Color::Green.bold().paint("7d 2d1h")))
        );
    }

    #[test]
    fn test_used_clamps_out_of_range_percentages() {
        assert_eq!(used(&RateLimitWindow::default()), 0.0);
        assert_eq!(used(&window(-5.0).unwrap()), 0.0);
        assert_eq!(used(&window(150.0).unwrap()), 100.0);
    }

    #[test]
    fn test_render_reset() {
        assert_eq!(render_reset(1_000_000, 1_000_000 - 5_400), "1h30m");
        assert_eq!(render_reset(1_000_000, 1_000_000 - 176_400), "2d1h");
        assert_eq!(render_reset(1_000_000, 1_000_000 - 174_600), "2d30m");
        assert_eq!(render_reset(1_000_000, 1_000_000 - 59), "");
        assert_eq!(render_reset(1_000_000, 1_000_001), "");
    }
}
