pub(crate) mod claims;
pub mod constants;
pub(crate) mod crypto;
pub(crate) mod doc_type;
pub(crate) mod document;
pub mod ids;
pub(crate) mod process;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SortingOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

pub fn parse_date(date: Option<String>, to_date: bool) -> Option<chrono::NaiveDateTime> {
    let time_format = if to_date { "23:59:59" } else { "00:00:00" };

    match date {
        Some(d) => {
            debug!("Parsing date: {}", &d);
            match chrono::NaiveDateTime::parse_from_str(
                format!("{} {}", &d, &time_format).as_str(),
                "%Y-%m-%d %H:%M:%S",
            ) {
                Ok(date) => Some(date),
                Err(e) => {
                    error!("Error occurred: {:#?}", e);
                    None
                }
            }
        }
        None => None,
    }
}

/// Validates the provided dates. `date_now` is optional and defaults to `chrono::Local::now().naive_local()`.
pub fn validate_and_sanitize_dates(
    date_from: Option<chrono::NaiveDateTime>,
    date_to: Option<chrono::NaiveDateTime>,
    date_now: Option<chrono::NaiveDateTime>,
) -> anyhow::Result<(chrono::NaiveDateTime, chrono::NaiveDateTime)> {
    let now = date_now.unwrap_or(chrono::Local::now().naive_local());
    debug!(
        "... validating dates: now: {:#?} , from: {:#?} , to: {:#?}",
        &now, &date_from, &date_to
    );

    let default_to_date = now;
    let default_from_date = default_to_date
        .date()
        .and_hms_opt(0, 0, 0)
        .expect("00:00:00 is a valid time")
        - chrono::Duration::weeks(2);

    println!("date_to: {:#?}", date_to);
    println!("date_from: {:#?}", date_from);

    println!("Default date_to: {:#?}", default_to_date);
    println!("Default date_from: {:#?}", default_from_date);

    match (date_from, date_to) {
        (Some(from), None) if from < now => Ok((from, default_to_date)),
        (Some(from), Some(to)) if from < now && to <= now && from < to => Ok((from, to)),
        (None, None) => Ok((default_from_date, default_to_date)),
        _ => Err(anyhow::anyhow!("Invalid date parameters")),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn validate_and_sanitize_dates() {
        // Setup dates for testing
        let date_now = chrono::Local::now().naive_local();
        let date_now_midnight = date_now.date().and_hms_opt(0, 0, 0).unwrap();
        let date_from = date_now_midnight - chrono::Duration::weeks(2);
        let date_to = date_now_midnight - chrono::Duration::weeks(1);

        // # Good cases
        assert_eq!(
            (date_from, date_now),
            super::validate_and_sanitize_dates(None, None, Some(date_now))
                .expect("Should be valid")
        );
        assert_eq!(
            (date_from, date_now),
            super::validate_and_sanitize_dates(Some(date_from), None, Some(date_now))
                .expect("Should be valid")
        );
        assert_eq!(
            (date_from, date_to),
            super::validate_and_sanitize_dates(Some(date_from), Some(date_to), Some(date_now))
                .expect("Should be valid")
        );
        assert_eq!(
            (date_from, date_to),
            super::validate_and_sanitize_dates(Some(date_from), Some(date_to), Some(date_to))
                .expect("Should be valid")
        );

        // # Bad cases
        // no to without from not satisfied
        assert!(super::validate_and_sanitize_dates(None, Some(date_to), Some(date_now)).is_err());
        // from < now not satisfied
        assert!(super::validate_and_sanitize_dates(Some(date_now), None, Some(date_to)).is_err());
        // from < to not satisfied
        assert!(
            super::validate_and_sanitize_dates(Some(date_to), Some(date_from), Some(date_now))
                .is_err()
        );
        // from < to not satisfied
        assert!(
            super::validate_and_sanitize_dates(Some(date_to), Some(date_to), Some(date_now))
                .is_err()
        );
        // to < now not satisfied
        assert!(
            super::validate_and_sanitize_dates(Some(date_from), Some(date_now), Some(date_to))
                .is_err()
        );
        // from < now && to < now not satisfied
        assert!(
            super::validate_and_sanitize_dates(Some(date_to), Some(date_now), Some(date_from))
                .is_err()
        );
    }
}
