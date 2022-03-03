use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    shadow_rs::new().map_err(|err| err.to_string())?;

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("starship.exe.manifest");
        res.compile()?;
    }

    Ok(())
}
