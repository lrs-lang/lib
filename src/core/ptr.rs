// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use intrinsics::{self};
use mem::{self};
use ops::{Eq, PartialOrd, Ordering, Deref, CoerceUnsized};
use cmp::{Ord};
use option::{Option};
use marker::{Sized, PhantomData, Sync, Send, Copy, Unsize, Pod};

/// Reads a value from a pointer.
///
/// [argument, src]
/// The pointer that will be read from.
///
/// [return_value]
/// Returns the object pointed to by the pointer.
pub unsafe fn read<T>(src: *const T) -> T {
    let mut tmp: T = mem::uninit();
    memcpy(&mut tmp, src, 1);
    tmp
}

/// Writes a value to a pointer.
///
/// [argument, dst]
/// The pointer that will be written to.
///
/// [argument, data]
/// The object that will be written.
pub unsafe fn write<T>(dst: *mut T, data: T) {
    intrinsics::move_val_init(&mut *dst, data);
}

/// Runs the destructor of an object.
///
/// [argument, dst]
/// A pointer to the object.
pub unsafe fn drop<T: ?Sized>(dst: *mut T) {
    intrinsics::drop_in_place(dst);
}

/// Copies a number of elements between two non-overlapping pointers.
///
/// [argument, dst]
/// The pointer that will be written to.
///
/// [argument, src]
/// The pointer that will be read from.
///
/// [argument, n]
/// The number of elements that will be copied.
///
/// = Remarks
///
/// If the pointers overlap, the behavior is undefined.
pub unsafe fn memcpy<T>(dst: *mut T, src: *const T, n: usize) {
    intrinsics::copy_nonoverlapping(src, dst, n);
}

/// Copies a number of elements between two, possibly overlapping, pointers.
///
/// [argument, dst]
/// The pointer that will be written to.
///
/// [argument, src]
/// The pointer that will be read from.
///
/// [argument, n]
/// The number of elements that will be copied.
pub unsafe fn memmove<T>(dst: *mut T, src: *const T, n: usize) {
    intrinsics::copy(src, dst, n);
}

#[lang = "const_ptr"]
impl<T> *const T {
    /// Returns whether this is a null pointer.
    pub fn is_null(self) -> bool {
        self as usize == 0
    }

    /// Creates a new pointer by calculating an offset from it.
    ///
    /// [argument, val]
    /// The value that will be added to the pointer.
    ///
    /// [return_value]
    /// Returns the new pointer.
    ///
    /// = Remarks
    ///
    /// This is like `ptr + val` in C. If the resulting pointer does not point into the
    /// same object or one byte after it, the behavior is undefined.
    pub unsafe fn offset(self, val: isize) -> *const T {
        intrinsics::offset(self, val)
    }

    /// Creates a new pointer by adding an offset to it.
    ///
    /// [argument, val]
    /// The value that will be added to the pointer.
    ///
    /// [return_value]
    /// Returns the new pointer.
    ///
    /// = Remarks
    ///
    /// This is like `ptr + val` in C. If the resulting pointer does not point into the
    /// same object or one byte after it, the behavior is undefined.
    pub unsafe fn add(self, val: usize) -> *const T {
        self.offset(val as isize)
    }

    /// Creates a new pointer by subtracting an offset from it.
    ///
    /// [argument, val]
    /// The value that will be subtracted from the pointer.
    ///
    /// [return_value]
    /// Returns the new pointer.
    ///
    /// = Remarks
    ///
    /// This is like `ptr - val` in C. If the resulting pointer does not point into the
    /// same object or one byte after it, the behavior is undefined.
    pub unsafe fn sub(self, val: usize) -> *const T {
        self.offset(-(val as isize))
    }
}

unsafe impl<T> Pod for *const T { }

impl<T> Eq for *const T {
    fn eq(&self, other: &*const T) -> bool {
        *self as usize == *other as usize
    }
}

impl<T> PartialOrd for *const T {
    fn partial_cmp(&self, other: &*const T) -> Option<Ordering> {
        (*self as usize).partial_cmp(&(*other as usize))
    }
}

impl<T> Ord for *const T {
    fn cmp(&self, other: &*const T) -> Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

#[lang = "mut_ptr"]
impl<T> *mut T {
    /// Returns whether this is a null pointer.
    pub fn is_null(self) -> bool {
        self as usize == 0
    }

    /// Creates a new pointer by calculating an offset from it.
    ///
    /// [argument, val]
    /// The value that will be added to the pointer.
    ///
    /// [return_value]
    /// Returns the new pointer.
    ///
    /// = Remarks
    ///
    /// This is like `ptr + val` in C. If the resulting pointer does not point into the
    /// same object or one byte after it, the behavior is undefined.
    pub unsafe fn offset(self, val: isize) -> *mut T {
        intrinsics::offset(self, val) as *mut T
    }

