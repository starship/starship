use super::{Context, Module, ModuleConfig};
use crate::configs::claude_cost::ClaudeCostConfig;
use crate::formatter::StringFormatter;
use crate::utils::{humanize_int, render_time};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("claude_cost");
    let config = ClaudeCostConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    // Read Claude Code data from Context
    let claude_data = context.claude_code_data.as_ref()?;
    let cost_info = claude_data.cost.as_ref()?;
    let total_cost = cost_info.total_cost_usd;

    let display_style = config
        .display
        .iter()
        .filter(|s| total_cost >= (s.threshold as f64))
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
                    "cost" => Some(Ok(format!("{:.2}", total_cost))),
                    "duration" => Some(Ok(render_time(cost_info.total_duration_ms as u128, false))),
                    "api_duration" => Some(Ok(render_time(
                        cost_info.total_api_duration_ms as u128,
                        false,
                    ))),
                    "lines_added" => Some(Ok(humanize_int(cost_info.total_lines_added))),
                    "lines_removed" => Some(Ok(humanize_int(cost_info.total_lines_removed))),
                    _ => None,
                })
                .parse(None, Some(context))
        });

        module.set_segments(match parsed {
            Ok(segments) => segments,
            Err(error) => {
                log::warn!("Error in module `claude_cost`: {error}");
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
        let actual = ModuleRenderer::new("claude_cost").collect();
        assert_eq!(actual, None);
    }

    fn get_test_claude_data(total_cost_usd: f64) -> crate::context::ClaudeCodeData {
        crate::context::ClaudeCodeData {
            cwd: None,
            model: crate::context::ModelInfo {
                id: "claude-3-5-sonnet".to_string(),
                display_name: "Claude 3.5 Sonnet".to_string(),
            },
            context_window: crate::context::ContextWindow {
                context_window_size: 200000,
                total_input_tokens: 1000,
                total_output_tokens: 500,
                current_usage: crate::context::CurrentUsage {
                    input_tokens: 100,
                    output_tokens: 50,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                },
            },
            cost: Some(crate::context::CostInfo {
                total_cost_usd,
                total_duration_ms: 60000,
                total_api_duration_ms: 45000,
                total_lines_added: 1200,
                total_lines_removed: 500,
            }),
            workspace: None,
        }
    }

    #[test]
    fn test_render_with_data() {
        let data = get_test_claude_data(1.234);

        let actual = ModuleRenderer::new("claude_cost")
            .config(toml::toml! {
                [claude_cost]
                format = "[$symbol(\\$$cost) (\\(+ $lines_added - $lines_removed\\))]($style) "
                [[claude_cost.display]]
                threshold = 0.0
                style = "bold yellow"
            })
            .claude_code_data(data)
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Yellow.bold().paint("💰 $1.23 (+ 1.2k - 500)")
        ));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_cost_below_threshold() {
        let data = get_test_claude_data(0.5);

        let actual = ModuleRenderer::new("claude_cost")
            .config(toml::toml! {
                [claude_cost]
                [[claude_cost.display]]
                threshold = 1.0
                style = "bold yellow"
            })
            .claude_code_data(data)
            .collect();

        assert_eq!(actual, None);
    }

    #[test]
    fn test_multiple_thresholds() {
        let data_low = get_test_claude_data(0.5);
        let data_medium = get_test_claude_data(2.5);
        let data_high = get_test_claude_data(5.5);

        let config = toml::toml! {
            [claude_cost]
            format = "[$symbol(\\$$cost)]($style) "
            [[claude_cost.display]]
            threshold = 0.0
            style = "bold green"
            [[claude_cost.display]]
            threshold = 2.0
            style = "bold yellow"
            [[claude_cost.display]]
            threshold = 5.0
            style = "bold red"
        };

        let actual_low = ModuleRenderer::new("claude_cost")
            .config(config.clone())
            .claude_code_data(data_low)
            .collect();
        let expected_low = Some(format!("{} ", Color::Green.bold().paint("💰 $0.50")));
        assert_eq!(actual_low, expected_low);

        let actual_medium = ModuleRenderer::new("claude_cost")
            .config(config.clone())
            .claude_code_data(data_medium)
            .collect();
        let expected_medium = Some(format!("{} ", Color::Yellow.bold().paint("💰 $2.50")));
        assert_eq!(actual_medium, expected_medium);

        let actual_high = ModuleRenderer::new("claude_cost")
            .config(config)
            .claude_code_data(data_high)
            .collect();
        let expected_high = Some(format!("{} ", Color::Red.bold().paint("💰 $5.50")));
        assert_eq!(actual_high, expected_high);
    }
}
