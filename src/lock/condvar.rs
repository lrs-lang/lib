// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use core::{mem};
use core::cell::{Cell};
use arch::atomic::{AtomicCInt};
use arch::syscall::{futex_wait, futex_wake};
use arch::cty::{c_int};
use {Lock, LockGuard};

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
}

pub struct Condvar {
    lock: Lock,
    user_lock: &'static Lock,
    inner: Cell<Inner>,
}

impl Condvar {
    pub fn new(lock: &'static Lock) -> Condvar {
        Condvar {
            lock: ::INIT,
            user_lock: lock,
            inner: Cell {
                data: Inner {
                    left_end: 0 as *mut _,
                    right_end: 0 as *mut _,
                },
            },
        }
    }

    pub fn set_lock(&mut self, lock: &'static Lock) {
        self.user_lock = lock;
    }

    pub fn wait(&self, guard: LockGuard) -> LockGuard {
        unsafe { self._wait(guard) }
    }

    unsafe fn _wait(&self, user_guard: LockGuard) -> LockGuard { 
        let mut node: Node = mem::zeroed();

        {
            let _cvguard = self.lock.as_static().lock();
            let inner = &mut *self.inner.ptr();

            node.left = inner.right_end;
            node.right = 0 as *mut Node;
            node.lock.store_seqcst(WAITING);

            if !inner.right_end.is_null() {
                (&mut *inner.right_end).right = &mut node;
            }
            if inner.left_end.is_null() {
                inner.left_end = &mut node;
            }
            inner.right_end = &mut node;
        }

        drop(user_guard);

        while node.lock.load_seqcst() == WAITING {
            futex_wait(node.lock.unwrap(), WAITING, None);
        }

        let user_guard = self.user_lock.lock();

        if !node.right.is_null() {
            let next = &mut *node.right;
            next.lock.store_seqcst(SIGNALED);
            futex_wake(next.lock.unwrap(), 1);
        }

        user_guard
    }

    pub fn signal(&self, n: usize) {
        unsafe { self._signal(n) }
    }

    unsafe fn _signal(&self, mut n: usize) {
        if n == 0 {
            return;
        }

        let _cvguard = self.lock.as_static().lock();
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
            inner.right_end = end
        }
        inner.left_end = end;

        start.lock.store_seqcst(SIGNALED);
        futex_wake(start.lock.unwrap(), 1);
    }
}
