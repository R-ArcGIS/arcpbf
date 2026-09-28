use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, ParseError};

// A date-only value as days since the Unix epoch, R's `Date`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateOnly(pub i64);

// An offset timestamp as UTC milliseconds since the Unix epoch; the offset itself is dropped
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimestampOffset(pub i64);

impl DateOnly {
    pub fn days(self) -> f64 {
        self.0 as f64
    }
}

impl TimestampOffset {
    pub fn seconds(self) -> f64 {
        self.0 as f64 / 1000.0
    }
}

// Parses `YYYY-MM-DD`
impl FromStr for DateOnly {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
        Ok(DateOnly(
            date.signed_duration_since(NaiveDate::default()).num_days(),
        ))
    }
}

// Parses `YYYY-MM-DDTHH:MM:SS[.fff](Z|±HH:MM)`, reading a missing offset as UTC
impl FromStr for TimestampOffset {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let utc = match DateTime::parse_from_rfc3339(s) {
            Ok(dt) => dt.to_utc(),
            Err(_) => NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")?.and_utc(),
        };
        Ok(TimestampOffset(utc.timestamp_millis()))
    }
}

#[cfg(test)]
mod tests;
