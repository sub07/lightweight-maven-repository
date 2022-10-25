use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidConfiguration(String),
    InvalidFileArtifactUrl(String),
    InvalidFileMetadataUrl(String),
    ArtifactOverwrite(String),
    MavenMetadataCreation(String),
    RequestDataExtraction(String),
    AuthenticationError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::InvalidConfiguration(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
