use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::events::Event;

#[derive(Deserialize,Serialize)]
pub enum InnerTimeFrameFilter {
    Prev,
    Current,
    Next,
}

#[derive(Deserialize)]
pub enum TimeFrameFilter {
    Day,
    Week(InnerTimeFrameFilter),
    Month(InnerTimeFrameFilter),
}

pub type Filter<'a>=Box<dyn FnMut(&Arc<Event>)->Option<Arc<Event>>+Send+'a>;