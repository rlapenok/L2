use std::{error::Error, sync::Arc};

use client::Client;
use tokio::{
    io::{stdin, stdout, AsyncReadExt, AsyncWriteExt},
    sync::Notify,
};

pub mod cli;
pub mod client;

pub struct Wget {
    client: Arc<Client>,
}

impl Wget {
    pub fn new(client: Client) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn run(&self) {
        let reader = self.client.clone();
        let writer = self.client.clone();
        let notify = Arc::new(Notify::new());
        let n1 = notify.clone();
        let n2 = notify.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = n1.notified()=>{
                        break;
                    }
                    _ = write(writer.clone(),n1.clone())=>{
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = n2.notified()=>{
                        break;
                    }
                    _ = read(reader.clone(),n2.clone())=>{
                    }
                }
            }
        });
        notify.notified().await;
    }
}

async fn read(reader: Arc<Client>, token: Arc<Notify>) {
    let mut buff = [0; 512];
    match reader.read(&mut buff).await {
        Ok(0) => {
            token.notify_waiters();
        }
        Ok(_) => {
            let res = format!(
                "Response from server:{}",
                String::from_utf8_lossy(buff.as_slice())
            );
            stdout().write_all(res.as_bytes()).await.unwrap();
            stdout().flush().await.unwrap();
        }
        Err(err) => {
            eprintln!("Eror recv message from server:{}", err);
        }
    }
}
async fn write(writer: Arc<Client>, not: Arc<Notify>) {
    let mut buff = [0; 512];
    match stdin().read(&mut buff).await {
        Ok(0) => {
            not.notify_waiters();
        }
        Ok(_) => {
            if let Err(err) = writer.write(buff.as_slice()).await {
                eprintln!("Error while send message to server:{}", err);
            }
        }
        Err(err) => {
            eprintln!("Error while read from console:{}", err);
            not.notify_waiters();
        }
    }
}
