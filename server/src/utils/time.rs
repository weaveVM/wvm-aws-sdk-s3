use chrono::{DateTime, NaiveDateTime, Utc};

pub fn to_rfc_7231_datetime(input: &str) -> Option<String> {
    let naive_dt = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S").ok()?;
    let utc_dt: DateTime<Utc> = DateTime::<Utc>::from_utc(naive_dt, Utc);
    Some(utc_dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
}
