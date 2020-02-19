// Copyright (C) 2019-2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A crate for more or less frequently used time parsing and formatting
//! functionality.
//!
//! The crate revolves around the `std::time::SystemTime` type in that
//! we attempt to convert into that or use this as the base to convert
//! from. We treat such a time as having no associated time zone. Think
//! of it as being in UTC.

#[cfg(feature = "math")]
mod math;
#[cfg(any(test, feature = "chrono"))]
mod parse;
#[cfg(any(test, all(feature = "chrono", feature = "serde")))]
mod serde;
#[cfg(any(test, feature = "chrono"))]
mod timezone;

#[cfg(not(any(feature = "math", feature = "chrono", feature = "serde")))]
compile_error!("Please specify one of the available features: math, chrono, or serde");

#[cfg(feature = "math")]
pub use crate::math::{
  days_back,
  days_back_from,
  next_day,
  tomorrow,
};

#[cfg(feature = "chrono")]
pub use crate::parse::{
  parse_system_time_from_date_str,
  parse_system_time_from_str,
};

#[cfg(all(feature = "chrono", feature = "serde"))]
pub use crate::serde::{
  optional_system_time_from_str,
  optional_system_time_to_rfc3339,
  system_time_from_date_str,
  system_time_from_millis,
  system_time_from_millis_in_tz,
  system_time_from_secs,
  system_time_from_str,
  system_time_to_millis,
  system_time_to_millis_in_tz,
  system_time_to_rfc3339,
};

#[cfg(feature = "chrono")]
pub use crate::timezone::{
  EST,
  UTC,
};
