// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use core::ops::{Eq};
use base::undef::{UndefState};
use base::{error};
use atomic::{Atomic};
use {arch_fns};

const UNLOCKED: u8 = 0;
const LOCKED:   u8 = 1;

/// The status of a lock.
pub enum SpinLockStatus {
    /// The lock is unlocked.
    Unlocked,
    /// The lock is locked.
    Locked,
}

/// A lock.
///
/// = Remarks
///
/// This lock can be used for inter-process synchronization.
#[repr(C)]
pub struct SpinLock {
    val: Atomic<u8>,
}

/// = Remarks
///
/// Two locks are equal if their addresses are equal.
impl Eq for SpinLock {
    fn eq(&self, other: &SpinLock) -> bool {
        mem::addr(self) == mem::addr(other)
    }
}

impl<'a> SpinLock {
    /// Creates a new, unlocked, lock.
    pub const fn new() -> SpinLock {
        SpinLock { val: Atomic::new(UNLOCKED) }
    }

    fn guard(&'a self) -> SpinLockGuard<'a> {
        SpinLockGuard { lock: self }
    }

    pub unsafe fn unlock(&self) {
        self.guard();
    }

    pub unsafe fn as_atomic(&self) -> &Atomic<u8> {
        &self.val
    }

    /// Returns the status of the lock.
    pub fn status(&self) -> SpinLockStatus {
        match self.val.load_unordered() {
            UNLOCKED => SpinLockStatus::Unlocked,
            _        => SpinLockStatus::Locked,
        }
    }

    /// Tries to lock the lock if it's currently unlocked.
    ///
    /// [return_value]
    /// Returns a guard if the operation succeeded.
    pub fn try_lock(&'a self) -> Result<SpinLockGuard<'a>> {
        if self.val.compare_exchange_acquire(UNLOCKED, LOCKED) == UNLOCKED {
            Ok(self.guard())
        } else {
            Err(error::ResourceBusy)
        }
    }

    /// Locks the lock by spinning until the lock is unlocked if it's currently locked.
    ///
    /// [return_value]
    /// Returns a lock guard.
    pub fn lock(&'a self) -> SpinLockGuard<'a> {
        while self.val.compare_exchange_acquire(UNLOCKED, LOCKED) != UNLOCKED {
            arch_fns::spin();
        }
        self.guard()
    }
}

unsafe impl UndefState for SpinLock {
    fn num() -> usize { u8::max() as usize - 1 }

    unsafe fn set_undef(val: *mut SpinLock, n: usize) {
        assert!(n < Self::num());
        assert!(mem::size_of::<SpinLock>() == mem::size_of::<u8>());
        *(val as *mut u8) = n as u8 + 2;
    }

    unsafe fn is_undef(val: *const SpinLock, n: usize) -> bool {
        assert!(mem::size_of::<SpinLock>() == mem::size_of::<u8>());
        *(val as *const u8) == n as u8 + 2
    }
}

/// A lock-guard.
///
/// = Remarks
///
/// This guard automatically unlocks the lock when it goes out of scope.
pub struct SpinLockGuard<'a> {
    lock: &'a SpinLock,
}

impl<'a> SpinLockGuard<'a> {
    /// Returns the lock guarded by this guard.
    pub fn as_lock(&self) -> &'a SpinLock {
        self.lock
    }

    /// Unlocks the lock and returns a reference to the lock.
    pub fn unlock(self) -> &'a SpinLock {
        self.lock
    }
}

impl<'a> Drop for SpinLockGuard<'a> {
    fn drop(&mut self) {
        self.lock.val.store_release(UNLOCKED);
    }
}
