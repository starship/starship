use super::{Context, Module, RootModuleConfig};

use crate::configs::jobs::JobsConfig;
use crate::formatter::StringFormatter;

/// Creates a segment to show if there are any active jobs running
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jobs");
    let config = JobsConfig::try_load(module.config);

    let props = &context.properties;
    let num_of_jobs = props
        .get("jobs")
        .map(String::as_str)
        .unwrap_or("0")
        .trim()
        .parse::<i64>()
        .ok()?;

    if config.threshold < 0 {
        log::warn!(
            "threshold in [jobs] ({}) was less than zero",
            config.threshold
        );
        return None;
    }

    if num_of_jobs == 0 && config.threshold > 0 {
        return None;
    }

    let module_number = if num_of_jobs > config.threshold || config.threshold == 0 {
        num_of_jobs.to_string()
    } else {
        "".to_string()
    };

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
                "number" => Some(Ok(module_number.clone())),
                _ => None,
            })
            .parse(None)
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
    use crate::test::TestRenderer;
    use ansi_term::Color;

    #[test]
    fn config_blank_job_0() {
        let actual = TestRenderer::new().jobs(0).module("jobs");

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_job_1() {
        let actual = TestRenderer::new().jobs(1).module("jobs");

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_job_2() {
        let actual = TestRenderer::new().jobs(2).module("jobs");

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦2")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_2_job_2() {
        let actual = TestRenderer::new()
            .config(toml::toml! {
                [jobs]
                threshold = 2
            })
            .jobs(2)
            .module("jobs");

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_2_job_3() {
        let actual = TestRenderer::new()
            .config(toml::toml! {
                [jobs]
                threshold = 2
            })
            .jobs(3)
            .module("jobs");

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦3")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_0_job_0() {
        let actual = TestRenderer::new()
            .config(toml::toml! {
                [jobs]
                threshold = 0
            })
            .jobs(0)
            .module("jobs");

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦0")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_0_job_1() {
        let actual = TestRenderer::new()
            .config(toml::toml! {
                [jobs]
                threshold = 0
            })
            .jobs(1)
            .module("jobs");

        let expected = Some(format!("{} ", Color::Blue.bold().paint("✦1")));
        assert_eq!(expected, actual);
    }
}
