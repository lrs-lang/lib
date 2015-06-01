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
extern crate lrs_time_base as time_base;
extern crate lrs_env as env;

mod lrs {
    pub use fmt::lrs::*;
    pub use {cty};
}

#[allow(unused_imports)] #[prelude_import] use base::prelude::*;
use core::{mem};
use syscall::{getpid, getppid, exit_group, umask, times};
use cty::alias::{ProcessId};
use cty::{c_int, tms};
use file::flags::{Mode};
use time_base::{Time};

pub mod exec;
pub mod wait;

/// Returns the process id of this process.
pub fn process_id() -> ProcessId {
    getpid()
}

/// Returns the process id of the parent process.
pub fn parent_process_id() -> ProcessId {
    getppid()
}

/// Exits the process.
///
/// [argument, code]
/// The exit code that will be shown to the parent process.
///
/// = See also
///
/// * link:man:exit_group(2)
pub fn exit(code: u8) -> ! {
    exit_group(code as c_int);
}

/// Sets the file mode creation mask of the process.
///
/// [argument, mode]
/// The mode to be masked.
///
/// [return_value]
/// Returns the previous mask.
///
/// = Remarks
///
/// The mask will be *subtracted* from the mask used in `open` etc. That is, if a bit is
/// set in the mask, it will be unset in calls to `open` etc.
///
/// = See also
///
/// * link:man:umask(2)
pub fn set_file_mask(mode: Mode) -> Mode {
    Mode(umask(mode.0))
}

/// The times used by a process and its children.
pub struct Times {
    data: tms,
}

impl Times {
    /// Returns the user-space time used by this process.
    pub fn user_time(&self) -> Time {
        Time::milliseconds(self.data.tms_utime as i64 * 10)
    }

    /// Returns the kernel-space time used by this process.
    pub fn kernel_time(&self) -> Time {
        Time::milliseconds(self.data.tms_stime as i64 * 10)
    }

    /// Returns the user-space time used by the children of this process.
    ///
    /// = Remarks
    ///
    /// Only the time used by children that have been reaped are counted.
    pub fn children_user_time(&self) -> Time {
        Time::milliseconds(self.data.tms_cutime as i64 * 10)
    }

    /// Returns the kernel-space time used by the children of this process.
    ///
    /// = Remarks
    ///
    /// Only the time used by children that have been reaped are counted.
    pub fn children_kernel_time(&self) -> Time {
        Time::milliseconds(self.data.tms_cstime as i64 * 10)
    }
}

/// Returns the CPU times used by this process and its children.
///
/// = See also
///
/// * link:man:times(2)
pub fn used_time() -> Result<Times> {
    let mut data = mem::zeroed();
    try!(rv!(times(&mut data)));
    Ok(Times { data: data })
}
