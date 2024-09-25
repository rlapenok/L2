use std::{
    io::{Error, ErrorKind},
    net::{SocketAddr, ToSocketAddrs},
    num::ParseIntError,
    time::Duration,
};

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(long,short='t',default_value_t=String::from("10s"))]
    timeout: String,
    host: String,
    port: u16,
}

impl Cli {
    pub fn get_duration(&self) -> Result<Duration, ParseIntError> {
        let last_index = &self.timeout.chars().last().unwrap();
        let sec = &self.timeout[..self.timeout.len() - 1].parse::<u64>()?;
        match last_index {
            's' => Ok(Duration::from_secs(*sec)),
            'm' => Ok(Duration::from_secs(60 * sec)),
            _ => Ok(Duration::from_secs(10)),
        }
    }
    pub fn get_addr(&self) -> Result<SocketAddr, Error> {
        let url = format!("{}:{}", self.host.trim(), self.port);
        let socket = url.to_socket_addrs()?.next().ok_or(ErrorKind::NotFound)?;
        Ok(socket)
    }
}
