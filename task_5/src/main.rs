mod grep_cli;
use grep_cli::grep_cli::GrepCli;

fn main() {
    let cli=GrepCli::new();
    cli.run().unwrap();
}
