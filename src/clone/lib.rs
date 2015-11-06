// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_clone"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_fmt as fmt;
extern crate lrs_libc as libc;

use base::prelude::*;
use base::{error};
use cty::alias::{ProcessId};
use syscall::{exit_group};

mod std { pub use fmt::std::*; }

pub mod flags;

// TODO: Libc-free version. There are some interesting posibilities with clone(2) that are
// not available via libc but we can only use clone(2) directly if we don't use libc.

/// Forks the process and executes a function in the child process.
///
/// [argument, f]
/// The function that will be executed in the child process.
///
/// [return_value]
/// Returns the process id of the child process.
///
/// = Remarks
///
/// The function `f` will only be executed in the child process. When the function
/// returns, the child process automatically exits with exit code `0`.
///
/// = Examples
///
/// ----
/// let res = fork(|| println!("I'm in the child process"));
/// match res {
///     Some(n) => println!("Executed the child process: {}", n),
///     Err(e) => println!("Could not execute the child process: {:?}", e),
/// }
/// ----
///
/// = See also
///
/// * link:man:fork(2)
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

/// Forks the process.
///
/// [return_value]
/// Returns the process id of the child process or `None` if we're in the child process.
///
/// = See also
///
/// * link:man:fork(2)
pub fn fork_continue() -> Result<Option<ProcessId>> {
    match unsafe { libc::fork() as ProcessId } {
        -1 => {
            let error = unsafe { *libc::__errno_location() };
            Err(error::Errno(error))
        },
        0 => Ok(None),
        n => Ok(Some(n)),
    }
}
