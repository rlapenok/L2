use std::error::Error;

use wc_cli::WcCli;

mod wc_cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = WcCli::new()?;
    cli.run()?;
    Ok(())
}
