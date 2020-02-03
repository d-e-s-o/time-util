// Copyright (C) 2019-2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A crate for more or less frequently used time parsing and formatting
//! functionality.
//!
//! The crate revolves around the `std::time::SystemTime` type in that
//! we attempt to convert into that or use this as the base to convert
//! from. We treat such a time as having no associated time zone. Think
//! of it as being in UTC.

mod timezone;

use std::convert::TryInto;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use chrono::naive::NaiveDate;
use chrono::offset::FixedOffset;
use chrono::offset::TimeZone as _;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::ParseError;

use serde::de::Deserializer;
use serde::de::Error;
use serde::de::Unexpected;
use serde::ser::Serializer;
use serde::Deserialize;

use crate::timezone::TimeZone;

/// The Eastern Standard Time time zone.
pub use crate::timezone::EST;
/// The Coordinated Universal Time time zone.
pub use crate::timezone::UTC;


type DateFn = fn(&str) -> Result<DateTime<FixedOffset>, ParseError>;

/// The list of time stamp formats we support.
const TIME_PARSE_FNS: [DateFn; 3] = [
  |s| FixedOffset::east(0).datetime_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ"),
  |s| FixedOffset::east(0).datetime_from_str(s, "%Y-%m-%dT%H:%M:%SZ"),
  |s| DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%z"),
];

const DATE_PARSE_FNS: [DateFn; 1] = [|s| {
  NaiveDate::parse_from_str(s, "%Y-%m-%d").and_then(|date| {
    Ok(DateTime::from_utc(
      date.and_hms(0, 0, 0),
      FixedOffset::east(0),
    ))
  })
}];


/// Parse a `SystemTime` from a string using any of the provided parsing
/// functions.
fn parse_system_time_from_str_impl(time: &str, parse_fns: &[DateFn]) -> Option<SystemTime> {
  for parse_fn in parse_fns {
    // Ideally we would want to only continue in case of
    // ParseErrorKind::Invalid. However, that member is private...
    let datetime = match parse_fn(&time) {
      Ok(datetime) => datetime,
      Err(_) => continue,
    };

    let sec = datetime.timestamp();
    let nsec = datetime.timestamp_subsec_nanos();
    let systime = if sec < 0 {
      UNIX_EPOCH - Duration::new(-sec as u64, 0) + Duration::new(0, nsec)
    } else {
      UNIX_EPOCH + Duration::new(sec as u64, nsec)
    };
    return Some(systime)
  }
  None
}


/// Parse a `SystemTime` from a string.
pub fn parse_system_time_from_str(time: &str) -> Option<SystemTime> {
  parse_system_time_from_str_impl(&time, &TIME_PARSE_FNS)
}


/// Deserialize a time stamp as a `SystemTime`.
pub fn system_time_from_str<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
  D: Deserializer<'de>,
{
  let time = String::deserialize(deserializer)?;
  parse_system_time_from_str_impl(&time, &TIME_PARSE_FNS)
    .ok_or_else(|| Error::invalid_value(Unexpected::Str(&time), &"a time stamp string"))
}


/// Deserialize an optional time stamp.
pub fn optional_system_time_from_str<'de, D>(
  deserializer: D,
) -> Result<Option<SystemTime>, D::Error>
where
  D: Deserializer<'de>,
{
  match Option::<String>::deserialize(deserializer)? {
    Some(time) => parse_system_time_from_str_impl(&time, &TIME_PARSE_FNS)
      .ok_or_else(|| Error::invalid_value(Unexpected::Str(&time), &"an optional time stamp string"))
      .map(Option::Some),
    None => Ok(None),
  }
}


/// Deserialize a `SystemTime` from a date.
pub fn system_time_from_date_str<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
  D: Deserializer<'de>,
{
  let date = String::deserialize(deserializer)?;
  parse_system_time_from_str_impl(&date, &DATE_PARSE_FNS)
    .ok_or_else(|| Error::invalid_value(Unexpected::Str(&date), &"a date string"))
}


/// Deserialize a `SystemTime` from a UNIX time stamp.
pub fn system_time_from_secs<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
  D: Deserializer<'de>,
{
  let seconds = u64::deserialize(deserializer)?;
  let time = UNIX_EPOCH + Duration::new(seconds, 0);
  Ok(time)
}


/// Deserialize a `SystemTime` from a timestamp containing the
/// milliseconds since 1970-01-01.
pub fn system_time_from_millis<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
  D: Deserializer<'de>,
{
  let ms = u64::deserialize(deserializer)?;
  let time = UNIX_EPOCH + Duration::from_millis(ms);
  Ok(time)
}


/// Deserialize a `SystemTime` from a timestamp containing the
/// milliseconds since 1970-01-01 in a given time zone.
///
/// The given time zone type specifies the time zone in which the
/// to-be-parsed time stamp is provided in. It will then be converted to
/// UTC.
pub fn system_time_from_millis_in_tz<'de, TZ, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
  D: Deserializer<'de>,
  TZ: TimeZone,
{
  system_time_from_millis(deserializer).map(TZ::add)
}


