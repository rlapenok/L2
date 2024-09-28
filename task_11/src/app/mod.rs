use std::env::{self, VarError};
use std::error::Error;

use axum::middleware::from_fn;
use axum::routing::{get, post, IntoMakeService};
use axum::Router;

use crate::api::logger_middleware::logger_middleware;
use crate::infrastructe::state::ServerState;
use crate::api::{create_event::create_event, delete_event::delete_event, events_for_day::events_for_day, events_for_month::events_for_month, events_for_week::events_for_week, update_event::update_event};


pub fn create_app()->IntoMakeService<Router>{
    let state=ServerState::new();

    Router::new()
    .route("/create_event", post(create_event))
    .route("/update_event",post(update_event))
    .route("/delete_event", post(delete_event))
    .route("/events_for_day", get(events_for_day))
    .route("/events_for_week", get(events_for_week))
    .route("/events_for_month",get(events_for_month))
    .layer(from_fn(logger_middleware))
    .with_state(state).into_make_service()
}

pub fn get_address_from_env()->Result<String,Box<dyn Error>>{

    let host=env::var("HOST")?;
    let port=env::var("PORT")?.parse::<u16>()?;
    Ok(format!("{}:{}",host,port))
}

pub fn get_logginig_level()->Result<String,VarError>{

    env::var("LEVEL")

}