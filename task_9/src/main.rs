use std::error::Error;

use wget::Wget;

mod wget;

#[tokio::main]

// cargo run https://crates.io/ OR cargo run https://crates.io/ -p <PATH>
async fn main() -> Result<(), Box<dyn Error>> {
    let wget = Wget::new()?;
    wget.run().await?;
    Ok(())
}
