use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig};
use crate::configs::battery::BatteryConfig;
use crate::segment::Segment;
use ansi_term::Style;

/// Creates a module for the battery percentage and charging state
pub struct BatteryModule<'a> {
    module: Module<'a>,
    config: BatteryConfig<'a>,
}

impl<'a> BatteryModule<'a> {
    pub fn new(context: &'a Context) -> Self {
        let module = context.new_module("battery");
        let config = BatteryConfig::try_load(module.config);

        BatteryModule { module, config }
    }

    pub fn module(mut self) -> Option<Module<'a>> {
        let battery_status = get_battery_status()?;

        // Set style based on percentage
        let module_style = self.display_style(&battery_status);

        let segments: Vec<Segment> =
            format_segments(self.config.format, module_style, |name, query| {
                let style = get_style_from_query(&query).or(module_style);
                match name {
                    "symbol" => self.symbol(&battery_status).map(|value| Segment {
                        _name: "symbol".to_string(),
                        value: value.to_string(),
                        style,
                    }),
                    "percentage" => {
                        let percentage = battery_status.percentage.round().to_string();
                        Some(Segment {
                            _name: "percentage".to_string(),
                            value: percentage,
                            style,
                        })
                    }
                    _ => None,
                }
            })
            .ok()?;

        self.module.set_segments(segments);

        Some(self.module)
    }

    fn display_style(&self, battery_status: &BatteryStatus) -> Option<Style> {
        // Parse config under `display`
        let BatteryStatus { percentage, .. } = battery_status;
        let display_styles = &self.config.display;
        display_styles
            .iter()
            .find(|display_style| *percentage <= display_style.threshold as f32)
            .map(|display_style| display_style.style)
    }

    fn symbol(&self, battery_status: &BatteryStatus) -> Option<&str> {
        let BatteryStatus { state, .. } = battery_status;

        match state {
            battery::State::Full => Some(self.config.full_symbol),
            battery::State::Charging => Some(self.config.charging_symbol),
            battery::State::Discharging => Some(self.config.discharging_symbol),
            battery::State::Unknown => {
                log::debug!("Unknown detected");
                self.config.unknown_symbol
            }
            battery::State::Empty => self.config.empty_symbol,
            _ => {
                log::debug!("Unhandled battery state `{}`", state);
                None
            }
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
                log::debug!("Unable to access battery information:\n{}", &e);
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
