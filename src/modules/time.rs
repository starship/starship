use super::{Context, Module};
use ansi_term::Color;
use chrono::prelude::*;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let module_color = Color::Blue.normal();
    let current_time = Local::now().format("%H:%M %p").to_string();

    let mut module = context.new_module("time")?;
    module.set_style(module_color);
    module.get_prefix().set_value("at ");

    module.new_segment("symbol", "🕒 ");
    module.new_segment("value", &current_time);
    Some(module)
}
