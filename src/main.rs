#![allow(unused)]

// region:    --- Modules

use std::fs::read_dir;

use rust_base::init;

pub use crate::prelude::*;

mod error;
mod fs;
mod prelude;
mod utils;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    init();

    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{}", entry);
    }

    Ok(())
}
