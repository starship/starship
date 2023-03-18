use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
/// ## AWS
///
/// The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials.
/// The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.
///
/// The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process` or `sso_start_url` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice.
/// If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.
///
/// When using [aws-vault](https://github.com/99designs/aws-vault) the profile
/// is read from the `AWS_VAULT` env var and the credentials expiration date
/// is read from the `AWS_SESSION_EXPIRATION` env var.
///
/// When using [awsu](https://github.com/kreuzwerker/awsu) the profile
/// is read from the `AWSU_PROFILE` env var.
///
/// When using [`AWSume`](https://awsu.me) the profile
/// is read from the `AWSUME_PROFILE` env var and the credentials expiration
/// date is read from the `AWSUME_EXPIRATION` env var.
pub struct AwsConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The symbol used before displaying the current AWS profile.
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    /// Disables the AWS module.
    pub disabled: bool,
    /// Table of region aliases to display in addition to the AWS name.
    pub region_aliases: HashMap<String, &'a str>,
    /// Table of profile aliases to display in addition to the AWS name.
    pub profile_aliases: HashMap<String, &'a str>,
    /// The symbol displayed when the temporary credentials have expired.
    pub expiration_symbol: &'a str,
    /// If true displays info even if `credentials`, `credential_process` or `sso_start_url` have not been setup.
    pub force_display: bool,
    /// Show seconds in duration
    pub duration_show_seconds: bool,
}

impl<'a> Default for AwsConfig<'a> {
    fn default() -> Self {
        AwsConfig {
            format: "on [$symbol($profile )(\\($region\\) )(\\[$duration\\] )]($style)",
            symbol: "☁️  ",
            style: "bold yellow",
            disabled: false,
            region_aliases: HashMap::new(),
            profile_aliases: HashMap::new(),
            expiration_symbol: "X",
            force_display: false,
            duration_show_seconds: true,
        }
    }
}
