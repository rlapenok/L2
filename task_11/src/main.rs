use std::{env, error::Error};

use app::{create_app, get_address_from_env, get_logginig_level};
use axum::serve;
use dotenv::dotenv;
use tokio::net::TcpListener;

mod domain;
mod infrastructe;
mod api;
mod app;

#[tokio::main]
async fn main() ->Result<(),Box<dyn Error>>{
    dotenv().ok();
    let level=get_logginig_level()?;
    env::set_var("RUST_LOG", level);

    env_logger::init();
    let address=get_address_from_env()?;
    let app=create_app();

    let listner=TcpListener::bind(&address).await?;
    println!("Server start on address:{}",address);
    serve(listner,app).await?;
    Ok(())

    
}
