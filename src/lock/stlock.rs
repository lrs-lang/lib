// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {atomic};
use core::{mem};
use core::ops::{Eq};
use cell::{Cell};

/// A single-threaded lock.
///
/// = Remarks
///
/// This can be used to protect critical sections from signal handlers. It cannot be
/// used for inter-thread synchronization.
pub struct SingleThreadLock {
    locked1: Cell<bool>,
    locked2: Cell<bool>,
}

/// = Remarks
///
/// Two locks are equal if their addresses are equal.
impl Eq for SingleThreadLock {
    fn eq(&self, other: &SingleThreadLock) -> bool {
        mem::addr(self) == mem::addr(other)
    }
}

impl !Sync for SingleThreadLock { }
unsafe impl Interrupt for SingleThreadLock { }
unsafe impl Send for SingleThreadLock { }

impl SingleThreadLock {
    /// Creates a new, unlocked, lock.
    pub const fn new() -> SingleThreadLock {
        SingleThreadLock {
            locked1: Cell::new(false),
            locked2: Cell::new(false),
        }
    }

    /// Returns whether the lock is locked.
    pub fn locked(&self) -> bool {
        self.locked1.get()
    }

    /// Locks the lock.
    ///
    /// [return_value]
    /// Returns a guard that will unlock the lock.
    ///
    /// = Remarks
    ///
    /// This function aborts the process if the lock is already locked.
    pub fn lock<'a>(&'a self) -> SingleThreadLockGuard<'a> {
        self.try_lock().unwrap()
    }

    /// Tries to lock the lock.
    ///
    /// [return_value]
    /// Returns a guard that will unlock the lock or `None` if the lock is already
    /// locked.
    pub fn try_lock<'a>(&'a self) -> Option<SingleThreadLockGuard<'a>> {
        let locked = self.locked1.get();
        self.locked1.set(true);

        atomic::single_thread_fence_acquire_release();

        if locked | self.locked2.get() {
            None
        } else {
            self.locked2.set(true);
            Some(SingleThreadLockGuard {
                lock: &self
            })
        }
    }
}

/// A lock-guard.
///
/// = Remarks
///
/// This guard automatically unlocks the lock when it goes out of scope.
pub struct SingleThreadLockGuard<'a> {
    lock: &'a SingleThreadLock,
}

impl<'a> SingleThreadLockGuard<'a> {
    /// Returns the lock guarded by this guard.
    pub fn as_lock(&self) -> &'a SingleThreadLock {
        self.lock
    }

    /// Unlocks the lock and returns a reference to the lock.
    pub fn unlock(self) -> &'a SingleThreadLock {
        self.lock
    }
}

impl<'a> Drop for SingleThreadLockGuard<'a> {
    fn drop(&mut self) {
        self.lock.locked2.set(false);
        atomic::single_thread_fence_release();
        self.lock.locked1.set(false);
    }
}
