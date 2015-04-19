// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_time_base"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts)]

#[macro_use]
extern crate linux_base as base;
#[prelude_import] use base::prelude::*;
mod linux { pub use base::linux::*; }
mod core { pub use base::core::*; }

use base::ops::{Add, Sub};
use base::cty::{timespec, time_t, k_long};
use base::fmt::{Debug, Write};

pub mod clock;
pub mod timer;

pub fn time_from_timespec(t: timespec) -> Time {
    Time {
        seconds:     t.tv_sec  as i64,
        nanoseconds: t.tv_nsec as i64,
    }
}

pub fn time_to_timespec(d: Time) -> timespec {
    timespec {
        tv_sec:  d.seconds     as time_t,
        tv_nsec: d.nanoseconds as k_long,
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
#[derive(Copy, Eq)]
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
        let (mut sec, mut nano) = self.nanoseconds.div_rem(NANOS_PER_SEC);
        if nano < 0 {
            sec -= 1;
            nano += NANOS_PER_SEC;
        }
        Time { seconds: self.seconds.saturating_add(sec), nanoseconds: nano }
    }

    /// Creates a `Time` that represents `n` nanoseconds.
    pub fn nanoseconds(n: i64) -> Time {
        let (s, n) = n.div_rem(NANOS_PER_SEC);
        Time { seconds: s, nanoseconds: n }
    }

    /// Creates a `Time` that represents `m` microseconds.
    pub fn microseconds(m: i64) -> Time {
        let (s, m) = m.div_rem(MICROS_PER_SEC);
        Time { seconds: s, nanoseconds: m * NANOS_PER_MICRO }
    }

    /// Creates a `Time` that represents `m` miliseconds.
    pub fn milliseconds(m: i64) -> Time {
        let (s, m) = m.div_rem(MILLIS_PER_SEC);
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

impl Debug for Time {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let time = self.normalize();
        write!(w, "\"{}s {}ns\"", time.seconds, time.nanoseconds)
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
