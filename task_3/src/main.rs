use cli::SortCli;

mod cli;

fn main() -> Result<(), String> {
    let cli = SortCli::new();
    cli.run()?;
    Ok(())
}
