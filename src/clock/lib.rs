// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_clock"]
#![crate_type = "lib"]
#![allow(trivial_numeric_casts)]

extern crate linux_core as core;

use core::cty::{timespec, time_t, c_long};

pub fn duration_from_timespec(t: timespec) -> Duration {
    Duration {
        seconds:     t.tv_sec  as i64,
        nanoseconds: t.tv_nsec as i64,
    }
}

pub fn duration_to_timespec(d: Duration) -> timespec {
    timespec {
        tv_sec:  d.seconds     as time_t,
        tv_nsec: d.nanoseconds as c_long,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration {
    pub seconds: i64,
    pub nanoseconds: i64,
}
