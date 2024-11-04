//! Main Crate Error

use crate::fs;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    // -- Modules
    Fs(fs::Error), // as example

    // -- Externals
    Io(std::io::Error), // as example
}

// region:    --- Froms

impl From<fs::Error> for Error {
    fn from(val: fs::Error) -> Self {
        Self::Fs(val)
    }
}

impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        Self::Io(val)
    }
}

// endregion: --- Froms

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
