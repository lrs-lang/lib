// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::marker::{Sync, NoSend, PhantomData, Send};
use core::ops::{Deref, DerefMut, Drop};
use cell::{Cell};

/// The status of a `RefCell`.
#[derive(Copy)]
pub enum RefCellStatus {
    /// Not borrowed.
    Free,
    /// Immutably borrowed. The argument is the number of immutable borrows minus one.
    Borrowed(usize),
    /// Mutably borrowed.
    BorrowedMut,
}

struct Inner<T> {
    status: RefCellStatus,
    data: T,
}

/// Interior mutability for non-Copy types.
pub struct RefCell<T> {
    inner: Cell<Inner<T>>,
}

impl<T> RefCell<T> {
    /// Creates a new `RefCell`.
    pub fn new(data: T) -> RefCell<T> {
        RefCell {
            inner: Cell {
                data: Inner {
                    status: RefCellStatus::Free,
                    data: data,
                },
            },
        }
    }

    fn inner(&self) -> &mut Inner<T> {
        unsafe { &mut *self.inner.ptr() }
    }

    /// Returns the borrow-status of the object.
    ///
    /// Note that there are no race conditions between this function and the borrow
    /// functions since `RefCell` is not `Sync`.
    pub fn status(&self) -> RefCellStatus {
        self.inner().status
    }

    /// Borrows the object immutably or panics if the object is borrowed mutably.
    pub fn borrow<'a>(&'a self) -> RefCellBorrow<'a, T> {
        let inner = self.inner();
        inner.status = match inner.status {
            RefCellStatus::Free => RefCellStatus::Borrowed(0),
            RefCellStatus::Borrowed(n) => RefCellStatus::Borrowed(n + 1),
            _ => abort!(),
        };
        RefCellBorrow {
            cell: self,
            _marker: NoSend,
        }
    }

    /// Borrows the object mutably or panics if the object is borrowed.
    pub fn borrow_mut<'a>(&'a self) -> RefCellBorrowMut<'a, T> {
        let inner = self.inner();
        inner.status = match inner.status {
            RefCellStatus::Free => RefCellStatus::BorrowedMut,
            _ => abort!(),
        };
        RefCellBorrowMut {
            cell: self,
            _marker: (PhantomData, NoSend),
        }
    }
}

unsafe impl<T: Send> Send for RefCell<T> { }

/// An immutable `RefCell` borrow.
pub struct RefCellBorrow<'a, T: 'a> {
    cell: &'a RefCell<T>,
    _marker: NoSend,
}

unsafe impl<'a, T: Sync> Sync for RefCellBorrow<'a, T> { }

impl<'a, T> Deref for RefCellBorrow<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.cell.inner().data
    }
}

impl<'a, T> Drop for RefCellBorrow<'a, T> {
    fn drop(&mut self) {
        let inner = self.cell.inner();
        inner.status = match inner.status {
            RefCellStatus::Borrowed(0) => RefCellStatus::Free,
            RefCellStatus::Borrowed(n) => RefCellStatus::Borrowed(n-1),
            _ => abort!(),
        }
    }
}

/// A mutable `RefCell` borrow.
pub struct RefCellBorrowMut<'a, T: 'a> {
    cell: &'a RefCell<T>,
    _marker: (PhantomData<&'a mut T>, NoSend),
}

unsafe impl<'a, T: Sync> Sync for RefCellBorrowMut<'a, T> { }

impl<'a, T> Deref for RefCellBorrowMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.cell.inner().data
    }
}

impl<'a, T> DerefMut for RefCellBorrowMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.cell.inner().data
    }
}

impl<'a, T> Drop for RefCellBorrowMut<'a, T> {
    fn drop(&mut self) {
        self.cell.inner().status = RefCellStatus::Free;
    }
}
