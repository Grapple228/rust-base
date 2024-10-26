#![allow(unused)] // For beginning only.

// region:    --- Modules

use tracing::info;
use tracing_subscriber::EnvFilter;

pub use crate::prelude::*;

pub mod config;
mod error;
mod fs;
pub mod prelude;
mod utils;

// endregion: --- Modules

pub fn init() {
    // LOGGING INITIALIZATION
    tracing_subscriber::fmt()
        .without_time() // For early development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Initializing");
}
