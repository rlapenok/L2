
use axum::{extract::{FromRequest, Request},Json};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{errors::CalendarErrors, filters::InnerTimeFrameFilter};

#[derive(Deserialize)]
pub struct CreateEvent{
    pub user_id:usize,
    pub data: String,
    pub date:NaiveDate,
}




#[async_trait::async_trait]
impl <S>FromRequest<S> for CreateEvent
    where S:Send+Sync
{
    type Rejection = CalendarErrors;
    async fn from_request(req:Request,state:&S)->Result<Self,Self::Rejection>{

        let create_event=Json::<Self>::from_request(req, state).await.map_err(|err|{
            CalendarErrors::DeserializeError(err.to_string())
        })?;
        Ok(create_event.0)
    }

}




#[derive(Deserialize)]
pub struct QueryParams{
    pub user_id:usize,
    pub date:NaiveDate,
    pub inner_filter:Option<InnerTimeFrameFilter>
}




#[derive(Deserialize)]
pub struct DeleteEvent{
    pub user_id:usize,
    pub event_uid: Uuid,
}


#[derive(Deserialize)]
pub struct UpdateEvent {
    pub user_id:usize,
    pub event_uid: Uuid,
    pub data: String,
}