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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DateTime {
    pub offset:      i64,
    pub year:        i64,
    pub day_in_year: i16,
    pub month:       i8,
    pub day:         i8,
    pub hour:        i8,
    pub minute:      i8,
    pub second:      i8,
    pub weekday:     Weekday,
    pub summer_time: bool,
}

impl fmt::Debug for DateTime {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let offset_minutes = self.offset / 60;
        let offset_hours = offset_minutes / 60;
        let offset_minutes = offset_minutes % 60;
        write!(fmt, "{}-{}-{}T{}:{}:{}+{}:{}", self.year, self.month, self.day,
               self.hour, self.minute, self.second, offset_hours, offset_minutes)
    }
}

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

    pub fn load<S: AsLinuxStr>(zone: S) -> Result<Zone> {
        let mut base = b"/usr/share/zoneinfo/".to_vec();
        base.push_all(zone.as_linux_str().as_slice());
        Zone::load_from(&base)
    }

    pub fn utc() -> Result<Zone> {
        Zone::load_from(b"/usr/share/zoneinfo/UTC")
    }

    pub fn local() -> Result<Zone> {
        Zone::load_from(b"/etc/localtime")
    }

    pub fn explode(&self, time: Time) -> DateTime {
        convert::explode(self, time.seconds)
    }

    pub fn compact(&self, date: DateTime) -> (DateTime, Time) {
        convert::compact(self, date)
    }
}
