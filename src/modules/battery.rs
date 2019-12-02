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
    match battery_manager.batteries().ok()?.next() {
        Some(Ok(battery)) => {
            log::debug!("Battery found: {:?}", battery);
            let battery_status = BatteryStatus {
                percentage: battery.state_of_charge().value * 100.0,
                state: battery.state(),
            };

            Some(battery_status)
        }
        Some(Err(e)) => {
            log::debug!("Unable to access battery information:\n{}", &e);
            None
        }
        None => {
            log::debug!("No batteries found");
            None
        }
    }
}

struct BatteryStatus {
    percentage: f32,
    state: battery::State,
}
