use super::{Context, Module, ModuleConfig};
use crate::configs::claude_context::ClaudeContextConfig;
use crate::formatter::StringFormatter;
use crate::utils::humanize_int;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("claude_context");
    let config = ClaudeContextConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    // Read Claude Code data from Context
    let claude_data = context.claude_code_data.as_ref()?;

    // Calculate context usage percentage
    let current_tokens = claude_data.context_window.total_input_tokens
        + claude_data.context_window.total_output_tokens;
    let total_tokens = claude_data.context_window.context_window_size;

    let percentage_float = if total_tokens > 0 {
        (current_tokens as f32 / total_tokens as f32) * 100.0
    } else {
        0.0
    };
    let percentage = percentage_float.round().clamp(0.0, 100.0) as u8;

    // Determine style based on percentage
    let display_style = config
        .display
        .iter()
        .filter(|s| percentage_float >= s.threshold)
        .max_by(|a, b| {
            a.threshold
                .partial_cmp(&b.threshold)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

    if display_style.is_some_and(|s| s.hidden) {
        return None;
    }

    if let Some(display_style) = display_style {
        let parsed = StringFormatter::new(config.format).and_then(|formatter| {
            formatter
                .map_meta(|variable, _| match variable {
                    "symbol" => Some(config.symbol),
                    _ => None,
                })
                .map_style(|variable| match variable {
                    "style" => Some(Ok(display_style.style)),
                    _ => None,
                })
                .map(|variable| match variable {
                    "gauge" => {
                        let filled_float = (percentage as f64 / 100.0) * config.gauge_width as f64;
                        let (filled_count, partial) = if !config.gauge_partial_symbol.is_empty() {
                            let full = filled_float.floor() as usize;
                            let rem = filled_float - full as f64;
                            // Show partial block if remainder is significant enough (> 0.25)
                            // but if it's very close to 1 (> 0.75), just round up to full (handled by empty_count check)
                            if rem >= 0.25 {
                                (full, true)
                            } else {
                                (full, false)
                            }
                        } else {
                            (filled_float.round() as usize, false)
                        };

                        let filled_count = filled_count.min(config.gauge_width as usize);
                        let partial_count = if partial && filled_count < config.gauge_width as usize
                        {
                            1
                        } else {
                            0
                        };
                        let empty_count = (config.gauge_width as usize)
                            .saturating_sub(filled_count + partial_count);

                        let gauge = config.gauge_full_symbol.repeat(filled_count)
                            + &config.gauge_partial_symbol.repeat(partial_count)
                            + &config.gauge_empty_symbol.repeat(empty_count);
                        Some(Ok(gauge))
                    }
                    "percentage" => Some(Ok(format!("{percentage}%"))),
                    "input_tokens" => Some(Ok(humanize_int(
                        claude_data.context_window.total_input_tokens,
                    ))),
                    "output_tokens" => Some(Ok(humanize_int(
                        claude_data.context_window.total_output_tokens,
                    ))),
                    "curr_input_tokens" => Some(Ok(humanize_int(
                        claude_data.context_window.current_usage.input_tokens,
                    ))),
                    "curr_output_tokens" => Some(Ok(humanize_int(
                        claude_data.context_window.current_usage.output_tokens,
                    ))),
                    "curr_cache_creation_tokens" => Some(Ok(humanize_int(
                        claude_data
                            .context_window
                            .current_usage
                            .cache_creation_input_tokens,
                    ))),
                    "curr_cache_read_tokens" => Some(Ok(humanize_int(
                        claude_data
                            .context_window
                            .current_usage
                            .cache_read_input_tokens,
                    ))),
                    "total_tokens" => Some(Ok(humanize_int(total_tokens))),
                    _ => None,
                })
                .parse(None, Some(context))
        });

        module.set_segments(match parsed {
            Ok(segments) => segments,
            Err(error) => {
                log::warn!("Error in module `claude_context`: {error}");
                return None;
            }
        });

        Some(module)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn test_without_data() {
        let actual = ModuleRenderer::new("claude_context").collect();
        assert_eq!(actual, None);
    }

    fn get_test_claude_data(
        total_input_tokens: u64,
        total_output_tokens: u64,
    ) -> crate::context::ClaudeCodeData {
        crate::context::ClaudeCodeData {
            cwd: None,
            model: crate::context::ModelInfo {
                id: "claude-3-5-sonnet".to_string(),
                display_name: "Claude 3.5 Sonnet".to_string(),
            },
            context_window: crate::context::ContextWindow {
                context_window_size: 200000,
                total_input_tokens,
                total_output_tokens,
                current_usage: crate::context::CurrentUsage {
                    input_tokens: 1000,
                    output_tokens: 500,
                    cache_creation_input_tokens: 1000,
                    cache_read_input_tokens: 2000,
                },
            },
            cost: None,
            workspace: None,
        }
    }

    #[test]
    fn test_render_with_data() {
        let data = get_test_claude_data(100000, 0); // 50%

        let actual = ModuleRenderer::new("claude_context")
            .config(toml::toml! {
                [claude_context.display]
                threshold = 0
                style = "bold green"
            })
            .claude_code_data(data)
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("▰▰▰▱▱ 50%")));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiple_thresholds() {
        let data_low = get_test_claude_data(50000, 0); // 25%
        let data_medium = get_test_claude_data(130000, 0); // 65%
        let data_high = get_test_claude_data(170000, 0); // 85%

        let config = toml::toml! {
            [claude_context]
            format = "[$gauge]($style) "
            [[claude_context.display]]
            threshold = 0
            style = "bold green"
            [[claude_context.display]]
            threshold = 60
            style = "bold yellow"
            [[claude_context.display]]
            threshold = 80
            style = "bold red"
        };

        let actual_low = ModuleRenderer::new("claude_context")
            .config(config.clone())
            .claude_code_data(data_low)
            .collect();
        let expected_low = Some(format!("{} ", Color::Green.bold().paint("▰▱▱▱▱")));
        assert_eq!(actual_low, expected_low);

        let actual_medium = ModuleRenderer::new("claude_context")
            .config(config.clone())
            .claude_code_data(data_medium)
            .collect();
        let expected_medium = Some(format!("{} ", Color::Yellow.bold().paint("▰▰▰▱▱")));
        assert_eq!(actual_medium, expected_medium);

        let actual_high = ModuleRenderer::new("claude_context")
            .config(config)
            .claude_code_data(data_high)
            .collect();
        let expected_high = Some(format!("{} ", Color::Red.bold().paint("▰▰▰▰▱")));
        assert_eq!(actual_high, expected_high);
    }

    #[test]
    fn test_gauge_width() {
        let data = get_test_claude_data(100000, 0); // 50%

        let actual = ModuleRenderer::new("claude_context")
            .config(toml::toml! {
                [claude_context]
                gauge_width = 10
                [[claude_context.display]]
                threshold = 0
                style = "bold green"
            })
            .claude_code_data(data)
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("▰▰▰▰▰▱▱▱▱▱ 50%")));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_partial_gauge_symbols() {
        let data = get_test_claude_data(55000, 0); // 27.5%

        let actual = ModuleRenderer::new("claude_context")
            .config(toml::toml! {
                [claude_context]
                gauge_full_symbol = "█"
                gauge_partial_symbol = "▒"
                gauge_empty_symbol = "░"
                gauge_width = 10
                [[claude_context.display]]
                threshold = 0
                style = "bold green"
            })
            .claude_code_data(data)
            .collect();

        // 27.5% of 10 is 2.75.
        // Full: 2 (floor of 2.75)
        // Rem: 0.75 -> >= 0.25 so partial is true
        // Gauge: 2 full + 1 partial + 7 empty = 10 total
        let expected = Some(format!("{} ", Color::Green.bold().paint("██▒░░░░░░░ 28%")));
        assert_eq!(actual, expected);
    }
}
