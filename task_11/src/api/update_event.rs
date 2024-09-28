use axum::{extract::State, http::StatusCode, Json};

use crate::{domain::{errors::CalendarErrors, note_service::NoteService}, infrastructe::state::ServerState as ServerState};

use super::data_for_request::UpdateEvent;



pub async fn update_event(State(state):State<ServerState>,Json(event):Json<UpdateEvent>)->Result<StatusCode,CalendarErrors>{

    state.update_event(event).await?;
    Ok(StatusCode::OK)
}