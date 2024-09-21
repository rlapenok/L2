use std::error::Error;

use lf_cli::LfCli;

mod lf_cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = LfCli::new();
    cli.run()?;
    Ok(())
}
