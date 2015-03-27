// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_clock"]
#![crate_type = "lib"]
#![allow(trivial_numeric_casts)]

extern crate linux_core as core;

use core::cty::{timespec};

pub fn duration_from_timespec(t: timespec) -> Duration {
    Duration {
        seconds: t.tv_sec as i64,
        nanoseconds: t.tv_nsec as i64,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration {
    pub seconds: i64,
    pub nanoseconds: i64,
}
