use std::sync::Arc;

use chrono::{Datelike, Duration, Months, NaiveDate};

use crate::domain::{errors::CalendarErrors, events::Event, filters::{Filter, InnerTimeFrameFilter, TimeFrameFilter}};



pub fn create_filter<'a>(date:NaiveDate,time_frame:TimeFrameFilter)->Result<Filter<'a>,CalendarErrors>{
    match time_frame {
        TimeFrameFilter::Day => {
            println!("Day");
            let filter = move |event: &Arc<Event>| {
                if event.date == date {
                    return Some(event.clone());
                }
                None
            };
            Ok(Box::new(filter)) 
        }
        TimeFrameFilter::Month(inner_time_frame) => match inner_time_frame {
            InnerTimeFrameFilter::Current => {
            println!("Montch Current");
                let filter = move |event: &Arc<Event>| {
                    if event.date.year() == date.year() && date.month()==event.date.month() {
                        return Some(event.clone());
                    }
                    None
                };
                Ok(Box::new(filter)) 
            }
            InnerTimeFrameFilter::Next => {
            println!("Montch next");
                let to_date=date+Months::new(1);
                let filter = move |event: &Arc<Event>| {

                        if event.date >= date && event.date <= to_date {
                            return Some(event.clone());
                        }
                        None
                    
                };
                Ok(Box::new(filter)) 
            }
            InnerTimeFrameFilter::Prev => {
            println!("Montch Prev");
            let from_date=date-Months::new(1);
                let filter = move |event: &Arc<Event>| {
  
                        if event.date >= from_date && event.date <= date {
                            return Some(event.clone());
                        }
                        None
                };
                Ok(Box::new(filter)) 
            }
        },
        TimeFrameFilter::Week(inner_time_frame) => match inner_time_frame {
            InnerTimeFrameFilter::Current => {
            println!("Week Current");
                let filter = move |event: &Arc<Event>| {
                    let week = date.iso_week().week();
                    if event.date.iso_week().week() == week &&event.date.year()==date.year() {
                        return Some(event.clone());
                    }
                    None
                };
                Ok(Box::new(filter)) 

            }
            InnerTimeFrameFilter::Next=>{
            println!("Week Next");
                let filter = move |event: &Arc<Event>| {
                    let to_date=date+Duration::days(7);
                    if event.date>=date && event.date<=to_date {
                        return Some(event.clone());
                    }
                    None
                };
                Ok(Box::new(filter)) 
            }
            InnerTimeFrameFilter::Prev=>{
            println!("Week Prev");
                let filter = move |event: &Arc<Event>| {
                    let from_date=date-Duration::days(7);
                    if event.date>=from_date && event.date<=date {
                        return Some(event.clone());
                    }
                    None
                };
                Ok(Box::new(filter))
            }
        }
    }

}


pub fn serialize(events:Vec<Arc<Event>>)->Result<String,CalendarErrors>{
    serde_json::to_string_pretty(&events).map_err(|err|{
        CalendarErrors::SerializeError(err.to_string())
    })
}
