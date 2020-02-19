// Copyright (C) 2019-2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A crate for more or less frequently used time parsing and formatting
//! functionality.
//!
//! The crate revolves around the `std::time::SystemTime` type in that
//! we attempt to convert into that or use this as the base to convert
//! from. We treat such a time as having no associated time zone. Think
//! of it as being in UTC.

mod math;
mod parse;
mod serde;
mod timezone;

pub use crate::math::days_back;
pub use crate::math::days_back_from;
pub use crate::math::next_day;
pub use crate::math::tomorrow;

pub use crate::parse::parse_system_time_from_date_str;
pub use crate::parse::parse_system_time_from_str;

pub use crate::serde::optional_system_time_from_str;
pub use crate::serde::optional_system_time_to_rfc3339;
pub use crate::serde::system_time_from_date_str;
pub use crate::serde::system_time_from_millis;
pub use crate::serde::system_time_from_millis_in_tz;
pub use crate::serde::system_time_from_secs;
pub use crate::serde::system_time_from_str;
pub use crate::serde::system_time_to_millis;
pub use crate::serde::system_time_to_rfc3339;

/// The Eastern Standard Time time zone.
pub use crate::timezone::EST;
/// The Coordinated Universal Time time zone.
pub use crate::timezone::UTC;
