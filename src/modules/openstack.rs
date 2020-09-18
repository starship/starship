use yaml_rust::YamlLoader;

use super::{Context, Module, RootModuleConfig};

use crate::configs::openstack::OspConfig;
use crate::formatter::StringFormatter;
use crate::utils;

type Cloud = String;
type Project = String;

fn get_osp_project_from_config(_context: &Context, osp_cloud: Option<&str>) -> Option<Project> {
    let config_file = dirs_next::home_dir()?.join(".config/openstack/clouds.yaml");
    let contents = utils::read_file(&config_file).ok()?;
    let clouds = YamlLoader::load_from_str(&contents).unwrap();
    let project = &clouds[0]["clouds"][osp_cloud.unwrap()]["auth"]["project_name"].as_str().unwrap();
    Some(project.to_string())
}

fn get_osp_cloud_and_project(context: &Context) -> (Option<Cloud>, Option<Project>) {
    match (
        context
            .get_env("OS_CLOUD"),
        context
            .get_env("OS_PROJECT_NAME")
    ) {
        (Some(p), Some(r)) => (Some(p), Some(r)),
        (None, Some(r)) => (None, Some(r)),
        (Some(ref p), None) => (
            Some(p.to_owned()),
            get_osp_project_from_config(context, Some(p)),
        ),
        (None, None) => (None, None),
    }
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("openstack");
    let config: OspConfig = OspConfig::try_load(module.config);

    let (osp_cloud, osp_project) = get_osp_cloud_and_project(context);

    if osp_cloud.is_none() {
        return None;
    }

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
            .parse(None)
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
