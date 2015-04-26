// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_process"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned, custom_derive)]
#![plugin(linux_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts)]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_syscall as syscall;
extern crate linux_cty as cty;
extern crate linux_fmt as fmt;
extern crate linux_str_one as str_one;
extern crate linux_str_two as str_two;
extern crate linux_str_three as str_three;
extern crate linux_alloc as alloc;
extern crate linux_c_ptr_ptr as c_ptr_ptr;
extern crate linux_rt as rt;
extern crate linux_file as file;
extern crate linux_rmo as rmo;
extern crate linux_rv as rv;
extern crate linux_env as env;

mod linux {
    pub use fmt::linux::*;
    pub use {cty};
}

#[allow(unused_imports)] #[prelude_import] use base::prelude::*;
use syscall::{getpid, getppid, exit_group};
use cty::alias::{ProcessId};
use cty::{c_int};

pub mod ids;
pub mod exec;
pub mod wait;

/// Returns the process id of this process.
pub fn this_process_id() -> ProcessId {
    getpid()
}

/// Returns the process id of the process that created this process.
pub fn parent_process_id() -> ProcessId {
    getppid()
}

/// Exits the process.
pub fn exit(code: c_int) -> ! {
    exit_group(code);
}
