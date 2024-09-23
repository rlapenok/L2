use reqwest::{Client, Error, Response};

#[derive(Clone)]
pub struct Downloader(Client);
impl Downloader {
    pub fn new() -> Result<Self, Error> {
        let client = Client::builder().user_agent("HandMadeWget").build()?;
        Ok(Self(client))
    }
    pub async fn download(&self, url: &str) -> Result<Response, Error> {
        self.0.get(url).send().await
    }
}
