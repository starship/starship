use std::fs::{self, File};
use std::io::Write;

use shadow_rs::SdResult;

fn main() -> SdResult<()> {
    shadow_rs::new().map_err(|err| err.to_string())?;
    shadow_rs::new_hook(gen_presets_hook)?;

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("starship.exe.manifest")
            .set_icon("media/icon.ico");
        res.compile()?;
    }

    Ok(())
}

fn gen_presets_hook(mut file: &File) -> SdResult<()> {
    let paths =
        fs::read_dir("docs/.vuepress/public/presets/toml").expect("failed to read directory");

    let mut presets = String::new();
    let mut hashmap_assignments = String::new();
    for path in paths {
        let file_name = path.expect("failed to get path info").file_name();
        let name = file_name
            .to_str()
            .expect("Failed to convert to string")
            .strip_suffix(".toml")
            .expect("Failed to trim .toml suffix")
            .clone();
        presets.push_str(format!("Preset(\"{}\"),\n", name).as_str());
        hashmap_assignments.push_str(
            format!(
                r#"
    let preset = String::from_utf8_lossy(include_bytes!(
        "../../../../../docs/.vuepress/public/presets/toml/{0}.toml"
    ))
    .to_string();
    preset_map.insert(String::from("{0}"), preset);
"#,
                name
            )
            .as_str(),
        );
    }

    let content: String = format!(
        r#"
use std::collections::HashMap;
use clap::{{ValueEnum, PossibleValue}};

#[derive(Clone, Debug)]
pub struct Preset(pub &'static str);

impl ValueEnum for Preset {{
    fn value_variants<'a>() -> &'a [Self] {{
        get_preset_list()
    }}

    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {{
        Self::value_variants()
            .iter()
            .find(|v| v.0 == self.0)
            .map(|v| PossibleValue::new(v.0))
    }}
}}

pub fn get_preset_list<'a>() -> &'a [Preset] {{
    &[
        {}
    ]
}}

pub fn get_preset_content(name: String) -> Option<String> {{
    let mut preset_map: HashMap<String, String> = HashMap::new();

    {}

    preset_map.get(&name).map(|v| v.clone())
}}
"#,
        presets, hashmap_assignments,
    );
    writeln!(file, "{}", content)?;
    Ok(())
}
