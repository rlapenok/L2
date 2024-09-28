
use uuid::Uuid;

use crate::api::data_for_request::{DeleteEvent, UpdateEvent};

use super::{
    errors::CalendarErrors, events::{Event, Events}, filters::Filter
};

#[async_trait::async_trait]
pub trait NoteRepository:Send+Sync {
    async fn create_event(&self, event: Event)->Uuid;
    async fn update_event(&self, event:UpdateEvent) -> Result<(), CalendarErrors>;
    async fn delete_event(&self, event:DeleteEvent) -> Result<(), CalendarErrors>;
    async fn get_events<'a>(&self, user_id:usize,filter:Filter<'a>) -> Result<Events, CalendarErrors>;

}
