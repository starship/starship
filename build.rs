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
    let paths = fs::read_dir("docs/.vuepress/public/presets/toml")?;

    let mut presets = String::new();
    let mut hashmap_assignments = String::new();
    for path in paths {
        let unwrapped = path?;
        let file_name = unwrapped.file_name();
        let full_path = dunce::canonicalize(unwrapped.path())?;
        let full_path = full_path.to_str().expect("failed to convert to string");
        let name = file_name
            .to_str()
            .and_then(|v| v.strip_suffix(".toml"))
            .expect("Failed to process filename");
        presets.push_str(format!("print::Preset(\"{}\"),\n", name).as_str());
        hashmap_assignments.push_str(
            format!(
                r#"
    let preset = String::from_utf8_lossy(include_bytes!(
        r"{}"
    ))
    .to_string();
    preset_map.insert(String::from("{}"), preset);
"#,
                full_path, name
            )
            .as_str(),
        );
    }

    let content: String = format!(
        r#"
use crate::print;
use std::collections::HashMap;

pub fn get_preset_list<'a>() -> &'a [print::Preset] {{
    &[
        {}
    ]
}}

pub fn get_preset_content(name: String) -> Option<String> {{
    let mut preset_map: HashMap<String, String> = HashMap::new();

    {}

    preset_map.get(&name).cloned()
}}
"#,
        presets, hashmap_assignments,
    );
    writeln!(file, "{}", content)?;
    Ok(())
}
