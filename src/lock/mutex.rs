// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use core::cell::{Cell};
use core::ops::{Deref, DerefMut};
use fmt::{Debug};
use io::{Write};
use lock::{LOCK_INIT, Lock, LockGuard};

pub struct Mutex<T> {
    lock: Lock,
    data: Cell<T>,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Mutex<T> {
        Mutex {
            lock: LOCK_INIT,
            data: Cell { data: data },
        }
    }

    fn guard<'a>(&'a self, guard: LockGuard<'a>) -> MutexGuard<'a, T> {
        MutexGuard {
            guard: guard,
            mutex: self,
            _marker: (NoSend, NoSync),
        }
    }

    pub fn as_lock(&self) -> &Lock {
        &self.lock
    }

    pub fn try_lock<'a>(&'a self) -> Option<MutexGuard<'a, T>> {
        self.lock.try_lock().map(|g| self.guard(g))
    }

    pub fn lock<'a>(&'a self) -> MutexGuard<'a, T> {
        self.guard(self.lock.lock())
    }

    pub fn existing_lock<'a>(&'a self, guard: LockGuard<'a>) -> MutexGuard<'a, T> {
        assert!(&self.lock == guard.as_lock());
        self.guard(guard)
    }

    pub fn data(&mut self) -> &mut T {
        unsafe { &mut *self.data.ptr() }
    }
}

unsafe impl<T: Sync> Sync for Mutex<T> { }
unsafe impl<T: Send> Send for Mutex<T> { }

pub struct MutexGuard<'a, T: 'a> {
    guard: LockGuard<'a>,
    mutex: &'a Mutex<T>,
    _marker: (NoSend, NoSync),
}

impl<'a, T> MutexGuard<'a, T> {
    pub fn as_lock_guard(&self) -> &LockGuard<'a> {
        &self.guard
    }

    pub fn into_lock_guard(self) -> LockGuard<'a> {
        self.guard
    }

    pub fn as_mutex(&self) -> &'a Mutex<T> {
        self.mutex
    }

    pub fn unlock(self) -> &'a Mutex<T> {
        self.mutex
    }
}

unsafe impl<'a, T: Sync> Sync for MutexGuard<'a, T> { }
unsafe impl<'a, T: Sync> Send for MutexGuard<'a, T> { }

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.ptr() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.ptr() }
    }
}

impl<'a, T: Debug> Debug for MutexGuard<'a, T> {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.deref().fmt(w)
    }
}
