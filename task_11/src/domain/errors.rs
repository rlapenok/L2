use axum::{http::StatusCode, response::{IntoResponse,Response}};
use serde::Serialize;
use thiserror::Error;

#[derive(Error,Debug,Serialize)]
pub enum CalendarErrors {
    #[error("not found events")]
    NotFoundEvents,
    #[error("not found user_id")]
    NotFoundUserId,
    #[error("error while serialize events:{0}")]
    SerializeError(String),
    #[error("error while let lock:{0}")]
    PoisonError(String),
    #[error("inner filter required")]
    InvalidFilter,
    #[error("error while serialize data from request:{0}")]
    DeserializeError(String),
}



impl IntoResponse for CalendarErrors {
        fn into_response(self) -> Response {
            match self {
                Self::NotFoundEvents=>{
                    (StatusCode::SERVICE_UNAVAILABLE,self.to_string()).into_response()
                }
                Self::NotFoundUserId=>{
                    (StatusCode::SERVICE_UNAVAILABLE,self.to_string()).into_response()

                }
                Self::PoisonError(err)=>{
                    (StatusCode::INTERNAL_SERVER_ERROR,err.to_string()).into_response()
                }
                Self::SerializeError(err)=>{
                    (StatusCode::INTERNAL_SERVER_ERROR,err).into_response()
                }
                Self::InvalidFilter=>{
                    (StatusCode::BAD_REQUEST,self.to_string()).into_response()
                }
                Self::DeserializeError(err)=>{
                    (StatusCode::BAD_REQUEST,err).into_response()
                }
                
            }
        }
}
