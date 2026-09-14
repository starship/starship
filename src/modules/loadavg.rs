use systemstat::{LoadAverage, Platform, System};

use super::{Context, Module, ModuleConfig};

use crate::configs::loadavg::{LoadavgConfig, LoadavgDisplayConfig};
use crate::formatter::StringFormatter;

// Check whether a display config is active
fn is_active(config: &LoadavgDisplayConfig, loadavg: &LoadAverage) -> bool {
    config.threshold_one <= loadavg.one
        || config.threshold_five <= loadavg.five
        || config.threshold_fifteen <= loadavg.fifteen
}

/// Creates a module with system load average information
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("loadavg");
    let config = LoadavgConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    }

    let system = System::new();

    let loadavg = match system.load_average() {
        Ok(load) => load,
        Err(e) => {
            log::warn!("Failed to retrieve loadavg: {}", e);
            return None;
        }
    };

    // Parse config under `display`.
    // Select the first style that match any threshold,
    // if all thresholds are lower do not display loadavg module.
    let display_config = config
        .display
        .iter()
        .find(|display_style| is_active(display_style, &loadavg))?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => display_config.symbol.or(Some(config.symbol)),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(display_config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "one" => Some(Ok(format!("{:.2}", loadavg.one))),
                "five" => Some(Ok(format!("{:.2}", loadavg.five))),
                "fifteen" => Some(Ok(format!("{:.2}", loadavg.fifteen))),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `loadavg`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test::ModuleRenderer;

    #[test]
    fn test_is_active() {
        let mut config: LoadavgDisplayConfig = LoadavgDisplayConfig::default();
        let loadavg: LoadAverage = LoadAverage {
            one: 1.0,
            five: 4.0,
            fifteen: 8.0,
        };

        assert!(!is_active(&config, &loadavg));

        config.threshold_one = 1.1;
        assert!(!is_active(&config, &loadavg));

        config.threshold_one = 1.0;
        assert!(is_active(&config, &loadavg));

        config.threshold_one = f32::NAN;
        config.threshold_fifteen = 8.1;
        assert!(!is_active(&config, &loadavg));

        config.threshold_fifteen = 8.0;
        assert!(is_active(&config, &loadavg));
    }

    #[test]
    fn zero_threshold() {
        let output = ModuleRenderer::new("loadavg")
            .config(toml::toml! {
                [loadavg]
                disabled = false
                [[loadavg.display]]
                threshold_one = 0.0
            })
            .collect();

        if System::new().load_average().is_ok() {
            assert!(output.is_some())
        } else {
            assert!(output.is_none())
        }
    }

    #[test]
    fn impossible_threshold() {
        let output = ModuleRenderer::new("loadavg")
            .config(toml::toml! {
                [loadavg]
                disabled = false
                [[loadavg.display]]
                threshold_one = 9999.9
                [[loadavg.display]]
                threshold_five = 9999.9
                [[loadavg.display]]
                threshold_fifteen = 9999.9
            })
            .collect();

        assert!(output.is_none())
    }

    #[test]
    fn zero_last_threshold() {
        let output = ModuleRenderer::new("loadavg")
            .config(toml::toml! {
                [loadavg]
                disabled = false
                [[loadavg.display]]
                threshold_one = 9999.9
                [[loadavg.display]]
                threshold_five = 9999.9
                [[loadavg.display]]
                threshold_fifteen = 0.0
            })
            .collect();

        if System::new().load_average().is_ok() {
            assert!(output.is_some())
        } else {
            assert!(output.is_none())
        }
    }
}
