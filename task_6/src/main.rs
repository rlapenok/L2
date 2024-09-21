use cli::CutCli;

mod cli;

fn main() {
    let cli = CutCli::new();
    cli.run();
}
