use std::path::PathBuf;

use super::Context;

pub struct JJRepo {
    #[expect(unused)] // will go away in follow up commits
    root: PathBuf,
}

impl JJRepo {
    pub fn discover(context: &Context) -> Option<Self> {
        let root = context.begin_ancestor_scan().set_folders(&[".jj"]).scan()?;

        Some(Self { root })
    }
}
