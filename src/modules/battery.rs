use ansi_term::{Color, Style};

use super::{Context, Module};
use crate::config::Config;

/// Creates a module for the battery percentage and charging state
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const BATTERY_FULL: &str = "•";
    const BATTERY_CHARGING: &str = "⇡";
    const BATTERY_DISCHARGING: &str = "⇣";
    // TODO: Update when v1.0 printing refactor is implemented to only
    // print escapes in a prompt context.
    let shell = std::env::var("STARSHIP_SHELL").unwrap_or_default();
    let percentage_char = match shell.as_str() {
        "zsh" => "%%", // % is an escape in zsh, see PROMPT in `man zshmisc`
        _ => "%",
    };

    let battery_status = get_battery_status()?;
    let BatteryStatus { state, percentage } = battery_status;

    let mut module = context.new_module("battery");

    // Parse config under `display`
    let display_styles = get_display_styles(&module);
    let display_style = display_styles.iter().find(|display_style| {
        let BatteryDisplayStyle { threshold, .. } = display_style;
        percentage <= *threshold as f32
    });

    if let Some(display_style) = display_style {
        let BatteryDisplayStyle { style, .. } = display_style;

        // Set style based on percentage
        module.set_style(*style);
        module.get_prefix().set_value("");

        match state {
            battery::State::Full => {
                module.new_segment("full_symbol", BATTERY_FULL);
            }
            battery::State::Charging => {
                module.new_segment("charging_symbol", BATTERY_CHARGING);
            }
            battery::State::Discharging => {
                module.new_segment("discharging_symbol", BATTERY_DISCHARGING);
            }
            battery::State::Unknown => {
                log::debug!("Unknown detected");
                module.new_segment_required("unknown_symbol")?;
            }
            battery::State::Empty => {
                module.new_segment_required("empty_symbol")?;
            }
            _ => {
                log::debug!("Unhandled battery state `{}`", state);
                return None;
            }
            _ => return None,
        }

        let mut percent_string = Vec::<String>::with_capacity(2);
        // Round the percentage to a whole number
        percent_string.push(percentage.round().to_string());
        percent_string.push(percentage_char.to_string());
        module.new_segment("percentage", percent_string.join("").as_ref());

        Some(module)
    } else {
        None
    }
}

fn get_display_styles(module: &Module) -> Vec<BatteryDisplayStyle> {
    if let Some(display_configs) = module.config_value_array("display") {
        let mut display_styles: Vec<BatteryDisplayStyle> = vec![];
        for display_config in display_configs.iter() {
            if let toml::Value::Table(config) = display_config {
                if let Some(display_style) = BatteryDisplayStyle::from_config(config) {
                    display_styles.push(display_style);
                }
            }
        }

        // Return display styles as long as display array exists, even if it is empty.
        display_styles
    } else {
        // Default display styles: [{ threshold = 10, style = "red bold" }]
        vec![BatteryDisplayStyle {
            threshold: 10,
            style: Color::Red.bold(),
        }]
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

#[derive(Clone, Debug)]
struct BatteryDisplayStyle {
    threshold: i64,
    style: Style,
}

impl BatteryDisplayStyle {
    /// construct battery display style from toml table
    pub fn from_config(config: &toml::value::Table) -> Option<BatteryDisplayStyle> {
        let threshold = config.get_as_i64("threshold")?;
        let style = config.get_as_ansi_style("style")?;

        Some(BatteryDisplayStyle { threshold, style })
    }
}
