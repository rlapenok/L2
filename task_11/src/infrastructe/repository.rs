use std::sync::Arc;

use crate::{api::data_for_request::{DeleteEvent, UpdateEvent}, domain::{
    errors::CalendarErrors, events::{Event, Events}, filters::Filter, note_repository::NoteRepository
}};
use dashmap::DashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct Repository(DashMap<usize, RwLock<Events>>);

impl Repository {
    pub fn new() -> Self {
        Self(DashMap::new())
    }
    async fn insert(&self, event: Event)->Uuid {
        let uuid=event.event_uid;
        self.0
            .entry(event.user_id)
            .or_insert_with(|| RwLock::new(Vec::new()))
            .write()
            .await
            .push(Arc::new(event));
        uuid
    }
    async fn update(&self, event: UpdateEvent) -> Result<(), CalendarErrors> {
        if let Some(events) = self.0.get(&event.user_id) {
            let guard = events.write().await;
            if let Some(old_event) = guard
                .iter()
                .find(|old_event| old_event.event_uid == event.event_uid)
            {
                let mut guard = old_event.data.lock().map_err(|err|{
                    CalendarErrors::PoisonError(err.to_string())
                })?;
                *guard = format!("{} {}",guard,event.data.trim());
                return Ok(())
            }
            return  Err(CalendarErrors::NotFoundEvents)
        }
        Err(CalendarErrors::NotFoundUserId)
    }
    async fn delete(&self, event: DeleteEvent) -> Result<(), CalendarErrors> {
        if let Some(events) = self.0.get(&event.user_id) {
            let mut guard = events.write().await;
            if let Some(position) = guard.iter().position(|event_in_repository| event_in_repository.event_uid ==event.event_uid ) {
                guard.remove(position);
                return Ok(());
            }
            return Err(CalendarErrors::NotFoundEvents);
        }
        Err(CalendarErrors::NotFoundUserId)
    }
    async fn find<'a>(&self, user_id:usize,filter:Filter<'a>) -> Result<Events, CalendarErrors>
    {
        if let Some(events) = self.0.get(&user_id) {
            let guard = events.read().await;
            let events = guard.iter().filter_map(filter).collect::<Events>();
            if !events.is_empty()  {
                return Ok(events);
            }
            return Err(CalendarErrors::NotFoundEvents);
        }
        Err(CalendarErrors::NotFoundUserId)
    }
}
#[async_trait::async_trait]
impl NoteRepository for Repository {
    async fn create_event(&self, event: Event) ->Uuid{
        self.insert(event).await
    }
    async fn update_event(&self, info: UpdateEvent) -> Result<(), CalendarErrors> {
        self.update(info).await
    }
    async fn delete_event(&self, info: DeleteEvent) -> Result<(), CalendarErrors> {
        self.delete(info).await
    }
    async fn get_events<'a>(&self, user_id:usize,filter:Filter<'a>) -> Result<Events, CalendarErrors>
    {
        self.find(user_id,filter).await
    }

}
