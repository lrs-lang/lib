// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::marker::{Sync};

/// A container with interior mutability.
///
/// Modifying data through immutable `&` references is undefined behavior unless the data
/// is (recursively) contained in a `Cell`.
///
/// # Example
///
/// ```
/// struct X {
///     val: Cell<i32>,
/// }
///
/// impl X {
///     fn modify(&self, new: i32) {
///         unsafe { *self.val.ptr() = new; }
///     }
/// }
/// ```
///
/// This type is not `Sync` because race conditions are undefined behavior. It should
/// rarely be used directly except to build more robust structures with interior
/// mutability such as `CopyCell`.
#[lang="unsafe_cell"]
#[derive(Copy)]
pub struct Cell<T> {
    pub data: T,
}

impl<T> !Sync for Cell<T> { }

impl<T> Cell<T> {
    /// Creates a new `Cell` with the specified data.
    pub fn new(data: T) -> Cell<T> {
        Cell { data: data }
    }

    /// Returns a mutable pointer to the data.
    pub fn ptr(&self) -> *mut T {
        &self.data as *const T as *mut T
    }
}
