// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use io::{Write};
use fmt::{Debug};
use cell::cell::{Cell};
use lock::{Lock, LockGuard};
use time_base::{Time};

/// A mutex protecting some data.
pub struct Mutex<T>
{
    lock: Lock,
    data: Cell<T>,
}

impl<T> Mutex<T> {
    /// Creates a new mutex.
    ///
    /// [argument, data]
    /// The data to be protected by the mutex.
    pub const fn new(data: T) -> Mutex<T> {
        Mutex {
            lock: Lock::new(),
            data: Cell::new(data),
        }
    }

    fn guard<'a>(&'a self, guard: LockGuard<'a>) -> MutexGuard<'a, T> {
        MutexGuard {
            guard: guard,
            mutex: self,
            _marker: (NoSend, NoSync),
        }
    }

    /// Returns the underlying lock of this mutex.
    pub fn as_lock(&self) -> &Lock {
        &self.lock
    }

    /// Tries to lock the mutex if it's currently unlocked.
    ///
    /// [return_value]
    /// Returns a guard if the operation succeeded.
    pub fn try_lock<'a>(&'a self) -> Result<MutexGuard<'a, T>> {
        self.lock.try_lock().map(|g| self.guard(g))
    }

    /// Locks the mutex by sleeping until the mutex is unlocked if it's currently locked.
    ///
    /// [return_value]
    /// Returns a mutex-guard.
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, T> {
        self.guard(self.lock.lock())
    }

    /// Locks the mutex by sleeping until the mutex is unlocked if it's currently locked
    /// or until a certain amount of time has expired.
    ///
    /// [argument, time]
    /// An upper bound for the amount of time until this function returns.
    ///
    /// [return_value]
    /// Returns a mutex guard or an error.
    ///
    /// = Remarks
    ///
    /// The function may take longer to return than allowed by the `time` parameter.
    pub fn try_lock_until<'a>(&'a self, time: Time) -> Result<MutexGuard<'a, T>> {
        self.lock.try_lock_for(time).map(|g| self.guard(g))
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
    pub fn existing_lock<'a>(&'a self, guard: LockGuard<'a>) -> MutexGuard<'a, T> {
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

unsafe impl<T> Sync for Mutex<T> where T: Send { }
unsafe impl<T> Send for Mutex<T> where T: Send { }

/// A mutex-guard.
///
/// = Remarks
///
/// This guard automatically unlocks the mutex when it goes out of scope.
pub struct MutexGuard<'a, T: 'a> {
    guard: LockGuard<'a>,
    mutex: &'a Mutex<T>,
    _marker: (NoSend, NoSync),
}

impl<'a, T> MutexGuard<'a, T> {
    /// Returns a reference to the underlying lock-guard.
    pub fn as_lock_guard(&self) -> &LockGuard<'a> {
        &self.guard
    }

    /// Turns the mutex-guard into the underlying lock-guard.
    pub fn into_lock_guard(self) -> LockGuard<'a> {
        self.guard
    }

    /// Returns a reference to the underlying mutex.
    pub fn as_mutex(&self) -> &'a Mutex<T> {
        self.mutex
    }

    /// Unlocks the mutex and returns a reference to it.
    pub fn unlock(self) -> &'a Mutex<T> {
        self.mutex
    }
}

unsafe impl<'a, T> Sync for MutexGuard<'a, T> where T: Sync { }
unsafe impl<'a, T> Send for MutexGuard<'a, T> where T: Send { }

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
