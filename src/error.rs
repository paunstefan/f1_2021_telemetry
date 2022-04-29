use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum F1Error {
    #[error("Can't convert value")]
    ConversionError,
    #[error("IO error")]
    IoError(#[from] io::Error),
}
