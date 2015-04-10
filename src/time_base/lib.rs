// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_time_base"]
#![crate_type = "lib"]
#![allow(trivial_numeric_casts)]

#[macro_use]
extern crate linux_core as core;

use std::ops::{Add, Sub};

use core::cty::{timespec, time_t, c_long};
use core::util::{div_rem};

pub mod clock;
pub mod timer;

pub fn time_from_timespec(t: timespec) -> Time {
    Time {
        seconds:      t.tv_sec  as i64,
        nanoseconds: t.tv_nsec as i64,
    }
}

pub fn time_to_timespec(d: Time) -> timespec {
    timespec {
        tv_sec:  d.seconds      as time_t,
        tv_nsec: d.nanoseconds as c_long,
    }
}

const NANOS_PER_SEC: i64 = 1_000_000_000;
const NANOS_PER_MILLI: i64 = 1_000_000;
const NANOS_PER_MICRO: i64 = 1_000;
const MICROS_PER_SEC: i64 = 1_000_000;
const MILLIS_PER_SEC: i64 = 1_000;
const SECS_PER_MIN: i64 = 60;
const SECS_PER_HOUR: i64 = 60 * SECS_PER_MIN;
const SECS_PER_DAY: i64 = 24 * SECS_PER_HOUR;

/// A time.
///
/// This can have various meanings.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time {
    pub seconds: i64,
    pub nanoseconds: i64,
}

impl Time {
    /// Normalizes the time.
    ///
    /// That is, transforms `self` into an equivalent representation where `nanoseconds`
    /// is in [0, 1_000_000_000).
    pub fn normalize(self) -> Time {
        let (mut sec, mut nano) = div_rem(self.nanoseconds, NANOS_PER_SEC);
        if nano < 0 {
            sec -= 1;
            nano += NANOS_PER_SEC;
        }
        Time { seconds: self.seconds.saturating_add(sec), nanoseconds: nano }
    }

    /// Creates a `Time` that represents `n` nanoseconds.
    pub fn nanoseconds(n: i64) -> Time {
        let (s, n) = div_rem(n, NANOS_PER_SEC);
        Time { seconds: s, nanoseconds: n }
    }

    /// Creates a `Time` that represents `m` microseconds.
    pub fn microseconds(m: i64) -> Time {
        let (s, m) = div_rem(m, MICROS_PER_SEC);
        Time { seconds: s, nanoseconds: m * NANOS_PER_MICRO }
    }

    /// Creates a `Time` that represents `m` miliseconds.
    pub fn milliseconds(m: i64) -> Time {
        let (s, m) = div_rem(m, MILLIS_PER_SEC);
        Time { seconds: s, nanoseconds: m * NANOS_PER_MILLI }
    }

    /// Creates a `Time` that represents `s` seconds.
    pub fn seconds(s: i64) -> Time {
        Time { seconds: s, nanoseconds: 0 }
    }

    /// Creates a `Time` that represents `m` minutes.
    pub fn minutes(m: i64) -> Time {
        Time::seconds(m.wrapping_mul(SECS_PER_MIN))
    }

    /// Creates a `Time` that represents `h` hours.
    pub fn hours(h: i64) -> Time {
        Time::seconds(h.wrapping_mul(SECS_PER_HOUR))
    }

    /// Creates a `Time` that represents `d` days.
    pub fn days(d: i64) -> Time {
        Time::seconds(d.wrapping_mul(SECS_PER_DAY))
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        let one = self.normalize();
        let two = other.normalize();
        let time = Time {
            seconds: one.seconds.saturating_add(two.seconds),
            nanoseconds: one.nanoseconds + two.nanoseconds,
        };
        time.normalize()
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        let one = self.normalize();
        let two = other.normalize();
        let time = Time {
            seconds: one.seconds.saturating_sub(two.seconds),
            nanoseconds: one.nanoseconds - two.nanoseconds,
        };
        time.normalize()
    }
}
