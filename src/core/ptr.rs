// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use intrinsics::{self};
use mem::{self};
use ops::{Eq, PartialOrd, Ordering, Deref, DerefMut, CoerceUnsized};
use cmp::{Ord};
use option::{Option};
use marker::{Sized, Copy, Unsize, Pod};
use non_zero::{NonZero};

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

pub struct NonZeroPtr<T: ?Sized>(NonZero<*const T>);

impl<T: ?Sized> Copy for NonZeroPtr<T> { }

impl<T: ?Sized> NonZeroPtr<T> {
    pub const unsafe fn new(ptr: *const T) -> NonZeroPtr<T> {
        NonZeroPtr(NonZero::new(ptr))
    }

    pub const fn get(self) -> *const T {
        self.0.into()
    }

    pub unsafe fn set(&mut self, val: *const T) {
        self.0.set(val);
    }
}

impl<T> Eq for NonZeroPtr<T> {
    fn eq(&self, other: &NonZeroPtr<T>) -> bool {
        self.0.into() == other.0.into()
    }
}

impl<T> PartialOrd for NonZeroPtr<T> {
    fn partial_cmp(&self, other: &NonZeroPtr<T>) -> Option<Ordering> {
        self.0.into().partial_cmp(&other.0.into())
    }
}

impl<T> Ord for NonZeroPtr<T> {
    fn cmp(&self, other: &NonZeroPtr<T>) -> Ordering {
        self.0.into().cmp(&other.0.into())
    }
}

impl<T: ?Sized, U: ?Sized> CoerceUnsized<NonZeroPtr<U>> for NonZeroPtr<T>
    where T: Unsize<U>,
{}

// == Examples:
//
//         | mutable memory | mutable object | immutable object |
// --------+----------------+----------------+------------------+
// noalias | Vec            | Box            | Rc               |
// --------+----------------+----------------+------------------+
// alias   | <unused>       | <unused>       | <unused>         |
// --------+----------------+----------------+------------------+
//
// == Copy:
//
//         | mutable memory | mutable object | immutable object |
// --------+----------------+----------------+------------------+
// noalias | <no>           | <no>           | Copy             |
// --------+----------------+----------------+------------------+
// alias   | Copy           | Copy           | Copy             |
// --------+----------------+----------------+------------------+
//
// == Deref:
//
//         | mutable memory | mutable object | immutable object |
// --------+----------------+----------------+------------------+
// noalias | <impossible>   | Deref/DerefMut | Deref            |
// --------+----------------+----------------+------------------+
// alias   | <impossible>   | <dangerous>    | <dangerous>      |
// --------+----------------+----------------+------------------+
//
// == Wrapper
//
//         | mutable memory | mutable object   | immutable object |
// --------+----------------+------------------+------------------+
// noalias | NoAliasMemPtr  | NoAliasMutObjPtr | NoAliasObjPtr    |
// --------+----------------+------------------+------------------+
// alias   | AliasMemPtr    | AliasMutObjPtr   | AliasObjPtr      |
// --------+----------------+------------------+------------------+

macro_rules! pointer_wrapper {
    ($name:ident, $raw_ty:ty) => {
        pub struct $name<T: ?Sized>(NonZeroPtr<T>);

        impl<T: ?Sized> $name<T> {
            pub const unsafe fn new(ptr: $raw_ty) -> Self {
                $name(NonZeroPtr::new(ptr))
            }

            pub unsafe fn set(&mut self, ptr: $raw_ty) {
                self.0.set(ptr);
            }

            pub const fn get(&self) -> *mut T {
                self.0.get() as *mut T
            }
        }

        impl<T> Eq for $name<T> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<T> PartialOrd for $name<T> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<T> Ord for $name<T> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl<T: ?Sized, U: ?Sized> CoerceUnsized<$name<U>> for $name<T>
            where T: Unsize<U>,
        {}
    }
}

pointer_wrapper!(NoAliasMemPtr, *mut T);
pointer_wrapper!(NoAliasMutObjPtr, *mut T);
pointer_wrapper!(NoAliasObjPtr, *const T);
pointer_wrapper!(AliasMemPtr, *mut T);
pointer_wrapper!(AliasMutObjPtr, *mut T);
pointer_wrapper!(AliasObjPtr, *const T);

macro_rules! deref {
    ($name:ident) => {
        impl<T: ?Sized> Deref for $name<T> {
            type Target = T;
            fn deref(&self) -> &T {
                unsafe { &*self.get() }
            }
        }
    }
}

macro_rules! deref_mut {
    ($name:ident) => {
        deref!($name);
        impl<T: ?Sized> DerefMut for $name<T> {
            fn deref_mut(&mut self) -> &mut T {
                unsafe { &mut *self.get() }
            }
        }
    }
}

deref!(NoAliasObjPtr);
deref_mut!(NoAliasMutObjPtr);
// XXX: Dangerous
deref!(AliasObjPtr);
deref_mut!(AliasMutObjPtr);

macro_rules! copy {
    ($name:ident) => {
        impl<T: ?Sized> Copy for $name<T> { }
    }
}

copy!(NoAliasObjPtr);
copy!(AliasMemPtr);
copy!(AliasMutObjPtr);
copy!(AliasObjPtr);
