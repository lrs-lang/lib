// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_lock"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_io as io;
extern crate lrs_fmt as fmt;
extern crate lrs_cell as cell;
extern crate lrs_atomic as atomic;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;

pub use raw_condvar::{RawCondvar, RAW_CONDVAR_INIT};
pub use lock::{Lock, LockGuard, LOCK_INIT, DUMMY};
pub use mutex::{Mutex, MutexGuard};
pub use condvar::{Condvar, CONDVAR_INIT};

mod lrs {
    pub use fmt::lrs::*;
}

mod raw_condvar;
mod condvar;
mod lock;
mod mutex;
