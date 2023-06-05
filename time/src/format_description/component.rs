//! Part of a format description.

use crate::format_description::modifier;

/// A component of a larger format description.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    /// Day of the month.
    Day(modifier::Day),
    /// Month of the year.
    Month(modifier::Month),
    /// Ordinal day of the year.
    Ordinal(modifier::Ordinal),
    /// Day of the week.
    Weekday(modifier::Weekday),
    /// Week within the year.
    WeekNumber(modifier::WeekNumber),
    /// Year of the date.
    Year(modifier::Year),
    /// Hour of the day.
    Hour(modifier::Hour),
    /// Minute within the hour.
    Minute(modifier::Minute),
    /// AM/PM part of the time.
    Period(modifier::Period),
    /// Second within the minute.
    Second(modifier::Second),
    /// Subsecond within the second.
    Subsecond(modifier::Subsecond),
    /// Hour of the UTC offset.
    OffsetHour(modifier::OffsetHour),
    /// Minute within the hour of the UTC offset.
    OffsetMinute(modifier::OffsetMinute),
    /// Second within the minute of the UTC offset.
    OffsetSecond(modifier::OffsetSecond),
    /// A number of bytes to ignore when parsing. This has no effect on formatting.
    Ignore(modifier::Ignore),
    /// A Unix timestamp.
    UnixTimestamp(modifier::UnixTimestamp),
}

impl Component {
    /// Determines whether this component can be ignored when formatting
    pub(crate) fn fmt_ignore(
        &self,
        _date: Option<crate::Date>,
        time: Option<crate::Time>,
        offset: Option<crate::UtcOffset>,
    ) -> bool {
        match self {
            Self::Day(_)
            | Self::Month(_)
            | Self::Ordinal(_)
            | Self::Weekday(_)
            | Self::WeekNumber(_)
            | Self::Year(_)
            | Self::Hour(_)
            | Self::Minute(_)
            | Self::Period(_)
            | Self::Ignore(_)
            | Self::UnixTimestamp(_) => false,
            Self::Second(_) => time.map(|t| t.second() == 0).unwrap_or(true),
            Self::Subsecond(s) => time
                .map(|t| s.digits.as_format_repr(t.nanosecond()).1 == 0)
                .unwrap_or(true),
            Self::OffsetHour(_) => offset.map(|t| t.is_utc()).unwrap_or(true),
            Self::OffsetMinute(_) => offset.map(|t| t.minutes_past_hour() == 0).unwrap_or(true),
            Self::OffsetSecond(_) => offset.map(|t| t.seconds_past_minute() == 0).unwrap_or(true),
        }
    }
}
