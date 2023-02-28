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
    println!("cargo:rerun-if-changed=docs/.vuepress/public/presets/toml");
    let paths = fs::read_dir("docs/.vuepress/public/presets/toml")?;

    let mut presets = String::new();
    let mut match_arms = String::new();
    for path in paths {
        let unwrapped = path?;
        let file_name = unwrapped.file_name();
        let full_path = dunce::canonicalize(unwrapped.path())?;
        let full_path = full_path.to_str().expect("failed to convert to string");
        let name = file_name
            .to_str()
            .and_then(|v| v.strip_suffix(".toml"))
            .expect("Failed to process filename");
        presets.push_str(format!("print::Preset(\"{name}\"),\n").as_str());
        match_arms.push_str(
            format!(
                r#"
"{name}" => {{
        include_bytes!(r"{full_path}")
}}
"#
            )
            .as_str(),
        );
    }

    writeln!(
        file,
        r#"
use std::io::{{self, Write}};
use crate::print;

pub fn get_preset_list<'a>() -> &'a [print::Preset] {{
    &[
        {presets}
    ]
}}

pub fn get_preset_content(name: &str) -> &'static [u8]{{
    match name {{
    {match_arms}
    _ => {{b""}}
    }}
}}

pub fn print_preset_content(name: &str) {{
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = stdout.write_all(get_preset_content(name));
}}
"#
    )?;
    Ok(())
}
