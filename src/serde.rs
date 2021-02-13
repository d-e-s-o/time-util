// Copyright (C) 2020-2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[cfg(feature = "chrono-tz")]
use chrono::offset::TimeZone as _;
#[cfg(feature = "chrono-tz")]
use chrono_tz::America::New_York;

use serde::de::Deserializer;
use serde::de::Error;
use serde::de::Unexpected;
use serde::ser::Serializer;
use serde::Deserialize;

use crate::parse::parse_system_time_from_str_impl;
use crate::parse::DATE_PARSE_FNS;
use crate::parse::TIME_PARSE_FNS;
use crate::print::print_system_time_to_rfc3339;


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
/// milliseconds since 1970-01-01 in the New York time zone.
#[cfg(feature = "chrono-tz")]
pub fn system_time_from_millis_in_new_york<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
  D: Deserializer<'de>,
{
  let time = system_time_from_millis(deserializer)?;
  let naive_time = DateTime::<Utc>::from(time).naive_local();
  let ny_time = New_York.from_utc_datetime(&naive_time);
  let utc_time = Utc.from_local_datetime(&ny_time.naive_local()).unwrap();

  Ok(SystemTime::from(utc_time))
}


/// Serialize a `SystemTime` into a RFC3339 time stamp.
pub fn system_time_to_rfc3339<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let string = print_system_time_to_rfc3339(time);
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


/// Serialize a `SystemTime` into a timestamp containing the
/// milliseconds since 1970-01-01 in New York.
#[cfg(feature = "chrono-tz")]
pub fn system_time_to_millis_in_new_york<S>(
  time: &SystemTime,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let utc_time = DateTime::<Utc>::from(*time);
  let ny_time = New_York.from_local_datetime(&utc_time.naive_utc()).unwrap();
  system_time_to_millis(&SystemTime::from(ny_time), serializer)
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::time::SystemTime;

  use serde::Deserialize;
  use serde::Serialize;
  use serde_json::from_str as from_json;
  use serde_json::to_string as to_json;

  #[cfg(feature = "chrono-tz")]
  use crate::parse::parse_system_time_from_str;


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
    assert_eq!(json, r#"{"time":"2018-12-06T20:47:00.000Z"}"#);
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
  #[cfg(feature = "chrono-tz")]
  struct MsTimeNY {
    #[serde(
      deserialize_with = "system_time_from_millis_in_new_york",
      serialize_with = "system_time_to_millis_in_new_york",
    )]
    time: SystemTime,
  }

  #[test]
  #[cfg(feature = "chrono-tz")]
  fn deserialize_serialize_system_time_millis_in_new_york() {
    // This time stamp represents 2018-02-01T00:00:00-05:00:
    // $ date --date='2018-02-01T00:00:00-05:00' +'%s'
    let time = from_json::<MsTimeNY>(r#"{"time": 1517461200000}"#).unwrap();
    let expected = parse_system_time_from_str("2018-02-01T00:00:00.000Z").unwrap();
    assert_eq!(time.time, expected);

    let json = to_json::<MsTimeNY>(&time).unwrap();
    let time = from_json::<MsTimeNY>(&json).unwrap();
    assert_eq!(time.time, expected);
  }

  #[test]
  #[cfg(feature = "chrono-tz")]
  fn deserialize_serialize_system_time_millis_in_new_york_daylight_savings() {
    let time = from_json::<MsTimeNY>(r#"{"time": 1599537600000}"#).unwrap();
    let expected = parse_system_time_from_str("2020-09-08T00:00:00.000Z").unwrap();
    assert_eq!(time.time, expected);

    let json = to_json::<MsTimeNY>(&time).unwrap();
    let time = from_json::<MsTimeNY>(&json).unwrap();
    assert_eq!(time.time, expected);
  }
}
