// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_process"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned)]
#![plugin(linux_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts)]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_syscall as syscall;
extern crate linux_cty as cty;
extern crate linux_fmt as fmt;

mod linux {
    pub use fmt::linux::*;
    pub use {cty};
}

use syscall::{getpid, getppid, exit_group};
use cty::alias::{ProcessId};

pub mod ids;

/// Returns the process id of this process.
pub fn this_process_id() -> ProcessId {
    getpid()
}

/// Returns the process id of the process that created this process.
pub fn parent_process_id() -> ProcessId {
    getppid()
}

/// Exits the process.
pub fn exit(code: i32) -> ! {
    exit_group(code);
}
