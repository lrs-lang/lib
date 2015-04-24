// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_fs"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts)]

#[macro_use]
extern crate linux_core      as core;
extern crate linux_base      as base;
extern crate linux_fmt       as fmt;
extern crate linux_cty       as cty;
extern crate linux_syscall   as syscall;
extern crate linux_rv        as rv;
extern crate linux_str_three as str_three;
extern crate linux_alloc as alloc;
extern crate linux_rmo as rmo;

mod linux {
    pub use fmt::linux::*;
    pub use {cty};
}

use syscall::{sync};

pub mod info;
pub mod mount;

/// Writes all buffered data and metadata to the disks.
pub fn sync_all() {
    sync()
}
