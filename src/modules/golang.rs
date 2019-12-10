use super::{Context, Module, RootModuleConfig};

use crate::configs::go::GoConfig;
use crate::utils;

/// Creates a module with the current Go version
///
/// Will display the Go version if any of the following criteria are met:
///     - Current directory contains a `go.mod` file
///     - Current directory contains a `go.sum` file
///     - Current directory contains a `glide.yaml` file
///     - Current directory contains a `Gopkg.yml` file
///     - Current directory contains a `Gopkg.lock` file
///     - Current directory contains a `Godeps` directory
///     - Current directory contains a file with the `.go` extension
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_go_project = context
        .try_begin_scan()?
        .set_files(&["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock"])
        .set_extensions(&["go"])
        .set_folders(&["Godeps"])
        .is_match();

    if !is_go_project {
        return None;
    }

    let mut module = context.new_module("golang");
    let config: GoConfig = GoConfig::try_load(module.config);

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    let formatted_version =
        format_go_version(&utils::exec_cmd("go", &["version"])?.stdout.as_str())?;
    module.create_segment("version", &config.version.with_value(&formatted_version));

    Some(module)
}

fn format_go_version(go_stdout: &str) -> Option<String> {
    // go version output looks like this:
    // go version go1.13.3 linux/amd64

    let version = go_stdout
        // split into ["", "1.12.4 linux/amd64"]
        .splitn(2, "go version go")
        // return "1.12.4 linux/amd64"
        .nth(1)?
        // split into ["1.12.4", "linux/amd64"]
        .split_whitespace()
        // return "1.12.4"
        .next()?;

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_go_version() {
        let input = "go version go1.12 darwin/amd64";
        assert_eq!(format_go_version(input), Some("v1.12".to_string()));
    }
}
