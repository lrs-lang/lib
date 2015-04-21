// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_lock"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_io as io;
extern crate linux_fmt as fmt;
extern crate linux_cell as cell;
extern crate linux_atomic as atomic;
extern crate linux_cty as cty;
extern crate linux_syscall as syscall;

pub use raw_condvar::{RawCondvar, RAW_CONDVAR_INIT};
pub use lock::{Lock, LockGuard, LOCK_INIT, DUMMY};
pub use mutex::{Mutex, MutexGuard};
pub use condvar::{Condvar, CONDVAR_INIT};

mod linux {
    pub use fmt::linux::*;
}

mod raw_condvar;
mod condvar;
mod lock;
mod mutex;