/// Serialize a `SystemTime` into a RFC3339 time stamp.
pub fn system_time_to_rfc3339<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let duration = time.duration_since(UNIX_EPOCH).unwrap();
  let secs = duration.as_secs().try_into().unwrap();
  let nanos = duration.subsec_nanos();
  let string = Utc.timestamp(secs, nanos).to_rfc3339();

  serializer.serialize_str(&string)
}

/// Serialize an optional `SystemTime` into a RFC3339 time stamp.
pub fn optional_system_time_to_rfc3339<S>(
  time: &Option<SystemTime>,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  match time {
    Some(time) => system_time_to_rfc3339(time, serializer),
    None => serializer.serialize_none(),
  }
}


/// Serialize a `SystemTime` into a timestamp containing the
/// milliseconds since 1970-01-01.
pub fn system_time_to_millis<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  // It should be safe to unwrap here given that there is absolutely no
  // way for a time stamp to ever point to a time before `UNIX_EPOCH`
  // and that the only (documented) error case for `duration_since`.
  let millis = time.duration_since(UNIX_EPOCH).unwrap().as_millis();
  serializer.serialize_u128(millis)
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::time::SystemTime;

  use serde::Deserialize;
  use serde::Serialize;
  use serde_json::from_str as from_json;
  use serde_json::to_string as to_json;


  #[test]
  fn parse_time() {
    let time = parse_system_time_from_str("2018-04-01T12:00:00.000Z").unwrap();
    let expected = UNIX_EPOCH + Duration::from_secs(1522584000);
    assert_eq!(time, expected)
  }

  #[derive(Debug, Deserialize)]
  struct Time {
    #[serde(deserialize_with = "system_time_from_str")]
    time: SystemTime,
  }

  #[test]
  fn deserialize_system_time_from_str() {
    let times = [
      r#"{"time": "2018-04-01T12:00:00Z"}"#,
      r#"{"time": "2018-04-01T12:00:00.000Z"}"#,
      r#"{"time": "2018-04-01T08:00:00.000-04:00"}"#,
    ];

    for json in &times {
      let time = from_json::<Time>(json).unwrap();
      assert_eq!(time.time, UNIX_EPOCH + Duration::from_secs(1522584000));
    }
  }

  #[derive(Debug, Deserialize)]
  struct Date {
    #[serde(deserialize_with = "system_time_from_date_str")]
    date: SystemTime,
  }

  #[test]
  fn deserialize_system_time_from_date_str() {
    let dates = [r#"{"date": "2019-08-01"}"#];

    for json in &dates {
      let date = from_json::<Date>(json).unwrap();
      assert_eq!(date.date, UNIX_EPOCH + Duration::from_secs(1564617600));
    }
  }


  #[derive(Debug, Deserialize, Serialize)]
  struct OtherTime {
    #[serde(
      deserialize_with = "system_time_from_secs",
      serialize_with = "system_time_to_rfc3339",
    )]
    time: SystemTime,
  }

  #[test]
  fn deserialize_system_time_from_secs() {
    let time = from_json::<OtherTime>(r#"{"time": 1544129220}"#).unwrap();
    assert_eq!(time.time, UNIX_EPOCH + Duration::from_secs(1544129220));
  }

  #[test]
  fn serialize_system_time_to_rfc3339() {
    let time = OtherTime {
      time: UNIX_EPOCH + Duration::from_secs(1544129220),
    };
    let json = to_json(&time).unwrap();
    assert_eq!(json, r#"{"time":"2018-12-06T20:47:00+00:00"}"#);
  }

  #[derive(Debug, Deserialize, Serialize)]
  struct MsTime {
    #[serde(
      deserialize_with = "system_time_from_millis",
      serialize_with = "system_time_to_rfc3339",
    )]
    time: SystemTime,
  }

  #[test]
  fn deserialize_system_time_from_millis() {
    let time = from_json::<MsTime>(r#"{"time": 1517461200000}"#).unwrap();
    assert_eq!(time.time, UNIX_EPOCH + Duration::from_secs(1517461200));
  }


  #[derive(Debug, Deserialize, Serialize)]
  struct MsTimeEST {
    #[serde(
      deserialize_with = "system_time_from_millis_in_tz::<EST, _>",
      serialize_with = "system_time_to_rfc3339",
    )]
    time: SystemTime,
  }

  #[test]
  fn deserialize_system_time_from_millis_in_tz() {
    // This time stamp represents 2018-02-01T00:00:00-05:00:
    // $ date --date='2018-02-01T00:00:00-05:00' +'%s'
    let time = from_json::<MsTimeEST>(r#"{"time": 1517461200000}"#).unwrap();
    let expected = parse_system_time_from_str("2018-02-01T00:00:00.000Z").unwrap();
    assert_eq!(time.time, expected);
  }
}
