use std::io;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("cannot open cache file")]
    OpenFile(#[source] io::Error),

    #[error("cannot write cache file")]
    WriteFile(#[source] io::Error),

    #[error("cannot read binary metadata")]
    ReadMetadata(#[source] io::Error),

    #[error("unable to serialize cache")]
    SerializeCache(#[source] toml::ser::Error),
}
