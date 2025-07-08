use {{project-name}}::{Result, Error};

#[tokio::main]
async fn main() -> Result<()> {
    {{project-name}}::init()?;

    println!("quick-dev");

    Ok(())
}
