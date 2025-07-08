use {{project-name}}::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    {{project-name}}::init()?;

    println!("app");

    Ok(())
}
