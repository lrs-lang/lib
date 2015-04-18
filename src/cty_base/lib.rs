// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_cty_base"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]
#![allow(non_upper_case_globals, non_camel_case_types)]

extern crate linux_core as core;

pub use arch::{errno, types};

mod gen;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
