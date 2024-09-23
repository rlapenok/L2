use std::{
    collections::HashSet, error::Error, io::ErrorKind, rc::Rc,time::{SystemTime, UNIX_EPOCH}
};

use clap::Parser as ClapParser;
use cli::Cli;
use downloader::Downloader;
use local::{Directory, Local};
use selector::WgetSelector;
use reqwest::Url;
use scraper::Html;


mod cli;
mod downloader;
mod local;
mod selector;

pub struct Wget {
    loader: Downloader,
    url: Rc<Url>,
    saver: Local,
    selector: WgetSelector,
}

impl Wget {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cli = Cli::parse();
        let local = Local::new(cli.path);
        let url = Url::parse(&cli.url)?;
        let loader = Downloader::new()?;
        let selector = WgetSelector::new("a,link,script,img")?;
        Ok(Self {
            saver: local,
            url: Rc::new(url),
            loader,
            selector,
        })
    }
    async fn download_site(&self, url: &str) -> Result<(), Box<dyn Error>> {
        let index_html = self.loader.download(url).await?.text().await?;
        let file_name = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string()
            + ".html";
        let html = Html::parse_document(&index_html);
        let data = html
            .select(&self.selector)
            .fold(HashSet::new(), |mut set, element_ref| {
                let tag = element_ref.value().name();
                element_ref.value().attrs().for_each(|attr| {
                    if (attr.0 == "src" || attr.0 == "href") && !attr.1.is_empty() {
                        if let Ok(url) = self.url.join(attr.1) {
                            set.insert((tag.to_owned(), url));
                        }
                    }
                });
                set
            });

        for (tag, url) in data {
            match tag.as_str() {
                "link" => {
                    let data = self.loader.download(url.as_str()).await?.bytes().await?;
                    let file_name = url.path().split("/").last().unwrap();

                    match self.saver.save(Directory::Links, file_name, &data) {
                        Ok(_new_path) => {}
                        Err(err) => {
                            if err.kind() != ErrorKind::AlreadyExists {
                                eprintln!("Error while save data:{}", err)
                            }
                        }
                    }
                }
                "a" => Box::pin(async { self.download_site(url.as_str()).await }).await?,
                "img" => {
                    let data = self.loader.download(url.as_str()).await?.bytes().await?;
                    let file_name = url.path().split("/").last().unwrap();

                    match self.saver.save(Directory::Image, file_name, &data) {
                        Ok(_new_path) => {}
                        Err(err) => {
                            if err.kind() != ErrorKind::AlreadyExists {
                                eprintln!("Error while save data:{}", err)
                            }
                        }
                    }
                }
                "script" => {
                    let data = self.loader.download(url.as_str()).await?.bytes().await?;
                    let file_name = url.path().split("/").last().unwrap();
                    match self.saver.save(Directory::Script, file_name, &data) {
                        Ok(_new_path) => {}
                        Err(err) => {
                            if err.kind() != ErrorKind::AlreadyExists {
                                eprintln!("Error while save data:{}", err)
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        self.saver
            .save(Directory::Html, file_name.as_str(), html.html().as_bytes())?;
        Ok(())
    }
    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let url = self.url.as_str();
        self.download_site(url).await?;
        Ok(())
    }
}
