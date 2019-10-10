use std::process::Command;

use ansi_term::Color;

use super::{Context, Module};

/// Creates a module with the current Java version
///
/// Will display the Java version if any of the following criteria are met:
///     - Current directory contains a file with a `.java`, `.class` or `.jar` extension
///     - Current directory contains a `pom.xml`, `build.gradle` or `build.sbt` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_java_project = context
        .try_begin_scan()?
        .set_files(&["pom.xml", "build.gradle", "build.sbt"])
        .set_extensions(&["java", "class", "jar"])
        .is_match();

    if !is_java_project {
        return None;
    }

    match get_java_version() {
        Some(java_version) => {
            const JAVA_CHAR: &str = "☕ ";

            let mut module = context.new_module("java");
            let module_style = module
                .config_value_style("style")
                .unwrap_or_else(|| Color::Red.dimmed());
            module.set_style(module_style);

            let formatted_version = format_java_version(java_version)?;
            module.new_segment("symbol", JAVA_CHAR);
            module.new_segment("version", &formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_java_version() -> Option<String> {
    let java_command = match std::env::var("JAVA_HOME") {
        Ok(java_home) => format!("{}/bin/java", java_home),
        Err(_) => String::from("java"),
    };

    match Command::new(java_command).arg("-Xinternalversion").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

/// Extract the java version from `java_stdout`.
/// The expected format is similar to: "JRE (1.8.0_222-b10)".
/// Some Java vendors don't follow this format: "JRE (Zulu 8.40.0.25-CA-linux64)").
fn format_java_version(java_stdout: String) -> Option<String> {
    let start = java_stdout.find("JRE (")? + "JRE (".len();
    let end = start
        + (java_stdout[start..].find(|c| match c {
            '0'..='9' | '.' => false,
            _ => true,
        })?);

    if start == end {
        None
    } else {
        Some(format!("v{}", &java_stdout[start..end]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_java_version_openjdk() {
        let java_8 = String::from("OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 10:18:43 by \"openjdk\" with gcc 4.4.7 20120313 (Red Hat 4.4.7-23)");
        let java_11 = String::from("OpenJDK 64-Bit Server VM (11.0.4+11-post-Ubuntu-1ubuntu219.04) for linux-amd64 JRE (11.0.4+11-post-Ubuntu-1ubuntu219.04), built on Jul 18 2019 18:21:46 by \"build\" with gcc 8.3.0");
        assert_eq!(format_java_version(java_11), Some(String::from("v11.0.4")));
        assert_eq!(format_java_version(java_8), Some(String::from("v1.8.0")));
    }

    #[test]
    fn test_format_java_version_oracle() {
        let java_8 = String::from("Java HotSpot(TM) Client VM (25.65-b01) for linux-arm-vfp-hflt JRE (1.8.0_65-b17), built on Oct  6 2015 16:19:04 by \"java_re\" with gcc 4.7.2 20120910 (prerelease)");
        assert_eq!(format_java_version(java_8), Some(String::from("v1.8.0")));
    }

    #[test]
    fn test_format_java_version_redhat() {
        let java_8 = String::from("OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 20:48:53 by \"root\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)");
        let java_12 = String::from("OpenJDK 64-Bit Server VM (12.0.2+10) for linux-amd64 JRE (12.0.2+10), built on Jul 18 2019 14:41:47 by \"jenkins\" with gcc 7.3.1 20180303 (Red Hat 7.3.1-5)");
        assert_eq!(format_java_version(java_8), Some(String::from("v1.8.0")));
        assert_eq!(format_java_version(java_12), Some(String::from("v12.0.2")));
    }

    #[test]
    fn test_format_java_version_zulu() {
        // Not currently supported
        let java_8 = String::from("OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (Zulu 8.40.0.25-CA-linux64) (1.8.0_222-b10), built on Jul 11 2019 11:36:39 by \"zulu_re\" with gcc 4.4.7 20120313 (Red Hat 4.4.7-3)");
        assert_eq!(format_java_version(java_8), None);
    }
}
