// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_time_base"]
#![crate_type = "lib"]
#![allow(trivial_numeric_casts)]

extern crate linux_core as core;

use core::cty::{timespec, time_t, c_long};

pub mod clock;

pub fn time_from_timespec(t: timespec) -> Time {
    Time {
        seconds:     t.tv_sec  as i64,
        nano_seconds: t.tv_nsec as i64,
    }
}

pub fn time_to_timespec(d: Time) -> timespec {
    timespec {
        tv_sec:  d.seconds     as time_t,
        tv_nsec: d.nano_seconds as c_long,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time {
    pub seconds: i64,
    pub nano_seconds: i64,
}
