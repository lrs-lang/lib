// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::marker::{Sync};

/// A container with interior mutability.
///
/// = Remarks
///
/// Modifying data through immutable `&` references is undefined behavior unless the data
/// is (recursively) contained in a `Cell`.
///
/// This type is not `Sync` because race conditions are undefined behavior. It should
/// rarely be used directly except to build more robust structures with interior
/// mutability such as `CopyCell`.
///
/// = Examples
///
/// ----
/// struct X {
///     val: Cell<i32>,
/// }
///
/// impl X {
///     fn modify(&self, new: i32) {
///         unsafe { *self.val.ptr() = new; }
///     }
/// }
/// ----
#[lang="unsafe_cell"]
#[derive(Copy)]
pub struct Cell<T> {
    /// The data contained in the cell.
    data: T,
}

impl<T> !Sync for Cell<T> { }

impl<T> Cell<T> {
    /// Creates a new cell.
    ///
    /// [argument, data]
    /// The datum initially contained in the cell.
    pub const fn new(data: T) -> Cell<T> {
        Cell { data: data }
    }

    /// Returns a mutable pointer to the data.
    pub fn ptr(&self) -> *mut T {
        &self.data as *const T as *mut T
    }
}

impl<T: Copy> Cell<T> {
    /// Returns a copy of the contained data.
    pub fn get(&self) -> T {
        self.data
    }

    /// Modifies the contained data.
    ///
    /// [argument, data]
    /// The new value.
    pub fn set(&self, data: T) {
        unsafe { *self.ptr() = data; }
    }
}
