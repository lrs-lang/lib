// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_clone"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_libc as libc;

#[prelude_import] use base::prelude::*;
use base::{error};
use core::ops::{FnOnce};
use cty::alias::{ProcessId};
use syscall::{exit_group};

mod lrs { pub use base::lrs::*; }

pub fn fork<F>(f: F) -> Result<ProcessId>
    where F: FnOnce()
{
    match unsafe { libc::fork() as ProcessId } {
        -1 => {
            let error = unsafe { *libc::__errno_location() };
            Err(error::Errno(error))
        },
        0 => {
            f();
            exit_group(0);
        },
        n => Ok(n),
    }
}
