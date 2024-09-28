use std::time::Instant;

use axum::{body::Body, http::Request, middleware::Next, response::Response};
use log::info;

pub async fn logger_middleware(req:Request<Body>,next:Next)->Response{
    let now=Instant::now();
    let method=req.method().to_string();
    let path=req.uri().path().to_string();
        let resp=next.run(req).await;
    let end=now.elapsed().as_secs_f32()*1000.0;
    info!("   METHOD: {}   PATH: {}   elapsed:{}ms",method,path,end);
    resp
}