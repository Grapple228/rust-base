#![allow(unused)] // For beginning only.

use rust_base::{config, execute, init};
use rust_base::{Error, Result};
use std::fs::read_dir;

#[tokio::main]
async fn main() -> Result<()> {
    rust_base::init();

    rust_base::execute(false)?;

    Ok(())
}
