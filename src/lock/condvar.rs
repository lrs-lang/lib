// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use mutex::{Mutex, MutexGuard};
use raw_condvar::{RawCondvar};

/// A condition variable to wait on mutexes.
///
/// = Remarks
///
/// This implementation cannot be used for inter-process synchronization.
pub struct Condvar {
    raw: RawCondvar,
}

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar { raw: RawCondvar::new() }
    }

    /// Atomically unlocks a mutex guard and waits for a signal on this condvar before
    /// re-locking the mutex.
    ///
    /// [argument, guard]
    /// The mutex guard to be unlocked.
    ///
    /// [return_value]
    /// Returns a guard created by re-locking the mutex of the guard argument.
    ///
    /// = Remarks
    ///
    /// While the condition variable is in use, the condition variable can only be used
    /// with the same mutex. The condition variable is in use while there are users
    /// waiting on it. If the condition variable is used with another a mutex, the process
    /// is aborted.
    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        self.wait2(guard.as_mutex(), guard)
    }

    /// Atomically unlocks a mutex guard and waits for a signal on this condvar.
    ///
    /// [argument, guard]
    /// The mutex guard to be unlocked.
    ///
    /// [argument, mutex]
    /// The mutex to be locked before returning.
    ///
    /// [return_value]
    /// Returns a guard created by locking the `mutex` argument.
    ///
    /// = Remarks
    ///
    /// While the condition variable is in use, the condition variable can only be used
    /// with the same mutex. The condition variable is in use while there are users
    /// waiting on it. If the condition variable is used with another a mutex, the process
    /// is aborted.
    ///
    /// The mutex in question in the above paragraph is the mutex passed as the `mutex`
    /// argument. The `guard` argument doesn't have to be related to the `mutex` in any
    /// way.
    pub fn wait2<'a, 'b, T, U>(&self, mutex: &'a Mutex<T>,
                               guard: MutexGuard<'b, U>) -> MutexGuard<'a, T> {
        let lock = mutex.as_lock();
        let guard = guard.into_lock_guard();
        let guard = self.raw.wait2(lock, guard);
        mutex.existing_lock(guard)
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
        self.raw.signal(n);
    }
}
