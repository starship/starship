use super::{Context, Module, ModuleConfig};

use crate::configs::nodejs::NodejsConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

use regex::Regex;
use semver::Version;
use semver::VersionReq;
use serde_json as json;
use std::ops::Deref;
use std::sync::LazyLock;

static VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"v?(\d+\.\d+\.\d+)").unwrap());

fn format_nodejs_version(captures: &regex::Captures) -> String {
    if captures[0].starts_with('v') {
        captures[0].to_string()
    } else {
        format!("v{}", &captures[1])
    }
}

/// Creates a module with the current Node.js version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("nodejs");
    let config = NodejsConfig::try_load(module.config);
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    let is_esy_project = context
        .try_begin_scan()?
        .set_folders(&["esy.lock"])
        .is_match();

    if !is_js_project || is_esy_project {
        return None;
    }

    let nodejs_version = LazyLock::new(|| {
        context.exec_cmd("node", &["--version"]).and_then(|cmd| {
            // If output is a single line and matches version format, return it directly
            let trimmed = cmd.stdout.trim();
            if !trimmed.contains('\n') && VERSION_REGEX.is_match(trimmed) {
                return Some(if trimmed.starts_with('v') {
                    trimmed.to_string()
                } else {
                    format!("v{}", trimmed)
                });
            }

            let lines: Vec<&str> = trimmed.lines().collect();

            if let Some(last_line) = lines.last() {
                if let Some(captures) = VERSION_REGEX.captures(last_line) {
                    return Some(format_nodejs_version(&captures));
                }
            }

            for line in lines.iter().rev() {
                if let Some(captures) = VERSION_REGEX.captures(line) {
                    return Some(format_nodejs_version(&captures));
                }
            }

            VERSION_REGEX
                .captures(trimmed)
                .map(|cap| format_nodejs_version(&cap))
        })
    });
    let engines_version = LazyLock::new(|| get_engines_version(context));

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => {
                    let in_engines_range = check_engines_version(
                        nodejs_version.as_deref(),
                        engines_version.as_deref(),
                    );

                    if in_engines_range {
                        Some(Ok(config.style))
                    } else {
                        Some(Ok(config.not_capable_style))
                    }
                }
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let node_ver = nodejs_version
                        .deref()
                        .as_ref()?
                        .trim_start_matches('v')
                        .trim();

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        node_ver,
                        config.version_format,
                    )
                    .map(Ok)
                }
                "engines_version" => {
                    let in_engines_range = check_engines_version(
                        nodejs_version.as_deref(),
                        engines_version.as_deref(),
                    );
                    let eng_ver = engines_version.as_deref()?.to_string();

                    (!in_engines_range).then_some(Ok(eng_ver))
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `nodejs`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_engines_version(context: &Context) -> Option<String> {
    let json_str = context.read_file_from_pwd("package.json")?;
    let package_json: json::Value = json::from_str(&json_str).ok()?;
    let raw_version = package_json.get("engines")?.get("node")?.as_str()?;

    Some(raw_version.to_string())
}

fn check_engines_version(nodejs_version: Option<&str>, engines_version: Option<&str>) -> bool {
    let (Some(nodejs_version), Some(engines_version)) = (nodejs_version, engines_version) else {
        return true;
    };

    let Ok(r) = VersionReq::parse(engines_version) else {
        return true;
    };

    let version_matches = match VERSION_REGEX.captures(nodejs_version) {
        Some(cap) => cap[1].to_string(),
        None => return true,
    };

    let v = match Version::parse(&version_matches) {
        Ok(v) => v,
        Err(_e) => return true,
    };
    r.matches(&v)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_node_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json_and_esy_lock() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;
        let esy_lock = dir.path().join("esy.lock");
        fs::create_dir_all(esy_lock)?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".node-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nvmrc() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".nvmrc"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_js_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.js"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mjs_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.mjs"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cjs_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.cjs"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ts_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.ts"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_modules() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let node_modules = dir.path().join("node_modules");
        fs::create_dir_all(node_modules)?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn engines_node_version_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\">=12.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn engines_node_version_not_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\"<12.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn show_expected_version_when_engines_does_not_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\"<=11.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .config(toml::toml! {
                [nodejs]
                format = "via [$symbol($version )($engines_version )]($style)"
            })
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Red.bold().paint(" v12.0.0 <=11.0.0 ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn do_not_show_expected_version_if_engines_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\">=12.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .config(toml::toml! [
                [nodejs]
                format = "via [$symbol($version )($engines_version )]($style)"
            ])
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn do_not_show_expected_version_if_no_set_engines_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .config(toml::toml! {
                [nodejs]
                format = "via [$symbol($version )($engines_version )]($style)"
            })
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn no_node_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.js"))?.sync_all()?;
        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .cmd("node --version", None)
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_clean_node_version_output() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .cmd(
                "node --version",
                Some(CommandOutput {
                    stdout: "v16.14.2\n".to_string(),
                    stderr: "".to_string(),
                }),
            )
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v16.14.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_polluted_node_version_output() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let polluted_output =
            "Installing Node.js v18.17.1...\nDownloading binary...\nExtracting...\nv18.17.1\n";

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .cmd(
                "node --version",
                Some(CommandOutput {
                    stdout: polluted_output.to_string(),
                    stderr: "".to_string(),
                }),
            )
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v18.17.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_output_with_multiple_versions() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let multi_version_output = "Checking dependencies...\nRequired Node.js version: v14.15.0\nInstalling Node.js v20.10.0...\nv20.10.0\n";

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .cmd(
                "node --version",
                Some(CommandOutput {
                    stdout: multi_version_output.to_string(),
                    stderr: "".to_string(),
                }),
            )
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v20.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_complex_output_case() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let complex_output = "Checking Node.js v14.17.0...\nRequired version: v16.14.2\nInstalling Node.js v16.14.2...\nDownload complete!\nInstallation successful.\n";

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .cmd(
                "node --version",
                Some(CommandOutput {
                    stdout: complex_output.to_string(),
                    stderr: "".to_string(),
                }),
            )
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v16.14.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_proto_auto_install_logs() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let proto_output = " vAuto-install is enabled, attempting to install Node.js 22.13.0 \n\n[node] Installing Node. js 22.13.0\n\n[node] Downloading pre-built archive node-v22.13.0-darwin-arm64. tar. xz\n\n[node] Verifying checksum against SHASUMS256.txt\n\n[node] Unpacking archive node-v22.13.0-darwin-arm64. tar.xz\n\n[node] Node. js 22.13.0 installed (3s)\n\n[npm] npm 10.9.2 installed (2s)\n\n| INFO\n| npm 10.9.2 has already been installed at /Users/user/. proto/tools/npm/10.9.2!\n\nv22.13.0\n";

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .cmd(
                "node --version",
                Some(CommandOutput {
                    stdout: proto_output.to_string(),
                    stderr: "".to_string(),
                }),
            )
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v22.13.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
