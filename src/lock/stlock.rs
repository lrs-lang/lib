// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {atomic};
use core::{mem};
use core::ops::{Eq};
use cell::{CopyCell};

/// A single-threaded lock.
///
/// = Remarks
///
/// This can be used to protect critical sections from signal handlers. It cannot be
/// used for inter-thread synchronization.
pub struct StLock {
    locked: CopyCell<bool>,
}

/// = Remarks
///
/// Two locks are equal if their addresses are equal.
impl Eq for StLock {
    fn eq(&self, other: &StLock) -> bool {
        mem::addr(self) == mem::addr(other)
    }
}

unsafe impl Sync for StLock { } // XXX: Should be ThreadLocal and not Sync.
unsafe impl Send for StLock { }

impl StLock {
    /// Creates a new, unlocked, lock.
    pub const fn new() -> StLock {
        StLock { locked: CopyCell::new(false) }
    }

    /// Returns whether the lock is locked.
    pub fn locked(&self) -> bool {
        atomic::single_thread_fence_acquire();
        self.locked.get()
    }

    /// Locks the lock.
    ///
    /// [return_value]
    /// Returns a guard that will unlock the lock.
    ///
    /// = Remarks
    ///
    /// This function aborts the process if the lock is already locked.
    pub fn lock<'a>(&'a self) -> StLockGuard<'a> {
        self.try_lock().unwrap()
    }

    /// Tries to lock the lock.
    ///
    /// [return_value]
    /// Returns a guard that will unlock the lock or `None` if the lock is already
    /// locked.
    pub fn try_lock<'a>(&'a self) -> Option<StLockGuard<'a>> {
        atomic::single_thread_fence_acquire();
        if !self.locked.get() {
            self.locked.set(true);
            atomic::single_thread_fence_acquire_release();
            Some(StLockGuard {
                lock: &self
            })
        } else {
            None
        }
    }
}

/// A lock-guard.
///
/// = Remarks
///
/// This guard automatically unlocks the lock when it goes out of scope.
pub struct StLockGuard<'a> {
    lock: &'a StLock,
}

impl<'a> StLockGuard<'a> {
    /// Returns the lock guarded by this guard.
    pub fn as_lock(&self) -> &'a StLock {
        self.lock
    }

    /// Unlocks the lock and returns a reference to the lock.
    pub fn unlock(self) -> &'a StLock {
        self.lock
    }
}

impl<'a> Drop for StLockGuard<'a> {
    fn drop(&mut self) {
        self.lock.locked.set(false);
        atomic::single_thread_fence_release();
    }
}
