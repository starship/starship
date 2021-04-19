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
        .unwrap_or(&"0".into())
        .trim()
        .parse::<i64>()
        .ok()?;
    if num_of_jobs == 0 {
        return None;
    }

    let module_number = if num_of_jobs > config.threshold {
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
    use crate::test::ModuleRenderer;
    use ansi_term::Color;

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
}
