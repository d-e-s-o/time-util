Unreleased
----------
- Added support for printing and serializing time as RFC 3339 time stamp
  with nanosecond resolution


0.3.2
-----
- Added missing use declarations when feature `chrono-tz` is enabled
- Enabled CI pipeline comprising building, testing, and linting of the
  project


0.3.1
-----
- Added support for printing a time as RFC 3339 time stamp


0.3.0
-----
- Use `chrono-tz` for time-zone aware conversions
  - Removed custom time-zone logic


0.2.0
-----
- Added support for parsing a time stamp from a date string
- Added support for serializing a time stamp for a given time zone
- Added support for rudimentary time calculations
- Split crate into three modules: `math`, `parse`, and `serde`
  - Each has to be opted-in with the corresponding feature flag


0.1.0
-----
- Initial release
