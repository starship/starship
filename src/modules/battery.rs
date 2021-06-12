use super::{Context, Module, RootModuleConfig, Shell};
use crate::configs::battery::BatteryConfig;
#[cfg(test)]
use mockall::automock;

use crate::formatter::StringFormatter;

/// Creates a module for the battery percentage and charging state
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // TODO: Update when v1.0 printing refactor is implemented to only
    // print escapes in a prompt context.
    let percentage_char = match context.shell {
        Shell::Zsh => "%%", // % is an escape in zsh, see PROMPT in `man zshmisc`
        _ => "%",
    };

    let battery_status = context.battery_status_provider.get_battery_status()?;
    let BatteryStatus { state, percentage } = battery_status;

    let mut module = context.new_module("battery");
    let config: BatteryConfig = BatteryConfig::try_load(module.config);

    // Parse config under `display`.
    // Select the first style that match the threshold,
    // if all thresholds are lower do not display battery module.
    let display_style = config
        .display
        .iter()
        .find(|display_style| percentage <= display_style.threshold as f32)?;

    // Parse the format string and build the module
    match StringFormatter::new(config.format) {
        Ok(formatter) => {
            let formatter = formatter
                .map_meta(|variable, _| match variable {
                    "symbol" => match state {
                        battery::State::Full => Some(config.full_symbol),
                        battery::State::Charging => display_style
                            .charging_symbol
                            .or(Some(config.charging_symbol)),
                        battery::State::Discharging => display_style
                            .discharging_symbol
                            .or(Some(config.discharging_symbol)),
                        battery::State::Unknown => Some(config.unknown_symbol),
                        battery::State::Empty => Some(config.empty_symbol),
                        _ => {
                            log::debug!("Unhandled battery state `{}`", state);
                            None
                        }
                    },
                    _ => None,
                })
                .map_style(|style| match style {
                    "style" => Some(Ok(display_style.style)),
                    _ => None,
                })
                .map(|variable| match variable {
                    "percentage" => Some(Ok(format!("{}{}", percentage.round(), percentage_char))),
                    _ => None,
                });

            match formatter.parse(None) {
                Ok(format_string) => {
                    module.set_segments(format_string);
                    Some(module)
                }
                Err(e) => {
                    log::warn!("Cannot parse `battery.format`: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            log::warn!("Cannot load `battery.format`: {}", e);
            None
        }
    }
}

/// the merge returns Charging if at least one is charging
///                   Discharging if at least one is Discharging
///                   Full if both are Full or one is Full and the other Unknow
///                   Empty if both are Empty or one is Empty and the other Unknow
///                   Unknown otherwise
fn merge_battery_states(state1: battery::State, state2: battery::State) -> battery::State {
    use battery::State::{Charging, Discharging, Unknown};
    if state1 == Charging || state2 == Charging {
        Charging
    } else if state1 == Discharging || state2 == Discharging {
        Discharging
    } else if state1 == state2 {
        state1
    } else if state1 == Unknown {
        state2
    } else if state2 == Unknown {
        state1
    } else {
        Unknown
    }
}

struct BatteryInfo {
    energy: f32,
    energy_full: f32,
    state: battery::State,
}

#[derive(Debug)]
pub struct BatteryStatus {
    percentage: f32,
    state: battery::State,
}

#[cfg_attr(test, automock)]
pub trait BatteryStatusProvider {
    fn get_battery_status(&self) -> Option<BatteryStatus>;
}

pub struct BatteryStatusProviderImpl;

impl BatteryStatusProvider for BatteryStatusProviderImpl {
    fn get_battery_status(&self) -> Option<BatteryStatus> {
        let battery_manager = battery::Manager::new().ok()?;
        let batteries = battery_manager.batteries().ok()?;
        let battery_contructor = batteries
            .filter_map(|battery| match battery {
                Ok(battery) => {
                    log::debug!("Battery found: {:?}", battery);
                    Some(BatteryInfo {
                        energy: battery.energy().value,
                        energy_full: battery.energy_full().value,
                        state: battery.state(),
                    })
                }
                Err(e) => {
                    let level = if cfg!(target_os = "linux") {
                        log::Level::Info
                    } else {
                        log::Level::Warn
                    };
                    log::log!(level, "Unable to access battery information:\n{}", &e);
                    None
                }
            })
            .fold(
                BatteryInfo {
                    energy: 0.0,
                    energy_full: 0.0,
                    state: battery::State::Unknown,
                },
                |mut acc, x| {
                    acc.energy += x.energy;
                    acc.energy_full += x.energy_full;
                    acc.state = merge_battery_states(acc.state, x.state);
                    acc
                },
            );
        if battery_contructor.energy_full != 0.0 {
            let battery = BatteryStatus {
                percentage: battery_contructor.energy / battery_contructor.energy_full * 100.0,
                state: battery_contructor.state,
            };
            log::debug!("Battery status: {:?}", battery);
            Some(battery)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;

    #[test]
    fn no_battery_status() {
        let mut mock = MockBatteryStatusProvider::new();

        mock.expect_get_battery_status().times(1).returning(|| None);

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_status_provider(&mock)
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_full() {
        let mut mock = MockBatteryStatusProvider::new();

        mock.expect_get_battery_status().times(1).returning(|| {
            Some(BatteryStatus {
                percentage: 100.0,
                state: battery::State::Full,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_status_provider(&mock)
            .collect();
        let expected = Some(String::from(" 100% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_charging() {
        let mut mock = MockBatteryStatusProvider::new();

        mock.expect_get_battery_status().times(1).returning(|| {
            Some(BatteryStatus {
                percentage: 80.0,
                state: battery::State::Charging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 90
                style = ""
            })
            .battery_status_provider(&mock)
            .collect();
        let expected = Some(String::from(" 80% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_discharging() {
        let mut mock = MockBatteryStatusProvider::new();

        mock.expect_get_battery_status().times(1).returning(|| {
            Some(BatteryStatus {
                percentage: 80.0,
                state: battery::State::Discharging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_status_provider(&mock)
            .collect();
        let expected = Some(String::from(" 80% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_unknown() {
        let mut mock = MockBatteryStatusProvider::new();

        mock.expect_get_battery_status().times(1).returning(|| {
            Some(BatteryStatus {
                percentage: 0.0,
                state: battery::State::Unknown,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_status_provider(&mock)
            .collect();
        let expected = Some(String::from(" 0% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_empty() {
        let mut mock = MockBatteryStatusProvider::new();

        mock.expect_get_battery_status().times(1).returning(|| {
            Some(BatteryStatus {
                percentage: 0.0,
                state: battery::State::Empty,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_status_provider(&mock)
            .collect();
        let expected = Some(String::from(" 0% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_hidden_when_percentage_above_threshold() {
        let mut mock = MockBatteryStatusProvider::new();

        mock.expect_get_battery_status().times(1).returning(|| {
            Some(BatteryStatus {
                percentage: 60.0,
                state: battery::State::Empty,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 50
                style = ""
            })
            .battery_status_provider(&mock)
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_uses_style() {
        let mut mock = MockBatteryStatusProvider::new();

        mock.expect_get_battery_status().times(1).returning(|| {
            Some(BatteryStatus {
                percentage: 40.0,
                state: battery::State::Discharging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 50
                style = "bold red"
            })
            .battery_status_provider(&mock)
            .collect();
        let expected = Some(format!("{} ", Color::Red.bold().paint(" 40%")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_displayed_precision() {
        let mut mock = MockBatteryStatusProvider::new();

        mock.expect_get_battery_status().times(1).returning(|| {
            Some(BatteryStatus {
                percentage: 12.987654321,
                state: battery::State::Discharging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_status_provider(&mock)
            .collect();
        let expected = Some(String::from(" 13% "));

        assert_eq!(expected, actual);
    }
}