    /// Creates a new pointer by adding an offset to it.
    ///
    /// [argument, val]
    /// The value that will be added to the pointer.
    ///
    /// [return_value]
    /// Returns the new pointer.
    ///
    /// = Remarks
    ///
    /// This is like `ptr + val` in C. If the resulting pointer does not point into the
    /// same object or one byte after it, the behavior is undefined.
    pub unsafe fn add(self, val: usize) -> *mut T {
        self.offset(val as isize)
    }

    /// Creates a new pointer by subtracting an offset from it.
    ///
    /// [argument, val]
    /// The value that will be subtracted from the pointer.
    ///
    /// [return_value]
    /// Returns the new pointer.
    ///
    /// = Remarks
    ///
    /// This is like `ptr - val` in C. If the resulting pointer does not point into the
    /// same object or one byte after it, the behavior is undefined.
    pub unsafe fn sub(self, val: usize) -> *mut T {
        self.offset(-(val as isize))
    }
}

unsafe impl<T> Pod for *mut T { }

impl<T> Eq for *mut T {
    fn eq(&self, other: &*mut T) -> bool {
        *self as usize == *other as usize
    }
}

impl<T> PartialOrd for *mut T {
    fn partial_cmp(&self, other: &*mut T) -> Option<Ordering> {
        (*self as usize).partial_cmp(&(*other as usize))
    }
}

impl<T> Ord for *mut T {
    fn cmp(&self, other: &*mut T) -> Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

#[lang = "non_zero"]
pub struct NonZeroPtr<T: ?Sized>(*const T);

impl<T: ?Sized> Copy for NonZeroPtr<T> { }

impl<T: ?Sized> NonZeroPtr<T> {
    pub const unsafe fn new(ptr: *const T) -> NonZeroPtr<T> {
        NonZeroPtr(ptr)
    }
}

impl<T: ?Sized> Deref for NonZeroPtr<T> {
    type Target = *const T;
    fn deref(&self) -> &*const T {
        &self.0
    }
}

impl<T> Eq for NonZeroPtr<T> {
    fn eq(&self, other: &NonZeroPtr<T>) -> bool {
        **self == **other
    }
}

impl<T> PartialOrd for NonZeroPtr<T> {
    fn partial_cmp(&self, other: &NonZeroPtr<T>) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T> Ord for NonZeroPtr<T> {
    fn cmp(&self, other: &NonZeroPtr<T>) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: ?Sized, U: ?Sized> CoerceUnsized<NonZeroPtr<U>> for NonZeroPtr<T>
    where T: Unsize<U>,
{}

pub struct OwnedPtr<T: ?Sized> {
    ptr: NonZeroPtr<T>,
    _marker: PhantomData<T>,
}

impl<T: ?Sized> Copy for OwnedPtr<T> { }

impl<T: ?Sized> OwnedPtr<T> {
    pub unsafe fn new(ptr: *const T) -> OwnedPtr<T> {
        OwnedPtr {
            ptr: NonZeroPtr::new(ptr),
            _marker: PhantomData,
        }
    }
}

impl<T: ?Sized> Deref for OwnedPtr<T> {
    type Target = *mut T;
    fn deref(&self) -> &*mut T {
        unsafe { mem::cast(&*self.ptr) }
    }
}

impl<T> Eq for OwnedPtr<T> {
    fn eq(&self, other: &OwnedPtr<T>) -> bool {
        **self == **other
    }
}

impl<T> PartialOrd for OwnedPtr<T> {
    fn partial_cmp(&self, other: &OwnedPtr<T>) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T> Ord for OwnedPtr<T> {
    fn cmp(&self, other: &OwnedPtr<T>) -> Ordering {
        (**self).cmp(&**other)
    }
}

unsafe impl<T: Sync + ?Sized> Sync for OwnedPtr<T> { }
unsafe impl<T: Send + ?Sized> Send for OwnedPtr<T> { }

impl<T: ?Sized, U: ?Sized> CoerceUnsized<OwnedPtr<U>> for OwnedPtr<T>
    where T: Unsize<U>,
{}

/// Performs a volatile load.
///
/// [argument, src]
/// The source to load from.
///
/// [return_value]
/// Returns the loaded value.
///
/// = Remarks
///
/// This is unsafe because the pointer might not be valid.
///
/// = See also
///
/// * link:lrs::mem::volatile_load
pub unsafe fn volatile_load<T>(src: *const T) -> T {
    intrinsics::volatile_load(src)
}

/// Performs a volatile store.
///
/// [argument, dst]
/// The destination to write to.
///
/// [argument, val]
/// The value to write.
///
/// = Remarks
///
/// This is unsafe because the pointer might not be valid.
///
/// = See also
///
/// * link:lrs::mem::volatile_store
pub unsafe fn volatile_store<T>(dst: *mut T, val: T) {
    intrinsics::volatile_store(dst, val);
}
