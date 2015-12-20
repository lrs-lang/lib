// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_time_ext"]
#![crate_type = "lib"]
#![feature(negate_unsigned, custom_derive)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_fmt as fmt;
extern crate lrs_str_one as str_one;
extern crate lrs_time_base as time_base;
extern crate lrs_io as io;
extern crate lrs_vec as vec;
#[cfg(not(freestanding))] extern crate lrs_file as file;

use base::prelude::*;
mod std { pub use vec::std::*; }

pub use time_base::{Time};

use fmt::{Debug, Write};
use vec::{Vec};

#[cfg(not(freestanding))] use str_one::{NoNullStr};
#[cfg(not(freestanding))] use io::{BufWrite};
#[cfg(not(freestanding))] use file::{File};

mod parse;
mod convert;

/// A weekday.
#[derive(Copy, Eq)]
pub enum Weekday {
    /// Monday
    Monday,
    /// Tuesday
    Tuesday,
    /// Wednesday
    Wednesday,
    /// Thursday
    Thursday,
    /// Friday
    Friday,
    /// Saturday
    Saturday,
    /// Sunday
    Sunday,
}

/// An expanded date.
#[derive(Copy, Eq)]
pub struct DateTime {
    /// The offset from UTC in seconds.
    pub offset:      i64,
    /// The year.
    pub year:        i64,
    /// The day in the year starting at 0.
    pub day_in_year: i16,
    /// The month in the year starting at 1.
    pub month:       i8,
    /// The day in the month starting at 1.
    pub day:         i8,
    /// The hour.
    pub hour:        i8,
    /// The minute.
    pub minute:      i8,
    /// The second.
    pub second:      i8,
    /// The weekday.
    pub weekday:     Weekday,
    /// Whether the date falls into summer time.
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
#[derive(TryTo, Eq)]
pub struct Zone {
    /// (UTC offset, summer time) (at least one exists in every zone)
    states: Vec<(i64, bool)>,
    /// (transition time, index into states)
    transitions: Vec<(i64, usize)>,
    /// (leap second time, number of leap seconds)
    leap_seconds: Vec<(i64, i64)>,
}

impl Zone {
    #[cfg(not(freestanding))]
    fn load_from(zone: &[u8]) -> Result<Zone> {
        let mut file = try!(File::open_read(&zone));
        let mut data: Vec<u8> = Vec::new();
        try!(data.read_to_eof(&mut file));
        Zone::load_bytes(&data)
    }

    /// Parses a time zone from memory.
    ///
    /// [argument, zone]
    /// The zone to parse.
    pub fn load_bytes(mut zone: &[u8]) -> Result<Zone> {
        parse::parse(&mut zone)
    }

    /// Loads a time zone from a well known name.
    ///
    /// [argument, zone]
    /// The name of the zone.
    ///
    /// = Remarks
    ///
    /// For example: "Europe/Berlin", "Asia/Tokyo". The full list of name can be found on
    /// wikipedia.
    #[cfg(not(freestanding))]
    pub fn load<S: ?Sized>(zone: &S) -> Result<Zone>
        where S: TryAsRef<NoNullStr>,
    {
        const PREFIX: &'static [u8] = b"/usr/share/zoneinfo/";
        let path = try!(zone.try_as_ref());
        let mut vec: Vec<u8> = try!(Vec::with_capacity(PREFIX.len() + path.len() + 1));
        vec.push_all(PREFIX);
        vec.push_all(path.as_ref());
        vec.push(0);
        Zone::load_from(&vec)
    }

    /// Loads the UTC time zone.
    #[cfg(not(freestanding))]
    pub fn utc() -> Result<Zone> {
        Zone::load_from(b"/usr/share/zoneinfo/UTC\0")
    }

    /// Loads the local time zone.
    #[cfg(not(freestanding))]
    pub fn local() -> Result<Zone> {
        Zone::load_from(b"/etc/localtime\0")
    }

    /// Expands a time since the epoch to a `DateTime` in the given time zone.
    ///
    /// [argument, time]
    /// The time to expand.
    pub fn expand(&self, time: Time) -> DateTime {
        convert::explode(self, time.seconds)
    }

    /// Returns a normalized version of a `DateTime` and a time that expands to the
    /// normalized date.
    ///
    /// [argument, date]
    /// The date to compact.
    ///
    /// = Remarks
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
    /// * second is reduced so that it is in the [0, 60) range and minute is adjusted
    ///   accordingly (this means that times that coincide with leap seconds will not be
    ///   normalized correctly.)
    /// * minute is reduced so that it is in the [0, 60) range and hour is adjusted
    ///   accordingly.
    /// * hour is reduced so that it is in the [0, 24) range and day is adjusted
    ///   accordingly.
    /// * month is reduced so that it is in the [0, 12) range and year is adjusted
    ///   accordingly.
    /// * day, month, and year are adjusted until day is a valid day in the month.
    pub fn compact(&self, date: DateTime) -> (DateTime, Time) {
        convert::compact(self, date)
    }
}
