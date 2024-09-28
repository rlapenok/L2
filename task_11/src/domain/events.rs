use std::sync::{Arc, Mutex};

use chrono::NaiveDate;

use serde::Serialize;
use uuid::Uuid;

use crate::api::data_for_request::CreateEvent;

use super::filters::TimeFrameFilter;



pub type Events = Vec<Arc<Event>>;

#[derive(Serialize)]
pub struct Event {
    pub user_id:usize,
    pub date: NaiveDate,
    pub event_uid: Uuid,
    pub data: Mutex<String>,
}

impl From<CreateEvent> for Event{
    fn from(value: CreateEvent) -> Self {
        let event_uid=Uuid::new_v4();
        Self { user_id:value.user_id,date:value.date, event_uid, data: Mutex::new(value.data.trim().to_string()) }
    }
}

pub struct GetEvents{
    pub user_id:usize,
    pub date:NaiveDate,
    pub filter:TimeFrameFilter
}
