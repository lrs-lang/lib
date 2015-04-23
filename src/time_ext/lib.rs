// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_time_ext"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core      as core;
extern crate linux_base      as base;
extern crate linux_fmt       as fmt;
extern crate linux_str_one   as str_one;
extern crate linux_file      as file;
extern crate linux_time_base as time_base;
extern crate linux_io        as io;
extern crate linux_vec       as vec;

#[prelude_import] use base::prelude::*;
mod linux { pub use vec::linux::*; }

pub use time_base::{Time};

use base::rmo::{AsRef};
use fmt::{Debug, Write};
use vec::{SVec, Vec};
use str_one::{AsNoNullStr};
use file::{File};

mod parse;
mod convert;

/// A weekday.
#[derive(Copy, Eq)]
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
#[derive(Copy, Eq)]
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

impl Debug for DateTime {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let offset_minutes = self.offset / 60;
        let offset_hours = offset_minutes / 60;
        let offset_minutes = offset_minutes % 60;
        write!(w, "{}-{:02}-{:02}T{:02}:{:02}:{:02}+{:02}:{:02}", self.year, self.month,
               self.day, self.hour, self.minute, self.second, offset_hours,
               offset_minutes)
    }
}

/// A time zone.
#[derive(Clone, Eq)]
pub struct Zone {
    /// (UTC offset, summer time) (at least one exists in every zone)
    states: SVec<(i64, bool)>,
    /// (transition time, index into states)
    transitions: SVec<(i64, usize)>,
    /// (leap second time, number of leap seconds)
    leap_seconds: SVec<(i64, i64)>,
}

impl Zone {
    fn load_from(zone: &[u8]) -> Result<Zone> {
        let mut file = try!(File::open_read(&zone));
        let mut data: Vec<u8> = Vec::new();
        try!(data.read_to_eof(&mut file));
        let mut input = &data[..];
        parse::parse(&mut input)
    }

    /// Loads a time zone from a well known name.
    ///
    /// For example: "Europe/Berlin", "Asia/Tokyo". The full list of name can be found on
    /// wikipedia.
    pub fn load<S>(zone: S) -> Result<Zone>
        where S: AsNoNullStr,
    {
        const PREFIX: &'static [u8] = b"/usr/share/zoneinfo/";
        let path = try!(zone.as_no_null_str());
        let mut vec: Vec<u8> = try!(Vec::with_capacity(PREFIX.len() + path.len() + 1));
        vec.push_all(PREFIX);
        vec.push_all(path.as_ref());
        vec.push(0);
        Zone::load_from(&vec)
    }

    /// Loads the UTC time zone.
    pub fn utc() -> Result<Zone> {
        Zone::load_from(b"/usr/share/zoneinfo/UTC\0")
    }

    /// Loads the local time zone.
    pub fn local() -> Result<Zone> {
        Zone::load_from(b"/etc/localtime\0")
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
