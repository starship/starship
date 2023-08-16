use super::{Context, Module, ModuleConfig};

use crate::configs::jobs::JobsConfig;
use crate::formatter::StringFormatter;

/// Creates a segment to show if there are any active jobs running
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jobs");
    let config = JobsConfig::try_load(module.config);

    if config.threshold < 0 {
        log::warn!(
            "threshold in [jobs] ({}) was less than zero",
            config.threshold
        );
        return None;
    }

    if config.symbol_threshold < 0 {
        log::warn!(
            "symbol_threshold in [jobs] ({}) was less than zero",
            config.symbol_threshold
        );
        return None;
    }

    if config.number_threshold < 0 {
        log::warn!(
            "number_threshold in [jobs] ({}) was less than zero",
            config.number_threshold
        );
        return None;
    }

    let props = &context.properties;
    let num_of_jobs = props.jobs;

    if num_of_jobs == 0
        && config.threshold > 0
        && config.number_threshold > 0
        && config.symbol_threshold > 0
    {
        return None;
    }

    let default_threshold = 1;
    let mut module_symbol = "";
    let mut module_number = String::new();
    if config.threshold != default_threshold {
        log::warn!("`threshold` in [jobs] is deprecated . Please remove it and use `symbol_threshold` and `number_threshold`.");

        // The symbol should be shown if there are *any* background
        // jobs running.
        if num_of_jobs > 0 {
            module_symbol = config.symbol;
        }

        if num_of_jobs > config.threshold || config.threshold == 0 {
            module_symbol = config.symbol;
            module_number = num_of_jobs.to_string();
        }
    } else {
        if num_of_jobs >= config.symbol_threshold {
            module_symbol = config.symbol;
        }

        if num_of_jobs >= config.number_threshold {
            module_number = num_of_jobs.to_string();
        }
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(module_symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "number" => Some(Ok(module_number.clone())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jobs`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod test {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn config_blank_job_0() {
        let actual = ModuleRenderer::new("jobs").jobs(0).collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_job_1() {
        let actual = ModuleRenderer::new("jobs").jobs(1).collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_job_2() {
        let actual = ModuleRenderer::new("jobs").jobs(2).collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦2")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_default_is_present_jobs_1() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 1
            })
            .jobs(1)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_default_is_present_jobs_2() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 1
            })
            .jobs(2)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦2")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_conflicting_thresholds_default_jobs_1() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 1
                number_threshold = 2
            })
            .jobs(1)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_conflicting_thresholds_default_no_symbol_jobs_1() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 1
                symbol_threshold = 0
                number_threshold = 2
            })
            .jobs(1)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_conflicting_thresholds_no_symbol_jobs_1() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 0
                symbol_threshold = 0
                number_threshold = 2
            })
            .jobs(1)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦1")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_conflicting_thresholds_jobs_2() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 1
                number_threshold = 2
            })
            .jobs(2)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦2")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_2_job_2() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 2
            })
            .jobs(2)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_number_2_job_2() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                number_threshold = 2
            })
            .jobs(2)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦2")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_number_2_symbol_3_job_2() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                number_threshold = 2
                symbol_threshold = 3
            })
            .jobs(2)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("2")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_2_job_3() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 2
            })
            .jobs(3)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦3")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_thresholds_0_jobs_0() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                number_threshold = 0
                symbol_threshold = 0
            })
            .jobs(0)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦0")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_thresholds_0_jobs_1() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                number_threshold = 0
                symbol_threshold = 0
            })
            .jobs(1)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦1")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_0_job_0() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 0
            })
            .jobs(0)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦0")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_0_job_1() {
        let actual = ModuleRenderer::new("jobs")
            .config(toml::toml! {
                [jobs]
                threshold = 0
            })
            .jobs(1)
            .collect();

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦1")));
        assert_eq!(expected, actual);
    }
}
