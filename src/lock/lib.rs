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
extern crate linux_arch as arch;

#[prelude_import] use core::prelude::*;
use arch::atomic::{AtomicCInt, ATOMIC_CINT_INIT};
use arch::syscall::{futex_wait, futex_wake};
use arch::cty::{c_int};

pub const INIT: Lock = Lock { val: ATOMIC_CINT_INIT };

const UNLOCKED: c_int = 0;
const LOCKED:   c_int = 1;
const WAITING:  c_int = 2;

pub enum LockStatus {
    Unlocked,
    Locked,
    Waiting,
}

#[repr(C)]
pub struct Lock {
    val: AtomicCInt,
}

impl Lock {
    fn guard(&'static self) -> LockGuard {
        LockGuard { val: &self.val }
    }

    pub fn status(&'static self) -> LockStatus {
        match self.val.load() {
            UNLOCKED => LockStatus::Unlocked,
            LOCKED   => LockStatus::Locked,
            _        => LockStatus::Waiting,
        }
    }

    pub fn try_lock(&'static self) -> Option<LockGuard> {
        if self.val.compare_exchange_seqcst(UNLOCKED, LOCKED) == UNLOCKED {
            Some(self.guard())
        } else {
            None
        }
    }

    pub fn lock(&'static self) -> LockGuard {
        let mut status = self.val.compare_exchange_seqcst(UNLOCKED, LOCKED);
        if status == UNLOCKED {
            return self.guard();
        }
        loop {
            if status == WAITING ||
                        self.val.compare_exchange_seqcst(LOCKED, WAITING) != UNLOCKED {
                unsafe { futex_wait(self.val.unwrap(), WAITING, None); }
            }
            status = self.val.compare_exchange_seqcst(UNLOCKED, WAITING);
            if status == 0 {
                return self.guard();
            }
        }
    }
}

pub struct LockGuard {
    val: &'static AtomicCInt,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        if self.val.sub_seqcst(1) != LOCKED {
            self.val.store(UNLOCKED);
            unsafe { futex_wake(self.val.unwrap(), 1); }
        }
    }
}
