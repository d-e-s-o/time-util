// Copyright (C) 2020-2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::SystemTime;

use chrono::naive::NaiveDate;
use chrono::offset::FixedOffset;
use chrono::offset::TimeZone as _;
use chrono::DateTime;
use chrono::ParseError;


type DateFn = fn(&str) -> Result<DateTime<FixedOffset>, ParseError>;


/// The list of time stamp formats we support.
pub(crate) const TIME_PARSE_FNS: [DateFn; 3] = [
  |s| FixedOffset::east(0).datetime_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ"),
  |s| FixedOffset::east(0).datetime_from_str(s, "%Y-%m-%dT%H:%M:%SZ"),
  |s| DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%z"),
];

pub(crate) const DATE_PARSE_FNS: [DateFn; 1] = [|s| {
  NaiveDate::parse_from_str(s, "%Y-%m-%d")
    .map(|date| DateTime::from_utc(date.and_hms(0, 0, 0), FixedOffset::east(0)))
}];


/// Parse a `SystemTime` from a string using any of the provided parsing
/// functions.
pub(crate) fn parse_system_time_from_str_impl(
  time: &str,
  parse_fns: &[DateFn],
) -> Option<SystemTime> {
  for parse_fn in parse_fns {
    // Ideally we would want to only continue in case of
    // ParseErrorKind::Invalid. However, that member is private...
    let datetime = match parse_fn(&time) {
      Ok(datetime) => datetime,
      Err(_) => continue,
    };
    return Some(SystemTime::from(
      datetime.with_timezone(&FixedOffset::east(0)),
    ))
  }
  None
}


/// Parse a `SystemTime` from a string.
pub fn parse_system_time_from_str(time: &str) -> Option<SystemTime> {
  parse_system_time_from_str_impl(&time, &TIME_PARSE_FNS)
}


/// Parse a `SystemTime` from a date string.
pub fn parse_system_time_from_date_str(time: &str) -> Option<SystemTime> {
  parse_system_time_from_str_impl(&time, &DATE_PARSE_FNS)
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::time::Duration;
  use std::time::UNIX_EPOCH;


  #[test]
  fn parse_time() {
    let time = parse_system_time_from_str("2018-04-01T12:00:00.000Z").unwrap();
    let expected = UNIX_EPOCH + Duration::from_secs(1_522_584_000);
    assert_eq!(time, expected)
  }

  #[test]
  fn parse_date() {
    let time = parse_system_time_from_date_str("2019-08-01").unwrap();
    let expected = UNIX_EPOCH + Duration::from_secs(1_564_617_600);
    assert_eq!(time, expected)
  }
}
