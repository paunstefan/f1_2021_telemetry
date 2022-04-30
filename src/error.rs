use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum F1Error {
    #[error("Can't convert value")]
    ConversionError,
    #[error("Not enough data to parse")]
    IncompleteData,
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Can't convert byte array to string")]
    UTF8Error(#[from] std::str::Utf8Error),
}
