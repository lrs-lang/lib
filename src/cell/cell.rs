// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{ptr};
use core::ops::{Eq};

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
pub struct Cell<T: ?Sized> {
    /// The data contained in the cell.
    data: T,
}

impl<T: ?Sized> !Interrupt for Cell<T> { }
impl<T: ?Sized> !Sync for Cell<T> { }

impl<T: Eq> Eq for Cell<T> {
    fn eq(&self, other: &Cell<T>) -> bool {
        self.data.eq(&other.data)
    }
}

impl<T: ?Sized> Cell<T> {
    /// Creates a new cell.
    ///
    /// [argument, data]
    /// The datum initially contained in the cell.
    pub const fn new(data: T) -> Cell<T> where T: Sized {
        Cell { data: data }
    }

    /// Returns a mutable pointer to the data.
    pub fn ptr(&self) -> *mut T {
        &self.data as *const T as *mut T
    }

    pub fn into(self) -> T
        where T: Sized
    {
        self.data
    }
}

impl<T> Cell<T> {
    /// Replaces the stored value by a new one.
    ///
    /// [argument, data]
    /// The replacement.
    ///
    /// [return_value]
    /// The old value.
    pub fn replace(&self, data: T) -> T {
        unsafe {
            let old = ptr::read(self.ptr());
            ptr::write(self.ptr(), data);
            old
        }
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

    /// Modifies the contained data.
    ///
    /// [argument, data]
    /// The new value.
    pub fn volatile_set(&self, data: T) {
        unsafe { ptr::volatile_store(self.ptr(), data); }
    }
}

impl<T, U> TryFrom<Cell<U>> for Cell<T>
    where U: TryTo<T>,
{
    fn try_from(from: &Cell<U>) -> Result<Cell<T>> {
        Ok(Cell::new(try!(from.data.try_to())))
    }
}

impl<T, U> From<Cell<U>> for Cell<T>
    where U: To<T>,
{
    fn from(from: &Cell<U>) -> Cell<T> {
        Cell::new(from.data.to())
    }
}
