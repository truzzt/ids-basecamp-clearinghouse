pub mod crypto;
pub mod document;
pub mod process;

#[cfg(test)]
mod tests;

pub fn new_uuid() -> String {
    use uuid::Uuid;
    Uuid::new_v4().hyphenated().to_string()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, FromFormField)]
pub enum SortingOrder {
    #[field(value = "asc")]
    #[serde(rename = "asc")]
    Ascending,
    #[field(value = "desc")]
    #[serde(rename = "desc")]
    Descending,
}

pub fn parse_date(date: Option<String>, to_date: bool) -> Option<chrono::NaiveDateTime> {
    let time_format;
    if to_date {
        time_format = "23:59:59"
    } else {
        time_format = "00:00:00"
    }

    match date {
        Some(d) => {
            debug!("Parsing date: {}", &d);
            match chrono::NaiveDateTime::parse_from_str(format!("{} {}", &d, &time_format).as_str(), "%Y-%m-%d %H:%M:%S") {
                Ok(date) => {
                    Some(date)
                }
                Err(e) => {
                    error!("Error occurred: {:#?}", e);
                    return None;
                }
            }
        }
        None => None
    }
}

pub fn sanitize_dates(date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> (chrono::NaiveDateTime, chrono::NaiveDateTime) {
    let default_to_date = chrono::Local::now().naive_local();
    let default_from_date = default_to_date.date()
        .and_hms_opt(0, 0, 0)
        .expect("00:00:00 is a valid time") - chrono::Duration::weeks(2);

    println!("date_to: {:#?}", date_to);
    println!("date_from: {:#?}", date_from);

    println!("Default date_to: {:#?}", default_to_date);
    println!("Default date_from: {:#?}", default_from_date);

    match (date_from, date_to) {
        (Some(from), Some(to)) => (from, to), // validate already checked that date_from > date_to
        (Some(from), None) => (from, default_to_date), // if to_date is missing, default to now
        (None, Some(_to)) => todo!("Not defined yet; check"),
        (None, None) => (default_from_date, default_to_date), // if both dates are none (case to_date is none and from_date is_some should be catched by validation); return dates for default duration (last 2 weeks)
    }
}

pub fn validate_dates(date_from: Option<chrono::NaiveDateTime>, date_to: Option<chrono::NaiveDateTime>) -> bool {
    let date_now = chrono::Local::now().naive_local();
    debug!("... validating dates: now: {:#?} , from: {:#?} , to: {:#?}", &date_now, &date_from, &date_to);
    // date_from before now
    if date_from.is_some() && date_from.as_ref().unwrap().clone() > date_now {
        debug!("oh no, date_from {:#?} is in the future! date_now is {:#?}", &date_from, &date_now);
        return false;
    }

    // date_to only if there is also date_from
    if date_from.is_none() && date_to.is_some() {
        return false;
    }

    // date_to before or equals now
    if date_to.is_some() && date_to.as_ref().unwrap().clone() >= date_now {
        debug!("oh no, date_to {:#?} is in the future! date_now is {:#?}", &date_to, &date_now);
        return false;
    }

    // date_from before date_to
    if date_from.is_some() && date_to.is_some() {
        if date_from.unwrap() > date_to.unwrap() {
            debug!("oh no, date_from {:#?} is before date_to {:#?}", &date_from, &date_to);
            return false;
        }
    }
    return true;
}