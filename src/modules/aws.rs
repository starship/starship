use std::env;

use super::{Context, Module, RootModuleConfig};

use crate::configs::aws::AwsConfig;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const AWS_PREFIX: &str = "on ";

    let aws_profile = env::var("AWS_PROFILE").ok()?;
    if aws_profile.is_empty() {
        return None;
    }

    let mut module = context.new_module("aws");
    let config: AwsConfig = AwsConfig::try_load(module.config);

    module.set_style(config.style);

    module.get_prefix().set_value(AWS_PREFIX);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("profile", &config.profile.with_value(&aws_profile));

    Some(module)
}
