use std::process::Command;

use ansi_term::Color;

use super::{Context, Module};

/// Creates a module with the current Java version
///
/// Will display the Java version if any of the following criteria are met:
///     - Current directory contains a file with a `.java` extension
///     - Current directory contains a `pom.xml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_java_project = context
        .new_scan_dir()
        .set_files(&["pom.xml"])
        .set_extensions(&["java"])
        .scan();

    if !is_java_project {
        return None;
    }

    match get_java_version() {
        Some(java_version) => {
            const JAVA_CHAR: &str = "☕ ";

            let mut module = context.new_module("java")?;
            let module_style = module
                .config_value_style("style")
                .unwrap_or_else(|| Color::RGB(166, 42, 42).bold());
            module.set_style(module_style);

            let formatted_version = format_java_version(java_version);
            module.new_segment("symbol", JAVA_CHAR);
            module.new_segment("version", &formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_java_version() -> Option<String> {
    // `javac -version` usually outputs to `stderr`,
    // but it's best check both outputs.
    match Command::new("javac").arg("-version").output() {
        Ok(output) => {
            if output.stdout.is_empty() {
                let stderr_string = String::from_utf8(output.stderr).unwrap();
                Some(stderr_string)
            } else {
                let stdout_string = String::from_utf8(output.stdout).unwrap();
                Some(stdout_string)
            }
        }
        Err(_) => None,
    }
}

fn format_java_version(java_stdout: String) -> String {
    format!("v{}", java_stdout.replace("javac", "").trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_java_version() {
        let input = String::from("javac 1.8.0_181");
        let formatted = format_java_version(input);

        assert_eq!(formatted, "v1.8.0_181");
    }
}
