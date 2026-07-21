use super::{Context, Module, ModuleConfig};

use crate::configs::jj_metrics::JJMetricsConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the JJ metrics in the current repository
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_metrics");
    let config = JJMetricsConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let current_change = context.get_jj_repo()?.current_change(context)?;

    let render_value = |value: u32| {
        if config.only_nonzero_diffs && value == 0 {
            return None;
        }

        Some(Ok(value.to_string()))
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "added_style" => Some(Ok(config.added_style)),
                "deleted_style" => Some(Ok(config.deleted_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "added" => render_value(current_change.lines_added),
                "deleted" => render_value(current_change.lines_removed),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj_metrics`:\n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;
    use toml::toml;

    use crate::context::JJRepo;

    use crate::modules::jj_bookmark::tests::Tester;

    fn tester(repo: &'static str) -> Tester {
        Tester::new("jj_metrics").repo(repo)
    }

    #[test]
    fn test_render_basics() {
        Tester::basic_tests("jj_metrics");
    }

    #[test]
    fn test_render_default_config() {
        tester(JJRepo::BASE)
            .expected(format!(
                "{} {} ",
                Color::Green.bold().paint("+100"),
                Color::Red.bold().paint("-90")
            ))
            .render();
    }

    #[test]
    fn test_render_style() {
        tester(JJRepo::BASE)
            .options(toml! {
                added_style = "italic blue"
                deleted_style = "italic yellow"
            })
            .expected(format!(
                "{} {} ",
                Color::Blue.italic().paint("+100"),
                Color::Yellow.italic().paint("-90")
            ))
            .render();
    }

    #[test]
    fn test_render_format() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "(-$deleted)/(+$added)"
            })
            .expected("-90/+100")
            .render();
    }

    #[test]
    fn test_render_only_added() {
        tester(JJRepo::METRIC_ADDED)
            .options(toml! {
                format = "(+$added )(-$deleted )"
            })
            .expected("+100 ")
            .render();

        tester(JJRepo::METRIC_ADDED)
            .options(toml! {
                format = "(+$added )(-$deleted )"
                only_nonzero_diffs = false
            })
            .expected("+100 -0 ")
            .render();
    }

    #[test]
    fn test_render_only_deleted() {
        tester(JJRepo::METRIC_DELETED)
            .options(toml! {
                format = "(+$added )(-$deleted )"
            })
            .expected("-90 ")
            .render();

        tester(JJRepo::METRIC_DELETED)
            .options(toml! {
                format = "(+$added )(-$deleted )"
                only_nonzero_diffs = false
            })
            .expected("+0 -90 ")
            .render();
    }

    #[test]
    fn test_render_zero_diffs() {
        tester(JJRepo::METRIC_ZERO)
            .options(toml! {
                format = "(+$added )(-$deleted )"
                only_nonzero_diffs = false
            })
            .expected("+0 -0 ")
            .render();

        tester(JJRepo::METRIC_ZERO).render();
    }
}
