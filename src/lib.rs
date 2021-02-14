// Copyright (C) 2019-2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A crate for more or less frequently used time parsing and formatting
//! functionality.
//!
//! The crate revolves around the `std::time::SystemTime` type in that
//! we attempt to convert into that or use this as the base to convert
//! from. We treat such a time as having no associated time zone. Think
//! of it as being in UTC.

#[allow(clippy::let_and_return)]

#[cfg(feature = "math")]
mod math;
#[cfg(any(test, feature = "chrono"))]
mod parse;
#[cfg(any(test, feature = "chrono"))]
mod print;
#[cfg(any(test, all(feature = "chrono", feature = "serde")))]
mod serde;

// We treat chrono-tz as optional on top of chrono.
#[cfg(not(any(feature = "math", feature = "chrono", feature = "serde")))]
compile_error!("Please specify one of the features: math, chrono, or serde");

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

#[cfg(feature = "chrono")]
pub use crate::print::print_system_time_to_rfc3339;

#[cfg(all(feature = "chrono", feature = "serde"))]
pub use crate::serde::{
  optional_system_time_from_str,
  optional_system_time_to_rfc3339,
  system_time_from_date_str,
  system_time_from_millis,
  system_time_from_secs,
  system_time_from_str,
  system_time_to_millis,
  system_time_to_rfc3339,
};

#[cfg(all(feature = "chrono", feature = "chrono-tz", feature = "serde"))]
pub use crate::serde::{
  system_time_from_millis_in_new_york,
  system_time_to_millis_in_new_york,
};
