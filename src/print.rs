// Copyright (C) 2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::SystemTime;

use chrono::offset::Utc;
use chrono::DateTime;
use chrono::SecondsFormat;


/// Print a `SystemTime` as a RFC3339 time stamp.
pub fn print_system_time_to_rfc3339(time: &SystemTime) -> String {
  DateTime::<Utc>::from(*time).to_rfc3339_opts(SecondsFormat::Millis, true)
}


/// Print a `SystemTime` as a RFC3339 time stamp.
pub fn print_system_time_to_rfc3339_with_nanos(time: &SystemTime) -> String {
  // Rust's `SystemTime` internally work with nano seconds and so by
  // doing the same we hope to have no loss of information.
  DateTime::<Utc>::from(*time).to_rfc3339_opts(SecondsFormat::Nanos, true)
}


#[cfg(test)]
mod tests {
  use super::*;

  use crate::parse::parse_system_time_from_str;


  /// Check that we can format a `SystemTime` as a RFC3339 time stamp.
  #[test]
  fn print_rfc3339() {
    let string = "2018-04-01T12:04:17.050Z";
    let time = parse_system_time_from_str(string).unwrap();
    let result = print_system_time_to_rfc3339(&time);
    assert_eq!(result, string)
  }


  /// Check that we can format a `SystemTime` as a RFC3339 time stamp
  /// with nanosecond resolution.
  #[test]
  fn print_rfc3339_with_nanos() {
    let string = "2018-04-01T12:04:17.050937231Z";
    let time = parse_system_time_from_str(string).unwrap();
    let result = print_system_time_to_rfc3339_with_nanos(&time);
    assert_eq!(result, string)
  }
}
