// Copyright (C) 2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;
use std::time::SystemTime;


/// An enumeration describing a timezone offset.
pub enum Offset {
  /// There is no offset, i.e., the time zone is UTC.
  None,
  /// An offset from UTC (in seconds) in the western hemisphere.
  ///
  /// E.g., a value of `1 * 60 * 60` maps to UTC-01:00
  West(u16),
  /// An offset from UTC (in seconds) in the eastern hemisphere.
  ///
  /// E.g., a value of `1 * 60 * 60` maps to UTC+01:00
  East(u16),
}


/// A trait representing a timezone and operations defined for it.
pub trait TimeZone {
  const OFFSET: Offset;

  /// Correct a system time by adding our offset.
  fn add(time: SystemTime) -> SystemTime {
    match Self::OFFSET {
      Offset::None => time,
      Offset::West(offset) => time - Duration::from_secs(offset.into()),
      Offset::East(offset) => time + Duration::from_secs(offset.into()),
    }
  }
}


/// The UTC timezone.
pub struct UTC {}

impl TimeZone for UTC {
  const OFFSET: Offset = Offset::None;
}


/// The eastern standard time zone.
pub struct EST {}

impl TimeZone for EST {
  const OFFSET: Offset = Offset::West(5 * 60 * 60);
}


#[cfg(test)]
mod tests {
  use super::*;

  use crate::parse_system_time_from_str;


  #[test]
  fn time_zone_correction() {
    let est_time = parse_system_time_from_str("2018-04-01T08:00:37.000-05:00").unwrap();
    let utc_time = parse_system_time_from_str("2018-04-01T08:00:37.000-00:00").unwrap();
    let expected = parse_system_time_from_str("2018-04-01T08:00:37.000Z").unwrap();

    assert_eq!(EST::add(est_time), expected);
    assert_eq!(UTC::add(utc_time), expected);
  }
}
