use axum::extract::State;

use crate::{domain::note_service::NoteService, infrastructe::state::ServerState as ServerState};

use super::{data_for_request::CreateEvent, data_for_response::EventUid};

pub async fn create_event(State(state):State<ServerState>,event:CreateEvent)->EventUid{

    let event=event.into();
    let uuid=state.create_event(event).await;
    EventUid(uuid)
}