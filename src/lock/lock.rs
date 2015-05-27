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

/// The status of a lock.
pub enum LockStatus {
    /// The lock is unlocked.
    Unlocked,
    /// The lock is locked and nobody is waiting for it to be unlocked.
    Locked,
    /// The lock is locked and there are threads waiting for it to be unlocked.
    Waiting,
}

/// A lock.
///
/// = Remarks
///
/// This lock can be used for inter-process synchronization.
#[repr(C)]
pub struct Lock {
    val: AtomicCInt,
}

/// = Remarks
///
/// Two locks are equal if their addresses are equal.
impl Eq for Lock {
    fn eq(&self, other: &Lock) -> bool {
        self as *const Lock as usize == other as *const Lock as usize
    }
}

impl<'a> Lock {
    /// Creates a new, unlocked, lock.
    pub const fn new() -> Lock {
        Lock { val: ATOMIC_CINT_INIT }
    }

    fn guard(&'a self) -> LockGuard<'a> {
        LockGuard { lock: self }
    }

    /// Returns the status of the lock.
    pub fn status(&self) -> LockStatus {
        match self.val.load_unordered() {
            UNLOCKED => LockStatus::Unlocked,
            LOCKED   => LockStatus::Locked,
            _        => LockStatus::Waiting,
        }
    }

    /// Tries to lock the lock if it's currently unlocked.
    ///
    /// [return_value]
    /// Returns a guard if the operation succeeded.
    pub fn try_lock(&'a self) -> Option<LockGuard<'a>> {
        if self.val.compare_exchange(UNLOCKED, LOCKED) == UNLOCKED {
            Some(self.guard())
        } else {
            None
        }
    }

    /// Locks the lock by sleeping until the lock is unlocked if it's currently locked.
    ///
    /// [return_value]
    /// Returns a lock guard.
    pub fn lock(&'a self) -> LockGuard<'a> {
        let mut status = self.val.compare_exchange(UNLOCKED, LOCKED);
        if status == UNLOCKED {
            return self.guard();
        }
        loop {
            if status == WAITING ||
                        self.val.compare_exchange(LOCKED, WAITING) != UNLOCKED {
                unsafe { futex_wait(self.val.unwrap(), WAITING, None); }
            }
            status = self.val.compare_exchange(UNLOCKED, WAITING);
            if status == UNLOCKED {
                return self.guard();
            }
        }
    }
}

/// A lock-guard.
///
/// = Remarks
///
/// This guard automatically unlocks the lock when it goes out of scope.
pub struct LockGuard<'a> {
    lock: &'a Lock,
}

impl<'a> LockGuard<'a> {
    /// Returns the lock guarded by this guard.
    pub fn as_lock(&self) -> &'a Lock {
        self.lock
    }

    /// Unlocks the lock and returns a reference to the lock.
    pub fn unlock(self) -> &'a Lock {
        self.lock
    }
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        if self.lock.val.sub(1) != LOCKED {
            self.lock.val.store(UNLOCKED);
            unsafe { futex_wake(self.lock.val.unwrap(), 1); }
        }
    }
}
