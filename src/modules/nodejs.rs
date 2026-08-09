use super::{Context, Module, ModuleConfig};

use crate::configs::nodejs::NodejsConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

use regex::Regex;
use semver::Version;
use semver::VersionReq;
use serde_json as json;
use std::ops::Deref;
use std::sync::LazyLock;

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
        context
            .exec_cmd("node", &["--version"])
            .map(|cmd| cmd.stdout)
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
            log::warn!("Error in module `nodejs`:\n{error}");
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

    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let version = re
        .captures(nodejs_version)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();

    let v = match Version::parse(version) {
        Ok(v) => v,
        Err(_e) => return true,
    };
    r.matches(&v)
}

#[cfg(test)]
mod tests {
    use crate::test::{Project, RenderedModule, project};
    use insta::assert_snapshot;
    use rstest::rstest;

    /// The `format` that also surfaces the `engines` requirement, used by the
    /// checks that care about how a mismatch is reported.
    const FORMAT_WITH_ENGINES_VERSION: &str = "via [$symbol($version )($engines_version )]($style)";

    /// A filesystem artifact that on its own must make the `nodejs` module
    /// treat a directory as a Node.js project.
    ///
    /// Each variant becomes its own independently named, independently reported
    /// test case, so a detection rule that regresses is identified by name
    /// rather than hidden behind whichever rule happens to fail first.
    #[derive(Clone, Copy, Debug)]
    enum NodeProjectMarker {
        PackageManifest,
        NodeVersionFile,
        NodeVersionManagerFile,
        JavaScriptFile,
        EcmaScriptModuleFile,
        CommonJsModuleFile,
        TypeScriptFile,
        InstalledDependenciesDirectory,
    }

    impl NodeProjectMarker {
        /// Materializes this marker inside `project`.
        fn create_in(self, project: &Project) {
            match self {
                Self::PackageManifest => project.create_file("package.json"),
                Self::NodeVersionFile => project.create_file(".node-version"),
                Self::NodeVersionManagerFile => project.create_file(".nvmrc"),
                Self::JavaScriptFile => project.create_file("index.js"),
                Self::EcmaScriptModuleFile => project.create_file("index.mjs"),
                Self::CommonJsModuleFile => project.create_file("index.cjs"),
                Self::TypeScriptFile => project.create_file("index.ts"),
                Self::InstalledDependenciesDirectory => project.create_directory("node_modules"),
            };
        }
    }

    /// A `package.json` declaring `engines.node` as `requirement`.
    fn package_manifest_requiring_node(requirement: &str) -> String {
        format!("{{\n  \"engines\": {{\n    \"node\": \"{requirement}\"\n  }}\n}}")
    }

    #[rstest]
    fn folder_without_node_files(project: Project) {
        assert_eq!(project.render("nodejs"), RenderedModule::Empty);
    }

    #[rstest]
    fn folder_with_node_project_marker(
        project: Project,
        #[values(
            NodeProjectMarker::PackageManifest,
            NodeProjectMarker::NodeVersionFile,
            NodeProjectMarker::NodeVersionManagerFile,
            NodeProjectMarker::JavaScriptFile,
            NodeProjectMarker::EcmaScriptModuleFile,
            NodeProjectMarker::CommonJsModuleFile,
            NodeProjectMarker::TypeScriptFile,
            NodeProjectMarker::InstalledDependenciesDirectory
        )]
        marker: NodeProjectMarker,
    ) {
        marker.create_in(&project);

        // Every marker must produce the very same rendering, so all cases share
        // one inline snapshot. `allow_duplicates!` tells insta that repeated
        // assertions against this location are intentional: under `cargo test`
        // all cases run in one process, and insta otherwise rejects the second
        // assertion at a given inline location as an accidental loop.
        insta::allow_duplicates! {
            assert_snapshot!(project.render("nodejs"), @r#""via \u{1b}[1;32m\u{e718} v12.0.0 \u{1b}[0m""#);
        }
    }

    /// An `esy` project happens to carry a `package.json`, but belongs to the
    /// `ocaml` module rather than to this one.
    #[rstest]
    fn folder_with_package_json_and_esy_lock(project: Project) {
        project.create_file("package.json");
        project.create_directory("esy.lock");

        assert_eq!(project.render("nodejs"), RenderedModule::Empty);
    }

    #[rstest]
    fn engines_node_version_match(project: Project) {
        project.write_file("package.json", &package_manifest_requiring_node(">=12.0.0"));

        assert_snapshot!(project.render("nodejs"), @r#""via \u{1b}[1;32m\u{e718} v12.0.0 \u{1b}[0m""#);
    }

    /// A version outside the declared `engines` range must switch the module
    /// over to `not_capable_style`.
    #[rstest]
    fn engines_node_version_not_match(project: Project) {
        project.write_file("package.json", &package_manifest_requiring_node("<12.0.0"));

        assert_snapshot!(project.render("nodejs"), @r#""via \u{1b}[1;31m\u{e718} v12.0.0 \u{1b}[0m""#);
    }

    #[rstest]
    fn show_expected_version_when_engines_does_not_match(project: Project) {
        project.write_file("package.json", &package_manifest_requiring_node("<=11.0.0"));

        let rendered = project
            .renderer("nodejs")
            .config(toml::toml! {
                [nodejs]
                format = FORMAT_WITH_ENGINES_VERSION
            })
            .collect();

        assert_snapshot!(RenderedModule::from(rendered), @r#""via \u{1b}[1;31m\u{e718} v12.0.0 <=11.0.0 \u{1b}[0m""#);
    }

    #[rstest]
    fn do_not_show_expected_version_if_engines_match(project: Project) {
        project.write_file("package.json", &package_manifest_requiring_node(">=12.0.0"));

        let rendered = project
            .renderer("nodejs")
            .config(toml::toml! {
                [nodejs]
                format = FORMAT_WITH_ENGINES_VERSION
            })
            .collect();

        assert_snapshot!(RenderedModule::from(rendered), @r#""via \u{1b}[1;32m\u{e718} v12.0.0 \u{1b}[0m""#);
    }

    #[rstest]
    fn do_not_show_expected_version_if_no_set_engines_version(project: Project) {
        project.create_file("package.json");

        let rendered = project
            .renderer("nodejs")
            .config(toml::toml! {
                [nodejs]
                format = FORMAT_WITH_ENGINES_VERSION
            })
            .collect();

        assert_snapshot!(RenderedModule::from(rendered), @r#""via \u{1b}[1;32m\u{e718} v12.0.0 \u{1b}[0m""#);
    }

    #[rstest]
    fn no_node_installed(project: Project) {
        project.create_file("index.js");

        let rendered = project
            .renderer("nodejs")
            .cmd("node --version", None)
            .collect();

        assert_snapshot!(RenderedModule::from(rendered), @r#""via \u{1b}[1;32m\u{e718} \u{1b}[0m""#);
    }
}
