use ansi_term::Color;

use super::{Context, Module};

/// Creates a segment for the battery percentage and charging state
pub fn segment(_context: &Context) -> Option<Module> {
    const BATTERY_FULL: &str = "•";
    const BATTERY_CHARGING: &str = "⇡";
    const BATTERY_DISCHARGING: &str = "⇣";
    const BATTERY_THRESHOLD: f32 = 10.0;

    let battery_status = get_battery_status()?;
    let BatteryStatus { state, percentage } = battery_status;

    if percentage > BATTERY_THRESHOLD {
        log::debug!(
            "Battery percentage is higher than threshold ({} > {})",
            percentage,
            BATTERY_THRESHOLD
        );
        return None;
    }

    // TODO: Set style based on percentage when threshold is modifiable
    let mut module = Module::new("battery");
    module.set_style(Color::Red.bold());
    module.get_prefix().set_value("");

    match state {
        battery::State::Full => {
            module.new_segment("full", BATTERY_FULL);
        }
        battery::State::Charging => {
            module.new_segment("charging", BATTERY_CHARGING);
        }
        battery::State::Discharging => {
            module.new_segment("discharging", BATTERY_DISCHARGING);
        }
        _ => return None,
    }

    let mut percent_string = Vec::<String>::with_capacity(2);
    // Truncate the percentage to a whole number
    percent_string.push(percentage.trunc().to_string());
    percent_string.push("%".to_string());
    module.new_segment("percentage", percent_string.join(""));

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
            log::debug!("Unable to access battery information:\n{}", e);
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
