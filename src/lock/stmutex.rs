// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// XXX: This is literally a copy of Mutex with St prefixes added everywhere. Can't make it
// generic over `MutexGuard<'a>`.

use base::prelude::*;
use io::{Write};
use fmt::{Debug};
use cell::{Cell};
use stlock::{SingleThreadLock, SingleThreadLockGuard};

/// A mutex protecting some data.
pub struct SingleThreadMutex<T> {
    lock: SingleThreadLock,
    data: Cell<T>,
}

impl<T> SingleThreadMutex<T> {
    /// Creates a new mutex.
    ///
    /// [argument, data]
    /// The data to be protected by the mutex.
    pub const fn new(data: T) -> SingleThreadMutex<T> {
        SingleThreadMutex {
            lock: SingleThreadLock::new(),
            data: Cell::new(data),
        }
    }

    fn guard<'a>(&'a self,
                 guard: SingleThreadLockGuard<'a>) -> SingleThreadMutexGuard<'a, T> {
        SingleThreadMutexGuard {
            guard: guard,
            mutex: self,
            _marker: (NoSend, NoInterrupt),
        }
    }

    /// Returns the underlying lock of this mutex.
    pub fn as_lock(&self) -> &SingleThreadLock {
        &self.lock
    }

    /// Tries to lock the mutex if it's currently unlocked.
    ///
    /// [return_value]
    /// Returns a guard if the operation succeeded.
    pub fn try_lock<'a>(&'a self) -> Option<SingleThreadMutexGuard<'a, T>> {
        self.lock.try_lock().map(|g| self.guard(g))
    }

    /// Locks the mutex by sleeping until the mutex is unlocked if it's currently locked.
    ///
    /// [return_value]
    /// Returns a mutex-guard.
    pub fn lock<'a>(&'a self) -> SingleThreadMutexGuard<'a, T> {
        self.guard(self.lock.lock())
    }

    /// Turns a lock-guard of the underlying lock into a mutex-guard.
    ///
    /// [argument, guard]
    /// The lock-guard of the underlying lock.
    ///
    /// [return_value]
    /// Returns a mutex-guard of this mutex.
    ///
    /// = Remarks
    ///
    /// The provided lock-guard must be a lock-guard of the underlying lock or the process
    /// is aborted.
    pub fn existing_lock<'a>(
        &'a self,
        guard: SingleThreadLockGuard<'a>
    ) -> SingleThreadMutexGuard<'a, T>
    {
        assert!(&self.lock == guard.as_lock());
        self.guard(guard)
    }

    /// Provides mutable access to the protected data without locking the lock.
    ///
    /// = Remarks
    ///
    /// This is safe because the availability of a mutable reference implies that there
    /// are currently no mutex-guards borrowing the mutex. Vice versa, no mutex guards can
    /// be created while the data is borrowed.
    pub fn data(&mut self) -> &mut T {
        unsafe { &mut *self.data.ptr() }
    }
}

unsafe impl<T> Interrupt for SingleThreadMutex<T> { }
unsafe impl<T> Send for SingleThreadMutex<T> where T: Send { }

/// A mutex-guard.
///
/// = Remarks
///
/// This guard automatically unlocks the mutex when it goes out of scope.
pub struct SingleThreadMutexGuard<'a, T: 'a> {
    guard: SingleThreadLockGuard<'a>,
    mutex: &'a SingleThreadMutex<T>,
    _marker: (NoSend, NoInterrupt),
}

impl<'a, T> SingleThreadMutexGuard<'a, T> {
    /// Returns a reference to the underlying lock-guard.
    pub fn as_lock_guard(&self) -> &SingleThreadLockGuard<'a> {
        &self.guard
    }

    /// Turns the mutex-guard into the underlying lock-guard.
    pub fn into_lock_guard(self) -> SingleThreadLockGuard<'a> {
        self.guard
    }

    /// Returns a reference to the underlying mutex.
    pub fn as_mutex(&self) -> &'a SingleThreadMutex<T> {
        self.mutex
    }

    /// Unlocks the mutex and returns a reference to it.
    pub fn unlock(self) -> &'a SingleThreadMutex<T> {
        self.mutex
    }
}

unsafe impl<'a, T> Sync for SingleThreadMutexGuard<'a, T> where T: Sync { }

impl<'a, T> Deref for SingleThreadMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.ptr() }
    }
}

impl<'a, T> DerefMut for SingleThreadMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.ptr() }
    }
}

impl<'a, T: Debug> Debug for SingleThreadMutexGuard<'a, T> {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.deref().fmt(w)
    }
}
