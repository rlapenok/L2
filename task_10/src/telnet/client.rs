use std::{
    io::{self, ErrorKind},
    net::SocketAddr,
    time::{Duration, Instant},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
};

pub struct Client {
    reader: Mutex<OwnedReadHalf>,
    writer: Mutex<OwnedWriteHalf>,
}

impl Client {
    pub async fn new(addr: SocketAddr, duration: Duration) -> Result<Self, io::Error> {
        let deadline = Instant::now() + duration;

        while Instant::now() < deadline {
            if let Ok(stream) = TcpStream::connect(addr).await {
                let stream = stream.into_split();
                println!("Connection -OK");
                return Ok(Self {
                    reader: Mutex::new(stream.0),
                    writer: Mutex::new(stream.1),
                });
            }
        }
        Err(ErrorKind::ConnectionRefused.into())
    }
    pub async fn write(&self, buff: &[u8]) -> io::Result<()> {
        let mut guard = self.writer.lock().await;
        guard.write_all(buff).await?;
        guard.flush().await?;
        Ok(())
    }
    pub async fn read(&self, buff: &mut [u8]) -> io::Result<usize> {
        let mut guard = self.reader.lock().await;
        guard.read(buff).await
    }
}
