use axum::{extract::State, http::StatusCode, Json};

use crate::{domain::{errors::CalendarErrors,  note_service::NoteService}, infrastructe::state::ServerState as ServerState};

use super::data_for_request::DeleteEvent;


pub async fn delete_event(State(state):State<ServerState>,Json(event):Json<DeleteEvent>)->Result<StatusCode,CalendarErrors>{

    state.delete_event(event).await?;
    Ok(StatusCode::OK)
}