use axum::{http::StatusCode, response::{IntoResponse,Response}, Json};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct EventUid(pub Uuid);


impl IntoResponse for EventUid{
    fn into_response(self) -> Response {
        (StatusCode::OK,Json(self)).into_response()
    }
}