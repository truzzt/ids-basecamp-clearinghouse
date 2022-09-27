use std::sync::RwLock;
use biscuit::Empty;
use biscuit::jwk::JWKSet;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};

pub mod crypto;
pub mod document;
pub mod process;

#[cfg(test)] mod tests;

pub fn new_uuid() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_hyphenated().to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFormField)]
pub enum SortingOrder{
    #[field(value = "asc")]
    #[serde(rename = "asc")]
    Ascending,
    #[field(value = "desc")]
    #[serde(rename = "desc")]
    Descending
}

#[derive(Debug)]
pub struct JwksCache{
    pub jwks: RwLock<Option<JWKSet<Empty>>>
}

impl JwksCache{
    pub fn new() -> JwksCache{
        JwksCache{
            jwks: RwLock::new(None)
        }
    }
}

pub fn parse_date(date: Option<String>, to_date: bool) -> Option<NaiveDateTime>{
    let time_format;
    if to_date{
        time_format = "23:59:59"
    }
    else{
        time_format = "00:00:00"
    }

    match date{
        Some(d) => {
            debug!("Parsing date: {}", &d);
            match NaiveDateTime::parse_from_str(format!("{} {}",&d, &time_format).as_str(), "%Y-%m-%d %H:%M:%S"){
                Ok(date) => {
                    Some(date)
                }
                Err(e) => {
                    error!("Error occurred: {:#?}", e);
                    return None
                }
            }
        }
        None => None
    }
}

pub fn sanitize_dates(date_from: Option<NaiveDateTime>, date_to: Option<NaiveDateTime>) -> (NaiveDateTime, NaiveDateTime){
    let default_to_date = Local::now().naive_local();
    let d = NaiveDate::from_ymd(default_to_date.year(), default_to_date.month(), default_to_date.day());
    let t = NaiveTime::from_hms(0, 0, 0);
    let default_from_date = NaiveDateTime::new(d,t) - Duration::weeks(2);

    println!("date_to: {:#?}", date_to);
    println!("date_from: {:#?}", date_from);

    println!("Default date_to: {:#?}", default_to_date);
    println!("Default date_from: {:#?}", default_from_date);

    // validate already checked that date_from > date_to
    if date_from.is_some() && date_to.is_some(){
        return (date_from.unwrap(), date_to.unwrap())
    }

    // if to_date is missing, default to now
    if date_from.is_some() && date_to.is_none(){
        return (date_from.unwrap(), default_to_date)
    }

    // if both dates are none (case to_date is none and from_date is_some should be catched by validation)
    // return dates for default duration (last 2 weeks)
    return (default_from_date, default_to_date)
}

pub fn validate_dates(date_from: Option<NaiveDateTime>, date_to: Option<NaiveDateTime>) -> bool{
    let date_now = Local::now().naive_local();
    debug!("... validating dates: now: {:#?} , from: {:#?} , to: {:#?}", &date_now, &date_from, &date_to);
    // date_from before now
    if date_from.is_some() && date_from.as_ref().unwrap().clone() > date_now{
        debug!("oh no, date_from {:#?} is in the future! date_now is {:#?}", &date_from, &date_now);
        return false;
    }

    // date_to only if there is also date_from
    if date_from.is_none() && date_to.is_some() {
        return false;
    }

    // date_to before or equals now
    if date_to.is_some() && date_to.as_ref().unwrap().clone() >= date_now{
        debug!("oh no, date_to {:#?} is in the future! date_now is {:#?}", &date_to, &date_now);
        return false;
    }

    // date_from before date_to
    if date_from.is_some() && date_to.is_some(){
        if date_from.unwrap() > date_to.unwrap() {
            debug!("oh no, date_from {:#?} is before date_to {:#?}", &date_from, &date_to);
            return false;
        }
    }
    return true;
}