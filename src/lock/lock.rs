// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use core::ops::{Eq};
use base::undef::{UndefState};
use base::{error};
use atomic::{AtomicCInt, ATOMIC_CINT_INIT};
use syscall::{futex_wait, futex_wake};
use cty::{c_int, c_uint};
use time_base::{time_to_timespec, Time, clock};

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
        mem::addr(self) == mem::addr(other)
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

    pub unsafe fn unlock(&self) {
        self.guard();
    }

    pub unsafe fn as_atomic(&self) -> &AtomicCInt {
        &self.val
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
    pub fn try_lock(&'a self) -> Result<LockGuard<'a>> {
        if self.val.compare_exchange(UNLOCKED, LOCKED) == UNLOCKED {
            Ok(self.guard())
        } else {
            Err(error::ResourceBusy)
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
                futex_wait(&self.val, WAITING, None);
            }
            status = self.val.compare_exchange(UNLOCKED, WAITING);
            if status == UNLOCKED {
                return self.guard();
            }
        }
    }

    /// Locks the lock by sleeping until the lock is unlocked if it's currently locked or
    /// until a certain amount of time has expired.
    ///
    /// [argument, time]
    /// An upper bound for the amount of time until this function returns.
    ///
    /// [return_value]
    /// Returns a lock guard or an error.
    ///
    /// = Remarks
    ///
    /// The function may take longer to return than allowed by the `time` parameter.
    pub fn try_lock_for(&'a self, mut time: Time) -> Result<LockGuard<'a>> {
        let mut status = self.val.compare_exchange(UNLOCKED, LOCKED);
        if status == UNLOCKED {
            return Ok(self.guard());
        }

        let now = try!(clock::MONO_RAW.get_time());
        let then = now + time;

        loop {
            if status == WAITING ||
                        self.val.compare_exchange(LOCKED, WAITING) != UNLOCKED {
                let spec = time_to_timespec(time);
                match rv!(futex_wait(&self.val, WAITING, Some(&spec))) {
                    Err(error::TimedOut) => break,
                    _ => { },
                }
            }
            status = self.val.compare_exchange(UNLOCKED, WAITING);
            if status == UNLOCKED {
                return Ok(self.guard());
            }

            let now = try!(clock::MONO_RAW.get_time());
            if now < then {
                time = then - now;
            } else {
                break;
            }
        }

        Err(error::TimedOut)
    }
}

unsafe impl UndefState for Lock {
    fn num() -> usize { c_uint::max() as usize - 2 }

    unsafe fn set_undef(val: *mut Lock, n: usize) {
        assert!(n < Self::num());
        assert!(mem::size_of::<Lock>() == mem::size_of::<c_uint>());
        *(val as *mut c_uint) = n as c_uint + 3;
    }

    unsafe fn is_undef(val: *const Lock, n: usize) -> bool {
        assert!(mem::size_of::<Lock>() == mem::size_of::<c_uint>());
        *(val as *const c_uint) == n as c_uint + 3
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
            futex_wake(&self.lock.val, 1);
        }
    }
}
