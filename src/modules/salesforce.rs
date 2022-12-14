use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::ModuleConfig;
use crate::configs::salesforce::SalesforceConfig;
use crate::formatter::StringFormatter;
use crate::utils::read_file;

use super::{Context, Module};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("salesforce");
    let config = SalesforceConfig::try_load(module.config);

    let repo = match context.get_repo() {
        Ok(r) => r,
        Err(_error) => return None,
    };

    let org_name_option = match config.use_sfdx {
        true => get_org_name_from_command(context),
        false => {
            let workdir = match &repo.workdir {
                Some(w) => w,
                None => return None,
            };
            get_org_name_from_config_files(&workdir)
        }
    };
    let org_name = match org_name_option {
        Some(o) => o,
        None => return None,
    };
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
                "org_name" => {
                    return Some(Ok(org_name.clone()));
                }
                _ => None,
            })
            .parse(None, Some(context))
    });
    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module 'salesforce':\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[derive(Deserialize, Serialize)]
struct SfdxCommandResult {
    result: Option<SfdxCommandResultResult>,
}

#[derive(Deserialize, Serialize)]
struct SfdxCommandResultResult {
    alias: Option<String>,
    username: Option<String>,
}

fn get_org_name_from_command(context: &Context) -> Option<String> {
    let output = context
        .exec_cmd("sfdx", &["force:org:display", "--json"])?
        .stdout;
    let result: SfdxCommandResult = match serde_json::from_str(output.as_str()) {
        Ok(r) => r,
        Err(error) => {
            log::warn!("Module salesforce failed to parse sfdx output: {}", error);
            return None;
        }
    };
    match result.result {
        Some(r) => {
            match r.alias {
                Some(alias) => return Some(alias),
                None => (),
            };
            return match r.username {
                Some(username) => Some(username),
                None => None,
            };
        }
        None => None,
    }
}

fn get_org_name_from_config_files(repo_path: &PathBuf) -> Option<String> {
    let sf_configuration_file = repo_path.join(".sf").join("config.json");
    if sf_configuration_file.exists() {
        return get_org_name_from_sf_config_file(sf_configuration_file);
    }
    let sfdx_configuration_file = repo_path.join(".sfdx").join("sfdx-config.json");
    if sfdx_configuration_file.exists() {
        return get_org_name_from_sfdx_config_file(sfdx_configuration_file);
    }
    return None;
}

#[derive(Deserialize, Serialize)]
struct SfdxConfig {
    defaultusername: Option<String>,
}

fn get_org_name_from_sfdx_config_file(sfdx_config_file: PathBuf) -> Option<String> {
    let file_contents = match read_file(sfdx_config_file) {
        Ok(f) => f,
        Err(error) => {
            log::warn!("Module salesforce failed to read sfdx config: {}", error);
            return None;
        }
    };
    let result: SfdxConfig = match serde_json::from_str(file_contents.as_str()) {
        Ok(r) => r,
        Err(error) => {
            log::warn!("Module salesforce failed to parse sfdx output: {}", error);
            return None;
        }
    };
    return result.defaultusername;
}

#[derive(Deserialize, Serialize)]
struct SfConfig {
    #[serde(alias = "target-org")]
    target_org: Option<String>,
}

fn get_org_name_from_sf_config_file(sfdx_config_file: PathBuf) -> Option<String> {
    let file_contents = match read_file(sfdx_config_file) {
        Ok(f) => f,
        Err(error) => {
            log::warn!("Module salesforce failed to read sf config: {}", error);
            return None;
        }
    };
    let result: SfConfig = match serde_json::from_str(file_contents.as_str()) {
        Ok(r) => r,
        Err(error) => {
            log::warn!("Module salesforce failed to parse sf output: {}", error);
            return None;
        }
    };
    return result.target_org;
}
