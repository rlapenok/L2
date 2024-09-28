use axum::extract::{Query, State as AxumState};

use crate::{domain::{errors::CalendarErrors, events::GetEvents, filters::TimeFrameFilter, note_service::NoteService }, infrastructe::{state::ServerState as ServerState, utils::serialize}};

use super::data_for_request::QueryParams;


pub async fn events_for_month(AxumState(state):AxumState<ServerState>,Query(req):Query<QueryParams>)->Result<String,CalendarErrors>{
    let inner_filter=req.inner_filter.ok_or(CalendarErrors::InvalidFilter)?;
    let filter=GetEvents{
        user_id:req.user_id,
        date:req.date,
        filter:TimeFrameFilter::Month(inner_filter)
    };
    let events=state.get_events(filter).await?;
    let events=serialize(events)?;
    Ok(events)
}