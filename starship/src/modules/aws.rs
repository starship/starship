use std::env;

use ansi_term::Color;

use super::{Context, Module};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const AWS_CHAR: &str = "☁️ ";
    const AWS_PREFIX: &str = "on ";

    let aws_profile = env::var("AWS_PROFILE").ok()?;
    if aws_profile.is_empty() {
        return None;
    }

    let mut module = context.new_module("aws");

    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Yellow.bold());
    module.set_style(module_style);

    module.get_prefix().set_value(AWS_PREFIX);

    module.new_segment("symbol", AWS_CHAR);
    module.new_segment("profile", &aws_profile);

    Some(module)
}
