use super::{Context, Module, ModuleConfig};

use crate::configs::haskell::HaskellConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Haskell version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("haskell");
    let config = HaskellConfig::try_load(module.config);

    let is_hs_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_hs_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => get_version(context).map(Ok),
                "ghc_version" => get_ghc_version(context).map(Ok),
                "snapshot" => get_snapshot(context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `haskell`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_ghc_version(context: &Context) -> Option<String> {
    Some(
        context
            .exec_cmd("ghc", &["--numeric-version"])?
            .stdout
            .trim()
            .to_string(),
    )
}

fn get_snapshot(context: &Context) -> Option<String> {
    if !is_stack_project(context) {
        return None;
    }
    let file_contents = context.read_file_from_pwd("stack.yaml")?;
    let yaml = yaml_rust::YamlLoader::load_from_str(&file_contents).ok()?;
    let version = yaml.first()?["resolver"]
        .as_str()
        .or_else(|| yaml.first()?["snapshot"].as_str())
        .filter(|s| s.starts_with("lts") || s.starts_with("nightly") || s.starts_with("ghc"))
        .unwrap_or("<custom snapshot>");
    Some(version.to_string())
}

fn get_version(context: &Context) -> Option<String> {
    get_snapshot(context).or_else(|| get_ghc_version(context))
}

fn is_stack_project(context: &Context) -> bool {
    match context.dir_contents() {
        Ok(dir) => dir.has_file_name("stack.yaml"),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_hs_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_stack() -> io::Result<()> {
        let cases = vec![
            ("resolver: lts-18.12\n", "lts-18.12"),
            ("snapshot:\tnightly-2011-11-11", "nightly-2011-11-11"),
            ("snapshot: ghc-8.10.7", "ghc-8.10.7"),
            (
                "snapshot: https://github.com/whatever/xxx.yaml\n",
                "<custom snapshot>",
            ),
            (
                "resolver:\n  url: https://github.com/whatever/xxx.yaml\n",
                "<custom snapshot>",
            ),
            (REANIMATE_STACK_YAML, "lts-14.27"),
        ];
        for (yaml, resolver) in &cases {
            let dir = tempfile::tempdir()?;
            let mut file = File::create(dir.path().join("stack.yaml"))?;
            file.write_all(yaml.as_bytes())?;
            file.sync_all()?;
            let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
            let expected = Some(format!(
                "via {}",
                Color::Purple.bold().paint(format!("λ {resolver} "))
            ));
            assert_eq!(expected, actual);
            dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn folder_cabal() -> io::Result<()> {
        let should_trigger = vec!["a.hs", "b.hs-boot", "cabal.project"];
        for hs_file in &should_trigger {
            let dir = tempfile::tempdir()?;
            File::create(dir.path().join(hs_file))?.sync_all()?;
            let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
            let expected = Some(format!("via {}", Color::Purple.bold().paint("λ 9.2.1 ")));
            assert_eq!(expected, actual);
            dir.close()?;
        }
        Ok(())
    }

    static REANIMATE_STACK_YAML: &str = r"
resolver: lts-14.27

allow-newer: false

packages:
- .

extra-deps:
- reanimate-svg-0.13.0.1
- chiphunk-0.1.2.1
- cubicbezier-0.6.0.6@sha256:2191ff47144d9a13a2784651a33d340cd31be1926a6c188925143103eb3c8db3
- fast-math-1.0.2@sha256:91181eb836e54413cc5a841e797c42b2264954e893ea530b6fc4da0dccf6a8b7
- matrices-0.5.0@sha256:b2761813f6a61c84224559619cc60a16a858ac671c8436bbac8ec89e85473058
- hmatrix-0.20.0.0@sha256:d79a9218e314f1a2344457c3851bd1d2536518ecb5f1a2fcd81daa45e46cd025,4870
- earcut-0.1.0.4@sha256:d5118b3eecf24d130263d81fb30f1ff56b1db43036582bfd1d8cc9ba3adae8be,1010
- tasty-rerun-1.1.17@sha256:d4a3ccb0f63f499f36edc71b33c0f91c850eddb22dd92b928aa33b8459f3734a,1373
- hgeometry-0.11.0.0@sha256:09ead201a6ac3492c0be8dda5a6b32792b9ae87cab730b8362d46ee8d5c2acb4,11714
- hgeometry-combinatorial-0.11.0.0@sha256:03176f235a1c49a415fe1266274dafca84deb917cbcbf9a654452686b4cd2bfe,8286
- vinyl-0.13.0@sha256:0f247cd3f8682b30881a07de18e6fec52d540646fbcb328420049cc8d63cd407,3724
- hashable-1.3.0.0@sha256:4c70f1407881059e93550d3742191254296b2737b793a742bd901348fb3e1fb1,5206
- network-3.1.2.1@sha256:188d6daea8cd91bc3553efd5a90a1e7c6d0425fa66a53baa74db5b6d9fd75c8b,4968
";
}
