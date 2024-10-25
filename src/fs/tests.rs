use crate::prelude::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>; // For tests.

use super::*;

#[tokio::test]
async fn test_name() -> Result<()> {
    // -- Setup & Fixtures

    // -- Exec

    // -- Check

    return Err("Unable to load file....".into());

    Ok(())
}
