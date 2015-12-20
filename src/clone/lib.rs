// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_clone"]
#![crate_type = "lib"]
#![feature(custom_derive)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_fmt as fmt;
extern crate lrs_r_syscall as r_syscall;
extern crate lrs_libc as libc;

use base::prelude::*;
use cty::alias::{ProcessId};

mod std { pub use fmt::std::*; }

#[cfg(not(no_libc))] #[path = "libc/mod.rs"] mod imp;
#[cfg(no_libc)] #[path = "no_libc/mod.rs"] mod imp;

pub mod flags;

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
    imp::fork(f)
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
    imp::fork_continue()
}
