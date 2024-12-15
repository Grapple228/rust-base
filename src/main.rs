use {{project-name}}::{config, init};
use {{project-name}}::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    {{project-name}}::init();

    Ok(())
}
