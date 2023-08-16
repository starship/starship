use yaml_rust::YamlLoader;

use super::{Context, Module, ModuleConfig};

use crate::configs::openstack::OspConfig;
use crate::formatter::StringFormatter;
use crate::utils;

type Cloud = String;
type Project = String;

fn get_osp_project_from_config(context: &Context, osp_cloud: &str) -> Option<Project> {
    // Attempt to follow OpenStack standards for clouds.yaml location:
    // 1st = $PWD/clouds.yaml, 2nd = $HOME/.config/openstack/clouds.yaml, 3rd = /etc/openstack/clouds.yaml
    let config = [
        context.get_env("PWD").map(|pwd| pwd + "/clouds.yaml"),
        context.get_home().map(|home| {
            home.join(".config/openstack/clouds.yaml")
                .display()
                .to_string()
        }),
        Some(String::from("/etc/openstack/clouds.yaml")),
    ];

    config
        .iter()
        .filter_map(|file| {
            let config = utils::read_file(file.as_ref()?).ok()?;
            let clouds = YamlLoader::load_from_str(config.as_str()).ok()?;
            clouds.get(0)?["clouds"][osp_cloud]["auth"]["project_name"]
                .as_str()
                .map(ToOwned::to_owned)
        })
        .find(|s| !s.is_empty())
}

fn get_osp_cloud_and_project(context: &Context) -> (Option<Cloud>, Option<Project>) {
    match (
        context.get_env("OS_CLOUD"),
        context.get_env("OS_PROJECT_NAME"),
    ) {
        (Some(p), Some(r)) => (Some(p), Some(r)),
        (None, Some(r)) => (None, Some(r)),
        (Some(ref p), None) => (Some(p.clone()), get_osp_project_from_config(context, p)),
        (None, None) => (None, None),
    }
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("openstack");
    let config: OspConfig = OspConfig::try_load(module.config);

    let (osp_cloud, osp_project) = get_osp_cloud_and_project(context);

    osp_cloud.as_ref()?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "cloud" => osp_cloud.as_ref().map(Ok),
                "project" => osp_project.as_ref().map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::error!("Error in module `openstack`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io::{self, Write};

    #[test]
    fn parse_valid_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("clouds.yaml");
        let mut file = File::create(config_path)?;
        file.write_all(
            b"---
clouds:
  corp:
    auth:
      auth_url: https://overcloud.example.com:13000/v3
      project_name: testproject
    identity_api_version: '3'
    interface: public
",
        )?;
        let actual = ModuleRenderer::new("openstack")
            .env("PWD", dir.path().to_str().unwrap())
            .env("OS_CLOUD", "corp")
            .config(toml::toml! {
                [openstack]
            })
            .collect();
        let expected = Some(format!(
            "on {} ",
            Color::Yellow.bold().paint("☁️  corp(testproject)")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn parse_broken_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("clouds.yaml");
        let mut file = File::create(config_path)?;
        file.write_all(
            b"---
dummy_yaml
",
        )?;
        let actual = ModuleRenderer::new("openstack")
            .env("PWD", dir.path().to_str().unwrap())
            .env("OS_CLOUD", "test")
            .config(toml::toml! {
                [openstack]
            })
            .collect();
        let expected = Some(format!("on {} ", Color::Yellow.bold().paint("☁️  test")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn dont_crash_on_empty_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join("clouds.yaml");
        let mut file = File::create(config_path)?;
        file.write_all(b"")?;
        drop(file);
        let actual = ModuleRenderer::new("openstack")
            .env("PWD", dir.path().to_str().unwrap())
            .env("OS_CLOUD", "test")
            .config(toml::toml! {
                [openstack]
            })
            .collect();
        let expected = Some(format!("on {} ", Color::Yellow.bold().paint("☁️  test")));

        assert_eq!(actual, expected);
        dir.close()
    }
}
