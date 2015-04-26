// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_process"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts)]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_syscall as syscall;
extern crate lrs_cty as cty;
extern crate lrs_fmt as fmt;
extern crate lrs_str_one as str_one;
extern crate lrs_str_two as str_two;
extern crate lrs_str_three as str_three;
extern crate lrs_alloc as alloc;
extern crate lrs_c_ptr_ptr as c_ptr_ptr;
extern crate lrs_rt as rt;
extern crate lrs_file as file;
extern crate lrs_rmo as rmo;
extern crate lrs_rv as rv;
extern crate lrs_env as env;

mod lrs {
    pub use fmt::lrs::*;
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
