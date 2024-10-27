// Example module

// -- Modules
mod error;
mod fs_sub;
#[cfg(test)]
mod tests;

// -- Flatten
pub use error::{Error, Result};
pub use fs_sub::execute;
