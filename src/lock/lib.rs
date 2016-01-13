// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_lock"]
#![crate_type = "lib"]
#![feature(optin_builtin_traits, const_fn, associated_consts)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_io as io;
extern crate lrs_fmt as fmt;
extern crate lrs_cell as cell;
extern crate lrs_atomic as atomic;
extern crate lrs_time_base as time_base;
extern crate lrs_cty as cty;

#[cfg(not(freestanding))]extern crate lrs_syscall as syscall;

#[cfg(not(freestanding))] pub use raw_condvar::{RawCondvar};
#[cfg(not(freestanding))] pub use condvar::{Condvar};
#[cfg(not(freestanding))] pub use lock::{Lock, LockGuard, DUMMY, LockStatus};
#[cfg(not(freestanding))] pub use mutex::{Mutex, MutexGuard};
#[cfg(not(freestanding))] pub use once::{Once, OnceStatus};
pub use stlock::{SingleThreadLock, SingleThreadLockGuard};
pub use stmutex::{SingleThreadMutex, SingleThreadMutexGuard};

mod std { pub use fmt::std::*; pub use cty; }

#[cfg(not(freestanding))] mod raw_condvar;
#[cfg(not(freestanding))] mod condvar;
#[cfg(not(freestanding))] mod lock;
#[cfg(not(freestanding))] mod mutex;
#[cfg(not(freestanding))] mod once;
mod stlock;
mod stmutex;
