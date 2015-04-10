// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_sys"]
#![crate_type = "lib"]
#![feature(core)]

#[macro_use]
extern crate linux_core as core;

use core::cty::{CPU_SET_SIZE};
use core::syscall::{sched_getaffinity};

use std::iter::{AdditiveIterator};

/// Returns the number of CPU available to this thread.
pub fn cpu_count() -> usize {
    let mut buf = [0; CPU_SET_SIZE];
    sched_getaffinity(0, &mut buf);
    buf.iter().map(|b| b.count_ones()).sum() as usize
}
