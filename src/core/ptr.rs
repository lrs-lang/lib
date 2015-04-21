// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use intrinsics::{self};
use mem::{self};
use ops::{Eq, PartialOrd, Ordering};
use cmp::{Ord};
use option::{Option};

/// Reads a value from a pointer.
pub unsafe fn read<T>(src: *const T) -> T {
    let mut tmp: T = mem::uninit();
    memcpy(&mut tmp, src, 1);
    tmp
}

/// Writes a value to a pointer.
pub unsafe fn write<T>(dst: *mut T, data: T) {
    memcpy(dst, &data, 1);
    intrinsics::forget(data);
}

/// Copies `n` elements from `src` to `dst` which must not overlap.
pub unsafe fn memcpy<T>(dst: *mut T, src: *const T, n: usize) {
    intrinsics::copy_nonoverlapping(src, dst, n);
}

/// Copies `n` elements from `src` to `dst` which are allowed to overlap.
pub unsafe fn memmove<T>(dst: *mut T, src: *const T, n: usize) {
    intrinsics::copy(src, dst, n);
}

#[lang = "const_ptr"]
impl<T> *const T {
    /// Returns whether this is a null pointer.
    pub fn is_null(self) -> bool {
        self as usize == 0
    }

    /// Like `ptr + val` in C. The result must be a valid pointer or the behavior is
    /// undefined.
    pub unsafe fn offset(self, val: isize) -> *const T {
        intrinsics::offset(self, val)
    }

    /// Like `offset`.
    pub unsafe fn add(self, val: usize) -> *const T {
        self.offset(val as isize)
    }

    /// Like `offset`.
    pub unsafe fn sub(self, val: usize) -> *const T {
        self.offset(-(val as isize))
    }
}

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

    /// Like `ptr + val` in C. The result must be a valid pointer or the behavior is
    /// undefined.
    pub unsafe fn offset(self, val: isize) -> *mut T {
        intrinsics::offset(self, val) as *mut T
    }

    /// Like `offset`.
    pub unsafe fn add(self, val: usize) -> *mut T {
        self.offset(val as isize)
    }

    /// Like `offset`.
    pub unsafe fn sub(self, val: usize) -> *mut T {
        self.offset(-(val as isize))
    }
}

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
