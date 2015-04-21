// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{Eq};
use atomic::{AtomicCInt, ATOMIC_CINT_INIT};
use syscall::{futex_wait, futex_wake};
use cty::{c_int};

pub const LOCK_INIT: Lock = Lock { val: ATOMIC_CINT_INIT };

pub static DUMMY: Lock = LOCK_INIT;

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

impl Eq for Lock {
    fn eq(&self, other: &Lock) -> bool {
        self as *const Lock as usize == other as *const Lock as usize
    }
}

impl<'a> Lock {
    fn guard(&'a self) -> LockGuard<'a> {
        LockGuard { lock: self }
    }

    pub fn status(&self) -> LockStatus {
        match self.val.load() {
            UNLOCKED => LockStatus::Unlocked,
            LOCKED   => LockStatus::Locked,
            _        => LockStatus::Waiting,
        }
    }

    pub fn try_lock(&'a self) -> Option<LockGuard<'a>> {
        if self.val.compare_exchange_seqcst(UNLOCKED, LOCKED) == UNLOCKED {
            Some(self.guard())
        } else {
            None
        }
    }

    pub fn lock(&'a self) -> LockGuard<'a> {
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
            if status == UNLOCKED {
                return self.guard();
            }
        }
    }
}

pub struct LockGuard<'a> {
    lock: &'a Lock,
}

impl<'a> LockGuard<'a> {
    pub fn as_lock(&self) -> &'a Lock {
        self.lock
    }

    pub fn unlock(self) -> &'a Lock {
        self.lock
    }
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        if self.lock.val.sub_seqcst(1) != LOCKED {
            self.lock.val.store_seqcst(UNLOCKED);
            unsafe { futex_wake(self.lock.val.unwrap(), 1); }
        }
    }
}
