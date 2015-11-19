// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use cell::cell::{Cell};
use atomic::{AtomicCInt};
use syscall::{futex_wait, futex_wake};
use cty::{c_int};
use lock::{Lock, LockGuard, LOCK_INIT};

const WAITING:  c_int = 0;
const SIGNALED: c_int = 1;

struct Node {
    left: *mut Node,
    right: *mut Node,
    lock: AtomicCInt,
}

struct Inner {
    left_end: *mut Node,
    right_end: *mut Node,
    user_lock: Option<*const Lock>,
}

/// An initializer for static condition variables.
pub const RAW_CONDVAR_INIT: RawCondvar = RawCondvar {
    lock: LOCK_INIT,
    inner: Cell {
        data: Inner {
            left_end: 0 as *mut Node,
            right_end: 0 as *mut Node,
            user_lock: None,
        }
    },
};

/// A condition variable to wait on locks.
///
/// = Remarks
///
/// This implementation cannot be used for inter-process synchronization.
pub struct RawCondvar {
    lock: Lock,
    inner: Cell<Inner>,
}

impl RawCondvar {
    /// Atomically unlocks a lock guard and waits for a signal on this condvar before
    /// re-locking the lock.
    ///
    /// [argument, guard]
    /// The lock guard to be unlocked.
    ///
    /// [return_value]
    /// Returns a guard created by re-locking the lock of the guard argument.
    ///
    /// = Remarks
    ///
    /// While the condition variable is in use, the condition variable can only be used
    /// with the same lock. The condition variable is in use while there are users
    /// waiting on it. If the condition variable is used with another a lock, the process
    /// is aborted.
    pub fn wait<'a>(&self, guard: LockGuard<'a>) -> LockGuard<'a> {
        self.wait2(guard.as_lock(), guard)
    }

    /// Atomically unlocks a lock guard and waits for a signal on this condvar.
    ///
    /// [argument, guard]
    /// The lock guard to be unlocked.
    ///
    /// [argument, lock]
    /// The lock to be locked before returning.
    ///
    /// [return_value]
    /// Returns a guard created by locking the `lock` argument.
    ///
    /// = Remarks
    ///
    /// While the condition variable is in use, the condition variable can only be used
    /// with the same lock. The condition variable is in use while there are users
    /// waiting on it. If the condition variable is used with another a lock, the process
    /// is aborted.
    ///
    /// The lock in question in the above paragraph is the lock passed as the `lock`
    /// argument. The `guard` argument doesn't have to be related to the `lock` in any
    /// way.
    pub fn wait2<'a, 'b>(&self, lock: &'a Lock, guard: LockGuard<'b>) -> LockGuard<'a> {
        unsafe { self.unsafe_wait(lock, guard) }
    }

    unsafe fn unsafe_wait<'a, 'b>(&self, user_lock: &'a Lock,
                                  user_guard: LockGuard<'b>) -> LockGuard<'a> { 
        let mut node: Node = mem::unsafe_zeroed();

        {
            let _cvguard = self.lock.lock();
            let inner = &mut *self.inner.ptr();

            match inner.user_lock {
                Some(l) => assert!(l == user_lock),
                None => inner.user_lock = Some(user_lock),
            }

            node.left = inner.right_end;
            node.right = 0 as *mut Node;
            node.lock.store(WAITING);

            if !inner.right_end.is_null() {
                (&mut *inner.right_end).right = &mut node;
            }
            if inner.left_end.is_null() {
                inner.left_end = &mut node;
            }
            inner.right_end = &mut node;
        }

        drop(user_guard);

        while node.lock.load() == WAITING {
            futex_wait(&node.lock, WAITING, None);
        }

        let user_guard = user_lock.lock();

        if !node.right.is_null() {
            let next = &mut *node.right;
            next.lock.store(SIGNALED);
            futex_wake(&next.lock, 1);
        }

        user_guard
    }

    /// Wakes a number of threads waiting on this condvar.
    ///
    /// [argument, n]
    /// The number of threads to be woken.
    ///
    /// = Remarks
    ///
    /// It's possible that fewer than `n` threads are woken because fewer than `n` threads
    /// are currently waiting on this condvar.
    pub fn signal(&self, n: usize) {
        unsafe { self._signal(n) }
    }

    unsafe fn _signal(&self, mut n: usize) {
        if n == 0 {
            return;
        }

        let _cvguard = self.lock.lock();
        let inner = &mut *self.inner.ptr();

        if inner.left_end.is_null() {
            return;
        }

        let start = &*inner.left_end;

        let mut end = inner.left_end;
        while !end.is_null() && n > 0 {
            n -= 1;
            end = (&*end).right;
        }

        if !end.is_null() {
            (&mut *(&mut *end).left).right = 0 as *mut _;
            (&mut *end).left = 0 as *mut _;
        } else {
            inner.right_end = end;
            inner.user_lock = None;
        }
        inner.left_end = end;

        start.lock.store(SIGNALED);
        futex_wake(&start.lock, 1);
    }
}
