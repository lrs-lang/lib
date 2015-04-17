// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use intrinsics::{self};
use mem::{self};
use ops::{Eq, PartialOrd, Ordering};
use cmp::{Ord};
use option::{Option};

pub unsafe fn read<T>(src: *const T) -> T {
    let mut tmp: T = mem::uninit();
    memcpy(&mut tmp, src, 1);
    tmp
}

pub unsafe fn write<T>(dst: *mut T, data: T) {
    memcpy(dst, &data, 1);
    intrinsics::forget(data);
}

pub unsafe fn memcpy<T>(dst: *mut T, src: *const T, n: usize) {
    intrinsics::copy_nonoverlapping(src, dst, n);
}

pub unsafe fn memmove<T>(dst: *mut T, src: *const T, n: usize) {
    intrinsics::copy(src, dst, n);
}

#[lang = "const_ptr"]
impl<T> *const T {
    pub fn is_null(self) -> bool {
        self as usize == 0
    }

    pub unsafe fn offset(self, val: isize) -> *const T {
        intrinsics::offset(self, val)
    }

    pub unsafe fn add(self, val: usize) -> *const T {
        self.offset(val as isize)
    }

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
    pub fn is_null(self) -> bool {
        self as usize == 0
    }

    pub unsafe fn offset(self, val: isize) -> *mut T {
        intrinsics::offset(self, val) as *mut T
    }

    pub unsafe fn add(self, val: usize) -> *mut T {
        self.offset(val as isize)
    }

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
