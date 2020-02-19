// Copyright (C) 2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use math_util::round_up;


/// The number of seconds in a day.
const DAY_SECS: u32 = 24 * 60 * 60;


fn next_day_duration(now: SystemTime) -> Duration {
  // `UNIX_EPOCH` is the first time stamp that can be represented, so
  // there is no way `SystemTime::duration_since` can ever fail with it
  // as a parameter.
  let duration = now.duration_since(UNIX_EPOCH).unwrap();
  let next_day = round_up(duration.as_secs(), DAY_SECS.into());
  let duration = Duration::from_secs(next_day);
  duration
}

/// Calculate the time stamp representing the next day of the given time
/// stamp.
///
/// Note that currently a time stamp marking precisely midnight will not
/// advance to the next day.
// TODO: We should fix this behavior.
pub fn next_day(now: SystemTime) -> SystemTime {
  let duration = next_day_duration(now);
  UNIX_EPOCH + duration
}

pub fn days_back_from(now: SystemTime, count: u32) -> SystemTime {
  let duration = next_day_duration(now) - Duration::from_secs(DAY_SECS.into()) * (count + 1);
  UNIX_EPOCH + duration
}

pub fn days_back(count: u32) -> SystemTime {
  days_back_from(SystemTime::now(), count)
}

/// Calculate a `SystemTime` representing 0:00:00 (i.e., the first
/// second of) the next day.
pub fn tomorrow() -> SystemTime {
  next_day(SystemTime::now())
}


#[cfg(test)]
pub mod tests {
  use super::*;

  use crate::parse_system_time_from_str;


  #[test]
  fn calculate_next_day() {
    let now = parse_system_time_from_str("2020-02-07T13:00:00Z").unwrap();
    let tomorrow = parse_system_time_from_str("2020-02-08T00:00:00Z").unwrap();
    assert_eq!(next_day(now), tomorrow);

    let now = parse_system_time_from_str("2019-03-31T23:59:59Z").unwrap();
    let tomorrow = parse_system_time_from_str("2019-04-01T00:00:00Z").unwrap();
    assert_eq!(next_day(now), tomorrow);
  }

  #[test]
  fn calculate_prev_days() {
    let now = parse_system_time_from_str("2020-02-07T09:00:00Z").unwrap();
    let yesterday = parse_system_time_from_str("2020-02-06T00:00:00Z").unwrap();
    assert_eq!(days_back_from(now, 1), yesterday);

    let now = parse_system_time_from_str("2020-02-02T23:59:59Z").unwrap();
    let five_ago = parse_system_time_from_str("2020-01-28T00:00:00Z").unwrap();
    assert_eq!(days_back_from(now, 5), five_ago);
  }
}
