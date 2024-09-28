use std::sync::Arc;


use uuid::Uuid;

use crate::{api::data_for_request::{DeleteEvent, UpdateEvent}, domain::{
    errors::CalendarErrors,
    events::{Event, Events, GetEvents},
    note_repository::NoteRepository,
    note_service::NoteService
}, infrastructe::utils::create_filter};

use super::repository::Repository;


#[derive(Clone)]
pub struct ServerState

{
    note_repo: Arc<dyn NoteRepository+Sync+Send>,
}



impl ServerState {
    pub fn new()->Self{
        let note_repo=Repository::new();
        Self { note_repo: Arc::new(note_repo) }
    }
}




#[async_trait::async_trait]
impl NoteService for ServerState

{
    async fn create_event(&self, event: Event)->Uuid {
        self.note_repo.create_event(event).await
    }

    async fn update_event(&self, event: UpdateEvent) -> Result<(), CalendarErrors> {
        self.note_repo.update_event(event).await
    }
    async fn delete_event(&self, event:DeleteEvent) -> Result<(), CalendarErrors> {
        self.note_repo.delete_event(event).await
    }
    async fn get_events(
        &self,
        filter:GetEvents
    ) -> Result<Events, CalendarErrors> {
        let date=filter.date;
        let time_frame=filter.filter;
        let id=filter.user_id;
       let filter=create_filter(date,time_frame )?;
        self.note_repo.get_events(id, filter).await
    }
}
