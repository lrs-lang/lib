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
    /// Used to check if we are in a signal handler that is interrupting the critical
    /// section.
    down_lock: Cell<bool>,

    /// Used to check if we have been interrupted by a signal handler which entered the
    /// critical section and never left it, e.g., it forgot the guard or put it into a
    /// global. Consider the following event:
    ///
    ///     Bad event:
    ///         1: a signal arrives
    ///         2: the signal handler tries to lock
    ///         3: the signal handler forgets the lock guard
    ///
    /// And let's see how it fits into our locking process
    ///
    ///     a: we load down_lock into `var`
    ///     b: we store true in down_lock
    ///     c: we check if `var` or up_lock are true
    ///
    /// * If the bad event happens before `a`, then `var` is true and our locking fails.
    /// * If the bad event happens before `b`, then `up_lock` is true and our locking
    ///   fails.
    /// * If the bad event happens after `b`, then step 2 of the bad event fails and they
    ///   can't acquire the lock.
    ///
    /// Hence, with this double-locking trick, there is only ever one lock guard.
    up_lock: Cell<bool>,
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
            down_lock: Cell::new(false),
            up_lock: Cell::new(false),
        }
    }

    /// Returns whether the lock is locked.
    pub fn locked(&self) -> bool {
        self.down_lock.get()
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
        let locked = self.down_lock.get();
        self.down_lock.set(true);

        atomic::single_thread_fence_acquire_release();

        if locked | self.up_lock.get() {
            None
        } else {
            self.up_lock.set(true);
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
        self.lock.up_lock.set(false);
        atomic::single_thread_fence_release();
        self.lock.down_lock.set(false);
    }
}
