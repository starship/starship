use ansi_term::Color;

use super::{Context, Module};

/// Creates a module for the battery percentage and charging state
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const BATTERY_FULL: &str = "•";
    const BATTERY_CHARGING: &str = "⇡";
    const BATTERY_DISCHARGING: &str = "⇣";
    const BATTERY_THRESHOLD: i64 = 10;
    // TODO: Update when v1.0 printing refactor is implemented to only
    // print escapes in a prompt context.
    let shell = std::env::var("STARSHIP_SHELL").unwrap_or_default();
    let percentage_char = match shell.as_str() {
        "zsh" => "%%", // % is an escape in zsh, see PROMPT in `man zshmisc`
        _ => "%",
    };

    let battery_status = get_battery_status()?;
    let BatteryStatus { state, percentage } = battery_status;

    let mut module = context.new_module("battery")?;
    let battery_threshold = match module.config_value_i64("display_threshold") {
        Some(threshold) => {
            if 0 <= threshold && threshold < 100 {
                threshold
            } else {
                log::debug!(
                    "Expect battery.battery_threshold in [0, 100), found `{}`",
                    threshold
                );
                BATTERY_THRESHOLD
            }
        }
        None => BATTERY_THRESHOLD,
    };

    if percentage > battery_threshold as f32 {
        log::debug!(
            "Battery percentage is higher than threshold ({} > {})",
            percentage,
            battery_threshold
        );
        return None;
    }

    // TODO: Set style based on percentage when threshold is modifiable
    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Red.bold());
    module.set_style(module_style);
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
        _ => return None,
    }

    let mut percent_string = Vec::<String>::with_capacity(2);
    // Round the percentage to a whole number
    percent_string.push(percentage.round().to_string());
    percent_string.push(percentage_char.to_string());
    module.new_segment("percentage", percent_string.join("").as_ref());

    Some(module)
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
