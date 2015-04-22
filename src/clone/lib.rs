// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_clone"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_cty as cty;
extern crate linux_syscall as syscall;
extern crate linux_libc as libc;

#[prelude_import] use base::prelude::*;
use base::{error};
use core::ops::{FnOnce};
use cty::alias::{ProcessId};
use syscall::{exit_group};

mod linux { pub use base::linux::*; }

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
