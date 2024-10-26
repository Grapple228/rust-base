#![allow(unused)] // For beginning only.

use rust_base::{config::config, init, prelude::*};
use std::fs::read_dir;

#[tokio::main]
async fn main() -> Result<()> {
    init();

    Err(Error::InvalidPath)?;

    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{}", entry);
    }

    Ok(())
}
