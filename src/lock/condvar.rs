// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use mutex::{Mutex, MutexGuard};
use raw_condvar::{RAW_CONDVAR_INIT, RawCondvar};

pub const CONDVAR_INIT: Condvar = Condvar { raw: RAW_CONDVAR_INIT };

pub struct Condvar {
    raw: RawCondvar,
}

impl Condvar {
    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        self.wait2(guard.as_mutex(), guard)
    }

    pub fn wait2<'a, 'b, T, U>(&self, mutex: &'a Mutex<T>,
                               guard: MutexGuard<'b, U>) -> MutexGuard<'a, T> {
        let lock = mutex.as_lock();
        let guard = guard.into_lock_guard();
        let guard = self.raw.wait2(lock, guard);
        mutex.existing_lock(guard)
    }

    pub fn signal(&self, n: usize) {
        self.raw.signal(n);
    }
}
