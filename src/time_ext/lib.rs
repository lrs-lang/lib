// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_time_ext"]
#![crate_type = "lib"]

#![feature(collections)]

extern crate linux_file as file;
extern crate linux_time_base as time_base;
extern crate linux_core as core;

pub use time_base::{Time};

use std::io::{Read};
use std::{fmt};

use file::{File};

use core::string::{AsLinuxStr};
use core::result::{Result};

mod parse;
mod convert;

/// A weekday.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednsday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// An expanded date.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DateTime {
    /// The offset from UTC in seconds.
    pub offset:      i64,
    pub year:        i64,
    /// The day in the year starting at 0.
    pub day_in_year: i16,
    /// The month in the year starting at 1.
    pub month:       i8,
    /// The day in the month starting at 1.
    pub day:         i8,
    pub hour:        i8,
    pub minute:      i8,
    pub second:      i8,
    pub weekday:     Weekday,
    /// True if the date falls into summer time.
    pub summer_time: bool,
}

impl fmt::Debug for DateTime {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let offset_minutes = self.offset / 60;
        let offset_hours = offset_minutes / 60;
        let offset_minutes = offset_minutes % 60;
        write!(fmt, "{}-{:02}-{:02}T{:02}:{:02}:{:02}+{:02}:{:02}", self.year, self.month,
               self.day, self.hour, self.minute, self.second, offset_hours,
               offset_minutes)
    }
}

/// A time zone.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Zone {
    /// (UTC offset, summer time) (at least one exists in every zone)
    states: Vec<(i64, bool)>,
    /// (transition time, index into states)
    transitions: Vec<(i64, usize)>,
    /// (leap second time, number of leap seconds)
    leap_seconds: Vec<(i64, i64)>,
}

impl Zone {
    fn load_from(zone: &[u8]) -> Result<Zone> {
        let mut file = try!(File::open_read(&zone));
        let mut data = vec!();
        try!(file.read_to_end(&mut data));
        let mut input = &data[..];
        parse::parse(&mut input)
    }

    /// Loads a time zone from a well known name.
    ///
    /// For example: "Europe/Berlin", "Asia/Tokyo". The full list of name can be found on
    /// wikipedia.
    pub fn load<S: AsLinuxStr>(zone: S) -> Result<Zone> {
        let mut base = b"/usr/share/zoneinfo/".to_vec();
        base.push_all(zone.as_linux_str().as_slice());
        Zone::load_from(&base)
    }

    /// Loads the UTC time zone.
    pub fn utc() -> Result<Zone> {
        Zone::load_from(b"/usr/share/zoneinfo/UTC")
    }

    /// Loads the local time zone.
    pub fn local() -> Result<Zone> {
        Zone::load_from(b"/etc/localtime")
    }

    /// Expands a time since the epoch to a `DateTime` in the given time zone.
    pub fn expand(&self, time: Time) -> DateTime {
        convert::explode(self, time.seconds)
    }

    /// Returns a normalized version of `date` and a time such that expands to the
    /// normalized `date`.
    ///
    /// This function looks at the following fields: year, month, day, hour, minute,
    /// second. All other fields will be calculated from these fields. Note that, if the
    /// date cannot be represented as a time in the given timezone (e.g. because the year
    /// is too large or the time falls into the one hour hole between winter and summer
    /// time) then the return value will not agree with the given date in the fields
    /// mentioned above but the property that the returned time expands to the returned
    /// `DateTime` still holds.
    ///
    /// The fields mentioned above are normalized according to the following algorithm:
    ///
    /// 1. second is reduced so that it is in the [0, 60) range and minute is adjusted
    ///    accordingly (this means that times that coincide with leap seconds will not be
    ///    normalized correctly.)
    /// 2. minute is reduced so that it is in the [0, 60) range and hour is adjusted
    ///    accordingly.
    /// 3. hour is reduced so that it is in the [0, 24) range and day is adjusted
    ///    accordingly.
    /// 4. month is reduced so that it is in the [0, 12) range and year is adjusted
    ///    accordingly.
    /// 5. day, month, and year are adjusted until day is a valid day in the month.
    pub fn compact(&self, date: DateTime) -> (DateTime, Time) {
        convert::compact(self, date)
    }
}
