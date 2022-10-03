use super::{Context, Module, ModuleConfig};
use crate::configs::battery::BatteryConfig;
#[cfg(test)]
use mockall::automock;
use starship_battery as battery;

use crate::formatter::StringFormatter;

/// Creates a module for the battery percentage and charging state
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let battery_status = get_battery_status(context)?;
    let BatteryStatus { state, percentage } = battery_status;

    let mut module = context.new_module("battery");
    let config: BatteryConfig = BatteryConfig::try_load(module.config);

    // Parse config under `display`.
    // Select the first style that match the threshold,
    // if all thresholds are lower do not display battery module.
    let display_style = config.display.iter().find(|display_style| {
        if percentage <= display_style.threshold as f32 {
            return state == battery::State::Discharging || !display_style.only_on_discharge;
        }
        false
    })?;

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
                    "percentage" => Some(Ok(format!("{}%", percentage.round()))),
                    _ => None,
                });

            match formatter.parse(None, Some(context)) {
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

fn get_battery_status(context: &Context) -> Option<BatteryStatus> {
    let battery_info = context.battery_info_provider.get_battery_info()?;
    if battery_info.energy_full != 0.0 {
        let battery = BatteryStatus {
            percentage: battery_info.energy / battery_info.energy_full * 100.0,
            state: battery_info.state,
        };
        log::debug!("Battery status: {:?}", battery);
        Some(battery)
    } else {
        None
    }
}

/// the merge returns Charging if at least one is charging
///                   Discharging if at least one is Discharging
///                   Full if both are Full or one is Full and the other Unknown
///                   Empty if both are Empty or one is Empty and the other Unknown
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

pub struct BatteryInfo {
    energy: f32,
    energy_full: f32,
    state: battery::State,
}

#[derive(Debug)]
struct BatteryStatus {
    percentage: f32,
    state: battery::State,
}

#[cfg_attr(test, automock)]
pub trait BatteryInfoProvider {
    fn get_battery_info(&self) -> Option<BatteryInfo>;
}

pub struct BatteryInfoProviderImpl;

impl BatteryInfoProvider for BatteryInfoProviderImpl {
    fn get_battery_info(&self) -> Option<BatteryInfo> {
        let battery_manager = battery::Manager::new().ok()?;
        let batteries = battery_manager.batteries().ok()?;
        Some(
            batteries
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
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn no_battery_status() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| None);

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn ignores_zero_capacity_battery() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 0.0,
                energy_full: 0.0,
                state: battery::State::Full,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_full() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 1000.0,
                energy_full: 1000.0,
                state: battery::State::Full,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = Some(String::from(" 100% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_charging() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 800.0,
                energy_full: 1000.0,
                state: battery::State::Charging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 90
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = Some(String::from(" 80% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_discharging() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 800.0,
                energy_full: 1000.0,
                state: battery::State::Discharging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = Some(String::from(" 80% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_unknown() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 0.0,
                energy_full: 1.0,
                state: battery::State::Unknown,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = Some(String::from(" 0% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_empty() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 0.0,
                energy_full: 1000.0,
                state: battery::State::Empty,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = Some(String::from(" 0% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_hidden_when_percentage_above_threshold() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 600.0,
                energy_full: 1000.0,
                state: battery::State::Full,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 50
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_only_on_discharge_charging() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 100.0,
                energy_full: 1000.0,
                state: battery::State::Charging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 10
                only_on_discharge = true
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_only_on_discharge_discharging() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 100.0,
                energy_full: 1000.0,
                state: battery::State::Discharging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 10
                only_on_discharge = true
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = Some(String::from(" 10% "));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_uses_style() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 400.0,
                energy_full: 1000.0,
                state: battery::State::Discharging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 50
                style = "bold red"
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = Some(format!("{} ", Color::Red.bold().paint(" 40%")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn battery_displayed_precision() {
        let mut mock = MockBatteryInfoProvider::new();

        mock.expect_get_battery_info().times(1).returning(|| {
            Some(BatteryInfo {
                energy: 129.87654,
                energy_full: 1000.0,
                state: battery::State::Discharging,
            })
        });

        let actual = ModuleRenderer::new("battery")
            .config(toml::toml! {
                [[battery.display]]
                threshold = 100
                style = ""
            })
            .battery_info_provider(&mock)
            .collect();
        let expected = Some(String::from(" 13% "));

        assert_eq!(expected, actual);
    }
}
