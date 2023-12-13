pub mod claims;
pub mod constants;
pub(crate) mod crypto;
pub(crate) mod doc_type;
pub(crate) mod document;
pub mod ids;
pub mod process;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SortingOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// Parses a date string into a `chrono::NaiveDateTime` object. If `to_date` is true, the time will be set to 23:59:59, otherwise it is 00:00:00.
pub fn parse_date(date: Option<String>, to_date: bool) -> Option<chrono::NaiveDateTime> {
    // If it is a to_date, we want to set the time to 23:59:59, otherwise it is 00:00:00
    let time: chrono::NaiveTime = if to_date {
        chrono::NaiveTime::from_hms_opt(23, 59, 59).expect("23:59:59 is a valid time")
    } else {
        chrono::NaiveTime::from_hms_opt(0, 0, 0).expect("00:00:00 is a valid time")
    };

    match date {
        Some(d) => {
            debug!("Parsing date: {}", &d);
            match chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                Ok(date) => Some(date.and_time(time)),
                Err(e) => {
                    error!("Parsing date '{d}' failed: {:#?}", e);
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
        let date_now_midnight = date_now
            .date()
            .and_hms_opt(0, 0, 0)
            .expect("00:00:00 is a valid time");
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

    #[test]
    fn parse_date() {
        let wrong_date = Some("2020-13-01".to_string());
        let valid_date = Some("2020-01-01".to_string());
        let valid_date_parsed = chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("This is valid");
        let day_start_time = chrono::NaiveTime::from_hms_opt(0, 0, 0).expect("This is valid");
        let day_end_time = chrono::NaiveTime::from_hms_opt(23, 59, 59).expect("This is valid");

        assert!(super::parse_date(wrong_date, false).is_none());
        assert_eq!(
            super::parse_date(valid_date.clone(), false),
            Some(valid_date_parsed.and_time(day_start_time))
        );
        assert_eq!(
            super::parse_date(valid_date, true),
            Some(valid_date_parsed.and_time(day_end_time))
        );
    }
}
