use std::{error::Error, process::exit};

use clap::Parser;
use telnet::{cli::Cli, client::Client, Wget};
mod telnet;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let addr = cli.get_addr()?;
    let timeout = cli.get_duration()?;
    let client = Client::new(addr, timeout).await?;
    let wget = Wget::new(client).unwrap();
    wget.run().await;

    /*Заюзал такой метод выхода из приложения, т.к. при CTRL+D  программа завершается корректно, но при CTRC+C (и отсутствии exit ) (см прикрепленные скриншот tokio-console) на сервере происходит блокировка Stdin
    почитал след и решил сделать проще
    https://github.com/tokio-rs/tokio/issues/2318
    https://github.com/tokio-rs/tokio/issues/2466
    https://github.com/deltachat/deltachat-core-rust/pull/4325
    https://github.com/tokio-rs/tokio/issues/2466*/
    exit(1);
}
