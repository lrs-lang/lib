// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Multi-threaded communication and synchronization.

pub use lrs_lock::{
    RawCondvar, Lock, LockGuard, DUMMY, Mutex,
    MutexGuard, Condvar, LockStatus, Once,
};
pub use lrs_queue::{
    Queue,
};
