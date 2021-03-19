use super::{Context, Module, RootModuleConfig, Shell};
use crate::configs::battery::BatteryConfig;

use crate::formatter::StringFormatter;

/// Creates a module for the battery percentage and charging state
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // TODO: Update when v1.0 printing refactor is implemented to only
    // print escapes in a prompt context.
    let percentage_char = match context.shell {
        Shell::Zsh => "%%", // % is an escape in zsh, see PROMPT in `man zshmisc`
        _ => "%",
    };

    let battery_status = get_battery_status()?;
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
                        battery::State::Charging => Some(config.charging_symbol),
                        battery::State::Discharging => {
                            if display_style.symbol != "" {
                                Some(display_style.symbol)
                            } else {
                                Some(config.discharging_symbol)
                            }
                        }
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

fn get_battery_status() -> Option<BatteryStatus> {
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
struct BatteryStatus {
    percentage: f32,
    state: battery::State,
}
