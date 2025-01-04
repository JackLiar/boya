use std::io::Error as IOError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid Opus decoder param: {0}")]
    InvalidDecoderParam(String),
    #[error("Invalid Opus packet: {0}")]
    InvalidPacket(String),
    #[error("IO Error: {0}")]
    IoError(IOError),
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Self::IoError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
