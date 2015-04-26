// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_fs"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts)]

#[macro_use]
extern crate lrs_core      as core;
extern crate lrs_base      as base;
extern crate lrs_fmt       as fmt;
extern crate lrs_cty       as cty;
extern crate lrs_syscall   as syscall;
extern crate lrs_rv        as rv;
extern crate lrs_str_three as str_three;
extern crate lrs_alloc as alloc;
extern crate lrs_rmo as rmo;

mod lrs {
    pub use fmt::lrs::*;
    pub use {cty};
}

use syscall::{sync};

pub mod info;
pub mod mount;

/// Writes all buffered data and metadata to the disks.
pub fn sync_all() {
    sync()
}
