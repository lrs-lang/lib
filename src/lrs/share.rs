// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Single-threaded interior mutability
//!
//! = Description
//!
//! This module provides structures that allow data-modification through immutable
//! references. None of the objects are safe to use from multiple threads.

pub use lrs_core::thread_local::{__ThreadLocal};
pub use lrs_cell::{
    Cell, RefCellStatus, RefCell, RefCellBorrow, RefCellBorrowMut,
};
pub use lrs_lock::{
    SingleThreadLock, SingleThreadMutex,
};
