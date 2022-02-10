fn main() -> shadow_rs::SdResult<()> {
    shadow_rs::new()?;
    embed_resource::compile("starship-res.rc");
    Ok(())
}
