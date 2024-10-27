type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>; // For tests.

use super::*;

#[tokio::test]
async fn test_fs_execute_not_error() -> Result<()> {
    // -- Setup & Fixtures

    // -- Exec

    // -- Check

    // Run method that can return error
    execute(true)?;

    Ok(())
}
